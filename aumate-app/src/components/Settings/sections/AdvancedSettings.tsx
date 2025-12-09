import { Toggle } from "@/components/ui/Toggle";
import { useSettingsStore } from "@/stores/settingsStore";

export function AdvancedSettings() {
  const { settings, updateAdvanced } = useSettingsStore();
  const { advanced } = settings;

  return (
    <div className="space-y-6">
      <h2 className="text-xl font-semibold text-white">Advanced</h2>
      <p className="text-sm text-gray-400">
        Advanced settings for power users.
      </p>

      <div className="space-y-4">
        <SettingRow
          label="Debug Mode"
          description="Enable debug logging and developer tools"
        >
          <Toggle
            checked={advanced.debug_mode}
            onChange={(checked) => updateAdvanced({ debug_mode: checked })}
          />
        </SettingRow>
      </div>

      <div className="pt-6 border-t border-white/10">
        <h3 className="text-sm font-medium text-white mb-4">Data</h3>
        <div className="space-y-3">
          <button className="w-full text-left px-4 py-3 text-sm bg-gray-800 hover:bg-gray-700 rounded-lg text-gray-300 transition-colors">
            Export Settings
          </button>
          <button className="w-full text-left px-4 py-3 text-sm bg-gray-800 hover:bg-gray-700 rounded-lg text-gray-300 transition-colors">
            Import Settings
          </button>
          <button className="w-full text-left px-4 py-3 text-sm bg-red-900/30 hover:bg-red-900/50 rounded-lg text-red-400 transition-colors">
            Reset All Settings
          </button>
        </div>
      </div>
    </div>
  );
}

interface SettingRowProps {
  label: string;
  description?: string;
  children: React.ReactNode;
}

function SettingRow({ label, description, children }: SettingRowProps) {
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
