<div align="center">
  <h1>@tego/bot</h1>
  
  <br />
  <br />
  
  <p>
    <strong>High-performance</strong> desktop automation library for <strong>Node.js</strong>.  
    Powered by <strong>Rust</strong> for extreme speed and memory safety.
  </p>
</div>

---

- **🚀 Extreme performance** – Rust core optimized for maximum speed & efficiency
- **🔒 Memory safe** – Rust's type system guarantees memory safety
- **🎯 API compatible** – Similar API design to robotjs for easy migration
- **🌍 Cross-platform** – Supports Windows, macOS, and Linux
- **📦 Zero dependencies** – No additional Node.js dependencies required
- **🧪 Well tested** – Comprehensive test coverage

---

## 🚀 Quick Start

You can add **@tego/bot** to your project:

```bash
pnpm add @tego/bot

# Or: npm/yarn/bun add @tego/bot
```

### Minimal Example

```ts
import { moveMouse, mouseClick, keyTap, typeString, captureScreen } from '@tego/bot';

// Move mouse and click
moveMouse(100, 200);
mouseClick('left');

// Type text
typeString('Hello from @tego/bot!');
keyTap('enter');

// Capture screen
const screenshot = await captureScreen();
// screenshot.image contains PNG buffer
```

---

## 📖 API Documentation

### Mouse Operations

```ts
import { moveMouse, moveMouseSmooth, mouseClick, getMousePos, dragMouse, scrollMouse } from '@tego/bot';

// Move mouse to coordinates
moveMouse(100, 200);

// Smooth movement
moveMouseSmooth(300, 400);
moveMouseSmooth(500, 600, 5.0); // with custom speed

// Click
mouseClick('left');           // Left click
mouseClick('right', true);    // Right double click
mouseClick('middle');         // Middle click

// Get mouse position
const pos = getMousePos();
console.log(`Mouse at: ${pos.x}, ${pos.y}`);

// Drag
dragMouse(500, 600);

// Scroll
scrollMouse(0, 3);  // Scroll down
scrollMouse(2, 0);  // Scroll right
```

### Keyboard Operations

```ts
import { keyTap, keyToggle, typeString, typeStringDelayed, unicodeTap } from '@tego/bot';

// Tap a key
keyTap('a');
keyTap('enter');
keyTap('c', ['control']);        // Ctrl+C
keyTap('v', ['control', 'shift']); // Ctrl+Shift+V

// Toggle key
keyToggle('a', 'down');  // Press 'a'
keyToggle('a', 'up');    // Release 'a'

// Type text
typeString('Hello, World!');

// Type with delay (characters per minute)
typeStringDelayed('Hello', 300); // 300 CPM

// Tap Unicode character
unicodeTap(0x1F600); // 😀
```

### Screen Operations

```ts
import { captureScreen, captureScreenRegion, getScreenSize, getPixelColor, screen } from '@tego/bot';
import fs from 'fs';

// Capture entire screen
const screenshot = await captureScreen();
fs.writeFileSync('screenshot.png', screenshot.image);

// Capture region
const region = await captureScreenRegion(100, 100, 800, 600);
fs.writeFileSync('region.png', region.image);

// Get screen size
const size = getScreenSize();
console.log(`Screen: ${size.width}x${size.height}`);

// Get pixel color (returns hex string)
const color = await getPixelColor(100, 200);
console.log(`Color: ${color}`); // e.g., "#FF0000"

// Using screen object
const bitmap = await screen.capture(0, 0, 800, 600);
const pixelColor = bitmap.colorAt(100, 200);
```

### Configuration

```ts
import { setMouseDelay, setKeyboardDelay, updateScreenMetrics } from '@tego/bot';

// Set delays (in milliseconds)
setMouseDelay(50);
setKeyboardDelay(10);

// Update screen metrics (refresh monitor information)
updateScreenMetrics();
```

---

## 🎯 Supported Keys

### Modifier Keys
- `control` / `ctrl` – Control key
- `shift` – Shift key
- `alt` – Alt key
- `command` / `cmd` / `meta` – Command/Meta key

### Function Keys
- `f1` – `f12` – F1 to F12

### Special Keys
- `enter` / `return` – Enter key
- `escape` / `esc` – ESC key
- `backspace` – Backspace key
- `tab` – Tab key
- `space` – Space key
- `delete` / `del` – Delete key
- `up` / `down` / `left` / `right` – Arrow keys
- `home` / `end` – Home/End keys
- `pageup` / `page_down` – Page Up/Down keys

### Mouse Buttons
- `left` – Left button
- `right` – Right button
- `middle` – Middle button

---

## 🛠️ Building

To build the library from source:

```bash
# Install dependencies
pnpm install

# Build Rust code and generate Node.js bindings
pnpm build

# Or build only Rust code
pnpm rs:build
```

---

## 📋 System Requirements

### macOS
- macOS 10.13+
- Screen recording permission required (System Preferences > Security & Privacy > Screen Recording)

### Windows
- Windows 10+
- No additional configuration needed

### Linux
- X11 or Wayland
- May require system dependencies:
  ```bash
  # Ubuntu/Debian
  sudo apt-get install libxcb1-dev libxrandr-dev libdbus-1-dev
  
  # Fedora
  sudo dnf install libxcb-devel libXrandr-devel dbus-devel
  ```

---

## 🧪 Testing

```bash
# Run Rust tests
pnpm test

# Build and test Node.js bindings
pnpm build
```

---

## 📊 Comparison with robotjs

| Feature | robotjs | @tego/bot |
|---------|---------|-----------|
| Performance | Medium (C++ bindings) | ⚡ Extremely high (Rust native) |
| Maintenance | ❌ No longer maintained | ✅ Actively maintained |
| Memory Safety | ⚠️ C++ | ✅ Rust |
| API Design | ✅ Simple | ✅ Compatible |
| Cross-platform | ✅ | ✅ |
| Type Safety | ⚠️ Runtime checks | ✅ Compile-time guarantees |

---

## 📄 License

Licensed under the [MIT License](LICENSE).

---

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

---
