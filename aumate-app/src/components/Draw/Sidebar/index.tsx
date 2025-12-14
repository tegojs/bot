import type React from "react";
import { useCallback, useContext, useEffect, useState } from "react";
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
import { DrawState, type ElementRect } from "../types";
import { ArrowOptions } from "./ArrowOptions";
import { ColorRow } from "./ColorRow";
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
  onSendToBack?: () => void;
  onSendBackward?: () => void;
  onBringForward?: () => void;
  onBringToFront?: () => void;
}

/**
 * 左侧工具选项面板
 * 根据当前工具动态显示配置项
 */
export const Sidebar: React.FC<SidebarProps> = ({
  className = "",
  getSelectRect,
  onSendToBack,
  onSendBackward,
  onBringForward,
  onBringToFront,
}) => {
  // 当前工具状态
  const [drawState, setDrawState] = useState<DrawState>(DrawState.Idle);
  const [sidebarPosition, setSidebarPosition] = useState<{
    x: number;
    y: number;
  } | null>(null);

  // 当前选中的值
  const [strokeColor, setStrokeColor] = useState("#e03131");
  const [backgroundColor, setBackgroundColor] = useState("transparent");
  const [fillStyle, setFillStyle] = useState("hachure");
  const [strokeWidth, setStrokeWidth] = useState(2);
  const [strokeStyle, setStrokeStyle] = useState("solid");
  const [roughness, setRoughness] = useState(1);
  const [cornerStyle, setCornerStyle] = useState("round");
  const [opacity, setOpacity] = useState(100);

  // 获取 publisher contexts
  const strokeColorContext = useContext(StrokeColorPublisher.context);
  const backgroundColorContext = useContext(BackgroundColorPublisher.context);
  const fillStyleContext = useContext(FillStylePublisher.context);
  const strokeWidthContext = useContext(StrokeWidthPublisher.context);
  const strokeStyleContext = useContext(StrokeStylePublisher.context);
  const roughnessContext = useContext(RoughnessPublisher.context);
  const cornerStyleContext = useContext(CornerStylePublisher.context);
  const opacityContext = useContext(OpacityPublisher.context);

  // 订阅 publishers
  useStateSubscriber(DrawStatePublisher, setDrawState);
  useStateSubscriber(StrokeColorPublisher, setStrokeColor);
  useStateSubscriber(BackgroundColorPublisher, setBackgroundColor);
  useStateSubscriber(FillStylePublisher, setFillStyle);
  useStateSubscriber(StrokeWidthPublisher, setStrokeWidth);
  useStateSubscriber(StrokeStylePublisher, setStrokeStyle);
  useStateSubscriber(RoughnessPublisher, setRoughness);
  useStateSubscriber(CornerStylePublisher, setCornerStyle);
  useStateSubscriber(OpacityPublisher, setOpacity);

  // 处理器函数
  const handleStrokeColorChange = useCallback(
    (color: string) => {
      setStrokeColor(color);
      strokeColorContext.publish(color);
    },
    [strokeColorContext],
  );

  const handleBackgroundColorChange = useCallback(
    (color: string) => {
      setBackgroundColor(color);
      backgroundColorContext.publish(color);
    },
    [backgroundColorContext],
  );

  const handleFillStyleChange = useCallback(
    (value: string) => {
      setFillStyle(value);
      fillStyleContext.publish(value);
    },
    [fillStyleContext],
  );

  const handleStrokeWidthChange = useCallback(
    (value: number) => {
      setStrokeWidth(value);
      strokeWidthContext.publish(value);
    },
    [strokeWidthContext],
  );

  const handleStrokeStyleChange = useCallback(
    (value: string) => {
      setStrokeStyle(value);
      strokeStyleContext.publish(value);
    },
    [strokeStyleContext],
  );

  const handleRoughnessChange = useCallback(
    (value: number) => {
      setRoughness(value);
      roughnessContext.publish(value);
    },
    [roughnessContext],
  );

  const handleCornerStyleChange = useCallback(
    (value: string) => {
      setCornerStyle(value);
      cornerStyleContext.publish(value);
    },
    [cornerStyleContext],
  );

  const handleOpacityChange = useCallback(
    (value: number) => {
      setOpacity(value);
      opacityContext.publish(value);
    },
    [opacityContext],
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
  // 仅在选择了绘图工具时显示（Idle 和 Select 时隐藏）
  const shouldShowSidebar =
    drawState !== DrawState.Idle &&
    drawState !== DrawState.Select &&
    sidebarPosition !== null;

  if (!shouldShowSidebar) {
    return null;
  }

  // 根据工具类型判断显示哪些配置项
  const isShapeTool = [
    DrawState.Rect,
    DrawState.Diamond,
    DrawState.Ellipse,
  ].includes(drawState);

  const isArrowTool = drawState === DrawState.Arrow;
  const isTextTool = drawState === DrawState.Text;
  const isImageTool = drawState === DrawState.Image;

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
            onChange={handleStrokeColorChange}
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
            onChange={handleBackgroundColorChange}
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
            onChange={handleFillStyleChange}
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
            onChange={handleStrokeWidthChange}
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
            onChange={handleStrokeStyleChange}
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
            onChange={handleRoughnessChange}
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
            onChange={handleCornerStyleChange}
          />
        </>
      )}

      {/* 箭头配置 */}
      {showArrowOptions && <ArrowOptions />}

      {/* 文字配置 */}
      {showTextOptions && <TextOptions />}

      {/* 透明度 */}
      <SectionHeader title="透明度" />
      <SliderRow
        min={0}
        max={100}
        value={opacity}
        onChange={handleOpacityChange}
      />

      {/* 图层操作 */}
      <LayerActions
        onSendToBack={onSendToBack}
        onSendBackward={onSendBackward}
        onBringForward={onBringForward}
        onBringToFront={onBringToFront}
      />
    </div>
  );
};

export { ArrowOptions } from "./ArrowOptions";
export { ColorRow } from "./ColorRow";
export { LayerActions } from "./LayerActions";
export { OptionRow } from "./OptionRow";
export { SectionHeader } from "./SectionHeader";
export { SliderRow } from "./SliderRow";
export { TextOptions } from "./TextOptions";
