/**
 * Execute command - Run saved automation scripts
 */

import { executeScript } from "../script-manager/executor.js";
import { ScriptStorage } from "../script-manager/storage.js";
import {
  displayError,
  displayScriptInfo,
  displaySectionHeader,
  displaySuccess,
  displayWarning,
} from "../ui/display.js";
import { promptForScriptSelection } from "../ui/prompts.js";

export async function executeCommand(scriptName?: string): Promise<void> {
  try {
    displaySectionHeader("Execute Automation Script");

    const storage = new ScriptStorage();

    // If no script name provided, let user select
    let selectedScriptName = scriptName;
    if (!selectedScriptName) {
      const scripts = await storage.listScripts();
      if (scripts.length === 0) {
        displayWarning(
          "No scripts found. Use 'bot-agent generate' to create one.",
        );
        return;
      }

      selectedScriptName = await promptForScriptSelection(
        scripts.map((s) => ({ name: s.name, description: s.description })),
      );
    }

    // Check if script exists
    const exists = await storage.scriptExists(selectedScriptName);
    if (!exists) {
      displayError(`Script '${selectedScriptName}' not found`);
      process.exit(1);
    }

    // Load the script
    const savedScript = await storage.loadScript(selectedScriptName);

    displayScriptInfo(
      savedScript.metadata.name,
      savedScript.metadata.description,
      `${storage.getScriptsDirectory()}/${selectedScriptName}.ts`,
    );

    // Execute the script
    const scriptPath = `${storage.getScriptsDirectory()}/${selectedScriptName}.ts`;
    const result = await executeScript(scriptPath);

    if (result.success) {
      displaySuccess("Script executed successfully!");
    } else {
      displayError("Script execution failed");
      if (result.error) {
        console.error(result.error);
      }
      process.exit(1);
    }
  } catch (error) {
    if (error instanceof Error) {
      displayError(error.message);
    } else {
      displayError("An unexpected error occurred");
    }
    process.exit(1);
  }
}
