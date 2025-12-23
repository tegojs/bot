# Getting Started

Welcome to **Tego Bot** - a high-performance desktop automation library for Node.js, powered by Rust.

## Installation

Install the TypeScript wrapper package (recommended):

```bash
npm install @tego/botjs
# or
pnpm add @tego/botjs
# or
yarn add @tego/botjs
```

For the AI-powered CLI tool:

```bash
npm install -g @tego/bot-agent
# or
pnpm add -g @tego/bot-agent
```

## Quick Start

### Basic Mouse Control

```typescript
import { moveMouse, mouseClick, getMousePos } from '@tego/botjs';

// Get current mouse position
const pos = getMousePos();
console.log('Current position:', pos); // { x: 100, y: 200 }

// Move mouse to coordinates
moveMouse(500, 300);

// Click at current position
mouseClick('left');

// Move and click in one operation
moveMouse(800, 400);
mouseClick('left');
```

### Smooth Mouse Movement

```typescript
import { moveMouseSmooth } from '@tego/botjs';

// Move mouse smoothly (animated)
moveMouseSmooth(1000, 500);

// With custom speed (default is 3.0)
moveMouseSmooth(1000, 500, 5.0); // faster
```

### Keyboard Input

```typescript
import { keyTap, typeString, typeStringDelayed } from '@tego/botjs';

// Type a single key
keyTap('a');

// Type with modifier keys
keyTap('c', ['command']); // Cmd+C (macOS) or Ctrl+C (Windows/Linux)
keyTap('v', ['command']); // Cmd+V / Ctrl+V

// Type a string quickly
typeString('Hello, World!');

// Type with delay between characters (in milliseconds)
typeStringDelayed('Slow typing...', 100);

// Special keys
keyTap('enter');
keyTap('escape');
keyTap('tab');
```

### Screen Capture

```typescript
import { captureScreen, captureScreenRegion, getPixelColor, getScreenSize } from '@tego/botjs';

// Get screen dimensions
const size = getScreenSize();
console.log('Screen size:', size); // { width: 1920, height: 1080 }

// Capture full screen (returns PNG buffer)
const screenshot = await captureScreen();
console.log('Screenshot size:', screenshot.image.length, 'bytes');

// Save to file
import { writeFileSync } from 'fs';
writeFileSync('screenshot.png', screenshot.image);

// Capture a specific region
const region = await captureScreenRegion(0, 0, 400, 300);
writeFileSync('region.png', region.image);

// Get pixel color at coordinates (returns hex color)
const color = await getPixelColor(100, 200);
console.log('Pixel color:', color); // "#FF0000"
```

### Clipboard Operations

```typescript
import { getClipboard, setClipboard, clearClipboard } from '@tego/botjs';

// Set text to clipboard
setClipboard('Hello from Tego Bot!');

// Get clipboard text
const text = getClipboard();
console.log('Clipboard:', text);

// Clear clipboard
clearClipboard();

// Image operations
import { getClipboardImage, setClipboardImage } from '@tego/botjs';
import { readFileSync } from 'fs';

// Set PNG image to clipboard
const imageBuffer = readFileSync('image.png');
setClipboardImage(imageBuffer);

// Get image from clipboard (returns PNG buffer)
const clipboardImage = getClipboardImage();
```

### Window Management

```typescript
import { getActiveWindow, getAllWindows, findWindowsByTitle, findWindowsByProcess } from '@tego/botjs';

// Get currently active window
const activeWin = getActiveWindow();
console.log('Active window:', activeWin);
// {
//   title: "Visual Studio Code",
//   processId: 12345,
//   processPath: "/Applications/Visual Studio Code.app/Contents/MacOS/Electron",
//   x: 0,
//   y: 23,
//   width: 1920,
//   height: 1057
// }

// Find windows by title (case-insensitive partial match)
const vscodeWindows = findWindowsByTitle('Code');

// Find windows by process name
const chromeWindows = findWindowsByProcess('Google Chrome');
```

## Helper Functions

Tego Bot provides convenient helper functions for common operations:

