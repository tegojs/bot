[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / textAreaWithValue

# Function: textAreaWithValue()

> **textAreaWithValue**(`value`): `Widget`

Defined in: [gui.ts:554](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/gui.ts#L554)

Create a multi-line text area widget with an initial value

## Parameters

### value

`string`

The initial text value

## Returns

`Widget`

A Widget instance

## Example

```typescript
import { textAreaWithValue } from "@tego/botjs";

const bio = textAreaWithValue("Enter your bio here...")
  .withId("bio")
  .withRows(4);
```
