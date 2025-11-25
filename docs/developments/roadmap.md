# Tego Bot 开发路线图

## 技术栈规划

### 一期：egui + winit（基础 UI 框架）
- **目标**：建立基础的窗口系统和 UI 组件库
- **技术选型**：
  - `egui` - 即时模式 GUI 框架，适合过程化开发
  - `winit` - 跨平台窗口管理
  - `softbuffer` - CPU 渲染后端（轻量级）
- **适用场景**：基础 UI 组件、简单动效、悬浮窗口基础功能

### 二期：egui + wgpu（高级渲染和特效）
- **目标**：支持复杂的图形特效和 GPU 加速渲染
- **技术选型**：
  - `egui` - 保持 UI 框架一致性
  - `wgpu` - GPU 加速渲染
  - `egui-wgpu` - egui 与 wgpu 集成
- **适用场景**：复杂动效、高级图形渲染、性能要求高的场景

---

## 功能模块详细设计

### 1. 屏幕截图与标注工具（Screen Capture & Annotation）

#### 功能描述
提供完整的屏幕截图工具链，包括：
- 全屏/区域截图
- 交互式区域选择（鼠标拖拽选择）
- **窗口吸附功能**：选区可自动吸附到窗口边缘
- **选区调整**：支持拖拽边缘和角落调整选区大小
- **选区变化回调**：实时获取选区坐标和吸附窗口信息
- 截图后标注功能（画笔、箭头、文字、矩形、圆形等）
- 像素颜色拾取器
- 截图保存（支持 PNG、JPG、WebP 格式，保存时指定）
- 剪贴板操作
- 截图后处理回调（OCR、识别等）

#### TypeScript API 设计

```typescript
// ============================================================================
// 截图工具核心 API
// ============================================================================

/**
 * 截图工具类 - 提供完整的截图和标注功能
 */
export class ScreenshotTool {
  /**
   * 创建截图工具实例
   * @param options - 配置选项
   */
  constructor(options?: ScreenshotToolOptions);

  /**
   * 启动交互式截图（类似 Snipaste）
   * 显示全屏遮罩，用户可拖拽选择区域
   * 支持窗口吸附、选区调整、实时回调
   * @param options - 截图选项
   * @returns Promise<ScreenshotResult> - 截图结果
   */
  captureInteractive(options?: InteractiveCaptureOptions): Promise<ScreenshotResult>;

  /**
   * 获取当前选区信息（在交互式截图过程中）
   * @returns SelectionInfo | null - 当前选区信息，如果未在截图过程中则返回 null
   */
  getCurrentSelection(): SelectionInfo | null;

  /**
   * 快速截图（直接截取全屏或指定区域，无交互）
   * @param region - 可选，指定截图区域
   * @returns Promise<ScreenshotResult>
   */
  captureQuick(region?: ScreenRegion): Promise<ScreenshotResult>;

  /**
   * 打开标注编辑器
   * 在截图结果上添加标注（画笔、箭头、文字等）
   * @param screenshot - 截图结果
   * @param options - 编辑器选项
   * @returns Promise<AnnotatedScreenshot> - 标注后的截图
   */
  annotate(
    screenshot: ScreenshotResult,
    options?: AnnotationEditorOptions
  ): Promise<AnnotatedScreenshot>;

  /**
   * 获取屏幕指定位置的像素颜色
   * @param x - X 坐标
   * @param y - Y 坐标
   * @returns Promise<ColorInfo> - 颜色信息
   */
  getPixelColor(x: number, y: number): Promise<ColorInfo>;

  /**
   * 启动颜色拾取器（实时显示鼠标位置的颜色）
   * @param options - 拾取器选项
   * @returns Promise<ColorInfo> - 选中的颜色
   */
  pickColor(options?: ColorPickerOptions): Promise<ColorInfo>;

  /**
   * 关闭截图工具，清理资源
   */
  close(): Promise<void>;
}

/**
 * 截图工具配置选项
 */
export interface ScreenshotToolOptions {
  /** 默认保存路径（可选） */
  defaultSavePath?: string;
  /** 是否自动保存到剪贴板 */
  autoCopyToClipboard?: boolean;
  /** 截图后处理回调 */
  onCaptureComplete?: (result: ScreenshotResult) => void | Promise<void>;
}

/**
 * 交互式截图选项
 */
export interface InteractiveCaptureOptions {
  /** 是否显示网格辅助线 */
  showGrid?: boolean;
  /** 是否显示坐标信息 */
  showCoordinates?: boolean;
  /** 是否显示尺寸信息 */
  showSize?: boolean;
  /** 选择区域时的提示文字 */
  hintText?: string;
  /** 是否启用窗口吸附 */
  enableWindowSnap?: boolean;
  /** 窗口吸附阈值（像素，默认 10） */
  snapThreshold?: number;
  /** 选区变化回调 */
  onSelectionChange?: (info: SelectionInfo) => void;
  /** 快捷键配置 */
  hotkeys?: {
    /** 确认截图（默认 Enter） */
    confirm?: string;
    /** 取消截图（默认 Escape） */
    cancel?: string;
    /** 切换全屏模式（默认 F） */
    toggleFullscreen?: string;
  };
}

/**
 * 截图结果
 */
export interface ScreenshotResult {
  /** 截图图像数据（PNG Buffer，原始格式） */
  image: Buffer;
  /** 截图区域信息 */
  region: ScreenRegion;
  /** 截图时间戳 */
  timestamp: number;
  /** 保存到文件（可指定格式和质量） */
  saveToFile(
    path: string,
    options?: SaveImageOptions
  ): Promise<void>;
  /** 复制到剪贴板 */
  copyToClipboard?(): Promise<void>;
}

/**
 * 保存图片选项
 */
export interface SaveImageOptions {
  /** 图片格式 */
  format?: 'png' | 'jpg' | 'webp';
  /** 图片质量（1-100，仅对 jpg/webp 有效，默认 90） */
  quality?: number;
}

/**
 * 选区信息（用于回调）
 */
export interface SelectionInfo {
  /** 当前选区区域 */
  region: ScreenRegion;
  /** 是否吸附到窗口 */
  isSnapped: boolean;
  /** 吸附的窗口信息（如果吸附） */
  snappedWindow?: WindowSnapInfo;
  /** 是否可以调整大小 */
  canResize: boolean;
}

/**
 * 窗口吸附信息
 */
export interface WindowSnapInfo {
  /** 窗口 ID */
  windowId: string;
  /** 窗口标题 */
  title: string;
  /** 窗口区域 */
  region: ScreenRegion;
  /** 吸附边缘（'left' | 'right' | 'top' | 'bottom' | 'all'） */
  snapEdge: string;
}

/**
 * 屏幕区域
 */
export interface ScreenRegion {
  x: number;
  y: number;
  width: number;
  height: number;
}

/**
 * 标注编辑器选项
 */
export interface AnnotationEditorOptions {
  /** 初始工具（默认 'arrow'） */
  defaultTool?: AnnotationTool;
  /** 画笔颜色 */
  brushColor?: string;
  /** 画笔大小 */
  brushSize?: number;
  /** 是否显示工具栏 */
  showToolbar?: boolean;
  /** 可用的标注工具 */
  availableTools?: AnnotationTool[];
}

/**
 * 标注工具类型
 */
export type AnnotationTool = 
  | 'arrow'      // 箭头
  | 'brush'      // 画笔
  | 'rectangle'  // 矩形
  | 'circle'     // 圆形
  | 'text'       // 文字
  | 'blur'       // 模糊/马赛克
  | 'highlight'  // 高亮
  | 'eraser';    // 橡皮擦

/**
 * 标注后的截图
 */
export interface AnnotatedScreenshot extends ScreenshotResult {
  /** 标注图层数据（可用于撤销/重做） */
  annotations: AnnotationLayer[];
}

/**
 * 标注图层
 */
export interface AnnotationLayer {
  id: string;
  type: AnnotationTool;
  data: unknown; // 根据工具类型不同而不同
  timestamp: number;
}

/**
 * 颜色信息
 */
export interface ColorInfo {
  /** RGB 值 */
  rgb: { r: number; g: number; b: number };
  /** RGBA 值 */
  rgba: { r: number; g: number; b: number; a: number };
  /** HEX 格式 */
  hex: string;
  /** HSL 值 */
  hsl: { h: number; s: number; l: number };
  /** 坐标位置 */
  position: { x: number; y: number };
}

/**
 * 颜色拾取器选项
 */
export interface ColorPickerOptions {
  /** 放大镜大小 */
  magnifierSize?: number;
  /** 放大倍数 */
  zoom?: number;
  /** 是否显示颜色历史 */
  showHistory?: boolean;
}

// ============================================================================
// 便捷函数
// ============================================================================

/**
 * 快速截图（全屏）
 */
export function quickScreenshot(): Promise<ScreenshotResult>;

/**
 * 快速截图（指定区域）
 */
export function quickScreenshotRegion(region: ScreenRegion): Promise<ScreenshotResult>;

/**
 * 启动交互式截图
 */
export function startInteractiveCapture(
  options?: InteractiveCaptureOptions
): Promise<ScreenshotResult>;
```

