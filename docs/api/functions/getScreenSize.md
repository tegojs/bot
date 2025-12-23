[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / getScreenSize

# Function: getScreenSize()

> **getScreenSize**(): [`ScreenSize`](../interfaces/ScreenSize.md)

Defined in: [botjs/src/index.ts:497](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/index.ts#L497)

Get the dimensions of the primary screen

## Returns

[`ScreenSize`](../interfaces/ScreenSize.md)

Object containing width and height in pixels

## Example

```typescript
import { getScreenSize } from "@tego/botjs";

const size = getScreenSize();
console.log(`Screen resolution: ${size.width}x${size.height}`);
```