```typescript
import { leftClick, rightClick, middleClick, doubleClick, mouseDown, mouseUp } from '@tego/botjs';

// Click at current position
leftClick();
rightClick();
middleClick();

// Click at specific coordinates
leftClick(100, 200);
rightClick(300, 400);

// Double click
doubleClick();
doubleClick(500, 300);

// Mouse button control
mouseDown('left');  // Press and hold
mouseUp('left');    // Release
```

## Class-based API

For advanced usage, you can use the class-based API:

```typescript
import { Mouse, Keyboard, Screen } from '@tego/botjs';

// Mouse instance
const mouse = new Mouse();
mouse.moveMouse(100, 200);
mouse.mouseClick('left');
const pos = mouse.getMousePos();

// Keyboard instance
const keyboard = new Keyboard();
keyboard.keyTap('a');
keyboard.typeString('Hello!');
keyboard.setKeyboardDelay(50); // Set delay between key presses

// Screen instance
const screen = new Screen();
const bitmap = await screen.capture(0, 0, 800, 600);
```

## Configuration

### Set Operation Delays

```typescript
import { setMouseDelay, setKeyboardDelay } from '@tego/botjs';

// Set delay after each mouse operation (in milliseconds)
setMouseDelay(50);

// Set delay after each keyboard operation
setKeyboardDelay(10);
```

## AI-Powered Script Generation

If you installed `@tego/bot-agent`, you can generate automation scripts using AI:

```bash
# Generate a new script
npx bot-agent generate

# Edit an existing script
npx bot-agent edit my-script

# Execute a saved script
npx bot-agent execute my-script

# List all saved scripts
npx bot-agent list
```

The AI agent will:
- Generate TypeScript code based on your description
- Show syntax-highlighted code preview
- Allow you to execute, save, or regenerate
- Support iterative editing through natural language

## Platform-Specific Notes

### macOS
- Requires macOS 10.13 or later
- Screen recording permission required: **System Preferences > Security & Privacy > Screen Recording**
- Accessibility permission may be required for some operations

### Linux
Install system dependencies:
```bash
# Ubuntu/Debian
sudo apt-get install libxcb1-dev libxrandr-dev libdbus-1-dev

# Fedora
sudo dnf install libxcb-devel libXrandr-devel dbus-devel
```

### Windows
- Windows 10 or later
- No additional configuration needed

## Next Steps

- 📚 [API Reference](/api/) - Complete API documentation
- 🔧 [Examples](https://github.com/tegojs/bot/tree/main/examples) - More example scripts
- 💬 [GitHub Issues](https://github.com/tegojs/bot/issues) - Report bugs or request features

## Common Patterns

### Wait and Retry

```typescript
function sleep(ms: number) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

// Wait for a specific pixel color
async function waitForColor(x: number, y: number, expectedColor: string, timeout = 5000) {
  const start = Date.now();
  while (Date.now() - start < timeout) {
    const color = await getPixelColor(x, y);
    if (color.toLowerCase() === expectedColor.toLowerCase()) {
      return true;
    }
    await sleep(100);
  }
  return false;
}

// Usage
const found = await waitForColor(100, 200, '#FF0000', 10000);
if (found) {
  console.log('Color found!');
  leftClick(100, 200);
}
```

### Image-based Detection

```typescript
import { captureScreenRegion } from '@tego/botjs';
import { readFileSync } from 'fs';

// Capture region and compare with reference image
async function findImage(x: number, y: number, width: number, height: number, referenceImagePath: string) {
  const captured = await captureScreenRegion(x, y, width, height);
  const reference = readFileSync(referenceImagePath);

  // Simple comparison (you may want to use image comparison library)
  return captured.image.equals(reference);
}
```

### Keyboard Shortcuts

```typescript
// Cross-platform copy/paste
const isMac = process.platform === 'darwin';
const modifier = isMac ? 'command' : 'control';

// Copy
keyTap('c', [modifier]);

// Paste
keyTap('v', [modifier]);

// Save
keyTap('s', [modifier]);

// Undo
keyTap('z', [modifier]);
```
