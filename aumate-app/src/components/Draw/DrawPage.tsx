import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useCallback, useEffect, useRef } from "react";
import {
  useStateSubscriber,
  withStatePublisher,
} from "@/hooks/useStatePublisher";
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
import { ImageLayer } from "./ImageLayer";
import { SelectLayer } from "./SelectLayer";
import { DrawLayer } from "./DrawLayer";
import { Toolbar } from "./Toolbar";

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

  // 状态订阅
  const [, setCaptureStep, resetCaptureStep] = useStateSubscriber(
    CaptureStepPublisher,
    undefined,
  );
  const [, , resetDrawState] = useStateSubscriber(DrawStatePublisher, undefined);
  const [, , resetScreenshotType] = useStateSubscriber(
    ScreenshotTypePublisher,
    undefined,
  );
  const [, setCaptureLoading] = useStateSubscriber(
    CaptureLoadingPublisher,
    undefined,
  );
  const [, ] = useStateSubscriber(CaptureEventPublisher, undefined);

  // 容器引用
  const layerContainerRef = useRef<HTMLDivElement>(null);

  /**
   * 完成截图并清理
   */
  const finishCapture = useCallback(
    async (_clearScrollScreenshot = true) => {
      console.log("[DrawPage] Finishing capture");

      // 清理状态
      imageBufferRef.current = undefined;
      resetCaptureStep();
      resetDrawState();
      resetScreenshotType();
      drawPageStateRef.current = DrawPageState.WaitRelease;

      // 调用后端命令关闭窗口
      try {
        await invoke("close_draw_window");
      } catch (error) {
        console.error("[DrawPage] Failed to close window:", error);
      }
    },
    [resetCaptureStep, resetDrawState, resetScreenshotType],
  );

  /**
   * 显示窗口（窗口已经在后端创建时设置好大小）
   */
  const showWindow = useCallback(async () => {
    console.log("[DrawPage] Window is already shown by backend");

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
    console.log("[DrawPage] Executing screenshot");

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
      // biome-ignore lint/suspicious/noExplicitAny: ImageBuffer type compatibility
      imageBufferRef.current = {
        encoder: "png" as any,
        data: blob,
        bufferType: "pixels" as any,
        buffer: uint8Array.buffer,
      };

      console.log("[DrawPage] Screenshot captured, loading image...");

      // 从图片 blob 中读取尺寸
      const img = new Image();
      img.onload = async () => {
        const width = img.width;
        const height = img.height;

        console.log("[DrawPage] Image size:", width, "x", height);

        // 创建边界框信息
        const boundingBox = new CaptureBoundingBoxInfo(
          { min_x: 0, min_y: 0, max_x: width, max_y: height },
          [{ rect: { min_x: 0, min_y: 0, max_x: width, max_y: height }, monitorId: 0 }],
          new MousePosition(0, 0),
        );

        // 保存到 ref
        captureBoundingBoxInfoRef.current = boundingBox;

        // 通知各层截图边界框信息
        await imageLayerActionRef.current?.onCaptureBoundingBoxInfoReady(
          width,
          height,
        );
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
      console.error("[DrawPage] Screenshot error:", error);
      setCaptureLoading(false);
      await finishCapture();
    }
  }, [showWindow, setCaptureLoading, setCaptureStep, finishCapture]);

  /**
   * 监听截图事件
   */
  useEffect(() => {
    const unlisten = appWindowRef.current.listen("start-screenshot", () => {
      console.log("[DrawPage] Received start-screenshot event");
      executeScreenshot();
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, [executeScreenshot]);

  /**
   * 初始化
   */
  useEffect(() => {
    drawPageStateRef.current = DrawPageState.Active;
    console.log("[DrawPage] Component initialized");
  }, []);

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
   * ESC 键退出
   */
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        finishCapture();
      }
    };

    document.addEventListener("keydown", handleKeyDown);

    return () => {
      document.removeEventListener("keydown", handleKeyDown);
    };
  }, [finishCapture]);

  /**
   * 合成截图和绘图层
   */
  const compositeImage = useCallback(async (): Promise<Blob | null> => {
    // 从 ImageLayer 获取图像数据
    const imageCanvas = imageLayerActionRef.current?.getCanvas?.();
    if (!imageCanvas) {
      console.error("[DrawPage] Cannot get image canvas");
      return null;
    }

    // 获取选区，如果没有选区则使用整个画布
    const selectRect = selectLayerActionRef.current?.getSelectRect?.();
    const boundingBox = captureBoundingBoxInfoRef.current;

    // 确定裁剪区域
    let cropX: number, cropY: number, cropWidth: number, cropHeight: number;

    if (selectRect && selectRect.max_x > selectRect.min_x && selectRect.max_y > selectRect.min_y) {
      // 使用选区
      cropX = selectRect.min_x;
      cropY = selectRect.min_y;
      cropWidth = selectRect.max_x - selectRect.min_x;
      cropHeight = selectRect.max_y - selectRect.min_y;
    } else if (boundingBox) {
      // 没有选区，使用整个截图
      cropX = 0;
      cropY = 0;
      cropWidth = boundingBox.width;
      cropHeight = boundingBox.height;
    } else {
      console.warn("[DrawPage] No selection and no bounding box");
      return null;
    }

    // 创建临时 canvas 裁剪选区
    const canvas = document.createElement("canvas");
    const ctx = canvas.getContext("2d");
    if (!ctx) return null;

    canvas.width = cropWidth;
    canvas.height = cropHeight;

    // 裁剪选区（背景截图）
    ctx.drawImage(
      imageCanvas,
      cropX,
      cropY,
      cropWidth,
      cropHeight,
      0,
      0,
      cropWidth,
      cropHeight
    );

    // 获取 DrawLayer 的绘图内容
    const drawCoreAction = drawLayerActionRef.current?.getDrawCoreAction?.();
    if (drawCoreAction?.hasDrawings?.()) {
      console.log("[DrawPage] Compositing drawings...");
      const drawBlob = await drawCoreAction.exportToBlob?.({ mimeType: "image/png" });
      if (drawBlob) {
        // 将绘图层合成到画布上
        const drawImage = new Image();
        await new Promise<void>((resolve, reject) => {
          drawImage.onload = () => resolve();
          drawImage.onerror = reject;
          drawImage.src = URL.createObjectURL(drawBlob);
        });

        // 绘制到画布上（绘图层覆盖在截图上）
        ctx.drawImage(drawImage, 0, 0, cropWidth, cropHeight);
        URL.revokeObjectURL(drawImage.src);
      }
    }

    // 转换为 Blob
    const blob = await new Promise<Blob | null>((resolve) => {
      canvas.toBlob(resolve, "image/png");
    });

    return blob;
  }, []);

  /**
   * 保存截图
   */
  const handleSave = useCallback(async () => {
    console.log("[DrawPage] Saving screenshot...");

    try {
      const blob = await compositeImage();
      if (!blob) {
        console.error("[DrawPage] Failed to create composite image");
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
        console.log("[DrawPage] Save cancelled");
        return;
      }

      // 转换 Blob 到 ArrayBuffer
      const arrayBuffer = await blob.arrayBuffer();
      const uint8Array = new Uint8Array(arrayBuffer);

      // 使用 Tauri 文件系统 API 保存
      const { writeFile } = await import("@tauri-apps/plugin-fs");
      await writeFile(filePath, uint8Array);

      console.log("[DrawPage] Screenshot saved:", filePath);

      // 保存成功后关闭窗口
      await finishCapture();
    } catch (error) {
      console.error("[DrawPage] Save error:", error);
    }
  }, [finishCapture, compositeImage]);

  /**
   * 复制到剪贴板
   */
  const handleCopy = useCallback(async () => {
    console.log("[DrawPage] Copying to clipboard...");

    try {
      const blob = await compositeImage();
      if (!blob) {
        console.error("[DrawPage] Failed to create composite image");
        return;
      }

      // 使用 Clipboard API
      await navigator.clipboard.write([
        new ClipboardItem({
          "image/png": blob,
        }),
      ]);

      console.log("[DrawPage] Screenshot copied to clipboard");

      // 复制成功后关闭窗口
      await finishCapture();
    } catch (error) {
      console.error("[DrawPage] Copy error:", error);
    }
  }, [finishCapture, compositeImage]);

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
        <div className="draw-layer-wrap" style={{ width: "100%", height: "100%" }}>
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
          onUndo={() => drawLayerActionRef.current?.getDrawCoreAction()?.undo?.()}
          onRedo={() => drawLayerActionRef.current?.getDrawCoreAction()?.redo?.()}
          getSelectRect={() => selectLayerActionRef.current?.getSelectRect?.()}
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

