[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / getActiveWindow

# Function: getActiveWindow()

> **getActiveWindow**(): `WindowInfoResult`

Defined in: [index.ts:623](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/index.ts#L623)

Get information about the currently active (focused) window

## Returns

`WindowInfoResult`

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
