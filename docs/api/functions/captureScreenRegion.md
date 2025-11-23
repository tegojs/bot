[**@tego/bot API Documentation v0.0.2**](../README.md)

***

[@tego/bot API Documentation](../globals.md) / captureScreenRegion

# Function: captureScreenRegion()

> **captureScreenRegion**(`x`, `y`, `width`, `height`): `Promise`\<`ScreenCapture`\>

Defined in: [index.ts:457](https://github.com/tegojs/bot/blob/89ac2876df45d1ed1dcc5ecd1e596298cff0b31f/packages/botjs/src/index.ts#L457)

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

`Promise`\<`ScreenCapture`\>

Promise resolving to screen capture with PNG buffer

## Example

```typescript
import { captureScreenRegion } from "@tego/botjs";
import fs from "fs";

// Capture 800x600 region starting at (100, 100)
const region = await captureScreenRegion(100, 100, 800, 600);
fs.writeFileSync('region.png', region.image);
```
