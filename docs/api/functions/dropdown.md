[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / dropdown

# Function: dropdown()

> **dropdown**(`options`): `Widget`

Defined in: [gui.ts:487](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/gui.ts#L487)

Create a dropdown select widget

## Parameters

### options

`string`[]

Array of option strings to display

## Returns

`Widget`

A Widget instance

## Example

```typescript
import { dropdown } from "@tego/botjs";

// Simple dropdown
const colorPicker = dropdown(["Red", "Green", "Blue"])
  .withId("color")
  .withPlaceholder("Select a color");

// Pre-selected dropdown
const sizePicker = dropdown(["Small", "Medium", "Large"])
  .withId("size")
  .withSelected(1); // Medium selected
```
