[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / middleClick

# Function: middleClick()

> **middleClick**(`x?`, `y?`): `void`

Defined in: [botjs/src/index.ts:776](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/index.ts#L776)

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
