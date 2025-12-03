[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / captureScreen

# Function: captureScreen()

> **captureScreen**(): `Promise`\<`ScreenCaptureResult`\>

Defined in: [index.ts:406](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/index.ts#L406)

Capture the entire screen as a PNG image

## Returns

`Promise`\<`ScreenCaptureResult`\>

Promise resolving to screen capture with PNG buffer

## Example

```typescript
import { captureScreen } from "@tego/botjs";
import fs from "fs";

const screenshot = await captureScreen();
fs.writeFileSync('screenshot.png', screenshot.image);
console.log(`Captured ${screenshot.width}x${screenshot.height} screenshot`);
```