---

### 2. 长截图功能（Long Screenshot / Scrolling Capture）

#### 功能描述
提供自动滚动截图功能，支持：
- 自动检测可滚动区域
- 垂直/水平滚动截图
- 多显示器拼接
- 滚动速度控制
- 智能去重（去除重叠部分）

#### TypeScript API 设计

```typescript
// ============================================================================
// 长截图 API
// ============================================================================

/**
 * 长截图工具类
 */
export class LongScreenshotTool {
  /**
   * 创建长截图工具实例
   */
  constructor(options?: LongScreenshotOptions);

  /**
   * 垂直长截图（向下滚动）
   * @param options - 长截图选项
   * @returns Promise<LongScreenshotResult>
   */
  captureVertical(options?: VerticalCaptureOptions): Promise<LongScreenshotResult>;

  /**
   * 水平长截图（向右滚动）
   * @param options - 长截图选项
   * @returns Promise<LongScreenshotResult>
   */
  captureHorizontal(options?: HorizontalCaptureOptions): Promise<LongScreenshotResult>;

  /**
   * 自定义方向长截图
   * @param direction - 滚动方向
   * @param options - 长截图选项
   * @returns Promise<LongScreenshotResult>
   */
  capture(
    direction: ScrollDirection,
    options?: LongScreenshotOptions
  ): Promise<LongScreenshotResult>;

  /**
   * 取消正在进行的截图
   */
  cancel(): Promise<void>;
}

/**
 * 长截图配置选项
 */
export interface LongScreenshotOptions {
  /** 目标区域（如果不指定，则自动检测） */
  region?: ScreenRegion;
  /** 滚动速度（像素/秒） */
  scrollSpeed?: number;
  /** 每次滚动后等待时间（ms） */
  scrollDelay?: number;
  /** 最大截图长度（像素，防止无限滚动） */
  maxLength?: number;
  /** 是否智能去重（检测并去除重叠部分） */
  enableDeduplication?: boolean;
  /** 进度回调 */
  onProgress?: (progress: LongScreenshotProgress) => void;
}

/**
 * 垂直截图选项
 */
export interface VerticalCaptureOptions extends LongScreenshotOptions {
  /** 滚动步长（像素） */
  scrollStep?: number;
  /** 是否自动检测滚动区域 */
  autoDetectScrollArea?: boolean;
}

/**
 * 水平截图选项
 */
export interface HorizontalCaptureOptions extends LongScreenshotOptions {
  /** 滚动步长（像素） */
  scrollStep?: number;
  /** 是否自动检测滚动区域 */
  autoDetectScrollArea?: boolean;
}

/**
 * 滚动方向
 */
export type ScrollDirection = 'up' | 'down' | 'left' | 'right';

/**
 * 长截图结果
 */
export interface LongScreenshotResult {
  /** 拼接后的完整图像 */
  image: Buffer;
  /** 最终尺寸 */
  dimensions: { width: number; height: number };
  /** 原始截图片段数量 */
  segmentCount: number;
  /** 截图耗时（ms） */
  duration: number;
  /** 保存到文件 */
  saveToFile(path: string): Promise<void>;
}

/**
 * 长截图进度
 */
export interface LongScreenshotProgress {
  /** 当前进度（0-1） */
  progress: number;
  /** 已截取的片段数 */
  segments: number;
  /** 当前总长度（像素） */
  currentLength: number;
  /** 预计总长度（像素，可能为 undefined） */
  estimatedLength?: number;
}

// ============================================================================
// 便捷函数
// ============================================================================

/**
 * 快速垂直长截图
 */
export function captureLongScreenshotVertical(
  options?: VerticalCaptureOptions
): Promise<LongScreenshotResult>;

/**
 * 快速水平长截图
 */
export function captureLongScreenshotHorizontal(
  options?: HorizontalCaptureOptions
): Promise<LongScreenshotResult>;
```

---

### 3. 悬浮窗口系统（Floating Window）

