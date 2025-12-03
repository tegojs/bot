[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / button

# Function: button()

> **button**(`text`): `Widget`

Defined in: [gui.ts:122](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/gui.ts#L122)

Create a button widget that can respond to clicks

## Parameters

### text

`string`

The button label text

## Returns

`Widget`

A Widget instance

## Example

```typescript
import { button } from "@tego/botjs";

// Simple button
const submitBtn = button("Submit");

// Button with ID for event handling
const loginBtn = button("Login")
  .withId("login-button")
  .withStyle({ backgroundColor: "#007AFF" });
```
