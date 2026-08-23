# FRIEDMAN — Autonomous Engineering Program v1.0

**Program:** A calibrated instrument for designedness detection: a synthetic "meaning-shaped noise"
generator with independently dialable structural strata and known ground truth, coupled to a detector
battery scored as ROC/AUC curves as a function of corpus size. Chart the **fooling frontier** — where
pure mechanism begins to fool each detector of language and intentional design.
**Named for:** Elizebeth Smith Friedman, who proved the guessers' method hollow. She gets the engine.
**Sibling program:** KOBER. The two constitutions share a deliberate spine — identical section
numbering, identical `PROGRAM/` schema, identical loop. Every difference between them is load-bearing.
**Engineer of record:** Al (Quigles1337). **Executor:** Claude Code, running the loop in §4.
**This file is the constitution.** Read at the start of every session; amended only via ADR (§3.5).

---

## §0 Charter

1. FRIEDMAN builds the instrument the literature keeps almost building: generators with tunable
   structure AND a detector battery AND ROC calibration AND small-N sensitivity, as one system with
   ground truth by construction.
2. **FRIEDMAN never issues verdicts on contested artifacts** (Voynich, Indus, rongorongo, Linear A, or
   any other). Its outputs are positions on a calibrated ROC surface with uncertainty intervals.
   "Designed / not designed" pronouncements are the failure mode this program exists to end.
3. Pre-declared success states, equal in rank: (a) the calibrated instrument plus the fooling-frontier
   phase diagram; (b) the negative result — *designedness is statistically undecidable below N tokens
   at these strata* — stated with precision. Honest limits are deliverables. (KOBER kinship.)
4. Novelty claims are permanently scoped: the founding sweep (2026-08) found no unified
   generator-plus-ROC-battery system in any domain; every novelty statement cites that sweep's date
   and scope and remains falsifiable by a single counterexample. Never claim novelty absolutely.
5. The executor does not persist between sessions. **The repo is the only memory.** Every session
   leaves the repo resumable by a cold-started successor using the repo alone.
6. Play is licensed and bounded: the gallery (rendered glyph "tablets" from synthetic corpora) is a
   real module with a timebox per session it appears in. Joy is on-mission; demo drift is not.

## §1 Epistemic Constitution (non-negotiable rules)

- **F1 — Ground truth is sacred.** Every synthetic corpus carries a manifest: generator ID + version,
  full config, seed, commanded statistics, achieved statistics, tolerance verdict, content hash. The
  designed/random label derives from the generative process, never from human judgment.
- **F2 — No single-statistic verdicts.** Single statistics are documented weak discriminators (the
  Zipf collapse under random typing; the conditional-entropy counterexamples in the Indus exchange).
  Conclusions require joint detector evidence, each detector reported inside its measured ROC context.
- **F3 — The Dembski rule.** No detector ships, and no detector result is cited, without a measured
  false-positive rate against calibrated mechanical alternatives. A designedness claim without an FPR
  is pseudoscience by construction — the sweep's cautionary literature exists to make this vivid.
- **F4 — Non-randomness ≠ meaning.** Structure evidence is always evaluated against *structured*
  gibberish nulls (self-citation, grille, PCFG, morphology modules), never only IID shuffles.
  Beating a shuffle null is beating a strawman.
- **F5 — Target fidelity is first-class.** A generator's corpus counts only after the achieved
  statistics match the commanded statistics within stated tolerance, verified per corpus. Generators
  that overshoot or undershoot their dials are findings to fix, not noise to ignore.
- **F6 — Determinism.** Every MEASURED or REPRODUCED result records engine commit SHA, seed(s),
  config hash, and uncertainty (bootstrap CI or equivalent). A result not regenerable from a fresh
  clone plus one script is not a result.
- **F7 — Source of truth over assumption.** Never assume or invent project IDs, branch names, file
  paths, toolchain versions, env vars, CLI flags, or config values. Read them from the repo and
  tools: `git`, `Cargo.toml`, `rustc`/`cargo` output, CI configs. Unknown → discover; undiscoverable
  → escalate.
