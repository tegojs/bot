[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / mouseClick

# Function: mouseClick()

> **mouseClick**(`button?`, `double?`): `void`

Defined in: [botjs/src/index.ts:110](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/index.ts#L110)

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
