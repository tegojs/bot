import { createPublisher } from "@/hooks/useStatePublisher";
import {
  type CanvasLayer,
  type CaptureBoundingBoxInfo,
  type CaptureEventData,
  CaptureStep,
  DrawState,
  type ElementRect,
  type ImageLayerActionType,
  MousePosition,
  ScreenshotType,
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
 */
export const DrawStatePublisher = createPublisher<DrawState>(DrawState.Idle);

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
export const DrawToolbarStatePublisher = createPublisher<any>(undefined, true);

/**
 * 元素拖拽发布者
 */
export const ElementDraggingPublisher = createPublisher<boolean>(false);

/**
 * Excalidraw 事件发布者
 */
export const ExcalidrawEventPublisher = createPublisher<any>(undefined, true);

/**
 * Excalidraw 橡皮擦事件发布者
 */
export const ExcalidrawOnHandleEraserPublisher = createPublisher<any>(
  undefined,
  true,
);

/**
 * 启用键盘事件发布者
 */
export const EnableKeyEventPublisher = createPublisher<boolean>(true);

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
  layer: CanvasLayer | undefined,
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
 * Z-Index 常量
 */
export const zIndexs = {
  Draw_ImageLayer: 1,
  Draw_DrawLayer: 2,
  Draw_SelectLayer: 3,
  Draw_Toolbar: 10,
  Draw_ColorPicker: 11,
  Draw_OcrBlocks: 12,
  Draw_StatusBar: 13,
  Draw_Cursor: 100,
};
