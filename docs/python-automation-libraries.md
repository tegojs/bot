# Python 桌面自动化库参考

本文档列出了 Python 中用于桌面自动化的主要库，这些库提供了类似于 `robotjs` 和 `@tego/bot` 的功能。

## 主要库对比

| 库名 | 功能 | 跨平台 | 活跃度 | 推荐度 |
|------|------|--------|--------|--------|
| **PyAutoGUI** | 鼠标、键盘、屏幕截图 | ✅ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **pynput** | 鼠标、键盘控制与监听 | ✅ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| **keyboard** | 键盘控制与监听 | ✅ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| **PyUserInput** | 鼠标、键盘控制 | ✅ | ⭐⭐ | ⭐⭐ |
| **pywinauto** | Windows GUI 自动化 | ❌ (仅 Windows) | ⭐⭐⭐⭐ | ⭐⭐⭐ (Windows) |
| **pyautogui** | 同 PyAutoGUI | ✅ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |

## 1. PyAutoGUI

**最流行的 Python 桌面自动化库**，功能全面，API 设计简洁。

### 安装
```bash
pip install pyautogui
```

### 主要功能

#### 鼠标控制
```python
import pyautogui

# 获取屏幕尺寸
screen_width, screen_height = pyautogui.size()

# 获取鼠标当前位置
x, y = pyautogui.position()

# 移动鼠标（绝对坐标）
pyautogui.moveTo(100, 200)

# 移动鼠标（相对坐标）
pyautogui.moveRel(50, 100)

# 平滑移动鼠标
pyautogui.moveTo(500, 500, duration=1.5)

# 鼠标点击
pyautogui.click()                    # 左键单击
pyautogui.click(100, 200)            # 在指定位置点击
pyautogui.click(button='right')      # 右键点击
pyautogui.click(button='middle')    # 中键点击
pyautogui.doubleClick()              # 双击
pyautogui.tripleClick()              # 三击

# 鼠标按下和释放
pyautogui.mouseDown(button='left')
pyautogui.mouseUp(button='left')

# 鼠标拖拽
pyautogui.dragTo(300, 400, duration=1)
pyautogui.dragRel(50, 100, duration=1)

# 鼠标滚动
pyautogui.scroll(3)      # 向上滚动
pyautogui.scroll(-3)     # 向下滚动
pyautogui.hscroll(3)     # 水平滚动
```

#### 键盘控制
```python
import pyautogui

# 输入字符串
pyautogui.write('Hello, World!')
pyautogui.write('Hello, World!', interval=0.1)  # 带延迟

# 按键
pyautogui.press('enter')
pyautogui.press('tab')
pyautogui.press('space')

# 组合键
pyautogui.hotkey('ctrl', 'c')        # Ctrl+C
pyautogui.hotkey('ctrl', 'shift', 'esc')  # Ctrl+Shift+Esc

# 按下和释放
pyautogui.keyDown('shift')
pyautogui.keyUp('shift')

# 特殊键
pyautogui.press(['left', 'left', 'left'])  # 按多次
```

#### 屏幕截图
```python
import pyautogui

# 全屏截图
screenshot = pyautogui.screenshot()
screenshot.save('screenshot.png')

# 区域截图
screenshot = pyautogui.screenshot(region=(0, 0, 300, 400))
screenshot.save('region.png')

# 获取像素颜色
pixel_color = pyautogui.pixel(100, 200)
print(pixel_color)  # (R, G, B)

# 查找图像
button_location = pyautogui.locateOnScreen('button.png')
if button_location:
    pyautogui.click(button_location)
```

#### 图像识别
```python
import pyautogui

# 查找图像位置
location = pyautogui.locateOnScreen('icon.png')
if location:
    center = pyautogui.center(location)
    pyautogui.click(center)

# 查找所有匹配
all_locations = pyautogui.locateAllOnScreen('icon.png')
for location in all_locations:
    print(location)

# 带置信度匹配
location = pyautogui.locateOnScreen('icon.png', confidence=0.8)
```

