//! GUI Widget Builders for creating declarative UIs
//!
//! This module provides TypeScript wrappers for the aumate widget system,
//! allowing creation of declarative UIs with a fluent builder pattern.

import * as bot from "@tego/bot";

// ============================================================================
// Type Re-exports
// ============================================================================

/**
 * Widget style configuration for customizing appearance
 *
 * @example
 * ```typescript
 * const style: WidgetStyle = {
 *   margin: 10,
 *   padding: 8,
 *   backgroundColor: "#333333",
 *   textColor: "#FFFFFF",
 *   fontSize: 14,
 *   textAlign: "center",
 * };
 * ```
 */
export type WidgetStyle = bot.JsWidgetStyle;

/**
 * Widget class representing a UI component that can be composed into UIs.
 * Widgets use a fluent builder pattern for configuration.
 *
 * @example
 * ```typescript
 * const myButton = button("Click me")
 *   .withId("submit-btn")
 *   .withStyle({ backgroundColor: "#007AFF", textColor: "#FFFFFF" })
 *   .withTooltip("Click to submit the form");
 * ```
 */
export type Widget = bot.Widget;

// Re-export the Widget class for direct access
export { Widget } from "@tego/bot";

// ============================================================================
// Basic Widget Constructors
// ============================================================================

/**
 * Create a text label widget for displaying static text
 *
 * @param text - The text content to display
 * @returns A Widget instance
 *
 * @example
 * ```typescript
 * import { label } from "@tego/botjs";
 *
 * // Simple label
 * const title = label("Welcome to My App");
 *
 * // Styled label
 * const styledLabel = label("Important Notice")
 *   .withStyle({ fontSize: 18, textColor: "#FF0000" });
 * ```
 */
export function label(text: string): bot.Widget {
  return bot.label(text);
}

/**
 * Create a button widget that can respond to clicks
 *
 * @param text - The button label text
 * @returns A Widget instance
 *
 * @example
 * ```typescript
 * import { button } from "@tego/botjs";
 *
 * // Simple button
 * const submitBtn = button("Submit");
 *
 * // Button with ID for event handling
 * const loginBtn = button("Login")
 *   .withId("login-button")
 *   .withStyle({ backgroundColor: "#007AFF" });
 * ```
 */
export function button(text: string): bot.Widget {
  return bot.button(text);
}

/**
 * Create a text input widget for user text entry
 *
 * @returns A Widget instance
 *
 * @example
 * ```typescript
 * import { textInput } from "@tego/botjs";
 *
 * // Simple text input
 * const nameInput = textInput()
 *   .withId("name")
 *   .withPlaceholder("Enter your name");
 *
 * // Password input
 * const passwordInput = textInput()
 *   .withId("password")
 *   .withPlaceholder("Enter password")
 *   .withPassword(true);
 * ```
 */
export function textInput(): bot.Widget {
  return bot.textInput();
}

/**
 * Create a text input widget with an initial value
 *
 * @param value - The initial text value
 * @returns A Widget instance
 *
 * @example
 * ```typescript
 * import { textInputWithValue } from "@tego/botjs";
 *
 * const emailInput = textInputWithValue("user@example.com")
 *   .withId("email");
 * ```
 */
export function textInputWithValue(value: string): bot.Widget {
  return bot.textInputWithValue(value);
}

/**
 * Create a checkbox widget for boolean input
 *
 * @param labelText - The label displayed next to the checkbox
 * @param checked - Initial checked state
 * @returns A Widget instance
 *
 * @example
 * ```typescript
 * import { checkbox } from "@tego/botjs";
 *
 * // Simple checkbox
 * const rememberMe = checkbox("Remember me", false)
 *   .withId("remember");
 *
 * // Pre-checked checkbox
 * const acceptTerms = checkbox("I accept the terms", true)
 *   .withId("terms");
 * ```
 */
export function checkbox(labelText: string, checked: boolean): bot.Widget {
  return bot.checkbox(labelText, checked);
}

/**
 * Create a slider widget for numeric input within a range
 *
 * @param value - Initial slider value
 * @param min - Minimum value
 * @param max - Maximum value
 * @returns A Widget instance
 *
 * @example
 * ```typescript
 * import { slider } from "@tego/botjs";
 *
 * // Volume slider (0-100)
 * const volumeSlider = slider(50, 0, 100)
 *   .withId("volume")
 *   .withStep(5);
 *
 * // Opacity slider (0-1)
 * const opacitySlider = slider(1.0, 0, 1)
 *   .withId("opacity")
 *   .withStep(0.1);
 * ```
 */
export function slider(value: number, min: number, max: number): bot.Widget {
  return bot.slider(value, min, max);
}

/**
 * Create a progress bar widget for displaying progress
 *
 * @param value - Progress value (0.0 to 1.0)
 * @returns A Widget instance
 *
 * @example
 * ```typescript
 * import { progressBar } from "@tego/botjs";
 *
 * // 50% progress
 * const downloadProgress = progressBar(0.5)
 *   .withId("download-progress");
 *
 * // Complete progress
 * const completeProgress = progressBar(1.0);
 * ```
 */
export function progressBar(value: number): bot.Widget {
  return bot.progressBar(value);
}

/**
 * Create a horizontal separator line
 *
 * @returns A Widget instance
 *
 * @example
 * ```typescript
 * import { separator, vbox, label } from "@tego/botjs";
 *
 * const content = vbox([
 *   label("Section 1"),
 *   separator(),
 *   label("Section 2"),
 * ]);
 * ```
 */
