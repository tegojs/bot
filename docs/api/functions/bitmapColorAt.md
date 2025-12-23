[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / bitmapColorAt

# Function: bitmapColorAt()

> **bitmapColorAt**(`bitmap`, `x`, `y`): `string`

Defined in: [botjs/src/index.ts:389](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/index.ts#L389)

Get the color at specific coordinates in a bitmap

## Parameters

### bitmap

[`Bitmap`](../interfaces/Bitmap.md)

Bitmap object from screen capture

### x

`number`

X coordinate in the bitmap

### y

`number`

Y coordinate in the bitmap

## Returns

`string`

Hex color string (e.g., "#FF0000" for red)

## Example

```typescript
import { captureScreen, bitmapColorAt } from "@tego/botjs";

const screenshot = await captureScreen();
const color = bitmapColorAt(screenshot, 100, 200);
console.log(`Color at (100, 200): ${color}`);
```
