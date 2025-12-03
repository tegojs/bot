[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / label

# Function: label()

> **label**(`text`): `Widget`

Defined in: [gui.ts:99](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/gui.ts#L99)

Create a text label widget for displaying static text

## Parameters

### text

`string`

The text content to display

## Returns

`Widget`

A Widget instance

## Example

```typescript
import { label } from "@tego/botjs";

// Simple label
const title = label("Welcome to My App");

// Styled label
const styledLabel = label("Important Notice")
  .withStyle({ fontSize: 18, textColor: "#FF0000" });
```
