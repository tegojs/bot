[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / slider

# Function: slider()

> **slider**(`value`, `min`, `max`): `Widget`

Defined in: [gui.ts:216](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/gui.ts#L216)

Create a slider widget for numeric input within a range

## Parameters

### value

`number`

Initial slider value

### min

`number`

Minimum value

### max

`number`

Maximum value

## Returns

`Widget`

A Widget instance

## Example

```typescript
import { slider } from "@tego/botjs";

// Volume slider (0-100)
const volumeSlider = slider(50, 0, 100)
  .withId("volume")
  .withStep(5);

// Opacity slider (0-1)
const opacitySlider = slider(1.0, 0, 1)
  .withId("opacity")
  .withStep(0.1);
```
