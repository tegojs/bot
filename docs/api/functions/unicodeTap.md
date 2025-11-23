[**@tego/bot API Documentation v0.0.2**](../README.md)

***

[@tego/bot API Documentation](../globals.md) / unicodeTap

# Function: unicodeTap()

> **unicodeTap**(`codePoint`): `void`

Defined in: [index.ts:365](https://github.com/tegojs/bot/blob/89ac2876df45d1ed1dcc5ecd1e596298cff0b31f/packages/botjs/src/index.ts#L365)

Tap a Unicode character by its code point

## Parameters

### codePoint

`number`

Unicode code point (e.g., 0x1F600 for 😀)

## Returns

`void`

## Example

```typescript
import { unicodeTap } from "@tego/botjs";

// Type emoji
unicodeTap(0x1F600); // 😀
unicodeTap(0x2764);  // ❤
unicodeTap(0x1F44D); // 👍
```
