[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / moveMouse

# Function: moveMouse()

> **moveMouse**(`x`, `y`): `void`

Defined in: [botjs/src/index.ts:64](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/index.ts#L64)

Move the mouse cursor to the specified coordinates instantly

## Parameters

### x

`number`

X coordinate in pixels

### y

`number`

Y coordinate in pixels

## Returns

`void`

## Example

```typescript
import { moveMouse } from "@tego/botjs";

// Move to absolute position
moveMouse(100, 200);
```
