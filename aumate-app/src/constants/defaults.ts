// Default settings values

import type { Settings } from "@/stores/settingsStore";
import { DEFAULT_DIALOGUE_PROMPT, DEFAULT_POLISH_PROMPT } from "./prompts";

export const DEFAULT_API_URL = "https://api.openai.com/v1";
export const DEFAULT_MODEL = "gpt-4";
export const DEFAULT_HOTKEY = "F3";
export const DEFAULT_MAX_HISTORY = 20;
export const DEFAULT_FILENAME_PATTERN = "screenshot_%Y%m%d_%H%M%S";

export const defaultSettings: Settings = {
  general: {
    follow_system_appearance: true,
    open_at_login: false,
    show_in_system_tray: true,
    hotkey: DEFAULT_HOTKEY,
    window_mode: "compact",
  },
  shortcuts: {
    toggle_palette: DEFAULT_HOTKEY,
    open_settings: "Ctrl+,",
  },
  advanced: {
    debug_mode: false,
  },
  expression_polishing: {
    api_url: DEFAULT_API_URL,
    api_key: "",
    model: DEFAULT_MODEL,
    system_prompt: DEFAULT_POLISH_PROMPT,
  },
  screenshot: {
    save_folder: "",
    filename_pattern: DEFAULT_FILENAME_PATTERN,
    image_format: "png",
    auto_copy_clipboard: true,
  },
  ai_dialogue: {
    api_url: DEFAULT_API_URL,
    api_key: "",
    model: DEFAULT_MODEL,
    system_prompt: DEFAULT_DIALOGUE_PROMPT,
    max_history_messages: DEFAULT_MAX_HISTORY,
  },
  enabled_modes: {
    search: true,
    polish: true,
    dialogue: true,
    switcher: true,
  },
};
