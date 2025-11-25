# Screenshot API Documentation

The Screenshot API provides advanced screenshot functionality with support for region selection, color picking, multiple formats, and clipboard operations.

## Table of Contents

- [Quick Start](#quick-start)
- [Core Concepts](#core-concepts)
- [API Reference](#api-reference)
- [Examples](#examples)
- [Advanced Usage](#advanced-usage)

## Quick Start

### Installation

```bash
npm install @tego/botjs
```

### Basic Usage

```typescript
import { quickScreenshot, saveScreenshotToFile } from "@tego/botjs";

// Capture full screen
const screenshot = await quickScreenshot();

// Save as PNG
await saveScreenshotToFile(screenshot, "screenshot.png");
```

## Core Concepts

### Screenshot Result

All capture functions return a `ScreenshotResult` object:

```typescript
interface ScreenshotResult {
  image: Buffer;        // PNG-encoded image data
  region: ScreenRegion; // Captured region
  timestamp: number;    // Unix timestamp
}
```

### Screen Region

Define areas to capture:

```typescript
interface ScreenRegion {
  x: number;      // X coordinate
  y: number;      // Y coordinate
  width: number;  // Width in pixels
  height: number; // Height in pixels
}
```

### Image Formats

Supported formats:
- **PNG** - Lossless, best for UI screenshots
- **JPG/JPEG** - Compressed, smaller file size
- **WebP** - Modern format with good compression

## API Reference

### Quick Functions

#### `quickScreenshot()`

Capture the entire screen.

```typescript
const screenshot = await quickScreenshot();
```

#### `quickScreenshotRegion(region)`

Capture a specific region.

```typescript
const screenshot = await quickScreenshotRegion({
  x: 0,
  y: 0,
  width: 1920,
  height: 1080
});
```

#### `captureRegion(x, y, width, height)`

Simplified region capture.

```typescript
const screenshot = await captureRegion(100, 100, 800, 600);
```

### File Operations

#### `saveScreenshotToFile(result, path, options?)`

Save screenshot to file.

```typescript
// PNG (default)
await saveScreenshotToFile(screenshot, "screenshot.png");

// JPEG with quality
await saveScreenshotToFile(screenshot, "screenshot.jpg", {
  format: "jpg",
  quality: 90
});

// WebP
await saveScreenshotToFile(screenshot, "screenshot.webp", {
  format: "webp",
  quality: 85
});
```

**Options:**
- `format?: 'png' | 'jpg' | 'webp'` - Image format
- `quality?: number` - Quality (1-100, for JPG/WebP)

### Clipboard Operations

#### `copyScreenshotToClipboard(result)`

Copy screenshot to clipboard.

```typescript
await copyScreenshotToClipboard(screenshot);
```

### Color Picker

#### `getPixelColor(x, y)`

Get color at specific coordinates.

```typescript
const color = await getPixelColor(500, 300);

console.log(color.hex);   // #FF5733
console.log(color.rgb);   // { r: 255, g: 87, b: 51 }
console.log(color.rgba);  // { r: 255, g: 87, b: 51, a: 1.0 }
console.log(color.hsl);   // { h: 11, s: 100, l: 60 }
```

**Returns:**
```typescript
interface ColorInfo {
  rgb: { r: number; g: number; b: number };
  rgba: { r: number; g: number; b: number; a: number };
  hex: string;  // #RRGGBB
  hsl: { h: number; s: number; l: number };
  position: { x: number; y: number };
}
```

### Convenience Functions

#### `captureAndSave(path, region?, options?)`

Capture and save in one call.

```typescript
// Full screen
await captureAndSave("screenshot.png");

// Region
await captureAndSave("region.jpg", { x: 0, y: 0, width: 800, height: 600 }, {
  quality: 90
});
```

#### `captureAndCopy(region?)`

Capture and copy to clipboard.

```typescript
// Full screen
await captureAndCopy();

// Region
await captureAndCopy({ x: 0, y: 0, width: 800, height: 600 });
```

### ScreenshotTool Class

For multiple operations, use the `ScreenshotTool` class:

```typescript
import { ScreenshotTool } from "@tego/botjs";

const tool = new ScreenshotTool({
  autoCopyToClipboard: false,
  defaultSavePath: "./screenshots"
});

try {
  const screenshot = await tool.captureQuick();
  const color = await tool.getPixelColor(100, 200);
  // ... more operations
} finally {
  await tool.close();
}
```

**Options:**
- `defaultSavePath?: string` - Default save directory
- `autoCopyToClipboard?: boolean` - Auto-copy captures

**Methods:**
- `captureQuick(region?)` - Quick screenshot
- `getPixelColor(x, y)` - Get pixel color
- `captureInteractive(options?)` - Interactive mode (not yet implemented)
- `pickColor(options?)` - Interactive color picker (not yet implemented)
- `close()` - Cleanup resources

## Examples

### Example 1: Basic Screenshot

```typescript
import { quickScreenshot, saveScreenshotToFile } from "@tego/botjs";

const screenshot = await quickScreenshot();
await saveScreenshotToFile(screenshot, "screenshot.png");
console.log("Screenshot saved!");
```

### Example 2: Region Capture

```typescript
import { captureRegion, saveScreenshotToFile } from "@tego/botjs";

// Capture top-left quarter of screen
const screenshot = await captureRegion(0, 0, 960, 540);
await saveScreenshotToFile(screenshot, "region.png");
```

### Example 3: Multiple Formats

```typescript
import { quickScreenshot, saveScreenshotToFile } from "@tego/botjs";

const screenshot = await quickScreenshot();

// Save in different formats
await saveScreenshotToFile(screenshot, "image.png");
await saveScreenshotToFile(screenshot, "image.jpg", { quality: 90 });
await saveScreenshotToFile(screenshot, "image.webp", { quality: 85 });
```

### Example 4: Color Sampling

```typescript
import { getPixelColor } from "@tego/botjs";

// Get colors at multiple positions
const positions = [
  { x: 100, y: 100 },
  { x: 500, y: 500 },
  { x: 1000, y: 100 }
];

for (const pos of positions) {
  const color = await getPixelColor(pos.x, pos.y);
  console.log(`(${pos.x}, ${pos.y}): ${color.hex}`);
}
```

### Example 5: Batch Screenshots

```typescript
import { captureRegion, saveScreenshotToFile } from "@tego/botjs";

const regions = [
  { name: "top-left", x: 0, y: 0, width: 400, height: 300 },
  { name: "top-right", x: 1520, y: 0, width: 400, height: 300 },
  { name: "center", x: 760, y: 390, width: 400, height: 300 }
];

for (const region of regions) {
  const screenshot = await captureRegion(
    region.x,
    region.y,
    region.width,
    region.height
  );
  await saveScreenshotToFile(screenshot, `${region.name}.png`);
}
```

### Example 6: Time-lapse Capture

```typescript
import { quickScreenshot, saveScreenshotToFile } from "@tego/botjs";

async function captureTimelapse(count: number, interval: number) {
  for (let i = 0; i < count; i++) {
    const screenshot = await quickScreenshot();
    await saveScreenshotToFile(
      screenshot,
      `frame_${i.toString().padStart(3, "0")}.png`
    );

    if (i < count - 1) {
      await new Promise(resolve => setTimeout(resolve, interval));
    }
  }
}

// Capture 10 frames at 1 second intervals
await captureTimelapse(10, 1000);
```

## Advanced Usage

### Screenshot with Metadata

```typescript
import { captureRegion, getPixelColor, saveScreenshotToFile } from "@tego/botjs";
import * as fs from "fs";

interface Metadata {
  timestamp: Date;
  region: { x: number; y: number; width: number; height: number };
  dominantColor: string;
  fileSize: number;
}

async function captureWithMetadata(x: number, y: number, width: number, height: number) {
  const screenshot = await captureRegion(x, y, width, height);

  // Sample center color
  const centerX = x + Math.floor(width / 2);
  const centerY = y + Math.floor(height / 2);
  const color = await getPixelColor(centerX, centerY);

  const metadata: Metadata = {
    timestamp: new Date(screenshot.timestamp * 1000),
    region: { x, y, width, height },
    dominantColor: color.hex,
    fileSize: screenshot.image.length
  };

  // Save screenshot and metadata
  await saveScreenshotToFile(screenshot, "screenshot.png");
  fs.writeFileSync("metadata.json", JSON.stringify(metadata, null, 2));

  return { screenshot, metadata };
}
```

### Color Analysis

```typescript
import { getPixelColor } from "@tego/botjs";

function getColorBrightness(r: number, g: number, b: number): number {
  return Math.sqrt(0.299 * r * r + 0.587 * g * g + 0.114 * b * b);
}

function isColorDark(r: number, g: number, b: number): boolean {
  return getColorBrightness(r, g, b) < 128;
}

const color = await getPixelColor(100, 100);
const brightness = getColorBrightness(color.rgb.r, color.rgb.g, color.rgb.b);
const isDark = isColorDark(color.rgb.r, color.rgb.g, color.rgb.b);

console.log(`Color: ${color.hex}`);
console.log(`Brightness: ${brightness.toFixed(0)}/255`);
console.log(`Appearance: ${isDark ? "Dark" : "Light"}`);
```

### Error Handling

```typescript
import { captureRegion, getPixelColor } from "@tego/botjs";

try {
  // Invalid region
  const screenshot = await captureRegion(-100, -100, 100, 100);
} catch (error) {
  console.error("Invalid region:", error.message);
}

try {
  // Out of bounds position
  const color = await getPixelColor(99999, 99999);
} catch (error) {
  console.error("Invalid position:", error.message);
}
```

## Best Practices

1. **Use appropriate formats**
   - PNG for UI screenshots (lossless)
   - JPEG for photos (smaller size)
   - WebP for web use (good balance)

2. **Handle errors gracefully**
   ```typescript
   try {
     const screenshot = await quickScreenshot();
     await saveScreenshotToFile(screenshot, "screenshot.png");
   } catch (error) {
     console.error("Screenshot failed:", error);
   }
   ```

3. **Close resources**
   ```typescript
   const tool = new ScreenshotTool();
   try {
     // Use tool
   } finally {
     await tool.close();
   }
   ```

4. **Use convenience functions for simple tasks**
   ```typescript
   // Instead of:
   const screenshot = await quickScreenshot();
   await saveScreenshotToFile(screenshot, "image.png");

   // Use:
   await captureAndSave("image.png");
   ```

## Performance Tips

- **Batch operations**: Use `ScreenshotTool` class for multiple captures
- **Region capture**: Capture only what you need to reduce file size
- **Format selection**: Use JPEG for large screenshots to save disk space
- **Async operations**: All functions return Promises for non-blocking execution

## Troubleshooting

### Screenshot is black or transparent
- Check if the window/region is visible
- Some applications may block screenshots

### Invalid region errors
- Ensure x, y coordinates are non-negative
- Verify width/height fit within screen bounds

### Color picker returns unexpected values
- Coordinates are screen-relative, not window-relative
- Multi-monitor setups use combined coordinate space

## Future Features

The following features are planned but not yet implemented:

- **Interactive capture mode** - Visual overlay for region selection
- **Window snapping** - Auto-snap to window boundaries
- **Interactive color picker** - UI for color selection
- **Annotations** - Draw on screenshots before saving

These features have complete infrastructure and will be enabled in future updates.

## See Also

- [Examples](../examples/)
- [API Reference](./api-reference.md)
- [Contributing](../CONTRIBUTING.md)
