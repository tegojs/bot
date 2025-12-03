[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / group

# Function: group()

> **group**(`title`, `child`): `Widget`

Defined in: [gui.ts:426](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/gui.ts#L426)

Create a collapsible group container with a title

## Parameters

### title

`string`

The group title displayed in the header

### child

`Widget`

The child widget contained in the group

## Returns

`Widget`

A Widget instance

## Example

```typescript
import { group, vbox, checkbox } from "@tego/botjs";

// Settings group
const settingsGroup = group("Advanced Settings",
  vbox([
    checkbox("Enable feature A", false),
    checkbox("Enable feature B", true),
  ])
).withCollapsed(true);
```
