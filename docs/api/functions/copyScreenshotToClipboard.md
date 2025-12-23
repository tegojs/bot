[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / copyScreenshotToClipboard

# Function: copyScreenshotToClipboard()

> **copyScreenshotToClipboard**(`result`): `Promise`\<`void`\>

Defined in: [botjs/src/screenshot.ts:466](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/screenshot.ts#L466)

Copy screenshot to clipboard

## Parameters

### result

[`ScreenshotResult`](../interfaces/ScreenshotResult.md)

Screenshot result to copy

## Returns

`Promise`\<`void`\>

## Example

```typescript
const screenshot = await quickScreenshot();
await copyScreenshotToClipboard(screenshot);
console.log('Screenshot copied to clipboard!');
```
