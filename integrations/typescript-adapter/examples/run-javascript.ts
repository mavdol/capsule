import { runJavaScript } from "@capsule-run/adapter";

console.log("Executing JavaScript code inside Capsule sandbox...");

const result = await runJavaScript(`
console.log("Hello from JavaScript!")
let x = 5 + 3
x * 2
`)

console.log("\n--- Sandbox Output ---");
console.log(result);
