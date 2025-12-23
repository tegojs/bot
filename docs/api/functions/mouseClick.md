[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / mouseClick

# Function: mouseClick()

> **mouseClick**(`button?`, `double?`): `void`

Defined in: [botjs/src/index.ts:110](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/index.ts#L110)

Click the mouse button at the current cursor position

## Parameters

### button?

`string`

Mouse button: "left", "right", or "middle" (default: "left")

### double?

`boolean`

Whether to perform a double click (default: false)

## Returns

`void`

## Example

```typescript
import { mouseClick } from "@tego/botjs";

// Single left click
mouseClick('left');

// Double right click
mouseClick('right', true);

// Single middle click
mouseClick('middle');
```
