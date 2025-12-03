[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / getMousePos

# Function: getMousePos()

> **getMousePos**(): `MousePositionResult`

Defined in: [index.ts:189](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/index.ts#L189)

Get the current mouse cursor position

## Returns

`MousePositionResult`

Object containing x and y coordinates

## Example

```typescript
import { getMousePos } from "@tego/botjs";

const pos = getMousePos();
console.log(`Mouse is at: ${pos.x}, ${pos.y}`);
```
