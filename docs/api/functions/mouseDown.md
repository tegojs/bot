[**@tego/bot API Documentation v0.0.2**](../README.md)

***

[@tego/bot API Documentation](../globals.md) / mouseDown

# Function: mouseDown()

> **mouseDown**(`button`): `void`

Defined in: [index.ts:846](https://github.com/tegojs/bot/blob/89ac2876df45d1ed1dcc5ecd1e596298cff0b31f/packages/botjs/src/index.ts#L846)

Press and hold a mouse button down

## Parameters

### button

Mouse button to hold: "left", "right", or "middle" (default: "left")

`"right"` | `"middle"` | `"left"`

## Returns

`void`

## Example

```typescript
import { mouseDown, mouseUp, moveMouse } from "@tego/botjs";

// Hold left button
mouseDown("left");

// Perform drag operation
moveMouse(500, 500);

// Release left button
mouseUp("left");
```
