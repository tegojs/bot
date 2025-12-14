import React, { useCallback, useContext, useState } from "react";
import { useStateSubscriber } from "@/hooks/useStatePublisher";
import { FontSizePublisher } from "../../extra";

// Preset font sizes for quick selection
const presetSizes = [16, 20, 28, 36, 48];

export interface FontSizeProps {
  className?: string;
}

/**
 * Font size selector with presets and slider
 */
export const FontSize: React.FC<FontSizeProps> = ({ className = "" }) => {
  const [size, setSize] = useState(20);

  // Get publish method from context
  const fontSizeContext = useContext(FontSizePublisher.context);

  // Subscribe to size changes
  const onSizeChange = useCallback((value: number) => {
    setSize(value);
  }, []);

  useStateSubscriber(FontSizePublisher, onSizeChange);

  // Handle preset click
  const handlePresetClick = (value: number) => {
    setSize(value);
    fontSizeContext.publish(value);
  };

  // Handle slider change
  const handleSliderChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = Number(e.target.value);
    setSize(value);
    fontSizeContext.publish(value);
  };

  // Handle wheel scroll on slider
  const handleWheel = (e: React.WheelEvent) => {
    e.preventDefault();
    const delta = e.deltaY > 0 ? -2 : 2;
    const newValue = Math.max(8, Math.min(128, size + delta));
    setSize(newValue);
    fontSizeContext.publish(newValue);
  };

  return (
    <div className={`flex items-center gap-2 ${className}`}>
      <span className="text-xs text-gray-400">Font</span>
      {/* Preset buttons */}
      <div className="flex items-center gap-1">
        {presetSizes.map((s) => (
          <button
            key={s}
            type="button"
            className={`w-7 h-7 rounded text-xs transition-all ${
              size === s
                ? "bg-blue-600 text-white"
                : "bg-gray-700 text-gray-300 hover:bg-gray-600"
            }`}
            onClick={() => handlePresetClick(s)}
          >
            {s}
          </button>
        ))}
      </div>
      {/* Slider */}
      <input
        type="range"
        min={8}
        max={128}
        value={size}
        onChange={handleSliderChange}
        onWheel={handleWheel}
        className="w-20 h-1 bg-gray-700 rounded-lg appearance-none cursor-pointer accent-blue-500"
      />
      {/* Value display */}
      <span className="text-xs text-gray-300 w-8 text-right">{size}px</span>
    </div>
  );
};
