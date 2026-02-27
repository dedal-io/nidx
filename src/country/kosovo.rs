//! Validate Kosovo personal numbers.
//!
//! Kosovo personal numbers are 10-digit numeric strings assigned by the
//! Civil Registration Agency. The first 9 digits are an opaque payload and
//! the 10th digit is a check digit.
//!
//! # Check digit algorithm
//!
//! The weights `[4, 3, 2, 7, 6, 5, 4, 3, 2]` are applied to digits 1–9.
//!
//! ```text
//! check = 11 - (sum mod 11)
//! if check == 10 → use 0
//! if check == 11 → use 0
//! ```
//!
//! Numbers starting with `'9'` bypass check digit validation.
//!
//! # Examples
//!
//! ```
//! assert!(nidx::kosovo::is_valid("1234567892"));
//! assert!(!nidx::kosovo::is_valid("invalid"));
//! ```

use std::fmt;

const WEIGHTS: [u8; 9] = [4, 3, 2, 7, 6, 5, 4, 3, 2];

/// Errors that can occur when validating a Kosovo personal number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum NidError {
    /// The input string has an invalid format (wrong length, non-digit characters).
    Format(FormatKind),
    /// The check digit does not match the computed value.
    Checksum,
}

impl fmt::Display for NidError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NidError::Format(kind) => write!(f, "format error: {kind}"),
            NidError::Checksum => write!(f, "checksum validation failed"),
        }
    }
}

impl std::error::Error for NidError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            NidError::Format(kind) => Some(kind),
            _ => None,
        }
    }
}

/// Specific reason a personal number was rejected due to formatting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum FormatKind {
    /// Input is not exactly 10 characters.
    InvalidLength,
    /// Not all characters are ASCII digits.
    NonDigitCharacter,
}

impl fmt::Display for FormatKind {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormatKind::InvalidLength => {
                write!(f, "personal number must be exactly 10 digits")
            }
            FormatKind::NonDigitCharacter => write!(f, "all characters must be ASCII digits"),
        }
    }
}

impl std::error::Error for FormatKind {}

/// Validate a Kosovo personal number string, returning a [`NidError`] on failure.
///
/// # Errors
///
/// Returns [`NidError::Format`] if the input has wrong length or non-digit characters.
/// Returns [`NidError::Checksum`] if the check digit does not match.
///
/// # Examples
///
/// ```
/// assert!(nidx::kosovo::validate("1234567892").is_ok());
/// assert!(nidx::kosovo::validate("invalid").is_err());
/// ```
pub fn validate(nid: &str) -> Result<(), NidError> {
    let bytes = nid.as_bytes();
    if bytes.len() != 10 {
        return Err(NidError::Format(FormatKind::InvalidLength));
    }
    if !bytes.iter().all(|b| b.is_ascii_digit()) {
        return Err(NidError::Format(FormatKind::NonDigitCharacter));
    }

    // Numbers starting with '9' bypass check digit validation.
    if bytes[0] == b'9' {
        return Ok(());
    }

    let sum: u16 = bytes[..9]
        .iter()
        .zip(WEIGHTS.iter())
        .map(|(&b, &w)| (b - b'0') as u16 * w as u16)
        .sum();

    let mut check = 11 - (sum % 11);
    if check >= 10 {
        check = 0;
    }

    let expected = (bytes[9] - b'0') as u16;
    if check != expected {
        return Err(NidError::Checksum);
    }

    Ok(())
}

