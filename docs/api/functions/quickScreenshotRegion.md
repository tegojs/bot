[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / quickScreenshotRegion

# Function: quickScreenshotRegion()

> **quickScreenshotRegion**(`region`): `Promise`\<[`ScreenshotResult`](../interfaces/ScreenshotResult.md)\>

Defined in: [screenshot.ts:382](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/screenshot.ts#L382)

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
