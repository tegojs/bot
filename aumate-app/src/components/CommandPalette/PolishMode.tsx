import { Copy, Sparkles, X } from "lucide-react";
import Markdown from "react-markdown";

interface PolishModeProps {
  polishResult: string;
  polishError: string;
  isPolishing: boolean;
  onCopy: () => void;
  onClear: () => void;
}

export function PolishMode({
  polishResult,
  polishError,
  isPolishing,
  onCopy,
  onClear,
}: PolishModeProps) {
  return (
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
            <span className="text-xs font-medium text-purple-400">Result</span>
            <div className="flex items-center gap-1">
              <button
                type="button"
                onClick={onCopy}
                className="p-1.5 text-muted-foreground hover:text-foreground hover:bg-white/5 rounded transition-colors"
                title="Copy to clipboard"
              >
                <Copy className="w-4 h-4" />
              </button>
              <button
                type="button"
                onClick={onClear}
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
  );
}