#### 功能描述
提供创建和管理悬浮窗口的能力，支持：
- 创建无边框、可拖拽的悬浮窗口
- **窗口形状**：支持矩形、圆形、异形（透明图片遮罩）
- **异形窗口**：使用透明图片定义窗口形状，鼠标仅在非透明区域响应
- **窗口图标**：支持 Emoji、预设图标（枚举）、自定义图片
- **边缘粒子特效**：内置 6 种边缘粒子特效（旋转光环、脉冲波纹、流光溢彩、星尘飘散、电流闪烁、烟雾缭绕）
- 动态替换窗口内容（图片、文本、自定义 UI）
- 窗口动效（淡入淡出、缩放、移动动画）
- 窗口层级管理
- 多窗口管理
- 窗口事件处理

#### 边缘粒子特效详细说明

所有粒子特效都在窗口边缘生成和运动，形成围绕窗口的视觉效果：

**1. 旋转光环（Rotating Halo）**
- **视觉效果**：粒子沿窗口边缘顺时针或逆时针旋转，形成连续的光环效果
- **算法实现**：
  - 粒子沿窗口边缘均匀分布，使用极坐标系统定位
  - 位置计算：`θ(t) = θ₀ + ω·t`，其中 `ω` 为角速度（弧度/秒）
  - 坐标转换：`x = center_x + r·cos(θ)`，`y = center_y + r·sin(θ)`
  - 使用 `sin` 和 `cos` 函数实现平滑的圆周运动
  - 粒子颜色可随角度变化，使用 `sin(θ + phase)` 控制颜色强度
- **参数**：旋转速度（ω）、粒子密度、颜色渐变

**2. 脉冲波纹（Pulse Ripple）**
- **视觉效果**：粒子从窗口边缘向外扩散，形成同心圆波纹，类似水波
- **算法实现**：
  - 粒子从边缘位置开始，沿径向向外运动
  - 径向距离：`r(t) = r₀ + v·t + A·sin(2πft)`，其中 `A` 为振幅，`f` 为频率
  - 透明度衰减：`α(t) = α₀·(1 - t/T)`，`T` 为生命周期
  - 使用 `sin` 函数实现波纹的周期性变化
  - 多层波纹叠加，使用相位差 `φ = 2π·n/N` 实现连续波纹
- **参数**：波纹速度、波纹频率、衰减速度

**3. 流光溢彩（Flowing Light）**
- **视觉效果**：粒子沿窗口边缘流动，形成连续的流光效果，类似霓虹灯
- **算法实现**：
  - 粒子沿边缘路径运动，使用参数化曲线
  - 路径参数：`s(t) = (s₀ + v·t) mod L`，`L` 为边缘总长度
  - 亮度变化：`brightness = (sin(2π·s/λ + phase) + 1) / 2`，`λ` 为波长
  - 使用 `sin` 函数实现流光的亮度渐变
  - 多个粒子形成连续的流光带，相位差实现流动感
- **参数**：流动速度、流光长度、颜色渐变

**4. 星尘飘散（Stardust Scatter）**
- **视觉效果**：粒子从窗口边缘随机位置生成，向随机方向飘散，形成星尘效果
- **算法实现**：
  - 粒子在边缘随机位置生成：`θ = random(0, 2π)`
  - 运动方向：`φ = random(0, 2π)`，速度：`v = v₀ + random(-Δv, Δv)`
  - 位置更新：`x(t) = x₀ + v·cos(φ)·t`，`y(t) = y₀ + v·sin(φ)·t`
  - 透明度衰减：`α(t) = α₀·exp(-t/τ)`，`τ` 为衰减时间常数
  - 使用随机函数和三角函数组合实现自然飘散
- **参数**：生成频率、飘散速度范围、生命周期

**5. 电流闪烁（Electric Spark）**
- **视觉效果**：粒子沿窗口边缘快速闪烁跳动，形成电流效果
- **算法实现**：
  - 粒子在边缘随机位置快速生成和消失
  - 位置跳跃：使用 Perlin 噪声或随机函数生成路径
  - 亮度闪烁：`brightness = random() > threshold ? 1.0 : 0.0`
  - 快速衰减：`α(t) = α₀·(1 - t/T_fast)`，`T_fast` 为快速衰减时间
  - 使用 `sin` 函数叠加实现闪烁频率控制
  - 分支效果：粒子可分裂成多个子粒子
- **参数**：闪烁频率、跳跃距离、分支概率

**6. 烟雾缭绕（Smoke Wisp）**
- **视觉效果**：粒子从窗口边缘缓慢上升并飘散，形成烟雾效果
- **算法实现**：
  - 粒子从边缘底部生成，向上运动
  - 垂直速度：`v_y = v₀·(1 - t/T)`，逐渐减速
  - 水平漂移：`x(t) = x₀ + A·sin(ω·t + φ)`，使用 `sin` 函数实现左右摆动
  - 透明度渐变：`α(t) = α₀·(1 - (t/T)^2)`，非线性衰减
  - 大小变化：`size(t) = size₀·(1 + t/T)`，逐渐变大
  - 使用 `cos` 函数控制烟雾的扭曲效果
- **参数**：上升速度、漂移幅度、扩散速度

#### TypeScript API 设计

