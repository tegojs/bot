[**@tego/bot API Documentation v0.0.2**](../README.md)

***

[@tego/bot API Documentation](../globals.md) / getActiveWindow

# Function: getActiveWindow()

> **getActiveWindow**(): `WindowInfo`

Defined in: [index.ts:651](https://github.com/tegojs/bot/blob/89ac2876df45d1ed1dcc5ecd1e596298cff0b31f/packages/botjs/src/index.ts#L651)

Get information about the currently active (focused) window

## Returns

`WindowInfo`

WindowInfo object with title, process, position, and dimensions

## Example

```typescript
import { getActiveWindow } from "@tego/botjs";

const win = getActiveWindow();
console.log(`Active window: ${win.title}`);
console.log(`Process: ${win.processPath} (PID: ${win.processId})`);
console.log(`Position: (${win.x}, ${win.y})`);
console.log(`Size: ${win.width}x${win.height}`);
```
