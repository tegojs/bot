[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / mouseUp

# Function: mouseUp()

> **mouseUp**(`button`): `void`

Defined in: [index.ts:839](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/index.ts#L839)

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
