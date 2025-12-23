[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / Screen

# Variable: Screen

> `const` **Screen**: *typeof* `Screen` = `bot.Screen`

Defined in: [botjs/src/index.ts:44](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/index.ts#L44)

Screen capture class for taking screenshots and getting pixel colors

## Example

```typescript
import { Screen } from "@tego/botjs";

const screen = new Screen();
const bitmap = await screen.capture(0, 0, 800, 600);
console.log(`Captured ${bitmap.width}x${bitmap.height} region`);
```
