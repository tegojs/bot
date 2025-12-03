[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / dragMouse

# Function: dragMouse()

> **dragMouse**(`x`, `y`): `void`

Defined in: [index.ts:148](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/index.ts#L148)

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
