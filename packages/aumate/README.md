# Aumate

> **Work in Progress** - This is a prototype package. APIs and features may change significantly between versions.

Cross-platform desktop automation library with GUI support, built in Rust. Originally developed as the Rust core for the [@tego/bot](https://github.com/tegojs/bot) Node.js automation library via napi-rs bindings.

## Quick Start - GUI Application

Install and run the GUI controller:

```bash
cargo install aumate
aumate
```

This launches the Aumate Controller with:
- Floating window management with 18+ particle effects
- Region capture (screenshot) with annotation tools
- Menu bar item creation
- Clipboard manager with history tracking
- Settings panel

## Features

### Core Automation
- **Input Control** - Mouse and keyboard automation via `enigo` and `rdev`
- **Screen Capture** - Screenshot functionality via `xcap`
- **Clipboard** - Text and image clipboard operations via `arboard`
- **Window Management** - Find and manage windows via `active-win-pos-rs`

### GUI Framework (v0.2.0+)
- **Floating Windows** - Draggable, always-on-top windows with custom shapes
- **18+ Particle Effects** - Aurora Wave, Matrix Rain, Silk Ribbon, Fire Glow, etc.
- **Animation System** - Smooth transitions with easing functions
- **Screenshot Mode** - Region selection with annotation tools (rectangle, ellipse, arrow, text, highlighter, mosaic, blur)
- **Menu Bar & Tray** - System tray icons and menu bar items
- **Widget System** - Declarative UI with 20+ widget types:
  - Basic: label, button, textInput, checkbox, slider, progressBar, separator, spacer
  - Layout: hbox, vbox, grid
  - Containers: panel, scrollArea, group
  - Advanced: dropdown, radioGroup, textArea, tabs, image
  - Interactive: link, selectableLabel, dragValue, colorPicker, hyperlink, imageButton
- **File Dialogs** - Native open/save/folder picker dialogs with filters (Note: requires main thread on macOS)
- **Font Enumeration** - Get list of system font families

### Clipboard Manager (v0.2.2+)
- **History Tracking** - Background monitoring with 500-entry limit
- **Content Types** - Text, images, and files
- **Sensitive Detection** - Auto-detect passwords, API keys, private keys, credit cards
- **Search & Filter** - Category filters and text search
- **Export/Import** - JSON format (sensitive entries excluded)

## Installation

### As a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
aumate = "0.2"
```

### Feature Flags

Enable only what you need:

```toml
[dependencies]
# Core automation only (no GUI)
aumate = { version = "0.2", default-features = false, features = ["input", "clipboard", "window"] }

# Full GUI support (default)
aumate = "0.2"
```

Available features:
- `input` - Mouse and keyboard control
- `screen` - Screen capture (requires system libraries on Linux)
- `clipboard` - Clipboard operations
- `window` - Window management
- `gui` - Full GUI framework with effects (includes screen, clipboard, and clipboard manager)

## Library Usage

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
    clipboard::set_text("Copied text")?;
    let text = clipboard::get_text()?;
    println!("Clipboard: {}", text);

    Ok(())
}
```

## Platform Support

| Platform | Status |
|----------|--------|
| macOS    | Full support |
| Windows  | Full support |
| Linux    | Requires X11/Wayland dependencies |

### Linux Dependencies

```bash
# Ubuntu/Debian
sudo apt-get install libxcb1-dev libxrandr-dev libdbus-1-dev libpipewire-0.3-dev libasound2-dev

# Fedora
sudo dnf install libxcb-devel libXrandr-devel dbus-devel pipewire-devel alsa-lib-devel
```

## Roadmap

Planned features (contributions welcome!):

- [ ] **OCR Integration** - Text recognition from screenshots
- [ ] **Hotkey System** - Global hotkey registration and handling
- [ ] **Macro Recording** - Record and replay mouse/keyboard actions
- [ ] **Plugin System** - Extensible action plugins
- [ ] **Multi-monitor** - Improved multi-display support
- [ ] **Accessibility** - Screen reader and accessibility API integration
- [ ] **Scripting** - Built-in scripting language for automation

## Contributing

Contributions are welcome! This project is in active development and we appreciate:

- Bug reports and feature requests via [GitHub Issues](https://github.com/tegojs/bot/issues)
- Pull requests for bug fixes and new features
- Documentation improvements
- Platform-specific testing and feedback

### Development

```bash
# Clone the repository
git clone https://github.com/tegojs/bot.git
cd bot/packages/aumate

# Build
cargo build

# Run tests
cargo test --all-features

# Run the GUI
cargo run
```

## Related Projects

- **[@tego/bot](https://www.npmjs.com/package/@tego/bot)** - Node.js bindings via napi-rs
- **[@tego/botjs](https://www.npmjs.com/package/@tego/botjs)** - TypeScript wrapper with enhanced APIs

### Running Examples

The monorepo includes TypeScript examples demonstrating all GUI features:

```bash
# Clone and setup
git clone https://github.com/tegojs/bot.git
cd bot
pnpm install
pnpm build

# Run examples (from project root)
pnpm ex:run gui-hello                # Simple GUI window
pnpm ex:run gui-form                 # Form with inputs
pnpm ex:run gui-events               # GUI event handling
pnpm ex:run gui-notification         # Notification windows
pnpm ex:run gui-widgets              # Dropdown, RadioGroup, TextArea, Tabs
pnpm ex:run gui-interactive-widgets  # Link, SelectableLabel, DragValue, ColorPicker, Hyperlink, ImageButton
pnpm ex:run gui-file-dialogs         # Native file open/save/folder dialogs
pnpm ex:run gui-font-picker          # System font enumeration and font rendering

# Other examples
pnpm ex:run basic                    # Basic mouse/keyboard operations
pnpm ex:run clipboard                # Clipboard operations
pnpm ex:run screenshot-basic         # Basic screenshot
```

## License

MIT License - see [LICENSE](LICENSE) for details.

---

**Note:** This package is under active development. While we strive for stability, breaking changes may occur between minor versions during the prototype phase. Pin to a specific version if stability is critical for your project.
