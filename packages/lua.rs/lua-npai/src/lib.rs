//! Lua Node-API export library.
//!
//! This crate does three things only: declare N-API objects, move arguments across
//! the bridge, and map pure Rust results from Boost `language.lua` back to the host.
//! The parser and language semantics belong to Boost `data.text.lua`; do not add a
//! parsing or highlighting bypass here.

#![deny(missing_docs)]

use data_text_core::{
    parser::session::ParseSession,
    tree::{GreenNode, GreenTree},
};
use data_text_lua::{LuaElementType, LuaLanguage, LuaLexer, LuaParser, LuaTokenType};
use language_lua::{
    LuaHighlighter,
    lsp::highlighter::{HighlightKind, Highlighter},
};
use napi_derive::napi;

/// Kind and byte span of a lexical or syntax leaf.
#[napi(object)]
pub struct TokenSpan {
    /// Leaf kind name (the debug name of `LuaTokenType`).
    pub kind: String,
    /// Start byte offset.
    pub start: u32,
    /// End byte offset (exclusive).
    pub end: u32,
}

/// Syntax tree node exposing only kind and byte span.
#[napi(object)]
pub struct SyntaxNode {
    /// Node kind name (the debug name of `LuaElementType` or `LuaTokenType`).
    pub kind: String,
    /// Start byte offset.
    pub start: u32,
    /// End byte offset (exclusive).
    pub end: u32,
    /// Child nodes; empty for leaves.
    pub children: Vec<SyntaxNode>,
}

/// Parse result.
#[napi(object)]
pub struct ParseOutcome {
    /// Whether a syntax tree was produced.
    pub ok: bool,
    /// Fatal error (present only when `ok = false`).
    pub error: Option<String>,
    /// Non-fatal diagnostics.
    pub diagnostics: Vec<String>,
    /// Syntax tree root.
    pub root: Option<SyntaxNode>,
    /// Number of leaves.
    pub token_count: u32,
}

/// Highlight span.
#[napi(object)]
pub struct HighlightSpan {
    /// Start byte offset.
    pub start: u32,
    /// End byte offset (exclusive).
    pub end: u32,
    /// Highlight kind: `keyword` / `string` / `number` / `comment` / `macro` /
    /// `identifier` / `operator`.
    pub kind: String,
}

struct Parsed {
    root: SyntaxNode,
    tokens: Vec<TokenSpan>,
    diagnostics: Vec<String>,
}

fn parse_source(source: &str) -> Result<Parsed, (String, Vec<String>)> {
    let language = LuaLanguage::default();
    let parser = LuaParser::new(&language);
    let lexer = LuaLexer::new(&language);
    let mut cache = ParseSession::<LuaLanguage>::default();
    let output = data_text_core::parser::parse(&parser, &lexer, source, &[], &mut cache);
    let mut diagnostics = output.diagnostics.iter().map(|error| format!("{error:?}")).collect::<Vec<_>>();

    match output.result {
        Ok(green) => {
            // The pinned `data.text.lua` parser records one `ExpectedToken` error for
            // the terminal `EndOfStream` token of every file. That token is the
            // parser's end-of-input marker, not user content, so the matching
            // diagnostic is dropped when it produced the trailing empty
            // `ExpressionStatement` described by `is_trailing_eof_statement`.
            if is_trailing_eof_statement(green) {
                diagnostics.pop();
            }
            let mut tokens = Vec::new();
            let root = to_syntax_node(green, 0, &mut tokens);
            Ok(Parsed { root, tokens, diagnostics })
        }
        Err(error) => Err((format!("{error:?}"), diagnostics)),
    }
}

/// Whether the tree ends with the empty `ExpressionStatement` the parser builds
/// from the terminal `EndOfStream` token.
///
/// Real incomplete input keeps its `Error` node inside the surrounding statement
/// (for example `local x =`), so this check only matches the end-of-input artifact.
fn is_trailing_eof_statement(node: &GreenNode<'_, LuaLanguage>) -> bool {
    let Some(GreenTree::Node(last)) = node.children.last() else { return false };
    if last.kind != LuaElementType::ExpressionStatement || last.byte_length != 0 {
        return false;
    }
    if last.children.len() != 1 {
        return false;
    }
    let GreenTree::Node(error) = &last.children[0] else { return false };
    if error.kind != LuaElementType::Error || error.children.len() != 1 {
        return false;
    }
    matches!(&error.children[0], GreenTree::Leaf(leaf) if leaf.kind == LuaTokenType::EndOfStream)
}

fn to_syntax_node(node: &GreenNode<'_, LuaLanguage>, start: u32, tokens: &mut Vec<TokenSpan>) -> SyntaxNode {
    let mut cursor = start;
    let mut children = Vec::with_capacity(node.children.len());

    for child in node.children {
        match child {
            GreenTree::Node(child) => {
                children.push(to_syntax_node(child, cursor, tokens));
                cursor += child.byte_length;
            }
            GreenTree::Leaf(leaf) => {
                let end = cursor + leaf.length;
                let kind = format!("{:?}", leaf.kind);
                tokens.push(TokenSpan { kind: kind.clone(), start: cursor, end });
                children.push(SyntaxNode { kind, start: cursor, end, children: Vec::new() });
                cursor = end;
            }
        }
    }

    SyntaxNode { kind: format!("{:?}", node.kind), start, end: start + node.byte_length, children }
}

fn highlight_kind(kind: HighlightKind) -> &'static str {
    match kind {
        HighlightKind::Keyword => "keyword",
        HighlightKind::String => "string",
        HighlightKind::Number => "number",
        HighlightKind::Comment => "comment",
        HighlightKind::Macro => "macro",
        HighlightKind::Identifier => "identifier",
        HighlightKind::Operator => "operator",
    }
}

/// npm package version (published with the crate version).
#[napi]
pub fn lua_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Parse Lua source and return the syntax tree.
#[napi]
pub fn lua_parse(source: String) -> ParseOutcome {
    match parse_source(&source) {
        Ok(parsed) => ParseOutcome {
            ok: true,
            error: None,
            diagnostics: parsed.diagnostics,
            token_count: parsed.tokens.len() as u32,
            root: Some(parsed.root),
        },
        Err((error, diagnostics)) => ParseOutcome { ok: false, error: Some(error), diagnostics, root: None, token_count: 0 },
    }
}

/// Lexical leaves in source order.
#[napi]
pub fn lua_tokens(source: String) -> Vec<TokenSpan> {
    parse_source(&source).map(|parsed| parsed.tokens).unwrap_or_default()
}

/// Diagnostics with the fatal error first.
#[napi]
pub fn lua_check(source: String) -> Vec<String> {
    match parse_source(&source) {
        Ok(parsed) => parsed.diagnostics,
        Err((error, diagnostics)) => {
            let mut all = Vec::with_capacity(diagnostics.len() + 1);
            all.push(error);
            all.extend(diagnostics);
            all
        }
    }
}

/// Lua highlighting backed by Boost `language.lua`.
#[napi]
pub fn lua_highlight(source: String) -> Vec<HighlightSpan> {
    let highlighter = LuaHighlighter::new();
    highlighter
        .highlight(&source)
        .into_iter()
        .map(|(start, end, kind)| HighlightSpan {
            start: start as u32,
            end: end as u32,
            kind: highlight_kind(kind).to_string(),
        })
        .collect()
}
