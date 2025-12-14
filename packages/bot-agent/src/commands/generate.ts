/**
 * Generate command - Create new automation scripts
 */

import type { ChatCompletionMessageParam } from "openai/resources/chat/completions";
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
  promptForDescription,
  promptForEditFeedback,
  promptForScriptName,
} from "../ui/prompts.js";

export async function generateCommand(description?: string): Promise<void> {
  try {
    displaySectionHeader("Generate New Automation Script");

    // Get description from user if not provided
    const userDescription = description || (await promptForDescription());

    // Initialize AI client
    const aiClient = createAIClientFromEnv();
    const storage = new ScriptStorage();

    let conversationHistory: ChatCompletionMessageParam[] = [];
    let currentCode = "";
    let continueEditing = true;

    while (continueEditing) {
      // Generate code
      const spinner = createSpinner("Generating automation code...").start();

      try {
        const result = await aiClient.generateCode(
          userDescription,
          conversationHistory,
        );
        currentCode = result.code;
        conversationHistory = result.conversationHistory;

        spinner.succeed("Code generated successfully!");
      } catch (error) {
        spinner.fail("Failed to generate code");
        throw error;
      }

      // Display generated code
      displayCode(currentCode);

      // Validate code
      const validation = validateTypeScriptCode(currentCode);
      if (!validation.valid || validation.warnings.length > 0) {
        displayValidationErrors(validation.errors, validation.warnings);
      }

      if (!validation.valid) {
        displayError(
          "Generated code has validation errors. Please try regenerating.",
        );
        continueEditing = await promptForContinuation();
        if (continueEditing) {
          const feedback = await promptForEditFeedback();
          conversationHistory.push({ role: "user", content: feedback });
        }
        continue;
      }

      // Ask user what to do
      const action = await promptForAction();

      if (action === "cancel") {
        displayWarning("Operation cancelled");
        return;
      }

      if (action === "regenerate") {
        const feedback = await promptForEditFeedback();
        conversationHistory.push({ role: "user", content: feedback });
        continue;
      }

      if (action === "edit") {
        const feedback = await promptForEditFeedback();
        const editSpinner = createSpinner("Editing code...").start();

        try {
          const result = await aiClient.editCode(
            currentCode,
            feedback,
            conversationHistory,
          );
          currentCode = result.code;
          conversationHistory = result.conversationHistory;
          editSpinner.succeed("Code edited successfully!");
        } catch (error) {
          editSpinner.fail("Failed to edit code");
          throw error;
        }
        continue;
      }

      // Generate script name
      const timestamp = new Date()
        .toISOString()
        .replace(/[:.]/g, "-")
        .slice(0, -5);
      const defaultName = `automation-${timestamp}`;
      const scriptName = await promptForScriptName(defaultName);

      // Save the script
      await storage.saveScript(
        scriptName,
        currentCode,
        userDescription,
        conversationHistory,
      );
      const scriptPath = `${storage.getScriptsDirectory()}/${scriptName}.ts`;

      displayScriptInfo(scriptName, userDescription, scriptPath);
      displaySuccess("Script saved successfully!");

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

      continueEditing = false;
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
