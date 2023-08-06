# Install

## Requirements

- Node.js 22.18 or newer.
- A supported host: Windows x64, Linux x64, macOS x64 or macOS arm64.

## npm

```bash
npm install @nyar/lua
```

`@nyar/lua` declares one optional dependency per host
(`@nyar/lua-win32-x64`, `@nyar/lua-linux-x64`, `@nyar/lua-darwin-x64`,
`@nyar/lua-darwin-arm64`). npm installs only the one matching your OS and CPU.

## Verify

```bash
lua version
lua parse example.lua
```

If the addon cannot be found, `lua version` prints every path it tried. See
[Platforms and troubleshooting](../native.md).

## Using a custom addon

Point `LUA_NATIVE` at a `.node` file to bypass platform resolution:

```bash
LUA_NATIVE=/absolute/path/lua.win32-x64-msvc.node lua version
```

This is useful when testing a local build before publishing a platform package.