```typescript
// ============================================================================
// 悬浮窗口 API
// ============================================================================

/**
 * 悬浮窗口类
 */
export class FloatingWindow {
  /**
   * 创建悬浮窗口
   * @param options - 窗口配置
   */
  constructor(options: FloatingWindowOptions);

  /**
   * 显示窗口
   * @param animation - 可选，显示动画
   */
  show(animation?: WindowAnimation): Promise<void>;

  /**
   * 隐藏窗口
   * @param animation - 可选，隐藏动画
   */
  hide(animation?: WindowAnimation): Promise<void>;

  /**
   * 关闭窗口（释放资源）
   */
  close(): Promise<void>;

  /**
   * 设置窗口位置
   * @param x - X 坐标
   * @param y - Y 坐标
   * @param animated - 是否使用动画
   */
  setPosition(x: number, y: number, animated?: boolean): Promise<void>;

  /**
   * 设置窗口大小
   * @param width - 宽度
   * @param height - 高度
   * @param animated - 是否使用动画
   */
  setSize(width: number, height: number, animated?: boolean): Promise<void>;

  /**
   * 设置窗口形状
   * @param shape - 窗口形状配置
   */
  setShape(shape: WindowShape): Promise<void>;

  /**
   * 设置窗口内容（图片）
   * @param image - 图片 Buffer 或路径
   * @param options - 显示选项
   */
  setImage(
    image: Buffer | string,
    options?: ImageDisplayOptions
  ): Promise<void>;

  /**
   * 设置窗口内容（自定义 UI）
   * @param renderer - 渲染函数
   */
  setContent(renderer: WindowContentRenderer): void;

  /**
   * 更新窗口内容
   */
  update(): Promise<void>;

  /**
   * 设置窗口层级
   * @param level - 层级（'normal' | 'top' | 'always-top'）
   */
  setLevel(level: WindowLevel): Promise<void>;

  /**
   * 设置窗口透明度
   * @param opacity - 透明度（0-1）
   * @param animated - 是否使用动画
   */
  setOpacity(opacity: number, animated?: boolean): Promise<void>;

  /**
   * 设置窗口图标
   * @param icon - 图标配置
   */
  setIcon(icon: WindowIcon): Promise<void>;

  /**
   * 应用预制特效
   * @param effect - 特效类型
   * @param options - 特效选项
   */
  applyPresetEffect(
    effect: PresetWindowEffect,
    options?: PresetEffectOptions
  ): Promise<void>;

  /**
   * 获取窗口信息
   */
  getInfo(): FloatingWindowInfo;

  /**
   * 事件监听
   */
  on(event: FloatingWindowEvent, handler: EventHandler): void;
  off(event: FloatingWindowEvent, handler: EventHandler): void;
}

/**
 * 悬浮窗口配置
 */
export interface FloatingWindowOptions {
  /** 窗口 ID（唯一标识） */
  id?: string;
  /** 初始位置 */
  position?: { x: number; y: number };
  /** 初始大小 */
  size?: { width: number; height: number };
  /** 窗口形状 */
  shape?: WindowShape;
  /** 是否可拖拽 */
  draggable?: boolean;
  /** 是否可调整大小 */
  resizable?: boolean;
  /** 是否点击穿透（鼠标事件穿透到下层） */
  clickThrough?: boolean;
  /** 窗口层级 */
  level?: WindowLevel;
  /** 初始透明度 */
  opacity?: number;
  /** 是否保持最前 */
  alwaysOnTop?: boolean;
  /** 窗口标题（调试用） */
  title?: string;
  /** 窗口图标 */
  icon?: WindowIcon;
  /** 初始内容 */
  content?: WindowContent;
  /** 预制特效（可选，使用内置特效） */
  presetEffect?: PresetWindowEffect;
}

/**
 * 窗口层级
 */
export type WindowLevel = 'normal' | 'top' | 'always-top';

/**
 * 窗口内容类型
 */
export type WindowContent = 
  | { type: 'image'; data: Buffer | string; options?: ImageDisplayOptions }
  | { type: 'text'; data: string; options?: TextDisplayOptions }
  | { type: 'custom'; renderer: WindowContentRenderer };

/**
 * 图片显示选项
 */
export interface ImageDisplayOptions {
  /** 缩放模式 */
  scaleMode?: 'fit' | 'fill' | 'stretch' | 'center';
  /** 背景颜色 */
  backgroundColor?: string;
  /** 是否保持宽高比 */
  maintainAspectRatio?: boolean;
}

/**
 * 文本显示选项
 */
export interface TextDisplayOptions {
  /** 字体大小 */
  fontSize?: number;
  /** 字体颜色 */
  color?: string;
  /** 背景颜色 */
  backgroundColor?: string;
  /** 文本对齐 */
  align?: 'left' | 'center' | 'right';
  /** 是否自动换行 */
  wrap?: boolean;
}

/**
 * 窗口内容渲染器
 * 使用过程化方式定义 UI
 */
export type WindowContentRenderer = (ui: WindowUI) => void;

/**
 * 窗口 UI API（过程化开发接口）
 */
export interface WindowUI {
  /** 绘制图片 */
  image(
    image: Buffer | string,
    x: number,
    y: number,
    width?: number,
    height?: number,
    options?: ImageDisplayOptions
  ): void;

  /** 绘制文本 */
  text(
    text: string,
    x: number,
    y: number,
    options?: TextDisplayOptions
  ): void;

  /** 绘制矩形 */
  rect(
    x: number,
    y: number,
    width: number,
    height: number,
    color: string,
    filled?: boolean
  ): void;

  /** 绘制圆形 */
  circle(
    x: number,
    y: number,
    radius: number,
    color: string,
    filled?: boolean
  ): void;

  /** 绘制线条 */
  line(
    x1: number,
    y1: number,
    x2: number,
    y2: number,
    color: string,
    width?: number
  ): void;

  /** 清除画布 */
  clear(color?: string): void;

  /** 获取画布尺寸 */
  getSize(): { width: number; height: number };
}

/**
 * 窗口形状配置
 */
export type WindowShape = 
  | { type: 'rectangle' }  // 矩形（默认）
  | { type: 'circle' }      // 圆形
  | { type: 'custom'; mask: Buffer | string };  // 异形（使用透明图片作为遮罩）

/**
 * 窗口图标配置
 */
export type WindowIcon = 
  | { type: 'emoji'; emoji: string }  // Emoji 图标
  | { type: 'icon'; icon: IconName }  // 预设图标（使用枚举）
  | { type: 'image'; image: Buffer | string };  // 自定义图片图标

/**
 * 预设图标名称（枚举）
 * 使用第三方图标库（如 Material Icons、Font Awesome 等）
 */
export enum IconName {
  // 常用图标
  CLOSE = 'close',
  MINIMIZE = 'minimize',
  MAXIMIZE = 'maximize',
  SETTINGS = 'settings',
  SEARCH = 'search',
  HOME = 'home',
  BACK = 'back',
  FORWARD = 'forward',
  REFRESH = 'refresh',
  DOWNLOAD = 'download',
  UPLOAD = 'upload',
  SAVE = 'save',
  EDIT = 'edit',
  DELETE = 'delete',
  ADD = 'add',
  REMOVE = 'remove',
  CHECK = 'check',
  CANCEL = 'cancel',
  INFO = 'info',
  WARNING = 'warning',
  ERROR = 'error',
  SUCCESS = 'success',
  // 更多图标...
}

/**
 * 预制窗口边缘粒子特效
 * 所有特效都在窗口边缘生成和运动
 */
export enum PresetWindowEffect {
  /** 旋转光环 - 粒子沿窗口边缘旋转形成光环 */
  ROTATING_HALO = 'rotating-halo',
  /** 脉冲波纹 - 粒子从边缘向外扩散形成波纹 */
  PULSE_RIPPLE = 'pulse-ripple',
  /** 流光溢彩 - 粒子沿边缘流动形成流光 */
  FLOWING_LIGHT = 'flowing-light',
  /** 星尘飘散 - 粒子从边缘随机飘散 */
  STARDUST_SCATTER = 'stardust-scatter',
  /** 电流闪烁 - 粒子沿边缘快速闪烁跳动 */
  ELECTRIC_SPARK = 'electric-spark',
  /** 烟雾缭绕 - 粒子从边缘缓慢上升飘散 */
  SMOKE_WISP = 'smoke-wisp',
}

/**
 * 预制特效选项
 */
export interface PresetEffectOptions {
  /** 粒子数量（默认根据窗口大小自动计算） */
  particleCount?: number;
  /** 粒子大小（像素，默认 2-4） */
  particleSize?: number | { min: number; max: number };
  /** 粒子颜色（默认根据特效类型自动选择） */
  particleColor?: string | string[];
  /** 粒子速度（默认根据特效类型自动设置） */
  particleSpeed?: number;
  /** 特效强度（0-1，影响粒子密度和速度，默认 0.5） */
  intensity?: number;
  /** 是否循环播放（默认 true） */
  loop?: boolean;
  /** 循环次数（-1 表示无限循环，默认 -1） */
  loopCount?: number;
  /** 边缘宽度（粒子生成区域，像素，默认 10） */
  edgeWidth?: number;
}

/**
 * 窗口动画
 */
export interface WindowAnimation {
  /** 动画类型 */
  type: 'fade' | 'scale' | 'slide' | 'none' | 'bounce' | 'rotate' | 'blink';
  /** 动画时长（ms） */
  duration?: number;
  /** 缓动函数 */
  easing?: EasingFunction;
  /** 动画方向（仅对 slide 有效） */
  direction?: 'up' | 'down' | 'left' | 'right';
}

/**
 * 缓动函数类型
 */
export type EasingFunction = 
  | 'linear'
  | 'ease-in'
  | 'ease-out'
  | 'ease-in-out'
  | 'bounce'
  | 'elastic';

/**
 * 窗口信息
 */
export interface FloatingWindowInfo {
  id: string;
  position: { x: number; y: number };
  size: { width: number; height: number };
  shape: WindowShape;
  visible: boolean;
  level: WindowLevel;
  opacity: number;
  icon?: WindowIcon;
}

/**
 * 窗口事件类型
 */
export type FloatingWindowEvent = 
  | 'show'
  | 'hide'
  | 'close'
  | 'move'
  | 'resize'
  | 'click'
  | 'drag-start'
  | 'drag'
  | 'drag-end';

/**
 * 事件处理器
 */
export type EventHandler = (event: WindowEventData) => void;

/**
 * 窗口事件数据
 */
export interface WindowEventData {
  type: FloatingWindowEvent;
  window: FloatingWindowInfo;
  data?: unknown; // 根据事件类型不同
}

// ============================================================================
// 窗口管理器
// ============================================================================

/**
 * 悬浮窗口管理器
 */
export class FloatingWindowManager {
  /**
   * 创建窗口
   */
  createWindow(options: FloatingWindowOptions): FloatingWindow;

  /**
   * 获取窗口
   */
  getWindow(id: string): FloatingWindow | undefined;

  /**
   * 关闭窗口
   */
  closeWindow(id: string): Promise<void>;

  /**
   * 关闭所有窗口
   */
  closeAll(): Promise<void>;

  /**
   * 获取所有窗口
   */
  getAllWindows(): FloatingWindow[];

  /**
   * 设置全局窗口配置
   */
  setGlobalOptions(options: Partial<FloatingWindowOptions>): void;
}

// ============================================================================
// 便捷函数
// ============================================================================

/**
 * 创建悬浮窗口
 */
export function createFloatingWindow(
  options: FloatingWindowOptions
): FloatingWindow;

/**
 * 获取窗口管理器实例
 */
export function getWindowManager(): FloatingWindowManager;
```

