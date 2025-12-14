import { useCallback, useEffect, useRef, useState } from "react";
import { type ChatMessage, sendDialogueMessage } from "@/lib/dialogue";
import { useDialogueStore } from "@/stores/dialogueStore";
import { useSettingsStore } from "@/stores/settingsStore";
import { ChatPanel } from "./ChatPanel";
import { ConversationList } from "./ConversationList";

export function DialogueMode() {
  const {
    conversations,
    activeConversationId,
    isLoading,
    createConversation,
    setActiveConversation,
    addMessage,
    updateLastMessage,
    deleteConversation,
    loadConversations,
    getActiveConversation,
  } = useDialogueStore();

  const { settings } = useSettingsStore();
  const { ai_dialogue } = settings;

  const [isStreaming, setIsStreaming] = useState(false);
  const [streamingContent, setStreamingContent] = useState("");
  const abortControllerRef = useRef<AbortController | null>(null);

  // Load conversations on mount
  useEffect(() => {
    loadConversations();
  }, [loadConversations]);

  // Get active conversation
  const activeConversation = getActiveConversation();

  // Handle sending a message
  const handleSendMessage = useCallback(
    async (content: string) => {
      // Create new conversation if none exists
      let conversationId = activeConversationId;
      if (!conversationId) {
        conversationId = createConversation();
      }

      // Add user message
      addMessage(conversationId, {
        role: "user",
        content,
        timestamp: Date.now(),
      });

      // Add empty assistant message (will be updated via streaming)
      addMessage(conversationId, {
        role: "assistant",
        content: "",
        timestamp: Date.now(),
      });

      // Prepare messages for API (limit to max_history_messages)
      const conversation = useDialogueStore.getState().getActiveConversation();
      if (!conversation) return;

      const recentMessages = conversation.messages
        .slice(-(ai_dialogue.max_history_messages * 2)) // Keep pairs of user/assistant
        .filter((m) => m.content) // Filter out empty messages
        .map((m) => ({
          role: m.role as "user" | "assistant",
          content: m.content,
        }));

      // Add the new user message if not included
      if (recentMessages[recentMessages.length - 1]?.content !== content) {
        recentMessages.push({ role: "user", content });
      }

      // Start streaming
      setIsStreaming(true);
      setStreamingContent("");
      abortControllerRef.current = new AbortController();

      const result = await sendDialogueMessage({
        apiUrl: ai_dialogue.api_url,
        apiKey: ai_dialogue.api_key,
        model: ai_dialogue.model,
        systemPrompt: ai_dialogue.system_prompt,
        messages: recentMessages as ChatMessage[],
        signal: abortControllerRef.current.signal,
        onChunk: (chunk) => {
          setStreamingContent((prev) => prev + chunk);
        },
      });

      setIsStreaming(false);
      abortControllerRef.current = null;

      // Update the last assistant message with the full content
      if (result.content) {
        updateLastMessage(conversationId, result.content);
        setStreamingContent("");
      } else if (result.error) {
        updateLastMessage(conversationId, `Error: ${result.error}`);
        setStreamingContent("");
      }
    },
    [
      activeConversationId,
      createConversation,
      addMessage,
      updateLastMessage,
      ai_dialogue,
    ],
  );

  // Handle stopping the stream
  const handleStopStreaming = useCallback(() => {
    if (abortControllerRef.current) {
      abortControllerRef.current.abort();
      abortControllerRef.current = null;
    }
    setIsStreaming(false);

    // Keep whatever content we have so far
    if (streamingContent && activeConversationId) {
      updateLastMessage(activeConversationId, streamingContent);
    }
    setStreamingContent("");
  }, [activeConversationId, streamingContent, updateLastMessage]);

  // Handle new conversation
  const handleNewConversation = useCallback(() => {
    createConversation();
  }, [createConversation]);

  if (isLoading) {
    return (
      <div className="flex-1 flex items-center justify-center text-muted-foreground">
        Loading conversations...
      </div>
    );
  }

  return (
    <div className="flex-1 flex h-full overflow-hidden">
      <ConversationList
        conversations={conversations}
        activeConversationId={activeConversationId}
        onSelectConversation={setActiveConversation}
        onNewConversation={handleNewConversation}
        onDeleteConversation={deleteConversation}
      />
      <ChatPanel
        messages={activeConversation?.messages ?? []}
        isStreaming={isStreaming}
        streamingContent={streamingContent}
        onSendMessage={handleSendMessage}
        onStopStreaming={handleStopStreaming}
      />
    </div>
  );
}
