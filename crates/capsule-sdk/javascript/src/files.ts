/**
 * Capsule Files API for WASM filesystem access.
 *
 * We provide this API for reading/writing files in allowed directories.
 *
 * @example
 * ```typescript
 * import { files } from '@capsule-run/sdk';
 *
 * const content = await files.readText("./data/input.txt");
 * await files.writeText("./data/output.txt", content);
 * ```
 */

declare const globalThis: {
  'wasi:filesystem/types': any;
  'wasi:filesystem/preopens': any;
};

interface Descriptor {
  read(length: bigint, offset: bigint): [Uint8Array, boolean];
  write(buffer: Uint8Array, offset: bigint): bigint;
  stat(): { size: bigint };
  readDirectory(): Array<{ name: string; type: string }>;
  openAt(pathFlags: number, path: string, openFlags: number, descriptorFlags: number): Descriptor;
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

function getPreopenedDir(): Descriptor | null {
  const fs = getFsBindings();
  if (!fs) return null;

  try {
    const dirs = fs.preopens.getDirectories();
    if (dirs && dirs.length > 0) {
      return dirs[0][0];
    }
  } catch {}
  return null;
}

/**
 * Read a file as text.
 * @param path - Relative path to the file (must be in allowed_files)
 */
export async function readText(path: string): Promise<string> {
  const bytes = await readBytes(path);
  return new TextDecoder().decode(bytes);
}

/**
 * Read a file as bytes.
 * @param path - Relative path to the file (must be in allowed_files)
 */
export async function readBytes(path: string): Promise<Uint8Array> {
  const dir = getPreopenedDir();
  if (!dir) {
    throw new Error("Filesystem not available. Make sure you've set allowedFiles in your task config.");
  }

  try {
    const fd = dir.openAt(0, path, 0, 1);
    const stat = fd.stat();
    const [data] = fd.read(stat.size, BigInt(0));
    return data;
  } catch (e) {
    throw new Error(`Failed to read file '${path}': ${e}`);
  }
}

/**
 * Write text content to a file.
 * @param path - Relative path to the file (must be in allowed_files)
 * @param content - Text content to write
 */
export async function writeText(path: string, content: string): Promise<void> {
  const bytes = new TextEncoder().encode(content);
  await writeBytes(path, bytes);
}

/**
 * Write bytes to a file.
 * @param path - Relative path to the file (must be in allowed_files)
 * @param data - Binary data to write
 */
export async function writeBytes(path: string, data: Uint8Array): Promise<void> {
  const dir = getPreopenedDir();
  if (!dir) {
    throw new Error("Filesystem not available. Make sure you've set allowedFiles in your task config.");
  }

  try {
    const fd = dir.openAt(0, path, 1 | 2, 2);
    fd.write(data, BigInt(0));
  } catch (e) {
    throw new Error(`Failed to write file '${path}': ${e}`);
  }
}

/**
 * List files/directories at a path.
 * @param path - Relative path to the directory (defaults to ".")
 */
export async function list(path: string = "."): Promise<string[]> {
  const dir = getPreopenedDir();
  if (!dir) {
    throw new Error("Filesystem not available. Make sure you've set allowedFiles in your task config.");
  }

  try {
    const targetDir = path === "." ? dir : dir.openAt(0, path, 0, 1);
    const entries = targetDir.readDirectory();
    return entries.map((entry: { name: string }) => entry.name);
  } catch (e) {
    throw new Error(`Failed to list directory '${path}': ${e}`);
  }
}

/**
 * Check if a file or directory exists.
 * @param path - Relative path to check
 */
export async function exists(path: string): Promise<boolean> {
  const dir = getPreopenedDir();
  if (!dir) {
    return false;
  }

  try {
    dir.openAt(0, path, 0, 1);
    return true;
  } catch {
    return false;
  }
}
