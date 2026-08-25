# SOURCES — citations and data provenance (F11)

An entry exists here only after the source was retrieved this-session and verified. Every entry
records: full citation, URL, access date, license terms, and SHA-256 of any stored copy. Licenses
(code and corpora) are recorded before ingestion; ambiguity → escalate to Al.

The founding sweep report (PROGRAM/SWEEP-2026-08.md, once ferried) is a **map, not evidence** — it
can suggest what to retrieve, but nothing enters this register from it or from executor memory.

Storage rule: a stored copy is committed to the repo only when its license permits redistribution
(this repo is public). Otherwise the entry records URL + access date + SHA-256 of the local copy and
notes that the copy is not committed.

---

## S-0001 — Nair 2026 (closest prior art; CLAIM-0001)

- **Citation:** Nair, Ashish (2026). "How Non-Linguistic Is the Indus Sign System? A
  Synthetic-Baseline Scorecard." arXiv:2604.17828v1 [cs.CL], submitted 2026-04-20. Independent
  Researcher, ashishn@alumni.cmu.edu. 13 pages.
- **URL:** https://arxiv.org/abs/2604.17828 (abs), https://arxiv.org/pdf/2604.17828v1 (PDF),
  https://arxiv.org/html/2604.17828v1 (HTML)
- **Access date:** 2026-08-22 (abs page, HTML full text, and PDF all retrieved this session; PDF
  read in full, pages 1–13)
- **License:** arXiv.org perpetual non-exclusive license 1.0 ("Nonexclusive-Distrib 1.0", as shown
  on the abs page). This license does NOT grant third-party redistribution rights → **stored copy
  not committed** (public repo). Local verification copy hashed at retrieval:
  - SHA-256 `43e7bab2dd269a2fb0cc80382687d49b1da066bf852e7ddd369df526670ff7a3`
    (749,585 bytes, `2604.17828v1` PDF). Re-retrievable from arXiv by version-pinned URL; any
    future copy can be checked against this digest.
- **Data/code availability (load-bearing):** the paper's abstract states "All code and data are
  publicly available," but its Data and Code Availability section states availability **from the
  corresponding author upon request**, with a public repository promised **upon acceptance**. As of
  access date there is no public release URL and no code/data license. Consequence recorded at
  CLAIM-0001.
- **Notes:** paper carries an AI Assistance Disclosure (pipeline developed with LLM assistance;
  all reported statistics computed by deterministic Python on the fixed corpus). Corpus:
  ICIT/Yajnadevam digitization, 1,916 deduplicated inscriptions, 584 unique signs, 11,110 tokens.

## S-0002 — Lin & Tegmark 2017 (MI-decay result; CLAIM-0002)

- **Citation:** Lin, Henry W. & Tegmark, Max (2017). "Criticality in Formal Languages and
  Statistical Physics." arXiv:1606.06737v3 [cond-mat.dis-nn] (v1 2016-06-21, v3 2017-08-23).
  Published as "Critical Behavior in Physics and Probabilistic Formal Languages," *Entropy* 19,
  299 (2017), per the paper's own p.1 footnote.
- **URL:** https://arxiv.org/abs/1606.06737 (abs), https://arxiv.org/pdf/1606.06737v3 (PDF).
  Journal version: https://www.mdpi.com/1099-4300/19/7/299 — **retrieval blocked this session
  (HTTP 403, bot protection); journal-page license terms therefore unverified.** Verification
  performed on the arXiv v3.
- **Access date:** 2026-08-22 (abs page fetched; PDF pp. 1–4 read first-hand: abstract, Fig. 1,
  §II incl. Theorem 1 and the HMM generalization).
- **License:** arXiv.org perpetual non-exclusive license (per abs page) → **stored copy not
  committed.** Local verification copy hashed at retrieval:
  - SHA-256 `31c0b7798ceb75d53ecd2601420e648eb2cd7d10f1f42ab0aecda2c6e995eb93`
    (1,744,340 bytes, `1606.06737v3` PDF).

## S-0003 — Timm self-citation generator code release (CLAIM-0003)

- **Citation (code):** Timm, Torsten (2019). *Self-citation text generator: Additional materials.*
  GitHub, `TorstenTimm/SelfCitationTextgenerator`, default branch `master`, pinned at commit
  `a6ede2202dd7ad6285ce2c007bf22c2a0e7709b7` (2019-11-29, last push). Zenodo DOI per README badge:
  10.5281/zenodo.2531632. Contents include `source/`, `executable/`, `graphs/`, `gephi/`.
- **URL:** https://github.com/TorstenTimm/SelfCitationTextgenerator
- **Access date:** 2026-08-22 (repo metadata, file listing, README, and LICENSE retrieved via
  GitHub API this session; code itself not yet ingested).
- **License:** **MIT** — confirmed from the repo's own `LICENSE` file ("MIT License, Copyright (c)
  2019 Torsten Timm") and GitHub's license detection for this repo. (Quirk recorded: the README's
  MIT badge hyperlinks to a different repo of the same author, `TorstenTimm/TestText` — the
  in-repo LICENSE governs.) MIT permits reuse, modification, and redistribution with attribution
  → wrapping or vendoring is license-clean when F2.1(iii) arrives.
