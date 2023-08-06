# 命令行

`lua` 命令是原生绑定的一层薄包装。它只解析 Lua，不执行 Lua。

## 命令

| 命令 | 用途 |
|------|------|
| `lua version` | 打印绑定版本 |
| `lua parse <file> [--json]` | 解析文件并输出语法树 |
| `lua tokens <file> [--json]` | 输出带字节跨距的词法单元 |
| `lua check <file...>` | 输出一个或多个文件的诊断 |
| `lua highlight <file> [--json]` | 输出高亮区间 |

## 示例

```bash
lua version
lua parse src/init.lua
lua parse src/init.lua --json
lua tokens src/init.lua
lua check src/*.lua
lua highlight src/init.lua
```

## 输出

- `parse` 打印 `parsed <file> (<n> token(s), <m> diagnostic(s))`，失败时打印致命错误。
  加 `--json` 输出完整的 `ParseOutcome`。
- `tokens` 每行打印 `kind`、`start`、`end`。加 `--json` 输出 `TokenSpan[]`。
- `check` 对每个文件打印 `ok` 或诊断，并输出一行汇总。
- `highlight` 每行打印 `kind`、`start`、`end`。加 `--json` 输出 `HighlightSpan[]`。

## 退出码

| 码 | 含义 |
|----|------|
| 0 | 成功 |
| 1 | 解析失败，或 `check` 报告了诊断 |
| 2 | 缺少输入或无法读取 |
