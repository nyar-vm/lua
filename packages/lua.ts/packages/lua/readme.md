# @nyar/lua

Lua language services for Node.js: syntax trees, tokens, diagnostics and
highlight spans over a native Node-API binding. It is read-only — you give it
source text, it gives you structured results. Execution belongs to the Lua
runtime, not to this package.

## Usage

```bash
npx @nyar/lua version
npx @nyar/lua parse path/to/file.lua --json
npx @nyar/lua tokens path/to/file.lua
npx @nyar/lua check src/*.lua
npx @nyar/lua highlight path/to/file.lua
```

## API

```ts
import { parse, tokens, check, highlight } from '@nyar/lua';

const outcome = parse('local x = 1');
console.log(outcome.ok, outcome.tokenCount);
```

The matching platform package is installed automatically.
