[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / moveMouseSmooth

# Function: moveMouseSmooth()

> **moveMouseSmooth**(`x`, `y`, `speed?`): `void`

Defined in: [index.ts:80](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/index.ts#L80)

Move the mouse cursor smoothly to the specified coordinates with easing animation

## Parameters

### x

`number`

X coordinate in pixels

### y

`number`

Y coordinate in pixels

### speed?

`number`

Optional movement speed multiplier (default: 3.0, higher = faster)

## Returns

`void`

## Example

```typescript
import { moveMouseSmooth } from "@tego/botjs";

// Smooth movement with default speed
moveMouseSmooth(500, 500);

// Faster smooth movement
moveMouseSmooth(500, 500, 5.0);
```
