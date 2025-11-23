[**@tego/bot API Documentation v0.0.2**](../README.md)

***

[@tego/bot API Documentation](../globals.md) / getMousePos

# Function: getMousePos()

> **getMousePos**(): `MousePosition`

Defined in: [index.ts:217](https://github.com/tegojs/bot/blob/89ac2876df45d1ed1dcc5ecd1e596298cff0b31f/packages/botjs/src/index.ts#L217)

Get the current mouse cursor position

## Returns

`MousePosition`

Object containing x and y coordinates

## Example

```typescript
import { getMousePos } from "@tego/botjs";

const pos = getMousePos();
console.log(`Mouse is at: ${pos.x}, ${pos.y}`);
```
