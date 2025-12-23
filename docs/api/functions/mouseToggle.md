[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / mouseToggle

# Function: mouseToggle()

> **mouseToggle**(`down`, `button?`): `void`

Defined in: [botjs/src/index.ts:133](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/index.ts#L133)

Toggle mouse button state (press down or release up)

## Parameters

### down

`string`

"down" to press the button, "up" to release it

### button?

`string`

Mouse button: "left", "right", or "middle" (default: "left")

## Returns

`void`

## Example

```typescript
import { mouseToggle } from "@tego/botjs";

// Press and hold left button
mouseToggle('down', 'left');

// Perform some actions while button is held...

// Release left button
mouseToggle('up', 'left');
```
