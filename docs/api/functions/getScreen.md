[**@tego/bot API Documentation v0.0.2**](../README.md)

***

[@tego/bot API Documentation](../globals.md) / getScreen

# Function: getScreen()

> **getScreen**(): `Screen`

Defined in: [index.ts:502](https://github.com/tegojs/bot/blob/89ac2876df45d1ed1dcc5ecd1e596298cff0b31f/packages/botjs/src/index.ts#L502)

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
