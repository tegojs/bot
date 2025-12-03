[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / setClipboardImage

# Function: setClipboardImage()

> **setClipboardImage**(`imageBuffer`): `void`

Defined in: [index.ts:599](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/index.ts#L599)

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
