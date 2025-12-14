/**
 * Script storage and management
 */

import fs from "node:fs/promises";
import os from "node:os";
import path from "node:path";
import type { ChatCompletionMessageParam } from "openai/resources/chat/completions";

export interface ScriptMetadata {
  name: string;
  description: string;
  createdAt: string;
  updatedAt: string;
  conversationHistory: ChatCompletionMessageParam[];
}

export interface SavedScript {
  metadata: ScriptMetadata;
  code: string;
}

export class ScriptStorage {
  private scriptsDir: string;

  constructor() {
    this.scriptsDir = path.join(os.homedir(), ".tego", "bot-scripts");
  }

  /**
   * Ensure scripts directory exists
   */
  async ensureScriptsDir(): Promise<void> {
    await fs.mkdir(this.scriptsDir, { recursive: true });
  }

  /**
   * Get path to script file
   */
  private getScriptPath(name: string): string {
    return path.join(this.scriptsDir, `${name}.ts`);
  }

  /**
   * Get path to metadata file
   */
  private getMetadataPath(name: string): string {
    return path.join(this.scriptsDir, `${name}.meta.json`);
  }

  /**
   * Save a script with metadata
   */
  async saveScript(
    name: string,
    code: string,
    description: string,
    conversationHistory: ChatCompletionMessageParam[],
  ): Promise<void> {
    await this.ensureScriptsDir();

    const scriptPath = this.getScriptPath(name);
    const metadataPath = this.getMetadataPath(name);

    // Check if script exists to determine if this is an update
    const isUpdate = await this.scriptExists(name);

    const metadata: ScriptMetadata = isUpdate
      ? {
          ...(await this.loadMetadata(name)),
          updatedAt: new Date().toISOString(),
          conversationHistory,
        }
      : {
          name,
          description,
          createdAt: new Date().toISOString(),
          updatedAt: new Date().toISOString(),
          conversationHistory,
        };

    await fs.writeFile(scriptPath, code, "utf-8");
    await fs.writeFile(
      metadataPath,
      JSON.stringify(metadata, null, 2),
      "utf-8",
    );
  }

  /**
   * Load a script with metadata
   */
  async loadScript(name: string): Promise<SavedScript> {
    const scriptPath = this.getScriptPath(name);
    const metadataPath = this.getMetadataPath(name);

    const [code, metadataContent] = await Promise.all([
      fs.readFile(scriptPath, "utf-8"),
      fs.readFile(metadataPath, "utf-8"),
    ]);

    const metadata = JSON.parse(metadataContent) as ScriptMetadata;

    return { metadata, code };
  }

  /**
   * Check if script exists
   */
  async scriptExists(name: string): Promise<boolean> {
    try {
      await fs.access(this.getScriptPath(name));
      return true;
    } catch {
      return false;
    }
  }

  /**
   * Load metadata only
   */
  private async loadMetadata(name: string): Promise<ScriptMetadata> {
    const metadataPath = this.getMetadataPath(name);
    const content = await fs.readFile(metadataPath, "utf-8");
    return JSON.parse(content) as ScriptMetadata;
  }

  /**
   * List all saved scripts
   */
  async listScripts(): Promise<ScriptMetadata[]> {
    await this.ensureScriptsDir();

    const files = await fs.readdir(this.scriptsDir);
    const scriptNames = files
      .filter((f) => f.endsWith(".ts"))
      .map((f) => f.replace(".ts", ""));

    const scripts: ScriptMetadata[] = [];
    for (const name of scriptNames) {
      try {
        const metadata = await this.loadMetadata(name);
        scripts.push(metadata);
      } catch {
        // Skip scripts with missing/invalid metadata
      }
    }

    return scripts.sort(
      (a, b) =>
        new Date(b.updatedAt).getTime() - new Date(a.updatedAt).getTime(),
    );
  }

  /**
   * Delete a script
   */
  async deleteScript(name: string): Promise<void> {
    const scriptPath = this.getScriptPath(name);
    const metadataPath = this.getMetadataPath(name);

    await Promise.all([
      fs.unlink(scriptPath).catch(() => {}),
      fs.unlink(metadataPath).catch(() => {}),
    ]);
  }

  /**
   * Get scripts directory path
   */
  getScriptsDirectory(): string {
    return this.scriptsDir;
  }
}
