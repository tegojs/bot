# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
# Build the project
cargo build

# Build release version
cargo build --release

# Run the application
cargo run

# Check for compilation errors without building
cargo check
```

## Architecture Overview

This is a macOS screenshot tool written in Rust that uses:
- **winit** for window management and event handling
- **wgpu** as the graphics backend
- **egui** for UI rendering (toolbar, text input)
- **xcap** for screen capture
- **arboard** for clipboard operations

### Core Components

**Main Application (`src/main.rs`)**
- `App` struct implements `winit::ApplicationHandler` for the event loop
- Manages application state: selection, renderer, plugins, drawing/text modes
- Handles mouse/keyboard events and delegates rendering

**Window (`src/window.rs`)**
- Creates a borderless, transparent fullscreen window
- macOS-specific configuration via objc to hide from dock and overlay menu bar

**Renderer (`src/renderer/`)**
- `EguiRenderer` - Main renderer combining wgpu and egui
- `wgpu_init.rs` - WGPU surface/device initialization
- `ui.rs` - Egui UI rendering (screenshot, selection mask, toolbar, text input)
- `input.rs` - Converts winit events to egui input
- `texture.rs` - Screenshot to egui texture conversion
- `render.rs` - WGPU render pass execution

**Selection (`src/selection.rs`)**
- Tracks rectangular screen selection with start/end points
- Handles DPI scaling between logical points and physical pixels

**Plugin System (`src/plugins/`)**
- `Plugin` trait defines interface: `id()`, `name()`, `icon()`, `on_click()`
- `PluginRegistry` manages plugin registration, enabling, and execution
- `PluginContext` provides selection coords, screenshot, and monitor to plugins
- Built-in plugins: save, copy, cancel, annotate, text

**UI (`src/ui/toolbar.rs`)**
- Toolbar positioned below selection area with icon buttons for each enabled plugin

### Data Flow

1. On launch: capture screenshot via xcap → create fullscreen window → initialize wgpu/egui renderer
2. User drags to create selection → `Selection` tracks coordinates
3. On selection complete → show `Toolbar` with plugin buttons
4. Button click → `PluginRegistry.execute_plugin()` → plugin `on_click()` → `PluginResult` (Exit/Continue/Success/Failure)
5. Drawing mode: mouse drag in selection area records points rendered as strokes
6. Text mode: click in selection area opens text input

### Coordinate Systems

The app handles two coordinate systems:
- **Logical points**: Used by winit/egui for UI positioning (DPI-independent)
- **Physical pixels**: Used by xcap for actual screen capture

`Selection.coords_with_scale()` converts between them using the window's scale factor.

## Key Dependencies

- `winit 0.30` - Window creation and event handling
- `wgpu 0.19` - GPU rendering backend
- `egui 0.27` / `egui-wgpu 0.27` - Immediate mode UI
- `xcap 0.7` - Cross-platform screen capture
- `arboard 3.6` - Clipboard access
- `glam 0.30` - Math types (Vec2)
- `image 0.25` - Image processing
