[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / findWindowsByProcess

# Function: findWindowsByProcess()

> **findWindowsByProcess**(`processName`): `WindowInfoResult`[]

Defined in: [index.ts:695](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/index.ts#L695)

Find windows by process name using case-insensitive partial matching

**Note:** Currently searches only the active window due to API limitations of the underlying library.
Future versions may support searching all windows.

## Parameters

### processName

`string`

Process name to search for (case-insensitive partial match)

## Returns

`WindowInfoResult`[]

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
