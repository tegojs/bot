[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / rightClick

# Function: rightClick()

> **rightClick**(`x?`, `y?`): `void`

Defined in: [index.ts:744](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/index.ts#L744)

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
