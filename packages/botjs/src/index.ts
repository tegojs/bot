// ============================================================================
// Re-export all types and implementations from @tego/bot with enhanced JSDoc
// ============================================================================

import * as bot from "@tego/bot";

// ============================================================================
// Type Exports
// ============================================================================

export type {
  Bitmap,
  MousePositionResult as MousePosition,
  ScreenCaptureResult as ScreenCapture,
  ScreenSizeResult as ScreenSize,
  WindowInfoResult as WindowInfo,
} from "@tego/bot";

/**
 * Hex color string in format "#RRGGBB"
 * @example "#FF0000" for red, "#00FF00" for green, "#0000FF" for blue
 */
export type PixelColor = string;

// ============================================================================
// Class Exports
// ============================================================================

// Note: Keyboard and Mouse classes are not currently exported from the Rust bindings.
// Use the standalone functions (keyTap, typeString, moveMouse, mouseClick, etc.) instead.

/**
 * Screen capture class for taking screenshots and getting pixel colors
 *
 * @example
 * ```typescript
 * import { Screen } from "@tego/botjs";
 *
 * const screen = new Screen();
 * const bitmap = await screen.capture(0, 0, 800, 600);
 * console.log(`Captured ${bitmap.width}x${bitmap.height} region`);
 * ```
 */
export const Screen = bot.Screen;

// ============================================================================
// Mouse Functions
// ============================================================================

/**
 * Move the mouse cursor to the specified coordinates instantly
 *
 * @param x - X coordinate in pixels
 * @param y - Y coordinate in pixels
 *
 * @example
 * ```typescript
 * import { moveMouse } from "@tego/botjs";
 *
 * // Move to absolute position
 * moveMouse(100, 200);
 * ```
 */
export function moveMouse(x: number, y: number): void {
  bot.moveMouse(x, y);
}

/**
 * Move the mouse cursor smoothly to the specified coordinates with easing animation
 *
 * @param x - X coordinate in pixels
 * @param y - Y coordinate in pixels
 * @param speed - Optional movement speed multiplier (default: 3.0, higher = faster)
 *
 * @example
 * ```typescript
 * import { moveMouseSmooth } from "@tego/botjs";
 *
 * // Smooth movement with default speed
 * moveMouseSmooth(500, 500);
 *
 * // Faster smooth movement
 * moveMouseSmooth(500, 500, 5.0);
 * ```
 */
export function moveMouseSmooth(x: number, y: number, speed?: number): void {
  bot.moveMouseSmooth(x, y, speed);
}

/**
 * Click the mouse button at the current cursor position
 *
 * @param button - Mouse button: "left", "right", or "middle" (default: "left")
 * @param double - Whether to perform a double click (default: false)
 *
 * @example
 * ```typescript
 * import { mouseClick } from "@tego/botjs";
 *
 * // Single left click
 * mouseClick('left');
 *
 * // Double right click
 * mouseClick('right', true);
 *
 * // Single middle click
 * mouseClick('middle');
 * ```
 */
export function mouseClick(button?: string, double?: boolean): void {
  bot.mouseClick(button, double);
}

/**
 * Toggle mouse button state (press down or release up)
 *
 * @param down - "down" to press the button, "up" to release it
 * @param button - Mouse button: "left", "right", or "middle" (default: "left")
 *
 * @example
 * ```typescript
 * import { mouseToggle } from "@tego/botjs";
 *
 * // Press and hold left button
 * mouseToggle('down', 'left');
 *
 * // Perform some actions while button is held...
 *
 * // Release left button
 * mouseToggle('up', 'left');
 * ```
 */
export function mouseToggle(down: string, button?: string): void {
  bot.mouseToggle(down, button);
}

/**
 * Drag the mouse from current position to target coordinates
 *
 * @param x - Target X coordinate in pixels
 * @param y - Target Y coordinate in pixels
 *
 * @example
 * ```typescript
 * import { moveMouse, dragMouse } from "@tego/botjs";
 *
 * // Move to start position
 * moveMouse(100, 100);
 *
 * // Drag to end position
 * dragMouse(500, 500);
 * ```
 */