### 优点
- ✅ API 简洁易用
- ✅ 功能全面（鼠标、键盘、截图、图像识别）
- ✅ 跨平台支持
- ✅ 文档完善
- ✅ 社区活跃

### 缺点
- ❌ 图像识别功能较慢
- ❌ 不支持中文字符输入（需要其他库配合）
- ❌ 某些平台需要额外依赖

---

## 2. pynput

**专注于鼠标和键盘控制与监听**，适合需要监听用户输入的场景。

### 安装
```bash
pip install pynput
```

### 主要功能

#### 鼠标控制
```python
from pynput.mouse import Button, Controller

mouse = Controller()

# 获取鼠标位置
position = mouse.position
print(f"Mouse position: {position}")

# 设置鼠标位置
mouse.position = (100, 200)

# 移动鼠标（相对）
mouse.move(50, 100)

# 鼠标点击
mouse.click(Button.left, 1)      # 左键单击
mouse.click(Button.right, 2)     # 右键双击
mouse.press(Button.left)          # 按下
mouse.release(Button.left)         # 释放

# 鼠标滚动
mouse.scroll(0, 3)    # 向上滚动
mouse.scroll(0, -3)   # 向下滚动
```

#### 键盘控制
```python
from pynput.keyboard import Key, Controller

keyboard = Controller()

# 输入字符串
keyboard.type('Hello, World!')

# 按键
keyboard.press(Key.space)
keyboard.release(Key.space)

# 组合键
with keyboard.pressed(Key.ctrl):
    keyboard.press('c')
    keyboard.release('c')

# 特殊键
keyboard.press(Key.enter)
keyboard.press(Key.tab)
keyboard.press(Key.esc)
```

#### 鼠标监听
```python
from pynput import mouse

def on_move(x, y):
    print(f"Mouse moved to ({x}, {y})")

def on_click(x, y, button, pressed):
    print(f"Mouse {'pressed' if pressed else 'released'} at ({x}, {y})")

def on_scroll(x, y, dx, dy):
    print(f"Mouse scrolled at ({x}, {y})")

# 创建监听器
listener = mouse.Listener(
    on_move=on_move,
    on_click=on_click,
    on_scroll=on_scroll
)
listener.start()
listener.join()
```

#### 键盘监听
```python
from pynput import keyboard

def on_press(key):
    try:
        print(f"Key pressed: {key.char}")
    except AttributeError:
        print(f"Special key pressed: {key}")

def on_release(key):
    if key == keyboard.Key.esc:
        return False  # 停止监听

# 创建监听器
listener = keyboard.Listener(
    on_press=on_press,
    on_release=on_release
)
listener.start()
listener.join()
```

### 优点
- ✅ 支持事件监听
- ✅ 控制精度高
- ✅ 跨平台支持
- ✅ 适合需要监听用户输入的场景

### 缺点
- ❌ 没有内置截图功能
- ❌ 没有图像识别功能
- ❌ API 相对复杂

---

## 3. keyboard

**专注于键盘操作**，轻量级，适合键盘自动化场景。

### 安装
```bash
pip install keyboard
```

### 主要功能

```python
import keyboard

# 按键
keyboard.press('a')
keyboard.release('a')

# 按键（按下并释放）
keyboard.press_and_release('enter')

# 输入字符串
keyboard.write('Hello, World!')

# 组合键
keyboard.press_and_release('ctrl+c')
keyboard.press_and_release('ctrl+shift+esc')

# 监听按键
keyboard.on_press_key('a', lambda _: print('a pressed'))
keyboard.on_release_key('a', lambda _: print('a released'))

# 等待按键
keyboard.wait('esc')  # 等待按下 Esc

# 记录和回放
keyboard.start_recording()
# ... 用户操作 ...
events = keyboard.stop_recording()
keyboard.replay(events)
```

### 优点
- ✅ 轻量级
- ✅ 键盘操作简单
- ✅ 支持录制和回放

### 缺点
- ❌ 仅支持键盘，不支持鼠标
- ❌ 某些功能需要管理员权限

---

## 4. PyUserInput

**跨平台的鼠标和键盘控制库**，但已不太活跃。

### 安装
```bash
pip install PyUserInput
```

