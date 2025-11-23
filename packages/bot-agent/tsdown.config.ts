import { defineConfig } from "tsdown";

export default defineConfig({
  entry: ["src/index.ts", "bin/bot-agent.ts"],
  format: ["esm"],
  dts: true,
  clean: true,
  shims: true,
  platform: "node",
  target: "node18",
  external: [
    // Keep these external as they are CLI dependencies
    "commander",
    "@inquirer/prompts",
    "openai",
    "chalk",
    "ora",
    "boxen",
    "cli-highlight",
    "tsx",
    // Note: @tego/botjs is NOT external - it will be bundled
    // This allows scripts to import it without installing separately
  ],
});
