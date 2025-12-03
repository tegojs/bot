[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / image

# Function: image()

> **image**(`data`, `width`, `height`): `Widget`

Defined in: [gui.ts:458](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/gui.ts#L458)

Create an image widget from RGBA pixel data

## Parameters

### data

`Buffer`

Buffer containing RGBA pixel data (4 bytes per pixel)

### width

`number`

Image width in pixels

### height

`number`

Image height in pixels

## Returns

`Widget`

A Widget instance

## Example

```typescript
import { image } from "@tego/botjs";

// Create a 100x100 red image
const size = 100 * 100 * 4;
const redPixels = Buffer.alloc(size);
for (let i = 0; i < size; i += 4) {
  redPixels[i] = 255;     // R
  redPixels[i + 1] = 0;   // G
  redPixels[i + 2] = 0;   // B
  redPixels[i + 3] = 255; // A
}
const redImage = image(redPixels, 100, 100);
```
