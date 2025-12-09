import {
  Settings,
  Keyboard,
  Wrench,
  Info,
  User,
} from "lucide-react";
import { cn } from "@/lib/utils";
import { useSettingsStore } from "@/stores/settingsStore";

interface NavItem {
  id: string;
  label: string;
  icon: React.ReactNode;
}

const navItems: NavItem[] = [
  { id: "general", label: "General", icon: <Settings className="w-5 h-5" /> },
  { id: "shortcuts", label: "Shortcuts", icon: <Keyboard className="w-5 h-5" /> },
  { id: "advanced", label: "Advanced", icon: <Wrench className="w-5 h-5" /> },
  { id: "about", label: "About", icon: <Info className="w-5 h-5" /> },
];

export function Sidebar() {
  const { activeSection, setActiveSection } = useSettingsStore();

  return (
    <div className="w-56 bg-gray-900/50 border-r border-white/10 flex flex-col">
      {/* User section */}
      <div className="p-4 border-b border-white/10">
        <div className="flex items-center gap-3">
          <div className="w-10 h-10 rounded-full bg-gray-700 flex items-center justify-center">
            <User className="w-5 h-5 text-gray-400" />
          </div>
          <div>
            <div className="text-sm font-medium text-white">Not Signed In</div>
            <div className="text-xs text-gray-400">Account</div>
          </div>
        </div>
      </div>

      {/* Navigation */}
      <nav className="flex-1 p-2">
        {navItems.map((item) => (
          <button
            key={item.id}
            onClick={() => setActiveSection(item.id)}
            className={cn(
              "w-full flex items-center gap-3 px-3 py-2 rounded-lg text-sm font-medium transition-colors",
              activeSection === item.id
                ? "bg-blue-600 text-white"
                : "text-gray-300 hover:bg-white/5 hover:text-white"
            )}
          >
            {item.icon}
            {item.label}
          </button>
        ))}
      </nav>
    </div>
  );
}
