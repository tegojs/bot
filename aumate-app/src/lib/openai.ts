import { streamChatCompletion, type StreamingResult } from "./streaming";

export interface PolishOptions {
  apiUrl: string;
  apiKey: string;
  model: string;
  systemPrompt: string;
  userInput: string;
  onChunk?: (chunk: string) => void;
  signal?: AbortSignal;
}

export type PolishResult = StreamingResult;

export async function polishExpression(
  options: PolishOptions,
): Promise<PolishResult> {
  const { apiUrl, apiKey, model, systemPrompt, userInput, onChunk, signal } =
    options;

  if (!userInput.trim()) {
    return { content: "", error: "Please enter some text to polish." };
  }

  const result = await streamChatCompletion({
    apiUrl,
    apiKey,
    model,
    messages: [
      { role: "system", content: systemPrompt },
      { role: "user", content: userInput },
    ],
    onChunk,
    signal,
  });

  if (result.error === "API key not configured.") {
    return {
      content: "",
      error:
        "API key not configured. Please set your API key in Settings > Expression Polishing.",
    };
  }

  return result;
}
