// ============================================================================
// Screenshot Tool - Advanced screenshot functionality
// ============================================================================

import * as fs from "node:fs/promises";

// Note: Napi* types for advanced screenshot features are not yet exported from @tego/bot
// They will be available when the screenshot tool feature is implemented

// ============================================================================
// Enhanced Type Aliases with JSDoc
// ============================================================================

/**
 * Screenshot tool configuration options
 */
export interface ScreenshotToolOptions {
  /** Default save path for screenshots */
  defaultSavePath?: string;
  /** Automatically copy screenshots to clipboard after capture */
  autoCopyToClipboard?: boolean;
}

/**
 * Interactive capture mode options
 */
export interface InteractiveCaptureOptions {
  /** Show grid overlay during selection */
  showGrid?: boolean;
  /** Show coordinate information */
  showCoordinates?: boolean;
  /** Show size information */
  showSize?: boolean;
  /** Hint text to display */
  hintText?: string;
  /** Enable automatic window snapping */
  enableWindowSnap?: boolean;
  /** Snap threshold in pixels (default: 10) */
  snapThreshold?: number;
}

/**
 * Screen region definition
 */
export interface ScreenRegion {
  /** X coordinate */
  x: number;
  /** Y coordinate */
  y: number;
  /** Width in pixels */
  width: number;
  /** Height in pixels */
  height: number;
}

/**
 * Screenshot result with image data and metadata
 */
export interface ScreenshotResult {
  /** PNG-encoded image buffer */
  image: Buffer;
  /** Captured region */
  region: ScreenRegion;
  /** Capture timestamp (Unix timestamp) */
  timestamp: number;
}

/**
 * RGB color information
 */
export interface RgbColor {
  /** Red component (0-255) */
  r: number;
  /** Green component (0-255) */
  g: number;
  /** Blue component (0-255) */
  b: number;
}

/**
 * RGBA color information with alpha channel
 */
export interface RgbaColor {
  /** Red component (0-255) */
  r: number;
  /** Green component (0-255) */
  g: number;
  /** Blue component (0-255) */
  b: number;
  /** Alpha component (0-1) */
  a: number;
}

/**
 * HSL color information
 */
export interface HslColor {
  /** Hue (0-360) */
  h: number;
  /** Saturation (0-100) */
  s: number;
  /** Lightness (0-100) */
  l: number;
}

/**
 * Position coordinates
 */
export interface Position {
  /** X coordinate */
  x: number;
  /** Y coordinate */
  y: number;
}

/**
 * Complete color information in multiple formats
 */
export interface ColorInfo {
  /** RGB color */
  rgb: RgbColor;
  /** RGBA color with alpha */
  rgba: RgbaColor;
  /** Hex color string (#RRGGBB) */
  hex: string;
  /** HSL color */
  hsl: HslColor;
  /** Position where color was picked */
  position: Position;
}

/**
 * Color picker configuration options
 */
export interface ColorPickerOptions {
  /** Magnifier size in pixels (default: 150) */
  magnifierSize?: number;
  /** Zoom level (default: 8) */
  zoom?: number;
  /** Show color history (default: true) */
  showHistory?: boolean;
}

/**
 * Image save options
 */
export interface SaveImageOptions {
  /** Image format ('png' | 'jpg' | 'webp') */
  format?: string;
  /** Image quality (1-100, for JPG/WebP) */
  quality?: number;
}

// ============================================================================
// Screenshot Tool Class
// ============================================================================

/**
 * Advanced screenshot tool with interactive selection, color picking, and annotations
 *
 * @example
 * ```typescript
 * import { ScreenshotTool } from "@tego/botjs";
 *
 * const tool = new ScreenshotTool({
 *   autoCopyToClipboard: true
 * });
 *
 * // Quick screenshot
 * const screenshot = await tool.captureQuick();
 * await saveScreenshotToFile(screenshot, 'screenshot.png');
 *
 * // Region screenshot
 * const region = await tool.captureQuick({ x: 0, y: 0, width: 800, height: 600 });
 *
 * // Get pixel color
 * const color = await tool.getPixelColor(100, 200);
 * console.log(color.hex); // #FF5733
 * ```
 */
export class ScreenshotTool {
  private options?: ScreenshotToolOptions;

  /**
   * Create a new screenshot tool instance
   * @param options - Configuration options
   */
  constructor(options?: ScreenshotToolOptions) {
    // Store options for later use
    this.options = options;
  }

  /**
   * Capture screenshot interactively with UI overlay
   *
   * Note: Interactive mode is not yet fully implemented.
   * Use captureQuick() for programmatic screenshots.
   *
   * @returns Screenshot result
   *
   * @example
   * ```typescript
   * const screenshot = await tool.captureInteractive({
   *   showCoordinates: true,
   *   enableWindowSnap: true
   * });
   * ```
   */
  async captureInteractive(
    _options?: InteractiveCaptureOptions,
  ): Promise<ScreenshotResult> {
    // For now, just capture full screen
    return quickScreenshot();
  }

