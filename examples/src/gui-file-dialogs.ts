/**
 * GUI File Dialogs Example
 *
 * Demonstrates native file dialog functions (non-blocking):
 * - showOpenFileDialog: open file picker with filters
 * - showSaveFileDialog: save file picker
 * - showFolderDialog: folder picker
 *
 * Note: File dialogs require GuiApp to be initialized first.
 * Results are delivered via the file_dialog_completed event.
 */

import {
  button,
  GuiApp,
  label,
  panel,
  separator,
  showFolderDialog,
  showOpenFileDialog,
  showSaveFileDialog,
  textArea,
  vbox,
} from "@tego/botjs";

const WINDOW_NAME = "File Dialog Demo";

async function main() {
  console.log("GUI File Dialogs Example\n");

  // Create the GUI application (required for file dialogs to work)
  const app = new GuiApp();

  const win = app.createWindow({
    title: WINDOW_NAME,
    width: 450,
    height: 450,
    alwaysOnTop: true,
  });

  win.setContent(
    panel(
      vbox([
        label("File Dialog Examples"),
        separator(),

        button("Open File Dialog").withId("open-file"),
        button("Open Multiple Files").withId("open-multiple"),
        button("Open Images Only").withId("open-images"),

        separator(),

        button("Save File Dialog").withId("save-file"),
        button("Save as JSON").withId("save-json"),

        separator(),

        button("Select Folder").withId("select-folder"),

        separator(),

        label("Result:"),
        textArea().withId("result").withRows(6),
      ]).withSpacing(8),
    ).withStyle({ padding: 16 }),
  );

  win.onEvent((event) => {
    console.log(`Event: ${event.eventType}, Widget: ${event.widgetId}`);

    // Handle button clicks - trigger file dialogs
    if (event.eventType === "button_click") {
      switch (event.widgetId) {
        case "open-file":
          showOpenFileDialog("open-file-request", WINDOW_NAME, {
            title: "Select a file",
          });
          break;

        case "open-multiple":
          showOpenFileDialog("open-multiple-request", WINDOW_NAME, {
            title: "Select multiple files",
            multiple: true,
          });
          break;

        case "open-images":
          showOpenFileDialog("open-images-request", WINDOW_NAME, {
            title: "Select an image",
            filters: [
              {
                name: "Images",
                extensions: ["png", "jpg", "jpeg", "gif", "webp"],
              },
              { name: "All Files", extensions: ["*"] },
            ],
          });
          break;

        case "save-file":
          showSaveFileDialog("save-file-request", WINDOW_NAME, {
            title: "Save file as",
            defaultName: "document.txt",
          });
          break;

        case "save-json":
          showSaveFileDialog("save-json-request", WINDOW_NAME, {
            title: "Save JSON file",
            defaultName: "data.json",
            filters: [
              { name: "JSON files", extensions: ["json"] },
              { name: "All Files", extensions: ["*"] },
            ],
          });
          break;

        case "select-folder":
          showFolderDialog("select-folder-request", WINDOW_NAME, {
            title: "Select a folder",
          });
          break;
      }
      return;
    }

    // Handle file dialog results
    if (event.eventType === "file_dialog_completed") {
      let resultText = "";

      if (event.cancelled) {
        resultText = `Dialog cancelled (${event.widgetId})`;
      } else {
        const paths = event.paths ?? [];
        switch (event.widgetId) {
          case "open-file-request":
            resultText = `Selected:\n${paths.join("\n")}`;
            break;

          case "open-multiple-request":
            resultText = `Selected ${paths.length} file(s):\n${paths.join("\n")}`;
            break;

          case "open-images-request":
            resultText = `Selected image:\n${paths.join("\n")}`;
            break;

          case "save-file-request":
            resultText = `Save to:\n${paths[0]}`;
            break;

          case "save-json-request":
            resultText = `Save JSON to:\n${paths[0]}`;
            break;

          case "select-folder-request":
            resultText = `Selected folder:\n${paths[0]}`;
            break;

          default:
            resultText = `Unknown request: ${event.widgetId}\nPaths: ${paths.join(", ")}`;
        }
      }

      console.log(resultText);
      win.updateWidget("result", { value: resultText });
    }
  });

  console.log("Showing window...");
  win.show();

  console.log("Click buttons to open file dialogs.");
  console.log("Press Ctrl+C to exit.\n");

  app.run();

  console.log("Application closed!");
}

main().catch(console.error);
