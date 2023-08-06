#!/usr/bin/env node
import { runLuaHarnessCli } from '../dist/cli.js';

const code = await runLuaHarnessCli(process.argv.slice(2));
process.exit(code ?? 0);
