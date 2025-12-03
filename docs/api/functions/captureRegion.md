[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / captureRegion

# Function: captureRegion()

> **captureRegion**(`x`, `y`, `width`, `height`): `Promise`\<[`ScreenshotResult`](../interfaces/ScreenshotResult.md)\>

Defined in: [screenshot.ts:521](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/screenshot.ts#L521)

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
