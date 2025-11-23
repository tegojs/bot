[**@tego/bot API Documentation v0.0.2**](../README.md)

***

[@tego/bot API Documentation](../globals.md) / rightClick

# Function: rightClick()

> **rightClick**(`x?`, `y?`): `void`

Defined in: [index.ts:772](https://github.com/tegojs/bot/blob/89ac2876df45d1ed1dcc5ecd1e596298cff0b31f/packages/botjs/src/index.ts#L772)

Perform a right-click at the current mouse position or at specified coordinates

## Parameters

### x?

`number`

Optional X coordinate to move to before right-clicking

### y?

`number`

Optional Y coordinate to move to before right-clicking

## Returns

`void`

## Example

```typescript
import { rightClick } from "@tego/botjs";

// Right-click at current position
rightClick();

// Move to (300, 400) and right-click
rightClick(300, 400);
```
