use nidx_wasm::{Albania, Kosovo};
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn albania_decode_valid_nid() {
    let info = Albania::decode("J00101999W").unwrap();
    assert_eq!(info.country(), "albania");
    assert_eq!(info.birthday(), "1990-01-01");
    assert_eq!(info.sex(), "M");
    assert!(info.is_national());
    assert_eq!(info.year(), 1990);
    assert_eq!(info.month(), 1);
    assert_eq!(info.day(), 1);
}

#[wasm_bindgen_test]
fn albania_decode_female_albanian() {
    let info = Albania::decode("J05115999K").unwrap();
    assert_eq!(info.birthday(), "1990-01-15");
    assert_eq!(info.sex(), "F");
    assert!(info.is_national());
}

#[wasm_bindgen_test]
fn albania_decode_male_foreigner() {
    let info = Albania::decode("J03101999F").unwrap();
    assert_eq!(info.birthday(), "1990-01-01");
    assert_eq!(info.sex(), "M");
    assert!(!info.is_national());
}

#[wasm_bindgen_test]
fn albania_decode_female_foreigner() {
    let info = Albania::decode("J08101999P").unwrap();
    assert_eq!(info.birthday(), "1990-01-01");
    assert_eq!(info.sex(), "F");
    assert!(!info.is_national());
}

#[wasm_bindgen_test]
fn albania_decode_case_insensitive() {
    let info = Albania::decode("j00101999w").unwrap();
    assert_eq!(info.birthday(), "1990-01-01");
}

#[wasm_bindgen_test]
fn albania_decode_invalid_returns_format_error() {
    let err = Albania::decode("invalid").unwrap_err();
    let msg = format!("{err:?}");
    assert!(
        msg.contains("[FORMAT]"),
        "expected FORMAT error, got: {msg}"
    );
}

#[wasm_bindgen_test]
fn albania_decode_empty_returns_format_error() {
    let err = Albania::decode("").unwrap_err();
    let msg = format!("{err:?}");
    assert!(
        msg.contains("[FORMAT]"),
        "expected FORMAT error, got: {msg}"
    );
}

#[wasm_bindgen_test]
fn albania_is_valid_accepts_valid() {
    assert!(Albania::is_valid("J00101999W"));
}

#[wasm_bindgen_test]
fn albania_is_valid_rejects_invalid() {
    assert!(!Albania::is_valid("invalid"));
    assert!(!Albania::is_valid(""));
    assert!(!Albania::is_valid("J00101999A"));
}

// ── Kosovo ──────────────────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn kosovo_validate_valid() {
    Kosovo::validate("1234567892").unwrap();
}

#[wasm_bindgen_test]
fn kosovo_validate_prefix_9_bypasses_checksum() {
    Kosovo::validate("9000000001").unwrap();
}

#[wasm_bindgen_test]
fn kosovo_validate_invalid_returns_format_error() {
    let err = Kosovo::validate("invalid").unwrap_err();
    let msg = format!("{err:?}");
    assert!(
        msg.contains("[FORMAT]"),
        "expected FORMAT error, got: {msg}"
    );
}

#[wasm_bindgen_test]
fn kosovo_validate_empty_returns_format_error() {
    let err = Kosovo::validate("").unwrap_err();
    let msg = format!("{err:?}");
    assert!(
        msg.contains("[FORMAT]"),
        "expected FORMAT error, got: {msg}"
    );
}

#[wasm_bindgen_test]
fn kosovo_validate_bad_checksum_returns_checksum_error() {
    let err = Kosovo::validate("1234567890").unwrap_err();
    let msg = format!("{err:?}");
    assert!(
        msg.contains("[CHECKSUM]"),
        "expected CHECKSUM error, got: {msg}"
    );
}

#[wasm_bindgen_test]
fn kosovo_is_valid_accepts_valid() {
    assert!(Kosovo::is_valid("1234567892"));
}

#[wasm_bindgen_test]
fn kosovo_is_valid_prefix_9_accepted() {
    assert!(Kosovo::is_valid("9000000001"));
}

#[wasm_bindgen_test]
fn kosovo_is_valid_rejects_invalid() {
    assert!(!Kosovo::is_valid("invalid"));
    assert!(!Kosovo::is_valid(""));
    assert!(!Kosovo::is_valid("1234567890"));
}
