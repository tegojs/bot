import { useState, useEffect, useRef, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";
import { listen } from "@tauri-apps/api/event";
import { Search, Sparkles, MessageSquare, Square } from "lucide-react";
import { cn } from "@/lib/utils";
import { polishExpression } from "@/lib/openai";
import { useSettingsStore, type Settings } from "@/stores/settingsStore";
import { SearchMode, getFilteredCommands, type CommandItem } from "./SearchMode";
import { PolishMode } from "./PolishMode";
import { DialogueMode } from "./DialogueMode";

type Mode = "search" | "polish" | "dialogue";

// Window dimensions per mode
const DIMENSIONS = {
  search: { width: 680, height: 400 },
  polish: { width: 680, height: 400 },
  dialogue: { width: 900, height: 600 },
};

const COMPACT_HEIGHT = 56; // Just the input bar

export function CommandPalette() {
  const [mode, setMode] = useState<Mode>("search");
  const [query, setQuery] = useState("");
  const [selectedIndex, setSelectedIndex] = useState(0);
  const [polishResult, setPolishResult] = useState("");
  const [polishError, setPolishError] = useState("");
  const [isPolishing, setIsPolishing] = useState(false);
  const inputRef = useRef<HTMLInputElement>(null);
  const abortControllerRef = useRef<AbortController | null>(null);

  const { settings, loadSettings, setSettings } = useSettingsStore();
  const windowMode = settings.general.window_mode;
  const enabledModes = settings.enabled_modes;

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
    // Dialogue mode always shows content (it has its own layout)
    if (mode === "dialogue") return true;
    if (windowMode === "expanded") return true;

    // Compact mode: show only when there's input or polish results
    if (mode === "search") {
      return query.trim().length > 0;
    }
    return query.trim().length > 0 || polishResult || polishError || isPolishing;
  })();

  // Resize window based on mode and content visibility
  useEffect(() => {
    const resizeWindow = async () => {
      const win = getCurrentWindow();
      const dims = DIMENSIONS[mode];

      if (mode === "dialogue") {
        // Dialogue mode always uses its dimensions
        await win.setSize(new LogicalSize(dims.width, dims.height));
        await win.center();
      } else if (windowMode === "compact") {
        // Compact mode: resize based on content
        const targetHeight = showContent ? dims.height : COMPACT_HEIGHT;
        await win.setSize(new LogicalSize(dims.width, targetHeight));
        await win.center();
      } else {
        // Expanded mode - ensure full height
        await win.setSize(new LogicalSize(dims.width, dims.height));
        await win.center();
      }
    };

    resizeWindow();
  }, [mode, showContent, windowMode]);

  // Hide window function
  const hideWindow = useCallback(async () => {
    try {
      await invoke("hide_command_palette");
    } catch {
      const window = getCurrentWindow();
      await window.hide();
    }
  }, []);

  // Execute selected command
  const executeCommand = useCallback(
    (command: CommandItem) => {
      command.action();
      setQuery("");
      setSelectedIndex(0);
      hideWindow();
    },
    [hideWindow]
  );

  // Cancel polishing request
  const cancelPolishing = useCallback(() => {
    if (abortControllerRef.current) {
      abortControllerRef.current.abort();
      abortControllerRef.current = null;
    }
    setIsPolishing(false);
  }, []);

  // Polish expression
  const doPolish = useCallback(async () => {
    if (!query.trim() || isPolishing) return;

    const { expression_polishing } = settings;

    setIsPolishing(true);
    setPolishResult("");
    setPolishError("");

    abortControllerRef.current = new AbortController();

    const result = await polishExpression({
      apiUrl: expression_polishing.api_url,
      apiKey: expression_polishing.api_key,
      model: expression_polishing.model,
      systemPrompt: expression_polishing.system_prompt,
      userInput: query,
      signal: abortControllerRef.current.signal,
      onChunk: (chunk) => {
        setPolishResult((prev) => prev + chunk);
      },
    });

    setIsPolishing(false);
    abortControllerRef.current = null;

    if (result.error) {
      setPolishError(result.error);
    }
  }, [query, settings, isPolishing]);

  // Clear polish results
  const clearPolishResults = useCallback(() => {
    setPolishResult("");
    setPolishError("");
    setQuery("");
    inputRef.current?.focus();
  }, []);

  // Copy result to clipboard
  const copyToClipboard = useCallback(async () => {
    if (polishResult) {
      await navigator.clipboard.writeText(polishResult);
    }
  }, [polishResult]);

  // Get list of enabled modes
  const getEnabledModesList = useCallback((): Mode[] => {
    const modes: Mode[] = [];
    if (enabledModes.search) modes.push("search");
    if (enabledModes.polish) modes.push("polish");
    if (enabledModes.dialogue) modes.push("dialogue");
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
    setPolishResult("");
    setPolishError("");
    setQuery("");
    setSelectedIndex(0);
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

  // Keyboard navigation
  useEffect(() => {
    const filteredCommands = getFilteredCommands(query);

    const handleKeyDown = (e: KeyboardEvent) => {
      // Ctrl+, to open settings (local shortcut)
      if (e.ctrlKey && e.key === ",") {
        e.preventDefault();
        openSettings();
        return;
      }

      // Ctrl+P for arrow up (like Emacs/Spotlight) - only in search mode
      if (e.ctrlKey && e.key === "p") {
        if (mode === "search") {
          e.preventDefault();
          setSelectedIndex((prev) => (prev > 0 ? prev - 1 : prev));
        }
        return;
      }

      // Ctrl+N for arrow down (like Emacs/Spotlight) - only in search mode
      if (e.ctrlKey && e.key === "n") {
        if (mode === "search") {
          e.preventDefault();
          setSelectedIndex((prev) =>
            prev < filteredCommands.length - 1 ? prev + 1 : prev
          );
        }
        return;
      }

      // Tab to cycle modes
      if (e.key === "Tab") {
        e.preventDefault();
        cycleMode();
        return;
      }

      switch (e.key) {
        case "Escape":
          e.preventDefault();
          if (isPolishing) {
            cancelPolishing();
          } else {
            hideWindow();
          }
          break;
        case "ArrowDown":
          if (mode === "search") {
            e.preventDefault();
            setSelectedIndex((prev) =>
              prev < filteredCommands.length - 1 ? prev + 1 : prev
            );
          }
          break;
        case "ArrowUp":
          if (mode === "search") {
            e.preventDefault();
            setSelectedIndex((prev) => (prev > 0 ? prev - 1 : prev));
          }
          break;
        case "Enter":
          if (mode === "search") {
            e.preventDefault();
            if (filteredCommands[selectedIndex]) {
              executeCommand(filteredCommands[selectedIndex]);
            }
          } else if (mode === "polish") {
            e.preventDefault();
            doPolish();
          }
          // Dialogue mode handles Enter in ChatPanel
          break;
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [
    mode,
    query,
    selectedIndex,
    executeCommand,
    hideWindow,
    cycleMode,
    doPolish,
    isPolishing,
    cancelPolishing,
    openSettings,
  ]);

  // Reset selection when query changes
  useEffect(() => {
    setSelectedIndex(0);
  }, [query]);

  // Focus input on mount and window focus
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
    return "Dialogue";
  };

  // Get mode icon
  const getModeIcon = () => {
    if (mode === "search") {
      return <Search className="w-5 h-5 text-muted-foreground shrink-0" />;
    }
    if (mode === "polish") {
      return (
        <Sparkles
          className={cn(
            "w-5 h-5 shrink-0",
            isPolishing ? "text-blue-400 animate-pulse" : "text-purple-400"
          )}
        />
      );
    }
    return <MessageSquare className="w-5 h-5 text-emerald-400 shrink-0" />;
  };

  return (
    <div className="w-full h-full flex flex-col overflow-hidden">
      {/* Top Bar */}
      <div
        className={cn(
          "flex items-center gap-3 px-4 py-3",
          showContent && "border-b border-white/10"
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
              mode === "search" ? "Search commands..." : "Enter text to polish..."
            }
            className="flex-1 bg-transparent text-foreground text-base placeholder:text-muted-foreground focus:outline-none"
            autoComplete="off"
            autoCorrect="off"
            autoCapitalize="off"
            spellCheck={false}
            disabled={isPolishing}
          />
        )}
        <div className="flex items-center gap-2">
          {getNextModeLabel() && (
            <kbd
              className={cn(
                "inline-flex items-center gap-1 px-2 py-1 text-xs font-medium rounded cursor-pointer transition-colors",
                mode === "search" && "text-muted-foreground bg-muted hover:bg-accent",
                mode === "polish" &&
                  "text-purple-300 bg-purple-500/20 hover:bg-purple-500/30",
                mode === "dialogue" &&
                  "text-emerald-300 bg-emerald-500/20 hover:bg-emerald-500/30"
              )}
              onClick={cycleMode}
            >
              Tab: {getNextModeLabel()}
            </kbd>
          )}
        </div>
      </div>

      {/* Content Area */}
      {showContent && (
        <>
          {mode === "search" && (
            <SearchMode
              query={query}
              selectedIndex={selectedIndex}
              onSelectIndex={setSelectedIndex}
              onExecuteCommand={executeCommand}
            />
          )}

          {mode === "polish" && (
            <PolishMode
              polishResult={polishResult}
              polishError={polishError}
              isPolishing={isPolishing}
              onCopy={copyToClipboard}
              onClear={clearPolishResults}
            />
          )}

          {mode === "dialogue" && <DialogueMode />}

          {/* Footer - only for search/polish modes */}
          {mode !== "dialogue" && (
            <div className="flex items-center justify-between px-4 py-2 border-t border-white/10 text-xs text-muted-foreground">
              {mode === "search" ? (
                <>
                  <div className="flex items-center gap-4">
                    <span className="flex items-center gap-1">
                      <kbd className="px-1.5 py-0.5 bg-muted rounded">↑</kbd>
                      <kbd className="px-1.5 py-0.5 bg-muted rounded">↓</kbd>
                      <span>Navigate</span>
                    </span>
                    <span className="flex items-center gap-1">
                      <kbd className="px-1.5 py-0.5 bg-muted rounded">Enter</kbd>
                      <span>Execute</span>
                    </span>
                  </div>
                  <span>{getFilteredCommands(query).length} commands</span>
                </>
              ) : (
                <>
                  <div className="flex items-center gap-4">
                    <span className="flex items-center gap-1">
                      <kbd className="px-1.5 py-0.5 bg-muted rounded">Enter</kbd>
                      <span>Polish</span>
                    </span>
                    {isPolishing && (
                      <button
                        type="button"
                        onClick={cancelPolishing}
                        className="flex items-center gap-1 text-red-400 hover:text-red-300"
                      >
                        <Square className="w-3 h-3" />
                        <span>Stop</span>
                      </button>
                    )}
                  </div>
                  <span className="text-purple-400/60">Expression Polishing</span>
                </>
              )}
            </div>
          )}
        </>
      )}
    </div>
  );
}

export default CommandPalette;
