[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / waitFor

# Function: waitFor()

> **waitFor**(`template`, `timeout`, `interval`, `config?`): `Promise`\<[`MatchResult`](../interfaces/MatchResult.md) \| `null`\>

Defined in: [botjs/src/image-match.ts:448](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/image-match.ts#L448)

Wait for a template image to appear on screen

## Parameters

### template

[`ImageResource`](../interfaces/ImageResource.md)

ImageResource to wait for

### timeout

`number` = `10000`

Maximum time to wait in milliseconds (default: 10000)

### interval

`number` = `500`

Time between checks in milliseconds (default: 500)

### config?

[`MatchConfig`](../interfaces/MatchConfig.md)

Optional matching configuration

## Returns

`Promise`\<[`MatchResult`](../interfaces/MatchResult.md) \| `null`\>

Promise resolving to MatchResult when found, or null if timeout

## Example

```typescript
import { imageResource, waitFor, getMatchCenter, moveMouse, mouseClick } from "@tego/botjs";

const dialog = await imageResource("./dialog.png");

// Wait up to 5 seconds for dialog to appear
const match = await waitFor(dialog, 5000, 250, { confidence: 0.9 });

if (match) {
  console.log("Dialog appeared!");
  const center = getMatchCenter(match);
  moveMouse(center.x, center.y);
  mouseClick();
} else {
  console.log("Dialog did not appear within timeout");
}
```
