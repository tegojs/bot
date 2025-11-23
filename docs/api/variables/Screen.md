[**@tego/bot API Documentation v0.0.2**](../README.md)

***

[@tego/bot API Documentation](../globals.md) / Screen

# Variable: Screen

> `const` **Screen**: *typeof* `Screen` = `bot.Screen`

Defined in: [index.ts:66](https://github.com/tegojs/bot/blob/89ac2876df45d1ed1dcc5ecd1e596298cff0b31f/packages/botjs/src/index.ts#L66)

Screen capture class for taking screenshots and getting pixel colors

## Example

```typescript
import { Screen } from "@tego/botjs";

const screen = new Screen();
const bitmap = await screen.capture(0, 0, 800, 600);
console.log(`Captured ${bitmap.width}x${bitmap.height} region`);
```
