# @tego/bot - `packages`

This folder contains all the **modular packages** that make up the @tego/bot desktop automation library.  
It is part of the **monorepo** managed with [`pnpm workspaces`](https://pnpm.io/workspaces).

Each package is **independent**, versioned, and published under the `@tego/*` namespace.

---

## 📂 Project structure

```
📂 packages/
├── 📂 bot
│   ├── 📄 Cargo.toml
│   ├── 📄 package.json
│   ├── 📄 build.rs
│   ├── 📄 README.md
│   ├── 📂 src
│   │   ├── 📄 lib.rs
│   │   ├── 📄 api.rs
│   │   ├── 📄 mouse.rs
│   │   ├── 📄 keyboard.rs
│   │   └── 📄 screen.rs
│   └── 📂 tests
│       └── 📄 integration_tests.rs
└── 📂 botjs
    ├── 📄 package.json
    ├── 📄 tsconfig.json
    ├── 📄 tsdown.config.ts
    ├── 📄 readme.md
    ├── 📂 src
    │   └── 📄 index.ts
    └── 📂 tests
        └── 📄 server.test.ts
```

---

## 📦 Packages

### `@tego/bot`
- The **Rust core** of the desktop automation library, exposing high-performance automation functionality via [N-API](https://github.com/napi-rs/napi-rs).  
- Responsibilities:
  - Mouse control (movement, clicks, scrolling, dragging).  
  - Keyboard simulation (key presses, text input, modifiers).  
  - Screen capture and pixel color reading.  
  - Integration with JS/TS through N-API bindings.  
- This package is **fully written in Rust**, compiled to a native module, and serves as the runtime for all automation operations.

### `@tego/botjs` (TypeScript library)
- Main **TypeScript wrapper** for @tego/bot, exposing a developer-friendly API.  
- Responsibilities:
  - Type-safe wrappers for all automation functions.  
  - Enhanced error handling and type definitions.  
  - Developer-friendly API with full TypeScript support.  
  - Re-exports and organizes the native module functions.  
- This package **depends on `@tego/bot`** for the runtime, but provides a polished TypeScript experience.

---

## 🛠️ Development workflow

Inside the monorepo, you can work on packages in isolation or all together:

```bash
# Build all packages
pnpm build

# Build only Rust core
pnpm rs:build

# Build only TypeScript wrapper
pnpm ts:build

# Build a specific package
pnpm --filter @tego/bot build
pnpm --filter @tego/botjs build
```

Each package is published independently but linked locally via the workspace.

---

## 🚀 Quick Start

```bash
# Install dependencies
pnpm install

# Build everything
pnpm build

# Run tests
pnpm test
```

---
