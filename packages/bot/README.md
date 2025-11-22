# @tego/bot

高性能的桌面自动化库，使用 Rust 编写并通过 N-API 绑定提供给 Node.js 使用。

## 特性

- 🚀 **高性能**: 使用 Rust 编写，性能远超 Node.js 原生实现
- 🎯 **API 兼容**: 参考 robotjs 的 API 设计，易于迁移
- 🔒 **内存安全**: Rust 的类型系统保证内存安全
- 🌍 **跨平台**: 支持 Windows、macOS 和 Linux
- 📦 **零依赖**: Node.js 端无需额外依赖
- 🧪 **完整测试**: 包含单元测试和集成测试

## 安装

```bash
npm install @tego/bot
# 或
pnpm add @tego/bot
# 或
yarn add @tego/bot
```

## 构建

```bash
cd packages/bot
npm run build
```

## API 文档

### 鼠标操作

```typescript
import { Mouse } from '@tego/bot';

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
mouse.mouseClick();                 // 默认左键单击

// 获取鼠标位置
const pos = mouse.getMousePos();
console.log(`Mouse at: ${pos.x}, ${pos.y}`);

// 按下/释放鼠标按钮
mouse.mouseToggle('down', 'left');  // 按下左键
mouse.mouseToggle('up', 'left');    // 释放左键
mouse.mouseToggle('down');          // 默认按下左键

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
// 方式 1: 使用类实例
import { Keyboard } from '@tego/bot';

const keyboard = new Keyboard();

// 按键（按下并释放）
keyboard.keyTap('a');
keyboard.keyTap('enter');
keyboard.keyTap('c', ['control']);        // Ctrl+C
keyboard.keyTap('v', ['control', 'shift']); // Ctrl+Shift+V

// 按下/释放按键
keyboard.keyToggle('a', 'down');           // 按下 'a'
keyboard.keyToggle('a', 'up');             // 释放 'a'
keyboard.keyToggle('shift', 'down', ['control']); // Ctrl+Shift 按下

// 输入文本
keyboard.typeString('Hello, World!');

// 延迟输入文本（字符每分钟）
keyboard.typeStringDelayed('Hello', 300); // 300 CPM

// 设置键盘操作延迟（毫秒）
keyboard.setKeyboardDelay(10);
```

```typescript
// 方式 2: 使用全局函数（推荐）
import { keyTap, keyToggle, typeString, typeStringDelayed, unicodeTap, setKeyboardDelay } from '@tego/bot';

// 按键
keyTap('a');
keyTap('enter');
keyTap('c', ['control']);                 // Ctrl+C
keyTap('v', ['control', 'shift']);        // Ctrl+Shift+V

// 按下/释放
keyToggle('shift', 'down');               // 按下 Shift
keyToggle('shift', 'up');                 // 释放 Shift

// 输入文本
typeString('Hello, World!');
typeStringDelayed('Hello', 300);          // 300 CPM

// Unicode 字符（如 emoji）
unicodeTap(0x1f600);                      // 😀

// 设置延迟
setKeyboardDelay(10);
```

### 屏幕操作

```typescript
import { getScreen, getScreenSize, getPixelColor, bitmapColorAt } from '@tego/bot';
import type { Bitmap } from '@tego/bot';
import fs from 'fs';

// 获取屏幕实例
const screen = getScreen();

// 截取整个屏幕
const fullScreen: Bitmap = await screen.capture();
fs.writeFileSync('screenshot.png', fullScreen.image);
console.log(`Captured: ${fullScreen.width}x${fullScreen.height}`);

// 截取屏幕区域 (x, y, width, height)
const region: Bitmap = await screen.capture(100, 100, 800, 600);
fs.writeFileSync('region.png', region.image);

// 获取屏幕尺寸
const size = getScreenSize();
console.log(`Screen size: ${size.width}x${size.height}`);

// 获取指定坐标的像素颜色（返回 hex 字符串，如 "#FF0000"）
const color = await getPixelColor(100, 200);
console.log(`Pixel color: ${color}`);

// 从 Bitmap 中获取指定坐标的颜色
const bitmapColor = bitmapColorAt(region, 50, 50);
console.log(`Color at (50, 50) in bitmap: ${bitmapColor}`);
```

