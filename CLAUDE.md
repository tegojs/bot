# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Tego Bot** is a high-performance desktop automation library for Node.js, powered by a Rust core using N-API bindings. It provides robotjs-compatible APIs for mouse control, keyboard input, screen capture, clipboard operations, and window management with superior performance and memory safety.

The project consists of npm packages under the `@tego/*` namespace and a Rust automation library:
- `@tego/bot` - Rust core with N-API bindings (depends on aumate)
- `@tego/botjs` - TypeScript wrapper with full type safety
- `@tego/bot-agent` - AI-powered CLI for script generation
- `aumate` - Cross-platform desktop automation library with GUI support (Rust)

## Architecture

The project uses a **monorepo workspace** structure with the following packages:

### 1. `packages/bot` - Rust Core (`@tego/bot`)
- **Language**: Rust 2024 edition (requires Rust 1.85+)
- **Build system**: Cargo with napi-build
- **Key dependencies**:
  - `napi` and `napi-derive` for Node.js bindings
  - `aumate` - Core automation library (local workspace dependency)
- **Module structure**:
  - `lib.rs` - N-API exports wrapping aumate functions (uses `#[napi]` macros)
- **Binary output**: Compiled to `cdylib` (native Node.js addon)
- **36 Exported Functions**:
  - Mouse: `moveMouse`, `moveMouseSmooth`, `mouseClick`, `mouseToggle`, `dragMouse`, `scrollMouse`, `getMousePos`, `setMouseDelay`
  - Keyboard: `keyTap`, `keyToggle`, `typeString`, `typeStringDelayed`, `unicodeTap`, `setKeyboardDelay`
  - Screen: `bitmapColorAt`, `captureScreen`, `captureScreenRegion`, `getPixelColor`, `getScreen`, `getScreenSize`, `updateScreenMetrics`
  - Clipboard: `getClipboard`, `setClipboard`, `clearClipboard`, `getClipboardImage`, `setClipboardImage`
  - Window: `getActiveWindow`, `getAllWindows`, `findWindowsByTitle`, `findWindowsByProcess`
  - Helpers: `doubleClick`, `rightClick`, `middleClick`, `leftClick`, `mouseDown`, `mouseUp`

### 2. `packages/botjs` - TypeScript Wrapper (`@tego/botjs`)
- **Language**: TypeScript (ESM)
- **Purpose**: Re-exports the Rust bindings from `@tego/bot` with TypeScript types and additional utilities
- **Build**: `tsdown` for TypeScript compilation
- **Testing**: Vitest with optional integration tests (set `ENABLE_INTEGRATION_TESTS=true`)
- **Documentation**: TypeDoc for API docs generation (output to markdown format)
- **Key features**:
  - Full TypeScript type definitions
  - Enhanced screen capture utilities
  - Convenience functions for common automation tasks
  - Comprehensive test coverage
- **GUI Widgets** (via `@tego/botjs` gui module):
  - Basic: `label`, `button`, `textInput`, `checkbox`, `slider`, `progressBar`, `separator`, `spacer`
  - Layout: `hbox`, `vbox`, `grid`
  - Containers: `panel`, `scrollArea`, `group`
  - Advanced: `dropdown`, `radioGroup`, `textArea`, `tabs`, `image`
  - Interactive: `link`, `selectableLabel`, `dragValue`, `colorPicker`, `hyperlink`, `hyperlinkUrl`, `imageButton`
- **File Dialog Functions**: `showOpenFileDialog`, `showSaveFileDialog`, `showFolderDialog`
- **Font Functions**: `getSystemFonts` - returns sorted list of system font families

### 3. `packages/bot-agent` - AI-Powered CLI (`@tego/bot-agent`)
- **Language**: TypeScript (ESM)
- **Purpose**: CLI tool for generating automation scripts using AI (OpenAI-compatible APIs)
- **Binary**: `bot-agent` command-line tool
- **Key features**:
  - Generate automation scripts from natural language descriptions
  - Edit existing scripts through conversational AI
  - Execute and manage saved scripts
  - Store scripts in `~/.tego/bot-scripts/` with conversation history
