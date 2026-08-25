# STATE

- **Updated:** 2026-08-25 (Session 5, close)
- **Phase:** F0 — FOUNDATION
- **Gate:** F0 (**open, decision pending Al**). Session 5 ran the cold-start exam under ADR-0003
  with the purged memory: every step passed from the repo alone, no value guessed (HANDOFF,
  Session 5). Result recorded as **CONDITIONAL PASS at `766a095`** because the session ran in the
  same conversation context as the sessions that built the state — ADR-0003's *unratified*
  executor corollary would void it. The executor does not close a gate on its own authority; Al
  rules (see escalations).
- **Default branch:** `main`. Remote: `origin` = `https://github.com/Quigles1337/FR13DMAN`
  (**public**; created and designated by Al 2026-08-22 — see ADR-0000).
  **ORIENT note (F7):** `git symbolic-ref refs/remotes/origin/HEAD` failed on this working copy at
  the start of Session 2 ("not a symbolic ref") and resolved after `git fetch --all --prune`.
  Source-of-truth fallback: `git remote show origin` (prints `HEAD branch: main`). Do not guess.
- **Build status:** **green** = `bash scripts/check.sh` prints `ALL GREEN` (ADR-0002 D6: fmt
  --check + `cargo test` + clippy `-D warnings` + lab verifier). Last verified locally 2026-08-25 at
  `766a095` (16/16 tests; verifier 44 checks). CI: `.github/workflows/check.yml` runs the same
  script on push to `main` and on PRs; last known CI success on `main` @ `766a095` (run
  32888642294). Check `gh run list -R Quigles1337/FR13DMAN --branch main` at ORIENT; this line
  lags the last push by one run. **Never make the workflow a required status check** (ADR-0002 D6).
- **Toolchain (pinned, ADR-0002 D1):** `rust-toolchain.toml` at repo root → `1.91.0` + clippy +
  rustfmt; edition 2024. Observed alongside, not pinned: Python 3.11.9 (lab verifier), `gh 2.74.0`
  authenticated as Quigles1337, cargo-audit 0.22.2.

## Gate F0 exit criteria

| Criterion | Status | Evidence |
|---|---|---|
| F0.1 sweep report at `PROGRAM/SWEEP-2026-08.md` with F11 banner | **done** | Ferried by Al 2026-08-25; identity verified against Al's stated digest before the move (SHA-256 `6ba3eb341ba2ecd325b68eb31d4bfafcbb46a47e6669c4d98374ec771e56275c`, 35,207 bytes, LF-only UTF-8, no BOM); F11 banner at line 1; ferry note at line 129 records that the sweep's "publicly released" claim about Nair's generators is superseded by S-0001. Commit `6c97b53`. Re-verified from the repo in Sessions 4 and 5 |
| F0.2 CLAIM-0001..0003 resolved VERIFIED or honestly UNKNOWN with consequences | **done (core)** | all three VERIFIED 2026-08-22 (commits `6649990`, `bee91d9`; S-0001..S-0003). "Verify on first use" list (Miller/Li, Ferrer-i-Cancho, Takahashi & Tanaka-Ishii, Rugg) remains open by design |
| F0.3 manifest schema ADR merged | **done** | ADR-0001 (`7288a49`) + verifier (`2e9bef0`); enforced by the canonical tier (`27dd471`); comparison arithmetic pinned by ADR-0004 (`a3f42d2`) with seven conformance vectors identical in Rust and Python |
| F0.4 scaffold (`engine/`, `lab/`, `corpora/`, `gallery/`, `quarantine/` + F8 README, CI) | **done** | ADR-0002 (`27dd471`): `rust-toolchain.toml` 1.91.0; `engine/` workspace, crate `friedman-manifest` (16 tests incl. RFC 8785 §3.2 sample bytes and all 24 finite Appendix B rows); `scripts/check.sh` gate; `.github/workflows/check.yml` (actions at v5, `91faff1`); `corpora/` `gallery/` `quarantine/` READMEs — F8 quotation checked verbatim programmatically in Session 5; `.gitignore`. CI success on `main` @ `766a095` |
| Licenses recorded for anything touched | **done for everything touched so far** | S-0001..S-0004 (papers/RFC: hashed, not committed; S-0004 digest re-reproduced from the URL in Session 5); S-0005 (all 24 locked crates, licenses before ingestion; `cargo audit` exit 0 re-run 2026-08-25 Session 5). Standing rule: new crate ⇒ S-0005 row in the same commit |
| Cold-start test | **CONDITIONAL PASS — Session 5, 2026-08-25, @ `766a095`; awaiting Al's ruling** | Session 4 attempt DISQUALIFIED (memory held state; HANDOFF Session 4). Session 5: memory purged by Al (recalled content disclosed verbatim, hazards + location only); §4.1 re-derivation complete without guessing; build ALL GREEN; CI success at tip; all nine cited SHAs on `main`; digests, files, F8 verbatim, EOLs all as claimed; doc defects found and fixed (HANDOFF Session 5 step 4). Condition: ADR-0003's unratified corollary (same-conversation sessions are not cold) — strike ⇒ PASS, ratify ⇒ void |

