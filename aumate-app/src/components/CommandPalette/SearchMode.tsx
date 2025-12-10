import { useEffect, useRef } from "react";
import {
  Camera,
  Command,
  FileText,
  Folder,
  Settings as SettingsIcon,
  Terminal,
} from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
import { cn } from "@/lib/utils";

// Command item interface
export interface CommandItem {
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
export const commands: CommandItem[] = [
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

interface SearchModeProps {
  query: string;
  selectedIndex: number;
  onSelectIndex: (index: number) => void;
  onExecuteCommand: (command: CommandItem) => void;
}

export function SearchMode({
  query,
  selectedIndex,
  onSelectIndex,
  onExecuteCommand,
}: SearchModeProps) {
  const listRef = useRef<HTMLDivElement>(null);

  // Filter commands based on search query
  const filteredCommands = commands.filter(
    (cmd) =>
      cmd.title.toLowerCase().includes(query.toLowerCase()) ||
      cmd.description?.toLowerCase().includes(query.toLowerCase())
  );

  // Scroll selected item into view
  useEffect(() => {
    const selectedElement = listRef.current?.children[selectedIndex] as HTMLElement;
    if (selectedElement) {
      selectedElement.scrollIntoView({ block: "nearest" });
    }
  }, [selectedIndex]);

  return (
    <div ref={listRef} className="flex-1 overflow-y-auto command-list py-2">
      {filteredCommands.length === 0 ? (
        <div className="px-4 py-8 text-center text-muted-foreground">
          No commands found
        </div>
      ) : (
        filteredCommands.map((command, index) => (
          <button
            key={command.id}
            type="button"
            onClick={() => onExecuteCommand(command)}
            onMouseEnter={() => onSelectIndex(index)}
            className={cn(
              "w-full flex items-center gap-3 px-4 py-2.5 text-left transition-colors",
              index === selectedIndex
                ? "bg-accent text-accent-foreground"
                : "text-foreground hover:bg-accent/50"
            )}
          >
            <span className="shrink-0 text-muted-foreground">{command.icon}</span>
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
  );
}

// Export filtered commands helper
export function getFilteredCommands(query: string): CommandItem[] {
  return commands.filter(
    (cmd) =>
      cmd.title.toLowerCase().includes(query.toLowerCase()) ||
      cmd.description?.toLowerCase().includes(query.toLowerCase())
  );
}
