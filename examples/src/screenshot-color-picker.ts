/**
 * Color Picker Example
 *
 * Demonstrates color picking functionality:
 * - Getting pixel colors
 * - Color format conversions
 * - Finding colors in screenshots
 */

import {
  getPixelColor,
  quickScreenshotRegion,
  ScreenshotTool,
} from "@tego/botjs";

async function main() {
  console.log("=== Color Picker Examples ===\n");

  try {
    // Example 1: Get pixel color at specific coordinates
    console.log("1. Getting pixel color at (500, 300)...");
    const color = await getPixelColor(500, 300);

    console.log("   Color information:");
    console.log(`   - HEX: ${color.hex}`);
    console.log(
      `   - RGB: rgb(${color.rgb.r}, ${color.rgb.g}, ${color.rgb.b})`,
    );
    console.log(
      `   - RGBA: rgba(${color.rgba.r}, ${color.rgba.g}, ${color.rgba.b}, ${color.rgba.a})`,
    );
    console.log(
      `   - HSL: hsl(${color.hsl.h.toFixed(0)}°, ${color.hsl.s.toFixed(0)}%, ${color.hsl.l.toFixed(0)}%)`,
    );
    console.log(`   - Position: (${color.position.x}, ${color.position.y})\n`);

    // Example 2: Get colors from multiple positions
    console.log("2. Sampling colors from multiple positions...");
    const positions = [
      { x: 100, y: 100 },
      { x: 500, y: 500 },
      { x: 1000, y: 100 },
    ];

    for (const pos of positions) {
      const pixelColor = await getPixelColor(pos.x, pos.y);
      console.log(`   (${pos.x}, ${pos.y}): ${pixelColor.hex}`);
    }
    console.log();

    // Example 3: Using ScreenshotTool class for multiple operations
    console.log("3. Using ScreenshotTool for multiple color picks...");
    const tool = new ScreenshotTool();

    try {
      const color1 = await tool.getPixelColor(200, 200);
      const color2 = await tool.getPixelColor(400, 400);

      console.log(`   First color: ${color1.hex}`);
      console.log(`   Second color: ${color2.hex}`);

      // Compare colors
      const isSimilar =
        Math.abs(color1.rgb.r - color2.rgb.r) < 30 &&
        Math.abs(color1.rgb.g - color2.rgb.g) < 30 &&
        Math.abs(color1.rgb.b - color2.rgb.b) < 30;

      console.log(`   Colors are ${isSimilar ? "similar" : "different"}\n`);
    } finally {
      await tool.close();
    }

    // Example 4: Color analysis utilities
    console.log("4. Color analysis utilities...");

    function _rgbToHex(r: number, g: number, b: number): string {
      return `#${r.toString(16).padStart(2, "0")}${g.toString(16).padStart(2, "0")}${b.toString(16).padStart(2, "0")}`.toUpperCase();
    }

    function _hexToRgb(
      hex: string,
    ): { r: number; g: number; b: number } | null {
      const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
      return result
        ? {
            r: parseInt(result[1], 16),
            g: parseInt(result[2], 16),
            b: parseInt(result[3], 16),
          }
        : null;
    }

    function getColorBrightness(r: number, g: number, b: number): number {
      // Calculate perceived brightness (0-255)
      return Math.sqrt(0.299 * r * r + 0.587 * g * g + 0.114 * b * b);
    }

    function isColorDark(r: number, g: number, b: number): boolean {
      return getColorBrightness(r, g, b) < 128;
    }

    const testColor = await getPixelColor(100, 100);
    const brightness = getColorBrightness(
      testColor.rgb.r,
      testColor.rgb.g,
      testColor.rgb.b,
    );
    const isDark = isColorDark(
      testColor.rgb.r,
      testColor.rgb.g,
      testColor.rgb.b,
    );

    console.log(`   Color at (100, 100): ${testColor.hex}`);
    console.log(`   Brightness: ${brightness.toFixed(0)}/255`);
    console.log(`   Appearance: ${isDark ? "Dark" : "Light"}\n`);

    // Example 5: Find dominant color in region
    console.log("5. Finding dominant color in region...");
    const _screenshot = await quickScreenshotRegion({
      x: 0,
      y: 0,
      width: 200,
      height: 200,
    });

    // Sample a few points in the region
    const samplePoints = [
      { x: 50, y: 50 },
      { x: 100, y: 100 },
      { x: 150, y: 150 },
    ];

    const colors = [];
    for (const point of samplePoints) {
      const c = await getPixelColor(point.x, point.y);
      colors.push(c.hex);
    }

    console.log(`   Sampled colors: ${colors.join(", ")}`);

    console.log("\n✅ All color picker examples completed!");
  } catch (error) {
    console.error("❌ Error:", error);
    process.exit(1);
  }
}

main();
