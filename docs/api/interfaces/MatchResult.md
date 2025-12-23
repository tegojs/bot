[**Tego Bot API Documentation v0.2.0**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / MatchResult

# Interface: MatchResult

Defined in: [botjs/src/image-match.ts:75](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/image-match.ts#L75)

Result from a successful image template match

## Properties

### x

> **x**: `number`

Defined in: [botjs/src/image-match.ts:77](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/image-match.ts#L77)

X coordinate of the match (top-left corner)

***

### y

> **y**: `number`

Defined in: [botjs/src/image-match.ts:79](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/image-match.ts#L79)

Y coordinate of the match (top-left corner)

***

### width

> **width**: `number`

Defined in: [botjs/src/image-match.ts:81](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/image-match.ts#L81)

Width of the matched region

***

### height

> **height**: `number`

Defined in: [botjs/src/image-match.ts:83](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/image-match.ts#L83)

Height of the matched region

***

### confidence

> **confidence**: `number`

Defined in: [botjs/src/image-match.ts:85](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/image-match.ts#L85)

Confidence score from 0.0 to 1.0 (higher = better match)

***

### scale

> **scale**: `number`

Defined in: [botjs/src/image-match.ts:87](https://github.com/tegojs/bot/blob/0a4decde0a125e094c5c44e05c0e4efe6c9e05df/packages/botjs/src/image-match.ts#L87)

Scale at which the template was matched (1.0 = original size)
