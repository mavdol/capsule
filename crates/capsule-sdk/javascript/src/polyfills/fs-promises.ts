/**
 * fs/promises polyfill for WASI environment
 * Allows: import fs from 'fs/promises'
 */

export { promises as default } from './fs.js';
export * from './fs.js';
