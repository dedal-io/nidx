//! Validate and decode Albanian National ID (NID) numbers.
//!
//! Albanian NIDs are 10-character alphanumeric strings that encode date of birth,
//! sex, national status, and a checksum.
//!
//! # Format
//!
//! `[decade][year_digit][month_code (2)][day (2)][serial (3)][checksum]`
//!
//! - **Decade char**: `0`–`9` maps to 1800–1890, `A`–`T` maps to 1900–2090.
//! - **Month code** encodes both the calendar month and sex/national status:
//!   - `01`–`12` = male Albanian
//!   - `31`–`42` = male foreigner
//!   - `51`–`62` = female Albanian
//!   - `81`–`92` = female foreigner
//! - **Checksum**: weighted sum of the first 9 characters mod 23.
//!
//! # Examples
//!
//! ```
//! let info = nidx::albania::decode("J00101999W").unwrap();
//! assert_eq!(info.birthday.to_string(), "1990-01-01");
//! assert_eq!(info.sex, nidx::Sex::Male);
//! assert!(info.is_national);
//!
//! assert!(nidx::albania::is_valid("J00101999W"));
//! assert!(!nidx::albania::is_valid("invalid"));
//! ```

use std::fmt;

use crate::date::validate_date;
use crate::{Date, Sex};

/// Maps decade character position to base year: index * 10 + 1800.
const DECADE_CHARS: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRST";

/// Alphabet used for checksum computation and the 10th (check) character.
const CHECKSUM_CHARS: &[u8] = b"WABCDEFGHIJKLMNOPQRSTUV";

/// Decode a two-digit month code into (offset, sex, is_national).
///
/// Month codes encode both the calendar month and sex/national status:
/// - `01`–`12` = male Albanian (offset 0)
/// - `31`–`42` = male foreigner (offset 30)
/// - `51`–`62` = female Albanian (offset 50)
/// - `81`–`92` = female foreigner (offset 80)
fn decode_month_code(code: u8) -> Option<(u8, Sex, bool)> {
    match code {
        1..=12 => Some((0, Sex::Male, true)),
        31..=42 => Some((30, Sex::Male, false)),
        51..=62 => Some((50, Sex::Female, true)),
        81..=92 => Some((80, Sex::Female, false)),
        _ => None,
    }
}

/// Decoded information from a valid Albanian NID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NidInfo {
    /// Date of birth.
    pub birthday: Date,
    /// Biological sex.
    pub sex: Sex,
    /// `true` if the NID holder is an Albanian national, `false` for foreign residents.
    pub is_national: bool,
}

/// Specific reason a NID was rejected due to formatting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum FormatKind {
    /// Input is not exactly 10 characters.
    InvalidLength,
    /// The first (decade) character is not in the valid range `0`–`9`, `A`–`T`.
    InvalidDecadeChar,
    /// Characters at positions 2–9 are not all ASCII digits.
    NonDigitCharacter,
    /// The checksum (10th) character is not in the checksum alphabet.
    InvalidChecksumChar,
    /// The two-digit month code does not map to any known range.
    InvalidMonthCode {
        /// The raw two-digit month code that was found.
        code: u8,
    },
}

impl fmt::Display for FormatKind {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormatKind::InvalidLength => write!(f, "NID must be exactly 10 characters"),
            FormatKind::InvalidDecadeChar => write!(f, "first character out of range"),
            FormatKind::NonDigitCharacter => write!(f, "characters 2-9 must be ASCII digits"),
            FormatKind::InvalidChecksumChar => write!(f, "invalid checksum character"),
            FormatKind::InvalidMonthCode { code } => {
                write!(f, "invalid month code: {code}")
            }
        }
    }
}

impl std::error::Error for FormatKind {}

/// Specific reason a NID was rejected due to an invalid date.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DateKind {
    /// The decoded month is outside 1–12.
    MonthOutOfRange {
        /// The decoded month value.
        month: u8,
    },
    /// The decoded day is outside the valid range for the given year and month.
    DayOutOfRange {
        /// The decoded year.
        year: u16,
        /// The decoded month.
        month: u8,
        /// The decoded day.
        day: u8,
    },
}

impl fmt::Display for DateKind {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DateKind::MonthOutOfRange { month } => write!(f, "month {month} out of range"),
            DateKind::DayOutOfRange { year, month, day } => {
                write!(f, "day {day} is out of range for {year}-{month:02}")
            }
        }
    }
}

impl std::error::Error for DateKind {}

/// Errors that can occur when decoding an Albanian NID.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum NidError {
    /// The input string has an invalid format (wrong length, illegal characters, etc.).
    Format(FormatKind),
    /// The checksum character does not match the computed value.
    Checksum,
    /// The encoded date is not a valid calendar date.
    InvalidDate(DateKind),
}

