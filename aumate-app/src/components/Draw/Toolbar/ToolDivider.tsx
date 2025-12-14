import type React from "react";

export interface ToolDividerProps {
  className?: string;
}

/**
 * Vertical divider for toolbar sections
 */
export const ToolDivider: React.FC<ToolDividerProps> = ({ className = "" }) => {
  return (
    <div
      className={`w-px h-6 bg-gray-600 mx-1 ${className}`}
      role="separator"
      aria-orientation="vertical"
    />
  );
};
