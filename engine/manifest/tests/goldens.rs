//! Golden and conformance tests for `friedman-manifest`.
//!
//! Every expected value here is either (a) an ADR-0001 D6 golden — hand-computed over fixed bytes,
//! independently re-derived by `lab/verify_adr0001.py` — or (b) a test vector printed in RFC 8785
//! itself (S-0004: §3.2.2 sample input, §3.2.4 expected bytes, Appendix B number table). If this
//! crate ever disagrees with them, the crate is wrong until proven otherwise.

use std::collections::BTreeMap;

use friedman_manifest::*;
use serde_json::{Value, json};

// ---- ADR-0001 D6 goldens -------------------------------------------------------------------

const D6_CORPUS: &[u8] = b"ba ka ba\nka\n";
const D6_CORPUS_HEX: &str = "62 61 20 6b 61 20 62 61 0a 6b 61 0a";
const D6_CONTENT_HASH: &str = "8d106b97f8396631fa7e3f0a597622291ec6c773bd1628a82d44fc0c38ec75fd";
const D6_CONFIG_AUTHORED: &str =
    r#"{"order": 1, "transition_bias": 0.75, "alphabet": ["ba", "ka"]}"#;
const D6_CONFIG_JCS: &[u8] = br#"{"alphabet":["ba","ka"],"order":1,"transition_bias":0.75}"#;
const D6_CONFIG_HASH: &str = "22cdc154338b0b3c5aa84e767452853343cdebf204e84595aaa2c7feed0cdc33";
const D6_CORPUS_ID: &str = "markov-v1.0.0-22cdc154338b-s7";

fn hex_to_bytes(s: &str) -> Vec<u8> {
    s.split_whitespace()
        .map(|h| u8::from_str_radix(h, 16).unwrap())
        .collect()
}

#[test]
fn d6_corpus_bytes_are_the_stated_bytes() {
    assert_eq!(hex_to_bytes(D6_CORPUS_HEX), D6_CORPUS);
    assert_eq!(D6_CORPUS.len(), 12);
}

#[test]
fn d6_content_hash() {
    assert_eq!(content_hash(D6_CORPUS), D6_CONTENT_HASH);
}

#[test]
fn d6_config_jcs_bytes_and_hash() {
    let config: Value = serde_json::from_str(D6_CONFIG_AUTHORED).unwrap();
    let jcs = jcs_bytes(&config).unwrap();
    assert_eq!(jcs, D6_CONFIG_JCS);
    assert_eq!(jcs.len(), 57);
    assert_eq!(config_hash(&config).unwrap(), D6_CONFIG_HASH);
}

#[test]
fn d6_corpus_id_derivation() {
    assert_eq!(
        corpus_id("markov", "1.0.0", D6_CONFIG_HASH, 7),
        D6_CORPUS_ID
    );
}

#[test]
fn d6_corpus_stats() {
    let stats = parse_corpus(D6_CORPUS).unwrap();
    assert_eq!(
        stats,
        CorpusStats {
            n_documents: 2,
            n_tokens: 4,
            n_types: 2,
            doc_length: DocLength {
                min: 1,
                max: 3,
                mean: 2.0
            },
        }
    );
}

/// The illustrative manifest printed in ADR-0001 D6 must parse, self-check against the D6 corpus
/// bytes, and survive a disk round-trip unchanged. The ADR is read from the repo so the two can
/// never drift apart silently.
#[test]
fn d6_example_manifest_from_adr_text() {
    let adr = include_str!("../../../PROGRAM/DECISIONS/ADR-0001.md");
    let start = adr.find("```json\n").expect("ADR-0001 has a ```json block") + "```json\n".len();
    let end = start + adr[start..].find("\n```").expect("json block is closed");
    let block = &adr[start..end];

    let manifest = from_json_str(block).expect("ADR-0001 example manifest parses");
    assert_eq!(manifest.manifest_schema, MANIFEST_SCHEMA);
    assert_eq!(manifest.corpus_id, D6_CORPUS_ID);
    assert_eq!(manifest.config_hash, D6_CONFIG_HASH);
    assert_eq!(manifest.corpus.content_hash, D6_CONTENT_HASH);
    assert_eq!(manifest.seed, 7);
    manifest
        .check(D6_CORPUS)
        .unwrap_or_else(|problems| panic!("ADR-0001 example fails its own rules: {problems:#?}"));

    // D1 on-disk form: parses back to the same manifest, ends with exactly one LF, no CR.
    let disk = to_disk_json(&manifest).unwrap();
    assert!(disk.ends_with('\n') && !disk.ends_with("\n\n"));
    assert!(!disk.contains('\r'));
    assert_eq!(from_json_str(&disk).unwrap(), manifest);
    // and is idempotent
    assert_eq!(to_disk_json(&from_json_str(&disk).unwrap()).unwrap(), disk);
    // and the ADR's block is itself in JCS key order at the top level (D1 presentation rule)
    let top: Vec<&str> = block
        .lines()
        .filter_map(|l| l.strip_prefix("  \"").and_then(|r| r.split('"').next()))
        .collect();
    let mut sorted = top.clone();
    sorted.sort_unstable();
    assert_eq!(
        top, sorted,
        "ADR-0001 example manifest top-level keys are not in JCS order"
    );
}

