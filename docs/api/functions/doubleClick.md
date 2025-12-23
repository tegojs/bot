[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / doubleClick

# Function: doubleClick()

> **doubleClick**(`x?`, `y?`): `void`

Defined in: [botjs/src/index.ts:728](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/index.ts#L728)

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
