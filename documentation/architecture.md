# Architecture

## One pipeline

```
Boost language.lua / data.text.lua
        │  pure Rust parsing, lexing, highlighting
        ▼
packages/lua.rs/lua-npai  (crate lua-napi, cdylib + rlib)
        │  N-API objects: parse tree, token spans, diagnostics, highlight spans
        ▼
packages/lua.ts/platforms/lua-<platform>/lib/lua.<triple>.node
        │  prebuilt addon selected by host OS/arch
        ▼
@nyar/lua (native.ts → CLI + API) · @nyar/lua-harness
```

The homepage (`@nyar/lua-homepage`) is a separate VMZ application. Its user docs
are Markdown under `packages/lua.ts/homepage/documents/` and mount at `/d/`.

## Ownership boundary

| Layer | Owner | May change |
|-------|-------|-----------|
| Lua grammar, lexer, highlighter | Boost `language.lua` / `data.text.lua` | No (bump the revision) |
| N-API object shape and exports | `lua-napi` | Yes |
| Host binding resolution and CLI | `@nyar/lua` | Yes |
| Diagnostics and latency loop | `@nyar/lua-harness` | Yes |
| User docs | homepage `documents/` | Yes (all locales) |

`lua-napi` must stay a projection layer. If a capability is missing, add it to
Boost and raise the pinned revision instead of reimplementing it here.

## Parse data flow

1. `luaParse(source)` builds a `LuaLanguage`, `LuaParser`, `LuaLexer` and a
   `ParseSession`, then calls `data_text_core::parser::parse`.
2. The green tree is walked once. Nodes become `SyntaxNode`; leaves become both
   `SyntaxNode` children and `TokenSpan` rows. Offsets are byte offsets.
3. `luaCheck` returns the session diagnostics; a fatal error is placed first.
4. `luaTokens` and `luaHighlight` reuse the same entry points so spans stay
   consistent across commands.

## Binding resolution

`native.ts` resolves the addon in this order:

1. `LUA_NATIVE` when set;
2. `@nyar/lua-<short>/lib/lua.<triple>.node` via `require.resolve`;
3. repository-relative `platforms/lua-<short>/lib/...` (local development);
4. `node_modules/@nyar/lua-<short>/...`.

If none exist the error lists every path tried. Rebuild with
`node scripts/build.mjs napi`.

## Deliberate non-goals

- No JavaScript parser fallback.
- No Lua interpreter or VM in this repository.
- No per-language code in the CLI.
- No committed `.node` binaries: platform packages are populated at release time.
