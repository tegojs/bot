[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / getClipboard

# Function: getClipboard()

> **getClipboard**(): `string`

Defined in: [botjs/src/index.ts:536](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/index.ts#L536)

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
