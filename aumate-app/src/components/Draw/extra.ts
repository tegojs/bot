import type {
  Arrowhead,
  FillStyle,
  FontFamilyValues,
  StrokeRoundness,
  StrokeStyle,
  TextAlign,
} from "@excalidraw/excalidraw/element/types";
import { createPublisher } from "@/hooks/useStatePublisher";
import {
  type CanvasLayer,
  type CaptureBoundingBoxInfo,
  type CaptureEventData,
  CaptureStep,
  DrawState,
  // type ElementRect,
  type ImageLayerActionType,
  MousePosition,
  // ScreenshotType,
  type ScreenshotTypeParams,
  type SelectLayerActionType,
} from "./types";

// ============ 状态发布者 ============

/**
 * 捕获步骤发布者
 */
export const CaptureStepPublisher = createPublisher<CaptureStep>(
  CaptureStep.Select,
);

/**
 * 绘图状态发布者
 * 默认为 Select（选择工具）
 */
export const DrawStatePublisher = createPublisher<DrawState>(DrawState.Select);

/**
 * 截图类型发布者
 */
export const ScreenshotTypePublisher = createPublisher<
  ScreenshotTypeParams | undefined
>(undefined, true);

/**
 * 捕获事件发布者
 */
export const CaptureEventPublisher = createPublisher<
  CaptureEventData | undefined
>(undefined, true);

/**
 * 捕获加载状态发布者
 */
export const CaptureLoadingPublisher = createPublisher<boolean>(false);

/**
 * 绘图事件发布者
 */
export const DrawEventPublisher = createPublisher<string | undefined>(
  undefined,
  true,
);

/**
 * 工具栏状态发布者
 */
// biome-ignore lint/suspicious/noExplicitAny: Dynamic toolbar state
export const DrawToolbarStatePublisher = createPublisher<any>(undefined, true);

/**
 * 元素拖拽发布者
 */
export const ElementDraggingPublisher = createPublisher<boolean>(false);

/**
 * Excalidraw 事件发布者
 */
// biome-ignore lint/suspicious/noExplicitAny: Excalidraw event types are complex
export const ExcalidrawEventPublisher = createPublisher<any>(undefined, true);

/**
 * Excalidraw 橡皮擦事件发布者
 */
// biome-ignore lint/suspicious/noExplicitAny: Excalidraw eraser event types
export const ExcalidrawOnHandleEraserPublisher = createPublisher<any>(
  undefined,
  true,
);

/**
 * 启用键盘事件发布者
 */
export const EnableKeyEventPublisher = createPublisher<boolean>(true);

// ============ 绘图参数发布者 ============

/**
 * 线条颜色发布者
 */
export const StrokeColorPublisher = createPublisher<string>("#e03131");

/**
 * 背景颜色发布者
 */
export const BackgroundColorPublisher = createPublisher<string>("transparent");

/**
 * 填充颜色发布者 (兼容旧代码)
 */
export const FillColorPublisher = BackgroundColorPublisher;

/**
 * 填充模式发布者: 'hachure' | 'cross-hatch' | 'solid' | 'zigzag'
 */
export const FillStylePublisher = createPublisher<FillStyle>("hachure");

/**
 * 线条粗细发布者
 */
export const StrokeWidthPublisher = createPublisher<number>(2);

/**
 * 边框样式发布者: 'solid' | 'dashed' | 'dotted'
 */
export const StrokeStylePublisher = createPublisher<StrokeStyle>("solid");

/**
 * 线条粗糙度/风格发布者: 0=建筑/直线, 1=手绘, 2=卡通
 */
export const RoughnessPublisher = createPublisher<number>(1);

/**
 * 边角样式发布者: 'sharp' | 'round'
 */
export const CornerStylePublisher = createPublisher<StrokeRoundness>("round");

/**
 * 字体大小发布者
 */
export const FontSizePublisher = createPublisher<number>(20);

/**
 * 字体系列发布者 (1=手写体, 2=普通, 3=代码, 4=第四种字体)
 */
export const FontFamilyPublisher = createPublisher<FontFamilyValues>(1);

/**
 * 文本对齐发布者: 'left' | 'center' | 'right'
 */
export const TextAlignPublisher = createPublisher<TextAlign>("left");

/**
 * 箭头起点发布者: null | 'arrow' | 'dot' | 'bar' | 其他 Arrowhead 类型
 */
export const ArrowStartPublisher = createPublisher<Arrowhead | null>(null);

/**
 * 箭头终点发布者: null | 'arrow' | 'dot' | 'bar' | 其他 Arrowhead 类型
 */
export const ArrowEndPublisher = createPublisher<Arrowhead | null>("arrow");

/**
 * 透明度发布者 (0-100)
 */
export const OpacityPublisher = createPublisher<number>(100);

/**
 * 工具锁定发布者
 */
export const ToolLockedPublisher = createPublisher<boolean>(false);

// ============ 辅助类 ============

/**
 * 捕获边界框信息类（从 types 导出，这里重新导出）
 */
export { type CaptureBoundingBoxInfo, MousePosition };

// ============ 辅助函数 ============

/**
 * 切换画布层级
 */
export function switchLayer(
  _layer: CanvasLayer | undefined,
  imageLayerAction: ImageLayerActionType | undefined,
  selectLayerAction: SelectLayerActionType | undefined,
) {
  if (!imageLayerAction || !selectLayerAction) {
    return;
  }

  // 根据层级切换显示/隐藏
  // 这里的具体实现需要根据实际的 ImageLayer 和 SelectLayer 组件来定
  // 目前先占位
}

/**
 * 监听器 ID 生成器
 */
let listenerIdCounter = 0;
export function generateListenerId(): number {
  return listenerIdCounter++;
}

// ============ 常量 ============

/**
 * Z-Index 常量 (参考 snow-shot 的架构)
 * 层级顺序: ImageLayer < DrawLayer < SelectLayer < UI 元素
 */
export const zIndexs = {
  Draw_ImageLayer: 102,
  Draw_BlurImageLayer: 103,
  Draw_DrawLayer: 104,
  Draw_DrawCacheLayer: 105,
  Draw_SelectLayer: 109,
  Draw_Toolbar: 204,
  Draw_ColorPicker: 206,
  Draw_OcrBlocks: 207,
  Draw_StatusBar: 208,
  Draw_Cursor: 300,
};