### 主要功能

```python
from pymouse import PyMouse
from pykeyboard import PyKeyboard

mouse = PyMouse()
keyboard = PyKeyboard()

# 获取屏幕尺寸
x_dim, y_dim = mouse.screen_size()

# 鼠标操作
mouse.move(100, 200)
mouse.click(100, 200, 1)  # 左键
mouse.click(100, 200, 2)  # 右键

# 键盘操作
keyboard.type_string('Hello, World!')
keyboard.press_key('Enter')
```

### 优点
- ✅ 跨平台
- ✅ API 简单

### 缺点
- ❌ 项目不太活跃
- ❌ 功能相对有限
- ❌ 依赖项可能有问题

---

## 5. pywinauto

**Windows 专用的 GUI 自动化库**，可以控制 Windows 应用程序。

### 安装
```bash
pip install pywinauto
```

### 主要功能

```python
from pywinauto import Application

# 启动应用程序
app = Application().start("notepad.exe")

# 连接到已运行的应用程序
app = Application().connect(title="Untitled - Notepad")

# 控制窗口
app.UntitledNotepad.type_keys("Hello, World!")

# 点击按钮
app.UntitledNotepad.Button.click()
```

### 优点
- ✅ Windows GUI 自动化强大
- ✅ 可以控制应用程序窗口和控件

### 缺点
- ❌ 仅支持 Windows
- ❌ 学习曲线较陡

---

## 6. 其他相关库

### Pillow (PIL)
用于图像处理，常与 PyAutoGUI 配合使用：
```python
from PIL import Image
import pyautogui

screenshot = pyautogui.screenshot()
screenshot = screenshot.convert('RGB')
```

### opencv-python
用于图像识别和计算机视觉：
```python
import cv2
import pyautogui

screenshot = pyautogui.screenshot()
screenshot_np = np.array(screenshot)
screenshot_np = cv2.cvtColor(screenshot_np, cv2.COLOR_RGB2BGR)
```

### pygetwindow
用于窗口管理：
```python
import pygetwindow as gw

# 获取所有窗口
windows = gw.getAllWindows()

# 获取活动窗口
active = gw.getActiveWindow()

# 移动和调整窗口
window = gw.getWindowsWithTitle('Notepad')[0]
window.moveTo(100, 100)
window.resizeTo(800, 600)
```

---

## 推荐使用场景

### 1. 通用桌面自动化
**推荐：PyAutoGUI**
- 功能最全面
- API 最简洁
- 适合大多数场景

### 2. 需要监听用户输入
**推荐：pynput**
- 支持鼠标和键盘监听
- 适合需要捕获用户操作的场景

### 3. 仅需要键盘操作
**推荐：keyboard**
- 轻量级
- 简单易用

### 4. Windows GUI 自动化
**推荐：pywinauto**
- 可以控制 Windows 应用程序
- 适合需要操作特定应用程序的场景

### 5. 图像识别和自动化
**推荐：PyAutoGUI + opencv-python**
- PyAutoGUI 提供基础功能
- opencv-python 提供高级图像处理

---

## 与 @tego/bot 的对比

| 功能 | PyAutoGUI | pynput | @tego/bot |
|------|-----------|--------|-----------|
| 鼠标控制 | ✅ | ✅ | ✅ |
| 键盘控制 | ✅ | ✅ | ✅ |
| 屏幕截图 | ✅ | ❌ | ✅ |
| 图像识别 | ✅ | ❌ | ❌ (可扩展) |
| 事件监听 | ❌ | ✅ | ❌ (可扩展) |
| 性能 | 中等 | 高 | 高 (Rust) |
| 跨平台 | ✅ | ✅ | ✅ |
| 类型安全 | ❌ | ❌ | ✅ (TypeScript) |

---

## 总结

Python 生态系统中有多个优秀的桌面自动化库，其中 **PyAutoGUI** 是最流行和最全面的选择，功能最接近 `robotjs` 和 `@tego/bot`。

对于需要高性能的场景，可以考虑使用 Rust 实现的 `@tego/bot`，它提供了更好的性能和类型安全。

