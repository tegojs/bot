**Tego Bot API Documentation v0.1.3**

***

<div align="center">
  <p>
    <strong>High-performance</strong> desktop automation for <strong>Node.js</strong> and <strong>Rust</strong>.
  </p>

  <p>
    <a href="https://www.npmjs.com/package/@tego/botjs"><img src="https://img.shields.io/npm/v/@tego/botjs?style=flat-square&logo=npm&color=cb3837" alt="npm version" /></a>
    <a href="https://crates.io/crates/aumate"><img src="https://img.shields.io/crates/v/aumate?style=flat-square&logo=rust&color=orange" alt="crates.io version" /></a>
    <a href="https://github.com/tegojs/bot"><img src="https://img.shields.io/badge/rust-25.8k%20lines-orange?style=flat-square&logo=rust" alt="Rust lines" /></a>
    <a href="https://github.com/tegojs/bot"><img src="https://img.shields.io/badge/typescript-4.4k%20lines-blue?style=flat-square&logo=typescript" alt="TypeScript lines" /></a>
    <a href="https://github.com/tegojs/bot/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/tegojs/bot/ci.yml?branch=main&style=flat-square&logo=github&label=CI" alt="CI status" /></a>
    <a href="https://github.com/tegojs/bot/blob/main/LICENSE"><img src="https://img.shields.io/github/license/tegojs/bot?style=flat-square&color=green" alt="License" /></a>
  </p>
</div>

---

## Packages

| Package | Description | Links |
|---------|-------------|-------|
| **aumate** | Cross-platform desktop automation library (Rust) | [crates.io](https://crates.io/crates/aumate) · [docs.rs](https://docs.rs/aumate) |
| **@tego/bot** | N-API bindings for Node.js | [npm](https://www.npmjs.com/package/@tego/bot) |
| **@tego/botjs** | TypeScript wrapper with full type safety | [npm](https://www.npmjs.com/package/@tego/botjs) |
| **@tego/bot-agent** | AI-powered CLI for script generation | [npm](https://www.npmjs.com/package/@tego/bot-agent) |

---

## Features

- **Mouse & Keyboard** – Move, click, drag, scroll, type, hotkeys
- **Screen Capture** – Full screen, region, pixel color
- **Clipboard** – Text and image operations
- **Window Management** – List, find, and query windows
- **GUI** – Floating windows, particle effects, screenshot UI (aumate)
- **Speech-to-Text** – Whisper-based voice input with hotkeys (aumate)
- **Cross-platform** – Windows, macOS, Linux

---

## Quick Start

### Node.js

```bash
pnpm add @tego/botjs
```

```ts
import { moveMouse, mouseClick, typeString, captureScreen } from '@tego/botjs';

moveMouse(100, 200);
mouseClick('left');
typeString('Hello!');

const screenshot = await captureScreen();
```

### Rust

```bash
cargo add aumate
```

```rust
use aumate::prelude::*;

let mouse = Mouse::new()?;
mouse.move_mouse(100, 200)?;

let keyboard = Keyboard::new()?;
keyboard.type_string("Hello!")?;
```

---

## Building

```bash
pnpm install
pnpm build        # Build all packages
pnpm test         # Run tests
```

---

## License

MIT