## 完整示例

```typescript
import { Mouse, Keyboard, getScreen, moveMouse, keyTap, typeString } from '@tego/bot';
import fs from 'fs';

async function automationExample() {
    // 使用类实例
    const mouse = new Mouse();
    const keyboard = new Keyboard();

    // 移动鼠标并点击
    mouse.moveMouseSmooth(500, 300);
    mouse.mouseClick('left');

    // 输入文本
    keyboard.typeString('Hello from @tego/bot!');
    keyboard.keyTap('enter');

    // 或使用全局函数
    moveMouse(600, 400);
    keyTap('enter');
    typeString('Using global functions');

    // 截屏
    const screen = getScreen();
    const screenshot = await screen.capture();
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
cd packages/bot
cargo test

# 构建并测试 Node.js 绑定
npm run build

# 运行 JavaScript 测试（在 botjs 包中）
cd ../botjs
pnpm test

# 运行集成测试（需要系统交互，本地开发时）
ENABLE_INTEGRATION_TESTS=true pnpm test:integration
```

## 与 robotjs 的对比

| 特性 | robotjs | @tego/bot |
|------|---------|-----------|
| 性能 | 中等（C++ 绑定） | ⚡ 极高（Rust 原生） |
| 维护状态 | ❌ 已停止维护 | ✅ 活跃维护 |
| 内存安全 | ⚠️ C++ | ✅ Rust |
| API 设计 | ✅ 简洁 | ✅ 兼容 |
| 跨平台 | ✅ | ✅ |
| 类型安全 | ⚠️ 运行时检查 | ✅ 编译期保证 |
| 测试覆盖 | ⚠️ 有限 | ✅ 完整 |
| 包名 | `robotjs` | `@tego/bot` |

## 系统要求

### macOS
- macOS 10.13+ 
- 需要屏幕录制权限（系统偏好设置 > 安全性与隐私 > 屏幕录制）

### Windows
- Windows 10+
- 无需额外配置

### Linux
- X11 或 Wayland
- 需要安装系统依赖：
  ```bash
  # Ubuntu/Debian
  sudo apt-get install -y \
    build-essential \
    pkg-config \
    libwayland-dev \
    libxcb1-dev \
    libxrandr-dev \
    libdbus-1-dev \
    libpipewire-0.3-dev \
    libegl1-mesa-dev \
    libgles2-mesa-dev \
    libgbm-dev \
    libxi-dev \
    libxtst-dev
  
  # Fedora
  sudo dnf install \
    gcc \
    pkg-config \
    wayland-devel \
    libxcb-devel \
    libXrandr-devel \
    dbus-devel \
    pipewire-devel \
    mesa-libEGL-devel \
    mesa-libGLES-devel \
    libgbm-devel \
    libXi-devel \
    libXtst-devel
  ```

## 许可证

MIT

## 贡献

欢迎提交 Issue 和 Pull Request！

## 📚 额外资源

### API 参考文档

- **[AutoHotkey API 参考](./docs/autohotkey-api-reference.md)** - AutoHotkey 的 API 参考，可作为功能扩展的灵感来源
- **[Hammerspoon API 参考](./docs/hammerspoon-api-reference.md)** - Hammerspoon (macOS) 的 API 参考，可作为功能扩展的灵感来源
- **[Python 自动化库参考](./docs/python-automation-libraries.md)** - Python 生态系统中类似的桌面自动化库参考

### 相关项目

- [robotjs](https://github.com/octalmage/robotjs) - 原始 Node.js 自动化库
- [enigo](https://github.com/enigo-rs/enigo) - Rust 键盘鼠标控制库
- [xcap](https://github.com/nashaofu/xcap) - Rust 屏幕捕获库