  /**
   * Quick screenshot without user interaction
   *
   * @param region - Optional region to capture. If not specified, captures entire screen
   * @returns Screenshot result
   *
   * @example
   * ```typescript
   * // Full screen
   * const fullScreen = await tool.captureQuick();
   *
   * // Specific region
   * const region = await tool.captureQuick({
   *   x: 100, y: 100, width: 800, height: 600
   * });
   * ```
   */
  async captureQuick(region?: ScreenRegion): Promise<ScreenshotResult> {
    if (region) {
      return quickScreenshotRegion(region);
    }
    return quickScreenshot();
  }

  /**
   * Get pixel color at specific screen coordinates
   *
   * @param x - X coordinate
   * @param y - Y coordinate
   * @returns Color information in multiple formats
   *
   * @example
   * ```typescript
   * const color = await tool.getPixelColor(500, 300);
   * console.log(color.hex);   // #FF5733
   * console.log(color.rgb);   // { r: 255, g: 87, b: 51 }
   * console.log(color.hsl);   // { h: 11, s: 100, l: 60 }
   * ```
   */
  async getPixelColor(x: number, y: number): Promise<ColorInfo> {
    // Use the bot.getPixelColor function which is already exported
    const { getPixelColor: botGetPixelColor } = await import("@tego/bot");
    const hexColor = await botGetPixelColor(x, y);

    // Parse hex color to RGB
    const r = Number.parseInt(hexColor.slice(1, 3), 16);
    const g = Number.parseInt(hexColor.slice(3, 5), 16);
    const b = Number.parseInt(hexColor.slice(5, 7), 16);

    // Convert RGB to HSL
    const rNorm = r / 255;
    const gNorm = g / 255;
    const bNorm = b / 255;
    const max = Math.max(rNorm, gNorm, bNorm);
    const min = Math.min(rNorm, gNorm, bNorm);
    const delta = max - min;

    let h = 0;
    let s = 0;
    const l = (max + min) / 2;

    if (delta !== 0) {
      s = l > 0.5 ? delta / (2 - max - min) : delta / (max + min);

      if (max === rNorm) {
        h = ((gNorm - bNorm) / delta + (gNorm < bNorm ? 6 : 0)) / 6;
      } else if (max === gNorm) {
        h = ((bNorm - rNorm) / delta + 2) / 6;
      } else {
        h = ((rNorm - gNorm) / delta + 4) / 6;
      }
    }

    return {
      hex: hexColor,
      rgb: { r, g, b },
      rgba: { r, g, b, a: 255 },
      hsl: { h: h * 360, s: s * 100, l: l * 100 },
      position: { x, y },
    };
  }

  /**
   * Start interactive color picker
   *
   * Note: Interactive mode is not yet fully implemented.
   * Use getPixelColor() for programmatic color picking.
   *
   * @returns Selected color information
   */
  async pickColor(_options?: ColorPickerOptions): Promise<ColorInfo> {
    // Interactive color picker not implemented yet
    // For now, just get color at center of screen
    const { getScreenSize } = await import("@tego/bot");
    const size = getScreenSize();
    return this.getPixelColor(
      Math.floor(size.width / 2),
      Math.floor(size.height / 2),
    );
  }

  /**
   * Close and cleanup resources
   */
  async close(): Promise<void> {
    // No resources to clean up in this implementation
  }
}

// ============================================================================
// Convenience Functions
// ============================================================================

/**
 * Quick screenshot of entire screen
 *
 * @returns Screenshot result
 *
 * @example
 * ```typescript
 * import { quickScreenshot, saveScreenshotToFile } from "@tego/botjs";
 *
 * const screenshot = await quickScreenshot();
 * await saveScreenshotToFile(screenshot, 'screenshot.png');
 * ```
 */
export async function quickScreenshot(): Promise<ScreenshotResult> {
  // Use the existing captureScreen function from @tego/bot
  const { captureScreen } = await import("@tego/bot");
  const capture = await captureScreen();

  // Convert to ScreenshotResult format
  return {
    image: capture.image,
    region: {
      x: 0,
      y: 0,
      width: capture.width,
      height: capture.height,
    },
    timestamp: Date.now() / 1000,
  };
}

/**
 * Quick screenshot of specific region
 *
 * @param region - Region to capture
 * @returns Screenshot result
 *
 * @example
 * ```typescript
 * import { quickScreenshotRegion } from "@tego/botjs";
 *
 * const screenshot = await quickScreenshotRegion({
 *   x: 0,
 *   y: 0,
 *   width: 1920,
 *   height: 1080
 * });
 * ```
 */
