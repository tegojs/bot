import { streamChatCompletion, type StreamingResult } from "./streaming";

export interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}

export interface DialogueOptions {
  apiUrl: string;
  apiKey: string;
  model: string;
  systemPrompt: string;
  messages: ChatMessage[];
  onChunk?: (chunk: string) => void;
  signal?: AbortSignal;
}

export type DialogueResult = StreamingResult;

export async function sendDialogueMessage(
  options: DialogueOptions,
): Promise<DialogueResult> {
  const { apiUrl, apiKey, model, systemPrompt, messages, onChunk, signal } =
    options;

  // Build messages array with system prompt
  const chatMessages = [
    { role: "system", content: systemPrompt },
    ...messages,
  ];

  const result = await streamChatCompletion({
    apiUrl,
    apiKey,
    model,
    messages: chatMessages,
    onChunk,
    signal,
  });

  if (result.error === "API key not configured.") {
    return {
      content: "",
      error:
        "API key not configured. Please set your API key in Settings > AI Dialogue.",
    };
  }

  return result;
}
