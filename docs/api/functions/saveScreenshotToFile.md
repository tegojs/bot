[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / saveScreenshotToFile

# Function: saveScreenshotToFile()

> **saveScreenshotToFile**(`result`, `filePath`, `_options?`): `Promise`\<`void`\>

Defined in: [botjs/src/screenshot.ts:445](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/screenshot.ts#L445)

Save screenshot to file

## Parameters

### result

[`ScreenshotResult`](../interfaces/ScreenshotResult.md)

Screenshot result to save

### filePath

`string`

File path (extension determines format)

### \_options?

[`SaveImageOptions`](../interfaces/SaveImageOptions.md)

## Returns

`Promise`\<`void`\>

## Example

```typescript
const screenshot = await quickScreenshot();

// Save as PNG (default)
await saveScreenshotToFile(screenshot, 'screenshot.png');

// Save as JPEG with quality
await saveScreenshotToFile(screenshot, 'screenshot.jpg', {
  format: 'jpg',
  quality: 90
});

// Save as WebP
await saveScreenshotToFile(screenshot, 'screenshot.webp', {
  format: 'webp',
  quality: 85
});
```
