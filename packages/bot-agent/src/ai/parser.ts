/**
 * Parse and extract code from AI responses
 */

/**
 * Extract TypeScript code from markdown code blocks
 */
export function extractCodeFromResponse(response: string): string {
  // Match ```typescript or ```ts code blocks
  const codeBlockRegex = /```(?:typescript|ts)\n([\s\S]*?)```/;
  const match = response.match(codeBlockRegex);

  if (match?.[1]) {
    return match[1].trim();
  }

  // Fallback: try generic code block
  const genericCodeBlockRegex = /```\n([\s\S]*?)```/;
  const genericMatch = response.match(genericCodeBlockRegex);

  if (genericMatch?.[1]) {
    return genericMatch[1].trim();
  }

  // If no code block found, return the whole response (might be plain code)
  return response.trim();
}

/**
 * Validate that extracted code looks like TypeScript
 */
export function validateExtractedCode(code: string): {
  valid: boolean;
  error?: string;
} {
  if (!code || code.length === 0) {
    return { valid: false, error: "No code extracted from response" };
  }

  // Basic validation: should have import statement
  if (
    !code.includes("from '@tego/botjs'") &&
    !code.includes('from "@tego/botjs"')
  ) {
    return {
      valid: false,
      error: "Code does not import from '@tego/botjs'",
    };
  }

  return { valid: true };
}
