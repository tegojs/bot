import type {
  Bitmap,
  bitmapColorAt,
  ColorInfo,
  captureAndCopy,
  captureAndSave,
  captureRegion,
  captureScreen,
  captureScreenRegion,
  clearClipboard,
  copyScreenshotToClipboard,
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
  MousePosition,
  middleClick,
  mouseClick,
  mouseDown,
  mouseToggle,
  mouseUp,
  moveMouse,
  moveMouseSmooth,
  PixelColor,
  quickScreenshot,
  quickScreenshotRegion,
  rightClick,
  Screen,
  ScreenCapture,
  ScreenRegion,
  ScreenSize,
  ScreenshotResult,
  ScreenshotTool,
  ScreenshotToolOptions,
  saveScreenshotToFile,
  scrollMouse,
  setClipboard,
  setClipboardImage,
  setKeyboardDelay,
  setMouseDelay,
  startInteractiveCapture,
  typeString,
  typeStringDelayed,
  unicodeTap,
  updateScreenMetrics,
  WindowInfo,
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
      expect(typeof getPixelColorHex).toBe("function");
      expect(typeof getScreenSize).toBe("function");
      expect(typeof bitmapColorAt).toBe("function");
      expect(typeof updateScreenMetrics).toBe("function");
    });

    it("should export all screenshot functions", () => {
      expect(typeof ScreenshotTool).toBe("function");
      expect(typeof quickScreenshot).toBe("function");
      expect(typeof quickScreenshotRegion).toBe("function");
      expect(typeof startInteractiveCapture).toBe("function");
      expect(typeof saveScreenshotToFile).toBe("function");
      expect(typeof copyScreenshotToClipboard).toBe("function");
      expect(typeof captureRegion).toBe("function");
      expect(typeof captureAndSave).toBe("function");
      expect(typeof captureAndCopy).toBe("function");
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

    it("should have ScreenshotResult type", () => {
      const result: ScreenshotResult = {
        image: Buffer.from([]),
        region: { x: 0, y: 0, width: 100, height: 100 },
        timestamp: Date.now() / 1000,
      };
      expect(result).toBeDefined();
    });

    it("should have ColorInfo type", () => {
      const colorInfo: ColorInfo = {
        rgb: { r: 255, g: 0, b: 0 },
        rgba: { r: 255, g: 0, b: 0, a: 1 },
        hex: "#ff0000",
        hsl: { h: 0, s: 100, l: 50 },
        position: { x: 100, y: 200 },
      };
      expect(colorInfo).toBeDefined();
    });

    it("should have ScreenRegion type", () => {
      const region: ScreenRegion = {
        x: 0,
        y: 0,
        width: 800,
        height: 600,
      };
      expect(region).toBeDefined();
    });

    it("should have ScreenshotToolOptions type", () => {
      const options: ScreenshotToolOptions = {
        defaultSavePath: "./screenshots",
        autoCopyToClipboard: false,
      };
      expect(options).toBeDefined();
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

  describe("Screenshot Tool", () => {
    it("should create ScreenshotTool instance", () => {
      const tool = new ScreenshotTool();
      expect(tool).toBeDefined();
      expect(tool).toBeInstanceOf(ScreenshotTool);
    });

    it("should create ScreenshotTool with options", () => {
      const tool = new ScreenshotTool({
        defaultSavePath: "./screenshots",
        autoCopyToClipboard: false,
      });
      expect(tool).toBeDefined();
    });

    it("should have all required methods", () => {
      const tool = new ScreenshotTool();
      expect(typeof tool.captureInteractive).toBe("function");
      expect(typeof tool.captureQuick).toBe("function");
      expect(typeof tool.getPixelColor).toBe("function");
      expect(typeof tool.pickColor).toBe("function");
      expect(typeof tool.close).toBe("function");
    });
  });

  describe("Screenshot Functions", () => {
    it("should have quickScreenshot function", () => {
      expect(typeof quickScreenshot).toBe("function");
    });

    it("should have quickScreenshotRegion function", () => {
      expect(typeof quickScreenshotRegion).toBe("function");
    });

    it("should have startInteractiveCapture function", () => {
      expect(typeof startInteractiveCapture).toBe("function");
    });

    it("should have saveScreenshotToFile function", () => {
      expect(typeof saveScreenshotToFile).toBe("function");
    });

    it("should have copyScreenshotToClipboard function", () => {
      expect(typeof copyScreenshotToClipboard).toBe("function");
    });

    it("should have captureRegion function", () => {
      expect(typeof captureRegion).toBe("function");
    });

    it("should have captureAndSave function", () => {
      expect(typeof captureAndSave).toBe("function");
    });

    it("should have captureAndCopy function", () => {
      expect(typeof captureAndCopy).toBe("function");
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
