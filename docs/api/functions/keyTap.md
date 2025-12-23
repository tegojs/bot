[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / keyTap

# Function: keyTap()

> **keyTap**(`key`, `modifier?`): `void`

Defined in: [botjs/src/index.ts:247](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/index.ts#L247)

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
