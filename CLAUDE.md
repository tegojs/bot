# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a **monorepo** containing two major projects:

### 1. Tego Bot - Node.js Automation Library
**Tego Bot** is a high-performance desktop automation library for Node.js, powered by Rust using N-API bindings. It provides robotjs-compatible APIs with superior performance and memory safety.

**Packages** (`packages/`):
- `@tego/bot` - Rust core with N-API bindings
- `@tego/botjs` - TypeScript wrapper with full type safety
- `@tego/bot-agent` - AI-powered CLI for script generation
- `aumate` - Cross-platform Rust automation library

### 2. Aumate App - Tauri Desktop Application
**Aumate App** is a desktop automation application built with Tauri, React, and TypeScript. It implements **Domain-Driven Design (DDD)** architecture with strict layering.

**Structure** (`aumate-app/` and `crates/`):
- DDD layered architecture (Domain → Application → Infrastructure → API)
- Workspace-based dependency management
- Cross-platform support (Windows, macOS, Linux)

**For DDD architecture rules**, see `.cursorrules` in the repository root.

## Architecture

### Tego Bot Packages Architecture

The Node.js automation packages use a **monorepo workspace** structure:

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
  - Image template matching
  - Comprehensive test coverage

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
- **Purpose**: Cross-platform desktop automation library
- **Features** (configurable via Cargo features):
  - `input` - Mouse and keyboard control via `enigo`
  - `screen` - Screen capture via `xcap`
  - `clipboard` - Text and image clipboard operations via `arboard`
  - `window` - Window management via `active-win-pos-rs`
  - `image_match` - Image template matching via `imageproc`
- **Module structure**:
  - `input/` - Mouse and keyboard control (implements `FromStr` for button parsing)
  - `screen.rs` - Screen capture operations
  - `clipboard.rs` - Clipboard operations
  - `window.rs` - Window management
  - `image_match/` - Image template matching engine

### Aumate App DDD Architecture

The Tauri application (`aumate-app/` and `crates/`) implements **Domain-Driven Design** with strict layering:

```
API Layer (Tauri Commands)
    ↓ calls
Application Layer (Use Cases)
    ↓ depends on
Domain Layer (Ports - Trait Interfaces)
    ↑ implements
Infrastructure Layer (Adapters)
```

**Crate Structure**:
```
crates/
├── core/
│   ├── shared/        # Shared types, errors
│   ├── domain/        # Domain models
│   └── traits/        # Port interfaces
├── application/       # Use Cases + DTOs
└── infrastructure/    # Adapters + Services

aumate-app/src-tauri/  # API Layer
└── src/
    ├── commands/      # Tauri Commands
    ├── state.rs       # AppState
    └── setup.rs       # Dependency Injection
```

**Key Principles**:
- Domain Layer defines interfaces (Ports), no dependencies on other layers
- Infrastructure Layer implements interfaces (Adapters)
- Application Layer depends only on Port interfaces, not concrete implementations
- API Layer only validates parameters and calls Use Cases
- Naming: `XxxPort` (interface), `XxxAdapter` (implementation), `XxxUseCase` (use case)

**Recent Improvements** (Dec 2024):
- ✅ Unified workspace dependency management (all versions in root `Cargo.toml`)
- ✅ Fixed Windows UI Automation dependencies
- ✅ Resolved F2 hotkey conflict (changed to Ctrl+4)
- ✅ Implemented global shortcut management following DDD architecture
  - Port: `GlobalShortcutPort` in `crates/core/traits/`
  - Adapter: `GlobalShortcutAdapter` in `crates/infrastructure/`
  - Use Cases: Register/Unregister/CheckAvailability in `crates/application/`
  - Commands: `register_global_shortcut`, `unregister_global_shortcut`, `check_global_shortcut_availability`

### Shared Design Patterns
- **Thread-safe state**: Rust modules use `Arc<Mutex<>>` for shared state
- **Async operations**: Screen capture and clipboard operations are async (use `tokio` runtime)
- **Rust 2024 edition**: All Rust packages use edition 2024, requiring Rust 1.85+
- **Cargo workspace**: All packages managed via workspace in root `Cargo.toml`
  - Tego Bot: `members = ["packages/*"]`
  - Aumate App: `members = ["aumate-app/src-tauri", "crates/*"]`

## Development Commands

### Tego Bot - Build
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
```

### Aumate App - Build
```bash
# Build Tauri app (development)
cd aumate-app && pnpm tauri dev

# Build Tauri app (production)
cd aumate-app && pnpm tauri build

# Check workspace (all crates)
cargo check --workspace

# Check specific crate
cargo check --package aumate-infrastructure
cargo check --package aumate-application
cargo check --package aumate-app
```

### Tego Bot - Testing
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

### Aumate App - Testing
```bash
# Test workspace
cargo test --workspace

# Test specific crate
cargo test --package aumate-application
cargo test --package aumate-infrastructure
```

### Linting & Formatting (Both Projects)
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

### Tego Bot
- **N-API bindings**: Rust code compiles to native Node.js addon. Changes require `pnpm rs:build`.
- **Workspace structure**: Use `pnpm --filter <package-name>` for specific packages, `pnpm -r` for all.
- **Async operations**: Screen capture and clipboard functions are async (return Promises).
- **Binary distribution**: `@tego/bot` includes pre-built binaries in `dist/` after building.
- **AI Agent**: Requires OpenAI API key. Scripts stored in `~/.tego/bot-scripts/`.
- **Test Coverage**: 36 API functions with 41 unit + 46 integration tests.

### Aumate App
- **DDD Architecture**: See `.cursorrules` for complete architecture guidelines and implementation flow.
- **Dependency Management**: All versions centralized in root `Cargo.toml` workspace.dependencies.
- **Platform Support**: Windows (UI Automation), macOS (Accessibility), Linux (experimental).
- **New Feature Implementation**: Follow the 5-step process in `.cursorrules`:
  1. Define Port interface in `crates/core/traits/`
  2. Implement Adapter in `crates/infrastructure/adapters/`
  3. Create Use Case in `crates/application/use_cases/`
  4. Add Tauri Command in `aumate-app/src-tauri/src/commands/`
  5. Wire dependencies in `state.rs` and `setup.rs`
- **Global Shortcuts**: Implemented following DDD (see `HOTKEY_CHANGES.md` for details).
- **Compilation**: Run `cargo check --workspace` before committing.

### Both Projects
- **Rust 2024 edition**: All Rust packages require Rust 1.85+
- **Git hooks**: Lefthook (Rust checks) + simple-git-hooks (JS/TS linting)
- **Documentation**: 
  - Tego Bot: VitePress + TypeDoc (`pnpm docs:dev`)
  - Aumate App: See `crates/docs/` for architecture details
- **Context7 MCP**: Use for version-specific docs lookup
- **Roadmap**: See `docs/developments/aumate-roadmap.md`
