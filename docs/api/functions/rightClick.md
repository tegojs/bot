[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / rightClick

# Function: rightClick()

> **rightClick**(`x?`, `y?`): `void`

Defined in: [botjs/src/index.ts:752](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/index.ts#L752)

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
