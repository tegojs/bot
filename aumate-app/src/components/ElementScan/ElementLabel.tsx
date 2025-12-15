import type React from "react";
import type { ScannableElement } from "./types";

interface ElementLabelProps {
  element: ScannableElement;
  isHovered: boolean;
  isPressed: boolean;
  onHover: () => void;
  onLeave: () => void;
}

export const ElementLabel: React.FC<ElementLabelProps> = ({
  element,
  isHovered,
  isPressed,
  onHover,
  onLeave,
}) => {
  const labelSize = 36;
  const padding = 4;

  // 计算标签位置（左上角附近）
  const style: React.CSSProperties = {
    position: "fixed",
    left: `${element.bounds.x}px`,
    top: `${element.bounds.y - labelSize - padding}px`, // 标签在元素上方
    width: `${labelSize}px`,
    height: `${labelSize}px`,
    zIndex: 10000,
  };

  return (
    <>
      {/* 元素高亮边框 */}
      {(isHovered || isPressed) && (
        <div
          style={{
            position: "fixed",
            left: `${element.bounds.x}px`,
            top: `${element.bounds.y}px`,
            width: `${element.bounds.width}px`,
            height: `${element.bounds.height}px`,
            border: isPressed
              ? "3px solid rgb(34, 197, 94)"
              : "2px solid rgb(59, 130, 246)",
            backgroundColor: isPressed
              ? "rgba(34, 197, 94, 0.1)"
              : "rgba(59, 130, 246, 0.05)",
            pointerEvents: "none",
            zIndex: 9999,
            transition: "all 0.15s ease",
          }}
        />
      )}

      {/* 字母标签 */}
      <div
        style={style}
        onMouseEnter={onHover}
        onMouseLeave={onLeave}
        className={`
          flex items-center justify-center
          rounded-full 
          font-bold text-xl
          transition-all duration-150
          ${
            isPressed
              ? "bg-green-500 text-white scale-110 shadow-lg"
              : isHovered
                ? "bg-blue-500 text-white scale-105 shadow-md"
                : "bg-blue-500/80 text-white shadow"
          }
        `}
      >
        {element.label}
      </div>

      {/* 元素标题提示（悬停时显示） */}
      {isHovered && element.title && (
        <div
          style={{
            position: "fixed",
            left: `${element.bounds.x}px`,
            top: `${element.bounds.y + element.bounds.height + padding}px`,
            maxWidth: "300px",
            zIndex: 10001,
          }}
          className="bg-gray-900/95 text-white text-sm px-3 py-1.5 rounded shadow-lg"
        >
          {element.title}
        </div>
      )}
    </>
  );
};

