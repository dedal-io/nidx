use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn albania_decode_valid_nid() {
    let info = nidx_wasm::albania_decode("J00101999W").unwrap();
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
    let info = nidx_wasm::albania_decode("J05115999K").unwrap();
    assert_eq!(info.birthday(), "1990-01-15");
    assert_eq!(info.sex(), "F");
    assert!(info.is_national());
}

#[wasm_bindgen_test]
fn albania_decode_male_foreigner() {
    let info = nidx_wasm::albania_decode("J03101999F").unwrap();
    assert_eq!(info.birthday(), "1990-01-01");
    assert_eq!(info.sex(), "M");
    assert!(!info.is_national());
}

#[wasm_bindgen_test]
fn albania_decode_female_foreigner() {
    let info = nidx_wasm::albania_decode("J08101999P").unwrap();
    assert_eq!(info.birthday(), "1990-01-01");
    assert_eq!(info.sex(), "F");
    assert!(!info.is_national());
}

#[wasm_bindgen_test]
fn albania_decode_case_insensitive() {
    let info = nidx_wasm::albania_decode("j00101999w").unwrap();
    assert_eq!(info.birthday(), "1990-01-01");
}

#[wasm_bindgen_test]
fn albania_decode_invalid_returns_format_error() {
    let err = nidx_wasm::albania_decode("invalid").unwrap_err();
    let msg = format!("{err:?}");
    assert!(
        msg.contains("[FORMAT]"),
        "expected FORMAT error, got: {msg}"
    );
}

#[wasm_bindgen_test]
fn albania_decode_empty_returns_format_error() {
    let err = nidx_wasm::albania_decode("").unwrap_err();
    let msg = format!("{err:?}");
    assert!(
        msg.contains("[FORMAT]"),
        "expected FORMAT error, got: {msg}"
    );
}

#[wasm_bindgen_test]
fn albania_is_valid_accepts_valid() {
    assert!(nidx_wasm::albania_is_valid("J00101999W"));
}

#[wasm_bindgen_test]
fn albania_is_valid_rejects_invalid() {
    assert!(!nidx_wasm::albania_is_valid("invalid"));
    assert!(!nidx_wasm::albania_is_valid(""));
    assert!(!nidx_wasm::albania_is_valid("J00101999A"));
}
