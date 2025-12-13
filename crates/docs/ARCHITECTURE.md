# Aumate 架构说明文档

**版本**: v0.3.0  
**架构模式**: Domain-Driven Design (DDD)  
**编程语言**: Rust (Edition 2024)  
**框架**: Tauri 2.x

---

## 📐 整体架构

### 分层结构

```
┌─────────────────────────────────────────────┐
│          Tauri Commands (API Layer)         │  ← 27 个 Commands
├─────────────────────────────────────────────┤
│       Application Layer (Use Cases)         │  ← 业务流程编排
├─────────────────────────────────────────────┤
│      Domain Layer (Business Logic)          │  ← 领域模型和规则
├─────────────────────────────────────────────┤
│   Infrastructure Layer (Adapters & Services)│  ← 平台实现
└─────────────────────────────────────────────┘
```

### Crate 组织

```
src-tauri/crates/
├── core/
│   ├── shared/       # 共享类型 (Rectangle, Point, ID等)
│   ├── domain/       # 领域模型 (Image, Screenshot, Page等)
│   └── traits/       # Port 接口定义
├── application/      # Use Cases (业务用例)
├── infrastructure/   # Adapters & Services (平台实现)
└── api/              # Tauri Commands (API网关)
```

---

## 🎯 设计原则

### 1. 依赖倒置 (Dependency Inversion)
- **Domain Layer** 定义 `Port` trait（接口）
- **Infrastructure Layer** 实现 `Adapter`（适配器）
- **Application Layer** 通过 Port 调用，不直接依赖 Adapter

```rust
// Domain Layer 定义接口
pub trait ScreenCapturePort: Send + Sync {
    async fn capture(...) -> Result<Screenshot>;
}

// Infrastructure Layer 实现
pub struct ScreenCaptureAdapter { ... }
impl ScreenCapturePort for ScreenCaptureAdapter { ... }

// Application Layer 使用
pub struct CaptureScreenUseCase {
    capture: Arc<dyn ScreenCapturePort>,  // 依赖接口，不依赖实现
}
```

### 2. 单一职责 (Single Responsibility)
- **API Layer**: 仅负责参数验证和错误转换
- **Application Layer**: 仅负责业务流程编排
- **Domain Layer**: 仅包含领域逻辑
- **Infrastructure Layer**: 仅负责技术实现

### 3. 开闭原则 (Open/Closed)
- 通过 Port/Adapter 模式，可以无需修改核心代码就添加新的平台支持
- 通过 Use Case 组合，可以灵活构建新的业务流程

---

## 📦 核心模块映射

### Core/Shared - 共享类型

| 新类型 | 原功能来源 | 说明 |
|--------|-----------|------|
| `Rectangle` | `app-shared::ElementRect` | 矩形区域（合并 `clip_rect` 等方法） |
| `Point` | 新增 | 坐标点 |
| `MonitorId` | 新增 | 监视器标识 |
| `WindowId` | 新增 | 窗口标识 |
| `ScreenshotId` | 新增 | 截图标识 |
| `PageId` | 新增 | 页面标识 |

### Core/Domain - 领域模型

| 领域模型 | 原功能来源 | 说明 |
|---------|-----------|------|
| `Image` | `app-utils` 部分 | 图像数据模型（增加 `ImageMetadata`） |
| `Screenshot` | 新增 | 截图模型（包装 Image + 捕获元数据） |
| `Monitor` | `xcap::Monitor` | 监视器信息 |
| `Page` | `hot_load_page_service` | 页面模型 |
| `Hotkey` | 新增 | 热键模型 |

### Core/Traits - Port 接口

| Port 接口 | 对应旧模块 | 实现平台 |
|-----------|-----------|---------|
| `ScreenCapturePort` | `tauri-commands/screenshot` | macOS |
| `ImageProcessingPort` | `app-utils` | 通用 |
| `StoragePort` | `file::*` commands | 文件系统/内存 |
| `ScrollCapturePort` | `app-scroll-screenshot-service` | macOS |
| `WindowManagementPort` | `core::*` window commands | macOS |
| `UIAutomationPort` | `app-os::ui_automation` | macOS（框架） |
| `HotkeyPort` | `listen_key/mouse_service` | macOS |
| `ClipboardPort` | `read_image_from_clipboard` | arboard (跨平台) |
| `PageManagementPort` | `hot_load_page_service` | Tauri |

