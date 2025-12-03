[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / getClipboard

# Function: getClipboard()

> **getClipboard**(): `string`

Defined in: [index.ts:530](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/index.ts#L530)

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
