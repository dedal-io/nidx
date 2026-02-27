# nidx

[![CI](https://github.com/dedal-io/nidx/actions/workflows/ci.yml/badge.svg)](https://github.com/dedal-io/nidx/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/nidx.svg)](https://crates.io/crates/nidx)
[![PyPI](https://img.shields.io/pypi/v/nidx.svg)](https://pypi.org/project/nidx/)
[![npm](https://img.shields.io/npm/v/nidx.svg)](https://www.npmjs.com/package/nidx)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Validate and extract information from national ID numbers across multiple countries.

The core library is written in Rust with zero dependencies and provides bindings for Python and JavaScript/WASM.

## Supported countries

| Country | Module | Extracted fields |
|---------|--------|-----------------|
| Albania | `albania` | Date of birth, sex, national status |

## Installation

### Rust

```toml
[dependencies]
nidx = "0.1"
```

Optional [serde](https://serde.rs/) support:

```toml
[dependencies]
nidx = { version = "0.1", features = ["serde"] }
```

### Python

```sh
pip install nidx
```

### JavaScript / TypeScript

```sh
npm install nidx
```

## Usage

### Rust

```rust
use nidx::albania;
use nidx::Sex;

fn main() {
    let info = albania::decode("J00101999W").unwrap();
    println!("{}", info.birthday);   // 1990-01-01
    println!("{}", info.sex);        // M
    println!("{}", info.is_national); // true

    assert!(albania::is_valid("J00101999W"));
    assert!(!albania::is_valid("invalid"));
}
```

### Python

```python
from nidx import albania

info = albania.decode("J00101999W")
print(info.birthday)    # 1990-01-01
print(info.sex)         # M
print(info.is_national) # True
print(info.year)        # 1990
print(info.month)       # 1
print(info.day)         # 1

assert albania.is_valid("J00101999W")
assert not albania.is_valid("invalid")
```

`decode` raises `ValueError` on invalid input.

### JavaScript

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

`albaniaDecode` throws on invalid input.

## API

Each country module exposes `decode(nid) -> NidInfo` and `is_valid(nid) -> bool`.

### Albania

`albania::decode(nid)` validates and decodes a 10-character Albanian NID string. Input is case-insensitive.

**NidInfo fields:**

| Field | Rust | Python | JavaScript |
|-------|------|--------|------------|
| Date of birth (ISO 8601) | `birthday: Date` | `birthday: str` | `birthday: string` |
| Year | `birthday.year: u16` | `year: int` | `year: number` |
| Month | `birthday.month: u8` | `month: int` | `month: number` |
| Day | `birthday.day: u8` | `day: int` | `day: number` |
| Sex | `sex: Sex` | `sex: str` | `sex: string` |
| National | `is_national: bool` | `is_national: bool` | `isNational: boolean` |

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

[MIT](LICENSE)