---

## 🔄 功能迁移映射

### 1. Screenshot 功能组

#### 保留并优化
- ✅ **`capture_current_monitor`** ← 直接保留
  - Application: `CaptureScreenUseCase`
  - Infrastructure: `ScreenCaptureAdapter` (`xcap` + macOS API)
  - 原代码: `tauri-commands/screenshot::capture_current_monitor`

- ✅ **`capture_monitor`** ← 合并 `capture_all_monitors`
  - 通过 `monitor_id` 参数化
  - 原代码: `tauri-commands/screenshot::capture_all_monitors`

- ✅ **`capture_region`** ← 合并 `capture_focused_window`
  - 通过 `region` 参数指定捕获区域
  - 原代码: `tauri-commands/screenshot::capture_focused_window`

#### 内部处理（移除）
- ❌ `init_ui_elements` - 自动初始化
- ❌ `init_ui_elements_cache` - 自动初始化
- ❌ `set_draw_window_style` - 内部处理

### 2. Scroll Screenshot 功能组

#### 保留并简化
- ✅ **`start_scroll_capture`** ← 合并多个步骤
  - Application: `ScrollScreenshotUseCase`
  - Infrastructure: `ScrollCaptureAdapter` + 完整服务栈
    - `ScrollScreenshotService`
    - `ScrollScreenshotCaptureService`
    - `ScrollScreenshotImageService`
  - 原代码: 
    - `scroll_screenshot_init`
    - `scroll_screenshot_capture`
    - `scroll_screenshot_handle_image`
    - `scroll_screenshot_get_image_data`

#### 内部处理（移除）
- ❌ `scroll_screenshot_get_size` - 响应中包含
- ❌ `scroll_screenshot_save_to_file` - 前端处理
- ❌ `scroll_screenshot_save_to_clipboard` - 前端处理
- ❌ `scroll_screenshot_clear` - 自动清理

### 3. Window Management 功能组

#### 保留并重命名
- ✅ **`create_window`** ← 合并多个创建命令
  - Application: `WindowManagementUseCase`
  - Infrastructure: `WindowManagementAdapter`
  - 原代码:
    - `create_draw_window`
    - `create_fixed_content_window`
    - `create_full_screen_draw_window`
    - `create_video_record_window`

- ✅ **`drag_window`** ← 重命名 `start_free_drag`
  - Infrastructure: 复用 `FreeDragWindowService`
  - 原代码: `tauri-commands/core::start_free_drag`

- ✅ **`resize_window`** ← 重命名 `start_resize_window`
  - Infrastructure: 复用 `ResizeWindowService`
  - 原代码: `tauri-commands/core::start_resize_window`

- ✅ **`pin_window` / `unpin_window`** ← 拆分 `switch_always_on_top`
  - 更明确的语义
  - 原代码: `screenshot::switch_always_on_top`

- ✅ **`close_window`** ← 合并关闭命令
  - 原代码:
    - `close_full_screen_draw_window`
    - `close_video_record_window`

#### 前端处理（移除）
- ❌ `close_window_after_delay` - 前端 setTimeout
- ❌ `has_video_record_window` - 前端状态
- ❌ `has_focused_full_screen_window` - 前端状态

### 4. Monitor 功能组

#### 保留并简化
- ✅ **`get_monitors`** ← 重命名 + 简化
  - Application: `GetMonitorsUseCase`
  - Infrastructure: `ScreenCaptureAdapter::get_monitors_macos`
  - 原代码: `get_monitors_bounding_box` (简化为纯信息获取)

- ✅ **`get_current_monitor`** ← 重命名
  - Application: `GetCurrentMonitorUseCase`
  - 原代码: `get_current_monitor_info`