// ---- RFC 8785 conformance (S-0004) ---------------------------------------------------------

/// §3.2.2 sample input, exactly as printed in the RFC (escapes are JSON escapes, not Rust ones).
const RFC_SAMPLE_INPUT: &str = r#"{
       "numbers": [333333333.33333329, 1E30, 4.50,
                   2e-3, 0.000000000000000000000000001],
       "string": "\u20ac$\u000F\u000aA'\u0042\u0022\u005c\\\"\/",
       "literals": [null, true, false]
     }"#;

/// §3.2.4 expected canonical bytes, exactly as printed in the RFC.
const RFC_SAMPLE_OUTPUT_HEX: &str = "7b 22 6c 69 74 65 72 61 6c 73 22 3a 5b 6e 75 6c 6c 2c 74 72 \
     75 65 2c 66 61 6c 73 65 5d 2c 22 6e 75 6d 62 65 72 73 22 3a \
     5b 33 33 33 33 33 33 33 33 33 2e 33 33 33 33 33 33 33 2c 31 \
     65 2b 33 30 2c 34 2e 35 2c 30 2e 30 30 32 2c 31 65 2d 32 37 \
     5d 2c 22 73 74 72 69 6e 67 22 3a 22 e2 82 ac 24 5c 75 30 30 \
     30 66 5c 6e 41 27 42 5c 22 5c 5c 5c 5c 5c 22 2f 22 7d";

#[test]
fn rfc8785_section_3_2_sample_canonicalizes_to_the_printed_bytes() {
    let value: Value = serde_json::from_str(RFC_SAMPLE_INPUT).unwrap();
    let got = jcs_bytes(&value).unwrap();
    let expected = hex_to_bytes(RFC_SAMPLE_OUTPUT_HEX);
    assert_eq!(
        got,
        expected,
        "\n got: {}\nwant: {}",
        String::from_utf8_lossy(&got),
        String::from_utf8_lossy(&expected)
    );
}

/// Appendix B, Table 1 — every finite row. NaN and Infinity rows are covered separately.
const RFC_APPENDIX_B: &[(u64, &str)] = &[
    (0x0000000000000000, "0"),
    (0x8000000000000000, "0"),
    (0x0000000000000001, "5e-324"),
    (0x8000000000000001, "-5e-324"),
    (0x7fefffffffffffff, "1.7976931348623157e+308"),
    (0xffefffffffffffff, "-1.7976931348623157e+308"),
    (0x4340000000000000, "9007199254740992"),
    (0xc340000000000000, "-9007199254740992"),
    (0x4430000000000000, "295147905179352830000"),
    (0x44b52d02c7e14af5, "9.999999999999997e+22"),
    (0x44b52d02c7e14af6, "1e+23"),
    (0x44b52d02c7e14af7, "1.0000000000000001e+23"),
    (0x444b1ae4d6e2ef4e, "999999999999999700000"),
    (0x444b1ae4d6e2ef4f, "999999999999999900000"),
    (0x444b1ae4d6e2ef50, "1e+21"),
    (0x3eb0c6f7a0b5ed8c, "9.999999999999997e-7"),
    (0x3eb0c6f7a0b5ed8d, "0.000001"),
    (0x41b3de4355555553, "333333333.3333332"),
    (0x41b3de4355555554, "333333333.33333325"),
    (0x41b3de4355555555, "333333333.3333333"),
    (0x41b3de4355555556, "333333333.3333334"),
    (0x41b3de4355555557, "333333333.33333343"),
    (0xbecbf647612f3696, "-0.0000033333333333333333"),
    (0x43143ff3c1cb0959, "1424953923781206.2"),
];

#[test]
fn rfc8785_appendix_b_number_serialization() {
    for &(bits, expected) in RFC_APPENDIX_B {
        let x = f64::from_bits(bits);
        // as a bare f64
        let got = serde_json_canonicalizer::to_string(&x).unwrap();
        assert_eq!(got, expected, "bits {bits:016x} as f64");
        // and inside a JSON value, the path config_hash actually takes
        let v = Value::from(x);
        let got = String::from_utf8(jcs_bytes(&v).unwrap()).unwrap();
        assert_eq!(got, expected, "bits {bits:016x} as Value");
    }
}