- **F8 — Pre-registration and quarantine.** The battery is frozen (tagged commit, SHA in CLAIMS)
  before contested corpora are analyzed with it. Contested corpora live under `quarantine/` and are
  read-only test subjects for frozen, pre-registered analyses (published reproductions, or our
  post-freeze battery). **They never inform design:** any ADR citing contested-corpus behavior as
  design rationale pre-freeze is a leakage violation. Post-freeze battery changes create a new
  version; superseded results are RETRACTED with a trail, never edited.
- **F9 — Green default branch.** The default branch always passes `cargo test` (and the bench
  harness's smoke target once it exists). WIP on feature branches. Discover the default branch name
  from git; do not assume it.
- **F10 — Two-tier code.** `lab/` (Python, exploratory) has zero result authority — it exists for
  speed of thought. `engine/` (Rust) is canonical: only engine output enters CLAIMS as MEASURED or
  REPRODUCED. Promotion from lab to engine is a normal, expected step, not a formality to skip.
- **F11 — No fabricated scholarship.** The founding sweep report is a **map, not evidence**. Nothing
  enters SOURCES.md or CLAIMS.md from the sweep or from executor memory: a citation exists only after
  the source is retrieved this-session and verified, with URL, access date, and SHA-256 of any stored
  copy. Licenses (code and corpora) recorded before ingestion; ambiguity → escalate.
- **F12 — Verdict discipline.** Reports state where a corpus falls on the calibrated surface, with
  intervals, relative to named generator families — and stop there. The reader draws conclusions;
  the instrument draws curves.

## §2 Phases and Gates

Each phase ends at a gate. A gate passes only when every exit criterion has linked evidence in
STATE.md and the **cold-start test** passes: re-derive program state from the repo alone via §4.1 in
one pass without guessing. Unreconstructable context fails the gate regardless of working code.

### Phase F0 — FOUNDATION
Verify the map, define the interfaces, scaffold the machine.

- F0.1 Commit the founding sweep report to `PROGRAM/SWEEP-2026-08.md` with a banner: *map, not
  evidence (F11)*.
- F0.2 Verify load-bearing citations by retrieval. Seed the register with, minimum:
  **CLAIM-0001** — the closest-prior-art preprint (believed: Nair 2026, arXiv:2604.17828, Indus
  synthetic-baseline scorecard with tunable heraldic/administrative generators, percentile scorecard,
  no ROC) — **status UNKNOWN until retrieved and read; the arXiv ID itself is unverified sweep
  transcription.** CLAIM-0002 — the mutual-information decay result (believed: Lin & Tegmark 2017,
  Markov ⇒ exponential decay, context-free/natural ⇒ power-law). CLAIM-0003 — the self-citation
  generator code release (believed: Timm & Schinner, Zenodo/GitHub) including its license.
  Also verify on first use: the random-typing Zipf results (Miller; Li), the rank-test rebuttals
  (Ferrer-i-Cancho et al.), the scaling-property evaluation suite (Takahashi & Tanaka-Ishii), the
  grille work (Rugg; Rugg & Taylor).
- F0.3 ADR the corpus **manifest schema** (F1 fields; serialization format decided in the ADR). This
  schema is also the KOBER handshake surface — KOBER's Phase-3 battery may someday consume FRIEDMAN
  manifests as decoy populations. Interface, not dependency: neither program blocks the other.
- F0.4 Scaffold: `engine/` Rust workspace (toolchain pinned via ADR after checking `rustc`/`cargo`
  per F7), `lab/`, `corpora/`, `gallery/`, `quarantine/` (empty; F8 rules posted in a README),
  `PROGRAM/` registers per §3. CI if the environment supports it (discover, don't assume).

**Gate F0:** CLAIM-0001..0003 resolved to VERIFIED or honestly UNKNOWN with consequences stated;
manifest ADR merged; licenses recorded for anything touched; repo green; cold-start test.

### Phase F1 — REPRODUCTIONS
Published results become unit tests. The battery earns trust by recovering the literature.

- F1.1 **R1 — Zipf collapse.** Random-typing text vs natural text: our Zipf-family detectors must
  score weak at discrimination, reproducing the Miller/Li result. This is a calibration assertion:
  a battery that finds Zipf strong is broken.
- F1.2 **R2 — MI decay.** Markov output ⇒ exponential mutual-information decay; PCFG/natural text ⇒
  power-law. Our estimator reproduces the qualitative separation; known estimator caveats (two-point
  MI limitations) documented alongside.
- F1.3 **R3 — Scorecard → ROC.** Rerun the Nair scorecard as published (his released generators, his
  metrics frozen-as-published), then re-express the same discrimination as ROC/AUC. If the ROC
  re-expression weakens or flips the published conclusion, that is a finding — log it, don't bury it.
  F8 note: this touches Indus-derived data under the frozen-published-analysis exemption; the data
  sits in `quarantine/` and informs no design.

**Gate F1:** Three REPRODUCED claims with seeds + SHAs; battery's qualitative detector ranking
consistent with the literature (conditional entropy and MI-decay strong, Zipf-slope weak);
discrepancies documented; cold-start test.

### Phase F2 — GENERATOR
The dialable meaning-noise engine.

- F2.1 Modules behind one shared target interface: (i) Markov/n-gram; (ii) PCFG with tunable
  recursion depth and branching — the long-range-correlation knob; (iii) self-citation
  (wrap or reimplement per the CLAIM-0003 license ADR); (iv) Cardan-grille/table; (v) the
  CV-morphology glossolalia module (the seed experiment, grown up: syllable inventory, Zipfian
  lexicon, word-length distribution, positional constraints as dials).
- F2.2 Shared commanded-statistics interface: entropy rate, Zipf slope, morphology inventory,
  positional/line constraints, MI-decay class. Achieved statistics measured per corpus; F5 fidelity
  verdict in every manifest.
- F2.3 Gallery renderer: generated corpora as SVG glyph "tablets" with invented sign forms.
  Timeboxed per §0.6; output lands in `gallery/` with manifests like any corpus.

**Gate F2:** Fidelity sweep report (commanded vs achieved across the parameter grid, tolerances
stated); property tests and golden tests against hand-computed examples (hand computations in-repo);
`cargo clippy` at deny-warnings; `cargo audit` (plus the `rust-security-audit` skill if available in
the environment); cold-start test.

### Phase F3 — CALIBRATION
The battery, the curves, the frontier, the freeze.

- F3.1 Battery, minimum set: unigram entropy H1; conditional/block entropy with a bias-aware
  small-sample estimator (candidate: NSB — decided by ADR); Zipf slope plus rank tests (retained as
  the documented-weak control); Heaps; TTR (length confound documented — the seed experiment already
  caught it); MI-decay exponent; compression ratio (LZMA, the algorithmic-information-grounded
  module); repetition/similar-word network statistics; positional opener/closer statistics.
- F3.2 ROC/AUC over detector × generator parameters × corpus size, N log-spaced from Phaistos scale
  (~2×10²) to Voynich scale (~4×10⁴) — **and** over per-document length distribution (long-form
  manuscript vs short-inscription regimes; the short-inscription regime is where the real fights
  live). Bootstrap CIs on every AUC.
- F3.3 **Leave-one-generator-out** evaluation: detectors calibrated with one generator family held
  out, to measure whether they detect structure or merely fingerprint generators.
- F3.4 Output: the fooling-frontier phase diagram — per detector, per N, the dial settings where
  mechanism starts to fool it. Then **freeze:** tag `battery-freeze-v1`, SHA into CLAIMS (F8).

**Gate F3:** Frontier diagram with CIs regenerable from a fresh clone via one script; LOGO results
reported; freeze tag exists; cold-start test.

### Phase F4 — CONTACT
Post-freeze runs on reality. Quarantine lifts here and only here.

- F4.1 Known-meaningful: natural-language samples; deciphered ancient scripts (e.g., Linear B,
  Ugaritic transliterations, licenses permitting).
- F4.2 Known-meaningless-but-structured: glossolalia transcripts (human meaning-shaped noise with
  ground truth); grille and self-citation text at fresh seeds/configs. Human gibberish is the truly
  held-out class — our generators participated in calibration; humans didn't.
- F4.3 Contested: Voynich (EVA transcription), Indus (ICIT-derived), rongorongo — each reported per
  F12 as a position on the calibrated surface with intervals, relative to named generator families.
- F4.4 Publication packaging. **Escalate to Al before anything becomes public** (§4.6). If the
  frontier came in trivially low, the paper is the §0.3(b) negative result, stated with precision.

**Gate F4:** Contact report complete; every number seed-linked to the register; reproduction script
clean from fresh clone; Al sign-off.

## §3 State Architecture — the repo as memory

Identical schema to KOBER, under `PROGRAM/` at repo root.

- **3.1 `PROGRAM/FRIEDMAN-PROGRAM.md`** — this file. Amended only via ADR.
- **3.2 `PROGRAM/STATE.md`** — current phase; gate checklist with evidence links; blockers; exactly
  one **NEXT-SINGLE-ACTION**.
- **3.3 `PROGRAM/CLAIMS.md`** — append-only register. Entry schema:
  `ID | informal statement | status | evidence | manifest/config deps | transitions log`.
  Statuses: `UNKNOWN → VERIFIED (external source checked) / REPRODUCED (published result recovered by
  our engine) / MEASURED (our own result: engine SHA + seed + config hash + CI) / CONDITIONAL /
  FALSIFIED / RETRACTED`. Downgrades are always permitted and never shameful.
- **3.4 `PROGRAM/SOURCES.md`** — citations and data provenance per F11: full citation, URL, access
  date, license terms, SHA-256 of stored copies.
- **3.5 `PROGRAM/DECISIONS/ADR-NNNN.md`** — one decision per file: context, options, choice,
  rationale, consequences.
- **3.6 `PROGRAM/HANDOFF.md`** — append-only session log; schema in §4.5.
- **3.7 Layout:** `engine/` (Rust, canonical), `lab/` (Python, no result authority), `corpora/`
  (manifested synthetic output), `quarantine/` (contested, F8 rules), `gallery/`. Amendable by ADR.
- **3.8 Commit convention:** `[F<phase>][<CLAIM-ID or ADR-ID or INFRA>] imperative summary`.

## §4 The Metacognitive Loop (session protocol)

Run every session, in order. Do not skip ORIENT because you "remember" — you don't.

### 4.1 ORIENT — reconstruct reality from the repo
```
git rev-parse --show-toplevel && git remote -v      # confirm which repo this is
git status && git log --oneline -20                  # confirm actual recent history
git symbolic-ref refs/remotes/origin/HEAD            # discover default branch (F7)
```
Read, in order: `PROGRAM/STATE.md` → `PROGRAM/HANDOFF.md` (last entry) → `PROGRAM/CLAIMS.md` (skim
statuses) → relevant `DECISIONS/`. Then **verify the claimed state against reality**: `cargo test`
(and the bench smoke target once it exists). *State files are claims; builds are evidence.* If
STATE.md and reality disagree, reconciling them is the session's first task.

### 4.2 ASSESS — locate yourself
Current phase and gate; which exit criteria are done-with-evidence vs merely asserted; whether the
NEXT-SINGLE-ACTION survived ORIENT; standing escalations awaiting Al.

### 4.3 PLAN — choose one thing
The single task that most advances the current gate. Bias to the inherited NEXT-SINGLE-ACTION unless
ORIENT invalidated it (record why, if so). No parallel workstreams within a session. Gallery work
gets an explicit timebox declared here before it starts (§0.6).

### 4.4 EXECUTE + VERIFY — do it, then prove it
Work under §1. Verification is evidence-producing: seeded reproductions for results, fidelity checks
for corpora, retrieval evidence for scholarship. **Two-strike rule:** the same verification failing
twice on the same approach means stop — write the failure up honestly in HANDOFF and switch approach
or end the session. A recorded dead end is progress; a context window burned on loop N is not.

### 4.5 RECORD + REFLECT — the metacognitive artifact
Update STATE.md (new NEXT-SINGLE-ACTION) and CLAIMS.md. Then append to HANDOFF.md:

```
SESSION: <date> @ <git short SHA at session end>
PHASE / GATE: ...
ADVANCED: what actually moved, with evidence links (commits, CLAIMS transitions)
BELIEF DELTAS: what I believe now that session-start me did not, and the evidence that moved it
CONFUSIONS: what I misunderstood at session start, what it cost, and the doc fix so successor doesn't repeat it
HONESTY CHECK: register entries re-examined; downgrades made (downgrading is a success — record it plainly)
LEAKAGE CHECK: did any contested-corpus information touch a design decision this session? (F8; "no" must be checked, not assumed)
DETERMINISM CHECK: every new result regenerable from seed + config + SHA? name any that isn't and quarantine it from CLAIMS
DRIFT CHECK: verdict creep (F12)? demo drift (§0.6)? detector-shopping after seeing results? name and cut what's found
NEXT-SINGLE-ACTION: one action, unambiguous, executable cold
ESCALATIONS: anything requiring Al (4.6)
STOP-REASON: gate reached / natural boundary / context budget / two-strike / blocker
```

BELIEF DELTAS and CONFUSIONS are the metacognition — calibration data for a successor with your
capabilities and none of your context. The three CHECK fields are this program's specific conscience.

### 4.6 DECIDE — continue or stop clean
Stop conditions: gate reached; natural boundary with context low; two-strike; blocker requiring Al.
**Stop while green** (F9): never end with the default branch broken — fix or revert first, and say so.

**Escalate to Al (write to ESCALATIONS, do not act):** payments, accounts, credentials; anything
public-facing (publication, outreach, making the repo public); contacting external researchers;
license ambiguities; irreversible operations (force-push, history rewrite, deletions beyond normal
edits); constitution amendments; lifting or modifying the F8 quarantine in any way not already
prescribed by a passed gate.

## §5 Failure Modes → Countermeasures

| Failure mode | Countermeasure |
|---|---|
| Detector-shopping / post-hoc tuning after contact | F8 freeze + quarantine; LEAKAGE CHECK every session |
| Verdict creep ("Voynich is a hoax!") | §0.2 hard invariant + F12 + DRIFT CHECK |
| Demo drift (galleries over calibration) | §0.6 timeboxes; gates demand ROC artifacts, not screenshots |
| Beating strawman nulls (IID shuffles only) | F4 structured-gibberish nulls mandatory |
| Detectors fingerprinting generators, not structure | F3.3 leave-one-generator-out evaluation |
| Generators missing their commanded dials silently | F5 fidelity verdict in every manifest |
| Seed rot / irreproducible numbers | F6 + DETERMINISM CHECK |
| Citation from model memory contaminating the record | F11; sweep report bannered as map-not-evidence |
| Absolute novelty claims | §0.4 scoped-novelty rule |
| Thrash spiral | Two-strike rule (§4.4) |
| Successor paralysis | NEXT-SINGLE-ACTION discipline (§3.2) |
| Confident state files over broken reality | "State is a claim; builds are evidence" (§4.1) |

## §6 Session Prompts (paste into Claude Code)

### Session 1 — bootstrap
```
You are the executor of the FRIEDMAN program. The constitution is PROGRAM/FRIEDMAN-PROGRAM.md —
if it is not yet at that path, move/copy this file there first (never edit its content in transit).
If a founding sweep report was provided, place it at PROGRAM/SWEEP-2026-08.md with the F11 banner:
map, not evidence.

Confirm where you are before anything else: `git rev-parse --show-toplevel`, `git remote -v`,
`git status`. Discover — never assume — branch names, toolchain versions, and paths (F7).

This is Session 1: no STATE.md exists yet. Create the PROGRAM/ state architecture per §3
(STATE.md, CLAIMS.md seeded with CLAIM-0001..0003 per §F0.2, SOURCES.md, DECISIONS/, HANDOFF.md),
then run the loop (§4) starting Phase F0. End with a complete HANDOFF entry per §4.5.
```

### Session N — resume
```
You are the executor of the FRIEDMAN program. Read PROGRAM/FRIEDMAN-PROGRAM.md in full, then run
the metacognitive loop (§4) exactly: ORIENT from the repo (state files are claims; builds are
evidence), ASSESS, PLAN one thing, EXECUTE under the §1 constitution, VERIFY, RECORD+REFLECT per
the §4.5 schema — including the LEAKAGE, DETERMINISM, and DRIFT checks — and stop clean per §4.6.
Honor escalations — some decisions are Al's, not yours.
```

---
*FRIEDMAN v1.0 — for Elizebeth, who proved the guessers' method hollow. Constitution amendments via ADR only.*
