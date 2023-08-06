# Native binding

## Crate

`packages/lua.rs/lua-npai` builds the `lua-napi` crate. It is a `cdylib` for Node
and an `rlib` for tests.

Dependencies come from Boost by pinned revision:

```toml
language-lua   = { git = "https://github.com/oovm/boost.rs", rev = "<sha>" }
data-text-core = { git = "https://github.com/oovm/boost.rs", rev = "<sha>" }
data-text-lua  = { git = "https://github.com/oovm/boost.rs", rev = "<sha>" }
```

All three use the same revision so Cargo resolves a single copy of each crate.
Raising the revision is a deliberate change: run the smoke tests before committing.

## Exports

| Rust | JS | Returns |
|------|----|---------|
| `lua_version` | `luaVersion()` | crate version |
| `lua_parse` | `luaParse(source)` | `{ ok, error, diagnostics, root, tokenCount }` |
| `lua_tokens` | `luaTokens(source)` | `TokenSpan[]` |
| `lua_check` | `luaCheck(source)` | `string[]` |
| `lua_highlight` | `luaHighlight(source)` | `HighlightSpan[]` |

N-API converts field names to camelCase on the JS side.

## `.node` names

| Host | Target triple | Platform package | File |
|------|---------------|------------------|------|
| Windows x64 | `x86_64-pc-windows-msvc` | `lua-win32-x64` | `lua.win32-x64-msvc.node` |
| Linux x64 | `x86_64-unknown-linux-gnu` | `lua-linux-x64` | `lua.linux-x64-gnu.node` |
| macOS x64 | `x86_64-apple-darwin` | `lua-darwin-x64` | `lua.darwin-x64.node` |
| macOS arm64 | `aarch64-apple-darwin` | `lua-darwin-arm64` | `lua.darwin-arm64.node` |

`scripts/build.mjs` maps these automatically and copies the artifact into the
matching `platforms/lua-<short>/lib/` directory.

## Adding an export

1. Add the pure function to Boost if it does not exist.
2. Wrap it in `lua-napi` with `#[napi]` and an English doc comment. `deny(missing_docs)`
   is enabled.
3. Re-export it from `packages/lua.ts/packages/lua/src/native.ts` and `index.ts`.
4. Add a CLI command or flag only if users need it.
5. Rebuild and run the smoke tests:

```bash
node scripts/build.mjs napi
node packages/lua.ts/packages/lua/bin/lua.mjs parse path/to/file.lua
```

`LUA_NATIVE=/absolute/path/to/lua.node` lets you validate an addon without
rebuilding platform packages.
