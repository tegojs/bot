[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / getScreen

# Function: getScreen()

> **getScreen**(): `Screen`

Defined in: [index.ts:474](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/index.ts#L474)

Get the global Screen instance for capture operations

## Returns

`Screen`

Screen object

## Example

```typescript
import { getScreen } from "@tego/botjs";

const screen = getScreen();
const bitmap = await screen.capture(0, 0, 800, 600);
```
