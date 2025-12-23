[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / quickScreenshotRegion

# Function: quickScreenshotRegion()

> **quickScreenshotRegion**(`region`): `Promise`\<[`ScreenshotResult`](../interfaces/ScreenshotResult.md)\>

Defined in: [botjs/src/screenshot.ts:378](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/screenshot.ts#L378)

Quick screenshot of specific region

## Parameters

### region

[`ScreenRegion`](../interfaces/ScreenRegion.md)

Region to capture

## Returns

`Promise`\<[`ScreenshotResult`](../interfaces/ScreenshotResult.md)\>

Screenshot result

## Example

```typescript
import { quickScreenshotRegion } from "@tego/botjs";

const screenshot = await quickScreenshotRegion({
  x: 0,
  y: 0,
  width: 1920,
  height: 1080
});
```
