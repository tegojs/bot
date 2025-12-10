export interface DialogueMessage {
  id: string;
  role: "user" | "assistant";
  content: string;
  timestamp: number;
}

export interface Conversation {
  id: string;
  title: string;
  messages: DialogueMessage[];
  createdAt: number;
  updatedAt: number;
}
