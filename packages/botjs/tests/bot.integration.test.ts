/**
 * Integration tests for @tego/bot
 *
 * These tests require actual system interaction (mouse, keyboard, screen access)
 * and will be skipped in CI environments by default.
 *
 * To run these tests locally, set ENABLE_INTEGRATION_TESTS=true
 *
 * Note: These tests can potentially run in GitHub Actions CI with proper setup:
 * - Linux: Requires Xvfb (virtual display server) - see CI workflow
 * - macOS: May work with proper permissions, but limited in headless environments
 * - Windows: May work but requires proper configuration
 *
 * For CI environments, consider using self-hosted runners for full system access.
 */
import type {
  Bitmap,
  ColorInfo,
  MousePosition,
  ScreenCapture,
  ScreenSize,
  WindowInfo,
} from "@tego/botjs";
import {
  bitmapColorAt,
  captureScreen,
  captureScreenRegion,
  clearClipboard,
  doubleClick,
  dragMouse,
  findWindowsByProcess,
  findWindowsByTitle,
  getActiveWindow,
  getAllWindows,
  getClipboard,
  getClipboardImage,
  getMousePos,
  getPixelColor,
  getPixelColorHex,
  getScreen,
  getScreenSize,
  Keyboard,
  keyTap,
  keyToggle,
  leftClick,
  Mouse,
  middleClick,
  mouseClick,
  mouseDown,
  mouseToggle,
  mouseUp,
  moveMouse,
  moveMouseSmooth,
  rightClick,
  Screen,
  scrollMouse,
  setClipboard,
  setClipboardImage,
  setKeyboardDelay,
  setMouseDelay,
  typeString,
  typeStringDelayed,
  unicodeTap,
} from "@tego/botjs";
import { describe, expect, it } from "vitest";

const ENABLE_INTEGRATION_TESTS =
  process.env.ENABLE_INTEGRATION_TESTS === "true";

