/**
 * Capsule SDK - Task Registry and TaskRunner
 */

export interface TaskInfo<T extends (...args: any[]) => any> {
  func: T;
  config: TaskConfig;
}

export interface TaskConfig {
  name: string;
  compute?: string;
  ram?: string;
  timeout?: string;
  maxRetries?: number;
  allowedFiles?: string[];
  allowedHosts?: string[];
  envVariables?: string[];
}

const TASKS: Map<string, TaskInfo<any>> = new Map();

/**
 * Register a task function by name with its configuration.
 */
export function registerTask<T extends (...args: any[]) => any>(
  name: string,
  func: T,
  config: TaskConfig
): void {
  TASKS.set(name, { func, config });
}

/**
 * Get a registered task by name.
 */
export function getTask(name: string): ((...args: any[]) => any) | undefined {
  return TASKS.get(name)?.func;
}

/**
 * Get the configuration for a registered task.
 */
export function getTaskConfig(name: string): TaskConfig | undefined {
  return TASKS.get(name)?.config;
}

/**
 * Get all registered task names.
 */
export function getTaskNames(): string[] {
  return Array.from(TASKS.keys());
}

interface TaskArgs {
  task_name?: string;
  args?: any[];
  kwargs?: Record<string, any>;
}

/**
 * Implementation of the capsule:host/task-runner interface.
 *
 * This class is instantiated by capsule-core when the component is loaded.
 * The Rust host calls `run(argsJson)` to execute a task.
 */
export class TaskRunner {
  /**
   * Execute a task with the given arguments.
   * Returns Ok(result_json) on success, Err(error_message) on failure.
   */
  async run(argsJson: string): Promise<string> {
    try {
      const data: TaskArgs = JSON.parse(argsJson);
      const taskName = data.task_name ?? "main";
      const args = data.args ?? [];
      const kwargs = data.kwargs ?? {};

      let taskFunc = getTask(taskName);

      if (!taskFunc && taskName !== "main") {
        taskFunc = getTask("main");
      }

      if (!taskFunc && TASKS.size > 0) {
        const firstTaskName = TASKS.keys().next().value;
        if (firstTaskName) {
          taskFunc = getTask(firstTaskName);
        }
      }

      if (!taskFunc) {
        throw `No tasks or main() function found. Available tasks: ${getTaskNames().join(", ")}`;
      }

      const result = taskFunc(...args, kwargs);

      if (result instanceof Promise) {
        const asyncResult = await result;
        return JSON.stringify({ result: asyncResult ?? null });
      }

      return JSON.stringify({ result: result ?? null });
    } catch (e) {
      const errorMsg = e instanceof Error
        ? `${e.message}\n${e.stack || ''}`
        : String(e);
      throw errorMsg;
    }
  }
}

export const exports = new TaskRunner();
