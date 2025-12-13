/**
 * Window Management - Rust Backend Implementation
 *
 * 所有窗口管理功能都通过 Rust 后端实现，包括动画。
 * 前端只需要简单的 invoke 调用。
 */

import { invoke } from "@tauri-apps/api/core";

/**
 * Resize window and keep it centered (no animation)
 *
 * 立即调整窗口大小并居中，无动画效果。
 * 适用于初始化或需要立即响应的场景。
 *
 * @param targetWidth Target width (logical pixels)
 * @param targetHeight Target height (logical pixels)
 */
export async function resizeAndCenter(
  targetWidth: number,
  targetHeight: number,
): Promise<void> {
  await invoke("resize_and_center", {
    targetWidth,
    targetHeight,
  });
}

/**
 * Animate window resize and keep it centered
 *
 * 完全由 Rust 后端实现，包括动画逻辑和像素计算。
 *
 * @param targetWidth Target width (logical pixels)
 * @param targetHeight Target height (logical pixels)
 * @param duration Animation duration (ms), default 300ms
 */
export async function animateResizeAndCenter(
  targetWidth: number,
  targetHeight: number,
  duration: number = 300,
): Promise<void> {
  await invoke("animate_resize_and_center", {
    targetWidth,
    targetHeight,
    duration,
  });
}
