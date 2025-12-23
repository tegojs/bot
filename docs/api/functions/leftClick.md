[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / leftClick

# Function: leftClick()

> **leftClick**(`x?`, `y?`): `void`

Defined in: [botjs/src/index.ts:800](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/index.ts#L800)

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
