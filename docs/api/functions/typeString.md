[**@tego/bot API Documentation v0.0.2**](../README.md)

***

[@tego/bot API Documentation](../globals.md) / typeString

# Function: typeString()

> **typeString**(`text`): `void`

Defined in: [index.ts:325](https://github.com/tegojs/bot/blob/89ac2876df45d1ed1dcc5ecd1e596298cff0b31f/packages/botjs/src/index.ts#L325)

Type a string of text by simulating individual keystrokes

## Parameters

### text

`string`

Text string to type

## Returns

`void`

## Example

```typescript
import { typeString } from "@tego/botjs";

// Type text
typeString('Hello, World!');

// Type email address
typeString('user@example.com');

// Type with special characters
typeString('Password123!@#');
```
