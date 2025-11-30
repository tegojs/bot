# Aumate

Cross-platform desktop automation library with GUI support, built in Rust.

## Features

- **Input Control** - Mouse and keyboard automation via `enigo` and `rdev`
- **Screen Capture** - Screenshot functionality via `xcap`
- **Clipboard** - Text and image clipboard operations via `arboard`
- **Window Management** - Find and manage windows via `active-win-pos-rs`
- **GUI Framework** - Floating window system with effects via `winit`, `wgpu`, and `egui`

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
aumate = "0.1"
```

### Feature Flags

Enable only what you need:

```toml
[dependencies]
# Core automation only (no GUI)
aumate = { version = "0.1", default-features = false, features = ["input", "screen", "clipboard", "window"] }

# Full GUI support (default)
aumate = "0.1"
```

Available features:
- `input` - Mouse and keyboard control
- `screen` - Screen capture
- `clipboard` - Clipboard operations
- `window` - Window management
- `gui` - Full GUI framework with effects (includes screen and clipboard)

## Quick Start

```rust
use aumate::prelude::*;

fn main() -> Result<(), AumateError> {
    // Mouse control
    let mouse = Mouse::new()?;
    mouse.move_mouse(100, 200)?;
    mouse.mouse_click(Some("left"), Some(false))?;

    // Keyboard input
    let keyboard = Keyboard::new()?;
    keyboard.type_string("Hello, World!")?;

    // Screen capture
    let capture = capture_screen()?;
    println!("Captured {}x{} screenshot", capture.width, capture.height);

    // Clipboard
    set_text("Copied text")?;
    let text = get_text()?;
    println!("Clipboard: {}", text);

    Ok(())
}
```

## GUI Framework

Aumate includes a powerful GUI framework for creating floating windows with visual effects:

- 18 particle effect presets (Aurora Wave, Matrix Rain, Fire Glow, etc.)
- Animation system with easing functions
- Screenshot mode with selection overlay
- Menu bar and system tray support

## Platform Support

| Platform | Status |
|----------|--------|
| macOS    | Full support |
| Windows  | Full support |
| Linux    | Requires X11/Wayland dependencies |

### Linux Dependencies

```bash
# Ubuntu/Debian
sudo apt-get install libxcb1-dev libxrandr-dev libdbus-1-dev

# Fedora
sudo dnf install libxcb-devel libXrandr-devel dbus-devel
```

## License

MIT License - see [LICENSE](LICENSE) for details.