#[test]
fn rfc8785_nan_and_infinity_are_errors_not_output() {
    // A serde_json::Value cannot even hold them (I-JSON), so config_hash can never see them...
    assert!(serde_json::Number::from_f64(f64::NAN).is_none());
    assert!(serde_json::Number::from_f64(f64::INFINITY).is_none());
    assert!(serde_json::Number::from_f64(f64::NEG_INFINITY).is_none());
    // ...and the canonicalizer refuses them as bare floats (RFC §3.2.2.3 "MUST ... terminate").
    assert!(serde_json_canonicalizer::to_string(&f64::NAN).is_err());
    assert!(serde_json_canonicalizer::to_string(&f64::INFINITY).is_err());
}

#[test]
fn rfc8785_property_sorting_is_by_utf16_code_units() {
    // §3.2.3 "In plain English" ordering, plus a non-BMP key (must sort by UTF-16 units, where a
    // surrogate pair 0xD83D.. sorts BEFORE U+FF5E even though its code point is larger).
    let v = json!({"\u{ff5e}": 1, "\u{1f600}": 2, "\u{20ac}": 3, "a": 4, "": 5, "\r": 6, "1": 7, "A": 8});
    let got = String::from_utf8(jcs_bytes(&v).unwrap()).unwrap();
    assert_eq!(
        got,
        "{\"\":5,\"\\r\":6,\"1\":7,\"A\":8,\"a\":4,\"\u{20ac}\":3,\"\u{1f600}\":2,\"\u{ff5e}\":1}"
    );
}

// ---- ADR-0001 D3 corpus format -----------------------------------------------------------------

#[test]
fn d3_rejects_each_violation() {
    use CorpusFormatError::*;
    assert_eq!(parse_corpus(b"\xff\n"), Err(NotUtf8));
    assert_eq!(parse_corpus("\u{FEFF}ba\n".as_bytes()), Err(ByteOrderMark));
    assert_eq!(
        parse_corpus(b"ba ka\r\n"),
        Err(CarriageReturn { offset: 5 })
    );
    assert_eq!(parse_corpus(b"ba\tka\n"), Err(Tab { offset: 2 }));
    assert_eq!(parse_corpus(b""), Err(NoDocuments));
    assert_eq!(parse_corpus(b"ba ka"), Err(MissingFinalNewline));
    assert_eq!(parse_corpus(b"ba\n\nka\n"), Err(EmptyDocument { line: 1 }));
    assert_eq!(parse_corpus(b"\n"), Err(EmptyDocument { line: 0 }));
    assert_eq!(parse_corpus(b"ba\n ka\n"), Err(LeadingSpace { line: 1 }));
    assert_eq!(parse_corpus(b"ba \n"), Err(TrailingSpace { line: 0 }));
    assert_eq!(parse_corpus(b"ba  ka\n"), Err(DoubleSpace { line: 0 }));
}

#[test]
fn d3_accepts_non_ascii_tokens_and_counts_types_exactly() {
    let stats = parse_corpus("𐘀 𐘁 𐘀\n𐘂\n𐘂 𐘂\n".as_bytes()).unwrap();
    assert_eq!(stats.n_documents, 3);
    assert_eq!(stats.n_tokens, 6);
    assert_eq!(stats.n_types, 3);
    assert_eq!(
        stats.doc_length,
        DocLength {
            min: 1,
            max: 3,
            mean: 2.0
        }
    );
}

// ---- ADR-0001 D4 fidelity rule --------------------------------------------------------------------

fn cmd(value: Value, tolerance: Tolerance) -> Commanded {
    Commanded { value, tolerance }
}

fn ach(value: Option<Value>) -> Achieved {
    Achieved {
        value,
        estimator: "test".into(),
        ci95: None,
        reason: None,
    }
}

