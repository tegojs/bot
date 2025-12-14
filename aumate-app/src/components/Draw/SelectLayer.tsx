/**
 * SelectLayer - 选择层
 * 负责矩形选区绘制、鼠标交互、智能窗口识别、8点调整手柄
 */

import { invoke } from "@tauri-apps/api/core";
import Flatbush from "flatbush";
import type React from "react";
import {
  forwardRef,
  useCallback,
  useContext,
  useEffect,
  useImperativeHandle,
  useRef,
  useState,
} from "react";
import { useStateSubscriber } from "@/hooks/useStatePublisher";
import { DrawStatePublisher, zIndexs } from "./extra";
import type {
  CaptureBoundingBoxInfo,
  ElementRect,
  SelectLayerActionType,
} from "./types";
import { DrawContext, DrawState } from "./types";

interface SelectLayerProps {
  style?: React.CSSProperties;
}

// 窗口元素接口
interface WindowElement {
  element_rect: ElementRect;
  window_id: number;
}

// 选择状态
enum SelectState {
  WaitSelect = "wait_select", // 等待选择
  Selecting = "selecting", // 选择中
  Selected = "selected", // 已选择
  Resizing = "resizing", // 调整大小中
}

// 调整手柄位置
enum ResizeHandle {
  None = "none",
  TopLeft = "top_left",
  TopCenter = "top_center",
  TopRight = "top_right",
  MiddleLeft = "middle_left",
  MiddleRight = "middle_right",
  BottomLeft = "bottom_left",
  BottomCenter = "bottom_center",
  BottomRight = "bottom_right",
}

// 手柄尺寸
const HANDLE_SIZE = 8;
const HANDLE_HALF = HANDLE_SIZE / 2;

// 根据手柄位置返回对应的光标样式
const getHandleCursor = (handle: ResizeHandle): string => {
  switch (handle) {
    case ResizeHandle.TopLeft:
    case ResizeHandle.BottomRight:
      return "nwse-resize";
    case ResizeHandle.TopRight:
    case ResizeHandle.BottomLeft:
      return "nesw-resize";
    case ResizeHandle.TopCenter:
    case ResizeHandle.BottomCenter:
      return "ns-resize";
    case ResizeHandle.MiddleLeft:
    case ResizeHandle.MiddleRight:
      return "ew-resize";
    default:
      return "crosshair";
  }
};

export const SelectLayer = forwardRef<
  SelectLayerActionType | undefined,
  SelectLayerProps
