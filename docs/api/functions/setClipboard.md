[**@tego/bot API Documentation v0.0.2**](../README.md)

***

[@tego/bot API Documentation](../globals.md) / setClipboard

# Function: setClipboard()

> **setClipboard**(`text`): `void`

Defined in: [index.ts:575](https://github.com/tegojs/bot/blob/89ac2876df45d1ed1dcc5ecd1e596298cff0b31f/packages/botjs/src/index.ts#L575)

Set text content to the system clipboard

## Parameters

### text

`string`

Text to copy to clipboard

## Returns

`void`

## Example

```typescript
import { setClipboard } from "@tego/botjs";

setClipboard('Hello from @tego/bot!');
setClipboard('user@example.com');
```
