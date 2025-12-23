[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / setMouseDelay

# Function: setMouseDelay()

> **setMouseDelay**(`delay`): `void`

Defined in: [botjs/src/index.ts:216](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/index.ts#L216)

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
