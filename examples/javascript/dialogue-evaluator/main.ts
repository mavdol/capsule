import { task, files, env } from "@capsule-run/sdk";
import OpenAI from "openai";

const getDialogueLines = task({
    name: "getDialogueLines",
    compute: "LOW",
    allowedFiles: ["source"]
}, async (): Promise<{lines: string[]}> => {
    const dialogues = await files.readText('source/dialogue-lines.json')
    return JSON.parse(dialogues);
})

const insertDialogueLine = task({
    name: "insertDialogueLine",
    allowedFiles: ["output"]
}, async (line: string, evaluation: string): Promise<void> => {
    const output = await files.readText('output/evaluated-dialogue-lines.csv')
    const outputCsv = output.trim().split('\n');

    outputCsv.push(`"${line}",${evaluation}`);

    await files.writeText('output/evaluated-dialogue-lines.csv', outputCsv.join('\n'));
})

const evaluateDialogueLine = task({
    name: "evaluateDialogueLine",
    envVariables: ["OPENAI_API_KEY"]
}, async (prompt: string): Promise<string> => {

    const openai = new OpenAI({
        baseURL: 'YOUR BASE URL',
        apiKey: env.get('OPENAI_API_KEY'),
    });

    const completion = await openai.chat.completions.create({
        messages: [{ role: "user", content: prompt }],
        model: "YOUR MODEL",
    });

    return completion.choices[0].message.content || "";
})

const agent = task({
    name: "agent",
    compute: "HIGH",
    maxRetries: 2,
    envVariables: ["OPENAI_API_KEY"]
}, async (line: string) => {
    const prompt = `
        Analyze the emotional tone of the following dialogue line: "${line}"

        Respond with a single lowercase word representing the emotion (e.g., neutral, angry, happy).
        Do not include any explanation.

        Examples:
        "What's in the suitcase?" -> "neutral"
        "You're out of your mind." -> "angry"
        "I'll buy it off you." -> "happy"
    `;

    const evaluateDialogueLineResponse = await evaluateDialogueLine(prompt);
    await insertDialogueLine(line, evaluateDialogueLineResponse.result);
    return;
})

// entrypoint
task({name: "main", compute: "HIGH"}, async () => {

    const getDialogueLinesResponse = await getDialogueLines();
    const lines = getDialogueLinesResponse.result.lines;

    const failedAttempts = [];
    let successfulAttempts = 0;

    for (const [index, line] of lines.entries()) {
        const agentResponse = await agent(line);

        if(!agentResponse.success) {
            if(failedAttempts.length == 3) {
                break;
            }

            failedAttempts.push(line);
            continue;
        }

        successfulAttempts++;

        if(successfulAttempts == 3) {
            break;
        }

        console.log(`Agent-${index} successfully evaluated line`);
    }

    console.log(
        "\n ----------\n",
        `Agent created: ${lines.length}\n`,
        `Successful attempts: ${successfulAttempts}\n`,
        `Failed attempts: ${failedAttempts.length}\n`,
        "----------\n"
    )

    return;
})
