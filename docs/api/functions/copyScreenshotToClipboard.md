[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / copyScreenshotToClipboard

# Function: copyScreenshotToClipboard()

> **copyScreenshotToClipboard**(`result`): `Promise`\<`void`\>

Defined in: [botjs/src/screenshot.ts:466](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/screenshot.ts#L466)

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
