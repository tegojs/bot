import { defineConfig } from "tsdown";

export default defineConfig({
  entry: ["src/index.ts", "bin/bot.ts"],
  format: ["esm"],
  dts: true,
  clean: true,
  shims: true,
  platform: "node",
  target: "node18",
  external: ["commander"],
});
