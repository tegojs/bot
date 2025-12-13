# Pages 目录

此目录包含 Aumate 应用的所有 HTML 入口页面。

## 页面列表

### 1. settings.html
**设置窗口** - 主窗口，应用启动时首先显示

- **用途**: 应用程序设置界面
- **入口**: `/src/settings-main.tsx`
- **组件**: `Settings/SettingsApp.tsx`
- **窗口配置**:
  - 尺寸: 1200x1000
  - 无装饰窗口
  - 透明背景
  - 启动时可见

### 2. index.html
**命令面板窗口**

- **用途**: 快捷命令面板（F3 唤起）
- **入口**: `/src/main.tsx`
- **组件**: `CommandPalette/index.tsx`
- **窗口配置**:
  - 无装饰窗口
  - 透明背景
  - 置顶显示
  - 跳过任务栏
  - 默认隐藏

### 3. screenshot.html
**截图窗口**

- **用途**: 截图编辑和标注
- **入口**: `/src/screenshot-main.tsx`
- **组件**: `Screenshot/ScreenshotMode.tsx`
- **窗口配置**:
  - 全屏覆盖
  - 透明背景
  - 动态创建

## 多页面构建

所有页面通过 `vite.config.ts` 配置为多入口构建：

```typescript
build: {
  rollupOptions: {
    input: {
      settings: path.resolve(__dirname, "pages/settings.html"),
      main: path.resolve(__dirname, "pages/index.html"),
      screenshot: path.resolve(__dirname, "pages/screenshot.html"),
    },
  },
}
```

## 开发注意事项

1. **添加新页面**:
   - 在此目录创建新的 HTML 文件
   - 在 `vite.config.ts` 中添加入口配置
   - 在 `src/` 下创建对应的 `xxx-main.tsx` 入口文件
   - 在 `tauri.conf.json` 中配置窗口（如需要）

2. **资源引用**:
   - 所有静态资源使用相对于项目根目录的路径
   - TypeScript 入口使用 `/src/xxx-main.tsx` 格式

3. **启动顺序**:
   - `tauri.conf.json` 中第一个窗口会在应用启动时创建
   - 当前配置为 `settings` 窗口首先启动