---

### 4. 剪贴板管理（Clipboard Manager）

#### 功能描述
提供完整的剪贴板历史管理功能，包括：
- 剪贴板历史记录（文本、图片）
- 历史记录搜索和过滤
- 剪贴板内容分类和标签
- 快速访问历史记录
- 剪贴板内容同步（可选）
- 敏感信息检测和过滤

#### TypeScript API 设计

```typescript
// ============================================================================
// 剪贴板管理 API
// ============================================================================

/**
 * 剪贴板管理器
 */
export class ClipboardManager {
  /**
   * 创建剪贴板管理器
   * @param options - 配置选项
   */
  constructor(options?: ClipboardManagerOptions);

  /**
   * 开始监听剪贴板变化
   */
  start(): Promise<void>;

  /**
   * 停止监听
   */
  stop(): Promise<void>;

  /**
   * 获取剪贴板历史记录
   * @param options - 查询选项
   * @returns Promise<ClipboardHistoryItem[]>
   */
  getHistory(options?: HistoryQueryOptions): Promise<ClipboardHistoryItem[]>;

  /**
   * 获取指定历史记录
   * @param id - 记录 ID
   * @returns Promise<ClipboardHistoryItem | null>
   */
  getHistoryItem(id: string): Promise<ClipboardHistoryItem | null>;

  /**
   * 搜索历史记录
   * @param query - 搜索关键词
   * @param options - 搜索选项
   * @returns Promise<ClipboardHistoryItem[]>
   */
  searchHistory(
    query: string,
    options?: SearchOptions
  ): Promise<ClipboardHistoryItem[]>;

  /**
   * 删除历史记录
   * @param id - 记录 ID
   */
  deleteHistoryItem(id: string): Promise<void>;

  /**
   * 清空历史记录
   * @param options - 清空选项
   */
  clearHistory(options?: ClearHistoryOptions): Promise<void>;

  /**
   * 将历史记录项复制到剪贴板
   * @param id - 记录 ID
   */
  pasteFromHistory(id: string): Promise<void>;

  /**
   * 添加标签到历史记录
   * @param id - 记录 ID
   * @param tags - 标签列表
   */
  addTags(id: string, tags: string[]): Promise<void>;

  /**
   * 从历史记录移除标签
   * @param id - 记录 ID
   * @param tags - 标签列表
   */
  removeTags(id: string, tags: string[]): Promise<void>;

  /**
   * 获取统计信息
   */
  getStatistics(): Promise<ClipboardStatistics>;

  /**
   * 导出历史记录
   * @param format - 导出格式
   * @param path - 导出路径
   */
  exportHistory(format: 'json' | 'csv', path: string): Promise<void>;

  /**
   * 导入历史记录
   * @param path - 导入文件路径
   */
  importHistory(path: string): Promise<void>;

  /**
   * 事件监听
   */
  on(event: ClipboardEvent, handler: ClipboardEventHandler): void;
  off(event: ClipboardEvent, handler: ClipboardEventHandler): void;
}

/**
 * 剪贴板管理器配置
 */
export interface ClipboardManagerOptions {
  /** 最大历史记录数 */
  maxHistorySize?: number;
  /** 历史记录保存路径 */
  storagePath?: string;
  /** 是否自动保存图片 */
  saveImages?: boolean;
  /** 图片保存路径 */
  imageStoragePath?: string;
  /** 是否启用敏感信息过滤 */
  enableSensitiveFilter?: boolean;
  /** 敏感信息检测规则 */
  sensitivePatterns?: RegExp[];
  /** 是否启用同步（未来功能） */
  enableSync?: boolean;
}

/**
 * 历史记录查询选项
 */
export interface HistoryQueryOptions {
  /** 限制返回数量 */
  limit?: number;
  /** 偏移量 */
  offset?: number;
  /** 按类型过滤 */
  type?: ClipboardItemType;
  /** 按标签过滤 */
  tags?: string[];
  /** 排序方式 */
  sortBy?: 'time' | 'size' | 'frequency';
  /** 排序顺序 */
  sortOrder?: 'asc' | 'desc';
}

/**
 * 搜索选项
 */
export interface SearchOptions extends HistoryQueryOptions {
  /** 是否大小写敏感 */
  caseSensitive?: boolean;
  /** 是否使用正则表达式 */
  useRegex?: boolean;
}

/**
 * 清空历史记录选项
 */
export interface ClearHistoryOptions {
  /** 是否清空图片文件 */
  clearImages?: boolean;
  /** 是否只清空指定类型 */
  type?: ClipboardItemType;
}

/**
 * 剪贴板历史记录项
 */
export interface ClipboardHistoryItem {
  /** 唯一 ID */
  id: string;
  /** 内容类型 */
  type: ClipboardItemType;
  /** 内容数据 */
  content: ClipboardContent;
  /** 创建时间 */
  timestamp: number;
  /** 使用次数 */
  usageCount: number;
  /** 最后使用时间 */
  lastUsed: number;
  /** 标签 */
  tags: string[];
  /** 内容大小（字节） */
  size: number;
  /** 预览文本（用于显示） */
  preview: string;
}

/**
 * 剪贴板内容类型
 */
export type ClipboardItemType = 'text' | 'image' | 'file' | 'html' | 'rtf';

/**
 * 剪贴板内容
 */
export type ClipboardContent = 
  | { type: 'text'; data: string }
  | { type: 'image'; data: Buffer; format: 'png' | 'jpg' }
  | { type: 'file'; data: string[] } // 文件路径列表
  | { type: 'html'; data: string }
  | { type: 'rtf'; data: string };

/**
 * 剪贴板统计信息
 */
export interface ClipboardStatistics {
  /** 总记录数 */
  totalItems: number;
  /** 文本记录数 */
  textItems: number;
  /** 图片记录数 */
  imageItems: number;
  /** 总大小（字节） */
  totalSize: number;
  /** 最常用的项目 */
  mostUsed: ClipboardHistoryItem[];
  /** 最近使用的项目 */
  recentlyUsed: ClipboardHistoryItem[];
}

/**
 * 剪贴板事件类型
 */
export type ClipboardEvent = 
  | 'change'        // 剪贴板内容变化
  | 'item-added'    // 新项目添加到历史
  | 'item-deleted'  // 项目从历史删除
  | 'item-updated'  // 项目更新
  | 'history-cleared'; // 历史清空

/**
 * 剪贴板事件处理器
 */
export type ClipboardEventHandler = (event: ClipboardEventData) => void;

/**
 * 剪贴板事件数据
 */
export interface ClipboardEventData {
  type: ClipboardEvent;
  item?: ClipboardHistoryItem;
  data?: unknown;
}

// ============================================================================
// 便捷函数
// ============================================================================

/**
 * 创建剪贴板管理器实例
 */
export function createClipboardManager(
  options?: ClipboardManagerOptions
): ClipboardManager;

/**
 * 获取全局剪贴板管理器实例
 */
export function getClipboardManager(): ClipboardManager;
```

