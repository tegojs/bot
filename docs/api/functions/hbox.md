[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / hbox

# Function: hbox()

> **hbox**(`children`): `Widget`

Defined in: [gui.ts:305](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/gui.ts#L305)

Create a horizontal box layout that arranges children in a row

## Parameters

### children

`Widget`[]

Array of child widgets

## Returns

`Widget`

A Widget instance

## Example

```typescript
import { hbox, button, spacer } from "@tego/botjs";

// Button row
const buttonRow = hbox([
  button("Cancel"),
  spacer(10),
  button("OK"),
]).withSpacing(8);
```
