import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useCallback, useEffect, useRef } from "react";
import {
  useStateSubscriber,
  withStatePublisher,
} from "@/hooks/useStatePublisher";
import { log } from "@/utils/logger";
import { DrawLayer } from "./DrawLayer";
import {
  CaptureEventPublisher,
  CaptureLoadingPublisher,
  CaptureStepPublisher,
  DrawEventPublisher,
  DrawStatePublisher,
  DrawToolbarStatePublisher,
  ElementDraggingPublisher,
  EnableKeyEventPublisher,
  ExcalidrawEventPublisher,
  ExcalidrawOnHandleEraserPublisher,
  ScreenshotTypePublisher,
  zIndexs,
} from "./extra";
import { ImageLayer } from "./ImageLayer";
import { SelectLayer } from "./SelectLayer";
import { Sidebar } from "./Sidebar";
import { Toolbar } from "./Toolbar";
import {
  CaptureBoundingBoxInfo,
  type CaptureHistoryActionType,
  CaptureStep,
  type ColorPickerActionType,
  DrawContext,
  type DrawContextType,
  type DrawLayerActionType,
  // DrawState,
  type DrawToolbarActionType,
  type ImageBuffer,
  type ImageLayerActionType,
  type ImageSharedBufferData,
  MousePosition,
  type OcrBlocksActionType,
  // ScreenshotType,
  type SelectLayerActionType,
} from "./types";

/**
 * 页面状态
 */
enum DrawPageState {
  Init = "init",
  Active = "active",
  WaitRelease = "wait-release",
  Release = "release",
}

/**
 * 核心绘图页面组件
 */