impl fmt::Display for NidError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NidError::Format(kind) => write!(f, "format error: {kind}"),
            NidError::Checksum => write!(f, "checksum validation failed"),
            NidError::InvalidDate(kind) => write!(f, "invalid date: {kind}"),
        }
    }
}

impl std::error::Error for NidError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            NidError::Format(kind) => Some(kind),
            NidError::InvalidDate(kind) => Some(kind),
            _ => None,
        }
    }
}

fn verify_checksum(bytes: &[u8; 10]) -> Result<(), NidError> {
    let check_char = bytes[9];
    if !CHECKSUM_CHARS.contains(&check_char) {
        return Err(NidError::Format(FormatKind::InvalidChecksumChar));
    }

    let mut total: usize = 0;
    for (i, &ch) in bytes[..9].iter().enumerate() {
        // Position 0 uses weight 1 (not 0) per spec, so the decade char contributes to the checksum.
        let weight = if i == 0 { 1 } else { i };
        let value = if ch.is_ascii_digit() {
            (ch - b'0') as usize
        } else {
            CHECKSUM_CHARS
                .iter()
                .position(|&c| c == ch)
                .ok_or(NidError::Format(FormatKind::InvalidChecksumChar))?
        };
        total += weight * value;
    }

    if CHECKSUM_CHARS[total % 23] == check_char {
        Ok(())
    } else {
        Err(NidError::Checksum)
    }
}

/// Decode an Albanian National ID string into its constituent parts.
///
/// The input is treated case-insensitively. Returns a [`NidInfo`] on success,
/// or a [`NidError`] describing why the input is invalid.
///
/// # Errors
///
/// Returns [`NidError::Format`] if the input has wrong length, illegal characters,
/// or an unrecognised month code. Returns [`NidError::Checksum`] if the check digit
/// does not match. Returns [`NidError::InvalidDate`] if the encoded date is not a
/// valid calendar date.
///
/// # Examples
///
/// ```
/// let info = nidx::albania::decode("J00101999W").unwrap();
/// assert_eq!(info.birthday.year, 1990);
/// assert_eq!(info.birthday.month, 1);
/// assert_eq!(info.birthday.day, 1);
/// assert_eq!(info.sex, nidx::Sex::Male);
/// assert!(info.is_national);
/// ```
///
/// ```
/// let err = nidx::albania::decode("invalid").unwrap_err();
/// assert!(matches!(err, nidx::albania::NidError::Format(_)));
/// ```
#[must_use = "this returns the decoded NID info; use `is_valid` if you only need a bool"]
pub fn decode(nid: &str) -> Result<NidInfo, NidError> {
    let src = nid.as_bytes();
    if src.len() != 10 {
        return Err(NidError::Format(FormatKind::InvalidLength));
    }
    let mut bytes = [0u8; 10];
    bytes.copy_from_slice(src);
    bytes.make_ascii_uppercase();

    let decade_index = DECADE_CHARS
        .iter()
        .position(|&c| c == bytes[0])
        .ok_or(NidError::Format(FormatKind::InvalidDecadeChar))?;

    if !bytes[1..9].iter().all(|b| b.is_ascii_digit()) {
        return Err(NidError::Format(FormatKind::NonDigitCharacter));
    }

    verify_checksum(&bytes)?;

    let year = 1800 + (decade_index as u16 * 10) + (bytes[1] - b'0') as u16;

    let month_code = (bytes[2] - b'0') * 10 + (bytes[3] - b'0');

    let (offset, sex, is_national) =
        decode_month_code(month_code).ok_or(NidError::Format(FormatKind::InvalidMonthCode {
            code: month_code,
        }))?;

    let month = month_code - offset;

    let day = (bytes[4] - b'0') * 10 + (bytes[5] - b'0');

    let birthday = validate_date(year, month, day).ok_or(NidError::InvalidDate(
        if !(1..=12).contains(&month) {
            DateKind::MonthOutOfRange { month }
        } else {
            DateKind::DayOutOfRange { year, month, day }
        },
    ))?;

    Ok(NidInfo {
        birthday,
        sex,
        is_national,
    })
}

