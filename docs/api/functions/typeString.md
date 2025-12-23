[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / typeString

# Function: typeString()

> **typeString**(`text`): `void`

Defined in: [botjs/src/index.ts:303](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/index.ts#L303)

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
