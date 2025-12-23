[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / findAllInRegion

# Function: findAllInRegion()

> **findAllInRegion**(`template`, `x`, `y`, `width`, `height`, `config?`): `Promise`\<[`MatchResult`](../interfaces/MatchResult.md)[]\>

Defined in: [botjs/src/image-match.ts:339](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/image-match.ts#L339)

Find all matches of template image in a specific screen region

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

`Promise`\<[`MatchResult`](../interfaces/MatchResult.md)[]\>

Promise resolving to array of MatchResults (coordinates are absolute screen coordinates)

## Example

```typescript
import { imageResource, findAllInRegion, getActiveWindow } from "@tego/botjs";

const icon = await imageResource("./icon.png");
const win = getActiveWindow();

// Search only within the active window
const matches = await findAllInRegion(
  icon,
  win.x, win.y, win.width, win.height,
  { confidence: 0.75 }
);
```
