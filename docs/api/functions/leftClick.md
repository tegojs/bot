[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / leftClick

# Function: leftClick()

> **leftClick**(`x?`, `y?`): `void`

Defined in: [index.ts:792](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/index.ts#L792)

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