- **Dependencies**: OpenAI client, Commander.js, Inquirer prompts, Chalk, Ora, cli-highlight
- **Configuration**: Uses environment variables (OPENAI_API_KEY, OPENAI_BASE_URL, OPENAI_MODEL)
- **Script execution**: Uses `tsx` for direct TypeScript execution

### 4. `packages/aumate` - Desktop Automation Library (`aumate`)
- **Language**: Rust 2024 edition (requires Rust 1.85+)
- **Purpose**: Cross-platform desktop automation library with GUI support
- **Features** (configurable via Cargo features):
  - `input` - Mouse and keyboard control via `enigo` and `rdev`
  - `screen` - Screen capture via `xcap`
  - `clipboard` - Text and image clipboard operations via `arboard`
  - `window` - Window management via `active-win-pos-rs`
  - `gui` - Floating window system with effects (includes screen and clipboard)
- **Key dependencies**:
  - `winit` for window management
  - `wgpu` for GPU rendering
  - `egui` / `egui-wgpu` / `egui-winit` for UI
  - `muda` for menu support
  - `tray-icon` for system tray
  - `rfd` for native file dialogs
  - `font-kit` for system font enumeration
  - `noise` for procedural effects
  - `glam` for math types
- **Module structure**:
  - `input/` - Mouse and keyboard control (implements `FromStr` for button parsing)
  - `screen.rs` - Screen capture operations
  - `clipboard.rs` - Clipboard operations
  - `window.rs` - Window management
  - `gui/` - GUI sub-modules:
    - `window/` - Window controller, floating windows, configuration
    - `effect/` - Particle effects system with 18 presets
    - `animation/` - Animation system with easing functions
    - `content/` - Content rendering (image, text)
    - `menu_bar/` - System tray and menu bar support
  - `screenshot/` - Screenshot mode with selection and toolbar:
    - `plugins/` - Plugin system (save, copy, cancel)
    - `renderer/` - WGPU/egui rendering
    - `ui/` - Toolbar components
- **18 Particle Effects**: Rotating Halo, Pulse Ripple, Flowing Light, Stardust Scatter, Electric Spark, Smoke Wisp, Aurora Wave, Cosmic Strings, Heartbeat Pulse, Laser Beam, Lightning Arc, Matrix Rain, Meteor Shower, Orbit Rings, Rain Drop, Silk Ribbon, Sonar Pulse, Fire Glow
- **Animations**: fade, scale, slide, bounce, rotate, blink with easing functions
- **25 GUI Widgets**: Label, Button, TextInput, Checkbox, Slider, ProgressBar, Image, Separator, Spacer, HBox, VBox, Grid, Panel, ScrollArea, Group, Dropdown, RadioGroup, TextArea, Tabs, Link, SelectableLabel, DragValue, ColorPicker, Hyperlink, ImageButton
- **19 Widget Events**: button_click, text_changed, text_submit, checkbox_changed, slider_changed, focus_gained, focus_lost, mouse_enter, mouse_leave, selection_changed, radio_changed, tab_changed, link_clicked, selectable_label_changed, drag_value_changed, color_changed, hyperlink_clicked, file_dialog_completed, font_changed

### Key Design Patterns
- **Thread-safe state**: Rust modules use `Arc<Mutex<>>` for shared state (Enigo instances, delay settings, clipboard)
- **Async operations**: Screen capture and clipboard operations are async (use `tokio` runtime)
- **Global delay settings**: Mouse and keyboard operations respect global delay values set via `setMouseDelay()` and `setKeyboardDelay()`
- **Rust 2024 edition**: All Rust packages use edition 2024, requiring Rust 1.85+
- **VitePress documentation**: Uses VitePress for documentation site with TypeDoc-generated markdown API docs
- **Cargo workspace**: All Rust packages are managed via workspace in root `Cargo.toml` with `members = ["packages/*"]`

## Development Commands

