# Aumate App Architecture

**Version**: v0.3.0
**Architecture**: Domain-Driven Design (DDD)
**Language**: Rust (Edition 2024)
**Framework**: Tauri 2.x

---

## Overview

Aumate App is a desktop automation application built with Tauri, implementing **Domain-Driven Design (DDD)** architecture with strict layering principles.

### Project Statistics

| Metric | Value |
|--------|-------|
| **Tauri Commands** | 28 |
| **Crates** | 5 |
| **Port Traits** | 10 |
| **Adapters** | 11 |
| **Use Cases** | 17+ |
| **Code Reduction** | 71% (98 → 28 commands) |

---

## Layered Architecture

```
┌─────────────────────────────────────────────────┐
│          Tauri Commands (API Layer)             │  ← 28 Commands
├─────────────────────────────────────────────────┤
│       Application Layer (Use Cases)             │  ← Business Orchestration
├─────────────────────────────────────────────────┤
│      Domain Layer (Business Logic)              │  ← Domain Models & Rules
├─────────────────────────────────────────────────┤
│   Infrastructure Layer (Adapters & Services)    │  ← Platform Implementation
└─────────────────────────────────────────────────┘
```

### Crate Organization

```
crates/
├── core/
│   ├── shared/       # Shared types (Rectangle, Point, IDs)
│   ├── domain/       # Domain models (Image, Screenshot, Page)
│   └── traits/       # Port interface definitions
├── application/      # Use Cases (business use cases)
└── infrastructure/   # Adapters & Services (platform implementation)

aumate-app/src-tauri/ # API Layer
└── src/
    ├── commands/     # Tauri Commands (API gateway)
    ├── state.rs      # AppState
    └── setup.rs      # Dependency injection
```

---

## Design Principles

### 1. Dependency Inversion

- **Domain Layer** defines `Port` traits (interfaces)
- **Infrastructure Layer** implements `Adapter` (adapters)
- **Application Layer** calls through Port, never directly depending on Adapter

```rust
// Domain Layer defines interface
pub trait ScreenCapturePort: Send + Sync {
    async fn capture(...) -> Result<Screenshot>;
}

// Infrastructure Layer implements
pub struct ScreenCaptureAdapter { ... }
impl ScreenCapturePort for ScreenCaptureAdapter { ... }

// Application Layer uses interface
pub struct CaptureScreenUseCase {
    capture: Arc<dyn ScreenCapturePort>,  // Depends on interface, not implementation
}
```

### 2. Single Responsibility

- **API Layer**: Only responsible for parameter validation and error conversion
- **Application Layer**: Only responsible for business process orchestration
- **Domain Layer**: Only contains domain logic
- **Infrastructure Layer**: Only responsible for technical implementation

### 3. Open/Closed Principle

- Through Port/Adapter pattern, new platform support can be added without modifying core code
- Through Use Case composition, new business processes can be flexibly constructed

---

## Core Module Mappings

### Core/Shared - Shared Types

| Type | Description |
|------|-------------|
| `Rectangle` | Rectangle region (with clip_rect methods) |
| `Point` | Coordinate point |
| `MonitorId` | Monitor identifier |
| `WindowId` | Window identifier |
| `ScreenshotId` | Screenshot identifier |
| `PageId` | Page identifier |

### Core/Domain - Domain Models

| Model | Description |
|-------|-------------|
| `Image` | Image data model (with ImageMetadata) |
| `Screenshot` | Screenshot model (wraps Image + capture metadata) |
| `Monitor` | Monitor information |
| `Page` | Page model |
| `Hotkey` | Hotkey model |

### Core/Traits - Port Interfaces

| Port Interface | Platform | Description |
|----------------|----------|-------------|
| `ScreenCapturePort` | macOS | Screen capture |
| `ImageProcessingPort` | Universal | Image processing |
| `StoragePort` | Filesystem/Memory | File storage |
| `ScrollCapturePort` | macOS | Scroll screenshot |
| `WindowManagementPort` | macOS | Window management |
| `UIAutomationPort` | macOS (framework) | UI automation |
| `HotkeyPort` | macOS | Hotkey listening |
| `ClipboardPort` | Cross-platform (arboard) | Clipboard operations |
| `PageManagementPort` | Tauri | Page pool management |
| `WindowListPort` | macOS/Windows/Linux | Window list |

