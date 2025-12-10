import { useSettingsStore } from "@/stores/settingsStore";
import { Toggle } from "@/components/ui/Toggle";
import { FolderOpen } from "lucide-react";

export function ScreenshotSettings() {
  const { settings, updateScreenshot } = useSettingsStore();
  const { screenshot } = settings;

  const handleSelectFolder = async () => {
    // TODO: Implement folder picker using Tauri dialog
    // For now, just show the current path
    console.log("Select folder clicked");
  };

  return (
    <div className="space-y-8">
      <div>
        <h2 className="text-lg font-semibold text-white mb-1">Screenshot</h2>
        <p className="text-sm text-gray-400">Configure screenshot capture and save settings.</p>
      </div>

      <div className="space-y-6">
        {/* Save Folder */}
        <div>
          <label className="block text-sm font-medium text-gray-300 mb-2">
            Save Folder
          </label>
          <div className="flex gap-2">
            <input
              type="text"
              value={screenshot.save_folder}
              onChange={(e) => updateScreenshot({ save_folder: e.target.value })}
              placeholder="Default: Pictures folder"
              className="flex-1 bg-gray-800 border border-gray-700 rounded-lg px-3 py-2 text-sm text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            />
            <button
              type="button"
              onClick={handleSelectFolder}
              className="px-3 py-2 bg-gray-700 hover:bg-gray-600 text-white rounded-lg transition-colors"
            >
              <FolderOpen className="w-5 h-5" />
            </button>
          </div>
          <p className="text-xs text-gray-500 mt-1">
            Leave empty to save to default Pictures folder
          </p>
        </div>

        {/* Filename Pattern */}
        <div>
          <label className="block text-sm font-medium text-gray-300 mb-2">
            Filename Pattern
          </label>
          <input
            type="text"
            value={screenshot.filename_pattern}
            onChange={(e) => updateScreenshot({ filename_pattern: e.target.value })}
            placeholder="screenshot_%Y%m%d_%H%M%S"
            className="w-full bg-gray-800 border border-gray-700 rounded-lg px-3 py-2 text-sm text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
          />
          <p className="text-xs text-gray-500 mt-1">
            Use %Y (year), %m (month), %d (day), %H (hour), %M (minute), %S (second)
          </p>
        </div>

        {/* Image Format */}
        <div>
          <label className="block text-sm font-medium text-gray-300 mb-2">
            Image Format
          </label>
          <select
            value={screenshot.image_format}
            onChange={(e) =>
              updateScreenshot({ image_format: e.target.value as "png" | "webp" | "jpeg" })
            }
            className="w-full bg-gray-800 border border-gray-700 rounded-lg px-3 py-2 text-sm text-white focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
          >
            <option value="png">PNG (Best quality, larger file)</option>
            <option value="webp">WebP (Good quality, smaller file)</option>
            <option value="jpeg">JPEG (Smaller file, lossy)</option>
          </select>
        </div>

        {/* Auto Copy to Clipboard */}
        <div className="flex items-center justify-between">
          <div>
            <div className="text-sm font-medium text-gray-300">
              Auto Copy to Clipboard
            </div>
            <p className="text-xs text-gray-500 mt-1">
              Automatically copy screenshot to clipboard after capture
            </p>
          </div>
          <Toggle
            checked={screenshot.auto_copy_clipboard}
            onChange={(checked) => updateScreenshot({ auto_copy_clipboard: checked })}
          />
        </div>
      </div>
    </div>
  );
}
