[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / mouseClick

# Function: mouseClick()

> **mouseClick**(`button?`, `double?`): `void`

Defined in: [index.ts:104](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/index.ts#L104)

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
