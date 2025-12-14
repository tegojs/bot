/**
 * Edit command - Modify existing automation scripts
 */

import { createAIClientFromEnv } from "../ai/client.js";
import { executeScript } from "../script-manager/executor.js";
import { ScriptStorage } from "../script-manager/storage.js";
import { validateTypeScriptCode } from "../script-manager/validator.js";
import {
  createSpinner,
  displayCode,
  displayError,
  displayScriptInfo,
  displaySectionHeader,
  displaySuccess,
  displayValidationErrors,
  displayWarning,
} from "../ui/display.js";
import {
  promptForAction,
  promptForContinuation,
  promptForEditFeedback,
  promptForScriptSelection,
} from "../ui/prompts.js";

export async function editCommand(scriptName?: string): Promise<void> {
  try {
    displaySectionHeader("Edit Automation Script");

    const storage = new ScriptStorage();
    const aiClient = createAIClientFromEnv();

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

    // Load the script
    const savedScript = await storage.loadScript(selectedScriptName);
    let currentCode = savedScript.code;
    let conversationHistory = savedScript.metadata.conversationHistory || [];

    displayScriptInfo(
      savedScript.metadata.name,
      savedScript.metadata.description,
      `${storage.getScriptsDirectory()}/${selectedScriptName}.ts`,
    );

    // Display current code
    console.log("\nCurrent code:");
    displayCode(currentCode);

    let continueEditing = true;

    while (continueEditing) {
      // Get edit feedback
      const feedback = await promptForEditFeedback();

      // Edit code
      const spinner = createSpinner("Editing code...").start();

      try {
        const result = await aiClient.editCode(
          currentCode,
          feedback,
          conversationHistory,
        );
        currentCode = result.code;
        conversationHistory = result.conversationHistory;

        spinner.succeed("Code edited successfully!");
      } catch (error) {
        spinner.fail("Failed to edit code");
        throw error;
      }

      // Display edited code
      displayCode(currentCode);

      // Validate code
      const validation = validateTypeScriptCode(currentCode);
      if (!validation.valid || validation.warnings.length > 0) {
        displayValidationErrors(validation.errors, validation.warnings);
      }

      if (!validation.valid) {
        displayError("Generated code has validation errors.");
        continueEditing = await promptForContinuation();
        continue;
      }

      // Ask user what to do
      const action = await promptForAction();

      if (action === "cancel") {
        displayWarning("Changes not saved");
        return;
      }

      if (action === "edit" || action === "regenerate") {
        continue;
      }

      // Save the updated script
      await storage.saveScript(
        selectedScriptName,
        currentCode,
        savedScript.metadata.description,
        conversationHistory,
      );

      const scriptPath = `${storage.getScriptsDirectory()}/${selectedScriptName}.ts`;
      displaySuccess("Script updated successfully!");

      // Execute if requested
      if (action === "execute") {
        displaySectionHeader("Executing Script");

        const result = await executeScript(scriptPath);

        if (result.success) {
          displaySuccess("Script executed successfully!");
        } else {
          displayError("Script execution failed");
          if (result.error) {
            console.error(result.error);
          }
        }
      }

      continueEditing = await promptForContinuation();
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
