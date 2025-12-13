/**
 * Window Management - Rust Backend Implementation (DDD Architecture)
 *
 * 所有窗口管理功能都通过 DDD 架构实现：
 * Port (Domain Layer) → Adapter (Infrastructure Layer) → Use Case (Application Layer) → Command (API Layer)
 */

import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";

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
  const windowLabel = getCurrentWindow().label;
  await invoke("resize_and_center", {
    windowLabel,
    targetWidth,
    targetHeight,
  });
}

/**
 * Animate window resize and keep it centered
 *
 * 完全由 Rust 后端实现，包括动画逻辑和像素计算。
 * 使用 DDD 架构，业务逻辑在 Application Layer 的 Use Case 中。
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
  const windowLabel = getCurrentWindow().label;
  await invoke("animate_resize_and_center", {
    windowLabel,
    targetWidth,
    targetHeight,
    duration,
  });
}
