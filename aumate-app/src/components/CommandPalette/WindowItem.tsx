import { AppWindow } from "lucide-react";
import { cn } from "@/lib/utils";

export interface WindowItemData {
  window_id: number;
  title: string;
  app_name: string;
  rect: {
    min_x: number;
    min_y: number;
    max_x: number;
    max_y: number;
  };
}

interface WindowItemProps {
  window: WindowItemData;
  selected: boolean;
  onClick: () => void;
}

export function WindowItem({ window, selected, onClick }: WindowItemProps) {
  // Extract process name from app_name (e.g., "chrome.exe" from path or app name)
  const processName = window.app_name.includes("\\")
    ? window.app_name.split("\\").pop() || window.app_name
    : window.app_name;

  return (
    <div
      className={cn(
        "flex items-center gap-3 px-4 py-3 cursor-pointer transition-colors",
        selected ? "bg-sky-500/20" : "hover:bg-white/5"
      )}
      onClick={onClick}
    >
      {/* App Icon placeholder */}
      <div className="w-8 h-8 rounded-lg bg-white/10 flex items-center justify-center shrink-0">
        <AppWindow className="w-5 h-5 text-sky-400" />
      </div>

      {/* Window Info */}
      <div className="flex-1 min-w-0">
        <div className="text-sm font-medium text-foreground truncate">
          {window.title || "Untitled Window"}
        </div>
        <div className="text-xs text-muted-foreground">
          Running: {processName}
        </div>
      </div>
    </div>
  );
}

export default WindowItem;
