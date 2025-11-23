[**@tego/bot API Documentation v0.0.2**](../README.md)

***

[@tego/bot API Documentation](../globals.md) / setKeyboardDelay

# Function: setKeyboardDelay()

> **setKeyboardDelay**(`ms`): `void`

Defined in: [index.ts:386](https://github.com/tegojs/bot/blob/89ac2876df45d1ed1dcc5ecd1e596298cff0b31f/packages/botjs/src/index.ts#L386)

Set the delay between keyboard operations in milliseconds

## Parameters

### ms

`number`

Delay in milliseconds

## Returns

`void`

## Example

```typescript
import { setKeyboardDelay, keyTap } from "@tego/botjs";

// Set 10ms delay between key presses
setKeyboardDelay(10);

// These will have 10ms delay between them
keyTap('h');
keyTap('i');
```
