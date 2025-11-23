[**@tego/bot API Documentation v0.0.2**](../README.md)

***

[@tego/bot API Documentation](../globals.md) / findWindowsByProcess

# Function: findWindowsByProcess()

> **findWindowsByProcess**(`processName`): `WindowInfo`[]

Defined in: [index.ts:723](https://github.com/tegojs/bot/blob/89ac2876df45d1ed1dcc5ecd1e596298cff0b31f/packages/botjs/src/index.ts#L723)

Find windows by process name using case-insensitive partial matching

**Note:** Currently searches only the active window due to API limitations of the underlying library.
Future versions may support searching all windows.

## Parameters

### processName

`string`

Process name to search for (case-insensitive partial match)

## Returns

`WindowInfo`[]

Array of matching WindowInfo objects

## Example

```typescript
import { findWindowsByProcess } from "@tego/botjs";

// Find VS Code windows by process
const vscodeWindows = findWindowsByProcess('code');
vscodeWindows.forEach(win => {
  console.log(`${win.title} - ${win.processPath}`);
});
```
