import { invoke } from "@tauri-apps/api/core";
import { useCallback, useEffect, useState } from "react";
import { useSettingsStore } from "@/stores/settingsStore";

export function ShortcutsSettings() {
  const { settings, updateShortcuts } = useSettingsStore();
  const { shortcuts } = settings;

  return (
    <div className="space-y-6">
      <h2 className="text-xl font-semibold text-white">Shortcuts</h2>
      <p className="text-sm text-gray-400">
        Configure keyboard shortcuts for Aumate.
      </p>

      {/* Global Shortcuts */}
      <div className="space-y-4">
        <h3 className="text-sm font-medium text-gray-300">Global Shortcuts</h3>
        <p className="text-xs text-gray-500">
          These shortcuts work from anywhere on your system.
        </p>
        <ShortcutRow
          label="Toggle Command Palette"
          description="Show or hide the command palette"
          shortcut={shortcuts.toggle_palette}
          onUpdate={(newShortcut) =>
            updateShortcuts({ toggle_palette: newShortcut })
          }
        />
        <ShortcutRow
          label="Take Screenshot"
          description="Capture screen and open drawing tools"
          shortcut={shortcuts.screenshot}
          onUpdate={(newShortcut) => updateShortcuts({ screenshot: newShortcut })}
        />
        <ShortcutRow
          label="Element Scanner"
          description="Scan and label clickable elements on screen"
          shortcut={shortcuts.element_scan}
          onUpdate={(newShortcut) =>
            updateShortcuts({ element_scan: newShortcut })
          }
        />
      </div>

      {/* Local Shortcuts */}
      <div className="space-y-4 pt-4 border-t border-white/10">
        <h3 className="text-sm font-medium text-gray-300">
          Command Palette Shortcuts
        </h3>
        <p className="text-xs text-gray-500">
          These shortcuts only work when the command palette is open.
        </p>
        <ShortcutRow
          label="Open Settings"
          description="Open the settings window"
          shortcut={shortcuts.open_settings}
          onUpdate={(newShortcut) =>
            updateShortcuts({ open_settings: newShortcut })
          }
        />
      </div>
    </div>
  );
}

interface ShortcutRowProps {
  label: string;
  description: string;
  shortcut: string;
  onUpdate: (newShortcut: string) => void;
}

function ShortcutRow({
  label,
  description,
  shortcut,
  onUpdate,
}: ShortcutRowProps) {
  const [isEditing, setIsEditing] = useState(false);
  const [tempShortcut, setTempShortcut] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [isChecking, setIsChecking] = useState(false);

  // 格式化快捷键显示
  const formatShortcut = (keys: string) => {
    return keys
      .split("+")
      .map((k) => k.trim())
      .join("+");
  };

  // 捕获按键并转换为快捷键字符串
  const handleKeyDown = useCallback(
    (e: KeyboardEvent) => {
      if (!isEditing) return;

      e.preventDefault();
      e.stopPropagation();

      // 按 Escape 取消编辑
      if (e.key === "Escape") {
        setIsEditing(false);
        setTempShortcut("");
        setError(null);
        return;
      }

      // 忽略单独的修饰键
      if (
        ["Control", "Alt", "Shift", "Meta"].includes(e.key)
      ) {
        return;
      }

      // 构建快捷键字符串
      const parts: string[] = [];
      if (e.ctrlKey) parts.push("Ctrl");
      if (e.altKey) parts.push("Alt");
      if (e.shiftKey) parts.push("Shift");
      if (e.metaKey) parts.push("Meta");

      // 处理特殊键
      let key = e.key;
      if (key === " ") {
        key = "Space";
      } else if (key.length === 1) {
        // 单个字符转换为大写
        key = key.toUpperCase();
      } else if (key.startsWith("Arrow")) {
        // ArrowUp -> Up
        key = key.replace("Arrow", "");
      } else if (key.startsWith("Key")) {
        // KeyA -> A (某些浏览器会这样)
        key = key.replace("Key", "");
      }

      parts.push(key);

      const shortcutStr = formatShortcut(parts.join("+"));
      setTempShortcut(shortcutStr);
    },
    [isEditing],
  );

  // 检查快捷键可用性
  const checkAvailability = async (shortcutStr: string) => {
    setIsChecking(true);
    setError(null);

    try {
      const available = await invoke<boolean>(
        "check_global_shortcut_availability",
        {
          shortcut: shortcutStr,
        },
      );

      if (!available) {
        setError("This shortcut is already in use");
        return false;
      }

      return true;
    } catch (err) {
      console.error("Failed to check shortcut availability:", err);
      setError("Failed to check shortcut availability");
      return false;
    } finally {
      setIsChecking(false);
    }
  };

  // 保存快捷键
  const handleSave = async () => {
    if (!tempShortcut || isChecking) return;

    const isAvailable = await checkAvailability(tempShortcut);
    if (isAvailable) {
      onUpdate(tempShortcut);
      setIsEditing(false);
      setTempShortcut("");
      setError(null);
    }
  };

  // 开始编辑
  const handleStartEdit = () => {
    setIsEditing(true);
    setTempShortcut("");
    setError(null);
  };

  // 取消编辑
  const handleCancel = () => {
    setIsEditing(false);
    setTempShortcut("");
    setError(null);
  };

  // 监听键盘事件
  useEffect(() => {
    if (isEditing) {
      window.addEventListener("keydown", handleKeyDown, { capture: true });
      return () => {
        window.removeEventListener("keydown", handleKeyDown, { capture: true });
      };
    }
  }, [isEditing, handleKeyDown]);

  return (
    <div className="flex items-center justify-between py-3 border-b border-white/5">
      <div>
        <div className="text-sm font-medium text-white">{label}</div>
        <div className="text-xs text-gray-400 mt-0.5">{description}</div>
        {error && <div className="text-xs text-red-400 mt-1">{error}</div>}
      </div>
      <div className="flex items-center gap-2">
        <kbd className="px-3 py-1.5 text-sm font-medium bg-gray-700 text-white rounded border border-gray-600">
          {isEditing ? (tempShortcut || "Press keys...") : shortcut}
        </kbd>
        {!isEditing ? (
          <button
            type="button"
            onClick={handleStartEdit}
            className="px-3 py-1.5 text-xs font-medium text-gray-400 hover:text-white border border-gray-600 rounded hover:bg-white/5"
          >
            Change
          </button>
        ) : (
          <>
            <button
              type="button"
              onClick={handleSave}
              disabled={!tempShortcut || isChecking}
              className="px-3 py-1.5 text-xs font-medium text-green-400 hover:text-green-300 border border-green-600 rounded hover:bg-green-500/10 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {isChecking ? "Checking..." : "Save"}
            </button>
            <button
              type="button"
              onClick={handleCancel}
              className="px-3 py-1.5 text-xs font-medium text-gray-400 hover:text-white border border-gray-600 rounded hover:bg-white/5"
            >
              Cancel
            </button>
          </>
        )}
      </div>
    </div>
  );
}
