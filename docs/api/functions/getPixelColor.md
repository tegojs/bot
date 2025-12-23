[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / getPixelColor

# Function: getPixelColor()

> **getPixelColor**(`x`, `y`): `Promise`\<[`ColorInfo`](../interfaces/ColorInfo.md)\>

Defined in: [botjs/src/screenshot.ts:491](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/screenshot.ts#L491)

Get pixel color at specific coordinates (shorthand)

## Parameters

### x

`number`

X coordinate

### y

`number`

Y coordinate

## Returns

`Promise`\<[`ColorInfo`](../interfaces/ColorInfo.md)\>

Color information

## Example

```typescript
const color = await getPixelColor(100, 200);
console.log(`Color at (100, 200): ${color.hex}`);
```
