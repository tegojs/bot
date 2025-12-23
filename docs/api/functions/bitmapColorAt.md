[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / bitmapColorAt

# Function: bitmapColorAt()

> **bitmapColorAt**(`bitmap`, `x`, `y`): `string`

Defined in: [botjs/src/index.ts:389](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/index.ts#L389)

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
