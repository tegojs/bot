import { useState, useEffect, useRef, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import {
  Search,
  Command,
  Settings,
  FileText,
  Terminal,
  Folder,
} from "lucide-react";
import { cn } from "@/lib/utils";

// Command item interface
interface CommandItem {
  id: string;
  title: string;
  description?: string;
  icon: React.ReactNode;
  shortcut?: string;
  action: () => void;
}

// Mock commands for demonstration
const mockCommands: CommandItem[] = [
  {
    id: "1",
    title: "Open Settings",
    description: "Configure application preferences",
    icon: <Settings className="w-4 h-4" />,
    shortcut: "Ctrl+,",
    action: () => console.log("Open Settings"),
  },
  {
    id: "2",
    title: "New File",
    description: "Create a new file",
    icon: <FileText className="w-4 h-4" />,
    shortcut: "Ctrl+N",
    action: () => console.log("New File"),
  },
  {
    id: "3",
    title: "Open Terminal",
    description: "Open integrated terminal",
    icon: <Terminal className="w-4 h-4" />,
    shortcut: "Ctrl+`",
    action: () => console.log("Open Terminal"),
  },
  {
    id: "4",
    title: "Open Folder",
    description: "Open a folder in the workspace",
    icon: <Folder className="w-4 h-4" />,
    shortcut: "Ctrl+O",
    action: () => console.log("Open Folder"),
  },
  {
    id: "5",
    title: "Command Palette",
    description: "Show all commands",
    icon: <Command className="w-4 h-4" />,
    shortcut: "Ctrl+Shift+P",
    action: () => console.log("Command Palette"),
  },
];

export function CommandPalette() {
  const [query, setQuery] = useState("");
  const [selectedIndex, setSelectedIndex] = useState(0);
  const inputRef = useRef<HTMLInputElement>(null);
  const listRef = useRef<HTMLDivElement>(null);

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

  // Keyboard navigation
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      switch (e.key) {
        case "Escape":
          e.preventDefault();
          hideWindow();
          break;
        case "ArrowDown":
          e.preventDefault();
          setSelectedIndex((prev) =>
            prev < filteredCommands.length - 1 ? prev + 1 : prev
          );
          break;
        case "ArrowUp":
          e.preventDefault();
          setSelectedIndex((prev) => (prev > 0 ? prev - 1 : prev));
          break;
        case "Enter":
          e.preventDefault();
          if (filteredCommands[selectedIndex]) {
            executeCommand(filteredCommands[selectedIndex]);
          }
          break;
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [filteredCommands, selectedIndex, executeCommand, hideWindow]);

  // Reset selection when query changes
  useEffect(() => {
    setSelectedIndex(0);
  }, [query]);

  // Scroll selected item into view
  useEffect(() => {
    const selectedElement = listRef.current?.children[
      selectedIndex
    ] as HTMLElement;
    if (selectedElement) {
      selectedElement.scrollIntoView({ block: "nearest" });
    }
  }, [selectedIndex]);

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
        <Search className="w-5 h-5 text-muted-foreground shrink-0" />
        <input
          ref={inputRef}
          type="text"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          placeholder="Search commands..."
          className="flex-1 bg-transparent text-foreground text-base placeholder:text-muted-foreground focus:outline-none"
          autoComplete="off"
          autoCorrect="off"
          autoCapitalize="off"
          spellCheck={false}
        />
        <kbd className="hidden sm:inline-flex items-center gap-1 px-2 py-1 text-xs font-medium text-muted-foreground bg-muted rounded">
          ESC
        </kbd>
      </div>

      {/* Command List */}
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

      {/* Footer */}
      <div className="flex items-center justify-between px-4 py-2 border-t border-white/10 text-xs text-muted-foreground">
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
      </div>
    </div>
  );
}
