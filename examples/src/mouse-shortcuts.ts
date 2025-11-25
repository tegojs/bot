/**
 * Mouse shortcuts example
 *
 * This example demonstrates the convenient mouse shortcut methods
 */

import {
  doubleClick,
  getMousePos,
  leftClick,
  middleClick,
  mouseDown,
  mouseUp,
  moveMouse,
  rightClick,
} from "@tego/botjs";

async function mouseShortcutsExample() {
  console.log("=== Mouse Shortcuts Example ===\n");

  // Get initial mouse position
  const initialPos = getMousePos();
  console.log(`Initial mouse position: (${initialPos.x}, ${initialPos.y})`);

  // Move to a safe area
  console.log("\n1. Moving mouse to (500, 500)");
  moveMouse(500, 500);
  await delay(500);

  // Left click at current position
  console.log("\n2. Left click at current position");
  leftClick();
  await delay(500);

  // Move and double click
  console.log("\n3. Double click at (600, 600)");
  doubleClick(600, 600);
  await delay(500);

  // Right click
  console.log("\n4. Right click at (700, 700)");
  rightClick(700, 700);
  await delay(500);

  // Middle click (if you have a middle button)
  console.log("\n5. Middle click at (800, 800)");
  middleClick(800, 800);
  await delay(500);

  // Mouse down/up for custom interactions
  console.log("\n6. Press and hold left button");
  mouseDown("left");
  await delay(1000);
  console.log("   Release left button");
  mouseUp("left");

  // Restore original position
  console.log(
    `\n7. Restoring mouse to original position (${initialPos.x}, ${initialPos.y})`,
  );
  moveMouse(initialPos.x, initialPos.y);

  console.log("\n✅ Mouse shortcuts example completed!");
  console.log(
    "\nNote: These are convenience methods that combine movement and clicking.",
  );
  console.log("Use them for cleaner, more readable automation code.");
}

function delay(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

// Run the example
mouseShortcutsExample().catch(console.error);