### Build
```bash
# Full build (Rust + TypeScript)
pnpm build

# Build Rust bindings only
pnpm rs:build

# Build TypeScript wrapper only
pnpm ts:build

# Build Rust for specific platform
pnpm rs:build --platform

# Build AI agent CLI only
pnpm agent:build

# Build aumate library
cargo build -p aumate

# Build aumate with specific features
cargo build -p aumate --no-default-features --features "input,screen"

# Run aumate demo (screenshot mode)
cargo run -p aumate
```

### Testing
```bash
# Run all tests (Rust + TypeScript)
pnpm test

# Run TypeScript tests only
pnpm --filter @tego/botjs test

# Run TypeScript tests with coverage
pnpm --filter @tego/botjs test:coverage

# Run integration tests (requires ENABLE_INTEGRATION_TESTS=true)
pnpm --filter @tego/botjs test:integration

# Run Rust tests only
cargo test --all-features

# Run Rust tests in a specific package
cargo test -p bot
cargo test -p aumate

# Run AI agent tests
pnpm agent:test
```

### Linting & Formatting
```bash
# Check TypeScript/JavaScript formatting
pnpm fmt

# Fix TypeScript/JavaScript formatting
pnpm fmt:fix

# Lint TypeScript/JavaScript
pnpm lint

# Fix TypeScript/JavaScript linting issues
pnpm lint:fix

# Check and fix both formatting and linting
pnpm check:fix

# Rust formatting
cargo fmt --all

# Rust linting
cargo clippy --all-targets --all-features -- -D warnings

# Rust compilation check
cargo check --all-targets --all-features
```

### Examples
```bash
# Run example scripts (pass example name without .ts extension)
pnpm ex:run <name>

# Available examples:
pnpm ex:run basic                    # Basic mouse/keyboard operations
pnpm ex:run clipboard                # Clipboard operations
pnpm ex:run mouse-shortcuts          # Mouse shortcut helpers
pnpm ex:run screenshot-basic         # Basic screenshot
pnpm ex:run screenshot-advanced      # Advanced screenshot operations
pnpm ex:run screenshot-color-picker  # Color picker from screen
pnpm ex:run window                   # Window management
pnpm ex:run gui-hello                # Simple GUI window
pnpm ex:run gui-form                 # Form with inputs
pnpm ex:run gui-events               # GUI event handling
pnpm ex:run gui-notification         # Notification windows
pnpm ex:run gui-widgets              # Dropdown, RadioGroup, TextArea, Tabs
pnpm ex:run gui-interactive-widgets  # Link, SelectableLabel, DragValue, ColorPicker, Hyperlink, ImageButton
pnpm ex:run gui-file-dialogs         # Native file open/save/folder dialogs
pnpm ex:run gui-font-picker          # System font enumeration and picker
```

### Documentation
```bash
# Generate TypeScript API docs (TypeDoc -> Markdown)
pnpm docs:api

# Or from botjs package
pnpm --filter @tego/botjs docs

# Build VitePress documentation site
pnpm docs:build

# Develop documentation site (watch mode)
pnpm docs:dev

# Preview built documentation
pnpm docs:preview
```

### AI Agent CLI
```bash
# Build the CLI
pnpm agent:build

# Development mode with watch
pnpm agent:dev

# Generate new automation script
npx bot-agent generate

# Edit existing script
npx bot-agent edit [script-name]

# Execute saved script
npx bot-agent execute [script-name]

# List all saved scripts
npx bot-agent list
```

## Git Hooks

The repository uses **Lefthook** for git hooks (configuration in `lefthook.yml`), along with **simple-git-hooks** and **lint-staged** for file-level linting.

### Pre-commit Hooks
- **Lefthook** (parallel):
  - **Rust**: `cargo fmt --check`, `cargo clippy`, `cargo check`
- **lint-staged** (via simple-git-hooks):
  - **TypeScript/JavaScript**: `biome check --write` on staged files

### Pre-push Hooks (sequential)
- **Tests**: `cargo test --all-features`
- **Security**: `cargo audit` and `pnpm audit --audit-level moderate`

