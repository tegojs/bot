import type React from "react";
import { useCallback, useContext, useEffect, useState } from "react";
import { useStateSubscriber } from "@/hooks/useStatePublisher";
import {
  CaptureStepPublisher,
  DrawStatePublisher,
  ToolLockedPublisher,
} from "../extra";
import { CaptureStep, DrawState, type ElementRect } from "../types";
import {
  ArrowIcon,
  CloseIcon,
  CopyIcon,
  DiamondIcon,
  EllipseIcon,
  EraserIcon,
  HandIcon,
  ImageToolIcon,
  LineIcon,
  LockIcon,
  PenIcon,
  RectIcon,
  RedoIcon,
  SaveIcon,
  SelectIcon,
  TextIcon,
  UndoIcon,
} from "./icons";
import { ToolButton } from "./ToolButton";
import { ToolDivider } from "./ToolDivider";

export interface ToolbarProps {
  onSave: () => void;
  onCopy: () => void;
  onClose: () => void;
  onUndo?: () => void;
  onRedo?: () => void;
  canUndo?: boolean;
  canRedo?: boolean;
  getSelectRect?: () => ElementRect | undefined;
  position?: { x: number; y: number };
}

/**
 * Main toolbar component for the screenshot tool
 * Displays drawing tools, undo/redo, and action buttons
 */
