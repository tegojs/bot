import {
  moveMouse,
  moveMouseSmooth,
  mouseClick,
  getMousePos,
  dragMouse,
  scrollMouse,
  setMouseDelay,
  keyTap,
  keyToggle,
  typeString,
  typeStringDelayed,
  unicodeTap,
  setKeyboardDelay,
  getScreenSize,
  getPixelColor,
  getScreen,
  bitmapColorAt,
  updateScreenMetrics,
} from "@tego/bot";
import fs from "fs";

async function main() {
  console.log("🚀 @tego/bot Basic Examples\n");

  // ============================================
  // Mouse Operations
  // ============================================
  console.log("📱 Mouse Operations:");

  // Get current mouse position
  const currentPos = getMousePos();
  console.log(`  Current mouse position: (${currentPos.x}, ${currentPos.y})`);

  // Move mouse to a specific position
  console.log("  Moving mouse to (500, 300)...");
  moveMouse(500, 300);

  // Smooth mouse movement
  console.log("  Smooth moving mouse to (800, 400)...");
  moveMouseSmooth(800, 400);

  // Smooth movement with custom speed
  console.log("  Smooth moving with speed 5.0 to (600, 500)...");
  moveMouseSmooth(600, 500, 5.0);

  // Click mouse
  console.log("  Clicking left button...");
  mouseClick("left");

  // Double click
  console.log("  Double clicking right button...");
  mouseClick("right", true);

  // Set mouse delay
  setMouseDelay(50);
  console.log("  Mouse delay set to 50ms");

  // Scroll mouse
  console.log("  Scrolling down...");
  scrollMouse(0, 3);

  // Drag mouse (press, move, release)
  console.log("  Dragging mouse from (600, 500) to (700, 600)...");
  dragMouse(700, 600);

  // ============================================
  // Keyboard Operations
  // ============================================
  console.log("\n⌨️  Keyboard Operations:");

  // Set keyboard delay
  setKeyboardDelay(10);
  console.log("  Keyboard delay set to 10ms");

  // Tap a key
  console.log("  Tapping 'a' key...");
  keyTap("a");

  // // Tap with modifier (Ctrl+C)
  // console.log("  Tapping Ctrl+C...");
  // keyTap("c", ["control"]);

  // // Tap with multiple modifiers (Ctrl+Shift+V)
  // console.log("  Tapping Ctrl+Shift+V...");
  // keyTap("v", ["control", "shift"]);

  // Toggle key (press and hold)
  console.log("  Pressing 'shift' key...");
  keyToggle("shift", "down");
  console.log("  Releasing 'shift' key...");
  keyToggle("shift", "up");

  // Type a string
  console.log("  Typing 'Hello from @tego/bot!'...");
  typeString("Hello from @tego/bot!");

  // Type with delay (300 characters per minute)
  console.log("  Typing 'Hello' with 300 CPM delay...");
  typeStringDelayed("Hello", 300);

  // Tap Unicode character (emoji)
  console.log("  Tapping Unicode character 😀...");
  unicodeTap(0x1f600);

  // ============================================
  // Screen Operations
  // ============================================
  console.log("\n🖥️  Screen Operations:");

  // Update screen metrics
  updateScreenMetrics();
  console.log("  Screen metrics updated");

  // Get screen size
  const screenSize = getScreenSize();
  console.log(`  Screen size: ${screenSize.width}x${screenSize.height}`);

  // Capture entire screen using screen object
  const screen = getScreen();
  console.log("  Capturing entire screen...");
  const fullScreen = await screen.capture();
  fs.writeFileSync("screenshot-full.png", fullScreen.image);
  console.log(`  Saved full screenshot: ${fullScreen.width}x${fullScreen.height}`);

  // Capture screen region
  console.log("  Capturing screen region (100, 100, 800, 600)...");
  const region = await screen.capture(100, 100, 800, 600);
  fs.writeFileSync("screenshot-region.png", region.image);
  console.log(`  Saved region screenshot: ${region.width}x${region.height}`);

  // Get pixel color
  console.log("  Getting pixel color at (100, 200)...");
  const pixelColor = await getPixelColor(100, 200);
  console.log(`  Pixel color: ${pixelColor}`);

  // Using screen object for another capture
  console.log("  Using screen.capture() for another region...");
  const bitmap = await screen.capture(0, 0, 400, 300);
  console.log(`  Captured bitmap: ${bitmap.width}x${bitmap.height}`);
  console.log(`  Bitmap properties: ${bitmap.byteWidth} bytes/row, ${bitmap.bitsPerPixel} bpp`);

  // Get color from bitmap
  const bitmapColor = bitmapColorAt(bitmap, 50, 50);
  console.log(`  Color at (50, 50) in bitmap: ${bitmapColor}`);

  console.log("\n✅ All examples completed!");
  console.log("\n📝 Generated files:");
  console.log("  - screenshot-full.png");
  console.log("  - screenshot-region.png");
}

// Run the examples
main().catch((error) => {
  console.error("❌ Error:", error);
  process.exit(1);
});
