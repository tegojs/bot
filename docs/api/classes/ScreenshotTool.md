[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / ScreenshotTool

# Class: ScreenshotTool

Defined in: [botjs/src/screenshot.ts:181](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/screenshot.ts#L181)

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

Defined in: [botjs/src/screenshot.ts:188](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/screenshot.ts#L188)

Create a new screenshot tool instance

#### Parameters

##### options?

[`ScreenshotToolOptions`](../interfaces/ScreenshotToolOptions.md)

Configuration options

#### Returns

`ScreenshotTool`

## Methods

### captureInteractive()

> **captureInteractive**(`_options?`): `Promise`\<[`ScreenshotResult`](../interfaces/ScreenshotResult.md)\>

Defined in: [botjs/src/screenshot.ts:209](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/screenshot.ts#L209)

Capture screenshot interactively with UI overlay

Note: Interactive mode is not yet fully implemented.
Use captureQuick() for programmatic screenshots.

#### Parameters

##### \_options?

[`InteractiveCaptureOptions`](../interfaces/InteractiveCaptureOptions.md)

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

Defined in: [botjs/src/screenshot.ts:233](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/screenshot.ts#L233)

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

Defined in: [botjs/src/screenshot.ts:255](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/screenshot.ts#L255)

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

> **pickColor**(`_options?`): `Promise`\<[`ColorInfo`](../interfaces/ColorInfo.md)\>

Defined in: [botjs/src/screenshot.ts:306](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/screenshot.ts#L306)

Start interactive color picker

Note: Interactive mode is not yet fully implemented.
Use getPixelColor() for programmatic color picking.

#### Parameters

##### \_options?

[`ColorPickerOptions`](../interfaces/ColorPickerOptions.md)

#### Returns

`Promise`\<[`ColorInfo`](../interfaces/ColorInfo.md)\>

Selected color information

***

### close()

> **close**(): `Promise`\<`void`\>

Defined in: [botjs/src/screenshot.ts:320](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/screenshot.ts#L320)

Close and cleanup resources

#### Returns

`Promise`\<`void`\>
