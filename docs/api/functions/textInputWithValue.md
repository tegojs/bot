[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / textInputWithValue

# Function: textInputWithValue()

> **textInputWithValue**(`value`): `Widget`

Defined in: [gui.ts:165](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/gui.ts#L165)

Create a text input widget with an initial value

## Parameters

### value

`string`

The initial text value

## Returns

`Widget`

A Widget instance

## Example

```typescript
import { textInputWithValue } from "@tego/botjs";

const emailInput = textInputWithValue("user@example.com")
  .withId("email");
```
