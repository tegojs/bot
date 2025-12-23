[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / getPixelColorHex

# Function: getPixelColorHex()

> **getPixelColorHex**(`x`, `y`): `Promise`\<`string`\>

Defined in: [botjs/src/index.ts:463](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/index.ts#L463)

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
