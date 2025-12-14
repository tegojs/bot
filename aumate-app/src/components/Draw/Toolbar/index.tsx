import type React from "react";
import { useCallback, useContext, useEffect, useState } from "react";
import { useStateSubscriber } from "@/hooks/useStatePublisher";
import { CaptureStepPublisher, DrawStatePublisher } from "../extra";
import { CaptureStep, DrawState, type ElementRect } from "../types";
import {
  ArrowIcon,
  CloseIcon,
  CopyIcon,
  EllipseIcon,
  PenIcon,
  RectIcon,
  RedoIcon,
  SaveIcon,
  TextIcon,
  UndoIcon,
} from "./icons";
import { SubToolbar } from "./SubToolbar";
import { ToolButton } from "./ToolButton";
import { ToolDivider } from "./ToolDivider";

export interface ToolbarProps {
  onSave: () => void;
  onCopy: () => void;
  onClose: () => void;
  onUndo?: () => void;
  onRedo?: () => void;
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
  getSelectRect,
  position,
}) => {
  // Local state for captureStep and drawState
  const [captureStep, setCaptureStep] = useState<CaptureStep>(
    CaptureStep.Select,
  );
  const [drawState, setDrawState] = useState<DrawState>(DrawState.Idle);
  const [toolbarPosition, setToolbarPosition] = useState<{
    x: number;
    y: number;
  } | null>(null);

  // Get publish method from context
  const drawStateContext = useContext(DrawStatePublisher.context);

  // Subscribe to state changes with callbacks
  const onCaptureStepChange = useCallback((value: CaptureStep) => {
    setCaptureStep(value);
  }, []);
  const onDrawStateChange = useCallback((value: DrawState) => {
    setDrawState(value);
  }, []);

  useStateSubscriber(CaptureStepPublisher, onCaptureStepChange);
  useStateSubscriber(DrawStatePublisher, onDrawStateChange);

  // Update toolbar position based on selection rect
  useEffect(() => {
    const updatePosition = () => {
      if (getSelectRect) {
        const rect = getSelectRect();

        if (rect && rect.max_x > rect.min_x && rect.max_y > rect.min_y) {
          const toolbarWidth = 360;
          const toolbarHeight = 48;
          const margin = 10;

          // Center horizontally relative to selection
          const centerX = (rect.min_x + rect.max_x) / 2;
          let x = centerX - toolbarWidth / 2;

          // Keep within screen bounds
          x = Math.max(
            margin,
            Math.min(x, window.innerWidth - toolbarWidth - margin),
          );

          // Position below selection, or above if not enough space below
          let y = rect.max_y + margin;
          if (y + toolbarHeight > window.innerHeight - margin) {
            y = rect.min_y - toolbarHeight - margin;
          }
          y = Math.max(margin, y);

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

  // 阻止鼠标事件冒泡到 SelectLayer，避免点击工具栏时修改选区
  const handleMouseDown = (e: React.MouseEvent) => {
    e.stopPropagation();
  };

  return (
    // biome-ignore lint/a11y/noStaticElementInteractions: need to stop event propagation
    <div
      className="flex flex-col items-center gap-2"
      style={{ ...style, zIndex: 10000 }}
      onMouseDown={handleMouseDown}
      onMouseUp={handleMouseDown}
      onMouseMove={handleMouseDown}
    >
      {/* Main toolbar */}
      <div className="flex items-center gap-1 bg-gray-800/95 backdrop-blur-sm rounded-lg p-1.5 shadow-xl border border-white/10">
        {/* Drawing tools */}
        <div className="flex items-center gap-0.5">
          <ToolButton
            icon={<RectIcon />}
            active={drawState === DrawState.Rect}
            onClick={() => handleToolClick(DrawState.Rect)}
            tooltip="Rectangle (R)"
          />
          <ToolButton
            icon={<EllipseIcon />}
            active={drawState === DrawState.Ellipse}
            onClick={() => handleToolClick(DrawState.Ellipse)}
            tooltip="Ellipse (O)"
          />
          <ToolButton
            icon={<ArrowIcon />}
            active={drawState === DrawState.Arrow}
            onClick={() => handleToolClick(DrawState.Arrow)}
            tooltip="Arrow (A)"
          />
          <ToolButton
            icon={<PenIcon />}
            active={drawState === DrawState.Pen}
            onClick={() => handleToolClick(DrawState.Pen)}
            tooltip="Pen (P)"
          />
          <ToolButton
            icon={<TextIcon />}
            active={drawState === DrawState.Text}
            onClick={() => handleToolClick(DrawState.Text)}
            tooltip="Text (T)"
          />
        </div>

        <ToolDivider />

        {/* History controls */}
        <div className="flex items-center gap-0.5">
          <ToolButton
            icon={<UndoIcon />}
            onClick={() => onUndo?.()}
            tooltip="Undo (Ctrl+Z)"
          />
          <ToolButton
            icon={<RedoIcon />}
            onClick={() => onRedo?.()}
            tooltip="Redo (Ctrl+Shift+Z)"
          />
        </div>

        <ToolDivider />

        {/* Action buttons */}
        <div className="flex items-center gap-0.5">
          <ToolButton
            icon={<SaveIcon />}
            onClick={onSave}
            tooltip="Save (Ctrl+S)"
            variant="primary"
          />
          <ToolButton
            icon={<CopyIcon />}
            onClick={onCopy}
            tooltip="Copy (Ctrl+C)"
          />
          <ToolButton
            icon={<CloseIcon />}
            onClick={onClose}
            tooltip="Close (ESC)"
            variant="danger"
          />
        </div>
      </div>

      {/* Sub toolbar for tool parameters */}
      <SubToolbar currentTool={drawState} />
    </div>
  );
};

export * from "./icons";
export { ToolButton } from "./ToolButton";
export { ToolDivider } from "./ToolDivider";
