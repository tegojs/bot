[**@tego/bot API Documentation v0.0.2**](../README.md)

***

[@tego/bot API Documentation](../globals.md) / moveMouseSmooth

# Function: moveMouseSmooth()

> **moveMouseSmooth**(`x`, `y`, `speed?`): `void`

Defined in: [index.ts:108](https://github.com/tegojs/bot/blob/89ac2876df45d1ed1dcc5ecd1e596298cff0b31f/packages/botjs/src/index.ts#L108)

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
