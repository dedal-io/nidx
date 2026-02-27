#![cfg(feature = "serde")]

use nidx::{Date, Sex};

#[test]
fn date_roundtrip_json() {
    let date = Date {
        year: 1990,
        month: 1,
        day: 1,
    };
    let json = serde_json::to_string(&date).unwrap();
    let back: Date = serde_json::from_str(&json).unwrap();
    assert_eq!(date, back);
}

#[test]
fn sex_roundtrip_json() {
    let json = serde_json::to_string(&Sex::Female).unwrap();
    let back: Sex = serde_json::from_str(&json).unwrap();
    assert_eq!(Sex::Female, back);
}

#[test]
fn nid_info_roundtrip_json() {
    let info = nidx::albania::decode("J00101999W").unwrap();
    let json = serde_json::to_string(&info).unwrap();
    let back: nidx::albania::NidInfo = serde_json::from_str(&json).unwrap();
    assert_eq!(info, back);
}