---

### 5. 过程化开发基础组件库（Procedural UI Components）

#### 功能描述
提供一套基于过程化开发方式的 UI 组件库，支持：
- 列表组件（可滚动、虚拟滚动）
- 按钮组件
- 输入框组件
- 选择器组件
- 对话框组件
- 菜单组件
- 布局组件

#### TypeScript API 设计

```typescript
// ============================================================================
// 过程化 UI 组件 API
// ============================================================================

/**
 * UI 上下文 - 过程化开发的核心接口
 */
export interface UIContext {
  /**
   * 创建列表组件
   */
  list<T>(id: string, items: T[], renderer: ListItemRenderer<T>): ListComponent<T>;

  /**
   * 创建按钮
   */
  button(label: string, onClick: () => void, options?: ButtonOptions): ButtonComponent;

  /**
   * 创建输入框
   */
  input(value: string, onChange: (value: string) => void, options?: InputOptions): InputComponent;

  /**
   * 创建选择器
   */
  select<T>(
    value: T,
    options: T[],
    onChange: (value: T) => void,
    options?: SelectOptions<T>
  ): SelectComponent<T>;

  /**
   * 创建对话框
   */
  dialog(options: DialogOptions): DialogComponent;

  /**
   * 创建菜单
   */
  menu(items: MenuItem[], options?: MenuOptions): MenuComponent;

  /**
   * 布局组件
   */
  layout(direction: 'horizontal' | 'vertical', children: Component[]): LayoutComponent;
}

/**
 * 列表组件
 */
export interface ListComponent<T> {
  /** 更新列表数据 */
  updateItems(items: T[]): void;
  /** 滚动到指定项 */
  scrollTo(index: number): void;
  /** 获取选中项 */
  getSelected(): T | null;
  /** 设置选中项 */
  setSelected(index: number): void;
  /** 事件监听 */
  on(event: 'select' | 'scroll', handler: (data: unknown) => void): void;
}

/**
 * 列表项渲染器
 */
export type ListItemRenderer<T> = (item: T, index: number) => ListItemContent;

/**
 * 列表项内容
 */
export interface ListItemContent {
  /** 主文本 */
  text: string;
  /** 副文本 */
  subtitle?: string;
  /** 图标 */
  icon?: Buffer | string;
  /** 是否可选中 */
  selectable?: boolean;
}

/**
 * 按钮组件
 */
export interface ButtonComponent {
  /** 设置文本 */
  setText(text: string): void;
  /** 设置启用/禁用 */
  setEnabled(enabled: boolean): void;
  /** 点击事件 */
  onClick(handler: () => void): void;
}

/**
 * 按钮选项
 */
export interface ButtonOptions {
  /** 按钮样式 */
  style?: 'primary' | 'secondary' | 'danger';
  /** 按钮大小 */
  size?: 'small' | 'medium' | 'large';
  /** 是否禁用 */
  disabled?: boolean;
  /** 图标 */
  icon?: Buffer | string;
}

/**
 * 输入框组件
 */
export interface InputComponent {
  /** 获取值 */
  getValue(): string;
  /** 设置值 */
  setValue(value: string): void;
  /** 设置占位符 */
  setPlaceholder(placeholder: string): void;
  /** 设置启用/禁用 */
  setEnabled(enabled: boolean): void;
  /** 聚焦 */
  focus(): void;
  /** 失焦 */
  blur(): void;
}

/**
 * 输入框选项
 */
export interface InputOptions {
  /** 占位符文本 */
  placeholder?: string;
  /** 输入类型 */
  type?: 'text' | 'password' | 'number' | 'email';
  /** 是否多行 */
  multiline?: boolean;
  /** 最大长度 */
  maxLength?: number;
  /** 是否只读 */
  readOnly?: boolean;
}

/**
 * 选择器组件
 */
export interface SelectComponent<T> {
  /** 获取选中值 */
  getValue(): T | null;
  /** 设置选中值 */
  setValue(value: T): void;
  /** 更新选项列表 */
  updateOptions(options: T[]): void;
}

/**
 * 选择器选项
 */
export interface SelectOptions<T> {
  /** 显示文本提取函数 */
  getLabel?: (item: T) => string;
  /** 是否可搜索 */
  searchable?: boolean;
  /** 占位符 */
  placeholder?: string;
}

/**
 * 对话框组件
 */
export interface DialogComponent {
  /** 显示对话框 */
  show(): Promise<void>;
  /** 隐藏对话框 */
  hide(): Promise<void>;
  /** 设置内容 */
  setContent(content: DialogContent): void;
  /** 获取结果 */
  getResult(): unknown;
}

/**
 * 对话框选项
 */
export interface DialogOptions {
  /** 标题 */
  title?: string;
  /** 内容 */
  content?: DialogContent;
  /** 按钮配置 */
  buttons?: DialogButton[];
  /** 是否模态 */
  modal?: boolean;
  /** 大小 */
  size?: { width: number; height: number };
}

/**
 * 对话框内容
 */
export type DialogContent = 
  | { type: 'text'; data: string }
  | { type: 'custom'; renderer: WindowContentRenderer };

/**
 * 对话框按钮
 */
export interface DialogButton {
  label: string;
  style?: 'primary' | 'secondary' | 'danger';
  onClick: () => void | Promise<void>;
}

/**
 * 菜单组件
 */
export interface MenuComponent {
  /** 显示菜单 */
  show(x: number, y: number): Promise<void>;
  /** 隐藏菜单 */
  hide(): Promise<void>;
  /** 更新菜单项 */
  updateItems(items: MenuItem[]): void;
}

/**
 * 菜单项
 */
export interface MenuItem {
  /** 标签 */
  label: string;
  /** 图标 */
  icon?: Buffer | string;
  /** 是否禁用 */
  disabled?: boolean;
  /** 是否分隔符 */
  separator?: boolean;
  /** 子菜单 */
  submenu?: MenuItem[];
  /** 点击事件 */
  onClick?: () => void;
}

/**
 * 菜单选项
 */
export interface MenuOptions {
  /** 菜单样式 */
  style?: 'default' | 'compact';
}

/**
 * 布局组件
 */
export interface LayoutComponent {
  /** 添加子组件 */
  addChild(component: Component): void;
  /** 移除子组件 */
  removeChild(component: Component): void;
  /** 设置间距 */
  setSpacing(spacing: number): void;
  /** 设置对齐方式 */
  setAlign(align: 'start' | 'center' | 'end' | 'stretch'): void;
}

/**
 * 组件基类
 */
export type Component = 
  | ListComponent<unknown>
  | ButtonComponent
  | InputComponent
  | SelectComponent<unknown>
  | DialogComponent
  | MenuComponent
  | LayoutComponent;

// ============================================================================
// 组件构建器（Builder Pattern，可选）
// ============================================================================

/**
 * 组件构建器 - 提供链式 API
 */
export class ComponentBuilder {
  /**
   * 创建列表
   */
  list<T>(items: T[]): ListBuilder<T>;

  /**
   * 创建按钮
   */
  button(label: string): ButtonBuilder;

  /**
   * 创建输入框
   */
  input(): InputBuilder;

  /**
   * 创建选择器
   */
  select<T>(options: T[]): SelectBuilder<T>;
}

/**
 * 列表构建器
 */
export interface ListBuilder<T> {
  render(renderer: ListItemRenderer<T>): ListBuilder<T>;
  virtual(enabled: boolean): ListBuilder<T>;
  selectable(enabled: boolean): ListBuilder<T>;
  build(): ListComponent<T>;
}

/**
 * 按钮构建器
 */
export interface ButtonBuilder {
  style(style: ButtonOptions['style']): ButtonBuilder;
  size(size: ButtonOptions['size']): ButtonBuilder;
  icon(icon: Buffer | string): ButtonBuilder;
  onClick(handler: () => void): ButtonBuilder;
  build(): ButtonComponent;
}

/**
 * 输入框构建器
 */
export interface InputBuilder {
  placeholder(text: string): InputBuilder;
  type(type: InputOptions['type']): InputBuilder;
  maxLength(length: number): InputBuilder;
  onChange(handler: (value: string) => void): InputBuilder;
  build(): InputComponent;
}

/**
 * 选择器构建器
 */
export interface SelectBuilder<T> {
  placeholder(text: string): SelectBuilder<T>;
  searchable(enabled: boolean): SelectBuilder<T>;
  getLabel(fn: (item: T) => string): SelectBuilder<T>;
  onChange(handler: (value: T) => void): SelectBuilder<T>;
  build(): SelectComponent<T>;
}

// ============================================================================
// 便捷函数
// ============================================================================

/**
 * 创建 UI 上下文
 */
export function createUIContext(window: FloatingWindow): UIContext;

/**
 * 获取组件构建器
 */
export function getComponentBuilder(): ComponentBuilder;
```