const DrawPageCore: React.FC = () => {
  // Window 引用
  const appWindowRef = useRef(getCurrentWindow());

  // 各层级的 Action 引用
  const imageLayerActionRef = useRef<ImageLayerActionType | undefined>(
    undefined,
  );
  const selectLayerActionRef = useRef<SelectLayerActionType | undefined>(
    undefined,
  );
  const drawLayerActionRef = useRef<DrawLayerActionType | undefined>(undefined);
  const drawToolbarActionRef = useRef<DrawToolbarActionType | undefined>(
    undefined,
  );
  const colorPickerActionRef = useRef<ColorPickerActionType | undefined>(
    undefined,
  );
  const ocrBlocksActionRef = useRef<OcrBlocksActionType | undefined>(undefined);
  const captureHistoryActionRef = useRef<CaptureHistoryActionType | undefined>(
    undefined,
  );

  // 状态引用
  const imageBufferRef = useRef<
    ImageBuffer | ImageSharedBufferData | undefined
  >(undefined);
  const captureBoundingBoxInfoRef = useRef<CaptureBoundingBoxInfo | undefined>(
    undefined,
  );
  const mousePositionRef = useRef<MousePosition>(new MousePosition(0, 0));
  const circleCursorRef = useRef<HTMLDivElement>(null);
  const drawPageStateRef = useRef<DrawPageState>(DrawPageState.Init);
  // 存储加载的图片 URL 和尺寸，用于直接合成
  const loadedImageRef = useRef<{
    src: string;
    width: number;
    height: number;
  } | null>(null);
  // 标记是否已经开始截图（防止重复启动）
  const hasStartedRef = useRef(false);

  // 状态订阅
  const [, setCaptureStep, resetCaptureStep] = useStateSubscriber(
    CaptureStepPublisher,
    undefined,
  );
  const [, , resetDrawState] = useStateSubscriber(
    DrawStatePublisher,
    undefined,
  );
  const [, , resetScreenshotType] = useStateSubscriber(
    ScreenshotTypePublisher,
    undefined,
  );
  const [, setCaptureLoading] = useStateSubscriber(
    CaptureLoadingPublisher,
    undefined,
  );
  const [,] = useStateSubscriber(CaptureEventPublisher, undefined);

  // 容器引用
  const layerContainerRef = useRef<HTMLDivElement>(null);

  /**
   * 完成截图并清理
   */
  const finishCapture = useCallback(
    async (_clearScrollScreenshot = true) => {
      log.info("[DrawPage] Finishing capture");

      // 清理状态
      imageBufferRef.current = undefined;
      resetCaptureStep();
      resetDrawState();
      resetScreenshotType();
      drawPageStateRef.current = DrawPageState.WaitRelease;

      // 释放图片资源
      if (loadedImageRef.current?.src) {
        URL.revokeObjectURL(loadedImageRef.current.src);
      }
      loadedImageRef.current = null;

      // 重置启动标志，允许下次窗口显示时重新开始截图
      hasStartedRef.current = false;

      // 调用后端命令关闭窗口
      try {
        await invoke("close_draw_window");
      } catch (error) {
        log.error("[DrawPage] Failed to close window:", error);
      }
    },
    [resetCaptureStep, resetDrawState, resetScreenshotType],
  );

  /**
   * 显示窗口（窗口已经在后端创建时设置好大小）
   */
  const showWindow = useCallback(async () => {
    log.info("[DrawPage] Window is already shown by backend");

    // 窗口在后端创建时已经设置好位置和大小，这里不需要额外操作
    appWindowRef.current.setIgnoreCursorEvents(false);
    if (layerContainerRef.current) {
      layerContainerRef.current.style.opacity = "1";
    }
  }, []);

  /**
   * 执行截图
   */
  const executeScreenshot = useCallback(async () => {
    log.info("[DrawPage] Executing screenshot");

    drawPageStateRef.current = DrawPageState.Active;
    setCaptureLoading(true);

    try {
      // 显示窗口
      await showWindow();

      // 调用后端截图 API
      const imageBuffer = await invoke<number[]>("capture_all_monitors");

      // 转换为 Blob
      const uint8Array = new Uint8Array(imageBuffer);
      const blob = new Blob([uint8Array], { type: "image/png" });
      const imageSrc = URL.createObjectURL(blob);

      // 保存 imageBuffer 到 ref，用于后续保存/复制操作
      imageBufferRef.current = {
        // biome-ignore lint/suspicious/noExplicitAny: ImageBuffer type compatibility
        encoder: "png" as any,
        data: blob,
        // biome-ignore lint/suspicious/noExplicitAny: ImageBuffer type compatibility
        bufferType: "pixels" as any,
        buffer: uint8Array.buffer,
      };

      log.info("[DrawPage] Screenshot captured, loading image...");

      // 从图片 blob 中读取尺寸
      const img = new Image();
      img.onload = async () => {
        // 物理像素尺寸（实际图片大小）
        const physicalWidth = img.width;
        const physicalHeight = img.height;

        // CSS像素尺寸（屏幕坐标系，用于UI层）
        const dpr = window.devicePixelRatio || 1;
        const cssWidth = Math.round(physicalWidth / dpr);
        const cssHeight = Math.round(physicalHeight / dpr);

        log.info(
          "[DrawPage] Image physical size:",
          physicalWidth,
          "x",
          physicalHeight,
        );
        log.info("[DrawPage] Image CSS size:", cssWidth, "x", cssHeight);
        log.info("[DrawPage] devicePixelRatio:", dpr);

        // 保存加载的图片信息（物理像素尺寸，用于合成时直接使用）
        loadedImageRef.current = {
          src: imageSrc,
          width: physicalWidth,
          height: physicalHeight,
        };
        log.info("[DrawPage] Saved loadedImageRef:", loadedImageRef.current);

        // 创建边界框信息（使用CSS像素尺寸，与鼠标坐标系一致）
        const boundingBox = new CaptureBoundingBoxInfo(
          { min_x: 0, min_y: 0, max_x: cssWidth, max_y: cssHeight },
          [
            {
              rect: { min_x: 0, min_y: 0, max_x: cssWidth, max_y: cssHeight },
              monitorId: 0,
            },
          ],
          new MousePosition(0, 0),
        );

        // 保存到 ref
        captureBoundingBoxInfoRef.current = boundingBox;

        // 通知各层截图边界框信息
        // ImageLayer 使用物理像素（用于绘制全分辨率图像）
        await imageLayerActionRef.current?.onCaptureBoundingBoxInfoReady(
          physicalWidth,
          physicalHeight,
        );
        // SelectLayer 使用 CSS 像素（与鼠标坐标系一致）
        await selectLayerActionRef.current?.onCaptureBoundingBoxInfoReady(
          boundingBox,
        );

        // 加载图片到 ImageLayer
        await imageLayerActionRef.current?.onCaptureLoad(
          imageSrc,
          undefined,
          boundingBox,
        );

        // 通知 SelectLayer 准备好
        await selectLayerActionRef.current?.onCaptureLoad();

        // 更新状态
        setCaptureStep(CaptureStep.Select);
        setCaptureLoading(false);
      };
      img.src = imageSrc;
    } catch (error) {
      log.error("[DrawPage] Screenshot error:", error);
      setCaptureLoading(false);
      await finishCapture();
    }
  }, [showWindow, setCaptureLoading, setCaptureStep, finishCapture]);

  /**
   * 监听截图事件
   */
  useEffect(() => {
    const unlisten = appWindowRef.current.listen("start-screenshot", () => {
      log.info("[DrawPage] Received start-screenshot event");
      executeScreenshot();
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, [executeScreenshot]);

  /**
   * 初始化 - 组件挂载时自动开始截图
   */
  useEffect(() => {
    if (hasStartedRef.current) return;
    hasStartedRef.current = true;

    drawPageStateRef.current = DrawPageState.Active;
    log.info("[DrawPage] Component initialized, auto-starting screenshot");

    // 自动执行截图（不依赖事件，因为事件可能在组件挂载前发送）
    executeScreenshot();
  }, [executeScreenshot]);

  /**
   * 鼠标移动跟踪
   */
  useEffect(() => {
    const handleMouseMove = (e: MouseEvent) => {
      mousePositionRef.current = new MousePosition(e.clientX, e.clientY);
    };

    document.addEventListener("mousemove", handleMouseMove);

    return () => {
      document.removeEventListener("mousemove", handleMouseMove);
    };
  }, []);

  /**
   * 合成截图和绘图层
   */
  const compositeImage = useCallback(async (): Promise<{
    blob: Blob | null;
    canvas: HTMLCanvasElement;
  } | null> => {
    log.info("[DrawPage] compositeImage called");

    // 优先使用保存的图片信息
    const loadedImage = loadedImageRef.current;
    log.info("[DrawPage] loadedImageRef:", loadedImage);

    if (!loadedImage) {
      log.error("[DrawPage] No loaded image available");
      return null;
    }

    // 加载原始图片
    const sourceImage = new Image();
    await new Promise<void>((resolve, reject) => {
      sourceImage.onload = () => resolve();
      sourceImage.onerror = reject;
      sourceImage.src = loadedImage.src;
    });

    log.info(
      "[DrawPage] Source image size:",
      sourceImage.width,
      "x",
      sourceImage.height,
    );

    // 获取选区
    const selectRect = selectLayerActionRef.current?.getSelectRect?.();
    log.info("[DrawPage] selectRect:", selectRect);

    // 获取设备像素比（Retina 屏幕为 2）
    const dpr = window.devicePixelRatio || 1;
    log.info("[DrawPage] devicePixelRatio:", dpr);
    log.info(
      "[DrawPage] Screen CSS size:",
      window.screen.width,
      "x",
      window.screen.height,
    );

    // 后端返回的 PNG 已经是物理像素大小
    // 选区坐标是 CSS 像素，需要乘以 dpr 转换为物理像素

    // 确定裁剪区域
    let cropX: number, cropY: number, cropWidth: number, cropHeight: number;

    if (
      selectRect &&
      selectRect.max_x > selectRect.min_x &&
      selectRect.max_y > selectRect.min_y
    ) {
      // 选区坐标乘以 dpr 转换为物理像素坐标
      cropX = Math.round(selectRect.min_x * dpr);
      cropY = Math.round(selectRect.min_y * dpr);
      cropWidth = Math.round((selectRect.max_x - selectRect.min_x) * dpr);
      cropHeight = Math.round((selectRect.max_y - selectRect.min_y) * dpr);
      log.info("[DrawPage] Using selectRect (scaled by dpr):", {
        cropX,
        cropY,
        cropWidth,
        cropHeight,
      });
    } else {
      // 没有选区，使用整个图片
      cropX = 0;
      cropY = 0;
      cropWidth = sourceImage.width;
      cropHeight = sourceImage.height;
      log.info("[DrawPage] Using entire image:", {
        cropX,
        cropY,
        cropWidth,
        cropHeight,
      });
    }

    // 确保裁剪区域不超出图片边界
    cropX = Math.max(0, Math.min(cropX, sourceImage.width - 1));
    cropY = Math.max(0, Math.min(cropY, sourceImage.height - 1));
    cropWidth = Math.min(cropWidth, sourceImage.width - cropX);
    cropHeight = Math.min(cropHeight, sourceImage.height - cropY);

    log.info("[DrawPage] Final crop:", { cropX, cropY, cropWidth, cropHeight });

    if (cropWidth <= 0 || cropHeight <= 0) {
      log.error("[DrawPage] Invalid crop dimensions");
      return null;
    }

    // 创建临时 canvas 裁剪选区
    const canvas = document.createElement("canvas");
    const ctx = canvas.getContext("2d");
    if (!ctx) return null;

    canvas.width = cropWidth;
    canvas.height = cropHeight;

    // 裁剪选区（从原始图片）
    ctx.drawImage(
      sourceImage,
      cropX,
      cropY,
      cropWidth,
      cropHeight,
      0,
      0,
      cropWidth,
      cropHeight,
    );

    // 获取 DrawLayer 的绘图内容
    const drawCoreAction = drawLayerActionRef.current?.getDrawCoreAction?.();
    if (drawCoreAction?.hasDrawings?.()) {
      log.info("[DrawPage] Compositing drawings...");
      const exportResult = await drawCoreAction.exportToBlob?.({
        mimeType: "image/png",
      });
      if (exportResult?.blob && exportResult?.bounds) {
        const { blob: drawBlob, bounds } = exportResult;

        // 将绘图层合成到画布上
        const drawImage = new Image();
        await new Promise<void>((resolve, reject) => {
          drawImage.onload = () => resolve();
          drawImage.onerror = reject;
          drawImage.src = URL.createObjectURL(drawBlob);
        });

        // 计算绘图内容相对于选区的偏移
        // bounds 是绘图元素在屏幕坐标系中的边界框（CSS像素）
        // cropX, cropY 是选区在物理像素坐标系中的起始位置
        // 需要将 bounds 也转换为物理像素坐标
        const scaledBoundsMinX = bounds.minX * dpr;
        const scaledBoundsMinY = bounds.minY * dpr;
        const drawX = scaledBoundsMinX - cropX;
        const drawY = scaledBoundsMinY - cropY;
        // 绘图导出的尺寸也需要缩放
        const drawWidth = (bounds.maxX - bounds.minX) * dpr;
        const drawHeight = (bounds.maxY - bounds.minY) * dpr;

        log.info("[DrawPage] Drawing bounds (original):", bounds);
        log.info("[DrawPage] Drawing bounds (scaled by dpr):", {
          scaledBoundsMinX,
          scaledBoundsMinY,
          drawWidth,
          drawHeight,
        });
        log.info("[DrawPage] Crop rect:", {
          cropX,
          cropY,
          cropWidth,
          cropHeight,
        });
        log.info("[DrawPage] Draw position:", {
          drawX,
          drawY,
          drawWidth,
          drawHeight,
        });

        // 绘制到画布上（按正确的位置和大小）
        ctx.drawImage(drawImage, drawX, drawY, drawWidth, drawHeight);
        URL.revokeObjectURL(drawImage.src);
      }
    }

    // 返回 canvas 和 blob（用于不同的导出需求）
    const blob = await new Promise<Blob | null>((resolve) => {
      canvas.toBlob(resolve, "image/png");
    });

    return { blob, canvas };
  }, []);

  /**
   * 保存截图
   */
  const handleSave = useCallback(async () => {
    log.info("[DrawPage] handleSave called - button clicked!");
    log.info("[DrawPage] Saving screenshot...");

    try {
      const result = await compositeImage();
      if (!result?.blob) {
        log.error("[DrawPage] Failed to create composite image");
        return;
      }

      // 调用 Tauri 保存对话框
      const { save } = await import("@tauri-apps/plugin-dialog");
      const filePath = await save({
        defaultPath: `screenshot-${Date.now()}.png`,
        filters: [
          {
            name: "PNG Image",
            extensions: ["png"],
          },
        ],
      });

      if (!filePath) {
        log.info("[DrawPage] Save cancelled");
        return;
      }

      // 转换 Blob 到 ArrayBuffer
      const arrayBuffer = await result.blob.arrayBuffer();
      const uint8Array = new Uint8Array(arrayBuffer);

      // 使用 Tauri 文件系统 API 保存
      const { writeFile } = await import("@tauri-apps/plugin-fs");
      await writeFile(filePath, uint8Array);

      log.info("[DrawPage] Screenshot saved:", filePath);

      // 保存成功后关闭窗口
      await finishCapture();
    } catch (error) {
      log.error("[DrawPage] Save error:", error);
    }
  }, [finishCapture, compositeImage]);

  /**
   * 复制到剪贴板
   */
  const handleCopy = useCallback(async () => {
    log.info("[DrawPage] handleCopy called - button clicked!");
    log.info("[DrawPage] Copying to clipboard...");

    try {
      const result = await compositeImage();
      log.info("[DrawPage] compositeImage result:", result);

      if (!result?.canvas) {
        log.error("[DrawPage] Failed to create composite image");
        return;
      }

      const { canvas } = result;

      // 将 canvas 转换为 PNG base64（比原始像素数组更高效）
      const dataUrl = canvas.toDataURL("image/png");
      // 去掉 data:image/png;base64, 前缀
      const base64Data = dataUrl.split(",")[1];
      log.info("[DrawPage] PNG base64 length:", base64Data.length);

      // 使用优化后的 PNG 命令写入剪贴板
      log.info("[DrawPage] Writing to clipboard via invoke...");
      await invoke("write_clipboard_image_png", {
        pngBase64: base64Data,
      });

      log.info("[DrawPage] Screenshot copied to clipboard successfully!");

      // 复制成功后关闭窗口
      await finishCapture();
    } catch (error) {
      log.error("[DrawPage] Copy error:", error);
    }
  }, [finishCapture, compositeImage]);

  /**
   * 键盘快捷键处理
   * - ESC: 退出截图
   * - Enter: 复制到剪贴板并退出
   */
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        finishCapture();
      } else if (e.key === "Enter") {
        // 只在选区完成后才触发复制
        const selectRect = selectLayerActionRef.current?.getSelectRect?.();
        if (
          selectRect &&
          selectRect.max_x - selectRect.min_x > 0 &&
          selectRect.max_y - selectRect.min_y > 0
        ) {
          handleCopy();
        }
      }
    };

    document.addEventListener("keydown", handleKeyDown);

    return () => {
      document.removeEventListener("keydown", handleKeyDown);
    };
  }, [finishCapture, handleCopy]);

  // 构建 DrawContext
  const drawContextValue: DrawContextType = {
    finishCapture,
    imageLayerActionRef,
    selectLayerActionRef,
    imageBufferRef,
    mousePositionRef,
    drawToolbarActionRef,
    circleCursorRef,
    drawLayerActionRef,
    ocrBlocksActionRef,
    colorPickerActionRef,
    captureBoundingBoxInfoRef,
    captureHistoryActionRef,
  };

  return (
    <DrawContext.Provider value={drawContextValue}>
      <div
        ref={layerContainerRef}
        className="layer-container"
        style={{ position: "fixed", top: 0, left: 0 }}
      >
        {/* 图像层和绘图层包在一起 */}
        <div
          className="draw-layer-wrap"
          style={{ width: "100%", height: "100%" }}
        >
          <ImageLayer ref={imageLayerActionRef} />
          <DrawLayer ref={drawLayerActionRef} />
        </div>

        {/* 选择层 - z-index 最高 */}
        <SelectLayer ref={selectLayerActionRef} />

        {/* 工具栏 */}
        <Toolbar
          onSave={handleSave}
          onCopy={handleCopy}
          onClose={finishCapture}
          onUndo={() =>
            drawLayerActionRef.current?.getDrawCoreAction()?.undo?.()
          }
          onRedo={() =>
            drawLayerActionRef.current?.getDrawCoreAction()?.redo?.()
          }
          getSelectRect={() => selectLayerActionRef.current?.getSelectRect?.()}
        />

        {/* 侧边栏 */}
        <Sidebar
          getSelectRect={() => selectLayerActionRef.current?.getSelectRect?.()}
          onSendToBack={() => {
            const api = drawLayerActionRef.current?.getExcalidrawAPI?.();
            if (api) {
              // TODO: 实现图层操作
              log.info("[DrawPage] Send to back");
            }
          }}
          onSendBackward={() => {
            const api = drawLayerActionRef.current?.getExcalidrawAPI?.();
            if (api) {
              log.info("[DrawPage] Send backward");
            }
          }}
          onBringForward={() => {
            const api = drawLayerActionRef.current?.getExcalidrawAPI?.();
            if (api) {
              log.info("[DrawPage] Bring forward");
            }
          }}
          onBringToFront={() => {
            const api = drawLayerActionRef.current?.getExcalidrawAPI?.();
            if (api) {
              log.info("[DrawPage] Bring to front");
            }
          }}
        />

        <div ref={circleCursorRef} style={{ zIndex: zIndexs.Draw_Cursor }} />
      </div>
    </DrawContext.Provider>
  );
};

/**
 * 使用状态发布者包装的组件
 */
const DrawPageContent = withStatePublisher(
  DrawPageCore,
  CaptureStepPublisher,
  DrawStatePublisher,
  CaptureLoadingPublisher,
  EnableKeyEventPublisher,
  ExcalidrawEventPublisher,
  CaptureEventPublisher,
  ExcalidrawOnHandleEraserPublisher,
  ScreenshotTypePublisher,
  DrawEventPublisher,
  DrawToolbarStatePublisher,
  ElementDraggingPublisher,
);

/**
 * 绘图页面主组件
 */
export const DrawPage: React.FC = () => {
  return <DrawPageContent />;
};
