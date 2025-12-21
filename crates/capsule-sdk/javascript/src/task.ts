/**
 * Capsule SDK - Task Wrapper
 *
 * Provides the `task` wrapper function for defining Capsule tasks
 * in an idiomatic TypeScript way.
 */

import { registerTask, type TaskConfig } from "./app.js";
import { isWasmMode, callHost } from "./hostApi.js";

export interface TaskOptions {
  /** Task name (required) */
  name: string;
  /** Compute level: "LOW", "MEDIUM", or "HIGH" */
  compute?: "LOW" | "MEDIUM" | "HIGH" | number;
  /** RAM limit, e.g., "512MB", "2GB" */
  ram?: string;
  /**
   * Timeout duration.
   * String format: "30s", "5m", "2h" (e.g., "10s")
   * Number format: milliseconds (e.g., 10000 for 10 seconds)
   */
  timeout?: string | number;
  /** Maximum number of retries */
  maxRetries?: number;
  /** Environment variables */
  envVars?: Record<string, string>;
}

interface TaskResult {
  result?: any;
  error?: string;
}

/**
 * Define a Capsule task with configuration.
 *
 * @example
 * ```typescript
 * // String timeout format
 * export const greet = task({
 *   name: "greet",
 *   compute: "LOW",
 *   timeout: "10s"
 * }, (name: string): string => {
 *   return `Hello, ${name}!`;
 * });
 *
 * // Numeric timeout format (milliseconds)
 * export const process = task({
 *   name: "process",
 *   compute: "HIGH",
 *   timeout: 30000  // 30 seconds
 * }, (data: any): any => {
 *   return processData(data);
 * });
 * ```
 *
 * In WASM mode:
 * - The function is registered in the task registry with its config
 * - When called from within a task, it schedules a new isolated instance
 * - The host creates a new Wasm instance with resource limits
 *
 * In non-WASM mode:
 * - The function executes directly
 */

/**
 * Normalize timeout to string format for the Rust host.
 *
 * @param timeout - String duration ("10s", "5m") or number (milliseconds)
 * @returns String format for host API, or undefined
 */
function normalizeTimeout(timeout?: string | number): string | undefined {
  if (timeout === undefined) return undefined;
  if (typeof timeout === "number") {
    return `${timeout}ms`;
  }
  return timeout;
}

export function task<TArgs extends any[], TReturn>(
  options: TaskOptions,
  fn: (...args: TArgs) => TReturn
): (...args: TArgs) => TReturn {
  const taskName = options.name;
  let compute = options.compute?.toString().toUpperCase() ?? "MEDIUM";

  const taskConfig: TaskConfig = {
    name: taskName,
    compute,
    ram: options.ram,
    timeout: normalizeTimeout(options.timeout),
    maxRetries: options.maxRetries,
    envVars: options.envVars,
  };

  const wrapper = (...args: TArgs): TReturn => {
    if (!isWasmMode()) {
      return fn(...args);
    }

    const resultJson = callHost(taskName, args, taskConfig);

    try {
      const result: TaskResult = JSON.parse(resultJson);
      if (result.error) {
        throw new Error(`Task ${taskName} failed: ${result.error}`);
      }
      return result.result as TReturn;
    } catch (e) {
      if (e instanceof SyntaxError) {
        return resultJson as unknown as TReturn;
      }
      throw e;
    }
  };

  registerTask(taskName, fn, taskConfig);

  return wrapper;
}
