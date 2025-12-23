[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / getMousePos

# Function: getMousePos()

> **getMousePos**(): [`MousePosition`](../interfaces/MousePosition.md)

Defined in: [botjs/src/index.ts:195](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/index.ts#L195)

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
