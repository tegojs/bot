/**
 * Display utilities for formatted output
 */

import boxen from "boxen";
import chalk from "chalk";
import { highlight } from "cli-highlight";
import ora, { type Ora } from "ora";

/**
 * Display success message
 */
export function displaySuccess(message: string): void {
  console.log(chalk.green("✓"), message);
}

/**
 * Display error message
 */
export function displayError(message: string): void {
  console.log(chalk.red("✗"), message);
}

/**
 * Display warning message
 */
export function displayWarning(message: string): void {
  console.log(chalk.yellow("⚠"), message);
}

/**
 * Display info message
 */
export function displayInfo(message: string): void {
  console.log(chalk.blue("ℹ"), message);
}

/**
 * Display code with syntax highlighting
 */
export function displayCode(code: string): void {
  const highlighted = highlight(code, {
    language: "typescript",
    theme: {
      keyword: chalk.magenta,
      built_in: chalk.cyan,
      string: chalk.green,
      comment: chalk.gray,
      function: chalk.blue,
      number: chalk.yellow,
    },
  });

  console.log(
    "\n" +
      boxen(highlighted, {
        padding: 1,
        margin: 1,
        borderStyle: "round",
        borderColor: "cyan",
        title: "Generated Code",
        titleAlignment: "center",
      }) +
      "\n",
  );
}

/**
 * Display script info
 */
export function displayScriptInfo(
  name: string,
  description: string,
  path: string,
): void {
  const info = [
    `${chalk.bold("Name:")} ${chalk.cyan(name)}`,
    `${chalk.bold("Description:")} ${description}`,
    `${chalk.bold("Location:")} ${chalk.gray(path)}`,
  ].join("\n");

  console.log(
    "\n" +
      boxen(info, {
        padding: 1,
        margin: 1,
        borderStyle: "round",
        borderColor: "green",
      }) +
      "\n",
  );
}

/**
 * Display validation errors
 */
export function displayValidationErrors(
  errors: string[],
  warnings: string[],
): void {
  if (errors.length > 0) {
    console.log(chalk.red.bold("\n✗ Validation Errors:"));
    errors.forEach((error) => {
      console.log(chalk.red(`  • ${error}`));
    });
  }

  if (warnings.length > 0) {
    console.log(chalk.yellow.bold("\n⚠ Warnings:"));
    warnings.forEach((warning) => {
      console.log(chalk.yellow(`  • ${warning}`));
    });
  }
}

/**
 * Display script list
 */
export function displayScriptList(
  scripts: Array<{ name: string; description: string; updatedAt: string }>,
): void {
  if (scripts.length === 0) {
    displayInfo("No scripts found. Use 'bot-agent generate' to create one.");
    return;
  }

  console.log(chalk.bold.cyan("\n📝 Saved Scripts:\n"));

  scripts.forEach((script, index) => {
    const date = new Date(script.updatedAt);
    const dateStr = `${date.toLocaleDateString()} ${date.toLocaleTimeString()}`;

    console.log(
      chalk.cyan(`${index + 1}.`),
      chalk.bold(script.name),
      chalk.gray(`(${dateStr})`),
    );
    console.log(chalk.gray(`   ${script.description}`));
    console.log();
  });
}

/**
 * Create and return a spinner
 */
export function createSpinner(text: string): Ora {
  return ora({
    text,
    color: "cyan",
  });
}

/**
 * Display welcome banner
 */
export function displayWelcomeBanner(): void {
  const banner = chalk.cyan.bold(`
╔═══════════════════════════════════════╗
║                                       ║
║      🤖 @tego/bot-agent               ║
║                                       ║
║   AI-Powered Automation Script        ║
║   Generator                           ║
║                                       ║
╚═══════════════════════════════════════╝
  `);

  console.log(banner);
}

/**
 * Display section header
 */
export function displaySectionHeader(title: string): void {
  console.log(`\n${chalk.bold.cyan("═".repeat(50))}`);
  console.log(chalk.bold.cyan(title));
  console.log(`${chalk.bold.cyan("═".repeat(50))}\n`);
}
