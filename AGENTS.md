# AGENTS

Guidance for AI agents working in this repository. Humans should start with
[readme.md](./readme.md) and the [maintainer handbook](./documentation/index.md).

## What this repository is

`lua` packages Lua language services for Node.js. The grammar, lexer and
highlighter are owned by Boost `language.lua` / `data.text.lua`. This repository
owns only:

- the `lua-napi` Node-API addon (`packages/lua.rs/lua-npai`);
- the prebuilt platform packages;
- the npm packages (`@nyar/lua`, `@nyar/lua-harness`, `@nyar/lua-skills`);
- the VMZ homepage and its multilingual user docs.

There is no interpreter, no VM and no `nyar-vm` dependency here. Execution belongs
to a Lua runtime; this repository is read-only language tooling.

## Hard rules

1. The default branch is `dev`; CI targets only `dev`.
2. Rust is nightly because Boost enables unstable features.
3. `lua-napi` stays a projection layer. Do not add parsing or highlighting
   fallbacks in Rust or JavaScript. Extend Boost and bump the pinned revision.
4. Rust and npm dependencies are pinned to the Boost revision in `Cargo.toml`.
5. Repository content is English except translated user docs under
   `packages/lua.ts/homepage/documents/`.
6. `documentation/` is plain Markdown with no build step.

## Commands

```bash
cargo check --workspace
cargo test --workspace
node scripts/build.mjs napi
pnpm install && pnpm build
```
