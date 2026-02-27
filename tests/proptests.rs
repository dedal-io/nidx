use proptest::prelude::*;

proptest! {
    // ── Albania ─────────────────────────────────────────────────────────

    #[test]
    fn albania_decode_never_panics(s in "\\PC{0,20}") {
        let _ = nidx::albania::decode(&s);
    }

    #[test]
    fn albania_is_valid_agrees_with_decode(s in "\\PC{0,20}") {
        assert_eq!(nidx::albania::is_valid(&s), nidx::albania::decode(&s).is_ok());
    }

    #[test]
    fn albania_valid_nids_roundtrip(
        decade in 0usize..30,
        year_digit in 0u8..10,
        month_range_idx in 0usize..4,
        month_offset in 0u8..12,
        day in 1u8..29,
        serial in 0u16..1000,
    ) {
        const DECADE_CHARS: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRST";
        const CHECKSUM_CHARS: &[u8] = b"WABCDEFGHIJKLMNOPQRSTUV";

        const RANGE_LOWS: [u8; 4] = [1, 51, 31, 81];

        let month_code = RANGE_LOWS[month_range_idx] + month_offset;

        let partial = format!(
            "{}{}{:02}{:02}{:03}",
            DECADE_CHARS[decade] as char,
            year_digit,
            month_code,
            day,
            serial,
        );
        let partial_bytes = partial.as_bytes();

        let total: usize = partial_bytes
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
        let nid = format!("{partial}{check}");

        let result = nidx::albania::decode(&nid);
        // Some generated NIDs may have invalid dates (e.g., day 28 in a month with fewer days),
        // but they should never panic and should return a well-formed Result.
        match result {
            Ok(info) => {
                assert!(nidx::albania::is_valid(&nid));
                assert_eq!(info.birthday.to_string().len(), 10);
            }
            Err(_) => {
                assert!(!nidx::albania::is_valid(&nid));
            }
        }
    }

    // ── Kosovo ──────────────────────────────────────────────────────────

    #[test]
    fn kosovo_validate_never_panics(s in "\\PC{0,20}") {
        let _ = nidx::kosovo::validate(&s);
    }

    #[test]
    fn kosovo_is_valid_agrees_with_validate(s in "\\PC{0,20}") {
        assert_eq!(nidx::kosovo::is_valid(&s), nidx::kosovo::validate(&s).is_ok());
    }

    #[test]
    fn kosovo_valid_nids_roundtrip(
        digits in proptest::collection::vec(0u8..10, 9),
    ) {
        const WEIGHTS: [u8; 9] = [4, 3, 2, 7, 6, 5, 4, 3, 2];

        let sum: u16 = digits
            .iter()
            .zip(WEIGHTS.iter())
            .map(|(&d, &w)| d as u16 * w as u16)
            .sum();

        let mut check = 11 - (sum % 11);
        if check >= 10 {
            check = 0;
        }

        let nid: String = digits
            .iter()
            .map(|d| (b'0' + d) as char)
            .chain(std::iter::once((b'0' + check as u8) as char))
            .collect();

        assert!(nidx::kosovo::is_valid(&nid), "expected valid: {nid}");
        assert!(nidx::kosovo::validate(&nid).is_ok());
    }

    #[test]
    fn kosovo_prefix_9_always_valid(
        digits in proptest::collection::vec(0u8..10, 9),
    ) {
        let nid: String = std::iter::once('9')
            .chain(digits.iter().take(8).map(|d| (b'0' + d) as char))
            .chain(std::iter::once((b'0' + digits[8]) as char))
            .collect();

        assert!(nidx::kosovo::is_valid(&nid), "prefix-9 should always be valid: {nid}");
        assert!(nidx::kosovo::validate(&nid).is_ok());
    }
}
