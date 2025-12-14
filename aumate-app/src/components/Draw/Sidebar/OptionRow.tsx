import React from "react";

export interface OptionItem<T = string | number> {
  value: T;
  label: string;
  icon?: string;
}

export interface OptionRowProps<T = string | number> {
  options: OptionItem<T>[];
  selectedValue: T;
  onChange: (value: T) => void;
}

/**
 * 选项按钮行组件
 */
export function OptionRow<T extends string | number>({
  options,
  selectedValue,
  onChange,
}: OptionRowProps<T>) {
  return (
    <div className="flex items-center gap-1.5">
      {options.map((option) => (
        <button
          key={String(option.value)}
          type="button"
          className={`flex-1 h-8 rounded border transition-all flex items-center justify-center ${
            selectedValue === option.value
              ? "border-blue-500 bg-blue-50 text-blue-600"
              : "border-gray-200 hover:border-gray-400 text-gray-600 hover:bg-gray-50"
          }`}
          onClick={() => onChange(option.value)}
          title={option.label}
        >
          <span className="text-sm font-medium">
            {option.icon || option.label}
          </span>
        </button>
      ))}
    </div>
  );
}
