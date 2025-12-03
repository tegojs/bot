[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / quickScreenshot

# Function: quickScreenshot()

> **quickScreenshot**(): `Promise`\<[`ScreenshotResult`](../interfaces/ScreenshotResult.md)\>

Defined in: [screenshot.ts:346](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/screenshot.ts#L346)

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
