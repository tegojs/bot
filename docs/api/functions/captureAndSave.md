[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / captureAndSave

# Function: captureAndSave()

> **captureAndSave**(`path`, `region?`, `options?`): `Promise`\<`void`\>

Defined in: [botjs/src/screenshot.ts:542](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/screenshot.ts#L542)

Capture and save screenshot in one call

## Parameters

### path

`string`

Output file path

### region?

[`ScreenRegion`](../interfaces/ScreenRegion.md)

Optional region to capture

### options?

[`SaveImageOptions`](../interfaces/SaveImageOptions.md)

Save options

## Returns

`Promise`\<`void`\>

## Example

```typescript
// Full screen
await captureAndSave('screenshot.png');

// Region
await captureAndSave('region.jpg', { x: 0, y: 0, width: 800, height: 600 }, {
  quality: 90
});
```
