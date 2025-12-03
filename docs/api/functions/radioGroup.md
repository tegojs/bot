[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / radioGroup

# Function: radioGroup()

> **radioGroup**(`options`): `Widget`

Defined in: [gui.ts:511](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/gui.ts#L511)

Create a radio button group widget

## Parameters

### options

`string`[]

Array of option strings to display

## Returns

`Widget`

A Widget instance

## Example

```typescript
import { radioGroup } from "@tego/botjs";

// Vertical radio group (default)
const paymentMethod = radioGroup(["Credit Card", "PayPal", "Bank Transfer"])
  .withId("payment");

// Horizontal radio group
const gender = radioGroup(["Male", "Female", "Other"])
  .withId("gender")
  .withHorizontal(true);
```
