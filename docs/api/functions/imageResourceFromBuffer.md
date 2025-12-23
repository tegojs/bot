[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / imageResourceFromBuffer

# Function: imageResourceFromBuffer()

> **imageResourceFromBuffer**(`buffer`): [`ImageResource`](../interfaces/ImageResource.md)

Defined in: [botjs/src/image-match.ts:165](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/image-match.ts#L165)

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
