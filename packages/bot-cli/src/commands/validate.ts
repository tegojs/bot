import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";

export interface ApiSpec {
  exports: Array<{
    name: string;
    kind: string;
    params?: Array<{ name: string; optional?: boolean }>;
  }>;
}

const scriptDir = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const apiPath = resolve(scriptDir, "../../botjs/generated/api.json");

let cachedSpec: ApiSpec | null = null;

async function loadApiSpec(): Promise<ApiSpec> {
  if (cachedSpec) return cachedSpec;
  const raw = await readFile(apiPath, "utf8");
  cachedSpec = JSON.parse(raw) as ApiSpec;
  return cachedSpec;
}

export async function validateArgs(
  exportName: string,
  args: unknown[],
  validate: boolean,
): Promise<void> {
  if (!validate) return;
  const spec = await loadApiSpec();
  const entry = spec.exports.find((item) => item.name === exportName);
  if (!entry || entry.kind !== "function") return;
  const requiredCount = (entry.params ?? []).filter(
    (param) => !param.optional,
  ).length;
  if (args.length < requiredCount) {
    throw new Error(
      `Expected at least ${requiredCount} argument(s) for '${exportName}', received ${args.length}.`,
    );
  }
}

function dirname(path: string): string {
  return path.slice(0, Math.max(0, path.lastIndexOf("/")));
}
