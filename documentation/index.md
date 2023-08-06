# Maintainer handbook

This handbook is for people changing the repository. End-user documentation lives
in the homepage at `/d/*` and is translated; this handbook is English-only.

## What this repository owns

The Lua language implementation is not here. Parsing, tokens and highlighting come
from Boost `language.lua` / `data.text.lua`. This repository owns:

- the Node-API addon (`lua-napi`) that exposes those Rust capabilities to Node.js;
- the prebuilt `.node` platform packages;
- the npm packages (`@nyar/lua`, `@nyar/lua-harness`, `@nyar/lua-skills`);
- the VMZ homepage and its multilingual user docs.

## Layout

| Path | Role |
|------|------|
| `packages/lua.rs/lua-npai` | `lua-napi` Node-API addon (Rust) |
| `packages/lua.ts/platforms/*` | Prebuilt `.node` addons per host |
| `packages/lua.ts/packages/lua` | Main package: CLI + API |
| `packages/lua.ts/packages/lua-harness` | Diagnostics and tuning harness |
| `packages/lua.ts/packages/lua-skills` | Agent guides |
| `packages/lua.ts/homepage` | VMZ homepage and `/d/*` user docs |
| `documentation` | This maintainer handbook |
| `scripts` | Native build, JS emit and publish preparation |

## Quick start

```bash
pnpm install
cargo check --workspace
pnpm build                 # native addon + JS entries
pnpm --filter @nyar/lua-homepage run build
```

Smoke test:

```bash
node packages/lua.ts/packages/lua/bin/lua.mjs version
node packages/lua.ts/packages/lua/bin/lua.mjs parse path/to/file.lua
```

## Rules of thumb

1. Keep the Rust crate a bridge. Do not implement parsing or highlighting in
   JavaScript or in `lua-napi`.
2. Bump the Boost revision in `Cargo.toml` when you need new language behaviour.
3. Translate user-facing docs; keep code, comments, CI and maintainer docs English.
4. Add a platform package only with a matching build target and `.node` name.

Continue with [Architecture](./architecture.md).
