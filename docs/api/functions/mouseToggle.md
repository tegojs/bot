[**@tego/bot API Documentation v0.0.2**](../README.md)

***

[@tego/bot API Documentation](../globals.md) / mouseToggle

# Function: mouseToggle()

> **mouseToggle**(`down`, `button?`): `void`

Defined in: [index.ts:155](https://github.com/tegojs/bot/blob/89ac2876df45d1ed1dcc5ecd1e596298cff0b31f/packages/botjs/src/index.ts#L155)

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