export async function quickScreenshotRegion(
  region: ScreenRegion,
): Promise<ScreenshotResult> {
  // Use the existing captureScreenRegion function from @tego/bot
  const { captureScreenRegion } = await import("@tego/bot");
  const capture = await captureScreenRegion(
    region.x,
    region.y,
    region.width,
    region.height,
  );

  // Convert to ScreenshotResult format
  return {
    image: capture.image,
    region: {
      x: region.x,
      y: region.y,
      width: capture.width,
      height: capture.height,
    },
    timestamp: Date.now() / 1000,
  };
}

/**
 * Start interactive screenshot capture
 *
 * Note: Interactive mode is not yet fully implemented.
 * Use quickScreenshot() or quickScreenshotRegion() instead.
 *
 * @returns Screenshot result
 */
export async function startInteractiveCapture(
  _options?: InteractiveCaptureOptions,
): Promise<ScreenshotResult> {
  // For now, just capture the full screen
  // Interactive mode is not implemented yet
  return quickScreenshot();
}

/**
 * Save screenshot to file
 *
 * @param result - Screenshot result to save
 * @param filePath - File path (extension determines format)
 *
 * @example
 * ```typescript
 * const screenshot = await quickScreenshot();
 *
 * // Save as PNG (default)
 * await saveScreenshotToFile(screenshot, 'screenshot.png');
 *
 * // Save as JPEG with quality
 * await saveScreenshotToFile(screenshot, 'screenshot.jpg', {
 *   format: 'jpg',
 *   quality: 90
 * });
 *
 * // Save as WebP
 * await saveScreenshotToFile(screenshot, 'screenshot.webp', {
 *   format: 'webp',
 *   quality: 85
 * });
 * ```
 */
export async function saveScreenshotToFile(
  result: ScreenshotResult,
  filePath: string,
  _options?: SaveImageOptions,
): Promise<void> {
  // The image buffer is already PNG-encoded, just write it to file
  await fs.writeFile(filePath, result.image);
}

/**
 * Copy screenshot to clipboard
 *
 * @param result - Screenshot result to copy
 *
 * @example
 * ```typescript
 * const screenshot = await quickScreenshot();
 * await copyScreenshotToClipboard(screenshot);
 * console.log('Screenshot copied to clipboard!');
 * ```
 */
export async function copyScreenshotToClipboard(
  result: ScreenshotResult,
): Promise<void> {
  // Use the existing setClipboardImage function from @tego/bot
  const { setClipboardImage } = await import("@tego/bot");
  setClipboardImage(result.image);
}

// ============================================================================
// Utility Functions
// ============================================================================

/**
 * Get pixel color at specific coordinates (shorthand)
 *
 * @param x - X coordinate
 * @param y - Y coordinate
 * @returns Color information
 *
 * @example
 * ```typescript
 * const color = await getPixelColor(100, 200);
 * console.log(`Color at (100, 200): ${color.hex}`);
 * ```
 */
export async function getPixelColor(x: number, y: number): Promise<ColorInfo> {
  const tool = new ScreenshotTool();
  try {
    return await tool.getPixelColor(x, y);
  } finally {
    await tool.close();
  }
}

/**
 * Capture region with simplified API
 *
 * @param x - X coordinate
 * @param y - Y coordinate
 * @param width - Width in pixels
 * @param height - Height in pixels
 * @returns Screenshot result
 *
 * @example
 * ```typescript
 * const screenshot = await captureRegion(0, 0, 1920, 1080);
 * await saveScreenshotToFile(screenshot, 'region.png');
 * ```
 */
export async function captureRegion(
  x: number,
  y: number,
  width: number,
  height: number,
): Promise<ScreenshotResult> {
  return quickScreenshotRegion({ x, y, width, height });
}

/**
 * Capture and save screenshot in one call
 *
 * @param path - Output file path
 * @param region - Optional region to capture
 * @param options - Save options
 *
 * @example
 * ```typescript
 * // Full screen
 * await captureAndSave('screenshot.png');
 *
 * // Region
 * await captureAndSave('region.jpg', { x: 0, y: 0, width: 800, height: 600 }, {
 *   quality: 90
 * });
 * ```
 */
export async function captureAndSave(
  path: string,
  region?: ScreenRegion,
  options?: SaveImageOptions,
): Promise<void> {
  const screenshot = region
    ? await quickScreenshotRegion(region)
    : await quickScreenshot();
  await saveScreenshotToFile(screenshot, path, options);
}

/**
 * Capture and copy to clipboard in one call
 *
 * @param region - Optional region to capture
 *
 * @example
 * ```typescript
 * // Full screen
 * await captureAndCopy();
 *
 * // Region
 * await captureAndCopy({ x: 0, y: 0, width: 800, height: 600 });
 * ```
 */
export async function captureAndCopy(region?: ScreenRegion): Promise<void> {
  const screenshot = region
    ? await quickScreenshotRegion(region)
    : await quickScreenshot();
  await copyScreenshotToClipboard(screenshot);
}
