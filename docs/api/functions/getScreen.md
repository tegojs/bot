[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / getScreen

# Function: getScreen()

> **getScreen**(): `Screen`

Defined in: [botjs/src/index.ts:480](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/index.ts#L480)

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
