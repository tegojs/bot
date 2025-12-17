import { listen } from "@tauri-apps/api/event";
import { useEffect } from "react";
import { useSettingsStore } from "@/stores/settingsStore";
import { Sidebar } from "./Sidebar";
import { AboutSettings } from "./sections/AboutSettings";
import { AdvancedSettings } from "./sections/AdvancedSettings";
import { AIDialogueSettings } from "./sections/AIDialogueSettings";
import { ExpressionPolishingSettings } from "./sections/ExpressionPolishingSettings";
import { GeneralSettings } from "./sections/GeneralSettings";
import { PermissionsSettings } from "./sections/PermissionsSettings";
import { ScreenshotSettings } from "./sections/ScreenshotSettings";
import { ShortcutsSettings } from "./sections/ShortcutsSettings";
import { TitleBar } from "./TitleBar";

export function SettingsApp() {
  const { activeSection, setActiveSection, loadSettings, isLoading } =
    useSettingsStore();

  // Load settings on mount
  useEffect(() => {
    loadSettings();
  }, [loadSettings]);

  // Listen for navigation events from tray menu
  useEffect(() => {
    const unlisten = listen<string>("navigate", (event) => {
      setActiveSection(event.payload);
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, [setActiveSection]);

  const renderContent = () => {
    switch (activeSection) {
      case "general":
        return <GeneralSettings />;
      case "shortcuts":
        return <ShortcutsSettings />;
      case "screenshot":
        return <ScreenshotSettings />;
      case "permissions":
        return <PermissionsSettings />;
      case "polishing":
        return <ExpressionPolishingSettings />;
      case "dialogue":
        return <AIDialogueSettings />;
      case "advanced":
        return <AdvancedSettings />;
      case "about":
        return <AboutSettings />;
      default:
        return <GeneralSettings />;
    }
  };

  if (isLoading) {
    return (
      <div className="flex flex-col h-screen text-white bg-black/85 rounded-xl">
        <TitleBar />
        <div className="flex-1 flex items-center justify-center">
          <div className="text-gray-400">Loading settings...</div>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-screen text-white bg-black/85 rounded-xl">
      <TitleBar />
      <div className="flex flex-1 overflow-hidden">
        <Sidebar />
        <main className="flex-1 overflow-y-auto">
          <div className="max-w-2xl mx-auto p-8">{renderContent()}</div>
        </main>
      </div>
    </div>
  );
}
