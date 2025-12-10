import { useSettingsStore } from "@/stores/settingsStore";

export function ShortcutsSettings() {
  const { settings } = useSettingsStore();
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
        <p className="text-xs text-gray-500">These shortcuts work from anywhere on your system.</p>
        <ShortcutRow
          label="Toggle Command Palette"
          description="Show or hide the command palette"
          shortcut={shortcuts.toggle_palette}
        />
      </div>

      {/* Local Shortcuts */}
      <div className="space-y-4 pt-4 border-t border-white/10">
        <h3 className="text-sm font-medium text-gray-300">Command Palette Shortcuts</h3>
        <p className="text-xs text-gray-500">These shortcuts only work when the command palette is open.</p>
        <ShortcutRow
          label="Open Settings"
          description="Open the settings window"
          shortcut={shortcuts.open_settings}
        />
      </div>
    </div>
  );
}

interface ShortcutRowProps {
  label: string;
  description: string;
  shortcut: string;
}

function ShortcutRow({ label, description, shortcut }: ShortcutRowProps) {
  return (
    <div className="flex items-center justify-between py-3 border-b border-white/5">
      <div>
        <div className="text-sm font-medium text-white">{label}</div>
        <div className="text-xs text-gray-400 mt-0.5">{description}</div>
      </div>
      <div className="flex items-center gap-2">
        <kbd className="px-3 py-1.5 text-sm font-medium bg-gray-700 text-white rounded border border-gray-600">
          {shortcut}
        </kbd>
        <button className="px-3 py-1.5 text-xs font-medium text-gray-400 hover:text-white border border-gray-600 rounded hover:bg-white/5">
          Change
        </button>
      </div>
    </div>
  );
}
