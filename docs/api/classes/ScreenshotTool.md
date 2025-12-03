[**Tego Bot API Documentation v0.1.3**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / ScreenshotTool

# Class: ScreenshotTool

Defined in: [screenshot.ts:183](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/screenshot.ts#L183)

Advanced screenshot tool with interactive selection, color picking, and annotations

## Example

```typescript
import { ScreenshotTool } from "@tego/botjs";

const tool = new ScreenshotTool({
  autoCopyToClipboard: true
});

// Quick screenshot
const screenshot = await tool.captureQuick();
await saveScreenshotToFile(screenshot, 'screenshot.png');

// Region screenshot
const region = await tool.captureQuick({ x: 0, y: 0, width: 800, height: 600 });

// Get pixel color
const color = await tool.getPixelColor(100, 200);
console.log(color.hex); // #FF5733
```

## Constructors

### Constructor

> **new ScreenshotTool**(`options?`): `ScreenshotTool`

Defined in: [screenshot.ts:190](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/screenshot.ts#L190)

Create a new screenshot tool instance

#### Parameters

##### options?

[`ScreenshotToolOptions`](../interfaces/ScreenshotToolOptions.md)

Configuration options

#### Returns

`ScreenshotTool`

## Methods

### captureInteractive()

> **captureInteractive**(`options?`): `Promise`\<[`ScreenshotResult`](../interfaces/ScreenshotResult.md)\>

Defined in: [screenshot.ts:212](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/screenshot.ts#L212)

Capture screenshot interactively with UI overlay

Note: Interactive mode is not yet fully implemented.
Use captureQuick() for programmatic screenshots.

#### Parameters

##### options?

[`InteractiveCaptureOptions`](../interfaces/InteractiveCaptureOptions.md)

Interactive capture options

#### Returns

`Promise`\<[`ScreenshotResult`](../interfaces/ScreenshotResult.md)\>

Screenshot result

#### Example

```typescript
const screenshot = await tool.captureInteractive({
  showCoordinates: true,
  enableWindowSnap: true
});
```

***

### captureQuick()

> **captureQuick**(`region?`): `Promise`\<[`ScreenshotResult`](../interfaces/ScreenshotResult.md)\>

Defined in: [screenshot.ts:236](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/screenshot.ts#L236)

Quick screenshot without user interaction

#### Parameters

##### region?

[`ScreenRegion`](../interfaces/ScreenRegion.md)

Optional region to capture. If not specified, captures entire screen

#### Returns

`Promise`\<[`ScreenshotResult`](../interfaces/ScreenshotResult.md)\>

Screenshot result

#### Example

```typescript
// Full screen
const fullScreen = await tool.captureQuick();

// Specific region
const region = await tool.captureQuick({
  x: 100, y: 100, width: 800, height: 600
});
```

***

### getPixelColor()

> **getPixelColor**(`x`, `y`): `Promise`\<[`ColorInfo`](../interfaces/ColorInfo.md)\>

Defined in: [screenshot.ts:258](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/screenshot.ts#L258)

Get pixel color at specific screen coordinates

#### Parameters

##### x

`number`

X coordinate

##### y

`number`

Y coordinate

#### Returns

`Promise`\<[`ColorInfo`](../interfaces/ColorInfo.md)\>

Color information in multiple formats

#### Example

```typescript
const color = await tool.getPixelColor(500, 300);
console.log(color.hex);   // #FF5733
console.log(color.rgb);   // { r: 255, g: 87, b: 51 }
console.log(color.hsl);   // { h: 11, s: 100, l: 60 }
```

***

### pickColor()

> **pickColor**(`options?`): `Promise`\<[`ColorInfo`](../interfaces/ColorInfo.md)\>

Defined in: [screenshot.ts:310](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/screenshot.ts#L310)

Start interactive color picker

Note: Interactive mode is not yet fully implemented.
Use getPixelColor() for programmatic color picking.

#### Parameters

##### options?

[`ColorPickerOptions`](../interfaces/ColorPickerOptions.md)

Color picker options

#### Returns

`Promise`\<[`ColorInfo`](../interfaces/ColorInfo.md)\>

Selected color information

***

### close()

> **close**(): `Promise`\<`void`\>

Defined in: [screenshot.ts:324](https://github.com/tegojs/bot/blob/3a83e5320af7390daf79eaa464ba6d0391a7e544/packages/botjs/src/screenshot.ts#L324)

Close and cleanup resources

#### Returns

`Promise`\<`void`\>
