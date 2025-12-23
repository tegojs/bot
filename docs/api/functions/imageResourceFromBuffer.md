[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / imageResourceFromBuffer

# Function: imageResourceFromBuffer()

> **imageResourceFromBuffer**(`buffer`): [`ImageResource`](../interfaces/ImageResource.md)

Defined in: [botjs/src/image-match.ts:165](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/image-match.ts#L165)

Create an image resource from a Buffer

## Parameters

### buffer

`Buffer`

PNG, JPG, or other image format buffer

## Returns

[`ImageResource`](../interfaces/ImageResource.md)

ImageResource

## Example

```typescript
import { imageResourceFromBuffer, findOnScreen, captureScreenRegion } from "@tego/botjs";

// Capture a region and use it as template
const capture = await captureScreenRegion(100, 100, 50, 50);
const template = imageResourceFromBuffer(capture.image);
const matches = await findAllOnScreen(template);
```
