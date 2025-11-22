# robot-rs

高性能的桌面自动化库，使用 Rust 编写并通过 napi 绑定提供给 Node.js 使用。

## 特性

- 🚀 **高性能**: 使用 Rust 编写，性能远超 Node.js 原生实现
- 🎯 **API 兼容**: 参考 robotjs 的 API 设计，易于迁移
- 🔒 **内存安全**: Rust 的类型系统保证内存安全
- 🌍 **跨平台**: 支持 Windows、macOS 和 Linux
- 📦 **零依赖**: Node.js 端无需额外依赖
- 🧪 **完整测试**: 包含单元测试和集成测试

## 安装

```bash
npm install robot-rs
```

## 构建

```bash
cd packages/robot-rs
npm run build
```

## API 文档

### 鼠标操作

```typescript
import { Mouse } from 'robot-rs';

const mouse = new Mouse();

// 移动鼠标到指定坐标
mouse.moveMouse(100, 200);

// 平滑移动鼠标
mouse.moveMouseSmooth(300, 400);
mouse.moveMouseSmoothWithSpeed(500, 600, 5.0); // 自定义速度

// 点击鼠标
mouse.mouseClick('left');           // 左键单击
mouse.mouseClick('right', true);    // 右键双击
mouse.mouseClick('middle');         // 中键单击

// 获取鼠标位置
const pos = mouse.getMousePos();
console.log(`Mouse at: ${pos.x}, ${pos.y}`);

// 按下/释放鼠标按钮
mouse.mouseToggle('down', 'left');  // 按下左键
mouse.mouseToggle('up', 'left');    // 释放左键

// 拖拽鼠标
mouse.dragMouse(500, 600);

// 滚动鼠标
mouse.scrollMouse(0, 3);  // 向下滚动 3 个单位
mouse.scrollMouse(2, 0);  // 向右滚动 2 个单位

// 设置鼠标操作延迟（毫秒）
mouse.setMouseDelay(50);
```

### 键盘操作

```typescript
import { Keyboard } from 'robot-rs';

const keyboard = new Keyboard();

// 按键（按下并释放）
keyboard.keyTap('a');
keyboard.keyTap('enter');
keyboard.keyTap('c', ['control']);        // Ctrl+C
keyboard.keyTap('v', ['control', 'shift']); // Ctrl+Shift+V

// 按下/释放按键
keyboard.keyToggle('a', 'down');  // 按下 'a'
keyboard.keyToggle('a', 'up');    // 释放 'a'

// 输入文本
keyboard.typeString('Hello, World!');

// 延迟输入文本（字符每分钟）
keyboard.typeStringDelayed('Hello', 300); // 300 CPM

// 设置键盘操作延迟（毫秒）
keyboard.setKeyboardDelay(10);
```

### 屏幕操作

```typescript
import { captureScreen, captureScreenRegion, getScreenSize, getPixelColor } from 'robot-rs';
import fs from 'fs';

// 截取整个屏幕
const screen = await captureScreen();
fs.writeFileSync('screenshot.png', screen.image);
console.log(`Captured: ${screen.width}x${screen.height}`);

// 截取屏幕区域
const region = await captureScreenRegion(100, 100, 800, 600);
fs.writeFileSync('region.png', region.image);

// 获取屏幕尺寸
const size = getScreenSize();
console.log(`Screen size: ${size.width}x${size.height}`);

// 获取指定坐标的像素颜色
const color = await getPixelColor(100, 200);
console.log(`Pixel color: RGB(${color.r}, ${color.g}, ${color.b})`);
```

## 完整示例

```typescript
import { Mouse, Keyboard, captureScreen } from 'robot-rs';
import fs from 'fs';

async function automationExample() {
    const mouse = new Mouse();
    const keyboard = new Keyboard();

    // 移动鼠标并点击
    mouse.moveMouseSmooth(500, 300);
    mouse.mouseClick('left');

    // 输入文本
    keyboard.typeString('Hello from robot-rs!');
    keyboard.keyTap('enter');

    // 截屏
    const screenshot = await captureScreen();
    fs.writeFileSync('automation.png', screenshot.image);

    console.log('Automation completed!');
}

automationExample();
```

## 支持的按键

### 修饰键
- `control` / `ctrl` - Control 键
- `shift` - Shift 键
- `alt` - Alt 键
- `command` / `cmd` / `meta` - Command/Meta 键

### 功能键
- `f1` - `f12` - F1 到 F12

### 特殊键
- `enter` / `return` - 回车键
- `escape` / `esc` - ESC 键
- `backspace` - 退格键
- `tab` - Tab 键
- `space` - 空格键
- `delete` / `del` - Delete 键
- `up` / `down` / `left` / `right` - 方向键
- `home` / `end` - Home/End 键
- `pageup` / `page_down` - Page Up/Down 键

### 鼠标按钮
- `left` - 左键
- `right` - 右键
- `middle` - 中键

## 测试

运行测试：

```bash
# Rust 单元测试
cargo test

# 构建并测试 Node.js 绑定
npm run build
npm test
```

## 与 robotjs 的对比

| 特性 | robotjs | robot-rs |
|------|---------|----------|
| 性能 | 中等（C++ 绑定） | ⚡ 极高（Rust 原生） |
| 维护状态 | ❌ 已停止维护 | ✅ 活跃维护 |
| 内存安全 | ⚠️ C++ | ✅ Rust |
| API 设计 | ✅ 简洁 | ✅ 兼容 |
| 跨平台 | ✅ | ✅ |
| 类型安全 | ⚠️ 运行时检查 | ✅ 编译期保证 |
| 测试覆盖 | ⚠️ 有限 | ✅ 完整 |

## 系统要求

### macOS
- macOS 10.13+ 
- 需要屏幕录制权限（系统偏好设置 > 安全性与隐私 > 屏幕录制）

### Windows
- Windows 10+
- 无需额外配置

### Linux
- X11 或 Wayland
- 可能需要安装系统依赖：
  ```bash
  # Ubuntu/Debian
  sudo apt-get install libxcb1-dev libxrandr-dev libdbus-1-dev
  
  # Fedora
  sudo dnf install libxcb-devel libXrandr-devel dbus-devel
  ```

## 许可证

MIT

## 贡献

欢迎提交 Issue 和 Pull Request！

## 相关项目

- [robotjs](https://github.com/octalmage/robotjs) - 原始 Node.js 自动化库
- [enigo](https://github.com/enigo-rs/enigo) - Rust 键盘鼠标控制库
- [xcap](https://github.com/nashaofu/xcap) - Rust 屏幕捕获库
