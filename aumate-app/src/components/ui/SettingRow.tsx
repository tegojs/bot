import type { ReactNode } from "react";

interface SettingRowProps {
  label: string;
  description?: string;
  children: ReactNode;
}

export function SettingRow({ label, description, children }: SettingRowProps) {
  return (
    <div className="flex items-center justify-between py-3 border-b border-white/5">
      <div>
        <div className="text-sm font-medium text-white">{label}</div>
        {description && (
          <div className="text-xs text-gray-400 mt-0.5">{description}</div>
        )}
      </div>
      {children}
    </div>
  );
}
