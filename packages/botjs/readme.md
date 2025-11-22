<div align="center">
  <h1>@tego/botjs</h1>
  
  <br />
  <br />
  
  <p>
    <strong>Type-safe</strong> TypeScript wrapper for <strong>@tego/bot</strong>.  
    High-performance desktop automation library powered by <strong>Rust</strong>.
  </p>
</div>

---

- **🎯 Type-safe** – Full TypeScript support with complete type definitions
- **🚀 High performance** – Powered by Rust core for maximum speed
- **🔒 Memory safe** – Rust's type system guarantees memory safety
- **📦 Zero dependencies** – No additional Node.js dependencies
- **🌍 Cross-platform** – Supports Windows, macOS, and Linux

---

## 🚀 Quick Start

You can add **@tego/botjs** to your project:

```bash
pnpm add @tego/botjs

# Or: npm/yarn/bun add @tego/botjs
```

### Minimal Example

```ts
import { moveMouse, mouseClick, keyTap, typeString, captureScreen } from '@tego/botjs';

// Move mouse and click
moveMouse(100, 200);
mouseClick('left');

// Type text
typeString('Hello from @tego/botjs!');
keyTap('enter');

// Capture screen
const screenshot = await captureScreen();
// screenshot.image contains PNG buffer
```

---

## 📖 API Documentation

### Mouse Operations

```ts
import { 
  moveMouse, 
  moveMouseSmooth, 
  mouseClick, 
  getMousePos, 
  dragMouse, 
  scrollMouse,
  setMouseDelay 
} from '@tego/botjs';

// Move mouse
moveMouse(100, 200);

// Smooth movement
moveMouseSmooth(300, 400);
moveMouseSmooth(500, 600, 5.0); // with custom speed

// Click
mouseClick('left');
mouseClick('right', true); // double click

// Get position
const pos = getMousePos();
console.log(`Mouse at: ${pos.x}, ${pos.y}`);

// Drag and scroll
dragMouse(500, 600);
scrollMouse(0, 3);

// Set delay
setMouseDelay(50);
```

### Keyboard Operations

```ts
import { 
  keyTap, 
  keyToggle, 
  typeString, 
  typeStringDelayed, 
  unicodeTap,
  setKeyboardDelay 
} from '@tego/botjs';

// Tap keys
keyTap('a');
keyTap('c', ['control']); // Ctrl+C

// Toggle keys
keyToggle('a', 'down');
keyToggle('a', 'up');

// Type text
typeString('Hello, World!');
typeStringDelayed('Hello', 300); // 300 CPM

// Unicode
unicodeTap(0x1F600); // 😀

// Set delay
setKeyboardDelay(10);
```

### Screen Operations

```ts
import { 
  captureScreen, 
  captureScreenRegion, 
  getScreenSize, 
  getPixelColor,
  screen,
  updateScreenMetrics 
} from '@tego/botjs';
import fs from 'fs';

// Capture screen
const screenshot = await captureScreen();
fs.writeFileSync('screenshot.png', screenshot.image);

// Capture region
const region = await captureScreenRegion(100, 100, 800, 600);

// Get screen size
const size = getScreenSize();
console.log(`Screen: ${size.width}x${size.height}`);

// Get pixel color
const color = await getPixelColor(100, 200);
console.log(`Color: ${color}`); // "#FF0000"

// Using screen object
const bitmap = await screen.capture(0, 0, 800, 600);
const pixelColor = bitmap.colorAt(100, 200);

// Update metrics
updateScreenMetrics();
```

---

## 🛠️ Building

```bash
# Build TypeScript code
pnpm build

# Or from root
pnpm ts:build
```

---

## 🧪 Testing

```bash
# Run tests
pnpm test

# Watch mode
pnpm test:watch

# Coverage
pnpm test:coverage
```

---

## 📚 Documentation

```bash
# Generate docs
pnpm docs

# Watch mode
pnpm docs:watch

# Serve docs
pnpm docs:serve
```

---

## 📄 License

Licensed under the [MIT License](LICENSE).

---

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

---
