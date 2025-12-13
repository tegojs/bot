# Tauri Commands 变更记录

**版本**: v0.3.0  
**架构**: DDD (Domain-Driven Design)  
**变更日期**: 2024-12-14

---

## 📊 总体变化

| 项目 | 旧版本 | 新版本 | 变化 |
|------|--------|--------|------|
| **总 Commands 数** | 98 个 | 27 个 | -71 个 (72% 精简) |
| **架构模式** | 分散式 (12 模块) | DDD 分层 (9 模块) | 重构 |
| **代码组织** | 平铺结构 | 领域驱动 | 优化 |

---

## 🔄 命令映射关系

### 1. Screenshot Commands (3个)

| 旧命令 | 新命令 | 变更类型 | 说明 |
|--------|--------|---------|------|
| `capture_current_monitor` | `capture_current_monitor` | ✅ 保留 | 捕获当前监视器 |
| `capture_all_monitors` | `capture_monitor` | ⚙️ 合并 | 通过 monitor_id 参数化 |
| `capture_full_screen` | `capture_current_monitor` | ⚙️ 合并 | 全屏等同于当前监视器 |
| `capture_focused_window` | `capture_region` | ⚙️ 合并 | 通过区域参数实现 |
| `init_ui_elements` | - | ❌ 移除 | 内部自动处理 |
| `init_ui_elements_cache` | - | ❌ 移除 | 内部自动处理 |
| `get_window_elements` | `get_element_from_position` | ⚙️ 重命名 | UI 自动化模块 |
| `switch_always_on_top` | `pin_window` / `unpin_window` | ⚙️ 拆分 | 语义更清晰 |

### 2. Scroll Screenshot Commands (1个)

| 旧命令 | 新命令 | 变更类型 | 说明 |
|--------|--------|---------|------|
| `scroll_screenshot_init` | `start_scroll_capture` | ⚙️ 重命名 | 统一命名风格 |
| `scroll_screenshot_capture` | `start_scroll_capture` | ⚙️ 合并 | 一次调用完成 |
| `scroll_screenshot_handle_image` | - | ❌ 移除 | 内部处理 |
| `scroll_screenshot_get_size` | - | ❌ 移除 | 响应中包含 |
| `scroll_screenshot_save_to_file` | - | ❌ 移除 | 前端处理 |
| `scroll_screenshot_save_to_clipboard` | - | ❌ 移除 | 前端处理 |
| `scroll_screenshot_clear` | - | ❌ 移除 | 自动清理 |
| `scroll_screenshot_get_image_data` | - | ❌ 移除 | 响应中包含 |

### 3. Window Management Commands (6个)

| 旧命令 | 新命令 | 变更类型 | 说明 |
|--------|--------|---------|------|
| `create_draw_window` | `create_window` | ⚙️ 重命名 | 统一窗口管理 |
| `create_fixed_content_window` | `create_window` | ⚙️ 合并 | 统一创建接口 |
| `create_full_screen_draw_window` | `create_window` | ⚙️ 合并 | 统一创建接口 |
| `create_video_record_window` | `create_window` | ⚙️ 合并 | 统一创建接口 |
| `close_full_screen_draw_window` | `close_window` | ⚙️ 合并 | 统一关闭接口 |
| `close_video_record_window` | `close_window` | ⚙️ 合并 | 统一关闭接口 |
| `start_free_drag` | `drag_window` | ⚙️ 重命名 | 更清晰语义 |
| `start_resize_window` | `resize_window` | ⚙️ 重命名 | 更清晰语义 |
| `set_current_window_always_on_top` | `pin_window` | ⚙️ 重命名 | 更简洁命名 |
| - | `unpin_window` | ✨ 新增 | 取消固定 |
| `close_window_after_delay` | - | ❌ 移除 | 前端实现 |
| `set_window_rect` | - | ❌ 移除 | 使用 resize + position |
| `has_video_record_window` | - | ❌ 移除 | 前端状态管理 |
| `has_focused_full_screen_window` | - | ❌ 移除 | 前端状态管理 |

### 4. Monitor Commands (3个)

| 旧命令 | 新命令 | 变更类型 | 说明 |
|--------|--------|---------|------|
| `get_current_monitor_info` | `get_current_monitor` | ⚙️ 重命名 | 简化命名 |
| `get_monitors_bounding_box` | `get_monitors` | ⚙️ 合并 | 统一监视器查询 |
| `get_mouse_position` | `get_mouse_position` | ✅ 保留 | 迁移至 monitor 模块 |

### 5. Hotkey Commands (4个)

| 旧命令 | 新命令 | 变更类型 | 说明 |
|--------|--------|---------|------|
| - | `listen_key_start` | ✨ 新增 | 开始键盘监听 |
| - | `listen_key_stop` | ✨ 新增 | 停止键盘监听 |
| - | `listen_mouse_start` | ✨ 新增 | 开始鼠标监听 |
| - | `listen_mouse_stop` | ✨ 新增 | 停止鼠标监听 |

