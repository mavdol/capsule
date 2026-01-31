/**
 * Process polyfill for WASI environment
 * Provides Node.js-compatible process object with WASI-backed values
 */

declare const globalThis: {
    'wasi:cli/environment': {
        getEnvironment(): [string, string][];
        getArguments(): string[];
        initialCwd(): string | null;
    };
};

/**
 * Internal helper to safely get the WASI environment bindings.
 */
function getEnvBindings() {
    try {
        const env = globalThis['wasi:cli/environment'];

        if (env && typeof env.getEnvironment === 'function') {
            return env;
        }
    } catch {}
    return null;
}

/**
 * Lazy-loaded environment variables from WASI
 */
function getEnv(): Record<string, string> {
    const bindings = getEnvBindings();
    if (!bindings) {
        return {};
    }

    try {
        const envPairs = bindings.getEnvironment();
        return Object.fromEntries(envPairs);
    } catch (e) {
        console.error("Failed to fetch environment variables:", e);
        return {};
    }
}

/**
 * Lazy-loaded command-line arguments from WASI
 */
function getArgv(): string[] {
    const bindings = getEnvBindings();
    if (!bindings || typeof bindings.getArguments !== 'function') {
        return ['capsule'];
    }

    try {
        return bindings.getArguments();
    } catch {
        return ['capsule'];
    }
}

/**
 * Get current working directory from WASI
 */
function getCwd(): string {
    const bindings = getEnvBindings();
    if (!bindings || typeof bindings.initialCwd !== 'function') {
        return '/';
    }

    try {
        const cwd = bindings.initialCwd();
        return cwd || '/';
    } catch {
        return '/';
    }
}

const process = {
    /**
     * Environment variables
     */
    get env(): Record<string, string> {
        return getEnv();
    },

    /**
     * Command-line arguments
     */
    get argv(): string[] {
        return getArgv();
    },

    /**
     * Returns the current working directory
     */
    cwd(): string {
        return getCwd();
    },

    /**
     * Change directory (not supported in WASI)
     */
    chdir(directory: string): void {
        throw new Error('process.chdir() is not supported in WASI environment');
    },

    /**
     * Platform identifier
     */
    get platform(): 'linux' {
        return 'linux';
    },

    /**
     * Architecture identifier
     */
    get arch(): 'wasm32' {
        return 'wasm32';
    },

    /**
     * Process ID (not available in WASI)
     */
    get pid(): number {
        return 1;
    },

    /**
     * Parent process ID (not available in WASI)
     */
    get ppid(): number {
        return 0;
    },

    /**
     * Process uptime in seconds
     */
    uptime(): number {
        return 0;
    },

    /**
     * Node.js version (WASI doesn't have Node.js)
     */
    get version(): string {
        return 'v0.0.0-wasi';
    },

    /**
     * Node.js versions object
     */
    get versions(): Record<string, string> {
        return {
            wasi: '0.2.0',
        };
    },

    /**
     * Exit the process (not fully supported in WASI)
     */
    exit(code?: number): never {
        throw new Error(`Process exit requested with code ${code ?? 0}`);
    },

    /**
     * Abort the process
     */
    abort(): never {
        throw new Error('Process aborted');
    },

    /**
     * Queue a microtask (uses native queueMicrotask)
     */
    nextTick(callback: (...args: any[]) => void, ...args: any[]): void {
        queueMicrotask(() => callback(...args));
    },

    /**
     * Standard streams (not fully implemented)
     */
    get stdout(): any {
        return {
            write: (data: string) => console.log(data),
            isTTY: false,
        };
    },

    get stderr(): any {
        return {
            write: (data: string) => console.error(data),
            isTTY: false,
        };
    },

    get stdin(): any {
        return {
            isTTY: false,
        };
    },

    /**
     * Memory usage (not available in WASI)
     */
    memoryUsage(): {
        rss: number;
        heapTotal: number;
        heapUsed: number;
        external: number;
        arrayBuffers: number;
    } {
        return {
            rss: 0,
            heapTotal: 0,
            heapUsed: 0,
            external: 0,
            arrayBuffers: 0,
        };
    },

    /**
     * CPU usage (not available in WASI)
     */
    cpuUsage(previousValue?: { user: number; system: number }): {
        user: number;
        system: number;
    } {
        return {
            user: 0,
            system: 0,
        };
    },

    /**
     * High-resolution time
     */
    hrtime: {
        bigint(): bigint {
            return BigInt(Date.now() * 1000000);
        },
    },

    /**
     * Execute path
     */
    get execPath(): string {
        return '/usr/bin/capsule';
    },

    /**
     * Execution arguments
     */
    get execArgv(): string[] {
        return [];
    },

    /**
     * Title (not settable in WASI)
     */
    get title(): string {
        return 'capsule';
    },

};

export default process;

export const {
    env,
    argv,
    cwd,
    platform,
    arch,
    pid,
    ppid,
    version,
    versions,
    exit,
    nextTick,
    stdout,
    stderr,
    stdin,
} = process;

(globalThis as any).process = process;
