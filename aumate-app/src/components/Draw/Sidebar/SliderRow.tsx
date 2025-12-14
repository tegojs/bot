import type React from "react";

export interface SliderRowProps {
  min: number;
  max: number;
  value: number;
  onChange: (value: number) => void;
  step?: number;
}

/**
 * 滑块行组件
 */
export const SliderRow: React.FC<SliderRowProps> = ({
  min,
  max,
  value,
  onChange,
  step = 1,
}) => {
  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    onChange(Number(e.target.value));
  };

  return (
    <div className="flex items-center gap-2">
      <span className="text-xs text-gray-400 w-4">{min}</span>
      <input
        type="range"
        min={min}
        max={max}
        step={step}
        value={value}
        onChange={handleChange}
        className="flex-1 h-1.5 bg-gray-200 rounded-full appearance-none cursor-pointer
                   [&::-webkit-slider-thumb]:appearance-none
                   [&::-webkit-slider-thumb]:w-3.5
                   [&::-webkit-slider-thumb]:h-3.5
                   [&::-webkit-slider-thumb]:bg-blue-500
                   [&::-webkit-slider-thumb]:rounded-full
                   [&::-webkit-slider-thumb]:cursor-pointer
                   [&::-webkit-slider-thumb]:shadow-md
                   [&::-webkit-slider-thumb]:hover:bg-blue-600
                   [&::-moz-range-thumb]:w-3.5
                   [&::-moz-range-thumb]:h-3.5
                   [&::-moz-range-thumb]:bg-blue-500
                   [&::-moz-range-thumb]:rounded-full
                   [&::-moz-range-thumb]:border-0
                   [&::-moz-range-thumb]:cursor-pointer"
      />
      <span className="text-xs text-gray-600 font-medium w-8 text-right">
        {value}
      </span>
    </div>
  );
};
