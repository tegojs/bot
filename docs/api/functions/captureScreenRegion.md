[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / captureScreenRegion

# Function: captureScreenRegion()

> **captureScreenRegion**(`x`, `y`, `width`, `height`): `Promise`\<`ScreenCaptureResult`\>

Defined in: [index.ts:429](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/index.ts#L429)

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

`Promise`\<`ScreenCaptureResult`\>

Promise resolving to screen capture with PNG buffer

## Example

```typescript
import { captureScreenRegion } from "@tego/botjs";
import fs from "fs";

// Capture 800x600 region starting at (100, 100)
const region = await captureScreenRegion(100, 100, 800, 600);
fs.writeFileSync('region.png', region.image);
```
