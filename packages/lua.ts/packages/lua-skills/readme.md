# @nyar/lua-skills

Agent guides for the Lua language services package. Install the skill, then ask the
agent to parse files, inspect tokens, triage diagnostics, or tune throughput.

## Install

```bash
npx skills add @nyar/lua-skills --skill lua-language -y
```

## Scope

The `lua-language` skill covers the `@nyar/lua` CLI/API, native binding resolution,
the `lua-harness` diagnostics loop, and the Boost `language.lua` ownership boundary.
It does not teach Lua application development, execute Lua, or replace the parser.
