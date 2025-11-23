[**@tego/bot API Documentation v0.0.2**](../README.md)

***

[@tego/bot API Documentation](../globals.md) / mouseClick

# Function: mouseClick()

> **mouseClick**(`button?`, `double?`): `void`

Defined in: [index.ts:132](https://github.com/tegojs/bot/blob/89ac2876df45d1ed1dcc5ecd1e596298cff0b31f/packages/botjs/src/index.ts#L132)

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
