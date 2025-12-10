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
  open_settings: string;
}

export interface AdvancedSettings {
  debug_mode: boolean;
}

export interface ExpressionPolishingSettings {
  api_url: string;
  api_key: string;
  model: string;
  system_prompt: string;
}

export interface Settings {
  general: GeneralSettings;
  shortcuts: ShortcutSettings;
  advanced: AdvancedSettings;
  expression_polishing: ExpressionPolishingSettings;
}

interface SettingsState {
  settings: Settings;
  isLoading: boolean;
  activeSection: string;
  setActiveSection: (section: string) => void;
  setSettings: (settings: Settings) => void;
  loadSettings: () => Promise<void>;
  saveSettings: () => Promise<void>;
  updateGeneral: (updates: Partial<GeneralSettings>) => void;
  updateShortcuts: (updates: Partial<ShortcutSettings>) => void;
  updateAdvanced: (updates: Partial<AdvancedSettings>) => void;
  updateExpressionPolishing: (updates: Partial<ExpressionPolishingSettings>) => void;
}

const DEFAULT_SYSTEM_PROMPT = `You are an expression polishing assistant. When given text:
1. Provide a polished, improved version of the expression
2. Explain the key adjustments you made

Format your response as:
**Polished:**
[improved text]

**Adjustments:**
[bullet points explaining changes]`;

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
    open_settings: "Ctrl+,",
  },
  advanced: {
    debug_mode: false,
  },
  expression_polishing: {
    api_url: "https://api.openai.com/v1",
    api_key: "",
    model: "gpt-4",
    system_prompt: DEFAULT_SYSTEM_PROMPT,
  },
};

export const useSettingsStore = create<SettingsState>((set, get) => ({
  settings: defaultSettings,
  isLoading: true,
  activeSection: "general",

  setActiveSection: (section) => set({ activeSection: section }),

  setSettings: (settings) => set({ settings }),

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

  updateExpressionPolishing: (updates) => {
    set((state) => ({
      settings: {
        ...state.settings,
        expression_polishing: { ...state.settings.expression_polishing, ...updates },
      },
    }));
    get().saveSettings();
  },
}));
