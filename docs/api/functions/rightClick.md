[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / rightClick

# Function: rightClick()

> **rightClick**(`x?`, `y?`): `void`

Defined in: [botjs/src/index.ts:752](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/index.ts#L752)

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
