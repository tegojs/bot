[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / findAllOnScreen

# Function: findAllOnScreen()

> **findAllOnScreen**(`template`, `config?`): `Promise`\<[`MatchResult`](../interfaces/MatchResult.md)[]\>

Defined in: [botjs/src/image-match.ts:259](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/image-match.ts#L259)

Find all matches of template image on screen

## Parameters

### template

[`ImageResource`](../interfaces/ImageResource.md)

ImageResource to search for

### config?

[`MatchConfig`](../interfaces/MatchConfig.md)

Optional matching configuration

## Returns

`Promise`\<[`MatchResult`](../interfaces/MatchResult.md)[]\>

Promise resolving to array of MatchResults, sorted by confidence descending

## Example

```typescript
import { imageResource, findAllOnScreen, moveMouse, mouseClick } from "@tego/botjs";

const icon = await imageResource("./checkbox.png");
const matches = await findAllOnScreen(icon, { confidence: 0.7, limit: 10 });

console.log(`Found ${matches.length} checkboxes`);

// Click each checkbox
for (const match of matches) {
  const center = getMatchCenter(match);
  moveMouse(center.x, center.y);
  mouseClick();
  await sleep(100);
}
```
