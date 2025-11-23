[**@tego/bot API Documentation v0.0.2**](../README.md)

***

[@tego/bot API Documentation](../globals.md) / dragMouse

# Function: dragMouse()

> **dragMouse**(`x`, `y`): `void`

Defined in: [index.ts:176](https://github.com/tegojs/bot/blob/89ac2876df45d1ed1dcc5ecd1e596298cff0b31f/packages/botjs/src/index.ts#L176)

Drag the mouse from current position to target coordinates

## Parameters

### x

`number`

Target X coordinate in pixels

### y

`number`

Target Y coordinate in pixels

## Returns

`void`

## Example

```typescript
import { moveMouse, dragMouse } from "@tego/botjs";

// Move to start position
moveMouse(100, 100);

// Drag to end position
dragMouse(500, 500);
```
