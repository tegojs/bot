[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / getAllWindows

# Function: getAllWindows()

> **getAllWindows**(): `WindowInfoResult`[]

Defined in: [index.ts:646](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/index.ts#L646)

Get a list of all visible windows

**Note:** Currently returns only the active window due to API limitations of the underlying library.
Future versions may support enumerating all windows.

## Returns

`WindowInfoResult`[]

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
