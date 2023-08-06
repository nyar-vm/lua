# @nyar/lua-harness

Diagnostics and tuning harness for `@nyar/lua`. It links the same native binding
through the platform optional dependencies, so a failing harness points at the same
addon the product CLI uses.

## Usage

```bash
lua-harness version
lua-harness check src/*.lua
lua-harness bench path/to/file.lua --iterations 200
lua-harness tokens path/to/file.lua
```
