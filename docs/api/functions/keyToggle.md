[**@tego/bot API Documentation v0.0.2**](../README.md)

***

[@tego/bot API Documentation](../globals.md) / keyToggle

# Function: keyToggle()

> **keyToggle**(`key`, `down`, `modifier?`): `void`

Defined in: [index.ts:298](https://github.com/tegojs/bot/blob/89ac2876df45d1ed1dcc5ecd1e596298cff0b31f/packages/botjs/src/index.ts#L298)

Toggle a key state (press down or release up)

## Parameters

### key

`string`

Key to toggle

### down

`string`

"down" to press, "up" to release

### modifier?

`string`[]

Optional array of modifier keys

## Returns

`void`

## Example

```typescript
import { keyToggle, keyTap } from "@tego/botjs";

// Hold Shift
keyToggle('shift', 'down');

// Type 'HELLO' (all caps due to Shift being held)
keyTap('h');
keyTap('e');
keyTap('l');
keyTap('l');
keyTap('o');

// Release Shift
keyToggle('shift', 'up');
```
