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
| Kosovo  | `kosovo`  | Validation only |

## Installation

### Rust

```toml
[dependencies]
nidx = "0.2"
```

Optional [serde](https://serde.rs/) support:

```toml
[dependencies]
nidx = { version = "0.2", features = ["serde"] }
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
use nidx::kosovo;
use nidx::Sex;

fn main() {
    // Albania: decode extracts structured data
    let info = albania::decode("J00101999W").unwrap();
    println!("{}", info.birthday);   // 1990-01-01
    println!("{}", info.sex);        // M
    println!("{}", info.is_national); // true

    assert!(albania::is_valid("J00101999W"));

    // Kosovo: validation only
    assert!(kosovo::validate("1234567892").is_ok());
    assert!(kosovo::is_valid("1234567892"));
}
```

### Python

```python
from nidx import albania, kosovo

# Albania: decode extracts structured data
info = albania.decode("J00101999W")
print(info.birthday)    # 1990-01-01
print(info.sex)         # M
print(info.is_national) # True

assert albania.is_valid("J00101999W")

# Kosovo: validation only
kosovo.validate("1234567892")  # raises on invalid input
assert kosovo.is_valid("1234567892")
```

`decode` raises `ValueError` on invalid input.

### JavaScript

```typescript
import { Albania, Kosovo } from "nidx";

// Albania: decode extracts structured data
const info = Albania.decode("J00101999W");
console.log(info.birthday);   // "1990-01-01"
console.log(info.sex);        // "M"
console.log(info.isNational); // true

console.log(Albania.isValid("J00101999W")); // true

// Kosovo: validation only
Kosovo.validate("1234567892"); // throws on invalid input
console.log(Kosovo.isValid("1234567892")); // true
```

`Albania.decode` throws on invalid input.

## API

### Albania

Each language exposes `decode(nid)` and `is_valid(nid)`.

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

### Kosovo

Each language exposes `validate(nid)` and `is_valid(nid)`.

`kosovo::validate(nid)` checks a 10-digit Kosovo personal number. Returns an error (or throws) on invalid input.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

[MIT](LICENSE)
