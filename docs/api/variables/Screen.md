[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / Screen

# Variable: Screen

> `const` **Screen**: *typeof* `Screen` = `bot.Screen`

Defined in: [index.ts:38](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/index.ts#L38)

Screen capture class for taking screenshots and getting pixel colors

## Example

```typescript
import { Screen } from "@tego/botjs";

const screen = new Screen();
const bitmap = await screen.capture(0, 0, 800, 600);
console.log(`Captured ${bitmap.width}x${bitmap.height} region`);
```