export function dragMouse(x: number, y: number): void {
  bot.dragMouse(x, y);
}

/**
 * Scroll the mouse wheel in horizontal and/or vertical directions
 *
 * @param x - Horizontal scroll amount (positive = right, negative = left)
 * @param y - Vertical scroll amount (positive = down, negative = up)
 *
 * @example
 * ```typescript
 * import { scrollMouse } from "@tego/botjs";
 *
 * // Scroll down
 * scrollMouse(0, 3);
 *
 * // Scroll up
 * scrollMouse(0, -3);
 *
 * // Scroll right
 * scrollMouse(2, 0);
 * ```
 */
export function scrollMouse(x: number, y: number): void {
  bot.scrollMouse(x, y);
}

/**
 * Get the current mouse cursor position
 *
 * @returns Object containing x and y coordinates
 *
 * @example
 * ```typescript
 * import { getMousePos } from "@tego/botjs";
 *
 * const pos = getMousePos();
 * console.log(`Mouse is at: ${pos.x}, ${pos.y}`);
 * ```
 */
export function getMousePos(): bot.MousePositionResult {
  return bot.getMousePos();
}

/**
 * Set the delay between mouse operations in milliseconds
 *
 * @param delay - Delay in milliseconds (applied after each mouse operation)
 *
 * @example
 * ```typescript
 * import { setMouseDelay, moveMouse } from "@tego/botjs";
 *
 * // Set 50ms delay between operations
 * setMouseDelay(50);
 *
 * // These will have 50ms delay between them
 * moveMouse(100, 100);
 * moveMouse(200, 200);
 * ```
 */
export function setMouseDelay(delay: number): void {
  bot.setMouseDelay(delay);
}

// ============================================================================
// Keyboard Functions
// ============================================================================

/**
 * Tap a key (press and immediately release)
 *
 * @param key - Key to tap (e.g., 'a', 'enter', 'escape', 'f1')
 * @param modifier - Optional array of modifier keys: 'control', 'shift', 'alt', 'command'
 *
 * @example
 * ```typescript
 * import { keyTap } from "@tego/botjs";
 *
 * // Type a single character
 * keyTap('a');
 *
 * // Press Enter
 * keyTap('enter');
 *
 * // Ctrl+C (copy)
 * keyTap('c', ['control']);
 *
 * // Ctrl+Shift+V (paste without formatting)
 * keyTap('v', ['control', 'shift']);
 * ```
 */
export function keyTap(key: string, modifier?: string[]): void {
  bot.keyTap(key, modifier);
}

/**
 * Toggle a key state (press down or release up)
 *
 * @param key - Key to toggle
 * @param down - "down" to press, "up" to release
 * @param modifier - Optional array of modifier keys
 *
 * @example
 * ```typescript
 * import { keyToggle, keyTap } from "@tego/botjs";
 *
 * // Hold Shift
 * keyToggle('shift', 'down');
 *
 * // Type 'HELLO' (all caps due to Shift being held)
 * keyTap('h');
 * keyTap('e');
 * keyTap('l');
 * keyTap('l');
 * keyTap('o');
 *
 * // Release Shift
 * keyToggle('shift', 'up');
 * ```
 */
export function keyToggle(
  key: string,
  down: string,
  modifier?: string[],
): void {
  bot.keyToggle(key, down, modifier);
}

/**
 * Type a string of text by simulating individual keystrokes
 *
 * @param text - Text string to type
 *
 * @example
 * ```typescript
 * import { typeString } from "@tego/botjs";
 *
 * // Type text
 * typeString('Hello, World!');
 *
 * // Type email address
 * typeString('user@example.com');
 *
 * // Type with special characters
 * typeString('Password123!@#');
 * ```
 */
export function typeString(text: string): void {
  bot.typeString(text);
}

