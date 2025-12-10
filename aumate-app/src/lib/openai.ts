export interface PolishResult {
  content: string;
  error?: string;
}

export interface PolishOptions {
  apiUrl: string;
  apiKey: string;
  model: string;
  systemPrompt: string;
  userInput: string;
  onChunk?: (chunk: string) => void;
  signal?: AbortSignal;
}

export async function polishExpression(options: PolishOptions): Promise<PolishResult> {
  const { apiUrl, apiKey, model, systemPrompt, userInput, onChunk, signal } = options;

  if (!apiKey) {
    return { content: "", error: "API key not configured. Please set your API key in Settings > Expression Polishing." };
  }

  if (!userInput.trim()) {
    return { content: "", error: "Please enter some text to polish." };
  }

  try {
    const response = await fetch(`${apiUrl}/chat/completions`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        "Authorization": `Bearer ${apiKey}`,
      },
      body: JSON.stringify({
        model,
        stream: true,
        messages: [
          { role: "system", content: systemPrompt },
          { role: "user", content: userInput },
        ],
      }),
      signal,
    });

    if (!response.ok) {
      const errorText = await response.text();
      let errorMessage = `API error: ${response.status}`;
      try {
        const errorJson = JSON.parse(errorText);
        errorMessage = errorJson.error?.message || errorMessage;
      } catch {
        // Keep the default error message
      }
      return { content: "", error: errorMessage };
    }

    if (!response.body) {
      return { content: "", error: "No response body received" };
    }

    const reader = response.body.getReader();
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

    return { content: fullContent };
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
