[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / unicodeTap

# Function: unicodeTap()

> **unicodeTap**(`codePoint`): `void`

Defined in: [botjs/src/index.ts:343](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/index.ts#L343)

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
