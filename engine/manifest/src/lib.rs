//! FRIEDMAN corpus manifests — the F1 carrier, as decided in ADR-0001.
//!
//! `engine/` is the canonical tier (F10): only what this crate computes may enter CLAIMS.md as
//! MEASURED or REPRODUCED. Every rule implemented here names its ADR-0001 clause so a successor can
//! check code against decision without guessing.
//!
//! - D1: JSON restricted to the I-JSON subset; on-disk form is pretty-printed with JCS-ordered keys
//!   and a single trailing LF ([`to_disk_json`]).
//! - D2: `corpus_id` is a pure function of (generator, config, seed) ([`corpus_id`]).
//! - D3: the `friedman-tokens/1` byte format ([`parse_corpus`]).
//! - D4: the schema ([`Manifest`]) and the fidelity verdict rule ([`statistic_verdict`],
//!   [`fidelity`]).
//! - D5: hashes — SHA-256 over RFC 8785 JCS bytes for `config_hash` ([`config_hash`]), SHA-256 over
//!   the exact corpus bytes for `content_hash` ([`content_hash`]).

#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

/// ADR-0001 D4: the only schema version this crate understands.
pub const MANIFEST_SCHEMA: &str = "friedman-manifest/1";
/// ADR-0001 D3: the only corpus byte format this crate understands.
pub const CORPUS_FORMAT: &str = "friedman-tokens/1";
/// ADR-0001 D2: the corpus file name inside `corpora/<corpus_id>/`.
pub const CORPUS_FILE: &str = "corpus.txt";
/// ADR-0001 D4: the ground-truth label of every synthetic corpus, by construction (F1).
pub const LABEL_MECHANISM: &str = "mechanism";
/// ADR-0001 D4: `origin` value for everything ADR-0001 covers.
pub const ORIGIN_SYNTHETIC: &str = "synthetic";
/// ADR-0001 D4: `ground_truth.derivation` for synthetic corpora.
pub const DERIVATION_BY_CONSTRUCTION: &str = "by-construction";

/// ADR-0001 D4 — top-level manifest object. Unknown fields are rejected (I-JSON strictness).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub manifest_schema: String,
    pub corpus_id: String,
    pub origin: String,
    pub generator: Generator,
    pub engine_commit: String,
    pub seed: u64,
    pub config: Value,
    pub config_hash: String,
    pub ground_truth: GroundTruth,
    pub corpus: CorpusInfo,
    pub commanded: BTreeMap<String, Commanded>,
    pub achieved: BTreeMap<String, Achieved>,
    pub fidelity: Fidelity,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// ADR-0001 D4 — `generator`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Generator {
    pub id: String,
    pub version: String,
    pub rng: String,
}

/// ADR-0001 D4 — `ground_truth`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GroundTruth {
    pub label: String,
    pub family: String,
    pub derivation: String,
}

/// ADR-0001 D4 — `corpus`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CorpusInfo {
    pub file: String,
    pub format: String,
    pub content_hash: String,
    pub n_documents: u64,
    pub n_tokens: u64,
    pub n_types: u64,
    pub doc_length: DocLength,
}

/// ADR-0001 D4 — `corpus.doc_length`, tokens per document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DocLength {
    pub min: u64,
    pub max: u64,
    pub mean: f64,
}

/// ADR-0001 D4 — one `commanded` entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Commanded {
    pub value: Value,
    pub tolerance: Tolerance,
}

/// ADR-0001 D4 — tolerance forms. Exactly one of `abs`, `rel`, `exact`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Tolerance {
    Abs { abs: f64 },
    Rel { rel: f64 },
    Exact { exact: bool },
}

/// ADR-0001 D4 — one `achieved` entry. `value` is serialized as `null` when absent; `reason` is
/// required iff `value` is null (checked by [`Manifest::check`], not by the type).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Achieved {
    pub value: Option<Value>,
    pub estimator: String,
    pub ci95: Option<[f64; 2]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// ADR-0001 D4 — `fidelity`, the F5 verdict.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Fidelity {
    pub per_statistic: BTreeMap<String, Verdict>,
    pub verdict: Verdict,
}

