[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / getPixelColorHex

# Function: getPixelColorHex()

> **getPixelColorHex**(`x`, `y`): `Promise`\<`string`\>

Defined in: [botjs/src/index.ts:463](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/index.ts#L463)

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
