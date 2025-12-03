[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / vbox

# Function: vbox()

> **vbox**(`children`): `Widget`

Defined in: [gui.ts:329](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/gui.ts#L329)

Create a vertical box layout that arranges children in a column

## Parameters

### children

`Widget`[]

Array of child widgets

## Returns

`Widget`

A Widget instance

## Example

```typescript
import { vbox, label, textInput, button } from "@tego/botjs";

// Login form
const loginForm = vbox([
  label("Username"),
  textInput().withId("username"),
  label("Password"),
  textInput().withId("password").withPassword(true),
  button("Login").withId("login"),
]).withSpacing(8);
```
