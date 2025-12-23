[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / setKeyboardDelay

# Function: setKeyboardDelay()

> **setKeyboardDelay**(`ms`): `void`

Defined in: [botjs/src/index.ts:364](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/index.ts#L364)

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
