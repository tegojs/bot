/**
 * Advanced Screenshot Example
 *
 * Demonstrates advanced use cases:
 * - Screenshot tool class
 * - Batch operations
 * - Automated workflows
 * - Error handling
 */

import * as fs from "node:fs";
import * as path from "node:path";
import {
  captureRegion,
  getPixelColor,
  getScreenSize,
  type ScreenshotResult,
  ScreenshotTool,
  saveScreenshotToFile,
} from "@tego/botjs";

async function main() {
  console.log("=== Advanced Screenshot Examples ===\n");

  try {
    // Get screen dimensions for dynamic region calculation
    const screenSize = getScreenSize();
    const screenWidth = screenSize.width;
    const screenHeight = screenSize.height;

    // Example 1: Using ScreenshotTool class
    console.log("1. Using ScreenshotTool class...");
    const tool = new ScreenshotTool({
      autoCopyToClipboard: false,
      defaultSavePath: "./screenshots",
    });

    const screenshot = await tool.captureQuick();
    console.log(
      `   ✓ Captured ${screenshot.region.width}x${screenshot.region.height}\n`,
    );

    await tool.close();

    // Example 2: Batch screenshot capture
    console.log("2. Capturing multiple regions in batch...");
    const regionWidth = Math.min(400, Math.floor(screenWidth / 3));
    const regionHeight = Math.min(300, Math.floor(screenHeight / 3));
    const regions = [
      {
        name: "top-left",
        x: 0,
        y: 0,
        width: regionWidth,
        height: regionHeight,
      },
      {
        name: "top-right",
        x: Math.max(0, screenWidth - regionWidth),
        y: 0,
        width: regionWidth,
        height: regionHeight,
      },
      {
        name: "center",
        x: Math.floor((screenWidth - regionWidth) / 2),
        y: Math.floor((screenHeight - regionHeight) / 2),
        width: regionWidth,
        height: regionHeight,
      },
    ];

    const outputDir = "./screenshots/batch";
    if (!fs.existsSync(outputDir)) {
      fs.mkdirSync(outputDir, { recursive: true });
    }

    for (const region of regions) {
      const result = await captureRegion(
        region.x,
        region.y,
        region.width,
        region.height,
      );
      const filename = path.join(outputDir, `${region.name}.png`);
      await saveScreenshotToFile(result, filename);
      console.log(`   ✓ Saved ${region.name}: ${filename}`);
    }
    console.log();

    // Example 3: Time-lapse screenshot capture
    console.log("3. Capturing time-lapse screenshots...");
    const timelapseDir = "./screenshots/timelapse";
    if (!fs.existsSync(timelapseDir)) {
      fs.mkdirSync(timelapseDir, { recursive: true });
    }

    const captures = 3;
    const interval = 1000; // 1 second
    const timelapseWidth = Math.min(800, screenWidth);
    const timelapseHeight = Math.min(600, screenHeight);

    for (let i = 0; i < captures; i++) {
      const screenshot = await captureRegion(
        0,
        0,
        timelapseWidth,
        timelapseHeight,
      );
      const filename = path.join(
        timelapseDir,
        `frame_${i.toString().padStart(3, "0")}.png`,
      );
      await saveScreenshotToFile(screenshot, filename);
      console.log(`   ✓ Captured frame ${i + 1}/${captures}`);

      if (i < captures - 1) {
        await new Promise((resolve) => setTimeout(resolve, interval));
      }
    }
    console.log();

    // Example 4: Screenshot comparison workflow
    console.log("4. Screenshot comparison workflow...");

    // Capture baseline
    const baseline = await captureRegion(100, 100, 300, 200);
    await saveScreenshotToFile(baseline, "./screenshots/baseline.png");
    console.log("   ✓ Captured baseline");

    // Wait for changes
    console.log("   ⏳ Waiting 2 seconds for changes...");
    await new Promise((resolve) => setTimeout(resolve, 2000));

    // Capture comparison
    const comparison = await captureRegion(100, 100, 300, 200);
    await saveScreenshotToFile(comparison, "./screenshots/comparison.png");
    console.log("   ✓ Captured comparison");

    // Simple comparison (just check if buffers differ)
    const isDifferent = !baseline.image.equals(comparison.image);
    console.log(
      `   ${isDifferent ? "⚠️  Changes detected" : "✓ No changes detected"}\n`,
    );

    // Example 5: Screenshot with metadata
    console.log("5. Screenshot with metadata...");

    interface ScreenshotMetadata {
      timestamp: Date;
      region: { x: number; y: number; width: number; height: number };
      fileSize: number;
      format: string;
      dominantColor?: string;
    }

    async function captureWithMetadata(
      x: number,
      y: number,
      width: number,
      height: number,
    ): Promise<{ screenshot: ScreenshotResult; metadata: ScreenshotMetadata }> {
      const screenshot = await captureRegion(x, y, width, height);

      // Get dominant color by sampling center
      const centerX = x + Math.floor(width / 2);
      const centerY = y + Math.floor(height / 2);
      const color = await getPixelColor(centerX, centerY);

      const metadata: ScreenshotMetadata = {
        timestamp: new Date(screenshot.timestamp * 1000),
        region: { x, y, width, height },
        fileSize: screenshot.image.length,
        format: "png",
        dominantColor: color.hex,
      };

      return { screenshot, metadata };
    }

    const { screenshot: metaScreenshot, metadata } = await captureWithMetadata(
      200,
      200,
      400,
      300,
    );

    console.log("   Screenshot metadata:");
    console.log(`   - Timestamp: ${metadata.timestamp.toISOString()}`);
    console.log(
      `   - Region: ${metadata.region.width}x${metadata.region.height} at (${metadata.region.x}, ${metadata.region.y})`,
    );
    console.log(`   - File size: ${(metadata.fileSize / 1024).toFixed(2)} KB`);
    console.log(`   - Dominant color: ${metadata.dominantColor}`);

    // Save with metadata
    const metadataFile = "./screenshots/metadata.json";
    fs.writeFileSync(metadataFile, JSON.stringify(metadata, null, 2));
    await saveScreenshotToFile(
      metaScreenshot,
      "./screenshots/with-metadata.png",
    );
    console.log(`   ✓ Saved with metadata: ${metadataFile}\n`);

    // Example 6: Error handling
    console.log("6. Error handling examples...");

    try {
      // Try to capture invalid region
      await captureRegion(-100, -100, 100, 100);
    } catch (error) {
      console.log(
        `   ✓ Caught invalid region error: ${(error as Error).message}`,
      );
    }

    try {
      // Try to get color from invalid position
      await getPixelColor(99999, 99999);
    } catch (error) {
      console.log(
        `   ✓ Caught invalid position error: ${(error as Error).message}`,
      );
    }

    console.log("\n✅ All advanced examples completed!");
  } catch (error) {
    console.error("❌ Error:", error);
    process.exit(1);
  }
}

main();
