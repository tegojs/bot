import React, { useCallback, useContext, useRef, useState } from "react";
import { useStateSubscriber } from "@/hooks/useStatePublisher";
import { StrokeColorPublisher } from "../../extra";

// Preset colors for quick selection
const presetColors = [
  "#e03131", // Red
  "#2f9e44", // Green
  "#1971c2", // Blue
  "#f08c00", // Orange
  "#9c36b5", // Purple
  "#000000", // Black
  "#ffffff", // White
];

export interface ColorPickerProps {
  className?: string;
}

/**
 * Color picker component with preset colors and custom color input
 */
export const ColorPicker: React.FC<ColorPickerProps> = ({ className = "" }) => {
  const [currentColor, setCurrentColor] = useState("#e03131");
  const colorInputRef = useRef<HTMLInputElement>(null);

  // Get publish method from context
  const strokeColorContext = useContext(StrokeColorPublisher.context);

  // Subscribe to color changes
  const onColorChange = useCallback((color: string) => {
    setCurrentColor(color);
  }, []);

  useStateSubscriber(StrokeColorPublisher, onColorChange);

  // Handle preset color click
  const handlePresetClick = (color: string) => {
    setCurrentColor(color);
    strokeColorContext.publish(color);
  };

  // Handle custom color change
  const handleCustomColorChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const color = e.target.value;
    setCurrentColor(color);
    strokeColorContext.publish(color);
  };

  // Open color picker on custom button click
  const handleCustomClick = () => {
    colorInputRef.current?.click();
  };

  return (
    <div className={`flex items-center gap-1 ${className}`}>
      <span className="text-xs text-gray-400 mr-1">Color</span>
      {presetColors.map((color) => (
        <button
          key={color}
          type="button"
          className={`w-6 h-6 rounded border-2 transition-all ${
            currentColor === color
              ? "border-blue-500 scale-110"
              : "border-transparent hover:border-gray-500"
          }`}
          style={{ backgroundColor: color }}
          onClick={() => handlePresetClick(color)}
          title={color}
        />
      ))}
      {/* Custom color picker */}
      <button
        type="button"
        className="w-6 h-6 rounded border-2 border-dashed border-gray-500 hover:border-gray-400 flex items-center justify-center relative overflow-hidden"
        onClick={handleCustomClick}
        title="Custom color"
      >
        <div
          className="absolute inset-1 rounded"
          style={{ backgroundColor: currentColor }}
        />
        <span className="relative text-xs text-gray-400">+</span>
      </button>
      <input
        ref={colorInputRef}
        type="color"
        value={currentColor}
        onChange={handleCustomColorChange}
        className="sr-only"
      />
    </div>
  );
};
