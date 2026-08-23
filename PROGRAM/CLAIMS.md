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
- **Status:** VERIFIED
- **Evidence:** S-0001 retrieved 2026-08-22 and read in full (13 pp.; SOURCES.md S-0001, SHA-256
  recorded). The arXiv ID transcription was correct: 2604.17828 = Nair, "How Non-Linguistic Is the
  Indus Sign System? A Synthetic-Baseline Scorecard" (cs.CL, 2026-04-20). Confirmed first-hand:
  (a) two tunable generator classes — heraldic (dials: Zipf exponent 0.9–1.9, positional strength
  0.05–0.25, bigram strength 0.3–0.9) and administrative (templates, Zipf exponent, noise rate),
  calibrated against six attested non-linguistic corpora (his Table 1);
  (b) percentile scorecard — §4.4: 100 synthetic corpora per baseline per metric, "discriminates"
  = observed value outside the 2.5th–97.5th percentile range;
  (c) **no ROC/AUC/false-positive-rate analysis anywhere in the paper** — discrimination is
  interval-based only; no detector operating curves, no calibration across corpus sizes.
  Load-bearing nuances: (i) abstract claims "All code and data are publicly available" but the
  Data and Code Availability section says upon-request only, public repository promised upon
  acceptance — no release URL, no license as of access date. **Consequence for F1.3 (R3):** "his
  released generators" do not yet exist publicly; R3 requires either the promised release
  appearing, an Al-gated author contact (§4.6: contacting external researchers), or reimplementation
  from the paper's published spec (§4.2 gives the dials and calibration table). (ii) His Table 2
  shows commanded-vs-achieved generator mismatch (heraldic achieves Zipf −0.97 against target
  −1.46) — prior art exhibiting exactly the F5 fidelity problem FRIEDMAN makes first-class.
- **Deps:** S-0001.
- **Transitions:**
  - 2026-08-22 — seeded UNKNOWN (Session 1, per §F0.2).
  - 2026-08-22 — UNKNOWN → VERIFIED (Session 1; retrieval + full first-hand read; S-0001).

## CLAIM-0002 — mutual-information decay separation

- **Statement:** A published result (believed: Lin & Tegmark 2017) shows Markov processes yield
  exponential decay of two-point mutual information with distance, while context-free/natural
  language yields power-law decay.
- **Status:** VERIFIED
- **Evidence:** S-0002 retrieved 2026-08-22; pp. 1–4 of arXiv:1606.06737v3 read first-hand.
  Attribution correct: Lin & Tegmark, "Criticality in Formal Languages and Statistical Physics"
  (Entropy 19, 299 (2017)). Confirmed first-hand: abstract states MI "decays exponentially in any
  probabilistic regular grammar, but can decay like a power law for a context-free grammar"; §II
  ("Markov implies exponential decay") Theorem 1 proves exponential decay with timescale set by
  the second eigenvalue for irreducible aperiodic Markov processes, states "no Markov process
  whatsoever can produce power-law decay," and p.4 generalizes to hidden Markov models; Fig. 1
  shows measured power-law MI decay in natural data (English/French text, Wikipedia, Bach,
  human genome) against the exponential Markov curve. Caveats to carry into F1.2: Fig. 1 uses a
  sliding-window two-point MI estimator (their App. D) — the constitution's "two-point MI
  limitations" note stands; the paper's own footnote distinguishes this power law from Zipf's law
  (one-point statistics), a distinction F1.2's write-up should preserve.
- **Deps:** S-0002.
- **Transitions:**
  - 2026-08-22 — seeded UNKNOWN (Session 1, per §F0.2).
  - 2026-08-22 — UNKNOWN → VERIFIED (Session 1; retrieval + first-hand read of the load-bearing
    sections; S-0002).

## CLAIM-0003 — self-citation generator code release and license

- **Statement:** The self-citation Voynich-text generator (believed: Timm & Schinner) has a public
  code release (believed: Zenodo/GitHub) with an identifiable license permitting reuse or wrapping.
- **Status:** VERIFIED
- **Evidence:** S-0003 retrieved 2026-08-22 via GitHub API: repo
  `TorstenTimm/SelfCitationTextgenerator` ("Self-citation text generator: Additional materials"),
  pinned at `a6ede220` (2019-11-29), containing `source/` and `executable/`; Zenodo DOI
  10.5281/zenodo.2531632 per README badge. **License: MIT**, confirmed from the repo's own LICENSE
  file (© 2019 Torsten Timm) — reuse and wrapping are license-clean, which resolves the licensing
  input to the F2.1(iii) wrap-vs-reimplement ADR (the engineering choice itself stays open).
  Honest scope limits: (i) the associated Cryptologia paper (Timm & Schinner 2020,
  DOI 10.1080/01611194.2019.1596999) is corroborated but not yet retrieved — publisher page 403;
  owed on first substantive use; (ii) the code itself has not been read or ingested — this claim
  covers existence + license only; (iii) the Zenodo record itself was not fetched this session
  (DOI recorded from the README badge).
- **Deps:** S-0003.
- **Transitions:**
  - 2026-08-22 — seeded UNKNOWN (Session 1, per §F0.2).
  - 2026-08-22 — UNKNOWN → VERIFIED (Session 1; GitHub API retrieval incl. in-repo LICENSE;
    S-0003).
