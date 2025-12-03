[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / scrollMouse

# Function: scrollMouse()

> **scrollMouse**(`x`, `y`): `void`

Defined in: [index.ts:172](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/index.ts#L172)

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
