import { useState, useEffect, useRef, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { listen } from "@tauri-apps/api/event";
import {
  Search,
  Command,
  Settings as SettingsIcon,
  FileText,
  Terminal,
  Folder,
  Sparkles,
  Copy,
  X,
  Square,
  Camera,
} from "lucide-react";
import Markdown from "react-markdown";
import { cn } from "@/lib/utils";
import { polishExpression } from "@/lib/openai";
import { useSettingsStore, type Settings } from "@/stores/settingsStore";

type Mode = "search" | "polish";

// Command item interface
interface CommandItem {
  id: string;
  title: string;
  description?: string;
  icon: React.ReactNode;
  shortcut?: string;
  action: () => void;
}

// Start screenshot function
const startScreenshot = async () => {
  try {
    await invoke("start_screenshot");
  } catch (error) {
    console.error("Failed to start screenshot:", error);
  }
};

// Open settings function
const openSettingsWindow = async () => {
  const { WebviewWindow } = await import("@tauri-apps/api/webviewWindow");
  const settingsWindow = await WebviewWindow.getByLabel("settings");
  if (settingsWindow) {
    await settingsWindow.show();
    await settingsWindow.center();
    await settingsWindow.setFocus();
  }
};

// Commands for the palette
const mockCommands: CommandItem[] = [
  {
    id: "screenshot",
    title: "Take Screenshot",
    description: "Capture screen region or window",
    icon: <Camera className="w-4 h-4" />,
    shortcut: "PrintScreen",
    action: startScreenshot,
  },
  {
    id: "settings",
    title: "Open Settings",
    description: "Configure application preferences",
    icon: <SettingsIcon className="w-4 h-4" />,
    shortcut: "Ctrl+,",
    action: openSettingsWindow,
  },
  {
    id: "new-file",
    title: "New File",
    description: "Create a new file",
    icon: <FileText className="w-4 h-4" />,
    shortcut: "Ctrl+N",
    action: () => console.log("New File"),
  },
  {
    id: "terminal",
    title: "Open Terminal",
    description: "Open integrated terminal",
    icon: <Terminal className="w-4 h-4" />,
    shortcut: "Ctrl+`",
    action: () => console.log("Open Terminal"),
  },
  {
    id: "folder",
    title: "Open Folder",
    description: "Open a folder in the workspace",
    icon: <Folder className="w-4 h-4" />,
    shortcut: "Ctrl+O",
    action: () => console.log("Open Folder"),
  },
  {
    id: "commands",
    title: "Command Palette",
    description: "Show all commands",
    icon: <Command className="w-4 h-4" />,
    shortcut: "Ctrl+Shift+P",
    action: () => console.log("Command Palette"),
  },
];

export function CommandPalette() {
  const [mode, setMode] = useState<Mode>("search");
  const [query, setQuery] = useState("");
  const [selectedIndex, setSelectedIndex] = useState(0);
  const [polishResult, setPolishResult] = useState("");
  const [polishError, setPolishError] = useState("");
  const [isPolishing, setIsPolishing] = useState(false);
  const inputRef = useRef<HTMLInputElement>(null);
  const listRef = useRef<HTMLDivElement>(null);
  const abortControllerRef = useRef<AbortController | null>(null);

  const { settings, loadSettings, setSettings } = useSettingsStore();
  const windowMode = settings.general.window_mode;

  // Load settings on mount
  useEffect(() => {
    loadSettings();
  }, [loadSettings]);

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
  const shouldShowContent = () => {
    if (windowMode === "expanded") {
      return true;
    }
    // Compact mode: show only when there's input or polish results
    if (mode === "search") {
      return query.trim().length > 0;
    } else {
      return query.trim().length > 0 || polishResult || polishError || isPolishing;
    }
  };

  // Filter commands based on search query
  const filteredCommands = mockCommands.filter(
    (cmd) =>
      cmd.title.toLowerCase().includes(query.toLowerCase()) ||
      cmd.description?.toLowerCase().includes(query.toLowerCase())
  );

  // Hide window function
  const hideWindow = useCallback(async () => {
    try {
      await invoke("hide_command_palette");
    } catch {
      // Fallback to frontend API
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

  // Toggle mode
  const toggleMode = useCallback(() => {
    setMode((prev) => (prev === "search" ? "polish" : "search"));
    setPolishResult("");
    setPolishError("");
    setQuery("");
    setSelectedIndex(0);
  }, []);

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
    const handleKeyDown = (e: KeyboardEvent) => {
      // Ctrl+, to open settings (local shortcut)
      if (e.ctrlKey && e.key === ",") {
        e.preventDefault();
        openSettings();
        return;
      }

      // Tab to switch modes
      if (e.key === "Tab") {
        e.preventDefault();
        toggleMode();
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
          e.preventDefault();
          if (mode === "search") {
            if (filteredCommands[selectedIndex]) {
              executeCommand(filteredCommands[selectedIndex]);
            }
          } else {
            // Polish mode - trigger polish
            doPolish();
          }
          break;
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [
    mode,
    filteredCommands,
    selectedIndex,
    executeCommand,
    hideWindow,
    toggleMode,
    doPolish,
    isPolishing,
    cancelPolishing,
    openSettings,
  ]);

  // Reset selection when query changes
  useEffect(() => {
    setSelectedIndex(0);
  }, [query]);

  // Scroll selected item into view
  useEffect(() => {
    if (mode === "search") {
      const selectedElement = listRef.current?.children[
        selectedIndex
      ] as HTMLElement;
      if (selectedElement) {
        selectedElement.scrollIntoView({ block: "nearest" });
      }
    }
  }, [selectedIndex, mode]);

  // Focus input on mount and window focus
  useEffect(() => {
    inputRef.current?.focus();

    const window = getCurrentWindow();
    const unlisten = window.onFocusChanged(({ payload: focused }) => {
      if (focused) {
        inputRef.current?.focus();
      } else {
        // Hide on blur (clicking outside)
        hideWindow();
      }
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, [hideWindow]);

  return (
    <div className="w-full h-full flex flex-col overflow-hidden">
      {/* Search Input */}
      <div className="flex items-center gap-3 px-4 py-3 border-b border-white/10">
        {mode === "search" ? (
          <Search className="w-5 h-5 text-muted-foreground shrink-0" />
        ) : (
          <Sparkles
            className={cn(
              "w-5 h-5 shrink-0",
              isPolishing ? "text-blue-400 animate-pulse" : "text-purple-400"
            )}
          />
        )}
        <input
          ref={inputRef}
          type="text"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          placeholder={
            mode === "search"
              ? "Search commands..."
              : "Enter text to polish..."
          }
          className="flex-1 bg-transparent text-foreground text-base placeholder:text-muted-foreground focus:outline-none"
          autoComplete="off"
          autoCorrect="off"
          autoCapitalize="off"
          spellCheck={false}
          disabled={isPolishing}
        />
        <div className="flex items-center gap-2">
          <kbd
            className={cn(
              "inline-flex items-center gap-1 px-2 py-1 text-xs font-medium rounded cursor-pointer transition-colors",
              mode === "search"
                ? "text-muted-foreground bg-muted hover:bg-accent"
                : "text-purple-300 bg-purple-500/20 hover:bg-purple-500/30"
            )}
            onClick={toggleMode}
          >
            Tab: {mode === "search" ? "Polish" : "Search"}
          </kbd>
        </div>
      </div>

      {/* Content Area - shown based on window mode */}
      {shouldShowContent() && (
        <>
          {mode === "search" ? (
            /* Command List */
            <div ref={listRef} className="flex-1 overflow-y-auto command-list py-2">
              {filteredCommands.length === 0 ? (
                <div className="px-4 py-8 text-center text-muted-foreground">
                  No commands found
                </div>
              ) : (
                filteredCommands.map((command, index) => (
                  <button
                    key={command.id}
                    onClick={() => executeCommand(command)}
                    onMouseEnter={() => setSelectedIndex(index)}
                    className={cn(
                      "w-full flex items-center gap-3 px-4 py-2.5 text-left transition-colors",
                      index === selectedIndex
                        ? "bg-accent text-accent-foreground"
                        : "text-foreground hover:bg-accent/50"
                    )}
                  >
                    <span className="shrink-0 text-muted-foreground">
                      {command.icon}
                    </span>
                    <div className="flex-1 min-w-0">
                      <div className="font-medium truncate">{command.title}</div>
                      {command.description && (
                        <div className="text-sm text-muted-foreground truncate">
                          {command.description}
                        </div>
                      )}
                    </div>
                    {command.shortcut && (
                      <kbd className="shrink-0 px-2 py-1 text-xs font-medium text-muted-foreground bg-muted rounded">
                        {command.shortcut}
                      </kbd>
                    )}
                  </button>
                ))
              )}
            </div>
          ) : (
            /* Polish Results Panel */
            <div className="flex-1 overflow-y-auto command-list">
              {!polishResult && !polishError && !isPolishing && (
                <div className="px-4 py-8 text-center text-muted-foreground">
                  <Sparkles className="w-8 h-8 mx-auto mb-3 text-purple-400/50" />
                  <p>Enter text and press Enter to polish</p>
                  <p className="text-xs mt-1 text-muted-foreground/60">
                    AI will improve your expression and explain the changes
                  </p>
                </div>
              )}

              {isPolishing && !polishResult && (
                <div className="px-4 py-8 text-center text-muted-foreground">
                  <Sparkles className="w-8 h-8 mx-auto mb-3 text-purple-400 animate-pulse" />
                  <p>Polishing your expression...</p>
                </div>
              )}

              {polishError && (
                <div className="px-4 py-4">
                  <div className="p-3 bg-red-500/10 border border-red-500/20 rounded-lg">
                    <p className="text-sm text-red-400">{polishError}</p>
                  </div>
                </div>
              )}

              {polishResult && (
                <div className="px-4 py-4 space-y-3">
                  <div className="flex items-center justify-between">
                    <span className="text-xs font-medium text-purple-400">
                      Result
                    </span>
                    <div className="flex items-center gap-1">
                      <button
                        onClick={copyToClipboard}
                        className="p-1.5 text-muted-foreground hover:text-foreground hover:bg-white/5 rounded transition-colors"
                        title="Copy to clipboard"
                      >
                        <Copy className="w-4 h-4" />
                      </button>
                      <button
                        onClick={clearPolishResults}
                        className="p-1.5 text-muted-foreground hover:text-foreground hover:bg-white/5 rounded transition-colors"
                        title="Clear"
                      >
                        <X className="w-4 h-4" />
                      </button>
                    </div>
                  </div>
                  <div className="p-3 bg-white/5 rounded-lg text-sm text-foreground prose prose-invert prose-sm max-w-none prose-p:my-2 prose-ul:my-2 prose-li:my-0.5 prose-headings:my-2 prose-strong:text-purple-300">
                    <Markdown>{polishResult}</Markdown>
                  </div>
                </div>
              )}
            </div>
          )}

          {/* Footer - only shown when content is visible */}
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
                <span>{filteredCommands.length} commands</span>
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
        </>
      )}
    </div>
  );
}
