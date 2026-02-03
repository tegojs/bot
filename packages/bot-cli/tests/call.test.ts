import { describe, expect, it, vi } from "vitest";

vi.mock("@tego/botjs", () => ({
  moveMouse: () => undefined,
  definitelyNotARealExport: undefined,
}));

// Minimal unit tests that avoid invoking real bot APIs.

describe("callCommand", () => {
  it("errors on unknown export", async () => {
    const { callCommand } = await import("../src/commands/call.js");
    await expect(callCommand("definitelyNotARealExport", {})).rejects.toThrow(
      "not a function",
    );
  });

  it("supports dry-run without invoking", async () => {
    const { callCommand } = await import("../src/commands/call.js");
    const writeSpy = vi
      .spyOn(process.stdout, "write")
      .mockImplementation(() => true);

    await callCommand("moveMouse", {
      json: "[1,2]",
      dryRun: true,
      jsonOutput: true,
    });

    const output = writeSpy.mock.calls.map((call) => call[0]).join("");
    expect(output).toContain('"dryRun":true');
    writeSpy.mockRestore();
  });
});
