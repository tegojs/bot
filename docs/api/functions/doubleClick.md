[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / doubleClick

# Function: doubleClick()

> **doubleClick**(`x?`, `y?`): `void`

Defined in: [index.ts:720](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/index.ts#L720)

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