/// ADR-0001 D4 — `"PASS"` / `"FAIL"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Verdict {
    Pass,
    Fail,
}

/// Lowercase-hex SHA-256 of `bytes` (ADR-0001 D5: every hash in a manifest).
pub fn sha256_hex(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

/// ADR-0001 D5: `content_hash` = SHA-256 over the exact bytes of `corpus.txt`.
pub fn content_hash(corpus_bytes: &[u8]) -> String {
    sha256_hex(corpus_bytes)
}

/// ADR-0001 D5: the RFC 8785 (JCS) canonical bytes of a JSON value.
///
/// Errors if the value cannot be canonicalized (e.g. a map with non-string keys). NaN and infinity
/// cannot be constructed as [`serde_json::Value`] numbers at all, so they cannot reach this point.
pub fn jcs_bytes(value: &Value) -> serde_json::Result<Vec<u8>> {
    serde_json_canonicalizer::to_vec(value)
}

/// ADR-0001 D5: `config_hash` = SHA-256 over JCS(`config`).
pub fn config_hash(config: &Value) -> serde_json::Result<String> {
    Ok(sha256_hex(&jcs_bytes(config)?))
}

/// ADR-0001 D2: `<generator.id>-v<generator.version>-<config_hash[0:12]>-s<seed>`.
pub fn corpus_id(
    generator_id: &str,
    generator_version: &str,
    config_hash: &str,
    seed: u64,
) -> String {
    let prefix: String = config_hash.chars().take(12).collect();
    format!("{generator_id}-v{generator_version}-{prefix}-s{seed}")
}

/// Descriptive counts of a `friedman-tokens/1` corpus (ADR-0001 D4 `corpus` fields).
#[derive(Debug, Clone, PartialEq)]
pub struct CorpusStats {
    pub n_documents: u64,
    pub n_tokens: u64,
    pub n_types: u64,
    pub doc_length: DocLength,
}

/// A violation of the `friedman-tokens/1` byte format (ADR-0001 D3). Lines are 0-based.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CorpusFormatError {
    NotUtf8,
    ByteOrderMark,
    CarriageReturn { offset: usize },
    Tab { offset: usize },
    MissingFinalNewline,
    NoDocuments,
    EmptyDocument { line: usize },
    LeadingSpace { line: usize },
    TrailingSpace { line: usize },
    DoubleSpace { line: usize },
}

impl fmt::Display for CorpusFormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotUtf8 => write!(f, "corpus is not valid UTF-8"),
            Self::ByteOrderMark => write!(f, "corpus starts with a byte order mark"),
            Self::CarriageReturn { offset } => write!(f, "U+000D at byte offset {offset}"),
            Self::Tab { offset } => write!(f, "U+0009 at byte offset {offset}"),
            Self::MissingFinalNewline => write!(f, "last line is not terminated by U+000A"),
            Self::NoDocuments => write!(f, "corpus has no documents"),
            Self::EmptyDocument { line } => write!(f, "line {line} is empty"),
            Self::LeadingSpace { line } => write!(f, "line {line} starts with U+0020"),
            Self::TrailingSpace { line } => write!(f, "line {line} ends with U+0020"),
            Self::DoubleSpace { line } => write!(f, "line {line} has consecutive U+0020"),
        }
    }
}

impl std::error::Error for CorpusFormatError {}

