[**@tego/bot API Documentation v0.0.2**](../README.md)

***

[@tego/bot API Documentation](../globals.md) / doubleClick

# Function: doubleClick()

> **doubleClick**(`x?`, `y?`): `void`

Defined in: [index.ts:748](https://github.com/tegojs/bot/blob/89ac2876df45d1ed1dcc5ecd1e596298cff0b31f/packages/botjs/src/index.ts#L748)

Perform a double-click at the current mouse position or at specified coordinates

## Parameters

### x?

`number`

Optional X coordinate to move to before double-clicking

### y?

`number`

Optional Y coordinate to move to before double-clicking

## Returns

`void`

## Example

```typescript
import { doubleClick } from "@tego/botjs";

// Double-click at current position
doubleClick();

// Move to (100, 200) and double-click
doubleClick(100, 200);
```
