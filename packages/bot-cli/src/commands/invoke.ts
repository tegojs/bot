import * as fs from "node:fs/promises";
import * as bot from "@tego/botjs";
import { validateArgs } from "./validate.js";

export interface InvokeOptions {
  out?: string;
  jsonOutput?: boolean;
  validate?: boolean;
  dryRun?: boolean;
}

function isBuffer(value: unknown): value is Buffer {
  return Buffer.isBuffer(value);
}

function extractBufferField(
  value: unknown,
): { buffer: Buffer; remaining?: Record<string, unknown> } | null {
  if (!value || typeof value !== "object") {
    return null;
  }
  const record = value as Record<string, unknown>;
  if (isBuffer(record.image)) {
    const { image, ...rest } = record;
    return { buffer: image, remaining: rest };
  }
  if (isBuffer(record.buffer)) {
    const { buffer, ...rest } = record;
    return { buffer, remaining: rest };
  }
  return null;
}

function isPromise<T>(value: T | Promise<T>): value is Promise<T> {
  return Boolean(value) && typeof (value as Promise<T>).then === "function";
}

async function writeResult(
  result: unknown,
  options: InvokeOptions,
): Promise<void> {
  if (isBuffer(result)) {
    if (!options.out) {
      throw new Error("Buffer result requires --out <path> to write file.");
    }
    await fs.writeFile(options.out, result);
    if (options.jsonOutput) {
      process.stdout.write(
        JSON.stringify({ out: options.out, bytes: result.length }),
      );
    } else {
      process.stdout.write(`Wrote ${result.length} bytes to ${options.out}\n`);
    }
    return;
  }

  const extracted = extractBufferField(result);
  if (options.out) {
    if (!extracted) {
      throw new Error(
        "--out was provided but result did not contain a Buffer.",
      );
    }
    await fs.writeFile(options.out, extracted.buffer);
    if (options.jsonOutput) {
      const payload = {
        out: options.out,
        bytes: extracted.buffer.length,
        ...(extracted.remaining && Object.keys(extracted.remaining).length > 0
          ? { result: extracted.remaining }
          : {}),
      };
      process.stdout.write(`${JSON.stringify(payload)}\n`);
    } else {
      process.stdout.write(
        `Wrote ${extracted.buffer.length} bytes to ${options.out}\n`,
      );
    }
    return;
  }

  if (options.jsonOutput) {
    process.stdout.write(`${JSON.stringify(result)}\n`);
    return;
  }

  if (typeof result === "undefined") {
    process.stdout.write("OK\n");
    return;
  }

  if (typeof result === "string") {
    process.stdout.write(`${result}\n`);
    return;
  }

  process.stdout.write(`${JSON.stringify(result, null, 2)}\n`);
}

export async function invokeExport(
  exportName: string,
  args: unknown[],
  options: InvokeOptions,
): Promise<void> {
  const fn = (bot as Record<string, unknown>)[exportName];
  if (typeof fn !== "function") {
    const names = Object.keys(bot).sort();
    const matches = names.filter((name) =>
      name.toLowerCase().includes(exportName.toLowerCase()),
    );
    const suffix =
      matches.length > 0 ? `\nClosest matches: ${matches.join(", ")}` : "";
    throw new Error(`Export '${exportName}' is not a function.${suffix}`);
  }

  const minArgs = fn.length;
  const shouldValidate = options.validate !== false;
  await validateArgs(exportName, args, shouldValidate);
  if (
    shouldValidate &&
    args.length === 1 &&
    args[0] &&
    typeof args[0] === "object" &&
    !Array.isArray(args[0]) &&
    minArgs > 1
  ) {
    throw new Error(
      `Argument for '${exportName}' should be an array when calling functions with ${minArgs} parameters.`,
    );
  }

  if (options.dryRun) {
    if (options.jsonOutput) {
      process.stdout.write(
        `${JSON.stringify({ ok: true, exportName, args, dryRun: true })}\n`,
      );
    } else {
      process.stdout.write(`OK (dry-run): ${exportName}\n`);
    }
    return;
  }

  let result: unknown;
  result = (fn as (...values: unknown[]) => unknown)(...args);

  if (isPromise(result)) {
    result = await result;
  }

  await writeResult(result, options);
}
