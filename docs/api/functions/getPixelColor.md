[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / getPixelColor

# Function: getPixelColor()

> **getPixelColor**(`x`, `y`): `Promise`\<[`ColorInfo`](../interfaces/ColorInfo.md)\>

Defined in: [screenshot.ts:497](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/screenshot.ts#L497)

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
