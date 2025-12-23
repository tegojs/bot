[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / captureAndCopy

# Function: captureAndCopy()

> **captureAndCopy**(`region?`): `Promise`\<`void`\>

Defined in: [botjs/src/screenshot.ts:567](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/screenshot.ts#L567)

Capture and copy to clipboard in one call

## Parameters

### region?

[`ScreenRegion`](../interfaces/ScreenRegion.md)

Optional region to capture

## Returns

`Promise`\<`void`\>

## Example

```typescript
// Full screen
await captureAndCopy();

// Region
await captureAndCopy({ x: 0, y: 0, width: 800, height: 600 });
```
