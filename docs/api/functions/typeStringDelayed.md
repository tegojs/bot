[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / typeStringDelayed

# Function: typeStringDelayed()

> **typeStringDelayed**(`text`, `cpm`): `void`

Defined in: [botjs/src/index.ts:324](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/index.ts#L324)

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
