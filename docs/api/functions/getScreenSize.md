[**@tego/bot API Documentation v0.0.2**](../README.md)

***

[@tego/bot API Documentation](../globals.md) / getScreenSize

# Function: getScreenSize()

> **getScreenSize**(): `ScreenSize`

Defined in: [index.ts:519](https://github.com/tegojs/bot/blob/89ac2876df45d1ed1dcc5ecd1e596298cff0b31f/packages/botjs/src/index.ts#L519)

Get the dimensions of the primary screen

## Returns

`ScreenSize`

Object containing width and height in pixels

## Example

```typescript
import { getScreenSize } from "@tego/botjs";

const size = getScreenSize();
console.log(`Screen resolution: ${size.width}x${size.height}`);
```
