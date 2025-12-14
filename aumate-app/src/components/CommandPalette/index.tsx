import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import {
  AppWindow,
  MessageSquare,
  Search,
  Sparkles,
  Square,
} from "lucide-react";
import { useCallback, useEffect, useRef, useState } from "react";
import {
  COMMAND_PALETTE_DIMENSIONS,
  COMPACT_HEIGHT,
} from "@/constants/dimensions";
import { polishExpression } from "@/lib/openai";
import { cn } from "@/lib/utils";
import { animateResizeAndCenter } from "@/lib/window";
import { type Settings, useSettingsStore } from "@/stores/settingsStore";
import { DialogueMode } from "./DialogueMode";
import { extractPolishedExpression, PolishMode } from "./PolishMode";
import {
  type CommandItem,
  getFilteredCommands,
  SearchMode,
} from "./SearchMode";
import { SwitcherMode, type WindowItemData } from "./SwitcherMode";

type Mode = "search" | "polish" | "dialogue" | "switcher";

export function CommandPalette() {
  const [mode, setMode] = useState<Mode>("search");
  const [query, setQuery] = useState("");
  const [selectedIndex, setSelectedIndex] = useState(0);
  const [polishResult, setPolishResult] = useState("");
  const [polishError, setPolishError] = useState("");
  const [isPolishing, setIsPolishing] = useState(false);
  const inputRef = useRef<HTMLInputElement>(null);
  const abortControllerRef = useRef<AbortController | null>(null);
  const polishScrollRef = useRef<HTMLDivElement>(null);

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

    // Compact mode: show only when there's input or polish results
    if (mode === "search") {
      return query.trim().length > 0;
    }
    return (
      query.trim().length > 0 || polishResult || polishError || isPolishing
    );
  })();

  // Resize window based on mode and content visibility
  useEffect(() => {
    const resizeWindow = async () => {
      const dims = COMMAND_PALETTE_DIMENSIONS[mode];

      if (mode === "dialogue" || mode === "switcher") {
        // Dialogue and switcher modes always use their dimensions
        await animateResizeAndCenter(dims.width, dims.height);
      } else if (windowMode === "compact") {
        // Compact mode: resize based on content
        const targetHeight = showContent ? dims.height : COMPACT_HEIGHT;
        await animateResizeAndCenter(dims.width, targetHeight);
      } else {
        // Expanded mode - ensure full height
        await animateResizeAndCenter(dims.width, dims.height);
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
    [hideWindow],
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

  // Copy polished expression to clipboard (only the polished text, not adjustments)
  const copyToClipboard = useCallback(async () => {
    if (polishResult) {
      const polishedOnly = extractPolishedExpression(polishResult);
      await navigator.clipboard.writeText(polishedOnly);
    }
  }, [polishResult]);

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

  // Switcher mode state
  const [switcherWindows, setSwitcherWindows] = useState<WindowItemData[]>([]);

  // Switch to a window
  const handleSwitchToWindow = useCallback(
    async (windowId: number) => {
      try {
        await invoke("switch_to_window", { windowId });
        hideWindow();
      } catch (err) {
        console.error("Failed to switch window:", err);
      }
    },
    [hideWindow],
  );

  // Close a window
  const handleCloseWindow = useCallback(async (windowId: number) => {
    try {
      await invoke("close_window", { windowId });
      // Remove from local list
      setSwitcherWindows((prev) =>
        prev.filter((w) => w.window_id !== windowId),
      );
    } catch (err) {
      console.error("Failed to close window:", err);
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

      // Ctrl+P for arrow up (search/switcher mode) or scroll up (polish mode)
      if (e.ctrlKey && e.key === "p") {
        e.preventDefault();
        if (mode === "search" || mode === "switcher") {
          setSelectedIndex((prev) => (prev > 0 ? prev - 1 : prev));
        } else if (mode === "polish" && polishScrollRef.current) {
          polishScrollRef.current.scrollBy({ top: -100, behavior: "smooth" });
        }
        return;
      }

      // Ctrl+N for arrow down (search/switcher mode) or scroll down (polish mode)
      if (e.ctrlKey && e.key === "n") {
        e.preventDefault();
        if (mode === "search") {
          setSelectedIndex((prev) =>
            prev < filteredCommands.length - 1 ? prev + 1 : prev,
          );
        } else if (mode === "switcher") {
          setSelectedIndex((prev) =>
            prev < switcherWindows.length - 1 ? prev + 1 : prev,
          );
        } else if (mode === "polish" && polishScrollRef.current) {
          polishScrollRef.current.scrollBy({ top: 100, behavior: "smooth" });
        }
        return;
      }

      // Ctrl+W to close window in switcher mode
      if (e.ctrlKey && e.key === "w") {
        if (mode === "switcher" && switcherWindows[selectedIndex]) {
          e.preventDefault();
          handleCloseWindow(switcherWindows[selectedIndex].window_id);
        }
        return;
      }

      // Ctrl+C in polish mode: copy polished expression if no text selected
      if (e.ctrlKey && e.key === "c") {
        if (mode === "polish" && polishResult) {
          const selection = window.getSelection();
          const hasSelection = selection && selection.toString().length > 0;
          if (!hasSelection) {
            e.preventDefault();
            copyToClipboard();
          }
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
              prev < filteredCommands.length - 1 ? prev + 1 : prev,
            );
          } else if (mode === "switcher") {
            e.preventDefault();
            setSelectedIndex((prev) =>
              prev < switcherWindows.length - 1 ? prev + 1 : prev,
            );
          }
          break;
        case "ArrowUp":
          if (mode === "search" || mode === "switcher") {
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
          } else if (mode === "switcher") {
            e.preventDefault();
            if (switcherWindows[selectedIndex]) {
              handleSwitchToWindow(switcherWindows[selectedIndex].window_id);
            }
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
    copyToClipboard,
    polishResult,
    switcherWindows,
    handleSwitchToWindow,
    handleCloseWindow,
  ]);

  // Reset selection when query changes
  useEffect(() => {
    setSelectedIndex(0);
  }, []);

  // Focus input on mount and window focus
  useEffect(() => {
    // All modes except dialogue should focus the input
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
      return (
        <Sparkles
          className={cn(
            "w-5 h-5 shrink-0",
            isPolishing ? "text-blue-400 animate-pulse" : "text-purple-400",
          )}
        />
      );
    }
    if (mode === "switcher") {
      return <AppWindow className="w-5 h-5 text-sky-400 shrink-0" />;
    }
    return <MessageSquare className="w-5 h-5 text-emerald-400 shrink-0" />;
  };

  return (
    <div className="w-full h-full flex flex-col overflow-hidden">
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
            disabled={isPolishing}
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
            <SearchMode
              query={query}
              selectedIndex={selectedIndex}
              onSelectIndex={setSelectedIndex}
              onExecuteCommand={executeCommand}
            />
          )}

          {mode === "polish" && (
            <PolishMode
              ref={polishScrollRef}
              polishResult={polishResult}
              polishError={polishError}
              isPolishing={isPolishing}
              onCopy={copyToClipboard}
              onClear={clearPolishResults}
            />
          )}

          {mode === "dialogue" && <DialogueMode />}

          {mode === "switcher" && (
            <SwitcherMode
              query={query}
              selectedIndex={selectedIndex}
              onSelectIndex={setSelectedIndex}
              onSwitchToWindow={handleSwitchToWindow}
              onWindowsChange={setSwitcherWindows}
            />
          )}

          {/* Footer - only for search/polish modes */}
          {(mode === "search" || mode === "polish") && (
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
                      <kbd className="px-1.5 py-0.5 bg-muted rounded">
                        Enter
                      </kbd>
                      <span>Execute</span>
                    </span>
                  </div>
                  <span>{getFilteredCommands(query).length} commands</span>
                </>
              ) : (
                <>
                  <div className="flex items-center gap-4">
                    <span className="flex items-center gap-1">
                      <kbd className="px-1.5 py-0.5 bg-muted rounded">
                        Enter
                      </kbd>
                      <span>Polish</span>
                    </span>
                    {polishResult && (
                      <>
                        <span className="flex items-center gap-1">
                          <kbd className="px-1.5 py-0.5 bg-muted rounded">
                            Ctrl+P/N
                          </kbd>
                          <span>Scroll</span>
                        </span>
                        <span className="flex items-center gap-1">
                          <kbd className="px-1.5 py-0.5 bg-muted rounded">
                            Ctrl+C
                          </kbd>
                          <span>Copy</span>
                        </span>
                      </>
                    )}
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
                  <span className="text-purple-400/60">
                    Expression Polishing
                  </span>
                </>
              )}
            </div>
          )}

          {/* Footer - switcher mode */}
          {mode === "switcher" && (
            <div className="flex items-center justify-between px-4 py-2 border-t border-white/10 text-xs text-muted-foreground">
              <div className="flex items-center gap-4">
                <span className="flex items-center gap-1">
                  <kbd className="px-1.5 py-0.5 bg-muted rounded">↑</kbd>
                  <kbd className="px-1.5 py-0.5 bg-muted rounded">↓</kbd>
                  <span>Navigate</span>
                </span>
                <span className="flex items-center gap-1">
                  <kbd className="px-1.5 py-0.5 bg-muted rounded">Enter</kbd>
                  <span>Switch</span>
                </span>
                <span className="flex items-center gap-1">
                  <kbd className="px-1.5 py-0.5 bg-muted rounded">Ctrl+W</kbd>
                  <span>Close</span>
                </span>
              </div>
              <span className="text-sky-400/60">Window Switcher</span>
            </div>
          )}
        </>
      )}
    </div>
  );
}

export default CommandPalette;
