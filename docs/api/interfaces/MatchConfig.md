[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../README.md) / MatchConfig

# Interface: MatchConfig

Defined in: [botjs/src/image-match.ts:34](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/image-match.ts#L34)

Configuration options for image template matching

## Properties

### searchMultipleScales?

> `optional` **searchMultipleScales**: `boolean`

Defined in: [botjs/src/image-match.ts:40](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/image-match.ts#L40)

Search at multiple scales to find scaled versions of the template.
Useful when UI elements may be displayed at different sizes.

#### Default

```ts
true
```

***

### useGrayscale?

> `optional` **useGrayscale**: `boolean`

Defined in: [botjs/src/image-match.ts:47](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/image-match.ts#L47)

Convert images to grayscale before matching.
Can improve matching for color-independent patterns.

#### Default

```ts
false
```

***

### scaleSteps?

> `optional` **scaleSteps**: `number`[]

Defined in: [botjs/src/image-match.ts:54](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/image-match.ts#L54)

Scale factors to search at when searchMultipleScales is true.
Values less than 1.0 search for smaller versions of the template.

#### Default

```ts
[1.0, 0.9, 0.8, 0.7, 0.6, 0.5]
```

***

### confidence?

> `optional` **confidence**: `number`

Defined in: [botjs/src/image-match.ts:62](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/image-match.ts#L62)

Minimum confidence threshold (0.0 to 1.0).
Higher values require closer matches but may miss valid results.
Lower values find more matches but may include false positives.

#### Default

```ts
0.8
```

***

### limit?

> `optional` **limit**: `number`

Defined in: [botjs/src/image-match.ts:69](https://github.com/tegojs/bot/blob/e85da06c4eac4d389045c1611f9140c5dd131bdf/packages/botjs/src/image-match.ts#L69)

Maximum number of results to return.
Results are sorted by confidence descending.

#### Default

```ts
100
```
