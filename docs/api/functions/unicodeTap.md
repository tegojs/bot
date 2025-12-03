[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / unicodeTap

# Function: unicodeTap()

> **unicodeTap**(`codePoint`): `void`

Defined in: [index.ts:337](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/index.ts#L337)

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
