[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / getClipboardImage

# Function: getClipboardImage()

> **getClipboardImage**(): `Buffer`

Defined in: [botjs/src/index.ts:586](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/index.ts#L586)

Get image from clipboard as a PNG-encoded buffer

## Returns

`Buffer`

PNG-encoded image buffer

## Example

```typescript
import { getClipboardImage } from "@tego/botjs";
import fs from "fs";

const imageBuffer = getClipboardImage();
fs.writeFileSync('clipboard.png', imageBuffer);
```