/**
 * Type a string with a specified delay between characters (simulates human typing speed)
 *
 * @param text - Text string to type
 * @param cpm - Characters per minute (typing speed)
 *
 * @example
 * ```typescript
 * import { typeStringDelayed } from "@tego/botjs";
 *
 * // Slow typing (300 characters per minute)
 * typeStringDelayed('Hello', 300);
 *
 * // Fast typing (1000 characters per minute)
 * typeStringDelayed('Fast typing!', 1000);
 * ```
 */
export function typeStringDelayed(text: string, cpm: number): void {
  bot.typeStringDelayed(text, cpm);
}

/**
 * Tap a Unicode character by its code point
 *
 * @param codePoint - Unicode code point (e.g., 0x1F600 for 😀)
 *
 * @example
 * ```typescript
 * import { unicodeTap } from "@tego/botjs";
 *
 * // Type emoji
 * unicodeTap(0x1F600); // 😀
 * unicodeTap(0x2764);  // ❤
 * unicodeTap(0x1F44D); // 👍
 * ```
 */
export function unicodeTap(codePoint: number): void {
  bot.unicodeTap(codePoint);
}

/**
 * Set the delay between keyboard operations in milliseconds
 *
 * @param ms - Delay in milliseconds
 *
 * @example
 * ```typescript
 * import { setKeyboardDelay, keyTap } from "@tego/botjs";
 *
 * // Set 10ms delay between key presses
 * setKeyboardDelay(10);
 *
 * // These will have 10ms delay between them
 * keyTap('h');
 * keyTap('i');
 * ```
 */
export function setKeyboardDelay(ms: number): void {
  bot.setKeyboardDelay(ms);
}

// ============================================================================
// Screen Functions
// ============================================================================

/**
 * Get the color at specific coordinates in a bitmap
 *
 * @param bitmap - Bitmap object from screen capture
 * @param x - X coordinate in the bitmap
 * @param y - Y coordinate in the bitmap
 * @returns Hex color string (e.g., "#FF0000" for red)
 *
 * @example
 * ```typescript
 * import { captureScreen, bitmapColorAt } from "@tego/botjs";
 *
 * const screenshot = await captureScreen();
 * const color = bitmapColorAt(screenshot, 100, 200);
 * console.log(`Color at (100, 200): ${color}`);
 * ```
 */
export function bitmapColorAt(
  bitmap: bot.Bitmap,
  x: number,
  y: number,
): string {
  return bot.bitmapColorAt(bitmap, x, y);
}

/**
 * Capture the entire screen as a PNG image
 *
 * @returns Promise resolving to screen capture with PNG buffer
 *
 * @example
 * ```typescript
 * import { captureScreen } from "@tego/botjs";
 * import fs from "fs";
 *
 * const screenshot = await captureScreen();
 * fs.writeFileSync('screenshot.png', screenshot.image);
 * console.log(`Captured ${screenshot.width}x${screenshot.height} screenshot`);
 * ```
 */
export function captureScreen(): Promise<bot.ScreenCaptureResult> {
  return bot.captureScreen();
}

/**
 * Capture a specific region of the screen as a PNG image
 *
 * @param x - X coordinate of the top-left corner
 * @param y - Y coordinate of the top-left corner
 * @param width - Width of the region in pixels
 * @param height - Height of the region in pixels
 * @returns Promise resolving to screen capture with PNG buffer
 *
 * @example
 * ```typescript
 * import { captureScreenRegion } from "@tego/botjs";
 * import fs from "fs";
 *
 * // Capture 800x600 region starting at (100, 100)
 * const region = await captureScreenRegion(100, 100, 800, 600);
 * fs.writeFileSync('region.png', region.image);
 * ```
 */
export function captureScreenRegion(
  x: number,
  y: number,
  width: number,
  height: number,
): Promise<bot.ScreenCaptureResult> {
  return bot.captureScreenRegion(x, y, width, height);
}