- ✅ **`get_mouse_position`** ← 移动模块
  - 原代码: `screenshot::get_mouse_position`
  - 变更: 从 screenshot 模块迁移至 monitor 模块（更合理）

### 5. Hotkey 功能组（新增）

#### 新增命令
- ✨ **`listen_key_start` / `listen_key_stop`**
  - Application: 直接调用 Adapter
  - Infrastructure: `HotkeyListenerAdapter` + `ListenKeyService`
  - 原代码: `listen_key_service` (封装为命令)

- ✨ **`listen_mouse_start` / `listen_mouse_stop`**
  - Infrastructure: `HotkeyListenerAdapter` + `ListenMouseService`
  - 原代码: `listen_mouse_service` (封装为命令)

### 6. Clipboard 功能组

#### 保留并扩展
- ✅ **`read_clipboard_image`** ← 重命名
  - 原代码: `read_image_from_clipboard`

#### 新增命令
- ✨ **`write_clipboard_image`** - 新增写入功能
- ✨ **`read_clipboard`** - 通用读取（文本/图像/文件）
- ✨ **`write_clipboard`** - 通用写入
- ✨ **`clear_clipboard`** - 清空剪贴板
- ✨ **`get_clipboard_types`** - 获取可用类型

#### Infrastructure 实现
- 使用 `arboard 3.6.1` (跨平台剪贴板库)
- 替换原 macOS 特定实现
- 原代码: `core::read_image_from_clipboard` (仅读取图像)

### 7. UI Automation 功能组

#### 保留框架
- ✅ **`get_element_from_position`** ← 保留
  - Application: 直接调用 Adapter
  - Infrastructure: `UIAutomationAdapter` + `UIElements`
  - 原代码: `screenshot::get_element_from_position`
  - 状态: 框架完整，但返回空列表（需 Accessibility API 完整集成）

- ✅ **`init_ui_elements`** ← 简化
  - 原代码: `init_ui_elements_cache`

#### 移除
- ❌ `get_window_elements` - 功能重复

### 8. Page Management 功能组（新增）

#### 新增命令
- ✨ **`add_page`**
  - Infrastructure: `PageManagementAdapter` + `HotLoadPageService`
  - 原代码: `hot_load_page_service` (封装为命令)

- ✨ **`remove_page`**
  - 新增: 从页面池移除（原服务无此方法）

---

## 🔧 Infrastructure Layer 详解

### Adapters（适配器）

| Adapter | 实现平台 | 复用代码来源 | 说明 |
|---------|---------|-------------|------|
| `ScreenCaptureAdapter` | macOS | `xcap` + `app-os::utils` | 屏幕截图 |
| `ImageProcessingAdapter` | 通用 | `app-utils` (HDR, overlay, encode) | 图像处理 |
| `FileSystemAdapter` | 通用 | 新实现 | 文件存储 |
| `MemoryCacheAdapter` | 通用 | 新实现 | 内存缓存 |
| `ScrollCaptureAdapter` | macOS | `app-scroll-screenshot-service` (完整迁移) | 滚动截图 |
| `WindowManagementAdapter` | Tauri | 新实现（框架） | 窗口管理 |
| `UIAutomationAdapter` | macOS | `app-os::ui_automation` (框架) | UI 自动化 |
| `HotkeyListenerAdapter` | macOS | `listen_key_service` + `listen_mouse_service` | 热键监听 |
| `ClipboardAdapter` | 跨平台 | `arboard` (替换原实现) | 剪贴板 |
| `PageManagementAdapter` | Tauri | `hot_load_page_service` | 页面池 |

### Services（内部服务）

