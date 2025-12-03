[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / findWindowsByTitle

# Function: findWindowsByTitle()

> **findWindowsByTitle**(`title`): `WindowInfoResult`[]

Defined in: [index.ts:671](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/index.ts#L671)

Find windows by title using case-insensitive partial matching

**Note:** Currently searches only the active window due to API limitations of the underlying library.
Future versions may support searching all windows.

## Parameters

### title

`string`

Title text to search for (case-insensitive partial match)

## Returns

`WindowInfoResult`[]

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
