[**@tego/bot API Documentation v0.0.2**](../README.md)

***

[@tego/bot API Documentation](../globals.md) / setClipboardImage

# Function: setClipboardImage()

> **setClipboardImage**(`imageBuffer`): `void`

Defined in: [index.ts:627](https://github.com/tegojs/bot/blob/89ac2876df45d1ed1dcc5ecd1e596298cff0b31f/packages/botjs/src/index.ts#L627)

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
