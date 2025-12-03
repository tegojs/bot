[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / getPixelColorHex

# Function: getPixelColorHex()

> **getPixelColorHex**(`x`, `y`): `Promise`\<`string`\>

Defined in: [index.ts:457](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/index.ts#L457)

Get the color of a pixel at specific screen coordinates

## Parameters

### x

`number`

X coordinate on screen

### y

`number`

Y coordinate on screen

## Returns

`Promise`\<`string`\>

Promise resolving to hex color string (e.g., "#FF0000")

## Example

```typescript
import { getPixelColor } from "@tego/botjs";

const color = await getPixelColor(100, 200);
console.log(`Pixel color: ${color}`);

if (color === "#FF0000") {
  console.log("Pixel is red!");
}
```
