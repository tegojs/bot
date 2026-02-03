import { mkdir, readFile, writeFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const scriptDir = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(scriptDir, "../../..");
const botjsGenerated = resolve(repoRoot, "packages/botjs/generated/api.json");
const outputFile = resolve(
  repoRoot,
  "packages/bot-cli/src/generated/commands.ts",
);

const payload = JSON.parse(await readFile(botjsGenerated, "utf8")) as {
  exports: Array<{
    name: string;
    kind: string;
    params?: Array<{ name: string; type: string }>;
  }>;
};

const functions = payload.exports.filter((entry) => entry.kind === "function");

const lines: string[] = [];
lines.push("// Generated file. Do not edit manually.");
lines.push('import { Command } from "commander";');
lines.push('import { invokeExport } from "../commands/invoke.js";');
lines.push("");
lines.push('const BOOL_TRUE = new Set(["true", "1", "yes", "y", "on"]);');
lines.push('const BOOL_FALSE = new Set(["false", "0", "no", "n", "off"]);');
lines.push("function parseArg(value: string): unknown {");
lines.push("  try {");
lines.push("    return JSON.parse(value);");
lines.push("  } catch {");
lines.push("    if (/^0x[0-9a-f]+$/i.test(value)) {");
lines.push("      return Number.parseInt(value, 16);");
lines.push("    }");
lines.push("    const lower = value.toLowerCase();");
lines.push("    if (BOOL_TRUE.has(lower)) return true;");
lines.push("    if (BOOL_FALSE.has(lower)) return false;");
lines.push("    if (/^-?\\\\d+(?:\\\\.\\\\d+)?$/.test(value)) {");
lines.push("      return Number(value);");
lines.push("    }");
lines.push("    return value;");
lines.push("  }");
lines.push("}");
lines.push("");
lines.push(
  "export function registerGeneratedCommands(program: Command): void {",
);

for (const fn of functions) {
  const params = fn.params ?? [];
  const argList = params.map((param) => param.name);
  const safeArgs = argList.map((name, index) => {
    if (name === "options") return `optionsArg${index}`;
    return name;
  });
  lines.push(`  program.command("${fn.name}")`);
  lines.push(`    .description("Call @tego/botjs ${fn.name}")`);
  for (const param of argList) {
    lines.push(`    .argument("<${param}>", "${param}")`);
  }
  lines.push('    .option("--out <path>", "Write Buffer result to file")');
  lines.push('    .option("--json-output", "Emit result as JSON")');
  lines.push('    .option("--no-validate", "Disable arg shape validation")');
  lines.push('    .option("--dry-run", "Validate args without invoking")');
  lines.push(
    `    .action(async (${safeArgs.join(", ")}${safeArgs.length > 0 ? ", " : ""}options) => {`,
  );
  if (safeArgs.length > 0) {
    lines.push(
      `      const args = [${safeArgs.join(", ")}].map((value) => parseArg(String(value)));`,
    );
  } else {
    lines.push("      const args: unknown[] = [];");
  }
  lines.push(`      await invokeExport("${fn.name}", args, options);`);
  lines.push("    });");
  lines.push("");
}

lines.push("}");
lines.push("");

await mkdir(dirname(outputFile), { recursive: true });
await writeFile(outputFile, `${lines.join("\n")}\n`, "utf8");
