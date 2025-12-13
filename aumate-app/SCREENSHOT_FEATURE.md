# 截图功能使用指南

## 🚀 快速开始

### 1. 安装依赖

```bash
cd aumate-app
pnpm install
```

### 2. 启动开发服务器

```bash
pnpm tauri dev
```

### 3. 使用截图功能

- **触发截图**: 按 `F2` 键
- **退出截图**: 按 `ESC` 键
- **命令面板**: 按 `F3` 键（原有功能）

## 📦 已完成的功能

### ✅ 基础架构
1. **完整的类型系统**
   - 定义了所有核心类型（DrawState、CaptureStep 等）
   - 接口定义完整（ImageLayerActionType 等）

2. **状态管理系统**
   - 发布订阅模式（Publisher/Subscriber）
   - 10+ 状态发布者

3. **页面架构**
   - DrawPage 主组件
   - 生命周期管理
   - 事件监听系统

### ✅ 用户界面
- 全屏截图窗口
- 状态栏提示
- ESC 键退出

## 🔧 项目结构

```
aumate-app/
├── pages/
│   └── draw.html              # 截图页面 HTML 入口
├── src/
│   ├── draw-main.tsx          # 截图页面 React 入口
│   ├── hooks/
│   │   └── useStatePublisher.tsx  # 状态管理 Hook
│   └── pages/draw/
│       ├── page.tsx           # 主页面组件
│       ├── types.ts           # 类型定义
│       ├── extra.ts           # 状态发布者
│       ├── page.module.css    # 样式
│       └── components/        # 子组件目录
│           ├── imageLayer/
│           ├── selectLayer/
│           ├── drawLayer/
│           ├── drawToolbar/
│           ├── colorPicker/
│           ├── ocrBlocks/
│           ├── statusBar/     # ✅ 已实现
│           └── captureHistory/
└── src-tauri/
    ├── tauri.conf.json        # ✅ 已配置 draw 窗口
    └── src/
        └── lib.rs             # ✅ 已注册 F2 快捷键
```

## 🎯 下一步开发

### 优先级 1: 核心截图功能
1. **后端命令实现**
   ```rust
   // 需要在 src-tauri/src/commands/ 中实现
   - capture_all_monitors()
   - get_monitors_bounding_box()
   - get_mouse_position()
   ```

2. **ImageLayer 组件**
   - 使用 PixiJS 渲染截图
   - 图像加载和显示

3. **SelectLayer 组件**
   - 矩形选区绘制
   - 窗口识别

### 优先级 2: 绘图功能
1. **DrawLayer 组件**
   - 集成 Excalidraw
   - 配置绘图工具

2. **DrawToolbar 组件**
   - 工具按钮
   - 操作按钮（保存、复制）

### 优先级 3: 高级功能
- OCR 文字识别
- 二维码扫描
- 滚动截图

## 📚 技术文档

### 状态管理

使用发布订阅模式管理状态：

```typescript
// 定义发布者
export const DrawStatePublisher = createPublisher<DrawState>(DrawState.Idle);

// 在组件中订阅
const [getDrawState, setDrawState] = useStateSubscriber(DrawStatePublisher);

// 发布状态更新
setDrawState(DrawState.Arrow);
```

### 组件通信

通过 Context 和 Action Refs 实现：

```typescript
// DrawContext 提供共享引用
const { imageLayerActionRef, selectLayerActionRef } = useContext(DrawContext);

// 通过 Action 接口调用子组件方法
await imageLayerActionRef.current?.onCaptureReady(imageSrc, imageBuffer);
```

## 🐛 调试

### 查看日志
- 前端日志：浏览器控制台
- 后端日志：终端 Tauri 输出

### 检查窗口状态
```typescript
// 在 DrawPage 组件中添加日志
console.log("[DrawPage] Window state:", drawPageStateRef.current);
```

## 🤝 贡献

参考文档：
- 详细计划：`.plan.md`
- 实现状态：`IMPLEMENTATION_STATUS.md`
- DDD 规范：`.cursorrules`

---

**提示**: 当前版本是基础架构版本，主要功能组件还在开发中。按 F2 可以看到截图窗口，但实际截图功能需要后续实现。
