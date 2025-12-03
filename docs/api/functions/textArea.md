[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / textArea

# Function: textArea()

> **textArea**(): `Widget`

Defined in: [gui.ts:535](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/gui.ts#L535)

Create a multi-line text area widget

## Returns

`Widget`

A Widget instance

## Example

```typescript
import { textArea } from "@tego/botjs";

// Simple text area
const description = textArea()
  .withId("description")
  .withPlaceholder("Enter description...");

// Text area with custom rows
const notes = textArea()
  .withId("notes")
  .withRows(6);
```
