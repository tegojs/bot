[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / captureScreen

# Function: captureScreen()

> **captureScreen**(): `Promise`\<[`ScreenCapture`](../interfaces/ScreenCapture.md)\>

Defined in: [botjs/src/index.ts:412](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/index.ts#L412)

Capture the entire screen as a PNG image

## Returns

`Promise`\<[`ScreenCapture`](../interfaces/ScreenCapture.md)\>

Promise resolving to screen capture with PNG buffer

## Example

```typescript
import { captureScreen } from "@tego/botjs";
import fs from "fs";

const screenshot = await captureScreen();
fs.writeFileSync('screenshot.png', screenshot.image);
console.log(`Captured ${screenshot.width}x${screenshot.height} screenshot`);
```
