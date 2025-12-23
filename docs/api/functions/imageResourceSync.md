[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / imageResourceSync

# Function: imageResourceSync()

> **imageResourceSync**(`path`): [`ImageResource`](../interfaces/ImageResource.md)

Defined in: [botjs/src/image-match.ts:141](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/image-match.ts#L141)

Load an image synchronously from file for use as a template

## Parameters

### path

`string`

Path to the image file (PNG, JPG, BMP, etc.)

## Returns

[`ImageResource`](../interfaces/ImageResource.md)

ImageResource

## Example

```typescript
import { imageResourceSync, findOnScreen } from "@tego/botjs";

const icon = imageResourceSync("./assets/icon.png");
const match = await findOnScreen(icon);
```
