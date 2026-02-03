import * as fs from "node:fs/promises";
import { stdin as stdinStream } from "node:process";
import { invokeExport } from "./invoke.js";

export interface CallOptions {
  json?: string;
  file?: string;
  stdin?: boolean;
  out?: string;
  jsonOutput?: boolean;
  validate?: boolean;
  dryRun?: boolean;
}

async function readStdin(): Promise<string> {
  const chunks: Buffer[] = [];
  for await (const chunk of stdinStream) {
    chunks.push(Buffer.isBuffer(chunk) ? chunk : Buffer.from(chunk));
  }
  return Buffer.concat(chunks).toString("utf8");
}

async function loadArgs(options: CallOptions): Promise<unknown> {
  if (options.json) {
    return JSON.parse(options.json);
  }
  if (options.file) {
    const contents = await fs.readFile(options.file, "utf8");
    return JSON.parse(contents);
  }
  if (options.stdin) {
    const contents = await readStdin();
    return JSON.parse(contents);
  }
  return [];
}

export async function callCommand(
  exportName: string,
  options: CallOptions,
): Promise<void> {
  const args = await loadArgs(options);
  const normalizedArgs = Array.isArray(args) ? args : [args];
  if (options.dryRun) {
    await invokeExport(exportName, normalizedArgs, {
      ...options,
      out: undefined,
    });
    return;
  }
  await invokeExport(exportName, normalizedArgs, options);
}
