[**@tego/bot API Documentation v0.0.2**](../README.md)

***

[@tego/bot API Documentation](../globals.md) / middleClick

# Function: middleClick()

> **middleClick**(`x?`, `y?`): `void`

Defined in: [index.ts:796](https://github.com/tegojs/bot/blob/89ac2876df45d1ed1dcc5ecd1e596298cff0b31f/packages/botjs/src/index.ts#L796)

Perform a middle-click at the current mouse position or at specified coordinates

## Parameters

### x?

`number`

Optional X coordinate to move to before middle-clicking

### y?

`number`

Optional Y coordinate to move to before middle-clicking

## Returns

`void`

## Example

```typescript
import { middleClick } from "@tego/botjs";

// Middle-click at current position
middleClick();

// Move to (500, 600) and middle-click
middleClick(500, 600);
```
