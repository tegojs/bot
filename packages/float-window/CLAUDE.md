# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

float-window is a Rust library for creating frameless, draggable floating windows with support for:
- Window shapes (rectangle, circle, custom mask)
- Window icons (emoji, preset, custom image)
- 6 particle effects (rotating halo, pulse ripple, flowing light, stardust scatter, electric spark, smoke wisp)
- Animations (fade, scale, slide, bounce, rotate, blink)
- Content rendering (image, text, custom)
- Event handling (click, drag, resize, etc.)

## Build Commands

```bash
cargo build              # Build the project
cargo build --features gpu  # Build with GPU acceleration
cargo run                # Run the demo application
cargo test               # Run tests
cargo clippy             # Run linter
cargo fmt                # Format code
```

## Architecture

```
src/
├── lib.rs           # Public API exports and prelude
├── main.rs          # Demo application
├── window/          # Window system (FloatingWindow, builder, manager)
├── shape/           # Shape handling (rectangle, circle, custom mask)
├── icon/            # Icon system (emoji, preset icons, custom images)
├── effect/          # Particle effects system
│   └── presets/     # 6 preset effects
├── animation/       # Animation system with easing functions
├── content/         # Content rendering (image, text, custom)
├── render/          # Rendering backend (egui painter, optional wgpu)
└── event/           # Event handling
```

## Usage Example

```rust
use float_window::prelude::*;

fn main() {
    FloatingWindow::builder()
        .size(200, 200)
        .position(100.0, 100.0)
        .shape(WindowShape::Circle)
        .draggable(true)
        .always_on_top(true)
        .effect(PresetEffect::RotatingHalo, PresetEffectOptions::default())
        .build()
        .unwrap()
        .run()
        .unwrap();
}
```

## Rust Edition

This project uses Rust edition 2024, which is valid with Rust 1.85+.

## Notes

- Use `cargo add` to add dependencies (without version specifier for latest)
- Use context7 MCP tool to find correct API documentation
- GPU acceleration is optional via the `gpu` feature flag
