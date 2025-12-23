[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / captureRegion

# Function: captureRegion()

> **captureRegion**(`x`, `y`, `width`, `height`): `Promise`\<[`ScreenshotResult`](../interfaces/ScreenshotResult.md)\>

Defined in: [botjs/src/screenshot.ts:515](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/screenshot.ts#L515)

Capture region with simplified API

## Parameters

### x

`number`

X coordinate

### y

`number`

Y coordinate

### width

`number`

Width in pixels

### height

`number`

Height in pixels

## Returns

`Promise`\<[`ScreenshotResult`](../interfaces/ScreenshotResult.md)\>

Screenshot result

## Example

```typescript
const screenshot = await captureRegion(0, 0, 1920, 1080);
await saveScreenshotToFile(screenshot, 'region.png');
```