---

## 实施计划

### 一期：基础功能（egui + winit）

#### Phase 1.1: 截图工具基础（2-3 周）
- [x] 实现基础截图功能（全屏/区域）
- [ ] 实现交互式区域选择
- [ ] 实现窗口吸附功能
- [ ] 实现选区调整（拖拽边缘调整大小）
- [ ] 实现选区变化回调
- [x] 实现像素颜色拾取
- [ ] 基础标注功能（画笔、矩形、箭头）
- [x] 实现截图保存（支持多种格式）

#### Phase 1.2: 长截图功能（1-2 周）
- [ ] 实现垂直滚动截图
- [ ] 实现图像拼接算法
- [ ] 实现智能去重
- [ ] 进度回调支持

#### Phase 1.3: 悬浮窗口基础（2-3 周）
- [ ] 实现窗口创建和管理
- [ ] 实现窗口拖拽
- [ ] 实现窗口形状（矩形、圆形、异形）
- [ ] 实现异形窗口透明区域鼠标穿透
- [ ] 实现图片显示
- [ ] 实现窗口图标支持（Emoji、预设图标）
- [ ] 实现边缘粒子特效（6种：旋转光环、脉冲波纹、流光溢彩、星尘飘散、电流闪烁、烟雾缭绕）
- [ ] 基础动画（淡入淡出）

