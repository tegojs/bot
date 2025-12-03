[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / setClipboard

# Function: setClipboard()

> **setClipboard**(`text`): `void`

Defined in: [index.ts:547](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/index.ts#L547)

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
