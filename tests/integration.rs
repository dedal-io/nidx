use nidx::albania::{FormatKind, NidError, NidInfo};
use nidx::{Date, Sex};

#[test]
fn public_api_decode_returns_expected_info() {
    let info = nidx::albania::decode("J00101999W").unwrap();
    assert_eq!(
        info,
        NidInfo {
            birthday: Date {
                year: 1990,
                month: 1,
                day: 1
            },
            sex: Sex::Male,
            is_national: true,
        }
    );
}

#[test]
fn public_api_is_valid_accepts_valid_nid() {
    assert!(nidx::albania::is_valid("J00101999W"));
}

#[test]
fn public_api_is_valid_rejects_invalid_nid() {
    assert!(!nidx::albania::is_valid(""));
    assert!(!nidx::albania::is_valid("0000000000"));
    assert!(!nidx::albania::is_valid("ZZZZZZZZZZ"));
}

#[test]
fn decode_is_case_insensitive() {
    let upper = nidx::albania::decode("J00101999W").unwrap();
    let lower = nidx::albania::decode("j00101999w").unwrap();
    let mixed = nidx::albania::decode("j00101999W").unwrap();
    assert_eq!(upper, lower);
    assert_eq!(upper, mixed);
}

#[test]
fn error_type_implements_std_error() {
    fn assert_error<T: std::error::Error>() {}
    assert_error::<NidError>();
}

#[test]
fn nid_info_is_copy() {
    let info = nidx::albania::decode("J00101999W").unwrap();
    let copy = info;
    assert_eq!(info, copy);
}

#[test]
fn date_display_is_iso8601() {
    let info = nidx::albania::decode("J00101999W").unwrap();
    assert_eq!(info.birthday.to_string(), "1990-01-01");
}

#[test]
fn sex_display_single_char() {
    assert_eq!(Sex::Male.to_string(), "M");
    assert_eq!(Sex::Female.to_string(), "F");
}

#[test]
fn error_format_variant() {
    let err = nidx::albania::decode("short").unwrap_err();
    assert_eq!(err, NidError::Format(FormatKind::InvalidLength));
    assert!(err.to_string().contains("format error"));
}

#[test]
fn error_checksum_variant() {
    let err = nidx::albania::decode("J00101999A").unwrap_err();
    assert!(matches!(err, NidError::Checksum));
}

/// Computes a valid 10-char NID string from 9 content bytes.
fn make_nid(partial: &[u8; 9]) -> String {
    const CHECKSUM_CHARS: &[u8] = b"WABCDEFGHIJKLMNOPQRSTUV";
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
fn all_sex_and_national_status_combinations() {
    // Male Albanian: month code 01-12
    let info = nidx::albania::decode("J00101999W").unwrap();
    assert_eq!(info.sex, Sex::Male);
    assert!(info.is_national);

    // Female Albanian: month code 51-62
    let nid = make_nid(b"J05115001");
    let info = nidx::albania::decode(&nid).unwrap();
    assert_eq!(info.sex, Sex::Female);
    assert!(info.is_national);

    // Male foreigner: month code 31-42
    let nid = make_nid(b"J03101001");
    let info = nidx::albania::decode(&nid).unwrap();
    assert_eq!(info.sex, Sex::Male);
    assert!(!info.is_national);

    // Female foreigner: month code 81-92
    let nid = make_nid(b"J08101001");
    let info = nidx::albania::decode(&nid).unwrap();
    assert_eq!(info.sex, Sex::Female);
    assert!(!info.is_national);
}

// ── Kosovo ──────────────────────────────────────────────────────────────────

#[test]
fn kosovo_is_valid_accepts_valid() {
    assert!(nidx::kosovo::is_valid("1234567892"));
}

#[test]
fn kosovo_is_valid_rejects_invalid() {
    assert!(!nidx::kosovo::is_valid(""));
    assert!(!nidx::kosovo::is_valid("1234567890"));
    assert!(!nidx::kosovo::is_valid("ABCDEFGHIJ"));
}

#[test]
fn kosovo_prefix_9_bypasses_checksum() {
    assert!(nidx::kosovo::is_valid("9000000001"));
    assert!(nidx::kosovo::validate("9000000001").is_ok());
}

#[test]
fn kosovo_validate_returns_format_error_for_wrong_length() {
    let err = nidx::kosovo::validate("12345").unwrap_err();
    assert_eq!(
        err,
        nidx::kosovo::NidError::Format(nidx::kosovo::FormatKind::InvalidLength)
    );
    assert!(err.to_string().contains("format error"));
}

#[test]
fn kosovo_validate_returns_checksum_error() {
    let err = nidx::kosovo::validate("1234567890").unwrap_err();
    assert!(matches!(err, nidx::kosovo::NidError::Checksum));
}

#[test]
fn kosovo_error_type_implements_std_error() {
    fn assert_error<T: std::error::Error>() {}
    assert_error::<nidx::kosovo::NidError>();
}
