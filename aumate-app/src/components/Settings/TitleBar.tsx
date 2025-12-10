import { X, Minus } from "lucide-react";
import { getCurrentWindow } from "@tauri-apps/api/window";

export function TitleBar() {
  const appWindow = getCurrentWindow();

  const handleClose = async () => {
    await appWindow.hide();
  };

  const handleMinimize = async () => {
    await appWindow.minimize();
  };

  const handleDrag = async (e: React.MouseEvent) => {
    // Only start drag on left mouse button and not on buttons
    if (e.button === 0 && (e.target as HTMLElement).closest('button') === null) {
      await appWindow.startDragging();
    }
  };

  return (
    <div
      onMouseDown={handleDrag}
      className="flex items-center justify-between h-10 px-4 border-b border-white/10 select-none cursor-default"
    >
      {/* Title - draggable area */}
      <div className="flex items-center gap-2">
        <div className="w-4 h-4 bg-gradient-to-br from-blue-500 to-purple-600 rounded" />
        <span className="text-sm font-medium text-white">Aumate Settings</span>
      </div>

      {/* Window controls */}
      <div className="flex items-center gap-1">
        <button
          onClick={handleMinimize}
          className="p-1.5 text-gray-400 hover:text-white hover:bg-white/10 rounded transition-colors"
          title="Minimize"
        >
          <Minus className="w-4 h-4" />
        </button>
        <button
          onClick={handleClose}
          className="p-1.5 text-gray-400 hover:text-white hover:bg-red-500/80 rounded transition-colors"
          title="Close"
        >
          <X className="w-4 h-4" />
        </button>
      </div>
    </div>
  );
}
