[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / tabs

# Function: tabs()

> **tabs**(`tabDefs`): `Widget`

Defined in: [gui.ts:586](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/gui.ts#L586)

Create a tabbed container widget

## Parameters

### tabDefs

[`TabDef`](../interfaces/TabDef.md)[]

Array of tab definitions with label and content

## Returns

`Widget`

A Widget instance

## Example

```typescript
import { tabs, label, vbox, button } from "@tego/botjs";

// Simple tabs
const tabWidget = tabs([
  { label: "Home", content: label("Welcome!") },
  { label: "Settings", content: vbox([
    label("Settings Page"),
    button("Save").withId("save"),
  ])},
]).withId("main-tabs");
```
