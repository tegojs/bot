import React from "react";

// ============ 枚举类型 ============

/**
 * 截图步骤
 */
export enum CaptureStep {
  /** 选择阶段 */
  Select = 1,
  /** 绘制阶段 */
  Draw = 2,
  /** 固定阶段 */
  Fixed = 3,
}

/**
 * 画布层级
 */
export enum CanvasLayer {
  Draw = 1,
  Select = 2,
}

/**
 * 绘图状态
 */
export enum DrawState {
  Idle = 0,
  // 基础绘图工具
  Select = 1,
  Rect = 2,
  Diamond = 3,
  Ellipse = 4,
  Arrow = 5,
  Line = 6,
  Pen = 7,
  Text = 8,
  SerialNumber = 9,
  Blur = 10,
  Eraser = 11,
  Lock = 12,
  DrawExtraTools = 13,
  Watermark = 14,
  Highlight = 15,
  BlurFreeDraw = 16,
  // 操作
  Undo = 101,
  Redo = 102,
  Cancel = 103,
  Save = 104,
  FastSave = 105,
  Fixed = 106,
  Copy = 107,
  Confirm = 107001,
  // 高级功能
  OcrDetect = 108,
  OcrTranslate = 108001,
  ScrollScreenshot = 109,
  ExtraTools = 110,
  ScanQrcode = 111,
  LaserPointer = 112,
  MouseThrough = 113,
  VideoRecord = 114,
  DragWindow = 115,
  SaveToCloud = 116,
  ResetCanvas = 117,
}

/**
 * 截图类型
 */
export enum ScreenshotType {
  Normal = "normal",
  Delay = "delay",
  TopWindow = "top_window",
  FullScreen = "full_screen",
  SwitchCaptureHistory = "switch_capture_history",
}

/**
 * 图像编码器
 */
export enum ImageEncoder {
  Png = "png",
  WebP = "webp",
}

/**
 * 图像缓冲区类型
 */
export enum ImageBufferType {
  /** 原始像素数据 */
  Pixels = "pixels",
  /** 通过 SharedBuffer 传输 */
  SharedBuffer = "shared-buffer",
}

/**
 * 捕获事件类型
 */
export enum CaptureEvent {
  onExecuteScreenshot = "onExecuteScreenshot",
  onCaptureReady = "onCaptureReady",
  onCaptureLoad = "onCaptureLoad",
  onCaptureFinish = "onCaptureFinish",
  onCaptureImageBufferReady = "onCaptureImageBufferReady",
}

// ============ 接口类型 ============

/**
 * 矩形区域
 */
export interface ElementRect {
  min_x: number;
  min_y: number;
  max_x: number;
  max_y: number;
}

/**
 * 窗口元素
 */
export interface WindowElement {
  element_rect: ElementRect;
  window_id: number;
  title?: string;
  app_name?: string;
}

/**
 * 图像缓冲区
 */
export interface ImageBuffer {
  encoder: ImageEncoder;
  data: Blob;
  bufferType: ImageBufferType;
  buffer: ArrayBuffer;
}

/**
 * 共享缓冲区数据
 */
export interface ImageSharedBufferData {
  sharedBuffer: string;
  width: number;
  height: number;
  bufferType: ImageBufferType.SharedBuffer;
}

/**
 * 鼠标位置
 */
export class MousePosition {
  constructor(
    public mouseX: number,
    public mouseY: number,
  ) {}
}

/**
 * 截图边界框信息
 */
export class CaptureBoundingBoxInfo {
  constructor(
    public rect: ElementRect,
    public monitorRectList: Array<{ rect: ElementRect; monitorId: number }>,
    public mousePosition: MousePosition,
  ) {}

  get width(): number {
    return this.rect.max_x - this.rect.min_x;
  }

  get height(): number {
    return this.rect.max_y - this.rect.min_y;
  }
}

/**
 * 截图类型参数
 */
export interface ScreenshotTypeParams {
  type: ScreenshotType;
  params: {
    windowId?: string;
    captureHistoryId?: string;
  };
}

/**
 * 捕获事件数据
 */
export interface CaptureEventData {
  event: CaptureEvent;
  params?: any;
}

// ============ 组件 Action 接口 ============

/**
 * 图像层 Action 接口
 */
