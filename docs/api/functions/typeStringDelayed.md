[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / typeStringDelayed

# Function: typeStringDelayed()

> **typeStringDelayed**(`text`, `cpm`): `void`

Defined in: [index.ts:318](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/index.ts#L318)

Type a string with a specified delay between characters (simulates human typing speed)

## Parameters

### text

`string`

Text string to type

### cpm

`number`

Characters per minute (typing speed)

## Returns

`void`

## Example

```typescript
import { typeStringDelayed } from "@tego/botjs";

// Slow typing (300 characters per minute)
typeStringDelayed('Hello', 300);

// Fast typing (1000 characters per minute)
typeStringDelayed('Fast typing!', 1000);
```
