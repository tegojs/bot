[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / findWindowsByProcess

# Function: findWindowsByProcess()

> **findWindowsByProcess**(`processName`): [`WindowInfo`](../interfaces/WindowInfo.md)[]

Defined in: [botjs/src/index.ts:701](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/index.ts#L701)

Find windows by process name using case-insensitive partial matching

**Note:** Currently searches only the active window due to API limitations of the underlying library.
Future versions may support searching all windows.

## Parameters

### processName

`string`

Process name to search for (case-insensitive partial match)

## Returns

[`WindowInfo`](../interfaces/WindowInfo.md)[]

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