export interface ImageLayerActionType {
  onCaptureReady(
    imageSrc: string | undefined,
    imageBuffer: ImageBuffer | ImageSharedBufferData | undefined,
  ): Promise<void>;
  onCaptureLoad(
    imageSrc: string | undefined,
    imageBuffer: ImageBuffer | ImageSharedBufferData | undefined,
    boundingBox: CaptureBoundingBoxInfo,
  ): Promise<void>;
  onCaptureFinish(): Promise<void>;
  onCaptureBoundingBoxInfoReady(width: number, height: number): Promise<void>;
  onExecuteScreenshot(): Promise<void>;
  renderImageSharedBufferToPng(): Promise<ArrayBuffer | undefined>;
}

/**
 * 选择层 Action 接口
 */
export interface SelectLayerActionType {
  onCaptureReady(): Promise<void>;
  onCaptureLoad(): Promise<void>;
  onCaptureFinish(): Promise<void>;
  onCaptureBoundingBoxInfoReady(
    boundingBox: CaptureBoundingBoxInfo,
  ): Promise<void>;
  onExecuteScreenshot(): Promise<void>;
  getSelectRect(): ElementRect | undefined;
  getSelectRectParams():
    | { rect: ElementRect; offset: { x: number; y: number } }
    | undefined;
  getWindowId(): number | undefined;
}

/**
 * 绘图层 Action 接口
 */
export interface DrawLayerActionType {
  onCaptureReady(): Promise<void>;
  onCaptureFinish(): Promise<void>;
  getExcalidrawAPI(): any;
  getDrawCoreAction(): any;
}

/**
 * 工具栏 Action 接口
 */
export interface DrawToolbarActionType {
  setEnable(enable: boolean): void;
  onToolClick(tool: DrawState): void;
}

/**
 * 颜色选择器 Action 接口
 */
export interface ColorPickerActionType {
  pickColor(mousePosition: MousePosition): Promise<string | undefined>;
  setForceEnable(enable: boolean): void;
  getCurrentColor(): string | undefined;
}

/**
 * OCR 块 Action 接口
 */
export interface OcrBlocksActionType {
  getSelectedText(): any;
  getOcrResultAction(): any;
}

/**
 * 截图历史 Action 接口
 */
export interface CaptureHistoryActionType {
  switch(id: string): Promise<void>;
  getCurrentIndex(): number;
  getCurrentCaptureHistoryItem(): any;
  captureFullScreen(): Promise<void>;
  saveCurrentCapture(...args: any[]): Promise<void>;
}

// ============ Context 类型 ============

/**
 * 绘图上下文类型
 */
export interface DrawContextType {
  finishCapture: (clearScrollScreenshot?: boolean) => Promise<void>;
  imageLayerActionRef: React.RefObject<ImageLayerActionType | undefined>;
  selectLayerActionRef: React.RefObject<SelectLayerActionType | undefined>;
  imageBufferRef: React.RefObject<
    ImageBuffer | ImageSharedBufferData | undefined
  >;
  mousePositionRef: React.RefObject<MousePosition>;
  drawToolbarActionRef: React.RefObject<DrawToolbarActionType | undefined>;
  circleCursorRef: React.RefObject<HTMLDivElement | null>;
  drawLayerActionRef: React.RefObject<DrawLayerActionType | undefined>;
  ocrBlocksActionRef: React.RefObject<OcrBlocksActionType | undefined>;
  colorPickerActionRef: React.RefObject<ColorPickerActionType | undefined>;
  captureBoundingBoxInfoRef: React.RefObject<
    CaptureBoundingBoxInfo | undefined
  >;
  captureHistoryActionRef: React.RefObject<
    CaptureHistoryActionType | undefined
  >;
}

/**
 * 绘图上下文
 */
export const DrawContext = React.createContext<DrawContextType>({
  mousePositionRef: { current: new MousePosition(0, 0) },
  imageBufferRef: { current: undefined },
  finishCapture: () => Promise.resolve(),
  imageLayerActionRef: { current: undefined },
  selectLayerActionRef: { current: undefined },
  drawToolbarActionRef: { current: undefined },
  circleCursorRef: { current: null },
  drawLayerActionRef: { current: undefined },
  ocrBlocksActionRef: { current: undefined },
  colorPickerActionRef: { current: undefined },
  captureBoundingBoxInfoRef: { current: undefined },
  captureHistoryActionRef: { current: undefined },
});
