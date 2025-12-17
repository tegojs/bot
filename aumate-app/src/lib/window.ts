/**
 * Window Management - Re-exports from Services Layer
 *
 * 为了向后兼容，从 services 层重新导出
 * 新代码应该直接从 @/services/window 导入
 *
 * @deprecated 请直接从 @/services/window 导入
 */

export {
  animateResizeAndCenter,
  getWindowSize,
  resizeAndCenter,
  setVibrancy,
  setWindowSizeImmediate,
} from "@/services/window";
