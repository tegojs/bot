import React, { useCallback, useContext, useState } from "react";
import { useStateSubscriber } from "@/hooks/useStatePublisher";
import { StrokeWidthPublisher } from "../../extra";

// Preset stroke widths for quick selection
const presetWidths = [1, 2, 4, 8];

export interface StrokeWidthProps {
  className?: string;
}

/**
 * Stroke width selector with presets and slider
 */
export const StrokeWidth: React.FC<StrokeWidthProps> = ({ className = "" }) => {
  const [width, setWidth] = useState(2);

  // Get publish method from context
  const strokeWidthContext = useContext(StrokeWidthPublisher.context);

  // Subscribe to width changes
  const onWidthChange = useCallback((value: number) => {
    setWidth(value);
  }, []);

  useStateSubscriber(StrokeWidthPublisher, onWidthChange);

  // Handle preset click
  const handlePresetClick = (value: number) => {
    setWidth(value);
    strokeWidthContext.publish(value);
  };

  // Handle slider change
  const handleSliderChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = Number(e.target.value);
    setWidth(value);
    strokeWidthContext.publish(value);
  };

  // Handle wheel scroll on slider
  const handleWheel = (e: React.WheelEvent) => {
    e.preventDefault();
    const delta = e.deltaY > 0 ? -1 : 1;
    const newValue = Math.max(1, Math.min(32, width + delta));
    setWidth(newValue);
    strokeWidthContext.publish(newValue);
  };

  return (
    <div className={`flex items-center gap-2 ${className}`}>
      <span className="text-xs text-gray-400">Width</span>
      {/* Preset buttons */}
      <div className="flex items-center gap-1">
        {presetWidths.map((w) => (
          <button
            key={w}
            type="button"
            className={`w-7 h-7 rounded text-xs transition-all ${
              width === w
                ? "bg-blue-600 text-white"
                : "bg-gray-700 text-gray-300 hover:bg-gray-600"
            }`}
            onClick={() => handlePresetClick(w)}
          >
            {w}
          </button>
        ))}
      </div>
      {/* Slider */}
      <input
        type="range"
        min={1}
        max={32}
        value={width}
        onChange={handleSliderChange}
        onWheel={handleWheel}
        className="w-20 h-1 bg-gray-700 rounded-lg appearance-none cursor-pointer accent-blue-500"
      />
      {/* Value display */}
      <span className="text-xs text-gray-300 w-8 text-right">{width}px</span>
    </div>
  );
};
