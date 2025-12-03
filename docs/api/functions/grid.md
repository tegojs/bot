[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / grid

# Function: grid()

> **grid**(`rows`): `Widget`

Defined in: [gui.ts:351](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/gui.ts#L351)

Create a grid layout that arranges widgets in rows and columns

## Parameters

### rows

`Widget`[][]

2D array where each inner array represents a row of widgets

## Returns

`Widget`

A Widget instance

## Example

```typescript
import { grid, label, textInput, button } from "@tego/botjs";

// Form grid
const formGrid = grid([
  [label("Name:"), textInput().withId("name")],
  [label("Email:"), textInput().withId("email")],
  [label(""), button("Submit").withId("submit")],
]);
```
