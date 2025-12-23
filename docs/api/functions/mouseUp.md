[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / mouseUp

# Function: mouseUp()

> **mouseUp**(`button`): `void`

Defined in: [botjs/src/index.ts:847](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/index.ts#L847)

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