/**
 * Get the color of a pixel at specific screen coordinates
 *
 * @param x - X coordinate on screen
 * @param y - Y coordinate on screen
 * @returns Promise resolving to hex color string (e.g., "#FF0000")
 *
 * @example
 * ```typescript
 * import { getPixelColor } from "@tego/botjs";
 *
 * const color = await getPixelColor(100, 200);
 * console.log(`Pixel color: ${color}`);
 *
 * if (color === "#FF0000") {
 *   console.log("Pixel is red!");
 * }
 * ```
 */
export function getPixelColorHex(x: number, y: number): Promise<string> {
  return bot.getPixelColor(x, y);
}

/**
 * Get the global Screen instance for capture operations
 *
 * @returns Screen object
 *
 * @example
 * ```typescript
 * import { getScreen } from "@tego/botjs";
 *
 * const screen = getScreen();
 * const bitmap = await screen.capture(0, 0, 800, 600);
 * ```
 */
export function getScreen(): bot.Screen {
  return bot.getScreen();
}

/**
 * Get the dimensions of the primary screen
 *
 * @returns Object containing width and height in pixels
 *
 * @example
 * ```typescript
 * import { getScreenSize } from "@tego/botjs";
 *
 * const size = getScreenSize();
 * console.log(`Screen resolution: ${size.width}x${size.height}`);
 * ```
 */
export function getScreenSize(): bot.ScreenSizeResult {
  return bot.getScreenSize();
}

/**
 * Update screen metrics (refresh monitor information)
 * Call this after display configuration changes
 *
 * @example
 * ```typescript
 * import { updateScreenMetrics, getScreenSize } from "@tego/botjs";
 *
 * // After connecting/disconnecting monitors
 * updateScreenMetrics();
 * const newSize = getScreenSize();
 * console.log(`Updated screen size: ${newSize.width}x${newSize.height}`);
 * ```
 */
export function updateScreenMetrics(): void {
  bot.updateScreenMetrics();
}

// ============================================================================
// Clipboard Functions
// ============================================================================

/**
 * Get text content from the system clipboard
 *
 * @returns Current clipboard text content
 *
 * @example
 * ```typescript
 * import { getClipboard } from "@tego/botjs";
 *
 * const text = getClipboard();
 * console.log(`Clipboard contains: ${text}`);
 * ```
 */
export function getClipboard(): string {
  return bot.getClipboard();
}

/**
 * Set text content to the system clipboard
 *
 * @param text - Text to copy to clipboard
 *
 * @example
 * ```typescript
 * import { setClipboard } from "@tego/botjs";
 *
 * setClipboard('Hello from @tego/bot!');
 * setClipboard('user@example.com');
 * ```
 */
export function setClipboard(text: string): void {
  bot.setClipboard(text);
}

/**
 * Clear the system clipboard contents
 *
 * @example
 * ```typescript
 * import { clearClipboard } from "@tego/botjs";
 *
 * clearClipboard();
 * console.log('Clipboard cleared');
 * ```
 */
export function clearClipboard(): void {
  bot.clearClipboard();
}

/**
 * Get image from clipboard as a PNG-encoded buffer
 *
 * @returns PNG-encoded image buffer
 *
 * @example
 * ```typescript
 * import { getClipboardImage } from "@tego/botjs";
 * import fs from "fs";
 *
 * const imageBuffer = getClipboardImage();
 * fs.writeFileSync('clipboard.png', imageBuffer);
 * ```
 */
export function getClipboardImage(): Buffer {
  return bot.getClipboardImage();
}

/**
 * Set image to clipboard from a PNG-encoded buffer
 *
 * @param imageBuffer - PNG-encoded image buffer
 *
 * @example
 * ```typescript
 * import { setClipboardImage } from "@tego/botjs";
 * import fs from "fs";
 *
 * const imageData = fs.readFileSync('image.png');
 * setClipboardImage(imageData);
 * console.log('Image copied to clipboard');
 * ```
 */
export function setClipboardImage(imageBuffer: Buffer): void {
  bot.setClipboardImage(imageBuffer);
}

