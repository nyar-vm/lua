# API

所有偏移都是源码字符串中的字节偏移。

## 解析

```ts
import { parse } from '@nyar/lua';

const outcome = parse('local x = 1');

outcome.ok;          // 产出语法树时为 true
outcome.error;       // 致命错误信息，或 null
outcome.diagnostics; // 非致命信息
outcome.tokenCount;  // 叶子数量
outcome.root;        // SyntaxNode | null
```

`SyntaxNode`：

```ts
type SyntaxNode = {
    kind: string;
    start: number;
    end: number;
    children: SyntaxNode[];
};
```

## 词法单元

```ts
import { tokens } from '@nyar/lua';

for (const token of tokens('local function f() end')) {
    console.log(token.kind, token.start, token.end);
}
```

## 诊断

```ts
import { check } from '@nyar/lua';

const messages = check('local x =');
for (const message of messages) console.error(message);
```

如果存在致命错误，它会是第一条。

## 高亮

```ts
import { highlight } from '@nyar/lua';

const spans = highlight('local function f() end');
// [{ start, end, kind: 'keyword' | 'string' | 'number' | 'comment' | 'macro' | 'identifier' | 'operator' }]
```

## 版本

```ts
import { version } from '@nyar/lua';
console.log(version());
```
