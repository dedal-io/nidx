# nidx

[![npm](https://img.shields.io/npm/v/nidx.svg)](https://www.npmjs.com/package/nidx)
[![CI](https://github.com/dedal-io/nidx/actions/workflows/ci.yml/badge.svg)](https://github.com/dedal-io/nidx/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/dedal-io/nidx/blob/main/LICENSE)

Validate and extract information from national ID numbers across multiple countries. Powered by Rust via WebAssembly.

Also available for [Rust](https://crates.io/crates/nidx) and [Python](https://pypi.org/project/nidx/).

## Installation

```sh
npm install nidx
```

## Supported countries

| Country | Functions | Extracted fields |
|---------|-----------|-----------------|
| Albania | `albaniaDecode`, `albaniaIsValid` | Date of birth, sex, national status |
| Kosovo  | `kosovoValidate`, `kosovoIsValid` | Validation only |

## Usage

### Albania

```javascript
import { albaniaDecode, albaniaIsValid } from "nidx";

const info = albaniaDecode("J00101999W");
console.log(info.birthday);   // "1990-01-01"
console.log(info.sex);        // "M"
console.log(info.isNational); // true
console.log(info.year);       // 1990
console.log(info.month);      // 1
console.log(info.day);        // 1

console.log(albaniaIsValid("J00101999W")); // true
console.log(albaniaIsValid("invalid"));    // false
```

`albaniaDecode` throws on invalid input with an error code prefix: `[FORMAT]`, `[CHECKSUM]`, or `[INVALID_DATE]`.

### Kosovo

```javascript
import { kosovoValidate, kosovoIsValid } from "nidx";

kosovoValidate("1234567892"); // throws on invalid input

console.log(kosovoIsValid("1234567892")); // true
console.log(kosovoIsValid("invalid"));    // false
```

`kosovoValidate` throws on invalid input with an error code prefix: `[FORMAT]` or `[CHECKSUM]`.

## API

### Albania

`albaniaDecode(nid: string): NidInfo` — validates and decodes a 10-character Albanian NID. Input is case-insensitive.

| Property | Type | Description |
|----------|------|-------------|
| `birthday` | `string` | Date of birth (ISO 8601) |
| `year` | `number` | Birth year |
| `month` | `number` | Birth month |
| `day` | `number` | Birth day |
| `sex` | `string` | `"M"` or `"F"` |
| `isNational` | `boolean` | Whether the person is an Albanian national |

`NidInfo` also has a `toJSON()` method that returns a plain object.

`albaniaIsValid(nid: string): boolean` — returns `true` if the NID is valid.

### Kosovo

`kosovoValidate(nid: string): void` — validates a 10-digit Kosovo personal number. Throws on invalid input.

`kosovoIsValid(nid: string): boolean` — returns `true` if the personal number is valid.

## License

[MIT](https://github.com/dedal-io/nidx/blob/main/LICENSE)
