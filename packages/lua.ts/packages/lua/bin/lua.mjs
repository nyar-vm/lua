#!/usr/bin/env node
import { runLuaCli } from '../dist/cli/lua-cli.js';

const code = await runLuaCli(process.argv.slice(2));
process.exit(code ?? 0);
