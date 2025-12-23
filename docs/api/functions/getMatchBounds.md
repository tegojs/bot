[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / getMatchBounds

# Function: getMatchBounds()

> **getMatchBounds**(`match`): `object`

Defined in: [botjs/src/image-match.ts:406](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/image-match.ts#L406)

Get the bounding rectangle of a match result

## Parameters

### match

[`MatchResult`](../interfaces/MatchResult.md)

MatchResult to get bounds for

## Returns

`object`

Object with left, top, right, bottom coordinates

### left

> **left**: `number`

### top

> **top**: `number`

### right

> **right**: `number`

### bottom

> **bottom**: `number`

## Example

```typescript
import { imageResource, findOnScreen, getMatchBounds } from "@tego/botjs";

const match = await findOnScreen(template);
if (match) {
  const bounds = getMatchBounds(match);
  console.log(`Match bounds: (${bounds.left}, ${bounds.top}) to (${bounds.right}, ${bounds.bottom})`);
}
```
