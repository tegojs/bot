[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / checkbox

# Function: checkbox()

> **checkbox**(`labelText`, `checked`): `Widget`

Defined in: [gui.ts:189](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/gui.ts#L189)

Create a checkbox widget for boolean input

## Parameters

### labelText

`string`

The label displayed next to the checkbox

### checked

`boolean`

Initial checked state

## Returns

`Widget`

A Widget instance

## Example

```typescript
import { checkbox } from "@tego/botjs";

// Simple checkbox
const rememberMe = checkbox("Remember me", false)
  .withId("remember");

// Pre-checked checkbox
const acceptTerms = checkbox("I accept the terms", true)
  .withId("terms");
```
