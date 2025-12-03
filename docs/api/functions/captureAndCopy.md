[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / captureAndCopy

# Function: captureAndCopy()

> **captureAndCopy**(`region?`): `Promise`\<`void`\>

Defined in: [screenshot.ts:573](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/screenshot.ts#L573)

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
