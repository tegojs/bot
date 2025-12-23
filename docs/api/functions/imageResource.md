[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / imageResource

# Function: imageResource()

> **imageResource**(`path`): `Promise`\<[`ImageResource`](../interfaces/ImageResource.md)\>

Defined in: [botjs/src/image-match.ts:119](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/image-match.ts#L119)

Load an image from file for use as a template

## Parameters

### path

`string`

Path to the image file (PNG, JPG, BMP, etc.)

## Returns

`Promise`\<[`ImageResource`](../interfaces/ImageResource.md)\>

Promise resolving to ImageResource

## Example

```typescript
import { imageResource, findOnScreen } from "@tego/botjs";

const button = await imageResource("./assets/button.png");
const match = await findOnScreen(button);
```
