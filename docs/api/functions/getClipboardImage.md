[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / getClipboardImage

# Function: getClipboardImage()

> **getClipboardImage**(): `Buffer`

Defined in: [botjs/src/index.ts:586](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/index.ts#L586)

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
