import type {
  Bitmap,
  MousePosition,
  PixelColor,
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
  updateScreenMetrics,
} from "@tego/botjs";
import { describe, expect, it, vi } from "vitest";

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

    it("should export all mouse functions", () => {
      expect(typeof moveMouse).toBe("function");
      expect(typeof moveMouseSmooth).toBe("function");
      expect(typeof mouseClick).toBe("function");
      expect(typeof mouseToggle).toBe("function");
      expect(typeof dragMouse).toBe("function");
      expect(typeof scrollMouse).toBe("function");
      expect(typeof getMousePos).toBe("function");
      expect(typeof setMouseDelay).toBe("function");
    });

    it("should export all keyboard functions", () => {
      expect(typeof keyTap).toBe("function");
      expect(typeof keyToggle).toBe("function");
      expect(typeof typeString).toBe("function");
      expect(typeof typeStringDelayed).toBe("function");
      expect(typeof unicodeTap).toBe("function");
      expect(typeof setKeyboardDelay).toBe("function");
    });

    it("should export all screen functions", () => {
      expect(typeof getScreen).toBe("function");
      expect(typeof captureScreen).toBe("function");
      expect(typeof captureScreenRegion).toBe("function");
      expect(typeof getPixelColor).toBe("function");
      expect(typeof getScreenSize).toBe("function");
      expect(typeof bitmapColorAt).toBe("function");
      expect(typeof updateScreenMetrics).toBe("function");
    });

    it("should export all clipboard functions", () => {
      expect(typeof getClipboard).toBe("function");
      expect(typeof setClipboard).toBe("function");
      expect(typeof clearClipboard).toBe("function");
      expect(typeof getClipboardImage).toBe("function");
      expect(typeof setClipboardImage).toBe("function");
    });

    it("should export all window management functions", () => {
      expect(typeof getActiveWindow).toBe("function");
      expect(typeof getAllWindows).toBe("function");
      expect(typeof findWindowsByTitle).toBe("function");
      expect(typeof findWindowsByProcess).toBe("function");
    });

    it("should export all helper functions", () => {
      expect(typeof doubleClick).toBe("function");
      expect(typeof rightClick).toBe("function");
      expect(typeof middleClick).toBe("function");
      expect(typeof leftClick).toBe("function");
      expect(typeof mouseDown).toBe("function");
      expect(typeof mouseUp).toBe("function");
    });
  });

  describe("Type Exports", () => {
    it("should have Bitmap type", () => {
      // Type check - will fail at compile time if type doesn't exist
      const bitmap: Bitmap = {
        width: 100,
        height: 100,
        image: Buffer.from([]),
        byteWidth: 400,
        bitsPerPixel: 32,
        bytesPerPixel: 4,
      };
      expect(bitmap).toBeDefined();
    });

    it("should have MousePosition type", () => {
      const pos: MousePosition = { x: 0, y: 0 };
      expect(pos).toBeDefined();
    });

    it("should have ScreenSize type", () => {
      const size: ScreenSize = { width: 1920, height: 1080 };
      expect(size).toBeDefined();
    });

    it("should have ScreenCapture type", () => {
      const capture: ScreenCapture = {
        width: 100,
        height: 100,
        image: Buffer.from([]),
      };
      expect(capture).toBeDefined();
    });

    it("should have WindowInfo type", () => {
      const win: WindowInfo = {
        title: "Test",
        processId: 123,
        processPath: "/test",
        x: 0,
        y: 0,
        width: 800,
        height: 600,
      };
      expect(win).toBeDefined();
    });

    it("should have PixelColor type", () => {
      const color: PixelColor = "#FF0000";
      expect(color).toBeDefined();
    });
  });

  describe("Keyboard class", () => {
    // Note: Creating Keyboard instances requires system connection (Enigo)
    // Actual instance creation tests are in bot.integration.test.ts
    it("should have Keyboard class defined", () => {
      expect(Keyboard).toBeDefined();
      expect(typeof Keyboard).toBe("function");
    });
  });

  describe("Mouse class", () => {
    // Note: Creating Mouse instances requires system connection (Enigo)
    // Actual instance creation tests are in bot.integration.test.ts
    it("should have Mouse class defined", () => {
      expect(Mouse).toBeDefined();
      expect(typeof Mouse).toBe("function");
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
  });

  describe("Global functions - Mouse", () => {
    it("should set mouse delay", () => {
      expect(() => {
        setMouseDelay(50);
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
    it("should get screen instance", () => {
      const screen = getScreen();
      expect(screen).toBeDefined();
      expect(screen).toBeInstanceOf(Screen);
    });

    it("should update screen metrics", () => {
      expect(() => {
        updateScreenMetrics();
      }).not.toThrow();
    });
  });

  describe("Helper Functions", () => {
    // Note: Helper functions require system connection (Enigo)
    // Functional tests are in bot.integration.test.ts
    describe("doubleClick", () => {
      it("should be defined", () => {
        expect(typeof doubleClick).toBe("function");
      });
    });

    describe("rightClick", () => {
      it("should be defined", () => {
        expect(typeof rightClick).toBe("function");
      });
    });

    describe("middleClick", () => {
      it("should be defined", () => {
        expect(typeof middleClick).toBe("function");
      });
    });

    describe("leftClick", () => {
      it("should be defined", () => {
        expect(typeof leftClick).toBe("function");
      });
    });

    describe("mouseDown", () => {
      it("should be defined", () => {
        expect(typeof mouseDown).toBe("function");
      });
    });

    describe("mouseUp", () => {
      it("should be defined", () => {
        expect(typeof mouseUp).toBe("function");
      });
    });
  });
});
