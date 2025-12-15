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

import type { ExcalidrawImperativeAPI } from "@excalidraw/excalidraw/types";
import { getCurrentWindow } from "@tauri-apps/api/window";
import type React from "react";
import {
  forwardRef,
  Suspense,
  useCallback,
  useContext,
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
  CaptureStepPublisher,
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
import { CaptureStep, type DrawLayerActionType, DrawState } from "./types";

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
  const [excalidrawAPI, setExcalidrawAPI] =
    useState<ExcalidrawImperativeAPI | null>(null);
  const [drawState, setDrawState] = useState<DrawState>(DrawState.Select);
  const [captureStep, setCaptureStep] = useState<CaptureStep>(
    CaptureStep.Select,
  );
  const drawCoreLayerRef = useRef<HTMLDivElement>(null);
  const apiInitializedRef = useRef(false);

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
  const [toolLocked, setToolLocked] = useState(false);
  const previousToolRef = useRef<DrawState>(DrawState.Select);

  // 获取 DrawState publisher context
  const drawStateContext = useContext(DrawStatePublisher.context);

  // 订阅状态变化
  useStateSubscriber(DrawStatePublisher, (state) => {
    if (state !== undefined) {
      setDrawState(state);
    }
  });

  useStateSubscriber(CaptureStepPublisher, (step) => {
    if (step !== undefined) {
      setCaptureStep(step);
    }
  });

  // 当进入绘图阶段时，自动设置默认工具为选择
  useEffect(() => {
    if (
      captureStep === CaptureStep.Draw &&
      excalidrawAPI &&
      apiInitializedRef.current
    ) {
      console.log(
        "[DrawLayer] Entering draw phase, setting default tool to selection",
      );
      setTimeout(() => {
        if (excalidrawAPI) {
          excalidrawAPI.setActiveTool({ type: "selection" });
        }
      }, 100);
    }
  }, [captureStep, excalidrawAPI]);

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

  // 初始化 API 后立即同步样式（但不设置工具，等到绘图阶段再设置）
  // biome-ignore lint/correctness/useExhaustiveDependencies: intentionally only run on API init
  useEffect(() => {
    if (!excalidrawAPI || apiInitializedRef.current) return;

    apiInitializedRef.current = true;
    console.log("[DrawLayer] API initialized, applying initial styles");

    // 给 Excalidraw 一点时间完成内部初始化
    setTimeout(() => {
      if (excalidrawAPI) {
        const roundness =
          cornerStyle === "round" ? { type: 2 as const } : { type: 3 as const };

        // 只设置样式，不设置工具
        // 工具会在用户完成选区创建进入绘图阶段后自动设置
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
            // biome-ignore lint/suspicious/noExplicitAny: Excalidraw appState types
          } as any,
        });

        console.log(
          "[DrawLayer] Initial styles applied (tool will be set when entering draw phase)",
        );
      }
    }, 300);
  }, [excalidrawAPI]);

  // Sync drawing parameters to Excalidraw
  useEffect(() => {
    if (!excalidrawAPI || !apiInitializedRef.current) return;

    // 转换边角样式：round -> { type: 2 }, sharp -> { type: 3 }
    const roundness =
      cornerStyle === "round" ? { type: 2 as const } : { type: 3 as const };

    try {
      // 直接更新样式
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
          // biome-ignore lint/suspicious/noExplicitAny: Excalidraw appState types
        } as any,
      });

      console.log("[DrawLayer] Styles updated:", {
        strokeColor,
        backgroundColor,
        strokeWidth,
        fillStyle,
        strokeStyle,
        roughness,
        opacity,
      });

      // 验证更新是否成功
      setTimeout(() => {
        const currentState = excalidrawAPI.getAppState();
        console.log("[DrawLayer] Verification - Excalidraw state:", {
          strokeColor: currentState.currentItemStrokeColor,
          backgroundColor: currentState.currentItemBackgroundColor,
          fillStyle: currentState.currentItemFillStyle,
          strokeWidth: currentState.currentItemStrokeWidth,
          strokeStyle: currentState.currentItemStrokeStyle,
        });
      }, 100);
    } catch (error) {
      console.error("[DrawLayer] Failed to update styles:", error);
    }
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

  // 处理图片工具的窗口置顶问题
  useEffect(() => {
    const handleImageTool = async () => {
      try {
        const window = getCurrentWindow();

        if (drawState === DrawState.Image) {
          // 选择图片工具时，临时禁用窗口置顶，让系统文件对话框能正常显示
          await window.setAlwaysOnTop(false);
          console.log("[DrawLayer] Disabled always-on-top for image selection");

          // 5秒后自动恢复置顶（防止用户忘记选择文件）
          setTimeout(async () => {
            if (drawState === DrawState.Image) {
              await window.setAlwaysOnTop(true);
              console.log("[DrawLayer] Re-enabled always-on-top after timeout");
            }
          }, 5000);
        } else {
          // 切换到其他工具时，恢复窗口置顶
          await window.setAlwaysOnTop(true);
        }
      } catch (error) {
        console.error("[DrawLayer] Failed to toggle always-on-top:", error);
      }
    };

    handleImageTool();
  }, [drawState]);

  // 当 DrawState 变化时，更新 Excalidraw 工具
  // 但只在绘图阶段生效，选区创建阶段不设置工具
  useEffect(() => {
    if (!excalidrawAPI) return;

    // 在选区创建阶段，不设置任何工具
    if (captureStep === CaptureStep.Select) {
      console.log("[DrawLayer] In selection phase, skipping tool activation");
      return;
    }

    const tool = mapDrawStateToExcalidrawTool(drawState);
    if (tool) {
      // 先更新样式
      const roundness =
        cornerStyle === "round" ? { type: 2 as const } : { type: 3 as const };

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
          // biome-ignore lint/suspicious/noExplicitAny: Excalidraw appState types
        } as any,
      });

      console.log("[DrawLayer] Tool changing to:", drawState, "with styles:", {
        strokeColor,
        backgroundColor,
        fillStyle,
        strokeWidth,
      });

      // 延迟切换工具，确保样式先生效
      setTimeout(() => {
        // biome-ignore lint/suspicious/noExplicitAny: Excalidraw tool type
        excalidrawAPI.setActiveTool(tool as any);

        // 记录当前工具（用于锁定功能）
        // 只记录绘图工具，不记录选择和抓手
        if (drawState !== DrawState.Select && drawState !== DrawState.Idle) {
          previousToolRef.current = drawState;
          console.log("[DrawLayer] Recorded tool for locking:", drawState);
        }
      }, 50);
    }
  }, [
    captureStep,
    drawState,
    excalidrawAPI,
    strokeColor,
    backgroundColor,
    strokeWidth,
    fillStyle,
    strokeStyle,
    roughness,
    cornerStyle,
    opacity,
    fontSize,
    fontFamily,
    textAlign,
    arrowStart,
    arrowEnd,
  ]);

  // 工具锁定功能：监听 Excalidraw 的工具变化，如果锁定则保持当前工具
  useEffect(() => {
    if (!excalidrawAPI || !toolLocked || !previousToolRef.current) return;

    const lockedTool = previousToolRef.current;

    // 只有当锁定的工具是绘图工具时，才进行锁定检查
    if (
      lockedTool === DrawState.Select ||
      lockedTool === (DrawState.Idle as DrawState)
    ) {
      return;
    }

    const checkInterval = setInterval(() => {
      const appState = excalidrawAPI.getAppState();
      const currentExcalidrawTool = appState.activeTool?.type;

      // 如果工具变回了 selection 或 hand，切回锁定的工具
      // （前面已经检查了 lockedTool 不是 Select 或 Idle）
      if (
        currentExcalidrawTool === "selection" ||
        currentExcalidrawTool === "hand"
      ) {
        const toolToRestore = mapDrawStateToExcalidrawTool(lockedTool);
        if (toolToRestore) {
          console.log("[DrawLayer] Tool locked, restoring:", lockedTool);
          // biome-ignore lint/suspicious/noExplicitAny: Excalidraw tool type
          excalidrawAPI.setActiveTool(toolToRestore as any);
        }
      }
    }, 100);

    return () => clearInterval(checkInterval);
  }, [excalidrawAPI, toolLocked]);

  // 控制 DrawLayer 的 pointer-events
  // 关键点：在选区创建阶段（CaptureStep.Select），DrawLayer 必须完全禁用
  // 这样 SelectLayer 才能独占鼠标事件来创建选区
  useEffect(() => {
    if (!drawCoreLayerRef.current) return;

    // 在选区创建阶段，禁用 DrawLayer 的所有交互
    if (captureStep === CaptureStep.Select) {
      drawCoreLayerRef.current.style.pointerEvents = "none";
      console.log("[DrawLayer] Disabled pointer-events during selection phase");
      return;
    }

    // 在绘图阶段，根据特定状态禁用画布
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
      // 绘图阶段默认启用 pointer-events
      drawCoreLayerRef.current.style.pointerEvents = "auto";
    }
  }, [captureStep, drawState]);

  // 暴露给父组件的方法
  useImperativeHandle(
    ref,
    () => ({
      async onCaptureReady() {},

      async onCaptureFinish() {
        // 清理 Excalidraw 场景
        if (excalidrawAPI) {
          // 切回选择工具
          excalidrawAPI.setActiveTool({ type: "selection" });
          excalidrawAPI.updateScene({
            elements: [],
            appState: {
              newElement: null,
              editingTextElement: null,
              selectedLinearElement: null,
              zoom: { value: 1 },
              scrollX: 0,
              scrollY: 0,
              // biome-ignore lint/suspicious/noExplicitAny: Excalidraw appState types
            } as any,
          });
          excalidrawAPI.history.clear();
        }
      },

      getExcalidrawAPI() {
        return excalidrawAPI;
      },

      getSelectedElementsCount() {
        if (!excalidrawAPI) return 0;
        const appState = excalidrawAPI.getAppState();
        const selectedIds = appState.selectedElementIds;
        return selectedIds ? Object.keys(selectedIds).length : 0;
      },

      getSelectedElementType() {
        if (!excalidrawAPI) return null;
        const appState = excalidrawAPI.getAppState();
        const selectedIds = appState.selectedElementIds;
        if (!selectedIds || Object.keys(selectedIds).length === 0) return null;

        const elements = excalidrawAPI.getSceneElements();
        // biome-ignore lint/suspicious/noExplicitAny: Excalidraw element types
        const selectedElement = elements.find((el: any) => selectedIds[el.id]);
        return selectedElement ? selectedElement.type : null;
      },

      // biome-ignore lint/suspicious/noExplicitAny: Excalidraw element props
      updateSelectedElements(props: any) {
        if (!excalidrawAPI) return;
        const appState = excalidrawAPI.getAppState();
        const selectedIds = appState.selectedElementIds;
        if (!selectedIds || Object.keys(selectedIds).length === 0) {
          console.warn("[DrawLayer] No elements selected to update");
          return;
        }

        const elements = excalidrawAPI.getSceneElements();
        // biome-ignore lint/suspicious/noExplicitAny: Excalidraw element types
        const updatedElements = elements.map((el: any) => {
          if (selectedIds[el.id]) {
            return { ...el, ...props };
          }
          return el;
        });

        excalidrawAPI.updateScene({
          elements: updatedElements,
        });

        console.log("[DrawLayer] Updated selected elements:", props);
      },

      canUndo() {
        if (!excalidrawAPI) return false;
        try {
          // Excalidraw 没有直接的 canUndo API
          // 简单判断：如果有元素就可以撤销
          const elements = excalidrawAPI.getSceneElements();
          return elements.length > 0;
        } catch (error) {
          console.error("[DrawLayer] Failed to check undo state:", error);
          return false;
        }
      },

      canRedo() {
        if (!excalidrawAPI) return false;
        try {
          // Excalidraw 没有直接的 canRedo API
          // 我们无法准确判断是否有重做历史，保守起见返回 true
          // 用户点击重做按钮时如果没有历史，Excalidraw 会自动忽略
          return true;
        } catch (error) {
          console.error("[DrawLayer] Failed to check redo state:", error);
          return false;
        }
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

          // 复制选中的元素
          copySelectedElements: () => {
            if (!excalidrawAPI) return;
            const appState = excalidrawAPI.getAppState();
            const selectedIds = appState.selectedElementIds;
            if (!selectedIds || Object.keys(selectedIds).length === 0) {
              console.warn("[DrawLayer] No elements selected to copy");
              return;
            }

            const elements = excalidrawAPI.getSceneElements();
            // biome-ignore lint/suspicious/noExplicitAny: Excalidraw element types
            const selectedElements = elements.filter(
              (el: any) => selectedIds[el.id],
            );

            // 复制元素（偏移位置避免完全重叠）
            // biome-ignore lint/suspicious/noExplicitAny: Excalidraw element types
            const copiedElements = selectedElements.map((el: any) => ({
              ...el,
              id: `${el.id}-copy-${Date.now()}`,
              x: el.x + 20,
              y: el.y + 20,
            }));

            excalidrawAPI.updateScene({
              elements: [...elements, ...copiedElements],
            });

            console.log(
              "[DrawLayer] Copied",
              copiedElements.length,
              "elements",
            );
          },

          // 删除选中的元素
          deleteSelectedElements: () => {
            if (!excalidrawAPI) return;
            const appState = excalidrawAPI.getAppState();
            const selectedIds = appState.selectedElementIds;
            if (!selectedIds || Object.keys(selectedIds).length === 0) {
              console.warn("[DrawLayer] No elements selected to delete");
              return;
            }

            const elements = excalidrawAPI.getSceneElements();
            // biome-ignore lint/suspicious/noExplicitAny: Excalidraw element types
            const remainingElements = elements.filter(
              (el: any) => !selectedIds[el.id],
            );

            excalidrawAPI.updateScene({
              elements: remainingElements,
              appState: {
                selectedElementIds: {},
              },
            });

            console.log(
              "[DrawLayer] Deleted",
              elements.length - remainingElements.length,
              "elements",
            );
          },

          // 图层操作
          bringToFront: () => {
            if (!excalidrawAPI) return;
            const appState = excalidrawAPI.getAppState();
            const selectedIds = appState.selectedElementIds;
            if (!selectedIds || Object.keys(selectedIds).length === 0) return;

            const elements = excalidrawAPI.getSceneElements();
            // biome-ignore lint/suspicious/noExplicitAny: Excalidraw element types
            const selected = elements.filter((el: any) => selectedIds[el.id]);
            // biome-ignore lint/suspicious/noExplicitAny: Excalidraw element types
            const others = elements.filter((el: any) => !selectedIds[el.id]);

            excalidrawAPI.updateScene({
              elements: [...others, ...selected],
            });
          },

          sendToBack: () => {
            if (!excalidrawAPI) return;
            const appState = excalidrawAPI.getAppState();
            const selectedIds = appState.selectedElementIds;
            if (!selectedIds || Object.keys(selectedIds).length === 0) return;

            const elements = excalidrawAPI.getSceneElements();
            // biome-ignore lint/suspicious/noExplicitAny: Excalidraw element types
            const selected = elements.filter((el: any) => selectedIds[el.id]);
            // biome-ignore lint/suspicious/noExplicitAny: Excalidraw element types
            const others = elements.filter((el: any) => !selectedIds[el.id]);

            excalidrawAPI.updateScene({
              elements: [...selected, ...others],
            });
          },

          bringForward: () => {
            if (!excalidrawAPI) return;
            const appState = excalidrawAPI.getAppState();
            const selectedIds = appState.selectedElementIds;
            if (!selectedIds || Object.keys(selectedIds).length === 0) return;

            // biome-ignore lint/suspicious/noExplicitAny: Excalidraw element types
            const elements = excalidrawAPI.getSceneElements() as any[];
            const newElements = [...elements];

            // 从后往前遍历，将选中的元素向后移动一位
            for (let i = newElements.length - 2; i >= 0; i--) {
              if (
                selectedIds[newElements[i].id] &&
                !selectedIds[newElements[i + 1]?.id]
              ) {
                [newElements[i], newElements[i + 1]] = [
                  newElements[i + 1],
                  newElements[i],
                ];
              }
            }

            excalidrawAPI.updateScene({
              elements: newElements,
            });
          },

          sendBackward: () => {
            if (!excalidrawAPI) return;
            const appState = excalidrawAPI.getAppState();
            const selectedIds = appState.selectedElementIds;
            if (!selectedIds || Object.keys(selectedIds).length === 0) return;

            // biome-ignore lint/suspicious/noExplicitAny: Excalidraw element types
            const elements = excalidrawAPI.getSceneElements() as any[];
            const newElements = [...elements];

            // 从前往后遍历，将选中的元素向前移动一位
            for (let i = 1; i < newElements.length; i++) {
              if (
                selectedIds[newElements[i].id] &&
                !selectedIds[newElements[i - 1]?.id]
              ) {
                [newElements[i], newElements[i - 1]] = [
                  newElements[i - 1],
                  newElements[i],
                ];
              }
            }

            excalidrawAPI.updateScene({
              elements: newElements,
            });
          },

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

  // 跟踪上次的 Excalidraw 工具状态和元素数量
  const lastExcalidrawToolRef = useRef<string | null>(null);
  const lastElementCountRef = useRef<number>(0);
  const isDrawingRef = useRef<boolean>(false);

  // 处理 Excalidraw 场景变化
  const handleChange = useCallback(
    // biome-ignore lint/suspicious/noExplicitAny: Excalidraw callback types
    (elements: readonly any[], appState: any, _files: unknown) => {
      const currentExcalidrawTool = appState.activeTool?.type;
      const currentElementCount = elements.length;

      // 监听 Excalidraw 工具变化，同步到我们的 DrawState
      if (
        currentExcalidrawTool &&
        currentExcalidrawTool !== lastExcalidrawToolRef.current
      ) {
        lastExcalidrawToolRef.current = currentExcalidrawTool;

        // 如果不是锁定状态，同步工具状态
        if (!toolLocked) {
          // Excalidraw 切换回 selection，我们也切回 Select
          if (
            currentExcalidrawTool === "selection" &&
            drawState !== DrawState.Select
          ) {
            console.log(
              "[DrawLayer] Excalidraw switched to selection, syncing DrawState",
            );
            drawStateContext.publish(DrawState.Select);
          }
          // Excalidraw 切换到 hand，我们切到 Idle
          else if (
            currentExcalidrawTool === "hand" &&
            drawState !== DrawState.Idle
          ) {
            console.log(
              "[DrawLayer] Excalidraw switched to hand, syncing DrawState",
            );
            drawStateContext.publish(DrawState.Idle);
          }
        }
      }

      // 检测绘制完成：如果没有锁定，并且正在使用绘图工具
      if (
        !toolLocked &&
        drawState !== DrawState.Select &&
        drawState !== DrawState.Idle &&
        currentExcalidrawTool !== "selection" &&
        currentExcalidrawTool !== "hand"
      ) {
        // 检测是否正在绘制
        const hasActiveElement =
          appState.editingElement ||
          appState.newElement ||
          appState.draggingElement ||
          appState.resizingElement ||
          (appState.selectedElementIds &&
            Object.keys(appState.selectedElementIds).length > 0);

        if (hasActiveElement) {
          isDrawingRef.current = true;
        } else if (isDrawingRef.current) {
          // 刚刚完成绘制，切回选择工具
          isDrawingRef.current = false;

          // 元素数量增加了，说明确实画了东西
          if (currentElementCount > lastElementCountRef.current) {
            console.log(
              "[DrawLayer] Drawing completed, switching to Select tool",
            );
            setTimeout(() => {
              if (excalidrawAPI && !toolLocked) {
                excalidrawAPI.setActiveTool({ type: "selection" });
                drawStateContext.publish(DrawState.Select);
              }
            }, 50);
          }
        }
      }

      lastElementCountRef.current = currentElementCount;
    },
    [toolLocked, drawState, excalidrawAPI, drawStateContext],
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
          excalidrawAPI={useCallback((api: ExcalidrawImperativeAPI) => {
            console.log("[DrawLayer] Excalidraw API initializing...");
            setExcalidrawAPI(api);
          }, [])}
          onChange={handleChange}
          handleKeyboardGlobally={true}
          initialData={{
            appState: {
              viewBackgroundColor: "#00000000",
              currentItemStrokeColor: "#e03131",
              currentItemBackgroundColor: "transparent",
              currentItemStrokeWidth: 2,
              currentItemFontSize: 20,
              currentItemFillStyle: "hachure",
              currentItemStrokeStyle: "solid",
              currentItemRoughness: 1,
              currentItemOpacity: 100,
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
              image: true, // 启用图像工具
            },
          }}
          viewModeEnabled={false}
          zenModeEnabled={false}
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

        /* 隐藏 Excalidraw 默认 UI 元素，但保留内部样式更新机制 */
        .draw-core-layer .excalidraw .App-toolbar-container,
        .draw-core-layer .excalidraw .App-menu,
        .draw-core-layer .excalidraw .ToolIcon__library,
        .draw-core-layer .excalidraw .layer-ui__wrapper__top-right,
        .draw-core-layer .excalidraw .layer-ui__wrapper__footer-center,
        .draw-core-layer .excalidraw .HelpDialog,
        .draw-core-layer .excalidraw .welcome-screen-center,
        .draw-core-layer .excalidraw .main-menu-trigger,
        .draw-core-layer .excalidraw .App-toolbar,
        .draw-core-layer .excalidraw .shapes-section,
        .draw-core-layer .excalidraw .undo-redo-buttons,
        .draw-core-layer .excalidraw .App-bottom-bar,
        .draw-core-layer .excalidraw .mobile-misc-tools-container,
        .draw-core-layer .excalidraw .scroll-back-to-content,
        .draw-core-layer .excalidraw .excalidraw-contextMenu {
          display: none !important;
        }

        /* 确保 Excalidraw 的对话框（如文件选择）显示在最上层 */
        .excalidraw .Modal,
        .excalidraw-modal-container,
        .excalidraw [role="dialog"],
        .excalidraw .layer-ui__wrapper__github-corner,
        input[type="file"]::file-selector-button {
          z-index: 999999 !important;
          position: relative !important;
        }
        
        /* Excalidraw 的文件输入框 */
        .draw-core-layer input[type="file"] {
          z-index: 999999 !important;
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
