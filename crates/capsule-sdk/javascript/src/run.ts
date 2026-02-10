/**
 * Capsule SDK - SDK Runner
 *
 * Provides the `run` function for running Capsule tasks
 * from third party applications.
 */

import { execFile } from 'child_process';
import { resolve } from 'path';

export interface RunnerOptions {
  file: string;
  args?: string[];
  cwd?: string;
  capsulePath?: string;
}

export interface RunnerResult {
  success: boolean;
  result: string | number | boolean | object | null;
  error: { error_type: string; message: string } | null;
  execution: {
    task_name: string;
    duration_ms: number;
    retries: number;
    fuel_consumed: number;
  };
}

/**
 * Run a Capsule task from a third-party application
 *
 * @param options - Runner options
 * @returns Promise with the runner result
 */
export function run(options: RunnerOptions): Promise<RunnerResult> {
  const { file, args = [], cwd, capsulePath = 'capsule' } = options;

  const resolvedFile = resolve(cwd || process.cwd(), file);

  return new Promise((resolve, reject) => {
    const cmdArgs = ['run', resolvedFile, '--json', ...args];

    execFile(capsulePath, cmdArgs, { cwd, encoding: 'utf-8' }, (error, stdout, stderr) => {
      if (error && !stdout) {
        if ((error as NodeJS.ErrnoException).code === 'ENOENT') {
          reject(new Error(
            `Capsule CLI not found. Use 'npm install -g @capsule-run/cli' to install it.`
          ));
          return;
        }
        reject(new Error(stderr || error.message));
        return;
      }

      try {
        resolve(JSON.parse(stdout));
      } catch {
        reject(new Error(`Failed to parse Capsule output: ${stdout}`));
      }
    });
  });
}