/// Check whether a Kosovo personal number string is valid.
///
/// This is a convenience wrapper around [`validate`] that returns a simple boolean.
///
/// # Examples
///
/// ```
/// assert!(nidx::kosovo::is_valid("1234567892"));
/// assert!(!nidx::kosovo::is_valid("invalid"));
/// ```
#[inline]
#[must_use]
pub fn is_valid(nid: &str) -> bool {
    validate(nid).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_NID: &str = "1234567892";

    /// Builds a valid 10-digit personal number by appending the computed check digit.
    fn make_nid(partial: &[u8; 9]) -> String {
        let sum: u16 = partial
            .iter()
            .zip(WEIGHTS.iter())
            .map(|(&b, &w)| (b - b'0') as u16 * w as u16)
            .sum();
        let mut check = 11 - (sum % 11);
        if check >= 10 {
            check = 0;
        }
        format!("{}{}", std::str::from_utf8(partial).unwrap(), check)
    }

    #[test]
    fn validate_valid() {
        assert!(validate(VALID_NID).is_ok());
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
        assert!(!is_valid("ABCDEFGHIJ"));
    }

    #[test]
    fn is_valid_false_bad_checksum() {
        // Flip the last digit
        let last = VALID_NID.as_bytes()[9];
        let bad_last = if last == b'0' { b'1' } else { b'0' };
        let mut bad = VALID_NID.to_string();
        bad.replace_range(9..10, std::str::from_utf8(&[bad_last]).unwrap());
        assert!(!is_valid(&bad));
    }

    #[test]
    fn error_too_short() {
        let err = validate("12345").unwrap_err();
        assert_eq!(err, NidError::Format(FormatKind::InvalidLength));
    }

    #[test]
    fn error_too_long() {
        let err = validate("12345678901").unwrap_err();
        assert_eq!(err, NidError::Format(FormatKind::InvalidLength));
    }

    #[test]
    fn error_non_digit() {
        let err = validate("12345678A0").unwrap_err();
        assert_eq!(err, NidError::Format(FormatKind::NonDigitCharacter));
    }

    #[test]
    fn error_checksum_mismatch() {
        let err = validate("1234567890").unwrap_err();
        assert_eq!(err, NidError::Checksum);
    }

    #[test]
    fn make_nid_produces_valid_numbers() {
        let nid = make_nid(b"123456789");
        assert!(is_valid(&nid));
        assert_eq!(nid, VALID_NID);
    }

    #[test]
    fn verify_example_calculation() {
        // 1×4 + 2×3 + 3×2 + 4×7 + 5×6 + 6×5 + 7×4 + 8×3 + 9×2 = 174
        // 174 mod 11 = 9
        // 11 - 9 = 2
        // check digit = 2
        let sum: u16 = [1, 2, 3, 4, 5, 6, 7, 8, 9]
            .iter()
            .zip(WEIGHTS.iter())
            .map(|(&d, &w)| d as u16 * w as u16)
            .sum();
        assert_eq!(sum, 174);
        assert_eq!(sum % 11, 9);
        assert_eq!(11 - (sum % 11), 2);
    }

    #[test]
    fn check_digit_10_maps_to_zero() {
        // Find a 9-digit prefix where 11 - (sum % 11) == 10, i.e. sum % 11 == 1.
        // 1×4 + 1×3 + 1×2 + 1×7 + 1×6 + 1×5 + 1×4 + 1×3 + 0×2 = 34
        // 34 % 11 = 1 → 11 - 1 = 10 → check = 0
        let nid = make_nid(b"111111110");
        assert_eq!(&nid[9..], "0");
        assert!(is_valid(&nid));
    }

    #[test]
    fn prefix_9_bypasses_checksum() {
        // 9000000001 has a wrong check digit, but starts with '9' so it passes.
        assert!(is_valid("9000000001"));
        assert!(validate("9000000001").is_ok());
    }

    #[test]
    fn prefix_9_still_requires_format() {
        assert!(!is_valid("9short"));
        assert!(!is_valid("9ABCDEFGH"));
    }

    #[test]
    fn various_valid_numbers() {
        let nid = make_nid(b"000000000");
        assert!(is_valid(&nid));

        let nid = make_nid(b"123456789");
        assert!(is_valid(&nid));

        let nid = make_nid(b"200000000");
        assert!(is_valid(&nid));
    }

    #[test]
    fn error_display() {
        assert_eq!(
            NidError::Format(FormatKind::InvalidLength).to_string(),
            "format error: personal number must be exactly 10 digits"
        );
        assert_eq!(
            NidError::Format(FormatKind::NonDigitCharacter).to_string(),
            "format error: all characters must be ASCII digits"
        );
        assert_eq!(NidError::Checksum.to_string(), "checksum validation failed");
    }
}
