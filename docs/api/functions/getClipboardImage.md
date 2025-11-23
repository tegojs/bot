[**@tego/bot API Documentation v0.0.2**](../README.md)

***

[@tego/bot API Documentation](../globals.md) / getClipboardImage

# Function: getClipboardImage()

> **getClipboardImage**(): `Buffer`

Defined in: [index.ts:608](https://github.com/tegojs/bot/blob/89ac2876df45d1ed1dcc5ecd1e596298cff0b31f/packages/botjs/src/index.ts#L608)

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