// ============================================================================
// Window Management Functions
// ============================================================================

/**
 * Get information about the currently active (focused) window
 *
 * @returns WindowInfo object with title, process, position, and dimensions
 *
 * @example
 * ```typescript
 * import { getActiveWindow } from "@tego/botjs";
 *
 * const win = getActiveWindow();
 * console.log(`Active window: ${win.title}`);
 * console.log(`Process: ${win.processPath} (PID: ${win.processId})`);
 * console.log(`Position: (${win.x}, ${win.y})`);
 * console.log(`Size: ${win.width}x${win.height}`);
 * ```
 */
export function getActiveWindow(): bot.WindowInfoResult {
  return bot.getActiveWindow();
}

/**
 * Get a list of all visible windows
 *
 * **Note:** Currently returns only the active window due to API limitations of the underlying library.
 * Future versions may support enumerating all windows.
 *
 * @returns Array of WindowInfo objects
 *
 * @example
 * ```typescript
 * import { getAllWindows } from "@tego/botjs";
 *
 * const windows = getAllWindows();
 * console.log(`Found ${windows.length} windows`);
 * windows.forEach(win => {
 *   console.log(`- ${win.title}`);
 * });
 * ```
 */
export function getAllWindows(): bot.WindowInfoResult[] {
  return bot.getAllWindows();
}

/**
 * Find windows by title using case-insensitive partial matching
 *
 * **Note:** Currently searches only the active window due to API limitations of the underlying library.
 * Future versions may support searching all windows.
 *
 * @param title - Title text to search for (case-insensitive partial match)
 * @returns Array of matching WindowInfo objects
 *
 * @example
 * ```typescript
 * import { findWindowsByTitle } from "@tego/botjs";
 *
 * // Find Chrome windows
 * const chromeWindows = findWindowsByTitle('chrome');
 * chromeWindows.forEach(win => console.log(win.title));
 *
 * // Find Visual Studio Code windows
 * const vscodeWindows = findWindowsByTitle('Visual Studio Code');
 * ```
 */
export function findWindowsByTitle(title: string): bot.WindowInfoResult[] {
  return bot.findWindowsByTitle(title);
}

/**
 * Find windows by process name using case-insensitive partial matching
 *
 * **Note:** Currently searches only the active window due to API limitations of the underlying library.
 * Future versions may support searching all windows.
 *
 * @param processName - Process name to search for (case-insensitive partial match)
 * @returns Array of matching WindowInfo objects
 *
 * @example
 * ```typescript
 * import { findWindowsByProcess } from "@tego/botjs";
 *
 * // Find VS Code windows by process
 * const vscodeWindows = findWindowsByProcess('code');
 * vscodeWindows.forEach(win => {
 *   console.log(`${win.title} - ${win.processPath}`);
 * });
 * ```
 */
export function findWindowsByProcess(
  processName: string,
): bot.WindowInfoResult[] {
  return bot.findWindowsByProcess(processName);
}

// ============================================================================
// Mouse Shortcut Helper Functions (botjs-specific)
// ============================================================================

/**
 * Perform a double-click at the current mouse position or at specified coordinates
 *
 * @param x - Optional X coordinate to move to before double-clicking
 * @param y - Optional Y coordinate to move to before double-clicking
 *
 * @example
 * ```typescript
 * import { doubleClick } from "@tego/botjs";
 *
 * // Double-click at current position
 * doubleClick();
 *
 * // Move to (100, 200) and double-click
 * doubleClick(100, 200);
 * ```
 */
export function doubleClick(x?: number, y?: number): void {
  if (x !== undefined && y !== undefined) {
    bot.moveMouse(x, y);
  }
  bot.mouseClick(undefined, true);
}

/**
 * Perform a right-click at the current mouse position or at specified coordinates
 *
 * @param x - Optional X coordinate to move to before right-clicking
 * @param y - Optional Y coordinate to move to before right-clicking
 *
 * @example
 * ```typescript
 * import { rightClick } from "@tego/botjs";
 *
 * // Right-click at current position
 * rightClick();
 *
 * // Move to (300, 400) and right-click
 * rightClick(300, 400);
 * ```
 */
