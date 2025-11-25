/**
 * Window management example
 *
 * This example demonstrates how to use window management in @tego/bot
 */

import {
  findWindowsByTitle,
  getActiveWindow,
  getAllWindows,
} from "@tego/botjs";

async function windowExample() {
  console.log("=== Window Management Example ===\n");

  try {
    // Get active window
    console.log("1. Getting active window:");
    const activeWindow = getActiveWindow();
    console.log(`   Title: ${activeWindow.title}`);
    console.log(`   Process: ${activeWindow.processPath}`);
    console.log(`   Process ID: ${activeWindow.processId}`);
    console.log(`   Position: (${activeWindow.x}, ${activeWindow.y})`);
    console.log(`   Size: ${activeWindow.width}x${activeWindow.height}`);
    console.log(`   Window ID: ${activeWindow.windowId}`);

    // Get all windows
    console.log("\n2. Getting all windows:");
    const windows = getAllWindows();
    console.log(`   Found ${windows.length} window(s)`);
    windows.forEach((win, index) => {
      console.log(`   [${index}] ${win.title} (${win.processPath})`);
    });

    // Find windows by title
    if (activeWindow.title) {
      const searchTerm = activeWindow.title.split(" ")[0]; // Get first word
      console.log(
        `\n3. Finding windows with title containing "${searchTerm}":`,
      );
      const matchingWindows = findWindowsByTitle(searchTerm);
      console.log(`   Found ${matchingWindows.length} matching window(s)`);
      matchingWindows.forEach((win, index) => {
        console.log(`   [${index}] ${win.title}`);
      });
    }

    console.log("\n✅ Window example completed!");
  } catch (error) {
    console.error("Error:", error);
    console.log(
      "\nNote: Window management requires an active GUI environment.",
    );
    console.log("Make sure you have windows open and try again.");
  }
}

// Run the example
windowExample().catch(console.error);
