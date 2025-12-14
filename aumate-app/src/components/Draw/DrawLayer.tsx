/**
 * DrawLayer - 绘图层
 * 使用 Excalidraw 提供绘图标注功能
 * 参考 snow-shot 的实现
 */

import {
  Excalidraw,
  exportToBlob,
  getCommonBounds,
} from "@excalidraw/excalidraw";
import type React from "react";
import {
  forwardRef,
  Suspense,
  useCallback,
  useEffect,
  useImperativeHandle,
  useRef,
  useState,
} from "react";
import { useStateSubscriber } from "@/hooks/useStatePublisher";
import {
  ArrowEndPublisher,
  ArrowStartPublisher,
  BackgroundColorPublisher,
  CornerStylePublisher,
  DrawStatePublisher,
  FillStylePublisher,
  FontFamilyPublisher,
  FontSizePublisher,
  OpacityPublisher,
  RoughnessPublisher,
  StrokeColorPublisher,
  StrokeStylePublisher,
  StrokeWidthPublisher,
  TextAlignPublisher,
  ToolLockedPublisher,
  zIndexs,
} from "./extra";
import { type DrawLayerActionType, DrawState } from "./types";

// biome-ignore lint/suspicious/noExplicitAny: Excalidraw API types are complex
type ExcalidrawAPI = any;

interface DrawLayerProps {
  style?: React.CSSProperties;
}

// 映射 DrawState 到 Excalidraw 工具类型
const mapDrawStateToExcalidrawTool = (
  state: DrawState,
): { type: string } | null => {
  switch (state) {
    case DrawState.Rect:
      return { type: "rectangle" };
    case DrawState.Diamond:
      return { type: "diamond" };
    case DrawState.Ellipse:
      return { type: "ellipse" };
    case DrawState.Arrow:
      return { type: "arrow" };
    case DrawState.Pen:
      return { type: "freedraw" };
    case DrawState.Text:
      return { type: "text" };
    case DrawState.Line:
      return { type: "line" };
    case DrawState.Select:
      return { type: "selection" };
    case DrawState.Idle:
      return { type: "hand" };
    case DrawState.Image:
      return { type: "image" };
    case DrawState.Eraser:
      return { type: "eraser" };
    case DrawState.Lock:
      // Lock 不是一个工具，而是状态切换，返回 null
      return null;
    default:
      return null;
  }
};

export const DrawLayer = forwardRef<
  DrawLayerActionType | undefined,
  DrawLayerProps
