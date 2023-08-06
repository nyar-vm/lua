# Lua 语言服务

`@nyar/lua` 通过原生 Node-API 插件，把 Lua 解析能力带到 Node.js。它是只读的：
输入源码文本，输出语法树、词法单元、诊断信息与高亮区间。它不执行 Lua。

## 能力

| 导出 | 返回 |
|------|------|
| `parse(source)` | `{ ok, error, diagnostics, root, tokenCount }` |
| `tokens(source)` | 带字节偏移的 `TokenSpan[]` |
| `check(source)` | `string[]` 诊断信息 |
| `highlight(source)` | 关键字/字符串/数字/注释/宏/标识符/运算符区间 |
| `version()` | 原生绑定版本 |

## 安装

```bash
npm install @nyar/lua
```

会自动安装与当前平台匹配的原生包。

## 命令行

```bash
lua version
lua parse path/to/file.lua --json
lua tokens path/to/file.lua
lua check src/*.lua
lua highlight path/to/file.lua
```

## 下一步

- [命令行](./cli.md)
- [API](./api.md)
- [平台与故障排查](./native.md)
