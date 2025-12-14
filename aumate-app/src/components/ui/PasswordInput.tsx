import { useState } from "react";

interface PasswordInputProps {
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
  className?: string;
}

export function PasswordInput({
  value,
  onChange,
  placeholder = "",
  className = "w-48",
}: PasswordInputProps) {
  const [showPassword, setShowPassword] = useState(false);

  return (
    <div className="flex items-center gap-2">
      <input
        type={showPassword ? "text" : "password"}
        value={value}
        onChange={(e) => onChange(e.target.value)}
        placeholder={placeholder}
        className={`${className} px-3 py-1.5 text-sm bg-gray-800 border border-gray-600 rounded text-white placeholder-gray-500 focus:outline-none focus:border-blue-500`}
      />
      <button
        type="button"
        onClick={() => setShowPassword(!showPassword)}
        className="px-2 py-1.5 text-xs text-gray-400 hover:text-white border border-gray-600 rounded hover:bg-white/5"
      >
        {showPassword ? "Hide" : "Show"}
      </button>
    </div>
  );
}
