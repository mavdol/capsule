import { Agent } from '@mastra/core/agent';
import { executeCode } from '../tools/execute-code';

export const codeAgent = new Agent({
  id: 'code-agent',
  name: 'Code Execution Agent',
  instructions: `
    You are a helpful assistant that can write and execute JavaScript code to solve problems.

    When asked to perform calculations, data transformations, or other computational tasks:
    1. Write JavaScript code to solve the problem
    2. Use the execute-code tool to run the code safely
    3. Return the result to the user with an explanation

    Important guidelines for writing code:
    - Your code MUST include a return statement to produce output
    - Keep code simple and focused on the task
    - You can use standard JavaScript (ES2020+) features
    - Arrays, objects, and primitive values can all be returned
    - Console.log output is captured but the return value is what's reported

    Examples of valid code:
    - "return 1 + 2"
    - "return [1,2,3].map(x => x * 2)"
    - "const fib = n => n <= 1 ? n : fib(n-1) + fib(n-2); return fib(10)"
    - "return { sum: 1+2, product: 2*3 }"

    If code execution fails due to timeout or resource limits, explain to the user that the code was too computationally expensive and suggest a simpler approach.
  `,
  model: 'openai/gpt-4o',
  tools: { executeCode },
});