### Commit Message Validation
- Enforces **Conventional Commits** format via `commitlint`: `<type>[optional scope]: <description>`
- Valid types: `build`, `chore`, `ci`, `docs`, `feat`, `fix`, `perf`, `refactor`, `revert`, `style`, `test`
- Max length: 72 characters
- Requires imperative mood (e.g., "add" not "added")

## Cross-Platform Considerations

### macOS
- Requires macOS 10.13+
- Screen recording permission required (System Preferences > Security & Privacy > Screen Recording)

### Linux
- May require system dependencies:
  ```bash
  # Ubuntu/Debian
  sudo apt-get install libxcb1-dev libxrandr-dev libdbus-1-dev

  # Fedora
  sudo dnf install libxcb-devel libXrandr-devel dbus-devel
  ```

### Windows
- Windows 10+ supported
- No additional configuration needed

## Performance Optimization

The Rust release profile in `Cargo.toml` is optimized for maximum performance:
- LTO enabled (`lto = "fat"`)
- Single codegen unit (`codegen-units = 1`)
- Full optimization (`opt-level = 3`)
- Binary stripping enabled (`strip = true`)

## AI Agent CLI (`@tego/bot-agent`)

The **bot-agent** CLI generates automation scripts using OpenAI-compatible APIs. It provides a conversational interface for creating, editing, and executing automation scripts.

### Configuration
Set environment variables before using the agent:
```bash
export OPENAI_API_KEY="your-api-key"
export OPENAI_BASE_URL="https://api.openai.com/v1"  # Optional
export OPENAI_MODEL="gpt-4"  # Optional, defaults to gpt-4
```

### Script Storage
- All scripts are saved to `~/.tego/bot-scripts/`
- Each script has two files:
  - `script-name.ts` - The TypeScript code
  - `script-name.meta.json` - Metadata including conversation history
- Conversation history is preserved for iterative editing

### Workflow
1. **Generate**: User describes automation task → AI generates TypeScript code
2. **Review**: Code is displayed with syntax highlighting and validation
3. **Action**: User can execute, save, edit, or regenerate
4. **Edit**: Iterative refinement through natural language feedback
5. **Execute**: Scripts run using `tsx` for direct TypeScript execution

### Code Generation
The AI is instructed to:
- Import only from `@tego/botjs`
- Use proper async/await for screen operations
- Include error handling
- Add descriptive comments
- Follow TypeScript best practices

## Important Notes

- **N-API bindings**: The Rust code compiles to a native Node.js addon. Changes to Rust code require rebuilding with `pnpm rs:build`.
- **Workspace structure**: Use `pnpm --filter <package-name>` to run commands in specific packages, or `pnpm -r` for recursive execution across all packages.
- **Async screen operations**: Screen capture functions (`captureScreen`, `getPixelColor`, etc.) are async and return Promises.
- **Async clipboard operations**: Clipboard functions (`getClipboard`, `setClipboard`, `getClipboardImage`, etc.) are async.
- **Binary distribution**: The `@tego/bot` package includes pre-built binaries in the `dist` directory after building.
- **AI Agent**: Requires OpenAI API key set in environment variables. Scripts are stored locally in `~/.tego/bot-scripts/`.
- **Documentation site**: Uses VitePress for documentation with TypeDoc-generated API docs. Run `pnpm docs:dev` to develop locally.
- **Git hooks**: Uses both Lefthook (for Rust checks) and simple-git-hooks (for JS/TS linting). Install with `pnpm prepare`.
- **Context7 MCP for Documentation**: Use the context7 MCP tool (`mcp__context7__resolve-library-id` and `mcp__context7__get-library-docs`) to look up version-specific documentation for dependencies like egui, wgpu, winit, etc. This provides accurate API references for the exact versions used in the project.
- **Aumate Roadmap**: See `docs/developments/aumate-roadmap.md` for the development roadmap including completed features and planned enhancements.
- **Test Coverage**: All 36 API functions have comprehensive tests (41 unit tests + 46 integration tests). Run integration tests with `ENABLE_INTEGRATION_TESTS=true pnpm test`.
