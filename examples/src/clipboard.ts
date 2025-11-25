/**
 * Clipboard operations example
 *
 * This example demonstrates how to use clipboard operations in @tego/bot
 */

import { clearClipboard, getClipboard, setClipboard } from "@tego/botjs";

async function clipboardExample() {
  console.log("=== Clipboard Example ===\n");

  // Save original clipboard content
  let originalContent = "";
  try {
    originalContent = getClipboard();
    console.log("Original clipboard:", originalContent);
  } catch (error) {
    console.log("Original clipboard: (empty or not available)");
  }

  // Set some text to clipboard
  console.log('\n1. Setting clipboard to "Hello from @tego/bot!"');
  setClipboard("Hello from @tego/bot!");

  // Read it back
  const newContent = getClipboard();
  console.log("   Clipboard now contains:", newContent);

  // Wait a bit
  await new Promise((resolve) => setTimeout(resolve, 1000));

  // Clear clipboard
  console.log("\n2. Clearing clipboard");
  clearClipboard();

  // Restore original content
  console.log("\n3. Restoring original clipboard content");
  if (originalContent) {
    setClipboard(originalContent);
  } else {
    console.log("   (No original content to restore)");
  }

  console.log("\n✅ Clipboard example completed!");
}

// Run the example
clipboardExample().catch(console.error);
