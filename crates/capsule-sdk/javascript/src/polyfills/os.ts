interface CpuInfo {
    model: string;
    speed: number;
    times: {
        user: number;
        nice: number;
        sys: number;
        idle: number;
        irq: number;
    };
}

interface NetworkInterface {
    address: string;
    netmask: string;
    family: string;
    mac: string;
    internal: boolean;
    cidr: string | null;
}

const os = {
    /**
     * Returns the operating system platform
     * WASI is POSIX-like, so we return 'linux'
     */
    platform(): 'aix' | 'darwin' | 'freebsd' | 'linux' | 'openbsd' | 'sunos' | 'win32' {
        return 'linux';
    },

    /**
     * Returns the CPU architecture
     * WASM runs on wasm32 architecture
     */
    arch(): string {
        return 'wasm32';
    },

    /**
     * Returns the operating system's default directory for temporary files
     */
    tmpdir(): string {
        return '/tmp';
    },

    /**
     * Returns the home directory of the current user
     */
    homedir(): string {
        return '/home/capsule';
    },

    /**
     * Returns the system uptime in seconds
     * Not available in WASI, returns 0
     */
    uptime(): number {
        return 0;
    },

    /**
     * Returns the amount of free system memory in bytes
     * Not available in WASI, returns 0
     */
    freemem(): number {
        return 0;
    },

    /**
     * Returns the total amount of system memory in bytes
     * Not available in WASI, returns 0
     */
    totalmem(): number {
        return 0;
    },

    /**
     * Returns an array of objects containing information about each CPU/core
     * Not available in WASI, returns empty array
     */
    cpus(): CpuInfo[] {
        return [];
    },

    /**
     * Returns the hostname of the operating system
     */
    hostname(): string {
        return 'capsule-wasm';
    },

    /**
     * Returns the operating system release
     */
    release(): string {
        return 'wasi';
    },

    /**
     * Returns the operating system type
     */
    type(): string {
        return 'WASI';
    },

    /**
     * Returns the endianness of the CPU
     * WASM is little-endian
     */
    endianness(): 'BE' | 'LE' {
        return 'LE';
    },

    /**
     * Returns an object containing network interfaces
     * Not available in WASI, returns empty object
     */
    networkInterfaces(): { [index: string]: NetworkInterface[] } {
        return {};
    },

    /**
     * Returns the operating system's default directory for user data
     */
    userInfo(options?: { encoding: 'buffer' }): {
        username: string;
        uid: number;
        gid: number;
        shell: string | null;
        homedir: string;
    } {
        return {
            username: 'capsule',
            uid: 1000,
            gid: 1000,
            shell: null,
            homedir: '/home/capsule',
        };
    },

    /**
     * Returns an array of objects containing information about the load average
     * Not available in WASI, returns [0, 0, 0]
     */
    loadavg(): number[] {
        return [0, 0, 0];
    },

    /**
     * Returns the operating system's version
     */
    version(): string {
        return 'WASI 0.2';
    },

    /**
     * Returns the machine type
     */
    machine(): string {
        return 'wasm32';
    },

    /**
     * Platform-specific path segment separator
     */
    get EOL(): string {
        return '\n';
    },

    /**
     * Platform-specific constants
     */
    constants: {
        signals: {},
        errno: {},
        priority: {},
    },

    /**
     * Returns the number of logical CPU cores
     * Not available in WASI, returns 1
     */
    availableParallelism(): number {
        return 1;
    },

    /**
     * Returns the string path of the current user's home directory
     */
    devNull: '/dev/null',
};

export default os;

// Named exports for destructuring
export const {
    platform,
    arch,
    tmpdir,
    homedir,
    uptime,
    freemem,
    totalmem,
    cpus,
    hostname,
    release,
    type,
    endianness,
    networkInterfaces,
    userInfo,
    loadavg,
    version,
    machine,
    EOL,
    constants,
    availableParallelism,
    devNull,
} = os;
