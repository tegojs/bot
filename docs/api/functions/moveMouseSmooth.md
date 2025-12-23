[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / moveMouseSmooth

# Function: moveMouseSmooth()

> **moveMouseSmooth**(`x`, `y`, `speed?`): `void`

Defined in: [botjs/src/index.ts:86](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/index.ts#L86)

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
