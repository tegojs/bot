[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / middleClick

# Function: middleClick()

> **middleClick**(`x?`, `y?`): `void`

Defined in: [botjs/src/index.ts:776](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/index.ts#L776)

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