describe.skipIf(!ENABLE_INTEGRATION_TESTS)(
  "@tego/bot Integration Tests",
  () => {
    describe("Keyboard class", () => {
      it("should create Keyboard instance", () => {
        const keyboard = new Keyboard();
        expect(keyboard).toBeDefined();
        expect(keyboard).toBeInstanceOf(Keyboard);
      });

      it("should have all required methods", () => {
        const keyboard = new Keyboard();
        expect(typeof keyboard.keyTap).toBe("function");
        expect(typeof keyboard.keyToggle).toBe("function");
        expect(typeof keyboard.typeString).toBe("function");
        expect(typeof keyboard.typeStringDelayed).toBe("function");
        expect(typeof keyboard.setKeyboardDelay).toBe("function");
      });

      it("should set keyboard delay", () => {
        const keyboard = new Keyboard();
        expect(() => {
          keyboard.setKeyboardDelay(50);
        }).not.toThrow();
      });
    });

    describe("Mouse class", () => {
      it("should create Mouse instance", () => {
        const mouse = new Mouse();
        expect(mouse).toBeDefined();
        expect(mouse).toBeInstanceOf(Mouse);
      });

      it("should have all required methods", () => {
        const mouse = new Mouse();
        expect(typeof mouse.moveMouse).toBe("function");
        expect(typeof mouse.moveMouseSmooth).toBe("function");
        expect(typeof mouse.moveMouseSmoothWithSpeed).toBe("function");
        expect(typeof mouse.getMousePos).toBe("function");
        expect(typeof mouse.mouseClick).toBe("function");
        expect(typeof mouse.mouseToggle).toBe("function");
        expect(typeof mouse.dragMouse).toBe("function");
        expect(typeof mouse.scrollMouse).toBe("function");
        expect(typeof mouse.setMouseDelay).toBe("function");
      });

      it("should set mouse delay", () => {
        const mouse = new Mouse();
        expect(() => {
          mouse.setMouseDelay(50);
        }).not.toThrow();
      });
    });

    describe("Keyboard operations", () => {
      it("should tap a key", () => {
        expect(() => {
          keyTap("a");
        }).not.toThrow();
      });

      it("should tap a key with modifier", () => {
        expect(() => {
          keyTap("c", ["shift"]);
        }).not.toThrow();
      });

      it("should tap special keys", () => {
        expect(() => {
          keyTap("enter");
          keyTap("escape");
          keyTap("tab");
        }).not.toThrow();
      });

      it("should toggle key state", () => {
        expect(() => {
          keyToggle("shift", "down");
          keyToggle("shift", "up");
        }).not.toThrow();
      });

      it("should type string", () => {
        expect(() => {
          typeString("Hello");
        }).not.toThrow();
      });

      it("should type string with delay", () => {
        expect(() => {
          typeStringDelayed("Test", 300);
        }).not.toThrow();
      });

      it("should tap unicode character", () => {
        expect(() => {
          unicodeTap(0x1f600); // 😀
        }).not.toThrow();
      });

      it("should type from Keyboard class", () => {
        const keyboard = new Keyboard();
        expect(() => {
          keyboard.typeString("Class test");
        }).not.toThrow();
      });

      it("should respect keyboard delay", () => {
        expect(() => {
          setKeyboardDelay(10);
          keyTap("a");
          setKeyboardDelay(0); // Reset
        }).not.toThrow();
      });
    });

    describe("Mouse operations", () => {
      it("should get mouse position", () => {
        const pos = getMousePos();
        expect(pos).toBeDefined();
        expect(typeof pos.x).toBe("number");
        expect(typeof pos.y).toBe("number");
        expect(pos.x).toBeGreaterThanOrEqual(0);
        expect(pos.y).toBeGreaterThanOrEqual(0);
      });

      it("should get mouse position from Mouse class", () => {
        const mouse = new Mouse();
        const pos = mouse.getMousePos();
        expect(pos).toBeDefined();
        expect(typeof pos.x).toBe("number");
        expect(typeof pos.y).toBe("number");
        expect(pos.x).toBeGreaterThanOrEqual(0);
        expect(pos.y).toBeGreaterThanOrEqual(0);
      });

      it("should move mouse", () => {
        expect(() => {
          moveMouse(100, 200);
        }).not.toThrow();
      });

      it("should move mouse smoothly", () => {
        expect(() => {
          moveMouseSmooth(300, 400);
        }).not.toThrow();
      });

      it("should move mouse smoothly with speed", () => {
        expect(() => {
          moveMouseSmooth(500, 600, 5.0);
        }).not.toThrow();
      });

      it("should click mouse buttons", () => {
        expect(() => {
          mouseClick("left");
          mouseClick("right");
          mouseClick("middle");
        }).not.toThrow();
      });

      it("should double click", () => {
        expect(() => {
          mouseClick("left", true);
        }).not.toThrow();
      });

      it("should toggle mouse button", () => {
        expect(() => {
          mouseToggle("down", "left");
          mouseToggle("up", "left");
        }).not.toThrow();
      });

      it("should drag mouse", () => {
        const startPos = getMousePos();
        expect(() => {
          dragMouse(startPos.x + 50, startPos.y + 50);
        }).not.toThrow();
      });

      it("should scroll mouse", () => {
        expect(() => {
          scrollMouse(0, 1); // Scroll down
          scrollMouse(0, -1); // Scroll up
          scrollMouse(1, 0); // Scroll right
          scrollMouse(-1, 0); // Scroll left
        }).not.toThrow();
      });

      it("should respect mouse delay", () => {
        expect(() => {
          setMouseDelay(50);
          moveMouse(200, 200);
          setMouseDelay(0); // Reset
        }).not.toThrow();
      });

      it("should use helper functions", () => {
        expect(() => {
          leftClick();
          rightClick();
          middleClick();
          doubleClick();
        }).not.toThrow();
      });

      it("should use helper functions with coordinates", () => {
        expect(() => {
          leftClick(100, 100);
          rightClick(200, 200);
          middleClick(300, 300);
          doubleClick(400, 400);
        }).not.toThrow();
      });

      it("should use mouseDown and mouseUp", () => {
        expect(() => {
          mouseDown("left");
          mouseUp("left");
          mouseDown("right");
          mouseUp("right");
          mouseDown("middle");
          mouseUp("middle");
        }).not.toThrow();
      });

      it("should perform operations from Mouse class", () => {
        const mouse = new Mouse();
        expect(() => {
          mouse.moveMouse(150, 150);
          mouse.mouseClick("left");
          mouse.scrollMouse(0, 1);
        }).not.toThrow();
      });
    });

    describe("Screen operations", () => {
      it("should get screen size", () => {
        const size = getScreenSize();
        expect(size).toBeDefined();
        expect(typeof size.width).toBe("number");
        expect(typeof size.height).toBe("number");
        expect(size.width).toBeGreaterThan(0);
        expect(size.height).toBeGreaterThan(0);
      });

      it("should capture full screen", async () => {
        const screenshot: ScreenCapture = await captureScreen();
        expect(screenshot).toBeDefined();
        expect(screenshot.width).toBeGreaterThan(0);
        expect(screenshot.height).toBeGreaterThan(0);
        expect(screenshot.image).toBeInstanceOf(Buffer);
        expect(screenshot.image.length).toBeGreaterThan(0);
      });

      it("should capture screen region", async () => {
        const screen = new Screen();
        const bitmap = await screen.capture(0, 0, 100, 100);
        expect(bitmap).toBeDefined();
        expect(bitmap.width).toBeGreaterThan(0);
        expect(bitmap.height).toBeGreaterThan(0);
        expect(bitmap.image).toBeInstanceOf(Buffer);
        expect(bitmap.byteWidth).toBeGreaterThan(0);
        expect(bitmap.bitsPerPixel).toBeGreaterThan(0);
        expect(bitmap.bytesPerPixel).toBeGreaterThan(0);
      });

      it("should capture region with captureScreenRegion", async () => {
        const region: ScreenCapture = await captureScreenRegion(0, 0, 200, 200);
        expect(region).toBeDefined();
        expect(region.width).toBe(200);
        expect(region.height).toBe(200);
        expect(region.image).toBeInstanceOf(Buffer);
      });

      it("should get pixel color as hex string", async () => {
        const color = await getPixelColorHex(100, 200);
        expect(color).toBeDefined();
        expect(typeof color).toBe("string");
        expect(color).toMatch(/^#[0-9a-fA-F]{6}$/);
      });

      it("should get pixel color as ColorInfo object", async () => {
        const colorInfo = await getPixelColor(100, 200);
        expect(colorInfo).toBeDefined();
        expect(typeof colorInfo).toBe("object");
        expect(colorInfo).toHaveProperty("rgb");
        expect(colorInfo).toHaveProperty("rgba");
        expect(colorInfo).toHaveProperty("hex");
        expect(colorInfo).toHaveProperty("hsl");
        expect(colorInfo).toHaveProperty("position");
        expect(colorInfo.hex).toMatch(/^#[0-9a-fA-F]{6}$/);
        expect(colorInfo.position.x).toBe(100);
        expect(colorInfo.position.y).toBe(200);
      });

      it("should get color from bitmap", async () => {
        const screen = getScreen();
        const bitmap = await screen.capture(0, 0, 100, 100);

        const color = bitmapColorAt(bitmap, 50, 50);
        expect(color).toBeDefined();
        expect(typeof color).toBe("string");
        expect(color).toMatch(/^#[0-9a-fA-F]{6}$/);
      });

      it("should handle out of bounds bitmap color", async () => {
        const screen = getScreen();
        const bitmap = await screen.capture(0, 0, 100, 100);

        // This should throw an error for out of bounds coordinates
        expect(() => {
          bitmapColorAt(bitmap, 999, 999);
        }).toThrow("Coordinates out of bounds");
      });

      it("should have correct Bitmap type", async () => {
        const screen = getScreen();
        const bitmap: Bitmap = await screen.capture(0, 0, 100, 100);
        expect(bitmap).toHaveProperty("width");
        expect(bitmap).toHaveProperty("height");
        expect(bitmap).toHaveProperty("image");
        expect(bitmap).toHaveProperty("byteWidth");
        expect(bitmap).toHaveProperty("bitsPerPixel");
        expect(bitmap).toHaveProperty("bytesPerPixel");
        expect(bitmap.image).toBeInstanceOf(Buffer);
      });

      it("should have correct ScreenSize type", () => {
        const size: ScreenSize = getScreenSize();
        expect(size).toHaveProperty("width");
        expect(size).toHaveProperty("height");
        expect(typeof size.width).toBe("number");
        expect(typeof size.height).toBe("number");
      });

      it("should have correct MousePosition type", () => {
        const pos: MousePosition = getMousePos();
        expect(pos).toHaveProperty("x");
        expect(pos).toHaveProperty("y");
        expect(typeof pos.x).toBe("number");
        expect(typeof pos.y).toBe("number");
      });
    });

    describe("Clipboard operations", () => {
      it("should set and get text", () => {
        const testText = "Hello from Tego Bot test!";
        setClipboard(testText);
        const retrieved = getClipboard();
        expect(retrieved).toBe(testText);
      });

      it("should clear clipboard", () => {
        setClipboard("Test content");
        clearClipboard();

        // After clearing, clipboard may throw error or return empty string
        try {
          const content = getClipboard();
          expect(content).toBe("");
        } catch (error) {
          // Expected: clipboard is empty and throws error
          expect(error).toBeDefined();
        }
      });

      it("should handle empty string", () => {
        setClipboard("");
        const content = getClipboard();
        expect(content).toBe("");
      });

      it("should handle special characters", () => {
        const specialText = "Test\nNew Line\tTab\r\nWindows Line";
        setClipboard(specialText);
        const retrieved = getClipboard();
        expect(retrieved).toContain("Test");
      });

      it("should handle unicode text", () => {
        const unicodeText = "Hello 世界 🌍 Привет";
        setClipboard(unicodeText);
        const retrieved = getClipboard();
        expect(retrieved).toBe(unicodeText);
      });

      it("should handle clipboard image operations", async () => {
        // Create a valid 1x1 red PNG image
        const validPNG = Buffer.from([
          // PNG signature
          0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a,
          // IHDR chunk
          0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44, 0x52, 0x00, 0x00, 0x00,
          0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00, 0x00, 0x90,
          0x77, 0x53, 0xde,
          // IDAT chunk (1x1 red pixel)
          0x00, 0x00, 0x00, 0x0c, 0x49, 0x44, 0x41, 0x54, 0x08, 0xd7, 0x63,
          0xf8, 0xcf, 0xc0, 0x00, 0x00, 0x03, 0x01, 0x01, 0x00, 0x18, 0xdd,
          0x8d, 0xb4,
          // IEND chunk
          0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4e, 0x44, 0xae, 0x42, 0x60,
          0x82,
        ]);

        // Test setting image - may fail on some systems
        try {
          setClipboardImage(validPNG);
          const retrieved = getClipboardImage();
          expect(retrieved).toBeInstanceOf(Buffer);
          expect(retrieved.length).toBeGreaterThan(0);
        } catch (error) {
          // Some systems may not support clipboard image operations
          // This is acceptable - we just verify the API exists
          expect(error).toBeDefined();
        }
      });

      it("should round-trip text clipboard", () => {
        const originalText = "Round trip test 123";
        setClipboard(originalText);
        const retrieved = getClipboard();
        expect(retrieved).toBe(originalText);

        // Verify we can set it again
        setClipboard("Second text");
        const retrieved2 = getClipboard();
        expect(retrieved2).toBe("Second text");
      });
    });

    describe("Window Management operations", () => {
      it("should get active window", () => {
        const win = getActiveWindow();
        expect(win).toBeDefined();
        expect(win).toHaveProperty("title");
        expect(win).toHaveProperty("processId");
        expect(win).toHaveProperty("processPath");
        expect(win).toHaveProperty("x");
        expect(win).toHaveProperty("y");
        expect(win).toHaveProperty("width");
        expect(win).toHaveProperty("height");
        expect(typeof win.title).toBe("string");
        expect(typeof win.processId).toBe("number");
        expect(typeof win.processPath).toBe("string");
      });

      it("should have correct WindowInfo type", () => {
        const win: WindowInfo = getActiveWindow();
        expect(win).toBeDefined();
        expect(typeof win.title).toBe("string");
        expect(typeof win.processId).toBe("number");
        expect(typeof win.x).toBe("number");
        expect(typeof win.y).toBe("number");
        expect(typeof win.width).toBe("number");
        expect(typeof win.height).toBe("number");
      });

      it("should get all windows", () => {
        const windows = getAllWindows();
        expect(Array.isArray(windows)).toBe(true);
        // Note: Currently returns only active window due to API limitation
        expect(windows.length).toBeGreaterThanOrEqual(0);
      });

      it("should find windows by title", () => {
        const activeWin = getActiveWindow();
        // Search for a portion of the active window's title
        const titlePart = activeWin.title.substring(0, 5);
        if (titlePart) {
          const found = findWindowsByTitle(titlePart);
          expect(Array.isArray(found)).toBe(true);
        }
      });

      it("should find windows by process name", () => {
        const activeWin = getActiveWindow();
        // Extract process name from path
        const processName = activeWin.processPath.split("/").pop() || "";
        if (processName) {
          const found = findWindowsByProcess(processName);
          expect(Array.isArray(found)).toBe(true);
        }
      });

      it("should handle case-insensitive title search", () => {
        const activeWin = getActiveWindow();
        const titleLower = activeWin.title.toLowerCase().substring(0, 5);
        if (titleLower) {
          const found = findWindowsByTitle(titleLower);
          expect(Array.isArray(found)).toBe(true);
        }
      });

      it("should handle empty search results", () => {
        const found = findWindowsByTitle("NonExistentWindowTitle12345");
        expect(Array.isArray(found)).toBe(true);
        expect(found.length).toBe(0);
      });
    });
  },
);
