[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / findInRegion

# Function: findInRegion()

> **findInRegion**(`template`, `x`, `y`, `width`, `height`, `config?`): `Promise`\<[`MatchResult`](../interfaces/MatchResult.md) \| `null`\>

Defined in: [botjs/src/image-match.ts:294](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/image-match.ts#L294)

Find first match of template image in a specific screen region

## Parameters

### template

[`ImageResource`](../interfaces/ImageResource.md)

ImageResource to search for

### x

`number`

X coordinate of search region

### y

`number`

Y coordinate of search region

### width

`number`

Width of search region

### height

`number`

Height of search region

### config?

[`MatchConfig`](../interfaces/MatchConfig.md)

Optional matching configuration

## Returns

`Promise`\<[`MatchResult`](../interfaces/MatchResult.md) \| `null`\>

Promise resolving to MatchResult or null (coordinates are absolute screen coordinates)

## Example

```typescript
import { imageResource, findInRegion } from "@tego/botjs";

const button = await imageResource("./button.png");
// Search only in the left half of a 1920x1080 screen
const match = await findInRegion(button, 0, 0, 960, 1080, { confidence: 0.8 });

if (match) {
  console.log(`Found at (${match.x}, ${match.y})`);
}
```
