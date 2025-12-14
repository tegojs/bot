import React from "react";
import { DrawState } from "../../types";
import { ColorPicker } from "./ColorPicker";
import { StrokeWidth } from "./StrokeWidth";
import { FontSize } from "./FontSize";

// Tools that need color picker
const toolsNeedingColor = [
  DrawState.Rect,
  DrawState.Diamond,
  DrawState.Ellipse,
  DrawState.Arrow,
  DrawState.Line,
  DrawState.Pen,
  DrawState.Text,
];

// Tools that need stroke width
const toolsNeedingStrokeWidth = [
  DrawState.Rect,
  DrawState.Diamond,
  DrawState.Ellipse,
  DrawState.Arrow,
  DrawState.Line,
  DrawState.Pen,
];

// Tools that need font size
const toolsNeedingFontSize = [DrawState.Text];

export interface SubToolbarProps {
  currentTool: DrawState;
  className?: string;
}

/**
 * Sub toolbar that shows tool-specific parameters
 * Displays color picker, stroke width, font size based on selected tool
 */
export const SubToolbar: React.FC<SubToolbarProps> = ({
  currentTool,
  className = "",
}) => {
  const showColor = toolsNeedingColor.includes(currentTool);
  const showStrokeWidth = toolsNeedingStrokeWidth.includes(currentTool);
  const showFontSize = toolsNeedingFontSize.includes(currentTool);

  // Don't render if no parameters to show
  if (!showColor && !showStrokeWidth && !showFontSize) {
    return null;
  }

  return (
    <div
      className={`flex items-center gap-3 bg-gray-800/95 backdrop-blur-sm rounded-lg p-2 shadow-xl border border-white/10 ${className}`}
    >
      {showColor && <ColorPicker />}
      {showColor && (showStrokeWidth || showFontSize) && (
        <div className="w-px h-6 bg-gray-600" />
      )}
      {showStrokeWidth && <StrokeWidth />}
      {showStrokeWidth && showFontSize && (
        <div className="w-px h-6 bg-gray-600" />
      )}
      {showFontSize && <FontSize />}
    </div>
  );
};

export { ColorPicker } from "./ColorPicker";
export { StrokeWidth } from "./StrokeWidth";
export { FontSize } from "./FontSize";
