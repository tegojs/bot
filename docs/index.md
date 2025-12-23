---
layout: home

hero:
  name: "Tego Bot"
  text: "Desktop Automation for Node.js"
  tagline: "High-performance automation library powered by Rust"
  actions:
    - theme: brand
      text: Get Started
      link: /guide/getting-started
    - theme: alt
      text: API Reference
      link: /api/
    - theme: alt
      text: View on GitHub
      link: https://github.com/tegojs/bot

features:
  - icon: 🚀
    title: Extreme Performance
    details: Rust core optimized for maximum speed & efficiency
  - icon: 🔒
    title: Memory Safe
    details: Rust's type system guarantees memory safety
  - icon: 🎯
    title: API Compatible
    details: Similar API design to robotjs for easy migration
  - icon: 🌍
    title: Cross-Platform
    details: Supports Windows, macOS, and Linux
  - icon: 📦
    title: Zero Dependencies
    details: No additional Node.js dependencies required
  - icon: 🧪
    title: Well Tested
    details: Comprehensive test coverage
---

# Tego Bot

**High-performance desktop automation library for Node.js, powered by Rust**

---

## ✨ Features

- **🚀 Extreme performance** – Rust core optimized for maximum speed & efficiency
- **🔒 Memory safe** – Rust's type system guarantees memory safety
- **🎯 API compatible** – Similar API design to robotjs for easy migration
- **🌍 Cross-platform** – Supports Windows, macOS, and Linux
- **📦 Zero dependencies** – No additional Node.js dependencies required
- **🧪 Well tested** – Comprehensive test coverage

---

## 🚀 Quick Start

### Installation

```bash
pnpm add @tego/botjs

# Or: npm/yarn/bun add @tego/botjs
```

### Basic Example

```typescript
import { moveMouse, mouseClick, keyTap, typeString, captureScreen } from '@tego/botjs';

// Move mouse and click
moveMouse(100, 200);
mouseClick('left');

// Type text
typeString('Hello from Tego Bot!');
keyTap('enter');

// Capture screen
const screenshot = await captureScreen();
// screenshot.image contains PNG buffer
```

---

## 📚 Documentation

- **[API Documentation](/api/)** - Complete API reference with examples
- **[Development Notes](/developments/)** - Research and development documentation

---

## 🎯 Capabilities

### Mouse Control
- Instant and smooth cursor movement
- Click operations (left, right, middle, double-click)
- Drag and drop
- Scroll wheel control
- Get cursor position

### Keyboard Input
- Key tapping with modifiers (Ctrl, Shift, Alt, Command)
- Text typing with customizable speed
- Unicode character input
- Key press/release control

### Screen Capture
- Full screen screenshots
- Region capture
- Pixel color detection
- PNG output format

### Clipboard Operations
- Get/set text content
- Get/set images (PNG format)
- Clear clipboard

### Window Management
- Get active window information
- Find windows by title
- Find windows by process name
- Window position and size information

---

## 🤖 AI-Powered Script Generator

The `@tego/bot-agent` package provides an AI-powered CLI tool for generating automation scripts:

```bash
npx bot-agent generate
```

Features:
- Generate scripts from natural language descriptions
- Edit existing scripts through conversational AI
- Execute and manage saved scripts
- Store scripts with conversation history

---

## 🏗️ Architecture

The project uses a **monorepo workspace** structure:

### Packages

1. **`Tego Bot`** - Rust core with N-API bindings
   - Cross-platform input simulation
   - Screen capture
   - High-performance native operations

2. **`@tego/botjs`** - TypeScript wrapper
   - Type-safe API
   - Additional helper functions
   - Comprehensive documentation

3. **`@tego/bot-agent`** - AI-powered CLI
   - Script generation
   - Interactive editing
   - Script management

---

## 📦 Installation Requirements

### macOS
- macOS 10.13+
- Screen recording permission required (System Preferences > Security & Privacy > Screen Recording)

### Linux
May require system dependencies:
```bash
# Ubuntu/Debian
sudo apt-get install libxcb1-dev libxrandr-dev libdbus-1-dev

# Fedora
sudo dnf install libxcb-devel libXrandr-devel dbus-devel
```

### Windows
- Windows 10+
- No additional configuration needed

---

## 🔗 Links

- [GitHub Repository](https://github.com/tegojs/bot)
- [npm Package (@tego/botjs)](https://www.npmjs.com/package/@tego/botjs)
- [Issues & Bug Reports](https://github.com/tegojs/bot/issues)

---

## 📄 License

MIT License - see [LICENSE](https://github.com/tegojs/bot/blob/main/LICENSE) for details
