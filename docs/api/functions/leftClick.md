[**@tego/bot API Documentation v0.0.2**](../README.md)

***

[@tego/bot API Documentation](../globals.md) / leftClick

# Function: leftClick()

> **leftClick**(`x?`, `y?`): `void`

Defined in: [index.ts:820](https://github.com/tegojs/bot/blob/89ac2876df45d1ed1dcc5ecd1e596298cff0b31f/packages/botjs/src/index.ts#L820)

Perform a left-click at the current mouse position or at specified coordinates

## Parameters

### x?

`number`

Optional X coordinate to move to before left-clicking

### y?

`number`

Optional Y coordinate to move to before left-clicking

## Returns

`void`

## Example

```typescript
import { leftClick } from "@tego/botjs";

// Left-click at current position
leftClick();

// Move to (150, 250) and left-click
leftClick(150, 250);
```
