import { platform } from "node:os";

interface DoctorResult {
  ok: boolean;
  platform: string;
  messages: string[];
}

export async function doctorCommand(jsonOutput: boolean): Promise<void> {
  const result: DoctorResult = {
    ok: true,
    platform: platform(),
    messages: [],
  };

  if (result.platform === "darwin") {
    result.messages.push(
      "macOS permissions: ensure Screen Recording access is granted (System Settings > Privacy & Security > Screen Recording).",
    );
  } else if (result.platform === "win32") {
    result.messages.push(
      "Windows: no additional permissions typically required.",
    );
  } else if (result.platform === "linux") {
    result.messages.push(
      "Linux: ensure a display server is available (X11/Wayland).",
    );
  } else {
    result.messages.push(`Platform '${result.platform}' is untested.`);
  }

  if (jsonOutput) {
    process.stdout.write(`${JSON.stringify(result)}\n`);
    return;
  }

  for (const message of result.messages) {
    process.stdout.write(`${message}\n`);
  }
}
