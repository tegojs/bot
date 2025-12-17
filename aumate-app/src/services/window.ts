/**
 * Window Service - API Layer
 *
 * 封装所有与窗口管理相关的 Tauri 命令调用
 * 这是前端的 Service Layer，负责与后端 API 通信
 */

import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";

/**
 * 立即调整窗口大小并居中（无动画）
 *
 * 通过 Rust DDD 架构实现
 */
export async function resizeAndCenter(
  targetWidth: number,
  targetHeight: number,
): Promise<void> {
  const windowLabel = getCurrentWindow().label;
  await invoke("resize_and_center", {
    windowLabel,
    targetWidth,
    targetHeight,
  });
}

/**
 * 带动画的调整窗口大小并居中
 *
 * 通过 Rust DDD 架构实现（60fps 帧动画）
 */
export async function animateResizeAndCenter(
  targetWidth: number,
  targetHeight: number,
  duration: number = 300,
): Promise<void> {
  const windowLabel = getCurrentWindow().label;
  await invoke("animate_resize_and_center", {
    windowLabel,
    targetWidth,
    targetHeight,
    duration,
  });
}

/**
 * 立即设置窗口大小并居中
 *
 * 直接使用 Tauri Window API，用于 HTML 动画方案
 */
export async function setWindowSizeImmediate(
  targetWidth: number,
  targetHeight: number,
): Promise<void> {
  const window = getCurrentWindow();
  await window.setSize(new LogicalSize(targetWidth, targetHeight));
  await window.center();
}

/**
 * 设置窗口毛玻璃效果
 *
 * 通过 Rust DDD 架构实现
 *
 * @param enabled - 是否启用毛玻璃效果
 */
export async function setVibrancy(enabled: boolean): Promise<void> {
  const windowLabel = getCurrentWindow().label;
  await invoke("set_window_vibrancy", { windowLabel, enabled });
}

/**
 * 获取当前窗口大小（逻辑像素）
 */
export async function getWindowSize(): Promise<{
  width: number;
  height: number;
}> {
  const window = getCurrentWindow();
  const size = await window.innerSize();
  const scaleFactor = await window.scaleFactor();
  return {
    width: size.width / scaleFactor,
    height: size.height / scaleFactor,
  };
}
