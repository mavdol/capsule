import { createTool } from '@mastra/core/tools';
import { execSync } from 'child_process';
import { z } from 'zod';
import { dirname, resolve } from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const PROJECT_ROOT = resolve(__dirname, '../../..');

interface Result {
  success: boolean;
  result: unknown;
  error: { error_type: string; message: string } | null;
  execution: {
    task_name: string;
    duration_ms: number;
    retries: number;
    fuel_consumed: number;
  };
}

export const executeCode = createTool({
  id: 'execute-code',
  description: 'Safely execute JavaScript code in an isolated Capsule sandbox. The code should include a return statement to produce output.',
  inputSchema: z.object({
    code: z.string().describe('JavaScript code to execute. Must include a return statement, e.g. "return 1 + 2"'),
  }),
  outputSchema: z.object({
    success: z.boolean(),
    result: z.any(),
    error: z.string().nullable(),
    duration_ms: z.number(),
  }),
  execute: async ({ code }: { code: string }) => {
    try {
      const capsulePath = resolve(PROJECT_ROOT, 'capsule.ts');

      const result = execSync(`capsule run "${capsulePath}" "${code}" --json`, {
        encoding: 'utf-8'
      }).toString();

      const parsed: Result = JSON.parse(result);

      return {
        success: parsed.success,
        result: parsed.result,
        error: parsed.error ? parsed.error.message : null,
        duration_ms: parsed.execution.duration_ms,
      };
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      return {
        success: false,
        result: null,
        error: message,
        duration_ms: 0,
      };
    }
  },
});
