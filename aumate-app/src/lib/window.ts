/**
 * Window Management - Rust Backend Implementation
 *
 * 所有窗口管理功能都通过 Rust 后端实现，包括动画。
 * 前端只需要简单的 invoke 调用。
 */

import { invoke } from "@tauri-apps/api/core";

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
