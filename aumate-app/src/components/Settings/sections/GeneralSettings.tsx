import { Toggle } from "@/components/ui/Toggle";
import { useSettingsStore } from "@/stores/settingsStore";
import { cn } from "@/lib/utils";
import { Search, Sparkles, MessageSquare, AppWindow } from "lucide-react";

export function GeneralSettings() {
  const { settings, updateGeneral, updateEnabledModes } = useSettingsStore();
  const { general, enabled_modes } = settings;

  return (
    <div className="space-y-6">
      <h2 className="text-xl font-semibold text-white">General</h2>

      {/* Follow System Appearance */}
      <SettingRow
        label="Follow System Appearance"
        description="Automatically switch between light and dark mode"
      >
        <Toggle
          checked={general.follow_system_appearance}
          onChange={(checked) =>
            updateGeneral({ follow_system_appearance: checked })
          }
        />
      </SettingRow>

      {/* Open at Login */}
      <SettingRow
        label="Open at Login"
        description="Start Aumate when you log in to your computer"
      >
        <Toggle
          checked={general.open_at_login}
          onChange={(checked) => updateGeneral({ open_at_login: checked })}
        />
      </SettingRow>

      {/* Show in System Tray */}
      <SettingRow
        label="Show in System Tray"
        description="Keep Aumate running in the system tray"
      >
        <Toggle
          checked={general.show_in_system_tray}
          onChange={(checked) => updateGeneral({ show_in_system_tray: checked })}
        />
      </SettingRow>

      {/* Hotkey */}
      <SettingRow
        label="Aumate Hotkey"
        description="Global shortcut to open the command palette"
      >
        <div className="flex items-center gap-2">
          <kbd className="px-3 py-1.5 text-sm font-medium bg-gray-700 text-white rounded border border-gray-600">
            {general.hotkey}
          </kbd>
          <button className="p-1.5 text-gray-400 hover:text-white rounded hover:bg-white/10">
            <svg
              className="w-4 h-4"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
              />
            </svg>
          </button>
        </div>
      </SettingRow>

      {/* Window Mode */}
      <div className="pt-4">
        <h3 className="text-sm font-medium text-white mb-2">Window Mode</h3>
        <p className="text-sm text-gray-400 mb-4">
          Choose your preferred mode when opening Aumate
        </p>
        <div className="flex gap-4">
          <WindowModeOption
            label="Compact"
            selected={general.window_mode === "compact"}
            onClick={() => updateGeneral({ window_mode: "compact" })}
          />
          <WindowModeOption
            label="Expanded"
            selected={general.window_mode === "expanded"}
            onClick={() => updateGeneral({ window_mode: "expanded" })}
            highlighted
          />
        </div>
      </div>

      {/* Enabled Modes */}
      <div className="pt-4">
        <h3 className="text-sm font-medium text-white mb-2">Enabled Modes</h3>
        <p className="text-sm text-gray-400 mb-4">
          Choose which modes are available in the command palette
        </p>
        <div className="space-y-3">
          <ModeToggle
            icon={<Search className="w-4 h-4" />}
            label="Search Mode"
            description="Search and execute commands"
            checked={enabled_modes.search}
            onChange={(checked) => updateEnabledModes({ search: checked })}
            color="blue"
          />
          <ModeToggle
            icon={<Sparkles className="w-4 h-4" />}
            label="Polish Mode"
            description="Polish and improve text expressions"
            checked={enabled_modes.polish}
            onChange={(checked) => updateEnabledModes({ polish: checked })}
            color="purple"
          />
          <ModeToggle
            icon={<MessageSquare className="w-4 h-4" />}
            label="Dialogue Mode"
            description="Multi-turn AI conversation"
            checked={enabled_modes.dialogue}
            onChange={(checked) => updateEnabledModes({ dialogue: checked })}
            color="emerald"
          />
          <ModeToggle
            icon={<AppWindow className="w-4 h-4" />}
            label="Switcher Mode"
            description="Quick switch between open windows"
            checked={enabled_modes.switcher}
            onChange={(checked) => updateEnabledModes({ switcher: checked })}
            color="sky"
          />
        </div>
        <p className="text-xs text-gray-500 mt-3">
          At least one mode will always remain enabled
        </p>
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

interface WindowModeOptionProps {
  label: string;
  selected: boolean;
  onClick: () => void;
  highlighted?: boolean;
}

function WindowModeOption({
  label,
  selected,
  onClick,
  highlighted,
}: WindowModeOptionProps) {
  return (
    <button
      onClick={onClick}
      className={cn(
        "flex flex-col items-center gap-2 p-3 rounded-lg border-2 transition-all",
        selected
          ? "border-blue-500 bg-blue-500/10"
          : "border-gray-700 hover:border-gray-600"
      )}
    >
      <div
        className={cn(
          "w-32 h-20 rounded-md flex items-center justify-center",
          highlighted
            ? "bg-gradient-to-br from-purple-500 to-pink-500"
            : "bg-gray-700"
        )}
      >
        <div className="w-20 h-2 bg-white/30 rounded" />
        <div className="absolute mt-6 w-16 h-1 bg-white/20 rounded" />
      </div>
      <span className="text-xs text-gray-300">{label}</span>
    </button>
  );
}

interface ModeToggleProps {
  icon: React.ReactNode;
  label: string;
  description: string;
  checked: boolean;
  onChange: (checked: boolean) => void;
  color: "blue" | "purple" | "emerald" | "sky";
}

function ModeToggle({
  icon,
  label,
  description,
  checked,
  onChange,
  color,
}: ModeToggleProps) {
  const colorClasses = {
    blue: {
      bg: "bg-blue-500/10",
      border: "border-blue-500/30",
      icon: "text-blue-400",
    },
    purple: {
      bg: "bg-purple-500/10",
      border: "border-purple-500/30",
      icon: "text-purple-400",
    },
    emerald: {
      bg: "bg-emerald-500/10",
      border: "border-emerald-500/30",
      icon: "text-emerald-400",
    },
    sky: {
      bg: "bg-sky-500/10",
      border: "border-sky-500/30",
      icon: "text-sky-400",
    },
  };

  const colors = colorClasses[color];

  return (
    <div
      className={cn(
        "flex items-center justify-between p-3 rounded-lg border transition-all",
        checked
          ? `${colors.bg} ${colors.border}`
          : "bg-gray-800/50 border-gray-700"
      )}
    >
      <div className="flex items-center gap-3">
        <div className={cn("p-2 rounded-md", checked ? colors.bg : "bg-gray-700", colors.icon)}>
          {icon}
        </div>
        <div>
          <div className="text-sm font-medium text-white">{label}</div>
          <div className="text-xs text-gray-400">{description}</div>
        </div>
      </div>
      <Toggle checked={checked} onChange={onChange} />
    </div>
  );
}
