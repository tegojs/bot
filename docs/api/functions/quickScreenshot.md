[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / quickScreenshot

# Function: quickScreenshot()

> **quickScreenshot**(): `Promise`\<[`ScreenshotResult`](../interfaces/ScreenshotResult.md)\>

Defined in: [botjs/src/screenshot.ts:342](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/screenshot.ts#L342)

Quick screenshot of entire screen

## Returns

`Promise`\<[`ScreenshotResult`](../interfaces/ScreenshotResult.md)\>

Screenshot result

## Example

```typescript
import { quickScreenshot, saveScreenshotToFile } from "@tego/botjs";

const screenshot = await quickScreenshot();
await saveScreenshotToFile(screenshot, 'screenshot.png');
```
