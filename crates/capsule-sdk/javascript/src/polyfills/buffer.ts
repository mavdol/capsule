/**
 * Buffer polyfill for WASI environment
 * Provides Node.js-compatible Buffer class using the buffer package
 */

import { Buffer as BufferPolyfill } from 'buffer';

(globalThis as any).Buffer = BufferPolyfill;

export const Buffer = BufferPolyfill;
export default BufferPolyfill;
