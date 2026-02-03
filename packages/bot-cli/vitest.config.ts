import { resolve } from "node:path";
import { defineConfig } from "vitest/config";

export default defineConfig({
  resolve: {
    alias: {
      "@tego/botjs": resolve(__dirname, "../botjs/src/index.ts"),
    },
  },
  test: {
    globals: true,
    environment: "node",
  },
});
