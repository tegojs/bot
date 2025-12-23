[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / getAllWindows

# Function: getAllWindows()

> **getAllWindows**(): [`WindowInfo`](../interfaces/WindowInfo.md)[]

Defined in: [botjs/src/index.ts:652](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/index.ts#L652)

Get a list of all visible windows

**Note:** Currently returns only the active window due to API limitations of the underlying library.
Future versions may support enumerating all windows.

## Returns

[`WindowInfo`](../interfaces/WindowInfo.md)[]

Array of WindowInfo objects

## Example

```typescript
import { getAllWindows } from "@tego/botjs";

const windows = getAllWindows();
console.log(`Found ${windows.length} windows`);
windows.forEach(win => {
  console.log(`- ${win.title}`);
});
```