/// ADR-0001 D3: validate `friedman-tokens/1` bytes and return the descriptive counts.
///
/// Rules, in the order they are checked: UTF-8, no BOM, no U+000D, no U+0009, at least one line,
/// final U+000A, no empty line, no leading/trailing U+0020, no consecutive U+0020. A token is then
/// any maximal run of non-U+0020 characters on a line; by the rules above it is non-empty and
/// whitespace-free.
pub fn parse_corpus(bytes: &[u8]) -> Result<CorpusStats, CorpusFormatError> {
    let text = std::str::from_utf8(bytes).map_err(|_| CorpusFormatError::NotUtf8)?;
    if text.starts_with('\u{FEFF}') {
        return Err(CorpusFormatError::ByteOrderMark);
    }
    if let Some(offset) = bytes.iter().position(|&b| b == b'\r') {
        return Err(CorpusFormatError::CarriageReturn { offset });
    }
    if let Some(offset) = bytes.iter().position(|&b| b == b'\t') {
        return Err(CorpusFormatError::Tab { offset });
    }
    if text.is_empty() {
        return Err(CorpusFormatError::NoDocuments);
    }
    if !text.ends_with('\n') {
        return Err(CorpusFormatError::MissingFinalNewline);
    }
    let mut types = std::collections::BTreeSet::new();
    let mut n_tokens: u64 = 0;
    let mut min: u64 = u64::MAX;
    let mut max: u64 = 0;
    let mut n_documents: u64 = 0;
    for (line, doc) in text[..text.len() - 1].split('\n').enumerate() {
        if doc.is_empty() {
            return Err(CorpusFormatError::EmptyDocument { line });
        }
        if doc.starts_with(' ') {
            return Err(CorpusFormatError::LeadingSpace { line });
        }
        if doc.ends_with(' ') {
            return Err(CorpusFormatError::TrailingSpace { line });
        }
        if doc.contains("  ") {
            return Err(CorpusFormatError::DoubleSpace { line });
        }
        let mut len: u64 = 0;
        for tok in doc.split(' ') {
            types.insert(tok);
            len += 1;
        }
        n_tokens += len;
        min = min.min(len);
        max = max.max(len);
        n_documents += 1;
    }
    Ok(CorpusStats {
        n_documents,
        n_tokens,
        n_types: types.len() as u64,
        doc_length: DocLength {
            min,
            max,
            mean: n_tokens as f64 / n_documents as f64,
        },
    })
}

/// ADR-0001 D4 — the per-statistic verdict rule.
///
/// FAIL if the statistic was not measured or its value is `null`. `abs`/`rel` compare point
/// estimates as f64 (non-numeric values FAIL); `exact` compares JSON values structurally. `ci95`
/// never enters the verdict.
pub fn statistic_verdict(commanded: &Commanded, achieved: Option<&Achieved>) -> Verdict {
    let Some(achieved_value) = achieved.and_then(|a| a.value.as_ref()) else {
        return Verdict::Fail;
    };
    let pass = match commanded.tolerance {
        Tolerance::Exact { exact: true } => *achieved_value == commanded.value,
        Tolerance::Exact { exact: false } => false,
        Tolerance::Abs { abs } => match (commanded.value.as_f64(), achieved_value.as_f64()) {
            (Some(c), Some(a)) => (a - c).abs() <= abs,
            _ => false,
        },
        Tolerance::Rel { rel } => match (commanded.value.as_f64(), achieved_value.as_f64()) {
            (Some(c), Some(a)) => (a - c).abs() <= rel * c.abs(),
            _ => false,
        },
    };
    if pass { Verdict::Pass } else { Verdict::Fail }
}

/// ADR-0001 D4 — the F5 fidelity block: one verdict per commanded key; overall PASS iff all PASS.
pub fn fidelity(
    commanded: &BTreeMap<String, Commanded>,
    achieved: &BTreeMap<String, Achieved>,
) -> Fidelity {
    let per_statistic: BTreeMap<String, Verdict> = commanded
        .iter()
        .map(|(k, c)| (k.clone(), statistic_verdict(c, achieved.get(k))))
        .collect();
    let verdict = if per_statistic.values().all(|v| *v == Verdict::Pass) {
        Verdict::Pass
    } else {
        Verdict::Fail
    };
    Fidelity {
        per_statistic,
        verdict,
    }
}

/// ADR-0001 D1 — the on-disk form: 2-space indent, keys in JCS order, single trailing LF.
///
/// Key order follows from `serde_json::Value`'s sorted map (this crate does not enable
/// `preserve_order`); for ASCII keys that order coincides with JCS §3.2.3.
pub fn to_disk_json(manifest: &Manifest) -> serde_json::Result<String> {
    let value = serde_json::to_value(manifest)?;
    let mut s = serde_json::to_string_pretty(&value)?;
    s.push('\n');
    Ok(s)
}

