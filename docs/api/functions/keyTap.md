[**@tego/bot API Documentation v0.0.2**](../README.md)

***

[@tego/bot API Documentation](../globals.md) / keyTap

# Function: keyTap()

> **keyTap**(`key`, `modifier?`): `void`

Defined in: [index.ts:269](https://github.com/tegojs/bot/blob/89ac2876df45d1ed1dcc5ecd1e596298cff0b31f/packages/botjs/src/index.ts#L269)

Tap a key (press and immediately release)

## Parameters

### key

`string`

Key to tap (e.g., 'a', 'enter', 'escape', 'f1')

### modifier?

`string`[]

Optional array of modifier keys: 'control', 'shift', 'alt', 'command'

## Returns

`void`

## Example

```typescript
import { keyTap } from "@tego/botjs";

// Type a single character
keyTap('a');

// Press Enter
keyTap('enter');

// Ctrl+C (copy)
keyTap('c', ['control']);

// Ctrl+Shift+V (paste without formatting)
keyTap('v', ['control', 'shift']);
```