#[test]
fn d4_verdict_rule() {
    let abs = |v| cmd(json!(-1.0), Tolerance::Abs { abs: v });
    // The boundary is inclusive (D4 "≤"), evaluated in IEEE 754 doubles: -1.125 - -1.0 is exactly
    // -0.125, so this is a true boundary case. (-1.1 - -1.0 is NOT exactly 0.1 — it is
    // 0.10000000000000009 — and therefore FAILs at abs 0.1; ADR-0002 records this.)
    assert_eq!(
        statistic_verdict(&abs(0.125), Some(&ach(Some(json!(-1.125))))),
        Verdict::Pass
    );
    assert_eq!(
        statistic_verdict(&abs(0.1), Some(&ach(Some(json!(-1.1))))),
        Verdict::Fail
    );
    assert_eq!(
        statistic_verdict(&abs(0.1), Some(&ach(Some(json!(-1.11))))),
        Verdict::Fail
    );
    assert_eq!(
        statistic_verdict(&abs(0.1), Some(&ach(None))),
        Verdict::Fail
    ); // null → FAIL
    assert_eq!(statistic_verdict(&abs(0.1), None), Verdict::Fail); // unmeasured → FAIL
    assert_eq!(
        statistic_verdict(&abs(0.1), Some(&ach(Some(json!("x"))))),
        Verdict::Fail
    ); // non-numeric

    // Same IEEE 754 point for `rel`: 0.125 * |-2.0| = 0.25 and -2.25 - -2.0 = -0.25 exactly.
    // (-2.2 - -2.0 is 0.20000000000000018, so rel 0.1 would FAIL there.)
    let rel = cmd(json!(-2.0), Tolerance::Rel { rel: 0.125 });
    assert_eq!(
        statistic_verdict(&rel, Some(&ach(Some(json!(-2.25))))),
        Verdict::Pass
    );
    assert_eq!(
        statistic_verdict(&rel, Some(&ach(Some(json!(-2.2))))),
        Verdict::Pass
    );
    assert_eq!(
        statistic_verdict(&rel, Some(&ach(Some(json!(-2.26))))),
        Verdict::Fail
    );

    let exact = cmd(json!("power_law"), Tolerance::Exact { exact: true });
    assert_eq!(
        statistic_verdict(&exact, Some(&ach(Some(json!("power_law"))))),
        Verdict::Pass
    );
    assert_eq!(
        statistic_verdict(&exact, Some(&ach(Some(json!("exponential"))))),
        Verdict::Fail
    );

    // ci95 never enters the verdict
    let wide_ci = Achieved {
        ci95: Some([-9.0, 9.0]),
        ..ach(Some(json!(-1.5)))
    };
    assert_eq!(statistic_verdict(&abs(0.1), Some(&wide_ci)), Verdict::Fail);
}

#[test]
fn d4_fidelity_block() {
    let mut commanded = BTreeMap::new();
    commanded.insert(
        "zipf_slope".to_string(),
        cmd(json!(-1.0), Tolerance::Abs { abs: 0.1 }),
    );
    commanded.insert(
        "mi_decay_class".to_string(),
        cmd(json!("power_law"), Tolerance::Exact { exact: true }),
    );
    let mut achieved = BTreeMap::new();
    achieved.insert("zipf_slope".to_string(), ach(Some(json!(-1.05))));
    achieved.insert("mi_decay_class".to_string(), ach(Some(json!("power_law"))));
    achieved.insert("entropy_rate".to_string(), ach(Some(json!(0.9)))); // extra achieved key is fine

    let f = fidelity(&commanded, &achieved);
    assert_eq!(f.verdict, Verdict::Pass);
    assert_eq!(f.per_statistic.len(), 2);

    achieved.get_mut("mi_decay_class").unwrap().value = Some(json!("exponential"));
    let f = fidelity(&commanded, &achieved);
    assert_eq!(f.verdict, Verdict::Fail);
    assert_eq!(f.per_statistic["zipf_slope"], Verdict::Pass);
    assert_eq!(f.per_statistic["mi_decay_class"], Verdict::Fail);
}

#[test]
fn d4_unknown_fields_are_rejected() {
    let adr = include_str!("../../../PROGRAM/DECISIONS/ADR-0001.md");
    let start = adr.find("```json\n").unwrap() + "```json\n".len();
    let end = start + adr[start..].find("\n```").unwrap();
    let with_extra = adr[start..end].replacen(
        "\"seed\": 7",
        "\"seed\": 7,\n  \"created\": \"2026-08-25\"",
        1,
    );
    assert!(
        from_json_str(&with_extra).is_err(),
        "a timestamp field must not be accepted (D4)"
    );
}

#[test]
fn d4_check_reports_every_discrepancy_not_just_the_first() {
    let adr = include_str!("../../../PROGRAM/DECISIONS/ADR-0001.md");
    let start = adr.find("```json\n").unwrap() + "```json\n".len();
    let end = start + adr[start..].find("\n```").unwrap();
    let mut m = from_json_str(&adr[start..end]).unwrap();
    m.seed = 8; // corpus_id no longer derives
    m.corpus.n_tokens = 5; // counts wrong
    m.fidelity.verdict = Verdict::Fail; // verdict wrong
    let problems = m.check(D6_CORPUS).unwrap_err();
    assert!(
        problems.iter().any(|p| p.starts_with("D2:")),
        "{problems:#?}"
    );
    assert!(
        problems.iter().any(|p| p.starts_with("D4: corpus counts")),
        "{problems:#?}"
    );
    assert!(
        problems.iter().any(|p| p.starts_with("D4/F5:")),
        "{problems:#?}"
    );
    assert_eq!(problems.len(), 3, "{problems:#?}");
}
