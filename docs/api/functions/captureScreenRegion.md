[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / captureScreenRegion

# Function: captureScreenRegion()

> **captureScreenRegion**(`x`, `y`, `width`, `height`): `Promise`\<[`ScreenCapture`](../interfaces/ScreenCapture.md)\>

Defined in: [botjs/src/index.ts:435](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/index.ts#L435)

Capture a specific region of the screen as a PNG image

## Parameters

### x

`number`

X coordinate of the top-left corner

### y

`number`

Y coordinate of the top-left corner

### width

`number`

Width of the region in pixels

### height

`number`

Height of the region in pixels

## Returns

`Promise`\<[`ScreenCapture`](../interfaces/ScreenCapture.md)\>

Promise resolving to screen capture with PNG buffer

## Example

```typescript
import { captureScreenRegion } from "@tego/botjs";
import fs from "fs";

// Capture 800x600 region starting at (100, 100)
const region = await captureScreenRegion(100, 100, 800, 600);
fs.writeFileSync('region.png', region.image);
```
