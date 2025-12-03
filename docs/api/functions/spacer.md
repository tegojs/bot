[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / spacer

# Function: spacer()

> **spacer**(`size`): `Widget`

Defined in: [gui.ts:279](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/gui.ts#L279)

Create a spacer for adding empty space in layouts

## Parameters

### size

`number`

Size of the spacer in pixels

## Returns

`Widget`

A Widget instance

## Example

```typescript
import { spacer, vbox, button } from "@tego/botjs";

const content = vbox([
  button("Top"),
  spacer(20),
  button("Bottom"),
]);
```
