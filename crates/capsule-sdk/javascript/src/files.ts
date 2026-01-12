/**
 * Capsule Files API for WASM filesystem access.
 */

declare const globalThis: {
  'wasi:filesystem/types': any;
  'wasi:filesystem/preopens': any;
};

interface Descriptor {
  read(length: bigint, offset: bigint): [Uint8Array, boolean];
  write(buffer: Uint8Array, offset: bigint): bigint;
  stat(): { size: bigint };
  readDirectory(): any;
  openAt(
    pathFlags: { symlinkFollow?: boolean },
    path: string,
    openFlags: { create?: boolean; directory?: boolean; exclusive?: boolean; truncate?: boolean },
    descriptorFlags: { read?: boolean; write?: boolean; mutateDirectory?: boolean }
  ): Descriptor;
}

interface PreopenedDir {
  descriptor: Descriptor;
  guestPath: string;
}

function getFsBindings(): { types: any; preopens: any } | null {
  try {
    const types = globalThis['wasi:filesystem/types'];
    const preopens = globalThis['wasi:filesystem/preopens'];
    if (types && preopens) {
      return { types, preopens };
    }
  } catch {}
  return null;
}

function getPreopenedDirs(): PreopenedDir[] {
  const fs = getFsBindings();
  if (!fs) return [];

  try {
    const dirs = fs.preopens.getDirectories();
    return (dirs || []).map((entry: [Descriptor, string]) => ({
      descriptor: entry[0],
      guestPath: entry[1]
    }));
  } catch {
    return [];
  }
}

function normalizePath(path: string): string {
  if (path.startsWith('./')) {
    return path.slice(2);
  }
  return path;
}

function resolvePath(path: string): { dir: Descriptor; relativePath: string } | null {
  const preopens = getPreopenedDirs();
  if (preopens.length === 0) return null;

  const normalizedPath = normalizePath(path);

  for (const { descriptor, guestPath } of preopens) {
    const normalizedGuest = normalizePath(guestPath);

    if (normalizedGuest === '.' || normalizedGuest === '') {
      return { dir: descriptor, relativePath: normalizedPath };
    }

    if (normalizedPath.startsWith(normalizedGuest + '/')) {
      const relativePath = normalizedPath.slice(normalizedGuest.length + 1);
      return { dir: descriptor, relativePath };
    }

    if (normalizedPath === normalizedGuest) {
      return { dir: descriptor, relativePath: '.' };
    }
  }

  return { dir: preopens[0].descriptor, relativePath: normalizedPath };
}

/**
 * Read a file as text.
 *
 * @param path - The path to read.
 * @returns A promise that resolves to a string containing the file contents.
 */
export async function readText(path: string): Promise<string> {
  const bytes = await readBytes(path);
  return new TextDecoder().decode(bytes);
}

/**
 * Read a file as bytes.
 *
 * @param path - The path to read.
 * @returns A promise that resolves to a Uint8Array containing the file contents.
 */
export async function readBytes(path: string): Promise<Uint8Array> {
  const resolved = resolvePath(path);
  if (!resolved) {
    throw new Error("Filesystem not available.");
  }

  try {
    const pathFlags = { symlinkFollow: false };
    const openFlags = {};
    const descriptorFlags = { read: true };

    const fd = resolved.dir.openAt(pathFlags, resolved.relativePath, openFlags, descriptorFlags);
    const stat = fd.stat();
    const [data] = fd.read(stat.size, BigInt(0));
    return data;
  } catch (e) {
    throw new Error(`Failed to read file '${path}': ${e}`);
  }
}

/**
 * Write text content to a file.
 *
 * @param path - The path to write.
 * @param content - The text content to write.
 */
export async function writeText(path: string, content: string): Promise<void> {
  const bytes = new TextEncoder().encode(content);
  await writeBytes(path, bytes);
}

/**
 * Write bytes to a file.
 *
 * @param path - The path to write.
 * @param data - The bytes to write.
 */
export async function writeBytes(path: string, data: Uint8Array): Promise<void> {
  const resolved = resolvePath(path);
  if (!resolved) {
    throw new Error("Filesystem not available.");
  }

  try {
    const pathFlags = { symlinkFollow: false };
    const openFlags = { create: true, truncate: true };
    const descriptorFlags = { write: true };

    const fd = resolved.dir.openAt(pathFlags, resolved.relativePath, openFlags, descriptorFlags);
    fd.write(data, BigInt(0));
  } catch (e) {
    throw new Error(`Failed to write file '${path}': ${e}`);
  }
}

/**
 * List files/directories at a path.
 *
 * @param path - The path to list.
 * @returns A promise that resolves to an array of strings representing the files and directories at the specified path.
 */
export async function list(path: string = "."): Promise<string[]> {
  const resolved = resolvePath(path);
  if (!resolved) {
    throw new Error("Filesystem not available.");
  }

  try {
    let targetDir = resolved.dir;
    if (resolved.relativePath !== ".") {
      const pathFlags = { symlinkFollow: false };
      const openFlags = { directory: true };
      const descriptorFlags = { read: true };
      targetDir = resolved.dir.openAt(pathFlags, resolved.relativePath, openFlags, descriptorFlags);
    }

    const stream = targetDir.readDirectory();
    const entries: string[] = [];

    let entry;
    while ((entry = stream.readDirectoryEntry()) !== undefined && entry !== null) {
      if (entry.name) {
        entries.push(entry.name);
      }
    }

    return entries;
  } catch (e) {
    throw new Error(`Failed to list directory '${path}': ${e}`);
  }
}

/**
 * Check if a file or directory exists.
 *
 * @param path - The path to check.
 * @returns A promise that resolves to a boolean indicating whether the file or directory exists.
 */
export async function exists(path: string): Promise<boolean> {
  const resolved = resolvePath(path);
  if (!resolved) {
    return false;
  }

  try {
    const pathFlags = { symlinkFollow: false };
    const openFlags = {};
    const descriptorFlags = { read: true };
    resolved.dir.openAt(pathFlags, resolved.relativePath, openFlags, descriptorFlags);
    return true;
  } catch {
    return false;
  }
}
