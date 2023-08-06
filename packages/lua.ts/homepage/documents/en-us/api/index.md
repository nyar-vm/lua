# API

All offsets are byte offsets into the source string.

## Parse

```ts
import { parse } from '@nyar/lua';

const outcome = parse('local x = 1');

outcome.ok;          // true when a tree was produced
outcome.error;       // fatal error message, or null
outcome.diagnostics; // non-fatal messages
outcome.tokenCount;  // number of leaves
outcome.root;        // SyntaxNode | null
```

`SyntaxNode`:

```ts
type SyntaxNode = {
    kind: string;
    start: number;
    end: number;
    children: SyntaxNode[];
};
```

## Tokens

```ts
import { tokens } from '@nyar/lua';

for (const token of tokens('local function f() end')) {
    console.log(token.kind, token.start, token.end);
}
```

## Diagnostics

```ts
import { check } from '@nyar/lua';

const messages = check('local x =');
for (const message of messages) console.error(message);
```

A fatal error, when present, is the first entry.

## Highlighting

```ts
import { highlight } from '@nyar/lua';

const spans = highlight('local function f() end');
// [{ start, end, kind: 'keyword' | 'string' | 'number' | 'comment' | 'macro' | 'identifier' | 'operator' }]
```

## Version

```ts
import { version } from '@nyar/lua';
console.log(version());
```
