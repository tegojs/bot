[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / setKeyboardDelay

# Function: setKeyboardDelay()

> **setKeyboardDelay**(`ms`): `void`

Defined in: [botjs/src/index.ts:364](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/index.ts#L364)

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
