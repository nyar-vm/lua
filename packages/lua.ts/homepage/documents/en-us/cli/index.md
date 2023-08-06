# CLI

The `lua` command is a thin wrapper over the native binding. It parses Lua; it
does not execute it.

## Commands

| Command | Purpose |
|---------|---------|
| `lua version` | Print the binding version |
| `lua parse <file> [--json]` | Parse a file and report the syntax tree |
| `lua tokens <file> [--json]` | List tokens with byte spans |
| `lua check <file...>` | Report diagnostics for one or more files |
| `lua highlight <file> [--json]` | List highlight spans |

## Examples

```bash
lua version
lua parse src/init.lua
lua parse src/init.lua --json
lua tokens src/init.lua
lua check src/*.lua
lua highlight src/init.lua
```

## Output

- `parse` prints `parsed <file> (<n> token(s), <m> diagnostic(s))`, or the fatal
  error. With `--json` it prints the full `ParseOutcome`.
- `tokens` prints `kind`, `start`, `end` per line. With `--json` it prints
  `TokenSpan[]`.
- `check` prints `ok` or the diagnostics per file and a summary line.
- `highlight` prints `kind`, `start`, `end` per line. With `--json` it prints
  `HighlightSpan[]`.

## Exit codes

| Code | Meaning |
|------|---------|
| 0 | success |
| 1 | parse failed or diagnostics were reported by `check` |
| 2 | missing or unreadable input |
