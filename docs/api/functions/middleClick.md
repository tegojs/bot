[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / middleClick

# Function: middleClick()

> **middleClick**(`x?`, `y?`): `void`

Defined in: [index.ts:768](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/index.ts#L768)

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
