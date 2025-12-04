/**
 * GUI Font Picker Example
 *
 * Demonstrates font enumeration and font rendering:
 * - getSystemFonts: list all available system fonts
 * - withFontFamily: apply font to widgets
 */

import {
  button,
  dropdown,
  GuiApp,
  getSystemFonts,
  hbox,
  label,
  panel,
  separator,
  slider,
  vbox,
} from "@tego/botjs";

async function main() {
  console.log("GUI Font Picker Example\n");

  // Get system fonts before creating the UI
  console.log("Loading system fonts...");
  const fonts = await getSystemFonts();
  console.log(`Found ${fonts.length} fonts\n`);

  // Show first 10 fonts as preview
  console.log("First 10 fonts:");
  for (const font of fonts.slice(0, 10)) {
    console.log(`  - ${font}`);
  }
  console.log("  ...\n");

  const app = new GuiApp();

  const win = app.createWindow({
    title: "Font Picker Demo",
    width: 500,
    height: 450,
    alwaysOnTop: true,
  });

  let selectedFont = "";
  let fontSize = 24;

  // Function to build the UI content with current font settings
  const buildContent = () => {
    const previewLabel = label(
      selectedFont
        ? `Preview: ${selectedFont}\nThe quick brown fox jumps over the lazy dog.\n中文字体预览：你好世界`
        : "Select a font from the dropdown above to see a preview here.\n选择一个字体来预览效果。",
    ).withId("preview-label");

    // Apply font family if selected
    const styledPreview = selectedFont
      ? previewLabel.withFontFamily(selectedFont).withStyle({ fontSize })
      : previewLabel;

    return panel(
      vbox([
        label("System Font Picker").withStyle({ fontSize: 18 }),
        separator(),

        hbox([
          label("Font:"),
          dropdown(fonts).withId("font-dropdown"),
        ]).withSpacing(8),

        hbox([
          label("Size:"),
          slider(fontSize, 12, 48).withId("font-size").withStep(2),
          label(`${fontSize}px`).withId("size-label"),
        ]).withSpacing(8),

        separator(),

        label("Preview:"),
        panel(styledPreview).withStyle({
          padding: 12,
          minHeight: 120,
          backgroundColor: "#f5f5f5",
        }),

        separator(),

        hbox([
          button("Reset").withId("reset-btn"),
          button("Apply").withId("apply-btn"),
        ]).withSpacing(8),

        label("").withId("status-label"),
      ]).withSpacing(10),
    ).withStyle({ padding: 16 });
  };

  // Set initial content
  win.setContent(buildContent());

  win.onEvent((event) => {
    console.log(`Event: ${event.eventType}, Widget: ${event.widgetId}`);

    switch (event.eventType) {
      case "selection_changed":
        if (event.widgetId === "font-dropdown" && event.value) {
          selectedFont = event.value;
          console.log(`Selected font: ${selectedFont}`);
          // Rebuild UI with new font
          win.setContent(buildContent());
          win.updateWidget("status-label", {
            value: `Applied: ${selectedFont}`,
          });
        }
        break;

      case "slider_changed":
        if (event.widgetId === "font-size" && event.numberValue !== undefined) {
          fontSize = Math.round(event.numberValue);
          console.log(`Font size: ${fontSize}px`);
          win.updateWidget("size-label", { value: `${fontSize}px` });
        }
        break;

      case "button_click":
        if (event.widgetId === "reset-btn") {
          selectedFont = "";
          fontSize = 24;
          win.setContent(buildContent());
          win.updateWidget("status-label", { value: "Reset to defaults" });
          console.log("Reset to defaults");
        } else if (event.widgetId === "apply-btn") {
          // Rebuild with current settings
          win.setContent(buildContent());
          win.updateWidget("status-label", {
            value: selectedFont
              ? `Applied: ${selectedFont} at ${fontSize}px`
              : "No font selected",
          });
          console.log(`Applied: ${selectedFont} at ${fontSize}px`);
        }
        break;
    }
  });

  console.log("Showing window...");
  win.show();

  console.log("Select a font from the dropdown to preview it.");
  console.log("Press Ctrl+C to exit.\n");

  app.run();

  console.log("Application closed!");
}

main().catch(console.error);
