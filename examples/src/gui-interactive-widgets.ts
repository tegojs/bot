/**
 * GUI Interactive Widgets Example
 *
 * Demonstrates the new interactive widgets:
 * - Link: clickable text that fires link_clicked event
 * - SelectableLabel: toggle-able label with selection state
 * - DragValue: numeric input with drag-to-change
 * - ColorPicker: color selection with optional alpha
 * - Hyperlink: opens URL in browser
 * - ImageButton: button with image
 */

import {
  colorPicker,
  dragValue,
  GuiApp,
  hbox,
  hyperlink,
  hyperlinkUrl,
  imageButton,
  label,
  link,
  panel,
  selectableLabel,
  separator,
  tabs,
  vbox,
} from "@tego/botjs";

// Helper to create a colored image buffer
function createColoredImage(
  width: number,
  height: number,
  r: number,
  g: number,
  b: number,
): Buffer {
  const size = width * height * 4;
  const pixels = Buffer.alloc(size);
  for (let i = 0; i < size; i += 4) {
    pixels[i] = r;
    pixels[i + 1] = g;
    pixels[i + 2] = b;
    pixels[i + 3] = 255;
  }
  return pixels;
}

async function main() {
  console.log("GUI Interactive Widgets Example\n");

  const app = new GuiApp();

  const win = app.createWindow({
    title: "Interactive Widgets Gallery",
    width: 550,
    height: 500,
    alwaysOnTop: true,
  });

  // Create image buttons with different colors
  const redIcon = createColoredImage(24, 24, 255, 80, 80);
  const greenIcon = createColoredImage(24, 24, 80, 200, 80);
  const blueIcon = createColoredImage(24, 24, 80, 120, 255);

  win.setContent(
    panel(
      tabs([
        {
          label: "Links",
          content: vbox([
            label("Link Widget (fires event when clicked):"),
            link("Click me to trigger an event").withId("action-link"),
            separator(),
            label("Hyperlink (opens URL in browser):"),
            hyperlink("Visit GitHub", "https://github.com").withId(
              "github-link",
            ),
            hyperlinkUrl("https://nodejs.org").withId("nodejs-link"),
          ]).withSpacing(10),
        },
        {
          label: "Selectable",
          content: vbox([
            label("Selectable Labels (click to toggle):"),
            selectableLabel("Item 1 - Not selected", false).withId("item-1"),
            selectableLabel("Item 2 - Pre-selected", true).withId("item-2"),
            selectableLabel("Item 3 - Not selected", false).withId("item-3"),
            separator(),
            label("Use these for list selection UIs"),
          ]).withSpacing(8),
        },
        {
          label: "DragValue",
          content: vbox([
            label("Drag Value Widgets (drag to change):"),
            hbox([
              label("Simple:"),
              dragValue(50).withId("simple-drag").withRange(0, 100),
            ]).withSpacing(8),
            hbox([
              label("With prefix:"),
              dragValue(9.99)
                .withId("price-drag")
                .withPrefix("$")
                .withDecimals(2)
                .withRange(0, 100),
            ]).withSpacing(8),
            hbox([
              label("With suffix:"),
              dragValue(75)
                .withId("percent-drag")
                .withSuffix("%")
                .withRange(0, 100)
                .withSpeed(0.5),
            ]).withSpacing(8),
            hbox([
              label("Fast speed:"),
              dragValue(500)
                .withId("fast-drag")
                .withRange(0, 1000)
                .withSpeed(5.0),
            ]).withSpacing(8),
          ]).withSpacing(10),
        },
        {
          label: "Color",
          content: vbox([
            label("Color Picker (with alpha):"),
            colorPicker([255, 100, 100, 255])
              .withId("color-alpha")
              .withAlpha(true),
            separator(),
            label("Color Picker (RGB only):"),
            colorPicker([100, 150, 255, 255])
              .withId("color-rgb")
              .withAlpha(false),
          ]).withSpacing(12),
        },
        {
          label: "ImageBtn",
          content: vbox([
            label("Image Buttons:"),
            hbox([
              imageButton(redIcon, 24, 24).withId("red-btn").withFrame(true),
              imageButton(greenIcon, 24, 24)
                .withId("green-btn")
                .withFrame(true),
              imageButton(blueIcon, 24, 24).withId("blue-btn").withFrame(true),
            ]).withSpacing(8),
            separator(),
            label("Without frame:"),
            hbox([
              imageButton(redIcon, 24, 24)
                .withId("red-btn-noframe")
                .withFrame(false),
              imageButton(greenIcon, 24, 24)
                .withId("green-btn-noframe")
                .withFrame(false),
              imageButton(blueIcon, 24, 24)
                .withId("blue-btn-noframe")
                .withFrame(false),
            ]).withSpacing(8),
          ]).withSpacing(10),
        },
      ]).withId("main-tabs"),
    ).withStyle({ padding: 12 }),
  );

  win.onEvent((event) => {
    console.log(`Event: ${event.eventType}`);
    console.log(`  Widget ID: ${event.widgetId}`);

    switch (event.eventType) {
      case "link_clicked":
        console.log("  Link was clicked!");
        break;

      case "selectable_label_changed":
        console.log(`  Selected: ${event.checked}`);
        break;

      case "drag_value_changed":
        console.log(`  Value: ${event.numberValue}`);
        break;

      case "color_changed":
        console.log(`  Color: ${JSON.stringify(event.color)}`);
        break;

      case "hyperlink_clicked":
        console.log(`  URL: ${event.value}`);
        break;

      case "button_click":
        console.log(`  Image button clicked: ${event.widgetId}`);
        break;

      case "tab_changed":
        console.log(`  Tab: ${event.value}`);
        break;

      default:
        if (event.value) console.log(`  Value: ${event.value}`);
        if (event.numberValue !== undefined)
          console.log(`  Number: ${event.numberValue}`);
    }
    console.log("");
  });

  console.log("Showing window...");
  win.show();

  console.log("Window shown. Interact with widgets to see events.");
  console.log("Press Ctrl+C to exit.\n");

  app.run();

  console.log("Application closed!");
}

main().catch(console.error);
