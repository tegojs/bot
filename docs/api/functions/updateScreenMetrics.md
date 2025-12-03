[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / updateScreenMetrics

# Function: updateScreenMetrics()

> **updateScreenMetrics**(): `void`

Defined in: [index.ts:509](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/index.ts#L509)

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
