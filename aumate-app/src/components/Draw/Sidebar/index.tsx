import type {
  FillStyle,
  StrokeRoundness,
  StrokeStyle,
} from "@excalidraw/excalidraw/element/types";
import type React from "react";
import { useCallback, useEffect, useState } from "react";
import { useStateSubscriber } from "@/hooks/useStatePublisher";
import {
  BackgroundColorPublisher,
  CornerStylePublisher,
  DrawStatePublisher,
  FillStylePublisher,
  OpacityPublisher,
  RoughnessPublisher,
  StrokeColorPublisher,
  StrokeStylePublisher,
  StrokeWidthPublisher,
  zIndexs,
} from "../extra";
import { usePublisherState } from "../hooks/usePublisherState";
import { DrawState, type ElementRect, type UpdateElementProps } from "../types";
import { ArrowOptions } from "./ArrowOptions";
import { ColorRow } from "./ColorRow";
import { ImageOptions } from "./ImageOptions";
import { LayerActions } from "./LayerActions";
import { OptionRow } from "./OptionRow";
import { SectionHeader } from "./SectionHeader";
import { SliderRow } from "./SliderRow";
import { TextOptions } from "./TextOptions";

// 描边颜色预设 (来自 snow-shot)
const STROKE_COLORS = [
  "#1e1e1e", // Black
  "#e03131", // Red
  "#2f9e44", // Green
  "#1098ad", // Cyan
  "#1971c2", // Blue
];

// 背景颜色预设 (带透明度)
const BACKGROUND_COLORS = [
  "transparent",
  "#ffccc7", // Light red
  "#d9f7be", // Light green
  "#bae0ff", // Light blue
  "#fff1b8", // Light yellow
  "#ffc0cb", // Pink
];

export interface SidebarProps {
  className?: string;
  getSelectRect?: () => ElementRect | undefined;
  getSelectedElementsCount?: () => number;
  getSelectedElementType?: () => string | null; // 获取选中元素的类型
  onSendToBack?: () => void;
  onSendBackward?: () => void;
  onBringForward?: () => void;
  onBringToFront?: () => void;
  onCopyElements?: () => void;
  onDeleteElements?: () => void;
  onUpdateSelectedElements?: (props: UpdateElementProps) => void;
}

/**
 * 左侧工具选项面板
 * 根据当前工具动态显示配置项
 */
