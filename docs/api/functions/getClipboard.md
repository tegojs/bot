[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / getClipboard

# Function: getClipboard()

> **getClipboard**(): `string`

Defined in: [botjs/src/index.ts:536](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/index.ts#L536)

Get text content from the system clipboard

## Returns

`string`

Current clipboard text content

## Example

```typescript
import { getClipboard } from "@tego/botjs";

const text = getClipboard();
console.log(`Clipboard contains: ${text}`);
```
