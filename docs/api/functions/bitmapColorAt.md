[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / bitmapColorAt

# Function: bitmapColorAt()

> **bitmapColorAt**(`bitmap`, `x`, `y`): `string`

Defined in: [index.ts:383](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/index.ts#L383)

Get the color at specific coordinates in a bitmap

## Parameters

### bitmap

`Bitmap`

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
