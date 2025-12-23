[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / mouseToggle

# Function: mouseToggle()

> **mouseToggle**(`down`, `button?`): `void`

Defined in: [botjs/src/index.ts:133](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/index.ts#L133)

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
