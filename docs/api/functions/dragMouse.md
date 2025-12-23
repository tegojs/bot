[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / dragMouse

# Function: dragMouse()

> **dragMouse**(`x`, `y`): `void`

Defined in: [botjs/src/index.ts:154](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/index.ts#L154)

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
