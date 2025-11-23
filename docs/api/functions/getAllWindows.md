[**@tego/bot API Documentation v0.0.2**](../README.md)

***

[@tego/bot API Documentation](../globals.md) / getAllWindows

# Function: getAllWindows()

> **getAllWindows**(): `WindowInfo`[]

Defined in: [index.ts:674](https://github.com/tegojs/bot/blob/89ac2876df45d1ed1dcc5ecd1e596298cff0b31f/packages/botjs/src/index.ts#L674)

Get a list of all visible windows

**Note:** Currently returns only the active window due to API limitations of the underlying library.
Future versions may support enumerating all windows.

## Returns

`WindowInfo`[]

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
