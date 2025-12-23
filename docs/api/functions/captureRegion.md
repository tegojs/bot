[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / captureRegion

# Function: captureRegion()

> **captureRegion**(`x`, `y`, `width`, `height`): `Promise`\<[`ScreenshotResult`](../interfaces/ScreenshotResult.md)\>

Defined in: [botjs/src/screenshot.ts:515](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/screenshot.ts#L515)

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
