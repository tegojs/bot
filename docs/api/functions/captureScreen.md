[**@tego/bot API Documentation v0.0.2**](../README.md)

***

[@tego/bot API Documentation](../globals.md) / captureScreen

# Function: captureScreen()

> **captureScreen**(): `Promise`\<`ScreenCapture`\>

Defined in: [index.ts:434](https://github.com/tegojs/bot/blob/89ac2876df45d1ed1dcc5ecd1e596298cff0b31f/packages/botjs/src/index.ts#L434)

Capture the entire screen as a PNG image

## Returns

`Promise`\<`ScreenCapture`\>

Promise resolving to screen capture with PNG buffer

## Example

```typescript
import { captureScreen } from "@tego/botjs";
import fs from "fs";

const screenshot = await captureScreen();
fs.writeFileSync('screenshot.png', screenshot.image);
console.log(`Captured ${screenshot.width}x${screenshot.height} screenshot`);
```
