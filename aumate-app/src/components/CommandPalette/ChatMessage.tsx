import { Bot, User } from "lucide-react";
import Markdown from "react-markdown";
import { cn } from "@/lib/utils";
import type { DialogueMessage } from "@/types/dialogue";

interface ChatMessageProps {
  message: DialogueMessage;
  isStreaming?: boolean;
}

export function ChatMessage({ message, isStreaming }: ChatMessageProps) {
  const isUser = message.role === "user";

  return (
    <div
      className={cn(
        "flex gap-3 px-4 py-3",
        isUser ? "bg-transparent" : "bg-white/5"
      )}
    >
      <div
        className={cn(
          "shrink-0 w-7 h-7 rounded-full flex items-center justify-center",
          isUser ? "bg-blue-500/20 text-blue-400" : "bg-emerald-500/20 text-emerald-400"
        )}
      >
        {isUser ? <User className="w-4 h-4" /> : <Bot className="w-4 h-4" />}
      </div>
      <div className="flex-1 min-w-0">
        <div className="text-xs text-muted-foreground mb-1">
          {isUser ? "You" : "Assistant"}
        </div>
        <div
          className={cn(
            "text-sm text-foreground prose prose-invert prose-sm max-w-none",
            "prose-p:my-1 prose-ul:my-1 prose-li:my-0.5 prose-headings:my-2",
            "prose-code:text-emerald-300 prose-code:bg-white/10 prose-code:px-1 prose-code:py-0.5 prose-code:rounded",
            isStreaming && "animate-pulse"
          )}
        >
          {isUser ? (
            <p className="whitespace-pre-wrap">{message.content}</p>
          ) : (
            <Markdown>{message.content || "..."}</Markdown>
          )}
        </div>
      </div>
    </div>
  );
}
