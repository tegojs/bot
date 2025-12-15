import type React from "react";

export type ToolButtonVariant = "default" | "primary" | "danger";

export interface ToolButtonProps {
  icon: React.ReactNode;
  onClick: () => void;
  active?: boolean;
  tooltip?: string;
  variant?: ToolButtonVariant;
  disabled?: boolean;
  className?: string;
}

/**
 * Tool button component with Tailwind styling
 */
export const ToolButton: React.FC<ToolButtonProps> = ({
  icon,
  onClick,
  active = false,
  tooltip,
  variant = "default",
  disabled = false,
  className = "",
}) => {
  // Base classes
  const baseClasses =
    "p-2 rounded-md transition-all duration-150 flex items-center justify-center";

  // Disabled state
  const disabledClasses = disabled ? "opacity-50 cursor-not-allowed" : "";

  // Variant and active state classes
  const variantClasses = {
    default: active
      ? "bg-blue-500 text-white hover:bg-blue-600"
      : "text-gray-700 hover:bg-gray-100",
    primary: "bg-green-500 text-white hover:bg-green-600",
    danger: "text-red-500 hover:bg-red-50",
  };

  const handleClick = (e: React.MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
    if (!disabled) {
      onClick();
    }
  };

  return (
    <button
      type="button"
      className={`${baseClasses} ${variantClasses[variant]} ${disabledClasses} ${className}`}
      onClick={handleClick}
      disabled={disabled}
      title={tooltip}
    >
      {icon}
    </button>
  );
};
