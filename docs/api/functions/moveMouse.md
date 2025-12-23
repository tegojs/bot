[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / moveMouse

# Function: moveMouse()

> **moveMouse**(`x`, `y`): `void`

Defined in: [botjs/src/index.ts:64](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/index.ts#L64)

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
