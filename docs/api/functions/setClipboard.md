[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / setClipboard

# Function: setClipboard()

> **setClipboard**(`text`): `void`

Defined in: [botjs/src/index.ts:553](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/index.ts#L553)

Set text content to the system clipboard

## Parameters

### text

`string`

Text to copy to clipboard

## Returns

`void`

## Example

```typescript
import { setClipboard } from "@tego/botjs";

setClipboard('Hello from @tego/bot!');
setClipboard('user@example.com');
```
