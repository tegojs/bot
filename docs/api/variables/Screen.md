[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / Screen

# Variable: Screen

> `const` **Screen**: *typeof* `Screen` = `bot.Screen`

Defined in: [botjs/src/index.ts:44](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/index.ts#L44)

Screen capture class for taking screenshots and getting pixel colors

## Example

```typescript
import { Screen } from "@tego/botjs";

const screen = new Screen();
const bitmap = await screen.capture(0, 0, 800, 600);
console.log(`Captured ${bitmap.width}x${bitmap.height} region`);
```
