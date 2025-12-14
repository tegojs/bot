import type React from "react";
import { useRef } from "react";

export interface ColorRowProps {
  colors: string[];
  selectedColor: string;
  onChange: (color: string) => void;
  showTransparent?: boolean;
}

/**
 * 颜色选择行组件
 */
export const ColorRow: React.FC<ColorRowProps> = ({
  colors,
  selectedColor,
  onChange,
}) => {
  const colorInputRef = useRef<HTMLInputElement>(null);

  const handleCustomClick = () => {
    colorInputRef.current?.click();
  };

  const handleCustomColorChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    onChange(e.target.value);
  };

  return (
    <div className="flex items-center gap-1.5 flex-wrap">
      {colors.map((color) => (
        <button
          key={color}
          type="button"
          className={`w-7 h-7 rounded border-2 transition-all flex items-center justify-center ${
            selectedColor === color
              ? "border-blue-500 scale-110"
              : "border-gray-200 hover:border-gray-400"
          }`}
          style={{
            backgroundColor: color === "transparent" ? undefined : color,
          }}
          onClick={() => onChange(color)}
          title={color}
        >
          {color === "transparent" && (
            // 透明图标 - 斜线方格
            <div className="w-5 h-5 rounded relative overflow-hidden">
              <div
                className="absolute inset-0"
                style={{
                  background:
                    "repeating-conic-gradient(#ccc 0% 25%, white 0% 50%) 50% / 8px 8px",
                }}
              />
              <div className="absolute inset-0 flex items-center justify-center">
                <div className="w-full h-0.5 bg-red-500 rotate-45 transform origin-center" />
              </div>
            </div>
          )}
        </button>
      ))}

      {/* 自定义颜色按钮 */}
      <button
        type="button"
        className="w-7 h-7 rounded border-2 border-dashed border-gray-300 hover:border-gray-500 flex items-center justify-center relative overflow-hidden"
        onClick={handleCustomClick}
        title="自定义颜色"
      >
        {selectedColor !== "transparent" && !colors.includes(selectedColor) && (
          <div
            className="absolute inset-1 rounded"
            style={{ backgroundColor: selectedColor }}
          />
        )}
        <span className="relative text-gray-400 text-sm font-bold">+</span>
      </button>

      <input
        ref={colorInputRef}
        type="color"
        value={selectedColor === "transparent" ? "#ffffff" : selectedColor}
        onChange={handleCustomColorChange}
        className="sr-only"
      />
    </div>
  );
};
