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

| Country | Namespace | Extracted fields |
|---------|-----------|-----------------|
| Albania | `Albania` | Date of birth, sex, national status |
| Kosovo  | `Kosovo`  | Validation only |

## Usage

### Albania

```typescript
import { Albania } from "nidx";

const info = Albania.decode("J00101999W");
console.log(info.birthday);   // "1990-01-01"
console.log(info.sex);        // "M"
console.log(info.isNational); // true
console.log(info.year);       // 1990
console.log(info.month);      // 1
console.log(info.day);        // 1

console.log(Albania.isValid("J00101999W")); // true
console.log(Albania.isValid("invalid"));    // false
```

`Albania.decode` throws on invalid input with an error code prefix: `[FORMAT]`, `[CHECKSUM]`, or `[INVALID_DATE]`.

### Kosovo

```typescript
import { Kosovo } from "nidx";

Kosovo.validate("1234567892"); // throws on invalid input

console.log(Kosovo.isValid("1234567892")); // true
console.log(Kosovo.isValid("invalid"));    // false
```

`Kosovo.validate` throws on invalid input with an error code prefix: `[FORMAT]` or `[CHECKSUM]`.

## API

### Albania

`Albania.decode(nid: string): NidInfo` — validates and decodes a 10-character Albanian NID. Input is case-insensitive.

| Property | Type | Description |
|----------|------|-------------|
| `birthday` | `string` | Date of birth (ISO 8601) |
| `year` | `number` | Birth year |
| `month` | `number` | Birth month |
| `day` | `number` | Birth day |
| `sex` | `string` | `"M"` or `"F"` |
| `isNational` | `boolean` | Whether the person is an Albanian national |

`NidInfo` also has a `toJSON()` method that returns a plain object.

`Albania.isValid(nid: string): boolean` — returns `true` if the NID is valid.

### Kosovo

`Kosovo.validate(nid: string): void` — validates a 10-digit Kosovo personal number. Throws on invalid input.

`Kosovo.isValid(nid: string): boolean` — returns `true` if the personal number is valid.

## License

[MIT](https://github.com/dedal-io/nidx/blob/main/LICENSE)
