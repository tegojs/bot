[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / copyScreenshotToClipboard

# Function: copyScreenshotToClipboard()

> **copyScreenshotToClipboard**(`result`): `Promise`\<`void`\>

Defined in: [screenshot.ts:472](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/screenshot.ts#L472)

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
