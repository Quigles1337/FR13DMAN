# CLAIMS — append-only register

Entry schema (§3.3): `ID | informal statement | status | evidence | manifest/config deps | transitions log`.
Statuses: `UNKNOWN → VERIFIED (external source checked) / REPRODUCED (published result recovered by
our engine) / MEASURED (our own result: engine SHA + seed + config hash + CI) / CONDITIONAL /
FALSIFIED / RETRACTED`. Downgrades are always permitted and never shameful. Entries are appended and
transitioned, never deleted or silently edited.

F11 reminder: nothing enters this register from the founding sweep or from executor memory. A claim
about a source transitions out of UNKNOWN only after the source is retrieved this-session and
verified, with the retrieval recorded in SOURCES.md.

---

## CLAIM-0001 — closest prior art (believed: Nair 2026 Indus scorecard)

- **Statement:** The closest prior art to FRIEDMAN is a preprint (believed: Nair 2026,
  arXiv:2604.17828) presenting an Indus synthetic-baseline scorecard with tunable
  heraldic/administrative generators and a percentile scorecard — but no ROC calibration.
- **Status:** UNKNOWN
- **Evidence:** none — the arXiv ID itself is unverified sweep transcription (F0.2). Retrieval
  required before any transition.
- **Deps:** —
- **Transitions:**
  - 2026-08-22 — seeded UNKNOWN (Session 1, per §F0.2).

## CLAIM-0002 — mutual-information decay separation

- **Statement:** A published result (believed: Lin & Tegmark 2017) shows Markov processes yield
  exponential decay of two-point mutual information with distance, while context-free/natural
  language yields power-law decay.
- **Status:** UNKNOWN
- **Evidence:** none — retrieval required (F0.2/F11).
- **Deps:** —
- **Transitions:**
  - 2026-08-22 — seeded UNKNOWN (Session 1, per §F0.2).

## CLAIM-0003 — self-citation generator code release and license

- **Statement:** The self-citation Voynich-text generator (believed: Timm & Schinner) has a public
  code release (believed: Zenodo/GitHub) with an identifiable license permitting reuse or wrapping.
- **Status:** UNKNOWN
- **Evidence:** none — retrieval required (F0.2/F11); license terms must be recorded before any
  ingestion or wrapping decision (feeds the F2.1(iii) wrap-vs-reimplement ADR).
- **Deps:** —
- **Transitions:**
  - 2026-08-22 — seeded UNKNOWN (Session 1, per §F0.2).