>((props, ref) => {
  const layerContainerRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const contextRef = useRef<CanvasRenderingContext2D | null>(null);
  const selectRectRef = useRef<ElementRect | undefined>(undefined);
  const [selectState, setSelectState] = useState<SelectState>(
    SelectState.WaitSelect,
  );
  const [isMouseDown, setIsMouseDown] = useState(false);
  const [currentCursor, setCurrentCursor] = useState("crosshair");
  const startPosRef = useRef<{ x: number; y: number } | null>(null);
  const activeHandleRef = useRef<ResizeHandle>(ResizeHandle.None);
  const flatbushRef = useRef<Flatbush | null>(null);
  const windowElementsRef = useRef<WindowElement[]>([]);
  const boundingBoxRef = useRef<CaptureBoundingBoxInfo | null>(null);
  const selectedWindowIdRef = useRef<number | undefined>(undefined);

  const drawContext = useContext(DrawContext);
  const { mousePositionRef } = drawContext || {};

  // 订阅 DrawState 以便在绘图工具激活时禁用选择层
  const [drawState, setDrawState] = useState<DrawState>(DrawState.Idle);
  useStateSubscriber(DrawStatePublisher, (state) => {
    if (state !== undefined) {
      setDrawState(state);
    }
  });

  // 检查是否处于绘图模式（需要禁用选择层交互）
  const isDrawingMode =
    drawState === DrawState.Rect ||
    drawState === DrawState.Ellipse ||
    drawState === DrawState.Arrow ||
    drawState === DrawState.Pen ||
    drawState === DrawState.Text ||
    drawState === DrawState.Line;

  // 初始化 canvas context 和尺寸
  useEffect(() => {
    if (canvasRef.current && !contextRef.current) {
      const ctx = canvasRef.current.getContext("2d");
      if (ctx) {
        contextRef.current = ctx;
      }

      // 设置 canvas 的实际尺寸为屏幕尺寸
      const updateCanvasSize = () => {
        if (canvasRef.current) {
          const rect = canvasRef.current.getBoundingClientRect();
          canvasRef.current.width = rect.width;
          canvasRef.current.height = rect.height;
        }
      };

      updateCanvasSize();

      // 监听窗口大小变化
      window.addEventListener("resize", updateCanvasSize);
      return () => window.removeEventListener("resize", updateCanvasSize);
    }
  }, []);

  // 加载窗口元素（用于智能识别）
  const loadWindowElements = useCallback(async () => {
    try {
      console.log("[SelectLayer] Loading window elements...");
      const startTime = performance.now();

      // 添加超时保护（5秒）
      const timeoutPromise = new Promise<WindowElement[]>((_, reject) => {
        setTimeout(
          () => reject(new Error("Window elements loading timeout")),
          5000,
        );
      });

      const windowsPromise = invoke<WindowElement[]>(
        "get_screenshot_window_elements",
      );

      const windows = await Promise.race([windowsPromise, timeoutPromise]);

      const loadTime = performance.now() - startTime;
      console.log(
        `[SelectLayer] Loaded ${windows.length} window elements in ${loadTime.toFixed(1)}ms`,
      );

      // 限制窗口数量（避免过多窗口影响性能）
      const MAX_WINDOWS = 100;
      const limitedWindows = windows.slice(0, MAX_WINDOWS);
      if (windows.length > MAX_WINDOWS) {
        console.warn(
          `[SelectLayer] Too many windows (${windows.length}), limited to ${MAX_WINDOWS}`,
        );
      }

      windowElementsRef.current = limitedWindows;

      // 构建 Flatbush 空间索引
      if (limitedWindows.length > 0) {
        const indexStartTime = performance.now();
        const index = new Flatbush(limitedWindows.length);
        for (const win of limitedWindows) {
          const rect = win.element_rect;
          index.add(rect.min_x, rect.min_y, rect.max_x, rect.max_y);
        }
        index.finish();
        flatbushRef.current = index;
        const indexTime = performance.now() - indexStartTime;
        console.log(
          `[SelectLayer] Built spatial index in ${indexTime.toFixed(1)}ms`,
        );
      }
    } catch (error) {
      console.error("[SelectLayer] Failed to load window elements:", error);
      // 失败时清空窗口列表，允许用户手动选择
      windowElementsRef.current = [];
      flatbushRef.current = null;
    }
  }, []);

  // 获取8个调整手柄的位置
  const getHandlePositions = useCallback((rect: ElementRect) => {
    const x = rect.min_x;
    const y = rect.min_y;
    const w = rect.max_x - rect.min_x;
    const h = rect.max_y - rect.min_y;
    const cx = x + w / 2;
    const cy = y + h / 2;

    return {
      [ResizeHandle.TopLeft]: { x: x - HANDLE_HALF, y: y - HANDLE_HALF },
      [ResizeHandle.TopCenter]: { x: cx - HANDLE_HALF, y: y - HANDLE_HALF },
      [ResizeHandle.TopRight]: { x: x + w - HANDLE_HALF, y: y - HANDLE_HALF },
      [ResizeHandle.MiddleLeft]: { x: x - HANDLE_HALF, y: cy - HANDLE_HALF },
      [ResizeHandle.MiddleRight]: {
        x: x + w - HANDLE_HALF,
        y: cy - HANDLE_HALF,
      },
      [ResizeHandle.BottomLeft]: { x: x - HANDLE_HALF, y: y + h - HANDLE_HALF },
      [ResizeHandle.BottomCenter]: {
        x: cx - HANDLE_HALF,
        y: y + h - HANDLE_HALF,
      },
      [ResizeHandle.BottomRight]: {
        x: x + w - HANDLE_HALF,
        y: y + h - HANDLE_HALF,
      },
    };
  }, []);

  // 检测点击位置是否在某个手柄上
  const getHandleAtPosition = useCallback(
    (x: number, y: number, rect: ElementRect | undefined): ResizeHandle => {
      if (!rect) return ResizeHandle.None;

      const handles = getHandlePositions(rect);
      const hitArea = HANDLE_SIZE + 4; // 增加点击区域

      for (const [handle, pos] of Object.entries(handles)) {
        if (
          x >= pos.x - 2 &&
          x <= pos.x + hitArea &&
          y >= pos.y - 2 &&
          y <= pos.y + hitArea
        ) {
          return handle as ResizeHandle;
        }
      }

      return ResizeHandle.None;
    },
    [getHandlePositions],
  );

  // 绘制选择矩形和调整手柄
  const drawSelectRect = useCallback(
    (rect: ElementRect | undefined, showHandles = false) => {
      if (!contextRef.current || !canvasRef.current) return;

      const ctx = contextRef.current;
      const canvas = canvasRef.current;

      // 清空画布
      ctx.clearRect(0, 0, canvas.width, canvas.height);

      if (!rect) return;

      // 绘制蒙层（半透明黑色）
      ctx.fillStyle = "rgba(0, 0, 0, 0.5)";
      ctx.fillRect(0, 0, canvas.width, canvas.height);

      // 清除选中区域（透明）
      const x = Math.min(rect.min_x, rect.max_x);
      const y = Math.min(rect.min_y, rect.max_y);
      const width = Math.abs(rect.max_x - rect.min_x);
      const height = Math.abs(rect.max_y - rect.min_y);

      ctx.clearRect(x, y, width, height);

      // 绘制选择边框
      ctx.strokeStyle = "#1890ff";
      ctx.lineWidth = 2;
      ctx.strokeRect(x, y, width, height);

      // 绘制尺寸信息（带背景）
      const sizeText = `${Math.round(width)} × ${Math.round(height)}`;
      ctx.font = "12px Arial";
      const textMetrics = ctx.measureText(sizeText);
      const textWidth = textMetrics.width;
      const textHeight = 16;
      const textPadding = 4;
      const textX = x;
      const textY = y - textHeight - textPadding - 2;

      // 背景
      ctx.fillStyle = "rgba(24, 144, 255, 0.9)";
      ctx.fillRect(
        textX - textPadding,
        textY - textPadding,
        textWidth + textPadding * 2,
        textHeight + textPadding,
      );

      // 文字
      ctx.fillStyle = "#ffffff";
      ctx.fillText(sizeText, textX, textY + textHeight - 4);

      // 绘制调整手柄（仅在已选择状态）
      if (showHandles && width > 20 && height > 20) {
        const handles = getHandlePositions(rect);

        ctx.fillStyle = "#ffffff";
        ctx.strokeStyle = "#1890ff";
        ctx.lineWidth = 1;

        for (const pos of Object.values(handles)) {
          ctx.fillRect(pos.x, pos.y, HANDLE_SIZE, HANDLE_SIZE);
          ctx.strokeRect(pos.x, pos.y, HANDLE_SIZE, HANDLE_SIZE);
        }
      }
    },
    [getHandlePositions],
  );

  // 使用 Flatbush 查找鼠标位置的窗口
  const findWindowAtPosition = useCallback((x: number, y: number) => {
    const windows = windowElementsRef.current;
    if (windows.length === 0) {
      return undefined;
    }

    // 使用 Flatbush 进行快速查询
    if (flatbushRef.current) {
      const results = flatbushRef.current.search(x, y, x, y);

      if (results.length === 0) {
        return undefined;
      }

      // 从结果中找最小的窗口（处理嵌套情况）
      let minArea = Infinity;
      let bestWindow: WindowElement | undefined;

      for (const idx of results) {
        const win = windows[idx];
        const rect = win.element_rect;
        const area = (rect.max_x - rect.min_x) * (rect.max_y - rect.min_y);

        if (area < minArea) {
          minArea = area;
          bestWindow = win;
        }
      }

      return bestWindow;
    }

    // 备用：简单遍历查找
    let minArea = Infinity;
    let bestWindow: WindowElement | undefined;

    for (const win of windows) {
      const rect = win.element_rect;

      // 检查点是否在矩形内
      if (
        x >= rect.min_x &&
        x <= rect.max_x &&
        y >= rect.min_y &&
        y <= rect.max_y
      ) {
        const area = (rect.max_x - rect.min_x) * (rect.max_y - rect.min_y);

        // 找最小的窗口（处理嵌套情况）
        if (area < minArea) {
          minArea = area;
          bestWindow = win;
        }
      }
    }

    return bestWindow;
  }, []);

  // 鼠标按下
  const handleMouseDown = useCallback(
    (e: React.MouseEvent) => {
      if (!canvasRef.current) {
        console.warn("[SelectLayer] Canvas not ready");
        return;
      }

      // 容器是 fixed 全屏的，直接使用 clientX/clientY
      const x = e.clientX;
      const y = e.clientY;

      // 检查是否点击了调整手柄
      if (selectState === SelectState.Selected && selectRectRef.current) {
        const handle = getHandleAtPosition(x, y, selectRectRef.current);
        if (handle !== ResizeHandle.None) {
          activeHandleRef.current = handle;
          startPosRef.current = { x, y };
          setIsMouseDown(true);
          setSelectState(SelectState.Resizing);
          return;
        }
      }

      // 开始新的选择
      startPosRef.current = { x, y };
      setIsMouseDown(true);
      setSelectState(SelectState.Selecting);
      selectedWindowIdRef.current = undefined;
    },
    [selectState, getHandleAtPosition],
  );

  // 鼠标移动
  const handleMouseMove = useCallback(
    (e: React.MouseEvent) => {
      if (!canvasRef.current) return;

      // 容器是 fixed 全屏的，直接使用 clientX/clientY
      const x = e.clientX;
      const y = e.clientY;

      // 更新鼠标位置（用于其他组件）
      if (mousePositionRef?.current) {
        mousePositionRef.current.mouseX = x;
        mousePositionRef.current.mouseY = y;
      }

      if (
        selectState === SelectState.Resizing &&
        startPosRef.current &&
        selectRectRef.current
      ) {
        // 调整大小中
        const dx = x - startPosRef.current.x;
        const dy = y - startPosRef.current.y;
        const handle = activeHandleRef.current;
        const currentRect = selectRectRef.current;

        const newRect: ElementRect = { ...currentRect };

        switch (handle) {
          case ResizeHandle.TopLeft:
            newRect.min_x = currentRect.min_x + dx;
            newRect.min_y = currentRect.min_y + dy;
            break;
          case ResizeHandle.TopCenter:
            newRect.min_y = currentRect.min_y + dy;
            break;
          case ResizeHandle.TopRight:
            newRect.max_x = currentRect.max_x + dx;
            newRect.min_y = currentRect.min_y + dy;
            break;
          case ResizeHandle.MiddleLeft:
            newRect.min_x = currentRect.min_x + dx;
            break;
          case ResizeHandle.MiddleRight:
            newRect.max_x = currentRect.max_x + dx;
            break;
          case ResizeHandle.BottomLeft:
            newRect.min_x = currentRect.min_x + dx;
            newRect.max_y = currentRect.max_y + dy;
            break;
          case ResizeHandle.BottomCenter:
            newRect.max_y = currentRect.max_y + dy;
            break;
          case ResizeHandle.BottomRight:
            newRect.max_x = currentRect.max_x + dx;
            newRect.max_y = currentRect.max_y + dy;
            break;
        }

        // 确保矩形有效（最小尺寸）
        if (newRect.max_x - newRect.min_x < 10) {
          if (handle.includes("left")) {
            newRect.min_x = newRect.max_x - 10;
          } else {
            newRect.max_x = newRect.min_x + 10;
          }
        }
        if (newRect.max_y - newRect.min_y < 10) {
          if (handle.includes("top")) {
            newRect.min_y = newRect.max_y - 10;
          } else {
            newRect.max_y = newRect.min_y + 10;
          }
        }

        selectRectRef.current = newRect;
        startPosRef.current = { x, y };
        drawSelectRect(newRect, true);
      } else if (isMouseDown && startPosRef.current) {
        // 正在拖动：绘制选择矩形
        const selectRect: ElementRect = {
          min_x: Math.min(startPosRef.current.x, x),
          min_y: Math.min(startPosRef.current.y, y),
          max_x: Math.max(startPosRef.current.x, x),
          max_y: Math.max(startPosRef.current.y, y),
        };

        selectRectRef.current = selectRect;
        drawSelectRect(selectRect, false);
      } else if (
        selectState === SelectState.Selected &&
        selectRectRef.current
      ) {
        // 已选择状态：检测是否悬停在手柄上
        const handle = getHandleAtPosition(x, y, selectRectRef.current);
        const newCursor = getHandleCursor(handle);
        if (newCursor !== currentCursor) {
          setCurrentCursor(newCursor);
        }
        drawSelectRect(selectRectRef.current, true);
      } else if (selectState === SelectState.WaitSelect) {
        // 等待选择状态：智能窗口识别
        const window = findWindowAtPosition(x, y);
        if (window) {
          drawSelectRect(window.element_rect, false);
        } else {
          drawSelectRect(undefined, false);
        }
        if (currentCursor !== "crosshair") {
          setCurrentCursor("crosshair");
        }
      }
    },
    [
      isMouseDown,
      selectState,
      currentCursor,
      drawSelectRect,
      findWindowAtPosition,
      getHandleAtPosition,
      mousePositionRef,
    ],
  );

  // 鼠标抬起
  const handleMouseUp = useCallback(
    (e: React.MouseEvent) => {
      if (!canvasRef.current || !startPosRef.current) return;

      // 使用 clientX/clientY 直接（与 handleMouseDown 保持一致）
      // 因为容器是 fixed 全屏的
      const x = e.clientX;
      const y = e.clientY;

      if (selectState === SelectState.Resizing) {
        // 调整大小完成
        setIsMouseDown(false);
        setSelectState(SelectState.Selected);
        activeHandleRef.current = ResizeHandle.None;
        return;
      }

      // 计算选择矩形
      const selectRect: ElementRect = {
        min_x: Math.min(startPosRef.current.x, x),
        min_y: Math.min(startPosRef.current.y, y),
        max_x: Math.max(startPosRef.current.x, x),
        max_y: Math.max(startPosRef.current.y, y),
      };

      const width = selectRect.max_x - selectRect.min_x;
      const height = selectRect.max_y - selectRect.min_y;

      // 如果区域太小，尝试智能窗口识别
      if (width < 10 && height < 10) {
        const window = findWindowAtPosition(x, y);
        if (window) {
          selectRectRef.current = window.element_rect;
          selectedWindowIdRef.current = window.window_id;
        }
      } else {
        selectRectRef.current = selectRect;
        selectedWindowIdRef.current = undefined;
      }

      setIsMouseDown(false);
      setSelectState(SelectState.Selected);

      // 保持绘制选区（带手柄）
      if (selectRectRef.current) {
        drawSelectRect(selectRectRef.current, true);
      }
    },
    [selectState, findWindowAtPosition, drawSelectRect],
  );

  // 暴露给父组件的方法
  useImperativeHandle(
    ref,
    () => ({
      async onCaptureReady() {},

      async onCaptureLoad() {
        // 如果已经有选区，不再加载窗口元素（避免影响已有选区）
        if (selectRectRef.current) {
          console.log(
            "[SelectLayer] Selection already exists, skipping window elements load",
          );
          return;
        }
        await loadWindowElements();
      },

      async onCaptureFinish() {
        // 清理
        selectRectRef.current = undefined;
        selectedWindowIdRef.current = undefined;
        setSelectState(SelectState.WaitSelect);
        drawSelectRect(undefined, false);
        flatbushRef.current = null;
      },

      async onCaptureBoundingBoxInfoReady(boundingBox: CaptureBoundingBoxInfo) {
        boundingBoxRef.current = boundingBox;

        // 设置 canvas 尺寸
        if (canvasRef.current) {
          canvasRef.current.width = boundingBox.width;
          canvasRef.current.height = boundingBox.height;

          // 获取 2D context（如果还没有的话）
          if (!contextRef.current) {
            const ctx = canvasRef.current.getContext("2d");
            if (ctx) {
              contextRef.current = ctx;
            }
          }
        }
      },

      async onExecuteScreenshot() {},

      getSelectRect() {
        // 只有在选择完成状态才返回选区，避免工具栏在拖动时闪烁
        if (
          selectState === SelectState.Selected ||
          selectState === SelectState.Resizing
        ) {
          return selectRectRef.current;
        }
        return undefined;
      },

      isSelectionComplete() {
        return selectState === SelectState.Selected;
      },

      getSelectRectParams() {
        if (!selectRectRef.current) return undefined;
        return {
          rect: selectRectRef.current,
          offset: { x: 0, y: 0 },
        };
      },

      getWindowId() {
        return selectedWindowIdRef.current;
      },
    }),
    [loadWindowElements, drawSelectRect, selectState],
  );

  return (
    // biome-ignore lint/a11y/noStaticElementInteractions: Selection layer requires mouse interactions
    <div
      ref={layerContainerRef}
      className="select-layer-container"
      onMouseDown={handleMouseDown}
      onMouseMove={handleMouseMove}
      onMouseUp={handleMouseUp}
      style={{
        position: "fixed",
        top: 0,
        left: 0,
        width: "100vw",
        height: "100vh",
        zIndex: zIndexs.Draw_SelectLayer, // 109
        cursor: isDrawingMode ? "default" : currentCursor,
        pointerEvents: isDrawingMode ? "none" : "auto",
        ...props.style,
      }}
    >
      <canvas
        ref={canvasRef}
        className="select-layer-canvas"
        style={{
          width: "100vw",
          height: "100vh",
          position: "absolute",
          top: 0,
          left: 0,
        }}
      />
    </div>
  );
});

SelectLayer.displayName = "SelectLayer";
