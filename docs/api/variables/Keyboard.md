[**@tego/bot API Documentation v0.0.2**](../README.md)

***

[@tego/bot API Documentation](../globals.md) / Keyboard

# Variable: Keyboard

> `const` **Keyboard**: *typeof* `Keyboard` = `bot.Keyboard`

Defined in: [index.ts:37](https://github.com/tegojs/bot/blob/89ac2876df45d1ed1dcc5ecd1e596298cff0b31f/packages/botjs/src/index.ts#L37)

Keyboard automation class for simulating keyboard input

## Example

```typescript
import { Keyboard } from "@tego/botjs";

const keyboard = new Keyboard();
keyboard.keyTap('a');
keyboard.typeString('Hello World');
keyboard.keyTap('c', ['control']); // Ctrl+C
```
