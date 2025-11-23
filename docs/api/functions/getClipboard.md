[**@tego/bot API Documentation v0.0.2**](../README.md)

***

[@tego/bot API Documentation](../globals.md) / getClipboard

# Function: getClipboard()

> **getClipboard**(): `string`

Defined in: [index.ts:558](https://github.com/tegojs/bot/blob/89ac2876df45d1ed1dcc5ecd1e596298cff0b31f/packages/botjs/src/index.ts#L558)

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