### 6. UI Automation Commands (2个)

| 旧命令 | 新命令 | 变更类型 | 说明 |
|--------|--------|---------|------|
| `get_element_from_position` | `get_element_from_position` | ✅ 保留 | UI 元素获取 |
| `init_ui_elements_cache` | `init_ui_elements` | ⚙️ 简化 | 初始化 UI 缓存 |

### 7. Clipboard Commands (6个)

| 旧命令 | 新命令 | 变更类型 | 说明 |
|--------|--------|---------|------|
| `read_image_from_clipboard` | `read_clipboard_image` | ⚙️ 重命名 | 统一命名风格 |
| - | `write_clipboard_image` | ✨ 新增 | 写入图像 |
| - | `read_clipboard` | ✨ 新增 | 读取剪贴板（通用） |
| - | `write_clipboard` | ✨ 新增 | 写入剪贴板（通用） |
| - | `clear_clipboard` | ✨ 新增 | 清空剪贴板 |
| - | `get_clipboard_types` | ✨ 新增 | 获取可用类型 |

### 8. Page Management Commands (2个)

| 旧命令 | 新命令 | 变更类型 | 说明 |
|--------|--------|---------|------|
| - | `add_page` | ✨ 新增 | 添加页面到热加载池 |
| - | `remove_page` | ✨ 新增 | 从热加载池移除页面 |

---

## ❌ 完全移除的模块

### File Management (8个命令) - 全部移除
- 原因：文件操作由前端或 Tauri 内置 API 处理
- 移除命令：`save_file`, `write_file`, `copy_file`, `remove_file`, `create_dir`, `remove_dir`, `get_app_config_dir`, `create_local_config_dir`

### OCR Service (4个命令) - 全部移除
- 原因：OCR 功能独立为插件
- 移除命令：`ocr_detect`, `ocr_detect_with_shared_buffer`, `ocr_init`, `ocr_release`

### Video Record (3个命令) - 全部移除
- 原因：视频录制功能移除
- 移除命令：`create_video_record_window`, `close_video_record_window`, `has_video_record_window`

### Misc Utilities (20+个命令) - 大部分移除
- 原因：功能重复、前端实现、或与核心业务无关
- 移除命令：`exit_app`, `set_enable_proxy`, `scroll_through`, `auto_scroll_through`, `click_through`, `get_selected_text`, `send_new_version_notification`, `auto_start_enable`, `auto_start_disable`, `restart_with_admin` 等

---

## ✨ 新增功能

### 1. 增强的剪贴板管理
- 统一的剪贴板 API，支持文本/图像/文件
- 类型检测和清空功能

### 2. 完整的窗口管理
- 语义化的置顶操作 (`pin`/`unpin`)
- 统一的窗口创建和关闭接口

### 3. 热键管理
- 独立的键盘/鼠标监听控制
- 更灵活的事件处理

### 4. 页面池管理
- 热加载页面池的显式管理
- 支持动态添加和移除

---

## 📈 架构优势

| 维度 | 旧架构 | 新架构 | 改进 |
|------|--------|--------|------|
| **命令数量** | 98 个 | 27 个 | 精简 72% |
| **命名一致性** | 低（多种风格） | 高（统一风格） | ⬆️ |
| **参数复用** | 低（专用命令） | 高（参数化） | ⬆️ |
| **可维护性** | 中 | 高（DDD 分层） | ⬆️ |
| **可测试性** | 低 | 高（依赖注入） | ⬆️ |
| **代码复用** | 低 | 高（共享 Domain） | ⬆️ |

---

## 🎯 设计原则

1. **命令合并**: 将功能相似的命令合并，通过参数区分
2. **内部处理**: 将初始化、清理等操作移至内部自动处理
3. **前端分离**: 将 UI 逻辑、状态管理移至前端
4. **统一命名**: 使用一致的动词+名词命名风格
5. **显式语义**: 使用明确的动词 (`pin`/`unpin` 而非 `set_always_on_top`)

---

## 📝 迁移指南

### 旧命令迁移示例

```typescript
// 旧: 捕获全屏
await invoke('capture_full_screen', { ... })

// 新: 捕获当前监视器（等效）
await invoke('capture_current_monitor', { ... })
```

```typescript
// 旧: 窗口置顶
await invoke('set_current_window_always_on_top', { alwaysOnTop: true })

// 新: 固定窗口
await invoke('pin_window', { windowId: 'main' })
```

```typescript
// 旧: 滚动截图（多步骤）
await invoke('scroll_screenshot_init', { ... })
await invoke('scroll_screenshot_capture', { ... })
const data = await invoke('scroll_screenshot_get_image_data', { ... })

// 新: 滚动截图（一次调用）
const result = await invoke('start_scroll_capture', { ... })
// result 包含所有数据
```

---

**变更完成日期**: 2024-12-14  
**架构版本**: v0.3.0-DDD



