[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / scrollMouse

# Function: scrollMouse()

> **scrollMouse**(`x`, `y`): `void`

Defined in: [botjs/src/index.ts:178](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/index.ts#L178)

Scroll the mouse wheel in horizontal and/or vertical directions

## Parameters

### x

`number`

Horizontal scroll amount (positive = right, negative = left)

### y

`number`

Vertical scroll amount (positive = down, negative = up)

## Returns

`void`

## Example

```typescript
import { scrollMouse } from "@tego/botjs";

// Scroll down
scrollMouse(0, 3);

// Scroll up
scrollMouse(0, -3);

// Scroll right
scrollMouse(2, 0);
```
