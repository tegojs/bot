[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / typeString

# Function: typeString()

> **typeString**(`text`): `void`

Defined in: [botjs/src/index.ts:303](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/index.ts#L303)

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