## Blockers

- **Al's ruling on the ADR-0003 executor corollary** decides whether Gate F0 closes on Session 5's
  evidence or the exam is repeated in a new conversation. (Memory purge: DONE by Al 2026-08-25.)

## Standing escalations (awaiting Al)

- **Rule on the ADR-0003 corollary** — strike or ratify (see Blockers).
- **Delete `MEMORY.md.bak` and `project_friedman.md.bak`** from the executor's memory directory:
  pre-purge copies with program state; not loaded by the harness, but a recall hazard. Memory
  write, so not done by an exam session (ADR-0003 D3).
- **Repository `LICENSE` file (ADR-0002 consequences).** Public repo, no license file; choosing one
  is public-facing/legal (§4.6). Not blocking F0–F3; load-bearing at F4.4 and for any reuse of
  `engine/`. Until decided, crates carry no `license` field.

## NEXT-SINGLE-ACTION

**Conditional on Al's ruling, recorded in the resume prompt or as an ADR-0003 amendment:**

- **Corollary struck** → the next session's first act: mark the Cold-start row above **CLOSED —
  Gate F0 passed 2026-08-25 (Session 5) @ `766a095`, ruling by Al on <date>**, set Phase to
  **F1 — REPRODUCTIONS**, and proceed to **F1.1 (R1 — Zipf collapse)**, whose first step is
  "verify on first use" (F0.2): retrieve the Miller random-typing source and the Li random-typing
  source, record each in SOURCES.md (citation, URL, access date, license, SHA-256 of any local
  copy; publisher 403s are a known hazard — arXiv/JSTOR/author pages may be needed), and seed
  CLAIM-0004 (the Zipf-collapse result, UNKNOWN until read) before any generator code. Then set
  the next NEXT-SINGLE-ACTION to the R1 design (which generator produces the random-typing text,
  which Zipf detectors, what "weak discrimination" means as a pre-declared number).
- **Corollary ratified** → repeat the exam in a **new conversation** by exactly this procedure:
  (0) first act, reproduce every recalled memory verbatim in a new HANDOFF entry; if any of it is
  program state, or if Al has ratified the corollary and the session is in a builder's
  conversation, record DISQUALIFIED and stop; (1) ORIENT exactly as §4.1 lists, then read STATE →
  HANDOFF (last entry) → CLAIMS → ADR-0000..0004; (2) `bash scripts/check.sh` must print `ALL
  GREEN`, and `gh run list -R Quigles1337/FR13DMAN --branch main --limit 3` must show `check`
  succeeding for `main`'s tip; (3) confirm each Gate F0 row's evidence link (commit on `main` via
  `git merge-base --is-ancestor`, file exists, digest matches — `PROGRAM/SWEEP-2026-08.md`
  against `6ba3eb34…`); (4) write down every point where the repo alone was insufficient — any
  guess fails the gate; (5) on pass, mark Gate F0 CLOSED with date and SHA, open Phase F1, and set
  NEXT-SINGLE-ACTION to the F1.1 first step above. No memory writes (ADR-0003 D3).
