// Generated file. Do not edit manually.
import type { Command } from "commander";
import { invokeExport } from "../commands/invoke.js";

const BOOL_TRUE = new Set(["true", "1", "yes", "y", "on"]);
const BOOL_FALSE = new Set(["false", "0", "no", "n", "off"]);
function parseArg(value: string): unknown {
  try {
    return JSON.parse(value);
  } catch {
    if (/^0x[0-9a-f]+$/i.test(value)) {
      return Number.parseInt(value, 16);
    }
    const lower = value.toLowerCase();
    if (BOOL_TRUE.has(lower)) return true;
    if (BOOL_FALSE.has(lower)) return false;
    if (/^-?\\d+(?:\\.\\d+)?$/.test(value)) {
      return Number(value);
    }
    return value;
  }
}

export function registerGeneratedCommands(program: Command): void {
  program
    .command("moveMouse")
    .description("Call @tego/botjs moveMouse")
    .argument("<x>", "x")
    .argument("<y>", "y")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (x, y, options) => {
      const args = [x, y].map((value) => parseArg(String(value)));
      await invokeExport("moveMouse", args, options);
    });

  program
    .command("moveMouseSmooth")
    .description("Call @tego/botjs moveMouseSmooth")
    .argument("<x>", "x")
    .argument("<y>", "y")
    .argument("<speed>", "speed")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (x, y, speed, options) => {
      const args = [x, y, speed].map((value) => parseArg(String(value)));
      await invokeExport("moveMouseSmooth", args, options);
    });

  program
    .command("mouseClick")
    .description("Call @tego/botjs mouseClick")
    .argument("<button>", "button")
    .argument("<double>", "double")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (button, double, options) => {
      const args = [button, double].map((value) => parseArg(String(value)));
      await invokeExport("mouseClick", args, options);
    });

  program
    .command("mouseToggle")
    .description("Call @tego/botjs mouseToggle")
    .argument("<down>", "down")
    .argument("<button>", "button")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (down, button, options) => {
      const args = [down, button].map((value) => parseArg(String(value)));
      await invokeExport("mouseToggle", args, options);
    });

  program
    .command("dragMouse")
    .description("Call @tego/botjs dragMouse")
    .argument("<x>", "x")
    .argument("<y>", "y")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (x, y, options) => {
      const args = [x, y].map((value) => parseArg(String(value)));
      await invokeExport("dragMouse", args, options);
    });

  program
    .command("scrollMouse")
    .description("Call @tego/botjs scrollMouse")
    .argument("<x>", "x")
    .argument("<y>", "y")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (x, y, options) => {
      const args = [x, y].map((value) => parseArg(String(value)));
      await invokeExport("scrollMouse", args, options);
    });

  program
    .command("getMousePos")
    .description("Call @tego/botjs getMousePos")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (options) => {
      const args: unknown[] = [];
      await invokeExport("getMousePos", args, options);
    });

  program
    .command("setMouseDelay")
    .description("Call @tego/botjs setMouseDelay")
    .argument("<delay>", "delay")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (delay, options) => {
      const args = [delay].map((value) => parseArg(String(value)));
      await invokeExport("setMouseDelay", args, options);
    });

  program
    .command("keyTap")
    .description("Call @tego/botjs keyTap")
    .argument("<key>", "key")
    .argument("<modifier>", "modifier")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (key, modifier, options) => {
      const args = [key, modifier].map((value) => parseArg(String(value)));
      await invokeExport("keyTap", args, options);
    });

  program
    .command("keyToggle")
    .description("Call @tego/botjs keyToggle")
    .argument("<key>", "key")
    .argument("<down>", "down")
    .argument("<modifier>", "modifier")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (key, down, modifier, options) => {
      const args = [key, down, modifier].map((value) =>
        parseArg(String(value)),
      );
      await invokeExport("keyToggle", args, options);
    });

  program
    .command("typeString")
    .description("Call @tego/botjs typeString")
    .argument("<text>", "text")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (text, options) => {
      const args = [text].map((value) => parseArg(String(value)));
      await invokeExport("typeString", args, options);
    });

  program
    .command("typeStringDelayed")
    .description("Call @tego/botjs typeStringDelayed")
    .argument("<text>", "text")
    .argument("<cpm>", "cpm")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (text, cpm, options) => {
      const args = [text, cpm].map((value) => parseArg(String(value)));
      await invokeExport("typeStringDelayed", args, options);
    });

  program
    .command("unicodeTap")
    .description("Call @tego/botjs unicodeTap")
    .argument("<codePoint>", "codePoint")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (codePoint, options) => {
      const args = [codePoint].map((value) => parseArg(String(value)));
      await invokeExport("unicodeTap", args, options);
    });

  program
    .command("setKeyboardDelay")
    .description("Call @tego/botjs setKeyboardDelay")
    .argument("<ms>", "ms")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (ms, options) => {
      const args = [ms].map((value) => parseArg(String(value)));
      await invokeExport("setKeyboardDelay", args, options);
    });

  program
    .command("bitmapColorAt")
    .description("Call @tego/botjs bitmapColorAt")
    .argument("<bitmap>", "bitmap")
    .argument("<x>", "x")
    .argument("<y>", "y")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (bitmap, x, y, options) => {
      const args = [bitmap, x, y].map((value) => parseArg(String(value)));
      await invokeExport("bitmapColorAt", args, options);
    });

  program
    .command("captureScreen")
    .description("Call @tego/botjs captureScreen")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (options) => {
      const args: unknown[] = [];
      await invokeExport("captureScreen", args, options);
    });

  program
    .command("captureScreenRegion")
    .description("Call @tego/botjs captureScreenRegion")
    .argument("<x>", "x")
    .argument("<y>", "y")
    .argument("<width>", "width")
    .argument("<height>", "height")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (x, y, width, height, options) => {
      const args = [x, y, width, height].map((value) =>
        parseArg(String(value)),
      );
      await invokeExport("captureScreenRegion", args, options);
    });

  program
    .command("getPixelColorHex")
    .description("Call @tego/botjs getPixelColorHex")
    .argument("<x>", "x")
    .argument("<y>", "y")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (x, y, options) => {
      const args = [x, y].map((value) => parseArg(String(value)));
      await invokeExport("getPixelColorHex", args, options);
    });

  program
    .command("getScreen")
    .description("Call @tego/botjs getScreen")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (options) => {
      const args: unknown[] = [];
      await invokeExport("getScreen", args, options);
    });

  program
    .command("getScreenSize")
    .description("Call @tego/botjs getScreenSize")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (options) => {
      const args: unknown[] = [];
      await invokeExport("getScreenSize", args, options);
    });

  program
    .command("updateScreenMetrics")
    .description("Call @tego/botjs updateScreenMetrics")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (options) => {
      const args: unknown[] = [];
      await invokeExport("updateScreenMetrics", args, options);
    });

  program
    .command("getClipboard")
    .description("Call @tego/botjs getClipboard")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (options) => {
      const args: unknown[] = [];
      await invokeExport("getClipboard", args, options);
    });

  program
    .command("setClipboard")
    .description("Call @tego/botjs setClipboard")
    .argument("<text>", "text")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (text, options) => {
      const args = [text].map((value) => parseArg(String(value)));
      await invokeExport("setClipboard", args, options);
    });

  program
    .command("clearClipboard")
    .description("Call @tego/botjs clearClipboard")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (options) => {
      const args: unknown[] = [];
      await invokeExport("clearClipboard", args, options);
    });

  program
    .command("getClipboardImage")
    .description("Call @tego/botjs getClipboardImage")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (options) => {
      const args: unknown[] = [];
      await invokeExport("getClipboardImage", args, options);
    });

  program
    .command("setClipboardImage")
    .description("Call @tego/botjs setClipboardImage")
    .argument("<imageBuffer>", "imageBuffer")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (imageBuffer, options) => {
      const args = [imageBuffer].map((value) => parseArg(String(value)));
      await invokeExport("setClipboardImage", args, options);
    });

  program
    .command("getActiveWindow")
    .description("Call @tego/botjs getActiveWindow")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (options) => {
      const args: unknown[] = [];
      await invokeExport("getActiveWindow", args, options);
    });

  program
    .command("getAllWindows")
    .description("Call @tego/botjs getAllWindows")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (options) => {
      const args: unknown[] = [];
      await invokeExport("getAllWindows", args, options);
    });

  program
    .command("findWindowsByTitle")
    .description("Call @tego/botjs findWindowsByTitle")
    .argument("<title>", "title")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (title, options) => {
      const args = [title].map((value) => parseArg(String(value)));
      await invokeExport("findWindowsByTitle", args, options);
    });

  program
    .command("findWindowsByProcess")
    .description("Call @tego/botjs findWindowsByProcess")
    .argument("<processName>", "processName")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (processName, options) => {
      const args = [processName].map((value) => parseArg(String(value)));
      await invokeExport("findWindowsByProcess", args, options);
    });

  program
    .command("doubleClick")
    .description("Call @tego/botjs doubleClick")
    .argument("<x>", "x")
    .argument("<y>", "y")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (x, y, options) => {
      const args = [x, y].map((value) => parseArg(String(value)));
      await invokeExport("doubleClick", args, options);
    });

  program
    .command("rightClick")
    .description("Call @tego/botjs rightClick")
    .argument("<x>", "x")
    .argument("<y>", "y")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (x, y, options) => {
      const args = [x, y].map((value) => parseArg(String(value)));
      await invokeExport("rightClick", args, options);
    });

  program
    .command("middleClick")
    .description("Call @tego/botjs middleClick")
    .argument("<x>", "x")
    .argument("<y>", "y")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (x, y, options) => {
      const args = [x, y].map((value) => parseArg(String(value)));
      await invokeExport("middleClick", args, options);
    });

  program
    .command("leftClick")
    .description("Call @tego/botjs leftClick")
    .argument("<x>", "x")
    .argument("<y>", "y")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (x, y, options) => {
      const args = [x, y].map((value) => parseArg(String(value)));
      await invokeExport("leftClick", args, options);
    });

  program
    .command("mouseDown")
    .description("Call @tego/botjs mouseDown")
    .argument("<button>", "button")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (button, options) => {
      const args = [button].map((value) => parseArg(String(value)));
      await invokeExport("mouseDown", args, options);
    });

  program
    .command("mouseUp")
    .description("Call @tego/botjs mouseUp")
    .argument("<button>", "button")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (button, options) => {
      const args = [button].map((value) => parseArg(String(value)));
      await invokeExport("mouseUp", args, options);
    });

  program
    .command("captureAndCopy")
    .description("Call @tego/botjs captureAndCopy")
    .argument("<region>", "region")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (region, options) => {
      const args = [region].map((value) => parseArg(String(value)));
      await invokeExport("captureAndCopy", args, options);
    });

  program
    .command("captureAndSave")
    .description("Call @tego/botjs captureAndSave")
    .argument("<path>", "path")
    .argument("<region>", "region")
    .argument("<options>", "options")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (path, region, optionsArg2, options) => {
      const args = [path, region, optionsArg2].map((value) =>
        parseArg(String(value)),
      );
      await invokeExport("captureAndSave", args, options);
    });

  program
    .command("captureRegion")
    .description("Call @tego/botjs captureRegion")
    .argument("<x>", "x")
    .argument("<y>", "y")
    .argument("<width>", "width")
    .argument("<height>", "height")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (x, y, width, height, options) => {
      const args = [x, y, width, height].map((value) =>
        parseArg(String(value)),
      );
      await invokeExport("captureRegion", args, options);
    });

  program
    .command("copyScreenshotToClipboard")
    .description("Call @tego/botjs copyScreenshotToClipboard")
    .argument("<result>", "result")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (result, options) => {
      const args = [result].map((value) => parseArg(String(value)));
      await invokeExport("copyScreenshotToClipboard", args, options);
    });

  program
    .command("getPixelColor")
    .description("Call @tego/botjs getPixelColor")
    .argument("<x>", "x")
    .argument("<y>", "y")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (x, y, options) => {
      const args = [x, y].map((value) => parseArg(String(value)));
      await invokeExport("getPixelColor", args, options);
    });

  program
    .command("quickScreenshot")
    .description("Call @tego/botjs quickScreenshot")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (options) => {
      const args: unknown[] = [];
      await invokeExport("quickScreenshot", args, options);
    });

  program
    .command("quickScreenshotRegion")
    .description("Call @tego/botjs quickScreenshotRegion")
    .argument("<region>", "region")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (region, options) => {
      const args = [region].map((value) => parseArg(String(value)));
      await invokeExport("quickScreenshotRegion", args, options);
    });

  program
    .command("saveScreenshotToFile")
    .description("Call @tego/botjs saveScreenshotToFile")
    .argument("<result>", "result")
    .argument("<filePath>", "filePath")
    .argument("<_options>", "_options")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (result, filePath, _options, options) => {
      const args = [result, filePath, _options].map((value) =>
        parseArg(String(value)),
      );
      await invokeExport("saveScreenshotToFile", args, options);
    });

  program
    .command("startInteractiveCapture")
    .description("Call @tego/botjs startInteractiveCapture")
    .argument("<_options>", "_options")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (_options, options) => {
      const args = [_options].map((value) => parseArg(String(value)));
      await invokeExport("startInteractiveCapture", args, options);
    });

  program
    .command("findAllInRegion")
    .description("Call @tego/botjs findAllInRegion")
    .argument("<template>", "template")
    .argument("<x>", "x")
    .argument("<y>", "y")
    .argument("<width>", "width")
    .argument("<height>", "height")
    .argument("<config>", "config")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (template, x, y, width, height, config, options) => {
      const args = [template, x, y, width, height, config].map((value) =>
        parseArg(String(value)),
      );
      await invokeExport("findAllInRegion", args, options);
    });

  program
    .command("findAllOnScreen")
    .description("Call @tego/botjs findAllOnScreen")
    .argument("<template>", "template")
    .argument("<config>", "config")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (template, config, options) => {
      const args = [template, config].map((value) => parseArg(String(value)));
      await invokeExport("findAllOnScreen", args, options);
    });

  program
    .command("findInRegion")
    .description("Call @tego/botjs findInRegion")
    .argument("<template>", "template")
    .argument("<x>", "x")
    .argument("<y>", "y")
    .argument("<width>", "width")
    .argument("<height>", "height")
    .argument("<config>", "config")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (template, x, y, width, height, config, options) => {
      const args = [template, x, y, width, height, config].map((value) =>
        parseArg(String(value)),
      );
      await invokeExport("findInRegion", args, options);
    });

  program
    .command("findOnScreen")
    .description("Call @tego/botjs findOnScreen")
    .argument("<template>", "template")
    .argument("<config>", "config")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (template, config, options) => {
      const args = [template, config].map((value) => parseArg(String(value)));
      await invokeExport("findOnScreen", args, options);
    });

  program
    .command("getMatchBounds")
    .description("Call @tego/botjs getMatchBounds")
    .argument("<match>", "match")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (match, options) => {
      const args = [match].map((value) => parseArg(String(value)));
      await invokeExport("getMatchBounds", args, options);
    });

  program
    .command("getMatchCenter")
    .description("Call @tego/botjs getMatchCenter")
    .argument("<match>", "match")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (match, options) => {
      const args = [match].map((value) => parseArg(String(value)));
      await invokeExport("getMatchCenter", args, options);
    });

  program
    .command("imageResource")
    .description("Call @tego/botjs imageResource")
    .argument("<path>", "path")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (path, options) => {
      const args = [path].map((value) => parseArg(String(value)));
      await invokeExport("imageResource", args, options);
    });

  program
    .command("imageResourceFromBuffer")
    .description("Call @tego/botjs imageResourceFromBuffer")
    .argument("<buffer>", "buffer")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (buffer, options) => {
      const args = [buffer].map((value) => parseArg(String(value)));
      await invokeExport("imageResourceFromBuffer", args, options);
    });

  program
    .command("imageResourceSync")
    .description("Call @tego/botjs imageResourceSync")
    .argument("<path>", "path")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (path, options) => {
      const args = [path].map((value) => parseArg(String(value)));
      await invokeExport("imageResourceSync", args, options);
    });

  program
    .command("waitFor")
    .description("Call @tego/botjs waitFor")
    .argument("<template>", "template")
    .argument("<timeout>", "timeout")
    .argument("<interval>", "interval")
    .argument("<config>", "config")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (template, timeout, interval, config, options) => {
      const args = [template, timeout, interval, config].map((value) =>
        parseArg(String(value)),
      );
      await invokeExport("waitFor", args, options);
    });

  program
    .command("waitForGone")
    .description("Call @tego/botjs waitForGone")
    .argument("<template>", "template")
    .argument("<timeout>", "timeout")
    .argument("<interval>", "interval")
    .argument("<config>", "config")
    .option("--out <path>", "Write Buffer result to file")
    .option("--json-output", "Emit result as JSON")
    .option("--no-validate", "Disable arg shape validation")
    .option("--dry-run", "Validate args without invoking")
    .action(async (template, timeout, interval, config, options) => {
      const args = [template, timeout, interval, config].map((value) =>
        parseArg(String(value)),
      );
      await invokeExport("waitForGone", args, options);
    });
}
