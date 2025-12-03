[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / getScreenSize

# Function: getScreenSize()

> **getScreenSize**(): `ScreenSizeResult`

Defined in: [index.ts:491](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/index.ts#L491)

Get the dimensions of the primary screen

## Returns

`ScreenSizeResult`

Object containing width and height in pixels

## Example

```typescript
import { getScreenSize } from "@tego/botjs";

const size = getScreenSize();
console.log(`Screen resolution: ${size.width}x${size.height}`);
```
