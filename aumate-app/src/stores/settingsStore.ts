import { create } from "zustand";
import { invoke } from "@tauri-apps/api/core";

export interface GeneralSettings {
  follow_system_appearance: boolean;
  open_at_login: boolean;
  show_in_system_tray: boolean;
  hotkey: string;
  window_mode: "compact" | "expanded";
}

export interface ShortcutSettings {
  toggle_palette: string;
}

export interface AdvancedSettings {
  debug_mode: boolean;
}

export interface Settings {
  general: GeneralSettings;
  shortcuts: ShortcutSettings;
  advanced: AdvancedSettings;
}

interface SettingsState {
  settings: Settings;
  isLoading: boolean;
  activeSection: string;
  setActiveSection: (section: string) => void;
  loadSettings: () => Promise<void>;
  saveSettings: () => Promise<void>;
  updateGeneral: (updates: Partial<GeneralSettings>) => void;
  updateShortcuts: (updates: Partial<ShortcutSettings>) => void;
  updateAdvanced: (updates: Partial<AdvancedSettings>) => void;
}

const defaultSettings: Settings = {
  general: {
    follow_system_appearance: true,
    open_at_login: false,
    show_in_system_tray: true,
    hotkey: "F3",
    window_mode: "compact",
  },
  shortcuts: {
    toggle_palette: "F3",
  },
  advanced: {
    debug_mode: false,
  },
};

export const useSettingsStore = create<SettingsState>((set, get) => ({
  settings: defaultSettings,
  isLoading: true,
  activeSection: "general",

  setActiveSection: (section) => set({ activeSection: section }),

  loadSettings: async () => {
    try {
      set({ isLoading: true });
      const settings = await invoke<Settings>("get_settings");
      set({ settings, isLoading: false });
    } catch (error) {
      console.error("Failed to load settings:", error);
      set({ isLoading: false });
    }
  },

  saveSettings: async () => {
    try {
      const { settings } = get();
      await invoke("save_settings", { settings });
    } catch (error) {
      console.error("Failed to save settings:", error);
    }
  },

  updateGeneral: (updates) => {
    set((state) => ({
      settings: {
        ...state.settings,
        general: { ...state.settings.general, ...updates },
      },
    }));
    // Auto-save after update
    get().saveSettings();
  },

  updateShortcuts: (updates) => {
    set((state) => ({
      settings: {
        ...state.settings,
        shortcuts: { ...state.settings.shortcuts, ...updates },
      },
    }));
    get().saveSettings();
  },

  updateAdvanced: (updates) => {
    set((state) => ({
      settings: {
        ...state.settings,
        advanced: { ...state.settings.advanced, ...updates },
      },
    }));
    get().saveSettings();
  },
}));
