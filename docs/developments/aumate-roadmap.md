# Aumate Development Roadmap

## Overview

Aumate is a cross-platform desktop automation library with GUI support, built in Rust. It provides input simulation, screen capture, clipboard operations, window management, and a GUI framework for floating windows with effects.

## Architecture

### Core Modules

1. **Input** - Mouse and keyboard control via `enigo` and `rdev`
2. **Screen** - Screen capture via `xcap`
3. **Clipboard** - Text and image clipboard operations via `arboard`
4. **Window** - Window management via `active-win-pos-rs`
5. **GUI** - Floating window system with effects via `winit`, `wgpu`, and `egui`

### GUI Sub-modules

- `window/` - Window controller, floating windows, configuration
- `effect/` - Particle effects system with 18 presets
- `animation/` - Animation system with easing functions
- `content/` - Content rendering (image, text)
- `menu_bar/` - System tray and menu bar support
- `screenshot/` - Screenshot mode with selection and toolbar

## Completed Features

### Phase 1: Core Automation (Completed)
- [x] Mouse control (move, click, drag, scroll, smooth movement)
- [x] Keyboard control (key tap, toggle, type string, unicode)
- [x] Screen capture (full screen, region)
- [x] Pixel color detection
- [x] Clipboard operations (text, image)
- [x] Window management (get active, find by title/process)

### Phase 2: GUI Framework (Completed)
- [x] Floating window system with winit + wgpu + egui
- [x] Window controller with event loop management
- [x] Window configuration (size, position, decorations, transparency)
- [x] Content rendering (image display)
- [x] 18 particle effects presets:
  - Rotating Halo, Pulse Ripple, Flowing Light
  - Stardust Scatter, Electric Spark, Smoke Wisp
  - Aurora Wave, Cosmic Strings, Heartbeat Pulse
  - Laser Beam, Lightning Arc, Matrix Rain
  - Meteor Shower, Orbit Rings, Rain Drop
  - Silk Ribbon, Sonar Pulse, Fire Glow
- [x] Animation system (fade, scale, slide, bounce, rotate, blink)
- [x] Easing functions (linear, ease-in, ease-out, ease-in-out, bounce, elastic)
- [x] Menu bar and system tray support
- [x] Command-based inter-window communication

### Phase 3: Screenshot Integration (Completed)
- [x] Screenshot mode integrated into GUI controller
- [x] Full-screen transparent overlay for selection
- [x] Rectangle selection with mouse drag
- [x] DPI/scale factor handling for coordinates
- [x] Toolbar with action buttons (copy, save, cancel)
- [x] Copy screenshot to clipboard
- [x] Save screenshot to file
- [x] macOS window level configuration (covers menu bar)
- [x] Screenshot mask closes after action completion

### Phase 4: Testing & Quality (Completed)
- [x] Unit tests for all exported functions (41 tests)
- [x] Integration tests with real system interaction (46 tests)
- [x] All 36 API functions tested:
  - Mouse: moveMouse, moveMouseSmooth, mouseClick, mouseToggle, dragMouse, scrollMouse, getMousePos, setMouseDelay
  - Keyboard: keyTap, keyToggle, typeString, typeStringDelayed, unicodeTap, setKeyboardDelay
  - Screen: bitmapColorAt, captureScreen, captureScreenRegion, getPixelColor, getScreen, getScreenSize, updateScreenMetrics
  - Clipboard: getClipboard, setClipboard, clearClipboard, getClipboardImage, setClipboardImage
  - Window: getActiveWindow, getAllWindows, findWindowsByTitle, findWindowsByProcess
  - Helpers: doubleClick, rightClick, middleClick, leftClick, mouseDown, mouseUp

## Planned Features

### Phase 5: Screenshot Enhancements
- [ ] Window snap detection (auto-snap to window edges)
- [ ] Selection resize handles (drag corners/edges)
- [ ] Selection change callback
- [ ] Annotation tools (brush, arrow, rectangle, circle, text)
- [ ] Blur/mosaic tool
- [ ] Annotation undo/redo
- [ ] Multi-format save (PNG, JPG, WebP with quality options)

### Phase 6: Long Screenshot
- [ ] Vertical scrolling capture
- [ ] Horizontal scrolling capture
- [ ] Image stitching algorithm
- [ ] Smart deduplication (remove overlap)
- [ ] Progress callback

### Phase 7: Clipboard Manager
- [ ] Clipboard history monitoring
- [ ] History storage (SQLite)
- [ ] Search and filter history
- [ ] Tags and categories
- [ ] Sensitive data detection
- [ ] Export/import history

### Phase 8: Advanced GUI Components
- [ ] List component (virtual scrolling)
- [ ] Button component
- [ ] Input component
- [ ] Select/dropdown component
- [ ] Dialog component
- [ ] Menu component
- [ ] Layout components (horizontal, vertical, grid)

### Phase 9: Performance & Polish
- [ ] GPU-accelerated effects optimization
- [ ] Memory usage optimization for large screenshots
- [ ] Cross-platform testing (Windows, Linux)
- [ ] Documentation and examples
- [ ] Theme system for UI components

## Technical Stack

### Dependencies
- **Window Management**: winit 0.30
- **GPU Rendering**: wgpu 27.0
- **UI Framework**: egui 0.33, egui-wgpu 0.33, egui-winit 0.33
- **Screen Capture**: xcap 0.7
- **Input Simulation**: enigo 0.6, rdev 0.5
- **Clipboard**: arboard 3.6
- **Image Processing**: image 0.25
- **Math**: glam (vectors, matrices)
- **Noise**: noise (procedural effects)
- **System Tray**: tray-icon, muda

### Build Requirements
- Rust 2024 Edition (1.85+)
- macOS: Screen recording permission required
- Linux: libxcb, libxrandr, libdbus
- Windows: Windows 10+

## Notes

- All GUI operations run on the main thread via winit event loop
- Screenshot mode uses fullscreen transparent overlay
- Coordinate systems handle logical (winit/egui) and physical (xcap) pixels
- Effects use particle systems with configurable parameters
- Window commands are sent via channels for thread-safe communication
