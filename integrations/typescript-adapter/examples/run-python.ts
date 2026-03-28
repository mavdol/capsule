import { runPython } from "@capsule-run/adapter";

console.log("Executing Python code inside Capsule sandbox...");

const result = await runPython(`
print("Hello from Python!")
x = 5 + 3
x * 2
`)

console.log("\n--- Sandbox Output ---");
console.log(result);
