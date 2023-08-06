# lua

Lua language services packaged for Node.js.

The parser, lexer and highlighter live in Boost `language.lua` / `data.text.lua`.
This repository ships the Node-API export layer and the npm packages around it.
It does not implement or embed a Lua virtual machine.

## Layout

| Path | Role |
|------|------|
| `packages/lua.rs/lua-npai` | `lua-napi` Node-API addon (Rust) |
| `packages/lua.ts/platforms/*` | Prebuilt `.node` addons per host |
| `packages/lua.ts/packages/lua` | Main package: CLI (`@vmz/commander`) + API |
| `packages/lua.ts/packages/lua-harness` | Diagnostics and parse-latency tuning |
| `packages/lua.ts/packages/lua-skills` | Agent guides |
| `packages/lua.ts/homepage` | VMZ homepage; user docs at `/d/*` (multilingual) |
| `documentation` | Maintainer handbook (English) |

## Build

```bash
cargo check --workspace
node scripts/build.mjs napi          # build addon, copy into the host platform package
node scripts/build-npm-package-js.mjs
```

## CLI

```bash
lua version
lua parse file.lua --json
lua tokens file.lua
lua check src/*.lua
lua highlight file.lua
lua-harness bench file.lua -n 200
```

## License

MIT
