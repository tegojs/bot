[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / captureAndSave

# Function: captureAndSave()

> **captureAndSave**(`path`, `region?`, `options?`): `Promise`\<`void`\>

Defined in: [screenshot.ts:548](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/screenshot.ts#L548)

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
