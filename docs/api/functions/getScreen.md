[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / getScreen

# Function: getScreen()

> **getScreen**(): `Screen`

Defined in: [botjs/src/index.ts:480](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/index.ts#L480)

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
