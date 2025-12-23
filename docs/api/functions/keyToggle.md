[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / keyToggle

# Function: keyToggle()

> **keyToggle**(`key`, `down`, `modifier?`): `void`

Defined in: [botjs/src/index.ts:276](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/index.ts#L276)

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
