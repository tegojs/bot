/**
 * Script execution
 */

import { createRequire } from "node:module";
import path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

export interface ExecutionResult {
  success: boolean;
  stdout: string;
  stderr: string;
  error?: Error;
}

/**
 * Execute a TypeScript script using tsx
 */
export async function executeScript(
  scriptPath: string,
): Promise<ExecutionResult> {
  const absolutePath = path.resolve(scriptPath);

  let stdout = "";
  let stderr = "";

  // Capture stdout
  const originalStdoutWrite = process.stdout.write.bind(process.stdout);
  process.stdout.write = ((chunk: any, ...args: any[]) => {
    const text = chunk.toString();
    stdout += text;
    return originalStdoutWrite(chunk, ...args);
  }) as any;

  // Capture stderr
  const originalStderrWrite = process.stderr.write.bind(process.stderr);
  process.stderr.write = ((chunk: any, ...args: any[]) => {
    const text = chunk.toString();
    stderr += text;
    return originalStderrWrite(chunk, ...args);
  }) as any;

  // Setup module resolution hook for @tego/botjs
  const Module = require("node:module");
  const originalResolveFilename = Module._resolveFilename;

  // Find the bot-agent binary location and locate @tego/botjs
  // Global install structure:
  // .volta/tools/image/packages/@tego/bot-agent/lib/node_modules/@tego/bot-agent/node_modules/@tego/botjs/
  const currentFileDir = path.dirname(fileURLToPath(import.meta.url));
  let botjsMainPath: string;

  try {
    // Try to resolve @tego/bot-agent to find its installation location
    const require2 = createRequire(import.meta.url);
    const botAgentPkgPath = require2.resolve("@tego/bot-agent/package.json");
    const botAgentDir = path.dirname(botAgentPkgPath);

    // @tego/botjs should be in bot-agent's node_modules
    botjsMainPath = path.join(
      botAgentDir,
      "node_modules/@tego/botjs/dist/index.mjs",
    );
  } catch {
    // Fallback: construct path relative to current bundled file
    // Current file is in dist/src-xxx.mjs (bundled)
    const distDir = path.dirname(currentFileDir);
    botjsMainPath = path.join(
      distDir,
      "node_modules/@tego/botjs/dist/index.mjs",
    );
  }

  Module._resolveFilename = function (
    request: string,
    parent: any,
    isMain: boolean,
  ) {
    // Redirect @tego/botjs imports to the resolved path
    if (request === "@tego/botjs") {
      return botjsMainPath;
    }
    return originalResolveFilename.call(this, request, parent, isMain);
  };

  try {
    // Register tsx to enable TypeScript loading
    await import("tsx");

    // Add timestamp to force fresh import
    const fileUrl = `${pathToFileURL(absolutePath).href}?t=${Date.now()}`;
    await import(fileUrl);

    // Restore module resolution
    Module._resolveFilename = originalResolveFilename;

    // Restore stdout/stderr
    process.stdout.write = originalStdoutWrite;
    process.stderr.write = originalStderrWrite;

    return {
      success: true,
      stdout,
      stderr,
    };
  } catch (error) {
    // Restore module resolution
    Module._resolveFilename = originalResolveFilename;

    // Restore stdout/stderr
    process.stdout.write = originalStdoutWrite;
    process.stderr.write = originalStderrWrite;

    return {
      success: false,
      stdout,
      stderr,
      error: error instanceof Error ? error : new Error(String(error)),
    };
  }
}

/**
 * Execute code string (saves to temp file first)
 */
export async function executeCodeString(
  code: string,
  tempFileName: string = "temp",
): Promise<ExecutionResult> {
  const fs = await import("node:fs/promises");
  const os = await import("node:os");

  const tempDir = os.tmpdir();
  const tempFile = path.join(tempDir, `${tempFileName}-${Date.now()}.ts`);

  try {
    await fs.writeFile(tempFile, code, "utf-8");
    return await executeScript(tempFile);
  } finally {
    // Clean up temp file
    await fs.unlink(tempFile).catch(() => {});
  }
}
