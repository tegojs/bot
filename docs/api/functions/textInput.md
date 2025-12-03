[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / textInput

# Function: textInput()

> **textInput**(): `Widget`

Defined in: [gui.ts:147](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/gui.ts#L147)

Create a text input widget for user text entry

## Returns

`Widget`

A Widget instance

## Example

```typescript
import { textInput } from "@tego/botjs";

// Simple text input
const nameInput = textInput()
  .withId("name")
  .withPlaceholder("Enter your name");

// Password input
const passwordInput = textInput()
  .withId("password")
  .withPlaceholder("Enter password")
  .withPassword(true);
```
