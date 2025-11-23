[**@tego/bot API Documentation v0.0.2**](../README.md)

***

[@tego/bot API Documentation](../globals.md) / getPixelColor

# Function: getPixelColor()

> **getPixelColor**(`x`, `y`): `Promise`\<`string`\>

Defined in: [index.ts:485](https://github.com/tegojs/bot/blob/89ac2876df45d1ed1dcc5ecd1e596298cff0b31f/packages/botjs/src/index.ts#L485)

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
