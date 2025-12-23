[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / updateScreenMetrics

# Function: updateScreenMetrics()

> **updateScreenMetrics**(): `void`

Defined in: [botjs/src/index.ts:515](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/index.ts#L515)

Update screen metrics (refresh monitor information)
Call this after display configuration changes

## Returns

`void`

## Example

```typescript
import { updateScreenMetrics, getScreenSize } from "@tego/botjs";

// After connecting/disconnecting monitors
updateScreenMetrics();
const newSize = getScreenSize();
console.log(`Updated screen size: ${newSize.width}x${newSize.height}`);
```