>((_props, ref) => {
  const [excalidrawAPI, setExcalidrawAPI] = useState<ExcalidrawAPI | null>(
    null,
  );
  const [drawState, setDrawState] = useState<DrawState>(DrawState.Idle);
  const drawCoreLayerRef = useRef<HTMLDivElement>(null);

  // Drawing parameters state
  const [strokeColor, setStrokeColor] = useState("#e03131");
  const [backgroundColor, setBackgroundColor] = useState("transparent");
  const [strokeWidth, setStrokeWidth] = useState(2);
  const [fontSize, setFontSize] = useState(20);
  const [fontFamily, setFontFamily] = useState(1);
  const [textAlign, setTextAlign] = useState("left");
  const [fillStyle, setFillStyle] = useState("hachure");
  const [strokeStyle, setStrokeStyle] = useState("solid");
  const [roughness, setRoughness] = useState(1);
  const [cornerStyle, setCornerStyle] = useState("round");
  const [opacity, setOpacity] = useState(100);
  const [arrowStart, setArrowStart] = useState<string | null>(null);
  const [arrowEnd, setArrowEnd] = useState<string | null>("arrow");
  const [, setToolLocked] = useState(false);

  // 订阅状态变化
  useStateSubscriber(DrawStatePublisher, (state) => {
    if (state !== undefined) {
      setDrawState(state);
    }
  });

  // Subscribe to drawing parameter changes
  useStateSubscriber(StrokeColorPublisher, setStrokeColor);
  useStateSubscriber(BackgroundColorPublisher, setBackgroundColor);
  useStateSubscriber(StrokeWidthPublisher, setStrokeWidth);
  useStateSubscriber(FontSizePublisher, setFontSize);
  useStateSubscriber(FontFamilyPublisher, setFontFamily);
  useStateSubscriber(TextAlignPublisher, setTextAlign);
  useStateSubscriber(FillStylePublisher, setFillStyle);
  useStateSubscriber(StrokeStylePublisher, setStrokeStyle);
  useStateSubscriber(RoughnessPublisher, setRoughness);
  useStateSubscriber(CornerStylePublisher, setCornerStyle);
  useStateSubscriber(OpacityPublisher, setOpacity);
  useStateSubscriber(ArrowStartPublisher, setArrowStart);
  useStateSubscriber(ArrowEndPublisher, setArrowEnd);
  useStateSubscriber(ToolLockedPublisher, setToolLocked);

  // Sync drawing parameters to Excalidraw
  useEffect(() => {
    if (!excalidrawAPI) return;

    // 转换边角样式：round -> { type: 2 }, sharp -> { type: 0 }
    const roundness = cornerStyle === "round" ? { type: 2 } : null;

    excalidrawAPI.updateScene({
      appState: {
        currentItemStrokeColor: strokeColor,
        currentItemBackgroundColor: backgroundColor,
        currentItemStrokeWidth: strokeWidth,
        currentItemFontSize: fontSize,
        currentItemFontFamily: fontFamily,
        currentItemTextAlign: textAlign,
        currentItemFillStyle: fillStyle,
        currentItemStrokeStyle: strokeStyle,
        currentItemRoughness: roughness,
        currentItemRoundness: roundness,
        currentItemOpacity: opacity,
        currentItemStartArrowhead: arrowStart,
        currentItemEndArrowhead: arrowEnd,
      },
    });
  }, [
    excalidrawAPI,
    strokeColor,
    backgroundColor,
    strokeWidth,
    fontSize,
    fontFamily,
    textAlign,
    fillStyle,
    strokeStyle,
    roughness,
    cornerStyle,
    opacity,
    arrowStart,
    arrowEnd,
  ]);

  // 当 DrawState 变化时，更新 Excalidraw 工具
  useEffect(() => {
    if (!excalidrawAPI) return;

    const tool = mapDrawStateToExcalidrawTool(drawState);
    if (tool) {
      // biome-ignore lint/suspicious/noExplicitAny: Excalidraw tool type
      excalidrawAPI.setActiveTool(tool as any);
    }
  }, [drawState, excalidrawAPI]);

  // 控制 DrawLayer 的 pointer-events（参考 snow-shot）
  // snow-shot 中 DrawLayer 默认是 auto，只在特定状态下禁用
  // SelectLayer 的 z-index (109) 高于 DrawLayer (105)，所以选区事件会先到达 SelectLayer
  // 当绘图工具激活时，SelectLayer 设置 pointer-events: none，让事件穿透到 DrawLayer
  useEffect(() => {
    if (!drawCoreLayerRef.current) return;

    // 只在 OCR、扫描二维码等特殊状态时禁用画布（参考 snow-shot）
    const disabledStates: DrawState[] = [
      // DrawState.OcrTranslate,
      // DrawState.OcrDetect,
      // DrawState.ScanQrcode,
      // DrawState.ExtraTools,
      // DrawState.VideoRecord,
      // DrawState.ScrollScreenshot,
    ];

    if (disabledStates.includes(drawState)) {
      drawCoreLayerRef.current.style.pointerEvents = "none";
    } else {
      // 默认启用 pointer-events
      drawCoreLayerRef.current.style.pointerEvents = "auto";
    }
  }, [drawState]);

  // 暴露给父组件的方法
  useImperativeHandle(
    ref,
    () => ({
      async onCaptureReady() {},

      async onCaptureFinish() {
        // 清理 Excalidraw 场景
        if (excalidrawAPI) {
          excalidrawAPI.setActiveTool({ type: "hand" });
          excalidrawAPI.updateScene({
            elements: [],
            appState: {
              newElement: null,
              editingTextElement: null,
              selectedLinearElement: null,
              zoom: { value: 1 },
              scrollX: 0,
              scrollY: 0,
            },
          });
          excalidrawAPI.history.clear();
        }
      },

      getExcalidrawAPI() {
        return excalidrawAPI;
      },

      getDrawCoreAction() {
        // 返回绘图核心操作
        const isMac = navigator.platform.toUpperCase().indexOf("MAC") >= 0;

        // 模拟键盘事件触发 Excalidraw 的 undo/redo
        const simulateKeyEvent = (shiftKey: boolean) => {
          const canvas = document.querySelector(".excalidraw");
          if (canvas) {
            const event = new KeyboardEvent("keydown", {
              key: "z",
              code: "KeyZ",
              ctrlKey: !isMac,
              metaKey: isMac,
              shiftKey,
              bubbles: true,
              cancelable: true,
            });
            canvas.dispatchEvent(event);
          }
        };

        return {
          undo: () => simulateKeyEvent(false),
          redo: () => simulateKeyEvent(true),
          clear: () => excalidrawAPI?.resetScene(),
          exportToBlob: async (opts?: {
            mimeType?: string;
            quality?: number;
          }): Promise<{
            blob: Blob;
            bounds: { minX: number; minY: number; maxX: number; maxY: number };
          } | null> => {
            if (!excalidrawAPI) return null;

            const elements = excalidrawAPI.getSceneElements();
            const appState = excalidrawAPI.getAppState();
            const files = excalidrawAPI.getFiles();

            if (elements.length === 0) {
              return null;
            }

            try {
              // 获取元素的边界框
              const [minX, minY, maxX, maxY] = getCommonBounds(elements);

              const blob = await exportToBlob({
                elements,
                appState: {
                  ...appState,
                  exportWithDarkMode: false,
                  exportBackground: false,
                },
                files,
                mimeType: opts?.mimeType || "image/png",
                quality: opts?.quality,
                exportPadding: 0, // 不添加额外的边距
              });
              return { blob, bounds: { minX, minY, maxX, maxY } };
            } catch (error) {
              console.error("[DrawLayer] Export error:", error);
              return null;
            }
          },
          getElements: () => excalidrawAPI?.getSceneElements() || [],
          hasDrawings: () => {
            const elements = excalidrawAPI?.getSceneElements() || [];
            return elements.length > 0;
          },
        };
      },
    }),
    [excalidrawAPI],
  );

  // 处理 Excalidraw 场景变化
  // biome-ignore lint/suspicious/noExplicitAny: Excalidraw callback types
  const handleChange = useCallback(
    (_elements: readonly unknown[], _appState: unknown, _files: unknown) => {
      // 场景变化时的处理
    },
    [],
  );

  return (
    <div
      ref={drawCoreLayerRef}
      className="draw-core-layer"
      style={{
        position: "fixed",
        top: 0,
        left: 0,
        width: "100vw",
        height: "100vh",
        zIndex: zIndexs.Draw_DrawCacheLayer, // 105，低于 SelectLayer 的 109
      }}
    >
      <Suspense fallback={null}>
        <Excalidraw
          excalidrawAPI={(api) => setExcalidrawAPI(api)}
          onChange={handleChange}
          handleKeyboardGlobally={true}
          initialData={{
            appState: {
              viewBackgroundColor: "#00000000",
              currentItemStrokeColor: "#e03131",
              currentItemStrokeWidth: 2,
              currentItemFontSize: 20,
            },
          }}
          UIOptions={{
            canvasActions: {
              loadScene: false,
              saveToActiveFile: false,
              export: false,
              clearCanvas: false,
              changeViewBackgroundColor: false,
            },
            tools: {
              image: false,
            },
          }}
          viewModeEnabled={false}
          zenModeEnabled={true}
          gridModeEnabled={false}
          theme="light"
        />
      </Suspense>

      <style>{`
        /* Excalidraw 容器样式 */
        .draw-core-layer .excalidraw,
        .draw-core-layer .excalidraw-container {
          width: 100% !important;
          height: 100% !important;
        }

        /* 隐藏 Excalidraw 默认 UI 元素 */
        .draw-core-layer .excalidraw .App-toolbar-container,
        .draw-core-layer .excalidraw .App-menu,
        .draw-core-layer .excalidraw .ToolIcon__library,
        .draw-core-layer .excalidraw .layer-ui__wrapper__top-right,
        .draw-core-layer .excalidraw .layer-ui__wrapper__footer,
        .draw-core-layer .excalidraw .HelpDialog,
        .draw-core-layer .excalidraw .welcome-screen-center,
        .draw-core-layer .excalidraw .main-menu-trigger,
        .draw-core-layer .excalidraw .App-toolbar,
        .draw-core-layer .excalidraw .shapes-section,
        .draw-core-layer .excalidraw .undo-redo-buttons,
        .draw-core-layer .excalidraw .App-bottom-bar,
        .draw-core-layer .excalidraw .mobile-misc-tools-container,
        .draw-core-layer .excalidraw .scroll-back-to-content {
          display: none !important;
        }

        /* 背景透明 */
        .draw-core-layer .excalidraw,
        .draw-core-layer .excalidraw .App-menu_top,
        .draw-core-layer .excalidraw .Island {
          background: transparent !important;
        }
      `}</style>
    </div>
  );
});

DrawLayer.displayName = "DrawLayer";
