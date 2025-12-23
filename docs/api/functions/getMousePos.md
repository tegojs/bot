[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / getMousePos

# Function: getMousePos()

> **getMousePos**(): [`MousePosition`](../interfaces/MousePosition.md)

Defined in: [botjs/src/index.ts:195](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/index.ts#L195)

Get the current mouse cursor position

## Returns

[`MousePosition`](../interfaces/MousePosition.md)

Object containing x and y coordinates

## Example

```typescript
import { getMousePos } from "@tego/botjs";

const pos = getMousePos();
console.log(`Mouse is at: ${pos.x}, ${pos.y}`);
```
