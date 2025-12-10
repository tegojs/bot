import { useRef, useEffect, useState, useCallback } from "react";
import { Send, Square, MessageSquare } from "lucide-react";
import { cn } from "@/lib/utils";
import { ChatMessage } from "./ChatMessage";
import type { DialogueMessage } from "@/types/dialogue";

interface ChatPanelProps {
  messages: DialogueMessage[];
  isStreaming: boolean;
  streamingContent: string;
  onSendMessage: (content: string) => void;
  onStopStreaming: () => void;
}

export function ChatPanel({
  messages,
  isStreaming,
  streamingContent,
  onSendMessage,
  onStopStreaming,
}: ChatPanelProps) {
  const [input, setInput] = useState("");
  const [queuedMessage, setQueuedMessage] = useState<string | null>(null);
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLTextAreaElement>(null);

  // Auto-scroll to bottom
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages, streamingContent]);

  // Focus input on mount
  useEffect(() => {
    inputRef.current?.focus();
  }, []);

  // Auto-send queued message when streaming finishes
  useEffect(() => {
    if (!isStreaming && queuedMessage) {
      onSendMessage(queuedMessage);
      setQueuedMessage(null);
    }
  }, [isStreaming, queuedMessage, onSendMessage]);

  const handleSubmit = useCallback(() => {
    if (!input.trim()) return;

    if (isStreaming) {
      // Queue the message for when streaming finishes
      setQueuedMessage(input.trim());
      setInput("");
    } else {
      onSendMessage(input.trim());
      setInput("");
    }
  }, [input, isStreaming, onSendMessage]);

  const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      handleSubmit();
    }
  };

  // Show streaming message as temporary assistant message
  const displayMessages = [...messages];
  if (isStreaming && streamingContent) {
    displayMessages.push({
      id: "streaming",
      role: "assistant",
      content: streamingContent,
      timestamp: Date.now(),
    });
  }

  return (
    <div className="flex-1 flex flex-col h-full min-w-0">
      {/* Messages */}
      <div className="flex-1 overflow-y-auto">
        {displayMessages.length === 0 ? (
          <div className="h-full flex flex-col items-center justify-center text-muted-foreground">
            <MessageSquare className="w-10 h-10 mb-3 opacity-50" />
            <p className="text-sm">Start a conversation</p>
            <p className="text-xs text-muted-foreground/60 mt-1">
              Type your message below
            </p>
          </div>
        ) : (
          <div className="divide-y divide-white/5">
            {displayMessages.map((msg) => (
              <ChatMessage
                key={msg.id}
                message={msg}
                isStreaming={msg.id === "streaming"}
              />
            ))}
          </div>
        )}
        <div ref={messagesEndRef} />
      </div>

      {/* Input Area */}
      <div className="border-t border-white/10 p-3">
        {/* Queued message indicator */}
        {queuedMessage && (
          <div className="mb-2 px-3 py-1.5 bg-blue-500/10 border border-blue-500/20 rounded-lg text-xs text-blue-400 flex items-center justify-between">
            <span className="truncate">Queued: {queuedMessage}</span>
            <button
              type="button"
              onClick={() => setQueuedMessage(null)}
              className="ml-2 hover:text-blue-300"
            >
              ×
            </button>
          </div>
        )}
        <div className="flex gap-2">
          <textarea
            ref={inputRef}
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder={isStreaming ? "Type next message (will send when done)..." : "Type your message..."}
            rows={1}
            className="flex-1 bg-white/5 border border-white/10 rounded-lg px-3 py-2 text-sm text-foreground placeholder:text-muted-foreground resize-none focus:outline-none focus:ring-2 focus:ring-emerald-500/50 focus:border-transparent"
          />
          {isStreaming ? (
            <button
              type="button"
              onClick={onStopStreaming}
              className="shrink-0 px-3 py-2 bg-red-500/20 hover:bg-red-500/30 text-red-400 rounded-lg transition-colors"
              title="Stop generating"
            >
              <Square className="w-4 h-4" />
            </button>
          ) : (
            <button
              type="button"
              onClick={handleSubmit}
              disabled={!input.trim()}
              className={cn(
                "shrink-0 px-3 py-2 bg-emerald-500/20 hover:bg-emerald-500/30 text-emerald-400 rounded-lg transition-colors",
                !input.trim() && "opacity-50 cursor-not-allowed"
              )}
              title="Send message"
            >
              <Send className="w-4 h-4" />
            </button>
          )}
        </div>
      </div>
    </div>
  );
}
