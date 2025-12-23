[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / mouseDown

# Function: mouseDown()

> **mouseDown**(`button`): `void`

Defined in: [botjs/src/index.ts:826](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/index.ts#L826)

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
