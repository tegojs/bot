[**@tego/bot API Documentation v0.0.2**](../README.md)

***

[@tego/bot API Documentation](../globals.md) / mouseUp

# Function: mouseUp()

> **mouseUp**(`button`): `void`

Defined in: [index.ts:867](https://github.com/tegojs/bot/blob/89ac2876df45d1ed1dcc5ecd1e596298cff0b31f/packages/botjs/src/index.ts#L867)

Release a held mouse button

## Parameters

### button

Mouse button to release: "left", "right", or "middle" (default: "left")

`"right"` | `"middle"` | `"left"`

## Returns

`void`

## Example

```typescript
import { mouseDown, mouseUp } from "@tego/botjs";

mouseDown("left");
// ... perform actions while button is held ...
mouseUp("left");

// Release right button
mouseUp("right");
```
