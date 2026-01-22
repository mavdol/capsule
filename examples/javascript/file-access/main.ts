import { task, files } from "@capsule-run/sdk";

/**
 * Sub-task with restricted file access.
 * When called, it can ONLY access files in ./data directory
 */
export const restrictedWriter = task({
    name: "restricted_writer",
    allowedFiles: ["./data"]
}, async (content: string) => {
    await files.writeText("./data/output.txt", content);
    return { written: true };
});

/**
 * Main task has full project access by default.
 */
export const main = task({ name: "main" }, async () => {
    const content = await files.readText("./data/input.txt");
    const lines = content.trim().split("\n");
    const lineCount = lines.length;

    const result = await restrictedWriter(
        `Processed ${lineCount} lines\nFirst line: ${lines[0]}\n`
    );

    return {
        inputLines: lineCount,
        firstLine: lines[0],
        writeResult: result
    };
});
