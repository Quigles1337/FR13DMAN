# STATE

- **Updated:** 2026-08-25 (Session 4, close)
- **Phase:** F0 — FOUNDATION
- **Gate:** F0 (**open**). Every exit criterion has evidence except the cold-start test. Session 4
  attempted the test and **disqualified itself** under ADR-0003: the recalled memory contained
  program state, and the session ran in the same conversation context that built Sessions 2–3.
  Its warm rehearsal of the procedure found every evidence link sound (see HANDOFF, Session 4),
  which is encouraging but is not the gate.
- **Default branch:** `main`. Remote: `origin` = `https://github.com/Quigles1337/FR13DMAN`
  (**public**; created and designated by Al 2026-08-22 — see ADR-0000).
  **ORIENT note (F7):** `git symbolic-ref refs/remotes/origin/HEAD` failed on this working copy at
  the start of Session 2 ("not a symbolic ref") and resolved after `git fetch --all --prune`.
  Source-of-truth fallback: `git remote show origin` (prints `HEAD branch: main`). Do not guess.
- **Build status:** **green** = `bash scripts/check.sh` prints `ALL GREEN` (ADR-0002 D6: fmt
  --check + `cargo test` + clippy `-D warnings` + lab verifier). Last verified locally 2026-08-25 at
  `91faff1` (16/16 tests; verifier 44 checks). CI: `.github/workflows/check.yml` runs the same
  script on push to `main` and on PRs; last known CI success on `main` @ `f992a78` (run
  32886443172). Check `gh run list -R Quigles1337/FR13DMAN --branch main` at ORIENT; this line
  lags the last push by one run. **Never make the workflow a required status check** (ADR-0002 D6).
- **Toolchain (pinned, ADR-0002 D1):** `rust-toolchain.toml` at repo root → `1.91.0` + clippy +
  rustfmt; edition 2024. Observed alongside, not pinned: Python 3.11.9 (lab verifier), `gh 2.74.0`
  authenticated as Quigles1337, cargo-audit 0.22.2.

## Gate F0 exit criteria

| Criterion | Status | Evidence |
|---|---|---|
| F0.1 sweep report at `PROGRAM/SWEEP-2026-08.md` with F11 banner | **done** | Ferried by Al 2026-08-25; identity verified against Al's stated digest before the move (SHA-256 `6ba3eb341ba2ecd325b68eb31d4bfafcbb46a47e6669c4d98374ec771e56275c`, 35,207 bytes, LF-only UTF-8, no BOM); F11 banner at line 1; ferry note at line 129 records that the sweep's "publicly released" claim about Nair's generators is superseded by S-0001. Commit `6c97b53`. Re-verified from the repo in Session 4 |
| F0.2 CLAIM-0001..0003 resolved VERIFIED or honestly UNKNOWN with consequences | **done (core)** | all three VERIFIED 2026-08-22 (commits `6649990`, `bee91d9`; S-0001..S-0003). "Verify on first use" list (Miller/Li, Ferrer-i-Cancho, Takahashi & Tanaka-Ishii, Rugg) remains open by design |
| F0.3 manifest schema ADR merged | **done** | ADR-0001 (`7288a49`) + verifier (`2e9bef0`); enforced by the canonical tier (`27dd471`); comparison arithmetic pinned by ADR-0004 (`a3f42d2`) with seven conformance vectors identical in Rust and Python |
| F0.4 scaffold (`engine/`, `lab/`, `corpora/`, `gallery/`, `quarantine/` + F8 README, CI) | **done** | ADR-0002 (`27dd471`): `rust-toolchain.toml` 1.91.0; `engine/` workspace, crate `friedman-manifest` (16 tests incl. RFC 8785 §3.2 sample bytes and all 24 finite Appendix B rows); `scripts/check.sh` gate; `.github/workflows/check.yml` (actions bumped to v5 at `91faff1`); `corpora/` `gallery/` `quarantine/` READMEs (F8 verbatim); `.gitignore`. CI success on `main` @ `f992a78` |
| Licenses recorded for anything touched | **done for everything touched so far** | S-0001..S-0004 (papers/RFC: hashed, not committed); S-0005 (all 24 locked crates, licenses before ingestion, `cargo audit` clean 2026-08-25). Standing rule: new crate ⇒ S-0005 row in the same commit |
| Cold-start test | **open — the NEXT-SINGLE-ACTION** | Session 4 attempt DISQUALIFIED (ADR-0003; HANDOFF Session 4). Must be run by a fresh session in a new conversation, after the memory purge below |

## Blockers

- **Memory purge precondition for the exam (ADR-0003 consequences).** The executor's machine-local
  memory (`project_friedman.md` and the FR13DMAN line of the memory index) still holds program
  state. It must be reduced to environment/tooling hazards + repo location *before* the next exam
  session starts, and the exam session itself must not do it (ADR-0003 D3; recall precedes action).
  Owner: Al on the machine, or a dedicated non-exam session whose only act is the purge.

## Standing escalations (awaiting Al)

- **Memory purge (above)** — perform it, or authorize a dedicated purge session.
- **ADR-0003 executor corollary** — "a session in the same conversation context that built the
  state is not cold, regardless of memory" — recorded as not yet ratified; confirm or strike.
- **Repository `LICENSE` file (ADR-0002 consequences).** Public repo, no license file; choosing one
  is public-facing/legal (§4.6). Not blocking F0–F3; load-bearing at F4.4 and for any reuse of
  `engine/`. Until decided, crates carry no `license` field.

## NEXT-SINGLE-ACTION

**Gate F0 cold-start exam, by a fresh session in a new conversation, under ADR-0003**, after the
memory purge above is confirmed done. Procedure: (0) as the first act, reproduce every recalled
memory verbatim in a new HANDOFF entry and state whether any of it is program state — if yes, or
if the session is not cold, record DISQUALIFIED and stop; (1) ORIENT exactly as §4.1 lists
(`git rev-parse --show-toplevel`, `git remote -v`, `git status`, `git log --oneline -20`, default
branch via `git symbolic-ref` with the `git remote show origin` fallback); read STATE → HANDOFF
(last entry) → CLAIMS → ADR-0000..0004; (2) verify the build claim: `bash scripts/check.sh` must
print `ALL GREEN` on the pinned toolchain, and `gh run list -R Quigles1337/FR13DMAN --branch main
--limit 3` must show the `check` workflow succeeding for `main`'s tip (if Actions did not run or
failed, that is a finding to fix before the gate, not a reason to skip it); (3) confirm each Gate
F0 row's evidence link resolves to what it claims (commit on `main` via `git merge-base
--is-ancestor`, file exists, digest matches — `PROGRAM/SWEEP-2026-08.md` against `6ba3eb34…`);
(4) write down every point where the repo alone was insufficient — each is a doc fix, and any
guess fails the gate; (5) if it passes, mark Gate F0 **CLOSED** in this table with the session
date and the SHA tested, open Phase F1, and set the next NEXT-SINGLE-ACTION to F1.1 (R1 — Zipf
collapse), whose first step is to retrieve and verify the Miller / Li random-typing sources into
SOURCES.md ("verify on first use", F0.2) before any generator code. No memory writes in that
session (ADR-0003 D3).
