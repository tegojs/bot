# 截图窗口问题修复

## 🐛 修复的问题

### 1. macOS 全屏模式切换到新 Space
**问题**: 使用 `fullscreen: true` 导致 macOS 切换到独立的全屏空间  
**解决**: 
- ❌ 移除配置文件中的 `fullscreen: true`
- ✅ 使用动态窗口创建，通过 `position` 和 `size` 覆盖屏幕
- ✅ 使用 `LogicalPosition` 和 `LogicalSize` (macOS 推荐)

### 2. ESC 键无法关闭窗口
**问题**: 只是隐藏窗口，没有真正关闭  
**解决**:
- ✅ 实现 `close_draw_window` 命令，使用 `destroy()` 销毁窗口
- ✅ ESC 键调用 `invoke("close_draw_window")` 真正关闭窗口

### 3. F2 无法重新打开窗口
**问题**: 窗口关闭后，F2 只尝试显示不存在的窗口  
**解决**:
- ✅ 实现 `create_draw_window` 命令，自动检测窗口是否存在
- ✅ 窗口不存在时自动创建新窗口
- ✅ 窗口存在时直接显示

### 4. 窗口无法覆盖 macOS 菜单栏和 Dock
**问题**: 窗口显示在菜单栏和 Dock 下方  
**解决**:
- ✅ 使用 macOS private API 设置窗口层级
- ✅ 设置 `NSWindowLevel` 为 `NSMainMenuWindowLevel + 1`
- ✅ 设置 `NSWindowCollectionBehavior`:
  - `CanJoinAllSpaces` - 出现在所有空间
  - `Stationary` - 静止窗口
  - `FullScreenAuxiliary` - 全屏辅助（覆盖菜单栏和 Dock）
- ✅ 添加 `cocoa` 依赖用于 Objective-C 调用

## 📝 修改的文件

### 后端 (Rust)

1. **`src-tauri/tauri.conf.json`**
   - 移除了 `draw` 窗口的预定义配置

2. **`src-tauri/Cargo.toml`**
   - 添加 `cocoa = "0.25"` 依赖 (仅 macOS)

3. **`src-tauri/src/commands/draw.rs`** (新建)
   - `create_draw_window()` - 创建或显示截图窗口
   - `close_draw_window()` - 关闭截图窗口
   - `set_window_above_menubar()` - 设置窗口覆盖菜单栏 (macOS)

4. **`src-tauri/src/commands/mod.rs`**
   - 添加 `draw` 模块导出

5. **`src-tauri/src/lib.rs`**
   - 注册 `create_draw_window` 和 `close_draw_window` 命令
   - 修改 F2 快捷键处理，调用 `create_draw_window()`

### 前端 (TypeScript/React)

1. **`src/pages/draw/page.tsx`**
   - 导入 `invoke` 用于调用后端命令
   - `finishCapture()` - 改为调用 `close_draw_window()`
   - `showWindow()` - 简化逻辑，窗口已由后端设置
   - 移除初始化时的 `hide()` 调用

## 🔧 技术细节

### 窗口创建逻辑

```rust
// 获取主显示器信息
let monitor = app.primary_monitor()?;
let position = monitor.position();
let size = monitor.size();
let scale_factor = monitor.scale_factor();

// 转换为逻辑像素
let logical_x = position.x as f64 / scale_factor;
let logical_y = position.y as f64 / scale_factor;
let logical_width = size.width as f64 / scale_factor;
let logical_height = size.height as f64 / scale_factor;

// 创建覆盖整个屏幕的窗口
let window = WebviewWindowBuilder::new(&app, "draw", WebviewUrl::App("pages/draw.html".into()))
    .position(logical_x, logical_y)
    .inner_size(logical_width, logical_height)
    .decorations(false)
    .transparent(true)
    .always_on_top(true)
    // ... 其他配置
    .build()?;

// macOS: 设置窗口层级覆盖菜单栏和 Dock
#[cfg(target_os = "macos")]
set_window_above_menubar(&window)?;
```

### macOS 窗口层级设置

```rust
fn set_window_above_menubar(window: &tauri::WebviewWindow) -> Result<(), String> {
    use cocoa::appkit::{NSMainMenuWindowLevel, NSWindow, NSWindowCollectionBehavior};
    use cocoa::base::id;
    
    let ns_window_ptr = window.ns_window()? as usize;
    
    window.run_on_main_thread(move || unsafe {
        let ns_window = ns_window_ptr as id;
        
        // 设置窗口层级高于菜单栏
        ns_window.setLevel_((NSMainMenuWindowLevel + 1) as i64);
        
        // 设置窗口行为
        let behavior = NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces
            | NSWindowCollectionBehavior::NSWindowCollectionBehaviorStationary
            | NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary;
        
        ns_window.setCollectionBehavior_(behavior);
    })?;
    
    Ok(())
}
```

### F2 快捷键处理

```rust
// F2 快捷键按下时
if hotkey == &f2_shortcut {
    let app_handle_clone = app_handle.clone();
    tauri::async_runtime::spawn(async move {
        // 调用命令创建或显示窗口
        if let Err(e) = commands::create_draw_window(app_handle_clone).await {
            log::error!("Failed to create draw window: {}", e);
        }
    });
}
```

### ESC 键关闭窗口

```typescript
// 前端 ESC 键处理
const finishCapture = useCallback(async () => {
  // 清理状态
  resetCaptureStep();
  resetDrawState();
  resetScreenshotType();

  // 调用后端命令关闭窗口
  await invoke("close_draw_window");
}, []);
```

## ✅ 测试步骤

1. **启动应用**
   ```bash
   cd aumate-app
   pnpm tauri dev
   ```

2. **测试 F2 打开窗口**
   - 按 `F2` → 窗口应该覆盖整个屏幕（不切换 Space）
   - 窗口应该半透明背景，显示提示信息
   - ✅ **窗口应该覆盖菜单栏和 Dock**

3. **测试 ESC 关闭窗口**
   - 在截图窗口中按 `ESC`
   - 窗口应该完全关闭（不仅仅是隐藏）

4. **测试重复打开**
   - 关闭窗口后再次按 `F2`
   - 窗口应该能够重新创建并显示
   - 窗口层级应该正确（覆盖菜单栏）

5. **测试多次快速按 F2**
   - 快速多次按 `F2`
   - 不应该创建多个窗口（有检测机制）

6. **测试 macOS 特性 (仅 macOS)**
   - 窗口应该出现在所有 Space 中
   - 菜单栏和 Dock 应该被窗口覆盖
   - 窗口应该保持在最顶层

## 🎯 预期行为

- ✅ 按 F2 → 窗口覆盖整个当前屏幕（不切换 Space）
- ✅ 窗口覆盖 macOS 菜单栏和 Dock
- ✅ 按 ESC → 窗口完全关闭并销毁
- ✅ 再按 F2 → 窗口重新创建并显示
- ✅ 窗口显示时自动获得焦点
- ✅ 窗口状态栏显示提示信息
- ✅ 窗口在所有 Space 中可见 (macOS)

## 📚 参考

- 原项目窗口创建: `snow-shot/src-tauri/src-crates/tauri-commands/core/src/lib.rs:252`
- Tauri 窗口 API: https://v2.tauri.app/reference/javascript/api/namespacewindow/
- LogicalPosition/Size: macOS 推荐使用逻辑像素而非物理像素

---

**修复日期**: 2025-12-13  
**状态**: ✅ 已修复，待测试
