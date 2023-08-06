---
name: lua-language
description: Use @nyar/lua and lua-harness to parse Lua, inspect tokens, triage diagnostics, and tune native parse throughput.
---

# Lua Language Services

Use this skill when a user asks to parse Lua, list tokens, collect parser
diagnostics, highlight Lua source, or benchmark the native binding.

## Ownership boundary

- The parser, lexer and highlighter live in Boost `language.lua` / `data.text.lua`.
- `@nyar/lua` is only an npm surface over the `lua-napi` Node-API addon.
- Do not add parsing fallbacks in JavaScript. If a capability is missing, extend
  `lua-napi` and rebuild the platform binding.
- `@nyar/lua` does not execute Lua. Evaluation belongs to a Lua runtime.

## Commands

| Command | Purpose |
|---------|---------|
| `lua version` | Binding version |
| `lua parse <file> [--json]` | Syntax tree with byte spans |
| `lua tokens <file> [--json]` | Token kinds and spans |
| `lua check <file...>` | Parser diagnostics |
| `lua highlight <file> [--json]` | Keyword/string/number/comment/macro/identifier/operator spans |
| `lua-harness bench <file> -n 200` | Repeated parse latency |
| `lua-harness tokens <file>` | Token counts by kind |

## API

```ts
import { parse, tokens, check, highlight } from '@nyar/lua';

const outcome = parse('local x = 1');
if (!outcome.ok) console.error(outcome.error, outcome.diagnostics);
```

## Native resolution

`@nyar/lua` loads `@nyar/lua-<platform>/lib/lua.<triple>.node`. Set `LUA_NATIVE`
to validate a freshly built addon without reinstalling platform packages.

## Rebuild

```bash
node scripts/build.mjs napi
node scripts/build.mjs napi --all
```