export function rightClick(x?: number, y?: number): void {
  if (x !== undefined && y !== undefined) {
    bot.moveMouse(x, y);
  }
  bot.mouseClick("right", false);
}

/**
 * Perform a middle-click at the current mouse position or at specified coordinates
 *
 * @param x - Optional X coordinate to move to before middle-clicking
 * @param y - Optional Y coordinate to move to before middle-clicking
 *
 * @example
 * ```typescript
 * import { middleClick } from "@tego/botjs";
 *
 * // Middle-click at current position
 * middleClick();
 *
 * // Move to (500, 600) and middle-click
 * middleClick(500, 600);
 * ```
 */
export function middleClick(x?: number, y?: number): void {
  if (x !== undefined && y !== undefined) {
    bot.moveMouse(x, y);
  }
  bot.mouseClick("middle", false);
}

/**
 * Perform a left-click at the current mouse position or at specified coordinates
 *
 * @param x - Optional X coordinate to move to before left-clicking
 * @param y - Optional Y coordinate to move to before left-clicking
 *
 * @example
 * ```typescript
 * import { leftClick } from "@tego/botjs";
 *
 * // Left-click at current position
 * leftClick();
 *
 * // Move to (150, 250) and left-click
 * leftClick(150, 250);
 * ```
 */
export function leftClick(x?: number, y?: number): void {
  if (x !== undefined && y !== undefined) {
    bot.moveMouse(x, y);
  }
  bot.mouseClick("left", false);
}

/**
 * Press and hold a mouse button down
 *
 * @param button - Mouse button to hold: "left", "right", or "middle" (default: "left")
 *
 * @example
 * ```typescript
 * import { mouseDown, mouseUp, moveMouse } from "@tego/botjs";
 *
 * // Hold left button
 * mouseDown("left");
 *
 * // Perform drag operation
 * moveMouse(500, 500);
 *
 * // Release left button
 * mouseUp("left");
 * ```
 */
export function mouseDown(button: "left" | "right" | "middle" = "left"): void {
  bot.mouseToggle("down", button);
}

/**
 * Release a held mouse button
 *
 * @param button - Mouse button to release: "left", "right", or "middle" (default: "left")
 *
 * @example
 * ```typescript
 * import { mouseDown, mouseUp } from "@tego/botjs";
 *
 * mouseDown("left");
 * // ... perform actions while button is held ...
 * mouseUp("left");
 *
 * // Release right button
 * mouseUp("right");
 * ```
 */
export function mouseUp(button: "left" | "right" | "middle" = "left"): void {
  bot.mouseToggle("up", button);
}

// ============================================================================
// Screenshot Tool - Advanced Screenshot Functionality
// ============================================================================

export type {
  ColorInfo,
  ColorPickerOptions,
  HslColor,
  InteractiveCaptureOptions,
  Position,
  RgbaColor,
  RgbColor,
  SaveImageOptions,
  ScreenRegion,
  ScreenshotResult,
  ScreenshotToolOptions,
} from "./screenshot";
export {
  captureAndCopy,
  captureAndSave,
  captureRegion,
  copyScreenshotToClipboard,
  getPixelColor,
  quickScreenshot,
  quickScreenshotRegion,
  ScreenshotTool,
  saveScreenshotToFile,
  startInteractiveCapture,
} from "./screenshot";

// ============================================================================
// Image Template Matching
// ============================================================================

export type {
  ImageResource,
  MatchConfig,
  MatchResult,
} from "./image-match";
export {
  findAllInRegion,
  findAllOnScreen,
  findInRegion,
  findOnScreen,
  getMatchBounds,
  getMatchCenter,
  imageResource,
  imageResourceFromBuffer,
  imageResourceSync,
  waitFor,
  waitForGone,
} from "./image-match";
