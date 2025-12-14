import type React from "react";
import { useCallback, useContext, useState } from "react";
import { useStateSubscriber } from "@/hooks/useStatePublisher";
import {
  BackgroundColorPublisher,
  CornerStylePublisher,
  FillStylePublisher,
  OpacityPublisher,
  RoughnessPublisher,
  StrokeColorPublisher,
  StrokeStylePublisher,
  StrokeWidthPublisher,
  zIndexs,
} from "../extra";
import { ColorRow } from "./ColorRow";
import { OptionRow } from "./OptionRow";
import { SectionHeader } from "./SectionHeader";
import { SliderRow } from "./SliderRow";

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
}

/**
 * 左侧工具选项面板
 * 包含描边、背景、填充、宽度、样式等选项
 */
export const Sidebar: React.FC<SidebarProps> = ({ className = "" }) => {
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

  return (
    <div
      className={`fixed left-4 top-1/2 -translate-y-1/2
                  bg-white rounded-lg shadow-xl p-3 w-56
                  flex flex-col gap-1 select-none ${className}`}
      style={{ zIndex: zIndexs.Draw_Toolbar }}
    >
      {/* 描边颜色 */}
      <SectionHeader title="描边" />
      <ColorRow
        colors={STROKE_COLORS}
        selectedColor={strokeColor}
        onChange={handleStrokeColorChange}
      />

      {/* 背景颜色 */}
      <SectionHeader title="背景" />
      <ColorRow
        colors={BACKGROUND_COLORS}
        selectedColor={backgroundColor}
        onChange={handleBackgroundColorChange}
        showTransparent
      />

      {/* 填充模式 */}
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

      {/* 描边宽度 */}
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

      {/* 边框样式 */}
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

      {/* 线条风格 */}
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

      {/* 边角样式 */}
      <SectionHeader title="边角" />
      <OptionRow
        options={[
          { value: "sharp", label: "直角", icon: "┐" },
          { value: "round", label: "圆角", icon: "╮" },
        ]}
        selectedValue={cornerStyle}
        onChange={handleCornerStyleChange}
      />

      {/* 透明度 */}
      <SectionHeader title="透明度" />
      <SliderRow
        min={0}
        max={100}
        value={opacity}
        onChange={handleOpacityChange}
      />
    </div>
  );
};

export { ColorRow } from "./ColorRow";
export { OptionRow } from "./OptionRow";
export { SectionHeader } from "./SectionHeader";
export { SliderRow } from "./SliderRow";
