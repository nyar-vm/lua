import { createRequire } from 'node:module';
import { existsSync } from 'node:fs';
import { arch, platform } from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

/** Kind and byte span of a lexical or syntax leaf. */
export type TokenSpan = {
    kind: string;
    start: number;
    end: number;
};

/** Syntax tree node. */
export type SyntaxNode = {
    kind: string;
    start: number;
    end: number;
    children: SyntaxNode[];
};

/** Result of `luaParse`. */
export type ParseOutcome = {
    ok: boolean;
    error: string | null;
    diagnostics: string[];
    root: SyntaxNode | null;
    tokenCount: number;
};

/** Highlight span. */
export type HighlightSpan = {
    start: number;
    end: number;
    kind: 'keyword' | 'string' | 'number' | 'comment' | 'macro' | 'identifier' | 'operator' | string;
};

/** Structured capability surface exported by `lua-napi`. */
type LuaNativeAddon = {
    luaVersion(): string;
    luaParse(source: string): ParseOutcome;
    luaTokens(source: string): TokenSpan[];
    luaCheck(source: string): string[];
    luaHighlight(source: string): HighlightSpan[];
};

const pkgRoot = path.dirname(fileURLToPath(import.meta.url));
const require = createRequire(import.meta.url);

/** Host triple used for platform-named native binaries (`lua.<triple>.node`). */
export function platformTriple(): string {
    const os = platform();
    const cpu = arch();
    if (os === 'win32' && cpu === 'x64') return 'win32-x64-msvc';
    if (os === 'linux' && cpu === 'x64') return 'linux-x64-gnu';
    if (os === 'darwin' && cpu === 'arm64') return 'darwin-arm64';
    if (os === 'darwin' && cpu === 'x64') return 'darwin-x64';
    return `${os}-${cpu}`;
}

/** npm platform short id (`win32-x64`, …) from host OS. */
export function platformShort(triple: string = platformTriple()): string {
    if (triple.startsWith('win32-')) return 'win32-x64';
    if (triple.startsWith('linux-')) return 'linux-x64';
    if (triple === 'darwin-arm64') return 'darwin-arm64';
    if (triple === 'darwin-x64') return 'darwin-x64';
    return triple;
}

/** Host triple used for platform-named native binaries. */
export function nativeBinaryName(triple: string = platformTriple()): string {
    return `lua.${triple}.node`;
}

/** npm platform id for the current host OS/arch. */
export function hostPlatformKey(): string {
    return `${platform()}-${arch()}`;
}

/**
 * Resolve native `.node` via the platform optionalDependency (`@nyar/lua-<short>`).
 *
 * `LUA_NATIVE` overrides resolution: used to validate a freshly built addon.
 */
export function resolveNativePath(): string {
    const override = process.env.LUA_NATIVE;
    if (override && override.trim().length > 0) return override;
    const triple = platformTriple();
    const short = platformShort(triple);
    const packageName = `@nyar/lua-${short}`;
    const binary = nativeBinaryName(triple);
    const candidates: string[] = [];

    try {
        const pkgJson = require.resolve(`${packageName}/package.json`);
        const pkgDir = path.dirname(pkgJson);
        candidates.push(path.join(pkgDir, 'lib', binary));
        candidates.push(path.join(pkgDir, binary));
    } catch {
        /* optional dep not installed */
    }

    // dist/native.js → package root → packages → lua.ts → platforms
    candidates.push(path.join(pkgRoot, '..', '..', '..', 'platforms', `lua-${short}`, 'lib', binary));
    candidates.push(path.join(pkgRoot, '..', '..', '..', 'platforms', `lua-${short}`, binary));
    candidates.push(path.join(pkgRoot, '..', '..', '..', '..', 'node_modules', packageName, 'lib', binary));
    candidates.push(path.join(pkgRoot, '..', '..', '..', '..', 'node_modules', packageName, binary));

    for (const candidate of candidates) {
        if (existsSync(candidate)) return candidate;
    }

    throw new Error(
        `lua native addon not found for ${packageName}. Run: node scripts/build.mjs napi\n` +
        `Looked in:\n${candidates.map((c) => `  - ${c}`).join('\n')}\n` +
        `(host ${hostPlatformKey()})`,
    );
}

let nativeAddon: LuaNativeAddon | undefined;

/** Load the lua-napi binding for the current platform. */
export function loadNative(): LuaNativeAddon {
    if (nativeAddon) return nativeAddon;
    nativeAddon = require(resolveNativePath()) as LuaNativeAddon;
    return nativeAddon;
}

/** Binding version. */
export function version(): string {
    return loadNative().luaVersion();
}

/** Parse Lua source and return the syntax tree. */
export function parse(source: string): ParseOutcome {
    return loadNative().luaParse(source);
}

/** Lexical leaves. */
export function tokens(source: string): TokenSpan[] {
    return loadNative().luaTokens(source);
}

/** Diagnostics. */
export function check(source: string): string[] {
    return loadNative().luaCheck(source);
}

/** Keyword/string/number/comment/macro/identifier/operator spans. */
export function highlight(source: string): HighlightSpan[] {
    return loadNative().luaHighlight(source);
}
