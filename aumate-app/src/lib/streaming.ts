// Shared streaming utilities for OpenAI-compatible APIs

export interface StreamingResult {
  content: string;
  error?: string;
}

export interface StreamingOptions {
  apiUrl: string;
  apiKey: string;
  model: string;
  messages: Array<{ role: string; content: string }>;
  onChunk?: (chunk: string) => void;
  signal?: AbortSignal;
}

/**
 * Parse SSE streaming response from OpenAI-compatible APIs
 */
async function parseSSEStream(
  reader: ReadableStreamDefaultReader<Uint8Array>,
  onChunk?: (chunk: string) => void,
): Promise<string> {
  const decoder = new TextDecoder();
  let fullContent = "";

  while (true) {
    const { done, value } = await reader.read();
    if (done) break;

    const chunk = decoder.decode(value, { stream: true });
    const lines = chunk.split("\n");

    for (const line of lines) {
      if (line.startsWith("data: ")) {
        const data = line.slice(6);
        if (data === "[DONE]") continue;

        try {
          const parsed = JSON.parse(data);
          const content = parsed.choices?.[0]?.delta?.content;
          if (content) {
            fullContent += content;
            onChunk?.(content);
          }
        } catch {
          // Skip invalid JSON lines
        }
      }
    }
  }

  return fullContent;
}

/**
 * Parse API error response
 */
async function parseApiError(response: Response): Promise<string> {
  const errorText = await response.text();
  let errorMessage = `API error: ${response.status}`;
  try {
    const errorJson = JSON.parse(errorText);
    errorMessage = errorJson.error?.message || errorMessage;
  } catch {
    // Keep the default error message
  }
  return errorMessage;
}

/**
 * Send a streaming request to an OpenAI-compatible API
 */
export async function streamChatCompletion(
  options: StreamingOptions,
): Promise<StreamingResult> {
  const { apiUrl, apiKey, model, messages, onChunk, signal } = options;

  if (!apiKey) {
    return {
      content: "",
      error: "API key not configured.",
    };
  }

  if (messages.length === 0) {
    return { content: "", error: "No messages to send." };
  }

  try {
    const response = await fetch(`${apiUrl}/chat/completions`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${apiKey}`,
      },
      body: JSON.stringify({
        model,
        stream: true,
        messages,
      }),
      signal,
    });

    if (!response.ok) {
      const errorMessage = await parseApiError(response);
      return { content: "", error: errorMessage };
    }

    if (!response.body) {
      return { content: "", error: "No response body received" };
    }

    const reader = response.body.getReader();
    const content = await parseSSEStream(reader, onChunk);

    return { content };
  } catch (error) {
    if (error instanceof Error) {
      if (error.name === "AbortError") {
        return { content: "", error: "Request cancelled" };
      }
      return { content: "", error: error.message };
    }
    return { content: "", error: "An unknown error occurred" };
  }
}
