[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / getActiveWindow

# Function: getActiveWindow()

> **getActiveWindow**(): [`WindowInfo`](../interfaces/WindowInfo.md)

Defined in: [botjs/src/index.ts:629](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/index.ts#L629)

Get information about the currently active (focused) window

## Returns

[`WindowInfo`](../interfaces/WindowInfo.md)

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
