[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / quickScreenshot

# Function: quickScreenshot()

> **quickScreenshot**(): `Promise`\<[`ScreenshotResult`](../interfaces/ScreenshotResult.md)\>

Defined in: [botjs/src/screenshot.ts:342](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/screenshot.ts#L342)

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
