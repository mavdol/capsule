import { task } from "@capsule-run/sdk";

export const main = task({
  name: "main",
  compute: "LOW",
  ram: "64MB",
  timeout: "10s"
}, () => {
  const code = process.argv[0];

  if (!code) {
    throw new Error("No code provided. Usage: capsule run capsule.ts \"<code>\"");
  }

  const fn = new Function(code);
  return fn();
});