---

## API Layer Commands (28 Total)

### Screenshot (3 commands)
- `capture_current_monitor` - Capture current monitor
- `capture_monitor` - Capture specific monitor by ID
- `capture_region` - Capture specific region

### Scroll Screenshot (1 command)
- `start_scroll_capture` - Start scroll capture (merged multiple steps)

### Window Management (7 commands)
- `create_window` - Create window (merged multiple creation commands)
- `drag_window` - Drag window
- `resize_window` - Resize window
- `pin_window` - Pin window always on top
- `unpin_window` - Unpin window
- `close_window` - Close window
- `get_window_elements` - Get system window list

### Monitor (2 commands)
- `get_monitors` - Get all monitors
- `get_current_monitor` - Get current monitor info

### Hotkey (4 commands)
- `listen_key_start` / `listen_key_stop` - Keyboard listener
- `listen_mouse_start` / `listen_mouse_stop` - Mouse listener

### Clipboard (6 commands)
- `read_clipboard` / `write_clipboard` - General clipboard operations
- `read_clipboard_image` / `write_clipboard_image` - Image clipboard operations
- `clear_clipboard` - Clear clipboard
- `get_clipboard_types` - Get available types

### UI Automation (2 commands)
- `get_element_from_position` - Get UI element at position
- `init_ui_elements` - Initialize UI elements

### Page Management (2 commands)
- `add_page` - Add page to pool
- `remove_page` - Remove page from pool

---

## Infrastructure Layer

### Adapters (11)

| Adapter | Platform | Description |
|---------|----------|-------------|
| `ScreenCaptureAdapter` | macOS | Screen capture using `xcap` + macOS API |
| `ImageProcessingAdapter` | Universal | HDR, overlay, encoding |
| `FileSystemAdapter` | Universal | File storage |
| `MemoryCacheAdapter` | Universal | Memory cache |
| `ScrollCaptureAdapter` | macOS | Scroll screenshot capture |
| `WindowManagementAdapter` | Tauri | Window management |
| `UIAutomationAdapter` | macOS | UI automation framework |
| `HotkeyListenerAdapter` | macOS | Hotkey listener |
| `ClipboardAdapter` | Cross-platform | Using `arboard` library |
| `PageManagementAdapter` | Tauri | Page pool management |
| `WindowListAdapter` | Cross-platform | Using `active-win-pos-rs` |

### Services (8)

| Service | Status |
|---------|--------|
| `DeviceEventHandlerService` | Fully migrated |
| `ListenKeyService` | Fully migrated |
| `ListenMouseService` | Fully migrated |
| `EnigoManager` | Fully migrated |
| `ScrollScreenshotService` | Fully migrated |
| `ScrollScreenshotCaptureService` | Fully migrated |
| `ScrollScreenshotImageService` | Fully migrated |
| `HotLoadPageService` | Fully migrated |

### Platform-Specific Modules

| Module | Status |
|--------|--------|
| `macos/ui_automation.rs` | Framework migrated |
| `macos/notification.rs` | Fully migrated |
| `macos/shared.rs` | Fully migrated |
| `macos/utils.rs` | Fully migrated |

---

## Removed Features

The following features were removed during the DDD refactoring:

### File Management (8 commands)
Removed because Tauri provides built-in filesystem API.

### OCR Service (4 commands)
OCR functionality separated into plugin system.

### Video Recording (3 commands)
Temporarily removed, may return as a plugin.

### Miscellaneous Utilities (20+ commands)
Various commands removed because they are:
- Available as Tauri built-ins
- Better handled by frontend
- Security concerns
- Should use Tauri plugins

---

## Architecture Benefits

### 1. Maintainability
- Clear DDD layering with defined responsibilities
- Dependency injection for easy testing and replacement
- Code organized by domain, not technology

### 2. Extensibility
- Port/Adapter pattern allows new platforms without modifying core
- Use Cases can be flexibly combined for new business processes
- Modular crates with independent compilation and testing

### 3. Testability
- All Ports can be mocked
- Each layer can be tested independently
- Integration tests at Use Case layer

### 4. Code Quality
- Strong typing with typed IDs (`MonitorId`, `WindowId`, etc.)
- Layered error types
- Consistent naming conventions

---

**Last Updated**: December 2025
**Maintainer**: Aumate Team
