import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { AppWindow, MessageSquare, Search, Sparkles } from "lucide-react";
import { useCallback, useEffect, useRef, useState } from "react";
import {
  COMMAND_PALETTE_DIMENSIONS,
  COMPACT_HEIGHT,
} from "@/constants/dimensions";
import { useWindowResize } from "@/hooks/useWindowResize";
import { cn } from "@/lib/utils";
import { type Settings, useSettingsStore } from "@/stores/settingsStore";
import { DialogueMode } from "./DialogueMode";
import { PolishMode } from "./PolishMode";
import { SearchMode } from "./SearchMode";
import { SwitcherMode } from "./SwitcherMode";

type Mode = "search" | "polish" | "dialogue" | "switcher";

export function CommandPalette() {
  const [mode, setMode] = useState<Mode>("search");
  const [query, setQuery] = useState("");
  const inputRef = useRef<HTMLInputElement>(null);

  const { settings, loadSettings, setSettings } = useSettingsStore();
  const windowMode = settings.general.window_mode;
  const enabledModes = settings.enabled_modes;

  // Use the HTML-based window resize hook for smooth animations
  const { resizeTo, animationClass } = useWindowResize({ duration: 200 });

  // Load settings on mount
  useEffect(() => {
    loadSettings();
  }, [loadSettings]);

  // Ensure current mode is enabled, switch to first enabled mode if not
  useEffect(() => {
    const modes: Mode[] = [];
    if (enabledModes.search) modes.push("search");
    if (enabledModes.polish) modes.push("polish");
    if (enabledModes.dialogue) modes.push("dialogue");
    if (enabledModes.switcher) modes.push("switcher");
    const availableModes = modes.length > 0 ? modes : ["search" as Mode];

    if (!availableModes.includes(mode)) {
      setMode(availableModes[0]);
    }
  }, [enabledModes, mode]);

  // Listen for settings-changed events from settings window
  useEffect(() => {
    const unlisten = listen<Settings>("settings-changed", (event) => {
      setSettings(event.payload);
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, [setSettings]);

  // Determine if content area should be shown based on window mode
  const showContent = (() => {
    // Dialogue and switcher modes always show content
    if (mode === "dialogue" || mode === "switcher") return true;
    if (windowMode === "expanded") return true;

    // Compact mode: show only when there's input
    return query.trim().length > 0;
  })();

  // Resize window based on mode and content visibility
  useEffect(() => {
    const handleResize = async () => {
      const dims = COMMAND_PALETTE_DIMENSIONS[mode];

      if (mode === "dialogue" || mode === "switcher") {
        // Dialogue and switcher modes always use their dimensions
        await resizeTo(dims.width, dims.height);
      } else if (windowMode === "compact") {
        // Compact mode: resize based on content
        const targetHeight = showContent ? dims.height : COMPACT_HEIGHT;
        await resizeTo(dims.width, targetHeight);
      } else {
        // Expanded mode - ensure full height
        await resizeTo(dims.width, dims.height);
      }
    };

    handleResize();
  }, [mode, showContent, windowMode, resizeTo]);

  // Hide window function
  const hideWindow = useCallback(async () => {
    try {
      await invoke("hide_command_palette");
    } catch {
      const window = getCurrentWindow();
      await window.hide();
    }
  }, []);

  // Get list of enabled modes
  const getEnabledModesList = useCallback((): Mode[] => {
    const modes: Mode[] = [];
    if (enabledModes.search) modes.push("search");
    if (enabledModes.polish) modes.push("polish");
    if (enabledModes.dialogue) modes.push("dialogue");
    if (enabledModes.switcher) modes.push("switcher");
    // Ensure at least search mode is always available
    return modes.length > 0 ? modes : ["search"];
  }, [enabledModes]);

  // Cycle through enabled modes only
  const cycleMode = useCallback(() => {
    const modes = getEnabledModesList();
    if (modes.length <= 1) return; // No cycling if only one mode

    setMode((prev) => {
      const currentIndex = modes.indexOf(prev);
      if (currentIndex === -1) return modes[0]; // Fallback to first enabled mode
      const nextIndex = (currentIndex + 1) % modes.length;
      return modes[nextIndex];
    });
    setQuery("");
  }, [getEnabledModesList]);

  // Open settings window
  const openSettings = useCallback(async () => {
    const { WebviewWindow } = await import("@tauri-apps/api/webviewWindow");
    const settingsWindow = await WebviewWindow.getByLabel("settings");
    if (settingsWindow) {
      await settingsWindow.show();
      await settingsWindow.center();
      await settingsWindow.setFocus();
    }
  }, []);

  // Global keyboard shortcuts (Tab, Escape, Ctrl+,)
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // Ctrl+, to open settings
      if (e.ctrlKey && e.key === ",") {
        e.preventDefault();
        openSettings();
        return;
      }

      // Tab to cycle modes
      if (e.key === "Tab") {
        e.preventDefault();
        cycleMode();
        return;
      }

      // Escape to hide window
      if (e.key === "Escape") {
        e.preventDefault();
        hideWindow();
        return;
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [cycleMode, hideWindow, openSettings]);

  // Focus input on mount and window focus (except for dialogue mode)
  useEffect(() => {
    if (mode !== "dialogue") {
      inputRef.current?.focus();
    }

    const window = getCurrentWindow();
    const unlisten = window.onFocusChanged(({ payload: focused }) => {
      if (focused && mode !== "dialogue") {
        inputRef.current?.focus();
      } else if (!focused) {
        hideWindow();
      }
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, [hideWindow, mode]);

  // Get mode label for Tab badge
  const getNextModeLabel = () => {
    const modes = getEnabledModesList();
    if (modes.length <= 1) return null; // No cycling available

    const currentIndex = modes.indexOf(mode);
    const nextIndex = (currentIndex + 1) % modes.length;
    const nextMode = modes[nextIndex];

    if (nextMode === "search") return "Search";
    if (nextMode === "polish") return "Polish";
    if (nextMode === "switcher") return "Switcher";
    return "Dialogue";
  };

  // Get mode icon
  const getModeIcon = () => {
    if (mode === "search") {
      return <Search className="w-5 h-5 text-muted-foreground shrink-0" />;
    }
    if (mode === "polish") {
      return <Sparkles className="w-5 h-5 text-purple-400 shrink-0" />;
    }
    if (mode === "switcher") {
      return <AppWindow className="w-5 h-5 text-sky-400 shrink-0" />;
    }
    return <MessageSquare className="w-5 h-5 text-emerald-400 shrink-0" />;
  };

  return (
    <div
      className={cn(
        "w-full h-full flex flex-col overflow-hidden window-content",
        animationClass,
      )}
    >
      {/* Top Bar */}
      <div
        className={cn(
          "flex items-center gap-3 px-4 py-3",
          showContent && "border-b border-white/10",
        )}
      >
        {getModeIcon()}
        {mode === "dialogue" ? (
          <span className="flex-1 text-base text-muted-foreground">
            AI Dialogue
          </span>
        ) : (
          <input
            ref={inputRef}
            type="text"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            placeholder={
              mode === "search"
                ? "Search commands..."
                : mode === "switcher"
                  ? "Search open windows..."
                  : "Enter text to polish..."
            }
            className="flex-1 bg-transparent text-foreground text-base placeholder:text-muted-foreground focus:outline-none"
            autoComplete="off"
            autoCorrect="off"
            autoCapitalize="off"
            spellCheck={false}
          />
        )}
        <div className="flex items-center gap-2">
          {getNextModeLabel() && (
            <button
              type="button"
              className={cn(
                "inline-flex items-center gap-1 px-2 py-1 text-xs font-medium rounded cursor-pointer transition-colors font-mono",
                mode === "search" &&
                  "text-muted-foreground bg-muted hover:bg-accent",
                mode === "polish" &&
                  "text-purple-300 bg-purple-500/20 hover:bg-purple-500/30",
                mode === "dialogue" &&
                  "text-emerald-300 bg-emerald-500/20 hover:bg-emerald-500/30",
                mode === "switcher" &&
                  "text-sky-300 bg-sky-500/20 hover:bg-sky-500/30",
              )}
              onClick={cycleMode}
            >
              Tab: {getNextModeLabel()}
            </button>
          )}
        </div>
      </div>

      {/* Content Area */}
      {showContent && (
        <>
          {mode === "search" && (
            <SearchMode query={query} onHide={hideWindow} isActive={true} />
          )}

          {mode === "polish" && <PolishMode query={query} isActive={true} />}

          {mode === "dialogue" && <DialogueMode />}

          {mode === "switcher" && (
            <SwitcherMode query={query} onHide={hideWindow} isActive={true} />
          )}
        </>
      )}
    </div>
  );
}

export default CommandPalette;
