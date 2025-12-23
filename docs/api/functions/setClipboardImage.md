[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / setClipboardImage

# Function: setClipboardImage()

> **setClipboardImage**(`imageBuffer`): `void`

Defined in: [botjs/src/index.ts:605](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/index.ts#L605)

Set image to clipboard from a PNG-encoded buffer

## Parameters

### imageBuffer

`Buffer`

PNG-encoded image buffer

## Returns

`void`

## Example

```typescript
import { setClipboardImage } from "@tego/botjs";
import fs from "fs";

const imageData = fs.readFileSync('image.png');
setClipboardImage(imageData);
console.log('Image copied to clipboard');
```
