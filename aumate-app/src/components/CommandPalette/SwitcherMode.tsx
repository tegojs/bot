import { useState, useEffect, useRef, useMemo } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Loader2 } from "lucide-react";
import { WindowItem, type WindowItemData } from "./WindowItem";

interface SwitcherModeProps {
  query: string;
  selectedIndex: number;
  onSelectIndex: (index: number) => void;
  onSwitchToWindow: (windowId: number) => void;
  onWindowsChange?: (windows: WindowItemData[]) => void;
}

export function SwitcherMode({
  query,
  selectedIndex,
  onSelectIndex,
  onSwitchToWindow,
  onWindowsChange,
}: SwitcherModeProps) {
  const [windows, setWindows] = useState<WindowItemData[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const listRef = useRef<HTMLDivElement>(null);

  // Load windows on mount
  useEffect(() => {
    loadWindows();
  }, []);

  const loadWindows = async () => {
    setIsLoading(true);
    setError(null);
    try {
      const windowElements = await invoke<WindowItemData[]>("get_window_elements");
      // Filter out the aumate app itself
      const filtered = windowElements.filter(
        (w) => !w.title.toLowerCase().includes("aumate") &&
               !w.app_name.toLowerCase().includes("aumate")
      );
      setWindows(filtered);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setIsLoading(false);
    }
  };

  // Filter windows by query
  const filteredWindows = useMemo(() => {
    return windows.filter((w) => {
      if (!query.trim()) return true;
      const searchLower = query.toLowerCase();
      return (
        w.title.toLowerCase().includes(searchLower) ||
        w.app_name.toLowerCase().includes(searchLower)
      );
    });
  }, [windows, query]);

  // Report filtered windows to parent
  useEffect(() => {
    onWindowsChange?.(filteredWindows);
  }, [filteredWindows, onWindowsChange]);

  // Auto-scroll to keep selected item visible
  useEffect(() => {
    if (listRef.current && filteredWindows.length > 0) {
      const selectedElement = listRef.current.children[selectedIndex] as HTMLElement;
      if (selectedElement) {
        selectedElement.scrollIntoView({ block: "nearest", behavior: "smooth" });
      }
    }
  }, [selectedIndex, filteredWindows.length]);

  // Reset selection when filtered list changes
  useEffect(() => {
    if (selectedIndex >= filteredWindows.length) {
      onSelectIndex(Math.max(0, filteredWindows.length - 1));
    }
  }, [filteredWindows.length, selectedIndex, onSelectIndex]);

  if (isLoading) {
    return (
      <div className="flex-1 flex items-center justify-center text-muted-foreground">
        <Loader2 className="w-5 h-5 animate-spin mr-2" />
        Loading windows...
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex-1 flex items-center justify-center text-red-400 px-4 text-center">
        <div>
          <p className="font-medium">Failed to load windows</p>
          <p className="text-sm text-muted-foreground mt-1">{error}</p>
        </div>
      </div>
    );
  }

  if (filteredWindows.length === 0) {
    return (
      <div className="flex-1 flex items-center justify-center text-muted-foreground">
        {query.trim() ? "No windows match your search" : "No windows found"}
      </div>
    );
  }

  return (
    <div ref={listRef} className="flex-1 overflow-y-auto">
      {filteredWindows.map((window, index) => (
        <WindowItem
          key={window.window_id}
          window={window}
          selected={index === selectedIndex}
          onClick={() => onSwitchToWindow(window.window_id)}
        />
      ))}
    </div>
  );
}

export { type WindowItemData };
export default SwitcherMode;
