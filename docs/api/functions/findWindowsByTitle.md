[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / findWindowsByTitle

# Function: findWindowsByTitle()

> **findWindowsByTitle**(`title`): [`WindowInfo`](../interfaces/WindowInfo.md)[]

Defined in: [botjs/src/index.ts:677](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/index.ts#L677)

Find windows by title using case-insensitive partial matching

**Note:** Currently searches only the active window due to API limitations of the underlying library.
Future versions may support searching all windows.

## Parameters

### title

`string`

Title text to search for (case-insensitive partial match)

## Returns

[`WindowInfo`](../interfaces/WindowInfo.md)[]

Array of matching WindowInfo objects

## Example

```typescript
import { findWindowsByTitle } from "@tego/botjs";

// Find Chrome windows
const chromeWindows = findWindowsByTitle('chrome');
chromeWindows.forEach(win => console.log(win.title));

// Find Visual Studio Code windows
const vscodeWindows = findWindowsByTitle('Visual Studio Code');
```