| Service | 来源 | 状态 |
|---------|------|------|
| `DeviceEventHandlerService` | `app-services` | ✅ 完整迁移 |
| `ListenKeyService` | `app-services` | ✅ 完整迁移 |
| `ListenMouseService` | `app-services` | ✅ 完整迁移 |
| `EnigoManager` | `app-shared` | ✅ 完整迁移 |
| `ScrollScreenshotService` | `app-scroll-screenshot-service` | ✅ 完整迁移 |
| `ScrollScreenshotCaptureService` | `app-scroll-screenshot-service` | ✅ 完整迁移 |
| `ScrollScreenshotImageService` | `app-scroll-screenshot-service` | ✅ 完整迁移 |
| `HotLoadPageService` | `app-services` | ✅ 完整迁移 |

### Platform (平台特定代码)

| 平台模块 | 来源 | 状态 |
|---------|------|------|
| `macos/ui_automation.rs` | `app-os` | ✅ 框架迁移 |
| `macos/notification.rs` | `app-os` | ✅ 完整迁移 |
| `macos/shared.rs` | `app-os` | ✅ 完整迁移 |
| `macos/utils.rs` | `app-os` | ✅ 完整迁移 |

---

## ❌ 完全移除的功能

### 1. File Management (8个命令)
- **原因**: Tauri 提供内置文件系统 API，前端可直接使用
- **移除命令**: `save_file`, `write_file`, `copy_file`, `remove_file`, `create_dir`, `remove_dir`, `get_app_config_dir`, `create_local_config_dir`

### 2. OCR Service (4个命令)
- **原因**: OCR 功能独立为插件系统，不属于核心功能
- **移除命令**: `ocr_detect`, `ocr_detect_with_shared_buffer`, `ocr_init`, `ocr_release`
- **移除代码**: `paddle-ocr-rs` 依赖

### 3. Video Record (3个命令)
- **原因**: 视频录制功能暂时移除，未来可作为插件
- **移除命令**: `create_video_record_window`, `close_video_record_window`, `has_video_record_window`

### 4. Misc Utilities (20+个命令)
- **exit_app** - Tauri 内置
- **set_enable_proxy** - 网络配置应由系统或前端处理
- **scroll_through / auto_scroll_through / click_through** - 前端实现
- **get_selected_text** - 前端实现
- **send_new_version_notification** - 更新逻辑移至前端
- **auto_start_enable / auto_start_disable** - 使用 Tauri 插件
- **restart_with_admin** - 系统级操作，减少安全风险

---

## 📊 代码复用统计

### 完整迁移的模块
- ✅ `app-scroll-screenshot-service` (100% 迁移)
- ✅ `listen_key_service` / `listen_mouse_service` (100% 迁移)
- ✅ `hot_load_page_service` (100% 迁移)
- ✅ `app-os::notification` (100% 迁移)

### 部分复用的模块
- ⚙️ `app-utils` (50% 复用: HDR, overlay, encode；移除: file操作)
- ⚙️ `app-os::ui_automation` (框架保留，完整实现待补充)
- ⚙️ `app-shared` (30% 复用: EnigoManager, 类型定义；移除: ElementRect)

### 完全替换的模块
- 🔄 剪贴板实现: 原 macOS NSPasteboard → `arboard` (跨平台)
- 🔄 `ElementRect` → `Rectangle` (增加方法)
- 🔄 `MonitorList` → 直接使用 `xcap::Monitor` + 辅助方法

---

## 🎯 架构优势总结

### 1. 可维护性 ⬆️
- **DDD 分层**: 清晰的职责划分
- **依赖注入**: 易于测试和替换实现
- **代码组织**: 按领域而非技术分组

### 2. 可扩展性 ⬆️
- **Port/Adapter**: 添加新平台无需修改核心
- **Use Case**: 灵活组合业务流程
- **模块化**: Crate 独立编译和测试

### 3. 可测试性 ⬆️
- **Mock 友好**: 所有 Port 都可 Mock
- **单元测试**: 每层独立测试
- **集成测试**: Use Case 层测试业务流程

### 4. 代码质量 ⬆️
- **类型安全**: 强类型 ID（`MonitorId`, `WindowId` 等）
- **错误处理**: 分层错误类型
- **命名一致**: 统一的命名风格

---

**文档版本**: v1.0  
**最后更新**: 2024-12-14  
**架构负责人**: Aumate Team