/// Parse a manifest from JSON text (D4; unknown fields rejected).
pub fn from_json_str(text: &str) -> serde_json::Result<Manifest> {
    serde_json::from_str(text)
}

impl Manifest {
    /// Recompute everything recomputable and report every discrepancy against ADR-0001.
    ///
    /// Returns `Ok(())` when the manifest is internally consistent with `corpus_bytes`; otherwise
    /// every violated clause, so a caller sees the whole picture rather than the first failure.
    pub fn check(&self, corpus_bytes: &[u8]) -> Result<(), Vec<String>> {
        let mut problems = Vec::new();
        if self.manifest_schema != MANIFEST_SCHEMA {
            problems.push(format!(
                "D4: manifest_schema is {:?}, expected {MANIFEST_SCHEMA:?}",
                self.manifest_schema
            ));
        }
        if self.origin != ORIGIN_SYNTHETIC {
            problems.push(format!(
                "D4: origin {:?} is outside ADR-0001's scope",
                self.origin
            ));
        }
        if self.ground_truth.label != LABEL_MECHANISM {
            problems.push(format!(
                "D4/F1: synthetic ground_truth.label must be {LABEL_MECHANISM:?}, got {:?}",
                self.ground_truth.label
            ));
        }
        if self.ground_truth.family != self.generator.id {
            problems.push("D4: ground_truth.family must equal generator.id".to_string());
        }
        if self.ground_truth.derivation != DERIVATION_BY_CONSTRUCTION {
            problems.push(format!(
                "D4: ground_truth.derivation must be {DERIVATION_BY_CONSTRUCTION:?}"
            ));
        }
        if self.engine_commit.len() != 40
            || !self
                .engine_commit
                .chars()
                .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
        {
            problems.push(
                "D4/F6: engine_commit must be a 40-character lowercase hex git SHA".to_string(),
            );
        }
        match config_hash(&self.config) {
            Ok(h) if h == self.config_hash => {
                let id = corpus_id(&self.generator.id, &self.generator.version, &h, self.seed);
                if id != self.corpus_id {
                    problems.push(format!(
                        "D2: corpus_id is {:?}, derived {id:?}",
                        self.corpus_id
                    ));
                }
            }
            Ok(h) => problems.push(format!(
                "D5: config_hash is {}, recomputed {h}",
                self.config_hash
            )),
            Err(e) => problems.push(format!("D5: config cannot be canonicalized: {e}")),
        }
        if self.corpus.file != CORPUS_FILE {
            problems.push(format!("D2: corpus.file must be {CORPUS_FILE:?}"));
        }
        if self.corpus.format != CORPUS_FORMAT {
            problems.push(format!("D3: corpus.format must be {CORPUS_FORMAT:?}"));
        }
        let ch = content_hash(corpus_bytes);
        if ch != self.corpus.content_hash {
            problems.push(format!(
                "D5: content_hash is {}, recomputed {ch}",
                self.corpus.content_hash
            ));
        }
        match parse_corpus(corpus_bytes) {
            Ok(stats) => {
                let stated = CorpusStats {
                    n_documents: self.corpus.n_documents,
                    n_tokens: self.corpus.n_tokens,
                    n_types: self.corpus.n_types,
                    doc_length: self.corpus.doc_length.clone(),
                };
                if stats != stated {
                    problems.push(format!(
                        "D4: corpus counts stated {stated:?}, measured {stats:?}"
                    ));
                }
            }
            Err(e) => problems.push(format!("D3: corpus bytes violate {CORPUS_FORMAT}: {e}")),
        }
        for (k, a) in &self.achieved {
            if a.value.is_none() != a.reason.is_some() {
                problems.push(format!(
                    "D4: achieved[{k:?}] must carry `reason` iff `value` is null"
                ));
            }
        }
        let expected = fidelity(&self.commanded, &self.achieved);
        if expected != self.fidelity {
            problems.push(format!(
                "D4/F5: fidelity stated {:?}, recomputed {expected:?}",
                self.fidelity
            ));
        }
        if problems.is_empty() {
            Ok(())
        } else {
            Err(problems)
        }
    }
}
