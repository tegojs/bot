import { invoke } from "@tauri-apps/api/core";
import { create, type StateCreator } from "zustand";
import { defaultSettings } from "@/constants";

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

export interface ScreenshotSettings {
  save_folder: string;
  filename_pattern: string;
  image_format: "png" | "webp" | "jpeg";
  auto_copy_clipboard: boolean;
}

export interface AIDialogueSettings {
  api_url: string;
  api_key: string;
  model: string;
  system_prompt: string;
  max_history_messages: number;
}

export interface EnabledModes {
  search: boolean;
  polish: boolean;
  dialogue: boolean;
  switcher: boolean;
}

export interface Settings {
  general: GeneralSettings;
  shortcuts: ShortcutSettings;
  advanced: AdvancedSettings;
  expression_polishing: ExpressionPolishingSettings;
  screenshot: ScreenshotSettings;
  ai_dialogue: AIDialogueSettings;
  enabled_modes: EnabledModes;
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
  updateExpressionPolishing: (
    updates: Partial<ExpressionPolishingSettings>,
  ) => void;
  updateScreenshot: (updates: Partial<ScreenshotSettings>) => void;
  updateAIDialogue: (updates: Partial<AIDialogueSettings>) => void;
  updateEnabledModes: (updates: Partial<EnabledModes>) => void;
}

// Debounce timer for auto-save
let saveDebounceTimer: ReturnType<typeof setTimeout> | null = null;
const SAVE_DEBOUNCE_MS = 500;

// Helper to create section update function
function createSectionUpdater<K extends keyof Settings>(
  sectionKey: K,
  set: (
    fn: (state: SettingsState) => Partial<SettingsState>,
  ) => void,
  get: () => SettingsState,
) {
  return (updates: Partial<Settings[K]>) => {
    set((state) => ({
      settings: {
        ...state.settings,
        [sectionKey]: { ...state.settings[sectionKey], ...updates },
      },
    }));
    // Debounced auto-save
    if (saveDebounceTimer) clearTimeout(saveDebounceTimer);
    saveDebounceTimer = setTimeout(() => get().saveSettings(), SAVE_DEBOUNCE_MS);
  };
}

const createSettingsStore: StateCreator<SettingsState> = (set, get) => ({
  settings: defaultSettings,
  isLoading: true,
  activeSection: "general",

  setActiveSection: (section) => set({ activeSection: section }),

  setSettings: (settings) => set({ settings }),

  loadSettings: async () => {
    try {
      set({ isLoading: true });
      const loaded = await invoke<Partial<Settings>>("get_settings");
      // Merge with defaults to handle missing fields from old settings files
      const settings: Settings = {
        general: { ...defaultSettings.general, ...loaded.general },
        shortcuts: { ...defaultSettings.shortcuts, ...loaded.shortcuts },
        advanced: { ...defaultSettings.advanced, ...loaded.advanced },
        expression_polishing: {
          ...defaultSettings.expression_polishing,
          ...loaded.expression_polishing,
        },
        screenshot: { ...defaultSettings.screenshot, ...loaded.screenshot },
        ai_dialogue: { ...defaultSettings.ai_dialogue, ...loaded.ai_dialogue },
        enabled_modes: {
          ...defaultSettings.enabled_modes,
          ...loaded.enabled_modes,
        },
      };
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

  updateGeneral: createSectionUpdater("general", set, get),
  updateShortcuts: createSectionUpdater("shortcuts", set, get),
  updateAdvanced: createSectionUpdater("advanced", set, get),
  updateExpressionPolishing: createSectionUpdater("expression_polishing", set, get),
  updateScreenshot: createSectionUpdater("screenshot", set, get),
  updateAIDialogue: createSectionUpdater("ai_dialogue", set, get),
  updateEnabledModes: createSectionUpdater("enabled_modes", set, get),
});

export const useSettingsStore = create<SettingsState>(createSettingsStore);
