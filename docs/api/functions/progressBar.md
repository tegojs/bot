[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / progressBar

# Function: progressBar()

> **progressBar**(`value`): `Widget`

Defined in: [gui.ts:238](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/gui.ts#L238)

Create a progress bar widget for displaying progress

## Parameters

### value

`number`

Progress value (0.0 to 1.0)

## Returns

`Widget`

A Widget instance

## Example

```typescript
import { progressBar } from "@tego/botjs";

// 50% progress
const downloadProgress = progressBar(0.5)
  .withId("download-progress");

// Complete progress
const completeProgress = progressBar(1.0);
```