#### Phase 1.4: 剪贴板管理基础（2 周）
- [ ] 实现剪贴板监听
- [ ] 实现历史记录存储
- [ ] 实现基础查询和搜索
- [ ] 实现历史记录 UI

#### Phase 1.5: 基础组件库（2-3 周）
- [ ] 实现列表组件
- [ ] 实现按钮组件
- [ ] 实现输入框组件
- [ ] 实现基础布局组件

### 二期：高级功能（egui + wgpu）

#### Phase 2.1: 高级标注功能（1-2 周）
- [ ] 实现更多标注工具（模糊、高亮等）
- [ ] 实现标注图层管理
- [ ] 实现撤销/重做功能

#### Phase 2.2: 高级窗口特效（2-3 周）
- [ ] 集成 wgpu 渲染后端
- [ ] 完善边缘粒子特效（所有6种特效的 GPU 加速版本）
- [ ] 实现复杂动画效果
- [ ] 实现自定义渲染
- [ ] 异形窗口性能优化
- [ ] 性能优化

#### Phase 2.3: 剪贴板高级功能（1-2 周）
- [ ] 实现标签系统
- [ ] 实现统计功能
- [ ] 实现导入/导出
- [ ] 敏感信息过滤

#### Phase 2.4: 组件库扩展（2 周）
- [ ] 实现更多组件（对话框、菜单等）
- [ ] 实现虚拟滚动
- [ ] 实现主题系统
- [ ] 性能优化

---

## 架构设计考虑

### 1. API 设计原则
- **类型安全**：完整的 TypeScript 类型定义
- **异步优先**：所有耗时操作使用 Promise
- **事件驱动**：支持事件监听和回调
- **过程化开发**：提供过程化 API，符合 roadmap 要求
- **可扩展性**：支持自定义渲染和扩展

### 2. 性能考虑
- **异步操作**：所有 I/O 操作异步化
- **资源管理**：及时释放窗口和资源
- **内存管理**：大图片使用流式处理
- **渲染优化**：虚拟滚动、按需渲染

### 3. 错误处理
- **统一错误类型**：定义标准错误接口
- **错误恢复**：关键操作支持重试
- **错误日志**：完整的错误日志记录

### 4. 扩展性
- **插件系统**：支持自定义标注工具
- **主题系统**：支持 UI 主题定制
- **国际化**：支持多语言（未来）

---

## 依赖项总结

### 一期依赖（egui + winit）
```toml
[package]
name = "bot"
version = "0.1.3"
edition = "2024"  # 使用 Rust 2024 Edition
rust-version = "1.85"  # Rust 2024 Edition 要求

[dependencies]
# 窗口管理
winit = "0.31"  # 最新版本
raw-window-handle = "0.6"  # 最新版本

# UI 框架
egui = "0.28"  # 最新版本
egui-winit = "0.28"  # 最新版本

# 渲染后端
softbuffer = "0.5"  # 最新版本，CPU 渲染

# 图像处理
image = "0.25"  # 最新版本
imageproc = "0.25"  # 最新版本，图像处理算法

# 图标支持
# 方案 1：使用 emoji（系统自带，无需额外依赖，推荐用于简单场景）
# 方案 2：内置 SVG 图标库（推荐，将常用图标打包为 SVG，使用 resvg 渲染）
resvg = "0.43"  # 最新版本，SVG 渲染库
# 方案 3：字体图标库（如 fontdue，用于字体渲染）
fontdue = "0.8"  # 最新版本，字体渲染（可选，用于图标和文本）

# 剪贴板
arboard = "3.5"  # 最新版本

# 数据库（剪贴板历史）
rusqlite = { version = "0.32", features = ["bundled"] }  # 最新版本

# 时间处理
chrono = { version = "0.4", features = ["serde"] }  # 最新版本

# 异步运行时
tokio = { version = "1.40", features = ["full"] }  # 最新版本

# 序列化
serde = { version = "1.0", features = ["derive"] }  # 最新版本
serde_json = "1.0"  # 最新版本

# 随机数生成（用于粒子特效）
rand = "0.8"  # 最新版本

# 噪声函数（用于粒子特效的随机路径）
noise = "0.9"  # 最新版本，Perlin 噪声等

# Node.js 绑定
napi = { version = "3.3", default-features = false, features = ["async"] }  # 最新版本
napi-derive = "3.3"  # 最新版本
```

### 二期依赖（egui + wgpu）
```toml
[dependencies]
# 在一期基础上添加：
wgpu = "0.21"  # 最新版本
egui-wgpu = "0.28"  # 最新版本，与 egui 版本对应
```

---

## 注意事项

1. **API 稳定性**：一期 API 设计要考虑二期扩展，避免破坏性变更
2. **性能测试**：每个功能模块需要进行性能测试
3. **跨平台兼容**：确保 Windows、macOS、Linux 都能正常工作
4. **文档完善**：每个 API 都需要完整的 JSDoc 注释和示例
5. **测试覆盖**：关键功能需要单元测试和集成测试

---

## 后续优化方向

1. **性能优化**：虚拟滚动、图像压缩、缓存策略
2. **用户体验**：快捷键支持、手势支持、主题定制
3. **功能扩展**：OCR 集成、图像识别、自动化脚本
4. **云同步**：剪贴板历史云同步（可选）
5. **插件系统**：支持第三方插件扩展功能
