[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / mouseDown

# Function: mouseDown()

> **mouseDown**(`button`): `void`

Defined in: [index.ts:818](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/index.ts#L818)

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
