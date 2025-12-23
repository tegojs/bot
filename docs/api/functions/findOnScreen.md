[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / findOnScreen

# Function: findOnScreen()

> **findOnScreen**(`template`, `config?`): `Promise`\<[`MatchResult`](../interfaces/MatchResult.md) \| `null`\>

Defined in: [botjs/src/image-match.ts:223](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/image-match.ts#L223)

Find first match of template image on screen

## Parameters

### template

[`ImageResource`](../interfaces/ImageResource.md)

ImageResource to search for

### config?

[`MatchConfig`](../interfaces/MatchConfig.md)

Optional matching configuration

## Returns

`Promise`\<[`MatchResult`](../interfaces/MatchResult.md) \| `null`\>

Promise resolving to MatchResult or null if not found

## Example

```typescript
import { imageResource, findOnScreen, moveMouse, mouseClick } from "@tego/botjs";

const button = await imageResource("./button.png");
const match = await findOnScreen(button, { confidence: 0.85 });

if (match) {
  console.log(`Found at (${match.x}, ${match.y}) with ${match.confidence * 100}% confidence`);
  const center = getMatchCenter(match);
  moveMouse(center.x, center.y);
  mouseClick();
} else {
  console.log("Button not found on screen");
}
```