/// Check whether an Albanian National ID string is valid.
///
/// This is a convenience wrapper around [`decode`] that discards the decoded
/// information and returns a simple boolean.
///
/// # Examples
///
/// ```
/// assert!(nidx::albania::is_valid("J00101999W"));
/// assert!(!nidx::albania::is_valid("invalid"));
/// ```
#[inline]
#[must_use]
pub fn is_valid(nid: &str) -> bool {
    decode(nid).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_NID: &str = "J00101999W";

    /// Builds a valid NID from 9 content bytes by appending the computed checksum.
    fn make_nid(partial: &[u8; 9]) -> String {
        let total: usize = partial
            .iter()
            .enumerate()
            .map(|(i, &ch)| {
                let weight = if i == 0 { 1 } else { i };
                let value = if ch.is_ascii_digit() {
                    (ch - b'0') as usize
                } else {
                    CHECKSUM_CHARS.iter().position(|&c| c == ch).unwrap()
                };
                weight * value
            })
            .sum();
        let check = CHECKSUM_CHARS[total % 23] as char;
        format!("{}{check}", std::str::from_utf8(partial).unwrap())
    }

    #[test]
    fn decode_valid() {
        let info = decode(VALID_NID).unwrap();
        assert_eq!(
            info.birthday,
            Date {
                year: 1990,
                month: 1,
                day: 1
            }
        );
        assert_eq!(info.sex, Sex::Male);
        assert!(info.is_national);
    }

    #[test]
    fn decode_lowercase_input() {
        let info = decode("j00101999w").unwrap();
        assert_eq!(info.sex, Sex::Male);
        assert!(info.is_national);
    }

    #[test]
    fn is_valid_true() {
        assert!(is_valid(VALID_NID));
    }

    #[test]
    fn is_valid_false_empty() {
        assert!(!is_valid(""));
    }

    #[test]
    fn is_valid_false_wrong_format() {
        assert!(!is_valid("ABCDEFGHIJK"));
    }

    #[test]
    fn is_valid_false_bad_checksum() {
        let mut bad = VALID_NID.to_string();
        bad.replace_range(9..10, "Z");
        assert!(!is_valid(&bad));
    }

    #[test]
    fn error_too_short() {
        let err = decode("J00101").unwrap_err();
        assert_eq!(err, NidError::Format(FormatKind::InvalidLength));
    }

    #[test]
    fn error_too_long() {
        let err = decode("J0010199945X").unwrap_err();
        assert_eq!(err, NidError::Format(FormatKind::InvalidLength));
    }

    #[test]
    fn error_invalid_decade_char() {
        let err = decode("Z001011230").unwrap_err();
        assert_eq!(err, NidError::Format(FormatKind::InvalidDecadeChar));
    }

    #[test]
    fn error_non_digit_middle() {
        let err = decode("J0A101123R").unwrap_err();
        assert_eq!(err, NidError::Format(FormatKind::NonDigitCharacter));
    }

    #[test]
    fn error_invalid_date_feb_30() {
        let result = decode("J00230123C");
        assert!(matches!(result, Err(NidError::InvalidDate(_))));
    }

    #[test]
    fn decode_female_albanian() {
        let nid = make_nid(b"J05115001");
        let info = decode(&nid).unwrap();
        assert_eq!(info.sex, Sex::Female);
        assert!(info.is_national);
        assert_eq!(info.birthday.month, 1);
        assert_eq!(info.birthday.day, 15);
    }

    #[test]
    fn decode_male_foreigner() {
        let nid = make_nid(b"J03101001");
        let info = decode(&nid).unwrap();
        assert_eq!(info.sex, Sex::Male);
        assert!(!info.is_national);
    }

    #[test]
    fn decode_female_foreigner() {
        let nid = make_nid(b"J08101001");
        let info = decode(&nid).unwrap();
        assert_eq!(info.sex, Sex::Female);
        assert!(!info.is_national);
    }

    #[test]
    fn error_invalid_month_code() {
        let nid = make_nid(b"J01301001");
        let result = decode(&nid);
        assert!(matches!(result, Err(NidError::Format(_))));
    }

    #[test]
    fn leap_year_feb_29() {
        let nid = make_nid(b"K00229001");
        let info = decode(&nid).unwrap();
        assert_eq!(info.birthday.year, 2000);
        assert_eq!(info.birthday.month, 2);
        assert_eq!(info.birthday.day, 29);
    }

    #[test]
    fn non_leap_year_feb_29_fails() {
        let nid = make_nid(b"J90229001");
        let result = decode(&nid);
        assert!(matches!(result, Err(NidError::InvalidDate(_))));
    }

    #[test]
    fn sex_display() {
        assert_eq!(Sex::Male.to_string(), "M");
        assert_eq!(Sex::Female.to_string(), "F");
    }

    #[test]
    fn date_display() {
        let d = Date {
            year: 1990,
            month: 1,
            day: 1,
        };
        assert_eq!(d.to_string(), "1990-01-01");
    }

    #[test]
    fn error_display() {
        assert_eq!(
            NidError::Format(FormatKind::InvalidLength).to_string(),
            "format error: NID must be exactly 10 characters"
        );
        assert_eq!(NidError::Checksum.to_string(), "checksum validation failed");
        assert_eq!(
            NidError::InvalidDate(DateKind::MonthOutOfRange { month: 13 }).to_string(),
            "invalid date: month 13 out of range"
        );
        assert_eq!(
            NidError::InvalidDate(DateKind::DayOutOfRange {
                year: 1990,
                month: 2,
                day: 30
            })
            .to_string(),
            "invalid date: day 30 is out of range for 1990-02"
        );
    }
}
