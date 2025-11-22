import type { Bitmap, MousePosition, ScreenSize } from "@tego/bot";
import {
  // Global functions
  bitmapColorAt,
  dragMouse,
  getMousePos,
  getPixelColor,
  getScreen,
  getScreenSize,
  // Classes
  Keyboard,
  keyTap,
  keyToggle,
  Mouse,
  mouseClick,
  mouseToggle,
  moveMouse,
  moveMouseSmooth,
  Screen,
  scrollMouse,
  setKeyboardDelay,
  setMouseDelay,
  typeString,
  typeStringDelayed,
  unicodeTap,
  updateScreenMetrics,
} from "@tego/bot";
import { describe, expect, it } from "vitest";

describe("@tego/bot", () => {
  describe("Exports", () => {
    it("should export Keyboard class", () => {
      expect(Keyboard).toBeDefined();
      expect(typeof Keyboard).toBe("function");
    });

    it("should export Mouse class", () => {
      expect(Mouse).toBeDefined();
      expect(typeof Mouse).toBe("function");
    });

    it("should export Screen class", () => {
      expect(Screen).toBeDefined();
      expect(typeof Screen).toBe("function");
    });

    it("should export all global functions", () => {
      expect(typeof getMousePos).toBe("function");
      expect(typeof getScreenSize).toBe("function");
      expect(typeof getScreen).toBe("function");
      expect(typeof getPixelColor).toBe("function");
      expect(typeof moveMouse).toBe("function");
      expect(typeof moveMouseSmooth).toBe("function");
      expect(typeof mouseClick).toBe("function");
      expect(typeof mouseToggle).toBe("function");
      expect(typeof dragMouse).toBe("function");
      expect(typeof scrollMouse).toBe("function");
      expect(typeof keyTap).toBe("function");
      expect(typeof keyToggle).toBe("function");
      expect(typeof typeString).toBe("function");
      expect(typeof typeStringDelayed).toBe("function");
      expect(typeof unicodeTap).toBe("function");
      expect(typeof setKeyboardDelay).toBe("function");
      expect(typeof setMouseDelay).toBe("function");
      expect(typeof updateScreenMetrics).toBe("function");
      expect(typeof bitmapColorAt).toBe("function");
    });
  });

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

    it("should get mouse position", () => {
      const mouse = new Mouse();
      const pos = mouse.getMousePos();
      expect(pos).toBeDefined();
      expect(typeof pos.x).toBe("number");
      expect(typeof pos.y).toBe("number");
      expect(pos.x).toBeGreaterThanOrEqual(0);
      expect(pos.y).toBeGreaterThanOrEqual(0);
    });
  });

  describe("Screen class", () => {
    it("should create Screen instance", () => {
      const screen = new Screen();
      expect(screen).toBeDefined();
      expect(screen).toBeInstanceOf(Screen);
    });

    it("should have capture method", () => {
      const screen = new Screen();
      expect(typeof screen.capture).toBe("function");
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
  });

  describe("Global functions - Mouse", () => {
    it("should get mouse position", () => {
      const pos = getMousePos();
      expect(pos).toBeDefined();
      expect(typeof pos.x).toBe("number");
      expect(typeof pos.y).toBe("number");
      expect(pos.x).toBeGreaterThanOrEqual(0);
      expect(pos.y).toBeGreaterThanOrEqual(0);
    });

    it("should set mouse delay", () => {
      expect(() => {
        setMouseDelay(50);
      }).not.toThrow();
    });

    it("should move mouse (no-op in test)", () => {
      // Note: This would actually move the mouse, so we just test it doesn't throw
      // In a real test environment, you might want to mock this
      expect(() => {
        moveMouse(100, 200);
      }).not.toThrow();
    });

    it("should move mouse smoothly (no-op in test)", () => {
      expect(() => {
        moveMouseSmooth(300, 400);
      }).not.toThrow();
    });

    it("should move mouse smoothly with speed (no-op in test)", () => {
      expect(() => {
        moveMouseSmooth(500, 600, 5.0);
      }).not.toThrow();
    });
  });

  describe("Global functions - Keyboard", () => {
    it("should set keyboard delay", () => {
      expect(() => {
        setKeyboardDelay(10);
      }).not.toThrow();
    });
  });

  describe("Global functions - Screen", () => {
    it("should get screen size", () => {
      const size = getScreenSize();
      expect(size).toBeDefined();
      expect(typeof size.width).toBe("number");
      expect(typeof size.height).toBe("number");
      expect(size.width).toBeGreaterThan(0);
      expect(size.height).toBeGreaterThan(0);
    });

    it("should get screen instance", () => {
      const screen = getScreen();
      expect(screen).toBeDefined();
      expect(screen).toBeInstanceOf(Screen);
    });

    it("should get pixel color", async () => {
      const color = await getPixelColor(100, 200);
      expect(color).toBeDefined();
      expect(typeof color).toBe("string");
      expect(color).toMatch(/^#[0-9a-fA-F]{6}$/);
    });

    it("should update screen metrics", () => {
      expect(() => {
        updateScreenMetrics();
      }).not.toThrow();
    });
  });

  describe("Bitmap operations", () => {
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
  });

  describe("Type definitions", () => {
    it("should have correct MousePosition type", () => {
      const pos: MousePosition = getMousePos();
      expect(pos).toHaveProperty("x");
      expect(pos).toHaveProperty("y");
      expect(typeof pos.x).toBe("number");
      expect(typeof pos.y).toBe("number");
    });

    it("should have correct ScreenSize type", () => {
      const size: ScreenSize = getScreenSize();
      expect(size).toHaveProperty("width");
      expect(size).toHaveProperty("height");
      expect(typeof size.width).toBe("number");
      expect(typeof size.height).toBe("number");
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
  });
});
