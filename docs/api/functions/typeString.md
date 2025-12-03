[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / typeString

# Function: typeString()

> **typeString**(`text`): `void`

Defined in: [index.ts:297](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/index.ts#L297)

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
