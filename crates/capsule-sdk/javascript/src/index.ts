/**
 * Capsule SDK for JavaScript/TypeScript
 *
 * @example
 * ```typescript
 * import { task } from '@capsule/sdk';
 *
 * export const greet = task({
 *   name: "greet",
 *   compute: "LOW"
 * }, (name: string): string => {
 *   return `Hello, ${name}!`;
 * });
 * ```
 */

export { task, type TaskOptions } from "./task.js";
export { TaskRunner, exports, type TaskConfig } from "./app.js";
export * as files from "./files.js";
export * as env from "./env.js";
export { isWasmMode } from "./hostApi.js";

