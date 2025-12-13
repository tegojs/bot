import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useCallback, useEffect, useRef, useState } from "react";
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
import styles from "./page.module.css";
import {
  type CaptureBoundingBoxInfo,
  type CaptureHistoryActionType,
  CaptureStep,
  type ColorPickerActionType,
  DrawContext,
  type DrawContextType,
  type DrawLayerActionType,
  DrawState,
  type DrawToolbarActionType,
  type ImageBuffer,
  type ImageLayerActionType,
  type ImageSharedBufferData,
  MousePosition,
  type OcrBlocksActionType,
  ScreenshotType,
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

  // 状态订阅
  const [getCaptureStep, setCaptureStep, resetCaptureStep] = useStateSubscriber(
    CaptureStepPublisher,
    undefined,
  );
  const [getDrawState, , resetDrawState] = useStateSubscriber(
    DrawStatePublisher,
    undefined,
  );
  const [getScreenshotType, setScreenshotType, resetScreenshotType] =
    useStateSubscriber(ScreenshotTypePublisher, undefined);
  const [, setCaptureLoading] = useStateSubscriber(
    CaptureLoadingPublisher,
    undefined,
  );
  const [getCaptureEvent, setCaptureEvent] = useStateSubscriber(
    CaptureEventPublisher,
    undefined,
  );

  // 容器引用
  const layerContainerRef = useRef<HTMLDivElement>(null);

  /**
   * 完成截图并清理
   */
  const finishCapture = useCallback(
    async (clearScrollScreenshot = true) => {
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

      // TODO: 调用后端截图 API
      // const imageBuffer = await invoke("capture_all_monitors", {...});
      // imageBufferRef.current = imageBuffer;

      setCaptureStep(CaptureStep.Select);
      setCaptureLoading(false);
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
        className={styles.layerContainer}
        ref={layerContainerRef}
        style={{
          position: "fixed",
          top: 0,
          left: 0,
          width: "100vw",
          height: "100vh",
          background: "rgba(0, 0, 0, 0.3)",
        }}
      >
        {/* 临时提示 */}
        <div
          style={{
            position: "absolute",
            top: "50%",
            left: "50%",
            transform: "translate(-50%, -50%)",
            color: "white",
            fontSize: "24px",
            textAlign: "center",
            pointerEvents: "none",
          }}
        >
          <div>截图编辑器</div>
          <div style={{ fontSize: "16px", marginTop: "20px" }}>
            Press ESC to exit
          </div>
          <div style={{ fontSize: "14px", marginTop: "10px", opacity: 0.7 }}>
            (Components are being implemented...)
          </div>
        </div>

        {/* TODO: 添加各个组件 */}
        {/* <ImageLayer actionRef={imageLayerActionRef} /> */}
        {/* <SelectLayer actionRef={selectLayerActionRef} /> */}
        {/* <DrawLayer actionRef={drawLayerActionRef} /> */}
        {/* <DrawToolbar actionRef={drawToolbarActionRef} /> */}
        {/* <ColorPicker actionRef={colorPickerActionRef} /> */}
        {/* <OcrBlocks actionRef={ocrBlocksActionRef} /> */}
        {/* <StatusBar /> */}

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
