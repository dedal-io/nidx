# nidx

[![PyPI](https://img.shields.io/pypi/v/nidx.svg)](https://pypi.org/project/nidx/)
[![CI](https://github.com/dedal-io/nidx/actions/workflows/ci.yml/badge.svg)](https://github.com/dedal-io/nidx/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/dedal-io/nidx/blob/main/LICENSE)

Validate and extract information from national ID numbers across multiple countries.

Also available for [Rust](https://crates.io/crates/nidx) and [JavaScript](https://www.npmjs.com/package/nidx).

## Installation

```sh
pip install nidx
```

Requires Python 3.9+.

## Supported countries

| Country | Module | Extracted fields |
|---------|--------|-----------------|
| Albania | `albania` | Date of birth, sex, national status |
| Kosovo  | `kosovo`  | Validation only |

## Usage

### Albania

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

`decode` raises `NidFormatError`, `NidChecksumError`, or `NidInvalidDateError` on invalid input (all subclasses of `NidError`, which is a `ValueError`).

### Kosovo

```python
from nidx import kosovo

kosovo.validate("1234567892")  # raises on invalid input

assert kosovo.is_valid("1234567892")
assert not kosovo.is_valid("invalid")
```

## API

### Albania

`albania.decode(nid: str) -> NidInfo` — validates and decodes a 10-character Albanian NID. Input is case-insensitive.

| Field | Type | Description |
|-------|------|-------------|
| `birthday` | `str` | Date of birth (ISO 8601) |
| `year` | `int` | Birth year |
| `month` | `int` | Birth month |
| `day` | `int` | Birth day |
| `sex` | `str` | `"M"` or `"F"` |
| `is_national` | `bool` | Whether the person is an Albanian national |

`albania.is_valid(nid: str) -> bool` — returns `True` if the NID is valid.

### Kosovo

`kosovo.validate(nid: str) -> None` — validates a 10-digit Kosovo personal number. Raises on invalid input.

`kosovo.is_valid(nid: str) -> bool` — returns `True` if the personal number is valid.

### Exceptions

| Exception | Parent | Raised when |
|-----------|--------|-------------|
| `NidError` | `ValueError` | Base exception for all NID errors |
| `NidFormatError` | `NidError` | Input has wrong length or characters |
| `NidChecksumError` | `NidError` | Checksum digit doesn't match |
| `NidInvalidDateError` | `NidError` | Encoded date is not a valid calendar date |

## License

[MIT](https://github.com/dedal-io/nidx/blob/main/LICENSE)
