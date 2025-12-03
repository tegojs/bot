[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / getClipboardImage

# Function: getClipboardImage()

> **getClipboardImage**(): `Buffer`

Defined in: [index.ts:580](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/index.ts#L580)

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
