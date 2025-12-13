# Aumate 项目文档

**项目版本**: v0.3.0  
**架构模式**: Domain-Driven Design (DDD)  
**语言**: Rust (Edition 2024)  
**框架**: Tauri 2.x

---

## 📚 文档导航

### 核心文档

1. **[架构说明](./ARCHITECTURE.md)** 
   - DDD 分层架构详解
   - 模块职责和依赖关系
   - 代码复用和迁移映射
   - Infrastructure Layer 实现细节

2. **[命令变更记录](./COMMANDS_CHANGELOG.md)**
   - 旧版本 → 新版本命令映射
   - 命令合并、重命名、移除说明
   - 迁移指南和示例代码

### 历史记录

- **[Phase 1 & 2 完成报告](./PHASE1_2_COMPLETION_REPORT.md)** - 核心类型系统和剪贴板API实现
- **[依赖重构报告](./DEPENDENCY_REFACTOR_REPORT.md)** - 依赖升级和清理
- **[重命名报告](./RENAME_TO_AUMATE_REPORT.md)** - snow-shot → aumate 重命名

---

## 🏗️ 项目结构

```
crates/
├── core/
│   ├── shared/       # 共享类型和工具
│   ├── domain/       # 领域模型
│   └── traits/       # Port 接口定义
├── application/      # Use Cases (业务用例)
├── infrastructure/   # Adapters & Services (平台实现)
└── docs/             # 📄 本文档目录

aumate-app/src-tauri/  # API Layer (Tauri Commands)
├── src/
│   ├── commands/     # Tauri Commands
│   ├── state.rs      # AppState
│   └── setup.rs      # 依赖注入
```

---

## 🎯 快速开始

### 查看架构设计
```bash
# 阅读架构说明
cat docs/ARCHITECTURE.md
```

### 查看命令变更
```bash
# 阅读命令变更记录
cat docs/COMMANDS_CHANGELOG.md
```

### 编译项目
```bash
cd src-tauri/crates
cargo build --workspace --release
```

### 运行测试
```bash
cargo test --workspace
```

---

## 📊 项目统计

| 指标 | 数值 |
|------|------|
| **Tauri Commands** | 28 个 |
| **Crates** | 5 个 |
| **Port Traits** | 10 个 |
| **Adapters** | 11 个 |
| **Use Cases** | 17+ 个 |
| **代码精简** | 71% (98 → 28 commands) |

---

## 🔄 架构原则

1. **依赖倒置**: Domain 定义接口，Infrastructure 实现
2. **单一职责**: 每层职责明确
3. **开闭原则**: 通过 Port/Adapter 扩展
4. **命令合并**: 减少冗余，参数化功能
5. **显式语义**: 清晰的命名和接口设计

---

## 📦 核心组件

### API Layer (28 Commands)
- **Screenshot** (3): `capture_current_monitor`, `capture_monitor`, `capture_region`
- **Scroll** (1): `start_scroll_capture`
- **Window** (7): `create_window`, `drag_window`, `resize_window`, `pin_window`, `unpin_window`, `close_window`, `get_window_elements`
- **Monitor** (2): `get_monitors`, `get_current_monitor`
- **Hotkey** (4): `listen_key_start/stop`, `listen_mouse_start/stop`
- **UI** (2): `get_element_from_position`, `init_ui_elements`
- **Clipboard** (6): `read/write_clipboard`, `read/write_clipboard_image`, `clear_clipboard`, `get_clipboard_types`
- **Page** (2): `add_page`, `remove_page`

### Application Layer (Use Cases)
- Screenshot Use Cases
- Scroll Screenshot Use Case
- Window Management Use Case
- Monitor Use Cases
- Clipboard Use Cases

### Infrastructure Layer (Adapters)
- `ScreenCaptureAdapter` (macOS)
- `ImageProcessingAdapter` (通用)
- `ScrollCaptureAdapter` (macOS)
- `WindowManagementAdapter` (Tauri)
- `WindowListAdapter` (macOS/Windows/Linux)
- `ClipboardAdapter` (arboard)
- `HotkeyListenerAdapter` (macOS)
- `PageManagementAdapter` (Tauri)
- `UIAutomationAdapter` (macOS 框架)

---

## 🚀 主要改进

### 架构层面
- ✅ DDD 分层架构
- ✅ 依赖注入和接口隔离
- ✅ 模块化 Crate 组织
- ✅ 统一的错误处理

### 代码层面
- ✅ 命令数量减少 72%
- ✅ 统一命名风格
- ✅ 完整的类型系统
- ✅ 增强的测试覆盖

### 功能层面
- ✅ 保留核心功能
- ✅ 增强剪贴板管理
- ✅ 统一窗口操作
- ✅ 新增热键管理

---

## 📝 文档维护

- **架构变更**: 更新 `ARCHITECTURE.md`
- **命令变更**: 更新 `COMMANDS_CHANGELOG.md`
- **新增功能**: 在相应文档中补充说明
- **代码示例**: 保持与实际代码同步

---

## 🔗 相关链接

- **Tauri 文档**: https://tauri.app/
- **Rust DDD**: https://github.com/varovainen/good-web-game-by-using-ddd
- **arboard**: https://github.com/1Password/arboard
- **xcap**: https://github.com/nashaofu/xcap

---

**最后更新**: 2024-12-14  
**维护者**: Aumate Team