export const Sidebar: React.FC<SidebarProps> = ({
  className = "",
  getSelectRect,
  getSelectedElementsCount,
  getSelectedElementType,
  onSendToBack,
  onSendBackward,
  onBringForward,
  onBringToFront,
  onCopyElements,
  onDeleteElements,
  onUpdateSelectedElements,
}) => {
  // 当前工具状态
  const [drawState, setDrawState] = useState<DrawState>(DrawState.Idle);
  useStateSubscriber(DrawStatePublisher, setDrawState);

  const [sidebarPosition, setSidebarPosition] = useState<{
    x: number;
    y: number;
  } | null>(null);

  // 判断是否处于选择模式（用于在 callbacks 中使用）
  const hasSelectedElements = getSelectedElementsCount
    ? getSelectedElementsCount() > 0
    : false;
  const isSelectingMode = drawState === DrawState.Select && hasSelectedElements;

  // 使用 usePublisherState Hook 管理样式状态
  const [strokeColor, setStrokeColor] = usePublisherState(
    StrokeColorPublisher,
    useCallback(
      (color: string) => {
        console.log("[Sidebar] Stroke color changed:", color);
        if (isSelectingMode && onUpdateSelectedElements) {
          onUpdateSelectedElements({ strokeColor: color });
        }
      },
      [isSelectingMode, onUpdateSelectedElements],
    ),
  );

  const [backgroundColor, setBackgroundColor] = usePublisherState(
    BackgroundColorPublisher,
    useCallback(
      (color: string) => {
        console.log("[Sidebar] Background color changed:", color);
        if (isSelectingMode && onUpdateSelectedElements) {
          onUpdateSelectedElements({ backgroundColor: color });
        }
      },
      [isSelectingMode, onUpdateSelectedElements],
    ),
  );

  const [fillStyle, setFillStyle] = usePublisherState(
    FillStylePublisher,
    useCallback(
      (value: FillStyle) => {
        console.log("[Sidebar] Fill style changed:", value);
        if (isSelectingMode && onUpdateSelectedElements) {
          onUpdateSelectedElements({ fillStyle: value });
        }
      },
      [isSelectingMode, onUpdateSelectedElements],
    ),
  );

  const [strokeWidth, setStrokeWidth] = usePublisherState(
    StrokeWidthPublisher,
    useCallback(
      (value: number) => {
        console.log("[Sidebar] Stroke width changed:", value);
        if (isSelectingMode && onUpdateSelectedElements) {
          onUpdateSelectedElements({ strokeWidth: value });
        }
      },
      [isSelectingMode, onUpdateSelectedElements],
    ),
  );

  const [strokeStyle, setStrokeStyle] = usePublisherState(
    StrokeStylePublisher,
    useCallback(
      (value: StrokeStyle) => {
        if (isSelectingMode && onUpdateSelectedElements) {
          onUpdateSelectedElements({ strokeStyle: value });
        }
      },
      [isSelectingMode, onUpdateSelectedElements],
    ),
  );

  const [roughness, setRoughness] = usePublisherState(
    RoughnessPublisher,
    useCallback(
      (value: number) => {
        if (isSelectingMode && onUpdateSelectedElements) {
          onUpdateSelectedElements({ roughness: value });
        }
      },
      [isSelectingMode, onUpdateSelectedElements],
    ),
  );

  const [cornerStyle, setCornerStyle] = usePublisherState(
    CornerStylePublisher,
    useCallback(
      (value: StrokeRoundness) => {
        if (isSelectingMode && onUpdateSelectedElements) {
          const roundness = value === "round" ? { type: 2 } : { type: 3 };
          onUpdateSelectedElements({ roundness });
        }
      },
      [isSelectingMode, onUpdateSelectedElements],
    ),
  );

  const [opacity, setOpacity] = usePublisherState(
    OpacityPublisher,
    useCallback(
      (value: number) => {
        if (isSelectingMode && onUpdateSelectedElements) {
          onUpdateSelectedElements({ opacity: value });
        }
      },
      [isSelectingMode, onUpdateSelectedElements],
    ),
  );

  // Update sidebar position based on selection rect
  useEffect(() => {
    const updatePosition = () => {
      if (getSelectRect) {
        const rect = getSelectRect();

        if (rect && rect.max_x > rect.min_x && rect.max_y > rect.min_y) {
          const sidebarWidth = 224; // 14rem
          const margin = 10;

          // 优先尝试放在选区左侧外部
          let x = rect.min_x - sidebarWidth - margin;
          if (x < margin) {
            // 空间不够，尝试放在选区右侧外部
            x = rect.max_x + margin;
            if (x + sidebarWidth > window.innerWidth - margin) {
              // 两侧都不够，放在选区内部左侧
              x = rect.min_x + margin;
            }
          }

          // 垂直居中
          const sidebarHeight = 600; // 估计高度
          const centerY = (rect.min_y + rect.max_y) / 2;
          let y = centerY - sidebarHeight / 2;
          y = Math.max(
            margin,
            Math.min(y, window.innerHeight - sidebarHeight - margin),
          );

          setSidebarPosition({ x, y });
        } else {
          setSidebarPosition(null);
        }
      }
    };

    updatePosition();
    const interval = setInterval(updatePosition, 200);
    return () => clearInterval(interval);
  }, [getSelectRect]);

  // 判断是否应该显示 Sidebar
  // 1. 使用绘图工具时显示（用于设置新绘制元素的样式）
  // 2. 使用选择工具且有选中元素时也显示（用于调整已绘制图形的样式）
  const shouldShowSidebar =
    sidebarPosition !== null &&
    // 使用绘图工具
    ((drawState !== DrawState.Idle && drawState !== DrawState.Select) ||
      // 或者使用选择工具但有选中的元素
      isSelectingMode);

  if (!shouldShowSidebar) {
    return null;
  }

  // 根据工具类型或选中元素类型判断显示哪些配置项
  // 如果是选择工具且有选中元素，根据选中元素的类型显示配置项
  const selectedType = getSelectedElementType?.();

  // 确定当前有效的元素类型（绘图工具 或 选中元素类型）
  let effectiveType = "";
  if (isSelectingMode && selectedType) {
    effectiveType = selectedType;
  } else {
    effectiveType = drawState.toString();
  }

  const isShapeTool =
    effectiveType === "rectangle" ||
    effectiveType === "diamond" ||
    effectiveType === "ellipse" ||
    [DrawState.Rect, DrawState.Diamond, DrawState.Ellipse].includes(drawState);

  const isArrowTool =
    effectiveType === "arrow" || drawState === DrawState.Arrow;
  const isTextTool = effectiveType === "text" || drawState === DrawState.Text;
  const isImageTool =
    effectiveType === "image" || drawState === DrawState.Image;

  const showStrokeColor = !isImageTool;
  const showBackgroundColor = isShapeTool;
  const showFillStyle = isShapeTool;
  const showStrokeWidth = !isImageTool && !isTextTool;
  const showStrokeStyle = !isImageTool && !isTextTool;
  const showRoughness = !isImageTool && !isTextTool;
  const showCornerStyle = isShapeTool;
  const showArrowOptions = isArrowTool;
  const showTextOptions = isTextTool;

  const style: React.CSSProperties = sidebarPosition
    ? {
        position: "fixed",
        left: `${sidebarPosition.x}px`,
        top: `${sidebarPosition.y}px`,
      }
    : {};

  return (
    <div
      className={`bg-white/95 backdrop-blur-sm rounded-lg shadow-xl p-3 w-56
                  flex flex-col gap-1.5 select-none max-h-[90vh] overflow-y-auto ${className}`}
      style={{ ...style, zIndex: zIndexs.Draw_Toolbar }}
    >
      {/* 描边颜色 */}
      {showStrokeColor && (
        <>
          <SectionHeader title="描边" />
          <ColorRow
            colors={STROKE_COLORS}
            selectedColor={strokeColor}
            onChange={setStrokeColor}
          />
        </>
      )}

      {/* 背景颜色 */}
      {showBackgroundColor && (
        <>
          <SectionHeader title="背景" />
          <ColorRow
            colors={BACKGROUND_COLORS}
            selectedColor={backgroundColor}
            onChange={setBackgroundColor}
            showTransparent
          />
        </>
      )}

      {/* 填充模式 */}
      {showFillStyle && (
        <>
          <SectionHeader title="填充" />
          <OptionRow
            options={[
              { value: "hachure", label: "斜线", icon: "╱" },
              { value: "cross-hatch", label: "网格", icon: "╳" },
              { value: "solid", label: "实心", icon: "■" },
            ]}
            selectedValue={fillStyle}
            onChange={setFillStyle}
          />
        </>
      )}

      {/* 描边宽度 */}
      {showStrokeWidth && (
        <>
          <SectionHeader title="描边宽度" />
          <OptionRow
            options={[
              { value: 1, label: "细", icon: "─" },
              { value: 2, label: "中", icon: "━" },
              { value: 4, label: "粗", icon: "▬" },
            ]}
            selectedValue={strokeWidth}
            onChange={setStrokeWidth}
          />
        </>
      )}

      {/* 边框样式 */}
      {showStrokeStyle && (
        <>
          <SectionHeader title="边框样式" />
          <OptionRow
            options={[
              { value: "solid", label: "实线", icon: "──" },
              { value: "dashed", label: "虚线", icon: "- -" },
              { value: "dotted", label: "点线", icon: "···" },
            ]}
            selectedValue={strokeStyle}
            onChange={setStrokeStyle}
          />
        </>
      )}

      {/* 线条风格 */}
      {showRoughness && (
        <>
          <SectionHeader title="线条风格" />
          <OptionRow
            options={[
              { value: 0, label: "建筑", icon: "╱" },
              { value: 1, label: "手绘", icon: "∿" },
              { value: 2, label: "卡通", icon: "〜" },
            ]}
            selectedValue={roughness}
            onChange={setRoughness}
          />
        </>
      )}

      {/* 边角样式 */}
      {showCornerStyle && (
        <>
          <SectionHeader title="边角" />
          <OptionRow
            options={[
              { value: "sharp", label: "直角", icon: "┐" },
              { value: "round", label: "圆角", icon: "╮" },
            ]}
            selectedValue={cornerStyle}
            onChange={setCornerStyle}
          />
        </>
      )}

      {/* 箭头配置 */}
      {showArrowOptions && <ArrowOptions />}

      {/* 文字配置 */}
      {showTextOptions && <TextOptions />}

      {/* 图片配置 */}
      {isImageTool && (
        <ImageOptions
          cornerStyle={cornerStyle}
          opacity={opacity}
          onCornerStyleChange={setCornerStyle}
          onOpacityChange={setOpacity}
          onCopy={() => {
            console.log("[Sidebar] Copy image");
            onCopyElements?.();
          }}
          onDelete={() => {
            console.log("[Sidebar] Delete image");
            onDeleteElements?.();
          }}
          onCrop={() => {
            // TODO: 实现裁剪功能
            console.log("[Sidebar] Crop image - Not implemented yet");
          }}
          onLink={() => {
            // TODO: 实现链接功能
            console.log("[Sidebar] Link image - Not implemented yet");
          }}
          onBringForward={onBringForward}
          onSendBackward={onSendBackward}
          onBringToFront={onBringToFront}
          onSendToBack={onSendToBack}
        />
      )}

      {/* 透明度（非图片工具） */}
      {!isImageTool && (
        <>
          <SectionHeader title="透明度" />
          <SliderRow min={0} max={100} value={opacity} onChange={setOpacity} />
        </>
      )}

      {/* 图层操作（非图片工具） */}
      {!isImageTool && (
        <LayerActions
          onSendToBack={onSendToBack}
          onSendBackward={onSendBackward}
          onBringForward={onBringForward}
          onBringToFront={onBringToFront}
        />
      )}
    </div>
  );
};

export { ArrowOptions } from "./ArrowOptions";
export { ColorRow } from "./ColorRow";
export { ImageOptions } from "./ImageOptions";
export { LayerActions } from "./LayerActions";
export { OptionRow } from "./OptionRow";
export { SectionHeader } from "./SectionHeader";
export { SliderRow } from "./SliderRow";
export { TextOptions } from "./TextOptions";
