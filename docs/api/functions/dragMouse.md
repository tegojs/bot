[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / dragMouse

# Function: dragMouse()

> **dragMouse**(`x`, `y`): `void`

Defined in: [botjs/src/index.ts:154](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/index.ts#L154)

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
