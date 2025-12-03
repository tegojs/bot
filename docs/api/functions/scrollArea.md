[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / scrollArea

# Function: scrollArea()

> **scrollArea**(`child`): `Widget`

Defined in: [gui.ts:402](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/gui.ts#L402)

Create a scroll area container for scrollable content

## Parameters

### child

`Widget`

The child widget to make scrollable

## Returns

`Widget`

A Widget instance

## Example

```typescript
import { scrollArea, vbox, label } from "@tego/botjs";

// Scrollable list
const scrollableList = scrollArea(
  vbox([
    label("Item 1"),
    label("Item 2"),
    label("Item 3"),
    // ... many more items
  ])
).withMaxHeight(200);
```
