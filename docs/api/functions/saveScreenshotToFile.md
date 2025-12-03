[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / saveScreenshotToFile

# Function: saveScreenshotToFile()

> **saveScreenshotToFile**(`result`, `filePath`, `options?`): `Promise`\<`void`\>

Defined in: [screenshot.ts:451](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/screenshot.ts#L451)

Save screenshot to file

## Parameters

### result

[`ScreenshotResult`](../interfaces/ScreenshotResult.md)

Screenshot result to save

### filePath

`string`

### options?

[`SaveImageOptions`](../interfaces/SaveImageOptions.md)

Save options

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
