[**@tego/bot API Documentation v0.0.2**](../README.md)

***

[@tego/bot API Documentation](../globals.md) / typeStringDelayed

# Function: typeStringDelayed()

> **typeStringDelayed**(`text`, `cpm`): `void`

Defined in: [index.ts:346](https://github.com/tegojs/bot/blob/89ac2876df45d1ed1dcc5ecd1e596298cff0b31f/packages/botjs/src/index.ts#L346)

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
