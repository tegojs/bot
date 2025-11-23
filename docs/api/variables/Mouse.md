[**@tego/bot API Documentation v0.0.2**](../README.md)

***

[@tego/bot API Documentation](../globals.md) / Mouse

# Variable: Mouse

> `const` **Mouse**: *typeof* `Mouse` = `bot.Mouse`

Defined in: [index.ts:52](https://github.com/tegojs/bot/blob/89ac2876df45d1ed1dcc5ecd1e596298cff0b31f/packages/botjs/src/index.ts#L52)

Mouse automation class for controlling mouse movements and clicks

## Example

```typescript
import { Mouse } from "@tego/botjs";

const mouse = new Mouse();
mouse.moveMouse(100, 200);
mouse.mouseClick('left');
mouse.dragMouse(500, 500);
```