- **Associated paper (corroborated, not yet retrieved):** Timm, Torsten & Schinner, Andreas.
  "A possible generating algorithm of the Voynich manuscript." *Cryptologia* 44(1), 2020,
  DOI 10.1080/01611194.2019.1596999. Publisher abstract page returned HTTP 403 this session;
  identity corroborated by the repo's subject matter and README. Full retrieval owed on first
  substantive use (F0.2 "verify on first use").

## S-0004 — RFC 8785, JSON Canonicalization Scheme (ADR-0001 hashing dependency)

- **Citation:** Rundgren, A., Jordan, B., Erdtman, S. (2020). "JSON Canonicalization Scheme (JCS)."
  RFC 8785, Independent Submission, Informational, ISSN 2070-1721, June 2020.
- **URL:** https://www.rfc-editor.org/rfc/rfc8785.txt (text), https://www.rfc-editor.org/info/rfc8785
  (info page, per the document's own status section)
- **Access date:** 2026-08-25 (text retrieved, HTTP 200, 41,879 bytes; §3.2.1 whitespace, §3.2.2.3
  number serialization, §3.2.3 property sorting, and §3.2.4 UTF-8 generation read first-hand and
  quoted into ADR-0001 D5).
- **License:** "Copyright (c) 2020 IETF Trust and the persons identified as the document authors.
  All rights reserved. This document is subject to BCP 78 and the IETF Trust's Legal Provisions
  Relating to IETF Documents (https://trustee.ietf.org/license-info)" — quoted from the document's
  Copyright Notice. The Trust Legal Provisions themselves were not retrieved this session, so
  redistribution rights are **unverified** → **stored copy not committed** (public repo). Local
  verification copy hashed at retrieval:
  - SHA-256 `63d52294eb0e3f0014174288186d388b4ddbf2c67d1ce8af1d9726eb0c3ab240`
    (41,879 bytes, `rfc8785.txt`). Re-retrievable from rfc-editor.org; RFC texts are immutable once
    published, so any future copy can be checked against this digest.
- **Role:** normative reference for `config_hash` canonicalization in ADR-0001 D5. Engineering
  standard, not scholarship — no CLAIM depends on it.

## S-0005 — Rust dependency set of `engine/` (ADR-0002 D5; licenses recorded before ingestion)

- **What:** every crate in `engine/Cargo.lock` at ADR-0002's commit. Source: crates.io.
- **Access date:** 2026-08-25 (fetched by `cargo fetch`; licenses read from the crates.io index
  metadata via `cargo info <crate>@<version>` the same day).
- **Stored copies:** the local cargo registry cache (not committed). **Digests:** `Cargo.lock`
  records the SHA-256 checksum of every `.crate` archive — that lockfile is the committed digest
  record for this entry.
- **Security audit:** `cargo audit` (cargo-audit 0.22.2, RustSec advisory DB, 1,226 advisories
  loaded) on 2026-08-25 scanned the 25-crate lockfile and reported no advisories.
- **Licenses (all permissive; SPDX expressions as published by each crate):**

| Crate | Version | License | Role |
|---|---|---|---|
| serde | 1.0.229 | MIT OR Apache-2.0 | direct |
| serde_json | 1.0.151 | MIT OR Apache-2.0 | direct (`float_roundtrip`) |
| serde_json_canonicalizer | 0.3.2 | MIT | direct — RFC 8785 JCS (ADR-0002 D4) |
| sha2 | 0.11.0 | MIT OR Apache-2.0 | direct |
| hex | 0.4.3 | MIT OR Apache-2.0 | direct |
| block-buffer | 0.12.1 | MIT OR Apache-2.0 | transitive (sha2) |
| cfg-if | 1.0.4 | MIT OR Apache-2.0 | transitive |
| const-oid | 0.10.2 | Apache-2.0 OR MIT | transitive (digest) |
| cpufeatures | 0.3.0 | MIT OR Apache-2.0 | transitive (sha2) |
| crypto-common | 0.2.2 | MIT OR Apache-2.0 | transitive (digest) |
| digest | 0.11.3 | MIT OR Apache-2.0 | transitive (sha2) |
| hybrid-array | 0.4.14 | MIT OR Apache-2.0 | transitive (digest) |
| itoa | 1.0.18 | MIT OR Apache-2.0 | transitive (serde_json) |
| libc | 0.2.189 | MIT OR Apache-2.0 | transitive (cpufeatures) |
| memchr | 2.8.3 | Unlicense OR MIT | transitive (serde_json) |
| proc-macro2 | 1.0.107 | MIT OR Apache-2.0 | transitive (serde_derive) |
| quote | 1.0.47 | MIT OR Apache-2.0 | transitive (serde_derive) |
| ryu-js | 1.0.3 | Apache-2.0 OR BSL-1.0 | transitive (serde_json_canonicalizer) — ECMAScript number formatting |
| serde_core | 1.0.229 | MIT OR Apache-2.0 | transitive (serde) |
| serde_derive | 1.0.229 | MIT OR Apache-2.0 | transitive (serde) |
| syn | 3.0.4 | MIT OR Apache-2.0 | transitive (serde_derive) |
| typenum | 1.20.1 | MIT OR Apache-2.0 | transitive (hybrid-array) |
| unicode-ident | 1.0.24 | (MIT OR Apache-2.0) AND Unicode-3.0 | transitive (proc-macro2) |
| zmij | 1.0.23 | MIT | transitive (serde_json) — float printing |

- **Rule going forward:** a crate is added to `engine/` only after its license is appended to this
  table (same session, same commit). Ambiguity → escalate to Al (F11).
