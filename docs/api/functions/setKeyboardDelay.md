[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / setKeyboardDelay

# Function: setKeyboardDelay()

> **setKeyboardDelay**(`ms`): `void`

Defined in: [index.ts:358](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/index.ts#L358)

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