export const Toolbar: React.FC<ToolbarProps> = ({
  onSave,
  onCopy,
  onClose,
  onUndo,
  onRedo,
  canUndo = true,
  canRedo = true,
  getSelectRect,
  position,
}) => {
  // Local state for captureStep and drawState
  const [captureStep, setCaptureStep] = useState<CaptureStep>(
    CaptureStep.Select,
  );
  const [drawState, setDrawState] = useState<DrawState>(DrawState.Select);
  const [toolLocked, setToolLocked] = useState(false);
  const [toolbarPosition, setToolbarPosition] = useState<{
    x: number;
    y: number;
  } | null>(null);

  // Get publish method from context
  const drawStateContext = useContext(DrawStatePublisher.context);
  const toolLockedContext = useContext(ToolLockedPublisher.context);

  // Subscribe to state changes with callbacks
  const onCaptureStepChange = useCallback((value: CaptureStep) => {
    setCaptureStep(value);
  }, []);
  const onDrawStateChange = useCallback((value: DrawState) => {
    setDrawState(value);
  }, []);

  useStateSubscriber(CaptureStepPublisher, onCaptureStepChange);
  useStateSubscriber(DrawStatePublisher, onDrawStateChange);
  useStateSubscriber(ToolLockedPublisher, setToolLocked);

  // Update toolbar position based on selection rect
  useEffect(() => {
    const updatePosition = () => {
      if (getSelectRect) {
        const rect = getSelectRect();

        if (rect && rect.max_x > rect.min_x && rect.max_y > rect.min_y) {
          const toolbarWidth = 600; // 更宽以容纳所有工具
          const toolbarHeight = 52;
          const margin = 10;

          // Center horizontally relative to selection
          const centerX = (rect.min_x + rect.max_x) / 2;
          let x = centerX - toolbarWidth / 2;

          // Keep within screen bounds
          x = Math.max(
            margin,
            Math.min(x, window.innerWidth - toolbarWidth - margin),
          );

          // 优先尝试放在选区下方外侧
          let y = rect.max_y + margin;
          if (y + toolbarHeight > window.innerHeight - margin) {
            // 空间不够，尝试放在选区上方外侧
            y = rect.min_y - toolbarHeight - margin;
            if (y < margin) {
              // 上方也不够，放在选区内部底部
              y = rect.max_y - toolbarHeight - margin;
            }
          }

          setToolbarPosition({ x, y });
        } else {
          setToolbarPosition(null);
        }
      }
    };

    // Initial check
    updatePosition();

    // Periodic check for selection changes
    const interval = setInterval(updatePosition, 200);

    return () => clearInterval(interval);
  }, [getSelectRect]);

  // Hide toolbar if no position available
  if (!toolbarPosition && !position) {
    return null;
  }

  // Only show during Draw step or after selection is made
  if (captureStep === CaptureStep.Select && !toolbarPosition && !position) {
    return null;
  }

  const style: React.CSSProperties = position
    ? {
        position: "fixed",
        left: `${position.x}px`,
        top: `${position.y}px`,
      }
    : toolbarPosition
      ? {
          position: "fixed",
          left: `${toolbarPosition.x}px`,
          top: `${toolbarPosition.y}px`,
        }
      : {};

  const handleToolClick = (tool: DrawState) => {
    drawStateContext.publish(tool);
  };

  const handleLockToggle = () => {
    toolLockedContext.publish(!toolLocked);
  };

  return (
    <div
      className="flex items-center gap-1"
      style={{ ...style, zIndex: 10000 }}
    >
      {/* Main toolbar */}
      <div className="flex items-center gap-1 bg-white/95 backdrop-blur-sm rounded-lg p-1.5 shadow-xl">
        {/* 基础工具组 */}
        <div className="flex items-center gap-0.5">
          <ToolButton
            icon={<LockIcon />}
            active={toolLocked}
            onClick={handleLockToggle}
            tooltip="锁定工具"
          />
          <ToolButton
            icon={<HandIcon />}
            active={drawState === DrawState.Idle}
            onClick={() => handleToolClick(DrawState.Idle)}
            tooltip="抓手 (H)"
          />
          <ToolButton
            icon={<SelectIcon />}
            active={drawState === DrawState.Select}
            onClick={() => handleToolClick(DrawState.Select)}
            tooltip="选择 (V)"
          />
        </div>

        <ToolDivider />

        {/* 形状工具组 */}
        <div className="flex items-center gap-0.5">
          <ToolButton
            icon={<RectIcon />}
            active={drawState === DrawState.Rect}
            onClick={() => handleToolClick(DrawState.Rect)}
            tooltip="矩形 (R)"
          />
          <ToolButton
            icon={<DiamondIcon />}
            active={drawState === DrawState.Diamond}
            onClick={() => handleToolClick(DrawState.Diamond)}
            tooltip="菱形 (D)"
          />
          <ToolButton
            icon={<EllipseIcon />}
            active={drawState === DrawState.Ellipse}
            onClick={() => handleToolClick(DrawState.Ellipse)}
            tooltip="椭圆 (O)"
          />
        </div>

        <ToolDivider />

        {/* 线条工具组 */}
        <div className="flex items-center gap-0.5">
          <ToolButton
            icon={<ArrowIcon />}
            active={drawState === DrawState.Arrow}
            onClick={() => handleToolClick(DrawState.Arrow)}
            tooltip="箭头 (A)"
          />
          <ToolButton
            icon={<LineIcon />}
            active={drawState === DrawState.Line}
            onClick={() => handleToolClick(DrawState.Line)}
            tooltip="线条 (L)"
          />
          <ToolButton
            icon={<PenIcon />}
            active={drawState === DrawState.Pen}
            onClick={() => handleToolClick(DrawState.Pen)}
            tooltip="自由书写 (P)"
          />
        </div>

        <ToolDivider />

        {/* 辅助工具组 */}
        <div className="flex items-center gap-0.5">
          <ToolButton
            icon={<TextIcon />}
            active={drawState === DrawState.Text}
            onClick={() => handleToolClick(DrawState.Text)}
            tooltip="文字 (T)"
          />
          <ToolButton
            icon={<ImageToolIcon />}
            active={drawState === DrawState.Image}
            onClick={() => handleToolClick(DrawState.Image)}
            tooltip="插入图像 (I)"
          />
          <ToolButton
            icon={<EraserIcon />}
            active={drawState === DrawState.Eraser}
            onClick={() => handleToolClick(DrawState.Eraser)}
            tooltip="橡皮擦 (E)"
          />
        </div>

        <ToolDivider />

        {/* 撤销/重做 */}
        <div className="flex items-center gap-0.5">
          <ToolButton
            icon={<UndoIcon />}
            onClick={onUndo || (() => {})}
            disabled={!canUndo}
            tooltip="撤销 (Ctrl+Z)"
          />
          <ToolButton
            icon={<RedoIcon />}
            onClick={onRedo || (() => {})}
            disabled={!canRedo}
            tooltip="重做 (Ctrl+Shift+Z)"
          />
        </div>

        <ToolDivider />

        {/* 操作按钮 */}
        <div className="flex items-center gap-0.5">
          <ToolButton
            icon={<SaveIcon />}
            onClick={onSave}
            tooltip="保存 (Ctrl+S)"
            variant="primary"
          />
          <ToolButton
            icon={<CopyIcon />}
            onClick={onCopy}
            tooltip="复制 (Ctrl+C)"
          />
          <ToolButton
            icon={<CloseIcon />}
            onClick={onClose}
            tooltip="关闭 (ESC)"
            variant="danger"
          />
        </div>
      </div>
    </div>
  );
};

export * from "./icons";
export { ToolButton } from "./ToolButton";
export { ToolDivider } from "./ToolDivider";
