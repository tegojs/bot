import { describe, expect, it, vi } from "vitest";
import { doctorCommand } from "./doctor.js";

describe("doctorCommand", () => {
  it("prints messages for macOS", async () => {
    const platformSpy = vi
      .spyOn(process, "platform", "get")
      .mockReturnValue("darwin");
    const writeSpy = vi
      .spyOn(process.stdout, "write")
      .mockImplementation(() => true);

    await doctorCommand(true);

    const output = writeSpy.mock.calls.map((call) => call[0]).join("");
    expect(output).toContain("Screen Recording");
    platformSpy.mockRestore();
    writeSpy.mockRestore();
  });
});
