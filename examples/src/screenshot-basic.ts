/**
 * Basic Screenshot Examples
 *
 * Demonstrates the core screenshot functionality:
 * - Full screen capture
 * - Region capture
 * - Saving to different formats
 * - Clipboard operations
 */

import {
  captureAndCopy,
  captureAndSave,
  copyScreenshotToClipboard,
  quickScreenshot,
  quickScreenshotRegion,
  saveScreenshotToFile,
} from "@tego/botjs";

async function main() {
  console.log("=== Basic Screenshot Examples ===\n");

  try {
    // Example 1: Full screen screenshot
    console.log("1. Capturing full screen...");
    const fullScreen = await quickScreenshot();
    console.log(
      `   ✓ Captured ${fullScreen.region.width}x${fullScreen.region.height} at (${fullScreen.region.x}, ${fullScreen.region.y})`,
    );

    // Save as PNG (default)
    await saveScreenshotToFile(fullScreen, "fullscreen.png");
    console.log("   ✓ Saved as fullscreen.png\n");

    // Example 2: Region screenshot
    console.log("2. Capturing region (800x600 at top-left)...");
    const region = await quickScreenshotRegion({
      x: 0,
      y: 0,
      width: 800,
      height: 600,
    });
    console.log(`   ✓ Captured ${region.region.width}x${region.region.height}`);

    // Save as JPEG with quality
    await saveScreenshotToFile(region, "region.jpg", {
      format: "jpg",
      quality: 90,
    });
    console.log("   ✓ Saved as region.jpg (quality: 90)\n");

    // Example 3: Different formats
    console.log("3. Saving in different formats...");

    // PNG (lossless)
    await saveScreenshotToFile(region, "screenshot.png");
    console.log("   ✓ PNG: screenshot.png");

    // JPEG (compressed)
    await saveScreenshotToFile(region, "screenshot.jpg", {
      format: "jpg",
      quality: 85,
    });
    console.log("   ✓ JPEG: screenshot.jpg (quality: 85)");

    // WebP (modern format)
    await saveScreenshotToFile(region, "screenshot.webp", {
      format: "webp",
      quality: 90,
    });
    console.log("   ✓ WebP: screenshot.webp (quality: 90)\n");

    // Example 4: Clipboard operations
    console.log("4. Copying to clipboard...");
    await copyScreenshotToClipboard(region);
    console.log("   ✓ Screenshot copied to clipboard!\n");

    // Example 5: Convenience functions
    console.log("5. Using convenience functions...");

    // Capture and save in one call
    await captureAndSave("convenience.png", {
      x: 100,
      y: 100,
      width: 640,
      height: 480,
    });
    console.log("   ✓ Captured and saved: convenience.png");

    // Capture and copy in one call
    await captureAndCopy({ x: 0, y: 0, width: 400, height: 300 });
    console.log("   ✓ Captured and copied to clipboard!\n");

    // Example 6: Screenshot metadata
    console.log("6. Screenshot metadata:");
    console.log(`   Timestamp: ${new Date(fullScreen.timestamp * 1000)}`);
    console.log(`   Image buffer size: ${fullScreen.image.length} bytes`);
    console.log(
      `   Region: (${fullScreen.region.x}, ${fullScreen.region.y}) ${fullScreen.region.width}x${fullScreen.region.height}`,
    );

    console.log("\n✅ All examples completed successfully!");
  } catch (error) {
    console.error("❌ Error:", error);
    process.exit(1);
  }
}

main();
