[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / unicodeTap

# Function: unicodeTap()

> **unicodeTap**(`codePoint`): `void`

Defined in: [botjs/src/index.ts:343](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/index.ts#L343)

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
