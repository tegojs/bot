[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / setMouseDelay

# Function: setMouseDelay()

> **setMouseDelay**(`delay`): `void`

Defined in: [index.ts:210](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/index.ts#L210)

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
