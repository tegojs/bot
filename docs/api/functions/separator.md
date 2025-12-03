[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / separator

# Function: separator()

> **separator**(): `Widget`

Defined in: [gui.ts:258](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/gui.ts#L258)

Create a horizontal separator line

## Returns

`Widget`

A Widget instance

## Example

```typescript
import { separator, vbox, label } from "@tego/botjs";

const content = vbox([
  label("Section 1"),
  separator(),
  label("Section 2"),
]);
```
