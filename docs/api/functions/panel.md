[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / panel

# Function: panel()

> **panel**(`child`): `Widget`

Defined in: [gui.ts:377](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/gui.ts#L377)

Create a panel container with a background

## Parameters

### child

`Widget`

The child widget to wrap in the panel

## Returns

`Widget`

A Widget instance

## Example

```typescript
import { panel, vbox, label } from "@tego/botjs";

const infoPanel = panel(
  vbox([
    label("Info"),
    label("This is a panel"),
  ])
).withStyle({ backgroundColor: "#F0F0F0" });
```
