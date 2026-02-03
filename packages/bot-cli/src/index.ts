/**
 * Main CLI module
 */

import { Command } from "commander";
import { callCommand } from "./commands/call.js";
import { doctorCommand } from "./commands/doctor.js";
import { registerGeneratedCommands } from "./generated/commands.js";

const program = new Command();

program.name("bot").description("CLI wrapper for @tego/botjs").version("0.0.1");

program
  .command("call")
  .description("Invoke a @tego/botjs export with JSON args")
  .argument("<exportName>", "Exported function name")
  .option("--json <json>", "Args as JSON array or object")
  .option("--file <path>", "Read args JSON from file")
  .option("--stdin", "Read args JSON from stdin")
  .option("--out <path>", "Write Buffer result to file")
  .option("--json-output", "Emit result as JSON")
  .option("--no-validate", "Disable arg shape validation")
  .option("--dry-run", "Validate args without invoking")
  .action(async (exportName: string, options) => {
    await callCommand(exportName, options);
  });

program
  .command("doctor")
  .description("Check system readiness for bot automation")
  .option("--json-output", "Emit result as JSON")
  .action(async (options) => {
    await doctorCommand(Boolean(options.jsonOutput));
  });

registerGeneratedCommands(program);

export { program };
