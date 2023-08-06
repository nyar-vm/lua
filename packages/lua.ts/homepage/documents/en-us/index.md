# Lua language services

`@nyar/lua` brings Lua parsing to Node.js through a native Node-API addon. It is
read-only: you give it source text, it gives you syntax trees, tokens, diagnostics
and highlight spans. It does not execute Lua.

## What you get

| Export | Returns |
|--------|---------|
| `parse(source)` | `{ ok, error, diagnostics, root, tokenCount }` |
| `tokens(source)` | `TokenSpan[]` with byte offsets |
| `check(source)` | `string[]` diagnostics |
| `highlight(source)` | keyword/string/number/comment/macro/identifier/operator spans |
| `version()` | native binding version |

## Install

```bash
npm install @nyar/lua
```

The matching platform package is installed automatically.

## CLI

```bash
lua version
lua parse path/to/file.lua --json
lua tokens path/to/file.lua
lua check src/*.lua
lua highlight path/to/file.lua
```

## Next

- [CLI](./cli.md)
- [API](./api.md)
- [Platforms and troubleshooting](./native.md)
