import { create } from "zustand";
import { invoke } from "@tauri-apps/api/core";
import type { Conversation, DialogueMessage } from "@/types/dialogue";

interface DialogueStore {
  conversations: Conversation[];
  activeConversationId: string | null;
  isLoading: boolean;

  // Actions
  createConversation: () => string;
  setActiveConversation: (id: string | null) => void;
  addMessage: (conversationId: string, message: Omit<DialogueMessage, "id">) => void;
  updateLastMessage: (conversationId: string, content: string) => void;
  deleteConversation: (id: string) => void;
  clearAllConversations: () => void;

  // Persistence
  loadConversations: () => Promise<void>;
  saveConversations: () => Promise<void>;

  // Helpers
  getActiveConversation: () => Conversation | null;
}

const STORAGE_KEY = "dialogue_conversations";

// Generate unique ID
function generateId(): string {
  return `${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
}

// Generate conversation title from first message
function generateTitle(content: string): string {
  const maxLength = 30;
  const trimmed = content.trim().replace(/\n/g, " ");
  if (trimmed.length <= maxLength) return trimmed;
  return `${trimmed.substring(0, maxLength)}...`;
}

export const useDialogueStore = create<DialogueStore>((set, get) => ({
  conversations: [],
  activeConversationId: null,
  isLoading: true,

  createConversation: () => {
    const id = generateId();
    const now = Date.now();
    const newConversation: Conversation = {
      id,
      title: "New Chat",
      messages: [],
      createdAt: now,
      updatedAt: now,
    };

    set((state) => ({
      conversations: [newConversation, ...state.conversations],
      activeConversationId: id,
    }));

    get().saveConversations();
    return id;
  },

  setActiveConversation: (id) => {
    set({ activeConversationId: id });
  },

  addMessage: (conversationId, message) => {
    const id = generateId();
    const fullMessage: DialogueMessage = { ...message, id };

    set((state) => {
      const conversations = state.conversations.map((conv) => {
        if (conv.id !== conversationId) return conv;

        const updatedMessages = [...conv.messages, fullMessage];
        // Update title from first user message
        const title =
          conv.messages.length === 0 && message.role === "user"
            ? generateTitle(message.content)
            : conv.title;

        return {
          ...conv,
          messages: updatedMessages,
          title,
          updatedAt: Date.now(),
        };
      });

      // Move updated conversation to top
      const updatedConv = conversations.find((c) => c.id === conversationId);
      const otherConvs = conversations.filter((c) => c.id !== conversationId);

      return {
        conversations: updatedConv ? [updatedConv, ...otherConvs] : conversations,
      };
    });

    get().saveConversations();
  },

  updateLastMessage: (conversationId, content) => {
    set((state) => ({
      conversations: state.conversations.map((conv) => {
        if (conv.id !== conversationId) return conv;
        if (conv.messages.length === 0) return conv;

        const messages = [...conv.messages];
        const lastIndex = messages.length - 1;
        messages[lastIndex] = {
          ...messages[lastIndex],
          content,
        };

        return {
          ...conv,
          messages,
          updatedAt: Date.now(),
        };
      }),
    }));
  },

  deleteConversation: (id) => {
    set((state) => {
      const conversations = state.conversations.filter((c) => c.id !== id);
      const activeConversationId =
        state.activeConversationId === id
          ? conversations[0]?.id ?? null
          : state.activeConversationId;

      return { conversations, activeConversationId };
    });

    get().saveConversations();
  },

  clearAllConversations: () => {
    set({
      conversations: [],
      activeConversationId: null,
    });
    get().saveConversations();
  },

  loadConversations: async () => {
    try {
      set({ isLoading: true });
      const stored = await invoke<string>("get_storage_value", { key: STORAGE_KEY });
      if (stored) {
        const conversations = JSON.parse(stored) as Conversation[];
        set({
          conversations,
          activeConversationId: conversations[0]?.id ?? null,
          isLoading: false,
        });
      } else {
        set({ isLoading: false });
      }
    } catch (error) {
      console.error("Failed to load conversations:", error);
      set({ isLoading: false });
    }
  },

  saveConversations: async () => {
    try {
      const { conversations } = get();
      await invoke("set_storage_value", {
        key: STORAGE_KEY,
        value: JSON.stringify(conversations),
      });
    } catch (error) {
      console.error("Failed to save conversations:", error);
    }
  },

  getActiveConversation: () => {
    const { conversations, activeConversationId } = get();
    return conversations.find((c) => c.id === activeConversationId) ?? null;
  },
}));
