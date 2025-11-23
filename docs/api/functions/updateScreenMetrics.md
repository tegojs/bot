[**@tego/bot API Documentation v0.0.2**](../README.md)

***

[@tego/bot API Documentation](../globals.md) / updateScreenMetrics

# Function: updateScreenMetrics()

> **updateScreenMetrics**(): `void`

Defined in: [index.ts:537](https://github.com/tegojs/bot/blob/89ac2876df45d1ed1dcc5ecd1e596298cff0b31f/packages/botjs/src/index.ts#L537)

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
