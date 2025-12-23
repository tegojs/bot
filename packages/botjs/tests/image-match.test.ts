/**
 * Unit tests for image-match module
 */
import type { ImageResource, MatchConfig, MatchResult } from "@tego/botjs";
import {
  getMatchBounds,
  getMatchCenter,
  imageResourceFromBuffer,
} from "@tego/botjs";
import { describe, expect, it } from "vitest";

describe("Image Match Module", () => {
  describe("Type Exports", () => {
    it("should have ImageResource type", () => {
      const resource: ImageResource = {
        buffer: Buffer.from([]),
        path: "./test.png",
      };
      expect(resource).toBeDefined();
      expect(resource.buffer).toBeInstanceOf(Buffer);
    });

    it("should have MatchConfig type", () => {
      const config: MatchConfig = {
        searchMultipleScales: true,
        useGrayscale: false,
        scaleSteps: [1.0, 0.9, 0.8],
        confidence: 0.8,
        limit: 10,
      };
      expect(config).toBeDefined();
    });

    it("should have MatchResult type", () => {
      const result: MatchResult = {
        x: 100,
        y: 200,
        width: 50,
        height: 30,
        confidence: 0.95,
        scale: 1.0,
      };
      expect(result).toBeDefined();
    });
  });

  describe("imageResourceFromBuffer", () => {
    it("should create ImageResource from Buffer", () => {
      const buffer = Buffer.from([0x89, 0x50, 0x4e, 0x47]); // PNG header
      const resource = imageResourceFromBuffer(buffer);

      expect(resource).toBeDefined();
      expect(resource.buffer).toBe(buffer);
      expect(resource.path).toBeUndefined();
    });

    it("should handle empty buffer", () => {
      const buffer = Buffer.from([]);
      const resource = imageResourceFromBuffer(buffer);

      expect(resource).toBeDefined();
      expect(resource.buffer.length).toBe(0);
    });
  });

  describe("getMatchCenter", () => {
    it("should calculate center of match result", () => {
      const match: MatchResult = {
        x: 100,
        y: 200,
        width: 50,
        height: 30,
        confidence: 0.9,
        scale: 1.0,
      };

      const center = getMatchCenter(match);

      expect(center.x).toBe(125); // 100 + 50/2
      expect(center.y).toBe(215); // 200 + 30/2
    });

    it("should handle zero position", () => {
      const match: MatchResult = {
        x: 0,
        y: 0,
        width: 100,
        height: 100,
        confidence: 0.9,
        scale: 1.0,
      };

      const center = getMatchCenter(match);

      expect(center.x).toBe(50);
      expect(center.y).toBe(50);
    });

    it("should handle odd dimensions (rounds to nearest)", () => {
      const match: MatchResult = {
        x: 10,
        y: 20,
        width: 51,
        height: 31,
        confidence: 0.9,
        scale: 1.0,
      };

      const center = getMatchCenter(match);

      expect(center.x).toBe(36); // Math.round(10 + 51/2) = Math.round(35.5) = 36
      expect(center.y).toBe(36); // Math.round(20 + 31/2) = Math.round(35.5) = 36
    });

    it("should handle large coordinates", () => {
      const match: MatchResult = {
        x: 1920,
        y: 1080,
        width: 200,
        height: 150,
        confidence: 0.9,
        scale: 1.0,
      };

      const center = getMatchCenter(match);

      expect(center.x).toBe(2020); // 1920 + 200/2
      expect(center.y).toBe(1155); // 1080 + 150/2
    });
  });

  describe("getMatchBounds", () => {
    it("should return correct bounds", () => {
      const match: MatchResult = {
        x: 100,
        y: 200,
        width: 50,
        height: 30,
        confidence: 0.9,
        scale: 1.0,
      };

      const bounds = getMatchBounds(match);

      expect(bounds.left).toBe(100);
      expect(bounds.top).toBe(200);
      expect(bounds.right).toBe(150); // 100 + 50
      expect(bounds.bottom).toBe(230); // 200 + 30
    });

    it("should handle zero position", () => {
      const match: MatchResult = {
        x: 0,
        y: 0,
        width: 100,
        height: 100,
        confidence: 0.9,
        scale: 1.0,
      };

      const bounds = getMatchBounds(match);

      expect(bounds.left).toBe(0);
      expect(bounds.top).toBe(0);
      expect(bounds.right).toBe(100);
      expect(bounds.bottom).toBe(100);
    });

    it("should handle large coordinates", () => {
      const match: MatchResult = {
        x: 1920,
        y: 1080,
        width: 200,
        height: 150,
        confidence: 0.9,
        scale: 1.0,
      };

      const bounds = getMatchBounds(match);

      expect(bounds.left).toBe(1920);
      expect(bounds.top).toBe(1080);
      expect(bounds.right).toBe(2120);
      expect(bounds.bottom).toBe(1230);
    });
  });

  describe("Function Exports", () => {
    it("should export imageResource function", async () => {
      const { imageResource } = await import("@tego/botjs");
      expect(typeof imageResource).toBe("function");
    });

    it("should export imageResourceSync function", async () => {
      const { imageResourceSync } = await import("@tego/botjs");
      expect(typeof imageResourceSync).toBe("function");
    });

    it("should export findOnScreen function", async () => {
      const { findOnScreen } = await import("@tego/botjs");
      expect(typeof findOnScreen).toBe("function");
    });

    it("should export findAllOnScreen function", async () => {
      const { findAllOnScreen } = await import("@tego/botjs");
      expect(typeof findAllOnScreen).toBe("function");
    });

    it("should export findInRegion function", async () => {
      const { findInRegion } = await import("@tego/botjs");
      expect(typeof findInRegion).toBe("function");
    });

    it("should export findAllInRegion function", async () => {
      const { findAllInRegion } = await import("@tego/botjs");
      expect(typeof findAllInRegion).toBe("function");
    });

    it("should export waitFor function", async () => {
      const { waitFor } = await import("@tego/botjs");
      expect(typeof waitFor).toBe("function");
    });

    it("should export waitForGone function", async () => {
      const { waitForGone } = await import("@tego/botjs");
      expect(typeof waitForGone).toBe("function");
    });
  });
});
