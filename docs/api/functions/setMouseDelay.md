[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / setMouseDelay

# Function: setMouseDelay()

> **setMouseDelay**(`delay`): `void`

Defined in: [botjs/src/index.ts:216](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/index.ts#L216)

Set the delay between mouse operations in milliseconds

## Parameters

### delay

`number`

Delay in milliseconds (applied after each mouse operation)

## Returns

`void`

## Example

```typescript
import { setMouseDelay, moveMouse } from "@tego/botjs";

// Set 50ms delay between operations
setMouseDelay(50);

// These will have 50ms delay between them
moveMouse(100, 100);
moveMouse(200, 200);
```
