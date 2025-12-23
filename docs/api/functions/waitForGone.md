[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / waitForGone

# Function: waitForGone()

> **waitForGone**(`template`, `timeout`, `interval`, `config?`): `Promise`\<`boolean`\>

Defined in: [botjs/src/image-match.ts:492](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/image-match.ts#L492)

Wait for a template image to disappear from screen

## Parameters

### template

[`ImageResource`](../interfaces/ImageResource.md)

ImageResource to wait for disappearance

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

`Promise`\<`boolean`\>

Promise resolving to true if disappeared, false if timeout

## Example

```typescript
import { imageResource, waitForGone, mouseClick } from "@tego/botjs";

const loadingSpinner = await imageResource("./loading.png");

// Wait up to 30 seconds for loading spinner to disappear
const isGone = await waitForGone(loadingSpinner, 30000, 1000);

if (isGone) {
  console.log("Loading complete!");
} else {
  console.log("Loading took too long");
}
```
