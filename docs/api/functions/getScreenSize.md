[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / getScreenSize

# Function: getScreenSize()

> **getScreenSize**(): [`ScreenSize`](../interfaces/ScreenSize.md)

Defined in: [botjs/src/index.ts:497](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/index.ts#L497)

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