export function separator(): bot.Widget {
  return bot.separator();
}

/**
 * Create a spacer for adding empty space in layouts
 *
 * @param size - Size of the spacer in pixels
 * @returns A Widget instance
 *
 * @example
 * ```typescript
 * import { spacer, vbox, button } from "@tego/botjs";
 *
 * const content = vbox([
 *   button("Top"),
 *   spacer(20),
 *   button("Bottom"),
 * ]);
 * ```
 */
export function spacer(size: number): bot.Widget {
  return bot.spacer(size);
}

// ============================================================================
// Layout Widget Constructors
// ============================================================================

/**
 * Create a horizontal box layout that arranges children in a row
 *
 * @param children - Array of child widgets
 * @returns A Widget instance
 *
 * @example
 * ```typescript
 * import { hbox, button, spacer } from "@tego/botjs";
 *
 * // Button row
 * const buttonRow = hbox([
 *   button("Cancel"),
 *   spacer(10),
 *   button("OK"),
 * ]).withSpacing(8);
 * ```
 */
export function hbox(children: bot.Widget[]): bot.Widget {
  return bot.hbox(children);
}

/**
 * Create a vertical box layout that arranges children in a column
 *
 * @param children - Array of child widgets
 * @returns A Widget instance
 *
 * @example
 * ```typescript
 * import { vbox, label, textInput, button } from "@tego/botjs";
 *
 * // Login form
 * const loginForm = vbox([
 *   label("Username"),
 *   textInput().withId("username"),
 *   label("Password"),
 *   textInput().withId("password").withPassword(true),
 *   button("Login").withId("login"),
 * ]).withSpacing(8);
 * ```
 */
export function vbox(children: bot.Widget[]): bot.Widget {
  return bot.vbox(children);
}

/**
 * Create a grid layout that arranges widgets in rows and columns
 *
 * @param rows - 2D array where each inner array represents a row of widgets
 * @returns A Widget instance
 *
 * @example
 * ```typescript
 * import { grid, label, textInput, button } from "@tego/botjs";
 *
 * // Form grid
 * const formGrid = grid([
 *   [label("Name:"), textInput().withId("name")],
 *   [label("Email:"), textInput().withId("email")],
 *   [label(""), button("Submit").withId("submit")],
 * ]);
 * ```
 */
export function grid(rows: bot.Widget[][]): bot.Widget {
  return bot.grid(rows);
}

// ============================================================================
// Container Widget Constructors
// ============================================================================

/**
 * Create a panel container with a background
 *
 * @param child - The child widget to wrap in the panel
 * @returns A Widget instance
 *
 * @example
 * ```typescript
 * import { panel, vbox, label } from "@tego/botjs";
 *
 * const infoPanel = panel(
 *   vbox([
 *     label("Info"),
 *     label("This is a panel"),
 *   ])
 * ).withStyle({ backgroundColor: "#F0F0F0" });
 * ```
 */
export function panel(child: bot.Widget): bot.Widget {
  return bot.panel(child);
}

/**
 * Create a scroll area container for scrollable content
 *
 * @param child - The child widget to make scrollable
 * @returns A Widget instance
 *
 * @example
 * ```typescript
 * import { scrollArea, vbox, label } from "@tego/botjs";
 *
 * // Scrollable list
 * const scrollableList = scrollArea(
 *   vbox([
 *     label("Item 1"),
 *     label("Item 2"),
 *     label("Item 3"),
 *     // ... many more items
 *   ])
 * ).withMaxHeight(200);
 * ```
 */
export function scrollArea(child: bot.Widget): bot.Widget {
  return bot.scrollArea(child);
}

/**
 * Create a collapsible group container with a title
 *
 * @param title - The group title displayed in the header
 * @param child - The child widget contained in the group
 * @returns A Widget instance
 *
 * @example
 * ```typescript
 * import { group, vbox, checkbox } from "@tego/botjs";
 *
 * // Settings group
 * const settingsGroup = group("Advanced Settings",
 *   vbox([
 *     checkbox("Enable feature A", false),
 *     checkbox("Enable feature B", true),
 *   ])
 * ).withCollapsed(true);
 * ```
 */
export function group(title: string, child: bot.Widget): bot.Widget {
  return bot.group(title, child);
}

// ============================================================================
// Image Widget Constructor
// ============================================================================

/**
 * Create an image widget from RGBA pixel data
 *
 * @param data - Buffer containing RGBA pixel data (4 bytes per pixel)
 * @param width - Image width in pixels
 * @param height - Image height in pixels
 * @returns A Widget instance
 *
 * @example
 * ```typescript
 * import { image } from "@tego/botjs";
 *
 * // Create a 100x100 red image
 * const size = 100 * 100 * 4;
 * const redPixels = Buffer.alloc(size);
 * for (let i = 0; i < size; i += 4) {
 *   redPixels[i] = 255;     // R
 *   redPixels[i + 1] = 0;   // G
 *   redPixels[i + 2] = 0;   // B
 *   redPixels[i + 3] = 255; // A
 * }
 * const redImage = image(redPixels, 100, 100);
 * ```
 */
export function image(data: Buffer, width: number, height: number): bot.Widget {
  return bot.image(data, width, height);
}
