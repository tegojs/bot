[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / keyToggle

# Function: keyToggle()

> **keyToggle**(`key`, `down`, `modifier?`): `void`

Defined in: [index.ts:270](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/index.ts#L270)

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
