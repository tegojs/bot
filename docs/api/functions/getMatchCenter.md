[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / getMatchCenter

# Function: getMatchCenter()

> **getMatchCenter**(`match`): `object`

Defined in: [botjs/src/image-match.ts:382](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/image-match.ts#L382)

Get the center point of a match result

## Parameters

### match

[`MatchResult`](../interfaces/MatchResult.md)

MatchResult to calculate center for

## Returns

`object`

Object with x and y coordinates of the center

### x

> **x**: `number`

### y

> **y**: `number`

## Example

```typescript
import { imageResource, findOnScreen, getMatchCenter, moveMouse, mouseClick } from "@tego/botjs";

const button = await imageResource("./button.png");
const match = await findOnScreen(button);

if (match) {
  const center = getMatchCenter(match);
  moveMouse(center.x, center.y);
  mouseClick();
}
```
