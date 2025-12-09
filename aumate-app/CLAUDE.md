# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**aumate-app** is a Tauri v2 desktop application that serves as the GUI frontend for the Tego Bot automation suite. It uses React 19 + TypeScript for the frontend and Rust for the native backend, built with Vite.

This package is part of the larger Tego Bot monorepo (`@tego/*` packages) and will integrate with the `aumate` Rust library for desktop automation features.

## Architecture

```
aumate-app/
├── src/                    # React frontend (TypeScript)
│   ├── App.tsx            # Main application component
│   ├── main.tsx           # React entry point
│   └── assets/            # Static assets
├── src-tauri/             # Tauri backend (Rust)
│   ├── src/
│   │   ├── lib.rs         # Tauri commands and app setup
│   │   └── main.rs        # Binary entry point
│   ├── Cargo.toml         # Rust dependencies
│   ├── tauri.conf.json    # Tauri configuration
│   └── capabilities/      # Tauri security capabilities
└── public/                # Public static files
```

### Frontend-Backend Communication
- Use `invoke()` from `@tauri-apps/api/core` to call Rust commands
- Define Rust commands with `#[tauri::command]` macro in `src-tauri/src/lib.rs`
- Register commands in `tauri::Builder::default().invoke_handler()`

## Development Commands

```bash
# Start development server (frontend + backend with hot reload)
pnpm tauri dev

# Build production app
pnpm tauri build

# Frontend only (for UI development without Tauri)
pnpm dev

# Type check and build frontend
pnpm build

# Preview frontend build
pnpm preview
```

## Key Dependencies

### Frontend
- React 19 with TypeScript
- Vite 7 as build tool
- `@tauri-apps/api` v2 for Tauri IPC
- `@tauri-apps/plugin-opener` for system file/URL opening

### Backend (Rust)
- Tauri v2 framework
- `tauri-plugin-opener` for opener functionality
- `serde` / `serde_json` for serialization

## Adding Tauri Commands

1. Define the command in `src-tauri/src/lib.rs`:
```rust
#[tauri::command]
fn my_command(arg: String) -> Result<String, String> {
    Ok(format!("Result: {}", arg))
}
```

2. Register in the invoke handler:
```rust
.invoke_handler(tauri::generate_handler![greet, my_command])
```

3. Call from React:
```typescript
import { invoke } from "@tauri-apps/api/core";
const result = await invoke("my_command", { arg: "value" });
```

## Configuration

- **App identifier**: `com.zhanglin.aumate-app`
- **Dev server**: `http://localhost:1420`
- **Window**: 800x600, configurable in `tauri.conf.json`

## Relation to Parent Monorepo

This app will integrate with the `aumate` Rust crate from `packages/aumate` for desktop automation features (mouse/keyboard control, screen capture, clipboard, window management, GUI widgets). To add aumate as a dependency, update `src-tauri/Cargo.toml`:

```toml
[dependencies]
aumate = { path = "../../packages/aumate", features = ["input", "screen", "clipboard", "window"] }
```
