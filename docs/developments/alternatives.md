# Desktop Automation Libraries Comparison

This document compares popular desktop automation libraries across different programming languages and platforms.

---

## Overview

| Library | Language | Platform Support | Last Active | Key Features |
|---------|----------|------------------|-------------|--------------|
| **PyAutoGUI** | Python | Windows, macOS, Linux | Active | Mouse, keyboard, screenshots, image recognition |
| **RobotJS** | Node.js | Windows, macOS, Linux | Slowed | Mouse, keyboard, screen capture |
| **nut.js** | Node.js | Windows, macOS, Linux | Active | Mouse, keyboard, OCR, image recognition, window management |
| **rustautogui** | Rust | Windows, macOS, Linux | Active | Mouse, keyboard, custom template matching (5x faster) |
| **autopilot-rs** | Rust | Windows, macOS, Linux (X11) | Moderate | Mouse, keyboard, screen size, alerts |

---

## Python Libraries

### PyAutoGUI

The most popular Python desktop automation library with comprehensive features.

**Repository**: [github.com/asweigart/pyautogui](https://github.com/asweigart/pyautogui)
**Package**: [pypi.org/project/PyAutoGUI](https://pypi.org/project/PyAutoGUI/)

#### Features

| Category | Capabilities |
|----------|-------------|
| **Mouse** | Move, click (left/right/middle), drag, scroll, position detection |
| **Keyboard** | Type strings, press keys, hotkeys, key up/down |
| **Screen** | Full/region screenshot, pixel color detection |
| **Image** | Locate image on screen, confidence matching, find all matches |

#### Platform Implementation

- **Windows**: Uses Windows API via `ctypes`
- **macOS**: Uses Cocoa API via `rubicon-objc`
- **Linux**: Uses X11/Xlib

#### Pros

- Simple, intuitive API
- Comprehensive functionality
- Excellent documentation
- Active community
- Cross-platform support

#### Cons

- Image recognition can be slow
- No native Unicode input (requires workarounds)
- Some platforms need additional dependencies
- No UI element selector support (coordinate-based only)

#### Example

```python
import pyautogui

# Mouse operations
pyautogui.moveTo(100, 200, duration=0.5)
pyautogui.click()
pyautogui.scroll(3)

# Keyboard operations
pyautogui.write('Hello, World!')
pyautogui.hotkey('ctrl', 'c')

# Screen operations
screenshot = pyautogui.screenshot()
location = pyautogui.locateOnScreen('button.png', confidence=0.8)
```

---

## Node.js Libraries

### RobotJS

Lightweight Node.js desktop automation library with native bindings.

**Repository**: [github.com/octalmage/robotjs](https://github.com/octalmage/robotjs)
**Package**: [npmjs.com/package/robotjs](https://www.npmjs.com/package/robotjs)

#### Features

| Category | Capabilities |
|----------|-------------|
| **Mouse** | Move, click, drag, scroll, position detection |
| **Keyboard** | Type strings, key tap, key toggle, modifiers |
| **Screen** | Screen capture, pixel color, screen size |

#### Pros

- Lightweight with no dependencies
- Fast native performance
- Simple API
- Cross-platform support

#### Cons

- **Development has slowed significantly**
- No image recognition
- No window management
- No explicit multi-monitor support
- Requires native compilation

#### Build Requirements

- Windows: windows-build-tools
- macOS: Xcode Command Line Tools
- Linux: libxtst-dev, libpng++-dev

#### Example

```javascript
const robot = require('robotjs');

// Mouse operations
robot.moveMouse(100, 200);
robot.mouseClick();
robot.scrollMouse(0, 3);

// Keyboard operations
robot.typeString('Hello, World!');
robot.keyTap('enter');

// Screen operations
const screenSize = robot.getScreenSize();
const color = robot.getPixelColor(100, 200);
```

---

### nut.js

Modern, well-maintained Node.js automation library with advanced features.

**Website**: [nutjs.dev](https://nutjs.dev/)
**Repository**: [github.com/nut-tree/nut.js](https://github.com/nut-tree/nut.js)
**Package**: [@nut-tree/nut-js](https://www.npmjs.com/package/@nut-tree/nut-js)

#### Features

| Category | Capabilities |
|----------|-------------|
| **Mouse** | Move (configurable speed), click, drag, scroll |
| **Keyboard** | Type, key press/release, multimedia keys |
| **Screen** | Pixel color, image detection, screen highlighting |
| **OCR** | Text recognition (via plugin) |
| **Window** | List, focus, resize, minimize/restore |
| **Clipboard** | Text copy/paste |

#### Platform Requirements

- **Windows**: Standard installation; Media Feature Pack required for Windows 10 N
- **macOS**: Xcode CLI tools; Accessibility + Screen Recording permissions
- **Linux**: libXtst; X11 only (Wayland not supported)

#### Pros

- Active development and maintenance
- Image recognition and OCR support
- Window management capabilities
- Jest testing integration
- Plugin architecture for extensibility

#### Cons

- Requires native compilation or paid pre-built packages
- More complex setup than RobotJS
- OCR requires additional plugin packages
- Larger bundle size

#### Example

```typescript
import { mouse, keyboard, screen, imageResource } from '@nut-tree/nut-js';

// Mouse operations
await mouse.move([{ x: 100, y: 200 }]);
await mouse.click(Button.LEFT);
await mouse.scrollDown(3);

// Keyboard operations
await keyboard.type('Hello, World!');
await keyboard.pressKey(Key.Enter);

// Screen operations
const img = await imageResource('button.png');
const region = await screen.find(img);
await mouse.move(straightTo(centerOf(region)));
```

---

## Rust Libraries

### rustautogui

High-performance Rust automation library inspired by PyAutoGUI with custom template matching.

**Repository**: [github.com/DavorMar/rustautogui](https://github.com/DavorMar/rustautogui)
**Crate**: [crates.io/crates/rustautogui](https://crates.io/crates/rustautogui)

#### Features

| Category | Capabilities |
|----------|-------------|
| **Mouse** | Move, click (left/right/middle), drag, scroll |
| **Keyboard** | Type strings, key press, multi-key combinations |
| **Screen** | Screen capture, template matching |
| **Image** | Custom template matching (Segmented + FFT algorithms) |

#### Template Matching Algorithms

1. **Segmented (V1)**: Custom multithreaded algorithm using Segmented Normalized Cross-Correlation
   - Best for general use with automatic threshold
   - Consistent performance

2. **FFT (V2)**: Fast Fourier Transform based
   - Can be faster when threshold is well-tuned
   - Best with manual threshold control

3. **OpenCL variants**: GPU-accelerated options (SegmentedOcl, SegmentedOclV2)

#### Performance

- **~5x faster than PyAutoGUI** on Windows
- No OpenCV dependency (unlike Python alternatives)
- Custom multithreaded implementation

#### Platform Notes

- **macOS Retina**: Templates searched at both original and half resolution
- **Multi-Monitor**: Windows/macOS limited to primary; Linux supports all monitors

#### Pros

- Extremely fast template matching
- No OpenCV dependency
- GPU acceleration available
- Memory-efficient image loading
- Lite version available (keyboard/mouse only)

#### Cons

- Smaller community than Python alternatives
- Less documentation
- Multi-monitor support varies by platform

#### Example

```rust
use rustautogui::RustAutoGui;

let mut gui = RustAutoGui::new(MatchMode::Segmented);

// Mouse operations
gui.move_mouse(100, 200);
gui.click(MouseButton::Left);
gui.scroll(3);

// Keyboard operations
gui.type_string("Hello, World!");
gui.key_press(&["ctrl", "c"]);

// Template matching
gui.store_template_from_file("button", "button.png", 0.8)?;
if let Some(locations) = gui.find_image_on_screen_stored("button", 0.9) {
    gui.move_mouse(locations[0].0, locations[0].1);
}
```

---

### autopilot-rs

Simple, cross-platform Rust GUI automation library (port of Python's AutoPy).

**Repository**: [github.com/autopilot-rs/autopilot-rs](https://github.com/autopilot-rs/autopilot-rs)
**Crate**: [crates.io/crates/autopilot](https://crates.io/crates/autopilot)
**Docs**: [docs.rs/autopilot](https://docs.rs/autopilot)

#### Features

| Category | Capabilities |
|----------|-------------|
| **Mouse** | Move, position detection |
| **Keyboard** | Type strings with timing control |
| **Screen** | Screen size detection |
| **Alerts** | Display alert dialogs |

#### Platform Support

- macOS
- Windows
- Linux (X11 with XTest extension)

#### Pros

- Very simple API
- Minimal dependencies
- Dual licensed (Apache-2.0 / MIT)
- Small codebase (~2K SLoC)

#### Cons

- Limited functionality compared to alternatives
- No image recognition
- No window management
- Less active development

#### Example

```rust
use autopilot::mouse;
use autopilot::key;
use autopilot::screen;
use autopilot::alert;

// Mouse operations
mouse::move_to(autopilot::geometry::Point::new(100.0, 200.0))?;

// Keyboard operations
key::type_string("Hello, World!", &[], 50.0, 0.0);

// Screen operations
let screen_size = screen::size();
println!("Screen: {}x{}", screen_size.width, screen_size.height);

// Alerts
alert::alert("Title", "Message", None, None);
```

---

## Feature Comparison Matrix

| Feature | PyAutoGUI | RobotJS | nut.js | rustautogui | autopilot-rs |
|---------|-----------|---------|--------|-------------|--------------|
| **Mouse Control** | Full | Full | Full | Full | Basic |
| **Keyboard Control** | Full | Full | Full | Full | Basic |
| **Screen Capture** | Full | Basic | Full | Full | Size only |
| **Image Recognition** | Built-in | None | Plugin | Built-in (fast) | None |
| **OCR** | Via OpenCV | None | Plugin | None | None |
| **Window Management** | Via pygetwindow | None | Built-in | None | None |
| **Clipboard** | Via pyperclip | None | Built-in | None | None |
| **Alerts/Dialogs** | Via pymsgbox | None | None | None | Built-in |
| **Cross-platform** | Excellent | Good | Good | Good | Good |
| **Performance** | Moderate | Fast | Fast | Very Fast | Fast |
| **Active Development** | Active | Slowed | Active | Active | Moderate |

---

## Use Case Recommendations

### General Desktop Automation
**Recommended**: PyAutoGUI (Python) or nut.js (Node.js)
- Comprehensive features
- Good documentation
- Active communities

### High-Performance Image Matching
**Recommended**: rustautogui (Rust)
- 5x faster than Python alternatives
- Custom optimized algorithms
- GPU acceleration available

### Simple Keyboard/Mouse Automation
**Recommended**: RobotJS (Node.js) or autopilot-rs (Rust)
- Lightweight
- Easy to integrate
- Minimal dependencies

### UI Testing with Image Recognition
**Recommended**: nut.js (Node.js)
- Jest integration
- OCR support
- Window management

### Cross-Platform CLI Tools
**Recommended**: rustautogui or autopilot-rs (Rust)
- Single binary distribution
- No runtime dependencies
- Fast startup

---

## Sources

- [PyAutoGUI - PyPI](https://pypi.org/project/PyAutoGUI/)
- [PyAutoGUI - GitHub](https://github.com/asweigart/pyautogui)
- [RobotJS - npm](https://www.npmjs.com/package/robotjs)
- [RobotJS - GitHub](https://github.com/octalmage/robotjs)
- [nut.js - Official Site](https://nutjs.dev/)
- [nut.js - GitHub](https://github.com/nut-tree/nut.js)
- [rustautogui - crates.io](https://crates.io/crates/rustautogui)
- [rustautogui - GitHub](https://github.com/DavorMar/rustautogui)
- [autopilot-rs - crates.io](https://crates.io/crates/autopilot)
- [autopilot-rs - GitHub](https://github.com/autopilot-rs/autopilot-rs)
