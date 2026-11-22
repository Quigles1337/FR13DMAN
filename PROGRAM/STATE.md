# STATE

- **Updated:** 2026-11-22 UTC (Session 6, close)
- **Phase:** F0 — FOUNDATION
- **Gate:** F0 (**open — the cold-start test has not been passed**). Al ruled on 2026-11-22 UTC
  (R1–R7, verbatim below) on the state at `7c3e5dd`. ADR-0003's corollary is ratified, so
  Session 5's attempt is **void as an examination**; its evidence that the documents can be
  walked, and its document fixes, stand. Gate F0 stays open until a cold session passes. Session 6
  recorded the rulings and built the state the exam will test; it is not cold and ran no exam.
- **Default branch:** `main`. Remote: `origin` = `https://github.com/Quigles1337/FR13DMAN`
  (**public**; created and designated by Al 2026-08-22 — see ADR-0000).
  **ORIENT note (F7):** `git symbolic-ref refs/remotes/origin/HEAD` failed on this working copy at
  the start of Session 2 ("not a symbolic ref") and resolved after `git fetch --all --prune`.
  Source-of-truth fallback: `git remote show origin` (prints `HEAD branch: main`). Do not guess.
- **Launch (R7):** from now on Al launches FRIEDMAN sessions from the repository directory (ruled
  2026-11-22 UTC). Session 6 itself was started in `C:\Users\LEET`.
- **Build status:** **green** = `bash scripts/check.sh` prints `ALL GREEN` (ADR-0002 D6: fmt
  --check + `cargo test` + clippy `-D warnings` + lab verifier). Last verified locally 2026-11-22
  UTC, at `7c3e5dd` and again on Session 6's edited tree before its commit (16/16 tests; verifier
  `RESULT: ALL PASS`). CI: `.github/workflows/check.yml` runs the same script on push to `main`
  and on PRs; last known CI success on `main` @ `7c3e5dd` (run 32890045263). Check `gh run list
  -R Quigles1337/FR13DMAN --branch main` at ORIENT; this line lags the last push by one run.
  **Never make the workflow a required status check** (ADR-0002 D6).
- **Toolchain (pinned, ADR-0002 D1):** `rust-toolchain.toml` at repo root → `1.91.0` + clippy +
  rustfmt; edition 2024. Observed alongside, not pinned (re-observed 2026-11-22 UTC): Python
  3.11.9 (lab verifier), `gh 2.74.0` authenticated as Quigles1337, cargo-audit 0.22.2.

## Gate F0 exit criteria

| Criterion | Status | Evidence |
|---|---|---|
| F0.1 sweep report at `PROGRAM/SWEEP-2026-08.md` with F11 banner | **done** | Ferried by Al 2026-08-25; identity verified against Al's stated digest before the move (SHA-256 `6ba3eb341ba2ecd325b68eb31d4bfafcbb46a47e6669c4d98374ec771e56275c`, 35,207 bytes, LF-only UTF-8, no BOM); F11 banner at line 1; ferry note at line 129 records that the sweep's "publicly released" claim about Nair's generators is superseded by S-0001. Commit `6c97b53`. Re-verified from the repo in Sessions 4 and 5 |
| F0.2 CLAIM-0001..0003 resolved VERIFIED or honestly UNKNOWN with consequences | **done (core)** | all three VERIFIED 2026-08-22 (commits `6649990`, `bee91d9`; S-0001..S-0003). "Verify on first use" list (Miller/Li, Ferrer-i-Cancho, Takahashi & Tanaka-Ishii, Rugg) remains open by design |
| F0.3 manifest schema ADR merged | **done** | ADR-0001 (`7288a49`) + verifier (`2e9bef0`); enforced by the canonical tier (`27dd471`); comparison arithmetic pinned by ADR-0004 (`a3f42d2`) with seven conformance vectors identical in Rust and Python |
| F0.4 scaffold (`engine/`, `lab/`, `corpora/`, `gallery/`, `quarantine/` + F8 README, CI) | **done** | ADR-0002 (`27dd471`): `rust-toolchain.toml` 1.91.0; `engine/` workspace, crate `friedman-manifest` (16 tests incl. RFC 8785 §3.2 sample bytes and all 24 finite Appendix B rows); `scripts/check.sh` gate; `.github/workflows/check.yml` (actions at v5, `91faff1`); `corpora/` `gallery/` `quarantine/` READMEs — F8 quotation checked verbatim programmatically in Session 5; `.gitignore`. CI success on `main` @ `766a095` |
| Licenses recorded for anything touched | **done for everything touched so far** | S-0001..S-0004 (papers/RFC: hashed, not committed; S-0004 digest re-reproduced from the URL in Session 5); S-0005 (all 24 locked crates, licenses before ingestion; `cargo audit` exit 0 re-run 2026-08-25 Session 5). Standing rule: new crate ⇒ S-0005 row in the same commit |
| Cold-start test | **OPEN — not passed.** Session 5's attempt (2026-08-25, @ `766a095`) is **void as an examination** — Al's ruling R1, 2026-11-22 UTC | Session 4 attempt DISQUALIFIED (memory held state; HANDOFF Session 4). Session 5 attempt void: it ran in the conversation context that built the state under test (ADR-0003, Amendment 1, R1); its evidence that the documents can be walked, and its document fixes, stand (HANDOFF Session 5). The row closes only when a cold session passes: a new conversation launched with the §6 Session-N prompt verbatim and nothing that carries program state — the NEXT-SINGLE-ACTION below |

## Blockers

- None that the executor can clear. The one remaining Gate F0 criterion is the cold-start exam,
  which only a cold session can run, and only Al can launch one (NEXT-SINGLE-ACTION).

## Al's rulings of 2026-11-22 UTC (verbatim)

Given in the message that launched Session 6, on the state at `7c3e5dd`, and copied here by script
from the harness's record of that message, not typed. The whole message, with its work order, is
in HANDOFF (Session 6). R1–R4 are implemented in ADR-0003 (Amendment 1) and in the constitution's
§6 and header; R5–R7 are implemented in this file. Everywhere in this file "R1" to "R7" mean
these rulings.

```text
R1 (ADR-0003, the corollary). Ratified. A session running in the conversation context that built the
state under test is not cold, whatever its memory holds, and its exam attempt does not count. I
extend it as SYNDEOM's SD15 states it: a cold session is launched with the §6 Session-N prompt
verbatim and nothing that carries program state. Session 5's attempt is therefore void as an
examination. Its evidence that the documents can be walked, and its document fixes, stand. Gate F0
stays open until a cold session passes.

R2 (disclosure first; amends ADR-0003 D2 and the constitution's §6). Every session launched with the
§6 Session-N prompt makes the D2 disclosure, whether or not it will attempt a cold-start test, as
the first entry of its HANDOFF block, before any action except reading the constitution and ADR-0003
in full and reading the memory it discloses. It appends the disclosure to PROGRAM/HANDOFF.md without
reading that file's existing contents. Reason: D2 says "before any other action", and a cold session
learns that it is an exam only from STATE, after it has acted. SYNDEOM made every session launched
with its Session-N prompt disclose first (Syndeom ADR-0009). This adopts that order and nothing else
of SYNDEOM's memory or orientation records.

R3 (what a disclosure holds; amends ADR-0003 D2). The disclosure covers the memory the harness loads
for the session, and the session opens no other store. It reproduces verbatim every recalled line
that names FRIEDMAN, FR13DMAN, KOBER or SYNDEOM, and this program's own memory file if the session
recalls or opens it. Every other line is withheld and given by count: this repository is public, and
a line about any other program is not this repository's to publish. A verbatim reproduction is
copied from the file by script, never typed.

R4 (the new §6 Session-N prompt). Replace the Session-N prompt in §6 with the text between the two
marker lines below, verbatim and without the marker lines, and note the amendment in the
constitution's header as SYNDEOM's constitution notes its own.

BEGIN SESSION-N PROMPT
Before anything else, make the memory disclosure that ADR-0003 requires: read
PROGRAM/FRIEDMAN-PROGRAM.md and PROGRAM/DECISIONS/ADR-0003.md in full, read the executor memory they
tell you to disclose, and append the disclosure as the first entry of a new block at the end of
PROGRAM/HANDOFF.md, whether or not this session will attempt a gate's cold-start test, and with no
other command or file read first. Append it without reading HANDOFF's existing contents (a shell
append or a script, not a file-editing tool that reads the file first), under a provisional block
header that carries the UTC date and that you complete after orientation without changing the
disclosure. You are the executor of the FRIEDMAN program. Having disclosed, run the metacognitive
loop (§4) exactly: ORIENT from the repo (state files are claims; builds are evidence), ASSESS, PLAN
one thing, EXECUTE under the §1 constitution, VERIFY, RECORD+REFLECT per the §4.5 schema — including
the LEAKAGE, DETERMINISM, and DRIFT checks — and stop clean per §4.6. Honor escalations — some
decisions are Al's, not yours.
END SESSION-N PROMPT

R5 (the pre-purge copies). In the executor memory directory that holds project_friedman.md, which I
believe is C:\Users\LEET\.claude\projects\C--Users-LEET\memory, delete MEMORY.md.bak,
project_friedman.md.bak and project_kober.md.bak if they exist. Confirm that project_friedman.md is
in that directory before you touch anything. Do not open the three files. Record each one's name,
size and SHA-256 before deleting it, and nothing about any other file there. This is a memory write
that I order, and this session is not an exam. The third file is KOBER's residue; deleting it is my
housekeeping on this machine and no decision about KOBER.

R6 (licence). The PolyForm Noncommercial License 1.0.0, under "Required Notice: Copyright 2026
Alexander David Zalewski (https://github.com/Quigles1337/FR13DMAN)", following Syndeom ADR-0010 D1
and D2 so far as they apply to FRIEDMAN. It is implemented by an ADR of its own, in a session I ask
for after Gate F0 closes. Ask me then for the commercial-licence contact line; do not copy it from
SYNDEOM. Until then there is no LICENSE file and no license field.

R7 (launch). From now on I launch FRIEDMAN sessions from the repository directory. Record it in
STATE.
```

## Standing escalations — as ruled

- **ADR-0003 corollary — ruled (R1): ratified, and extended to the launch.** Recorded in
  ADR-0003, Amendment 1. Session 5's exam attempt is void; Gate F0 stays open. **Closed.**
- **Pre-purge memory copies — ruled (R5); carried out, and nothing was left to delete.** On
  2026-11-22 UTC Session 6 confirmed that `project_friedman.md` is in the directory R5 names, then
  tested for `MEMORY.md.bak`, `project_friedman.md.bak` and `project_kober.md.bak` by name: all
  three were absent, by two methods. Nothing was deleted or opened, so there is no size or SHA-256
  to record (HANDOFF Session 6). Session 5 reported the first two present on 2026-08-25; this repo
  does not record who removed them. **Closed.**
- **Licence — ruled (R6); not yet implemented, by the ruling's own terms.** The PolyForm
  Noncommercial License 1.0.0 under the Required Notice that R6 states, following Syndeom ADR-0010
  D1 and D2 so far as they apply to FRIEDMAN. It is implemented by an ADR of its own, in a session
  Al asks for after Gate F0 closes; that session asks Al for the commercial-licence contact line
  and does not copy SYNDEOM's. Until then there is no `LICENSE` file and no crate carries a
  `license` field. **Open — waits for Al's request after Gate F0 closes.**

## NEXT-SINGLE-ACTION

**The Gate F0 cold-start exam**, in a **new conversation** that Al launches with the amended §6
Session-N prompt **verbatim and nothing else**. No other session can run it: one that reads this
in the conversation that wrote it, or that was launched with any other text, is not cold
(ADR-0003, Amendment 1, R1) and does not attempt the exam.

**The launch directory is not a condition of the exam.** Launching from the repository directory
(R7) is Al's practice. A session launched from another directory is no less cold for it. Al said
so on 2026-11-22 UTC in accepting Session 6's diff; his answer is in HANDOFF (Session 6),
verbatim.

The procedure is the one STATE gave at `7c3e5dd` for a ratified corollary. Step (0) is brought
into line with ADR-0003's amended D2 and carries Al's addition of 2026-11-22 UTC; step (1) says
what the session's own disclosure changes; steps (2) to (5) are as they were.

- **(0) Standing.** The session has already appended its memory disclosure: the §6 prompt orders
  it before this file is read. Below the disclosure, without changing it, the session records the
  launching message it received, verbatim, and whether any turn preceded it. It records
  **DISQUALIFIED** and stops if the message carries anything beyond §6's text, or a prior turn
  exists (Al's addition); if any disclosed memory is program state (ADR-0003 D2); or if the
  session is running in a conversation context that built this state (R1). A disqualified session
  does not go on with the exam and does not close the gate.

  The executor's readings of this step, shown to Al before the commit:
  - *The launching message is what Al sent as the first user turn.* What the harness attaches
    around it is not part of it. In Session 6 the harness delivered Al's pasted text inside a
    paste wrapper and attached context blocks of its own, the loaded memory among them. The
    session says that such attachments were present and reproduces memory only as D2 allows.
  - *A difference from §6's text in line breaks or surrounding whitespace alone is not content.*
    Compare the message with the Session-N block in §6 of the constitution and record the result.
  - *A breach of the disclosure order is Al's to judge.* If the session ran a command or read a
    file before disclosing, other than what the prompt allows, it says so here, does not mark the
    gate CLOSED, and escalates: the rulings do not say what such a breach costs.
- **(1)** ORIENT exactly as §4.1 lists — `git status` will show `PROGRAM/HANDOFF.md` modified by
  the session's own append, and should show nothing else — then read STATE → HANDOFF (the last
  entry before the session's own block: Session 6) → CLAIMS → ADR-0000..0004 (ADR-0003 with its
  Amendment 1).
- **(2)** `bash scripts/check.sh` must print `ALL GREEN`, and `gh run list -R Quigles1337/FR13DMAN
  --branch main --limit 3` must show `check` succeeding for `main`'s tip.
- **(3)** Confirm each Gate F0 row's evidence link (commit on `main` via `git merge-base
  --is-ancestor`, file exists, digest matches — `PROGRAM/SWEEP-2026-08.md` against `6ba3eb34…`).
- **(4)** Write down every point where the repo alone was insufficient — any guess fails the gate.
- **(5)** On pass, mark Gate F0 CLOSED with date and SHA, open Phase F1, and set
  NEXT-SINGLE-ACTION to the F1.1 first step below. No memory writes (ADR-0003 D3).

**The F1.1 first step** (not the next action until Gate F0 is CLOSED): F1.1 — the Zipf-collapse
reproduction, which the constitution's §2 also labels "R1" and which has nothing to do with ruling
R1 — begins with "verify on first use" (F0.2): retrieve the Miller random-typing source and the Li
random-typing source, record each in SOURCES.md (citation, URL, access date, license, SHA-256 of
any local copy; publisher 403s are a known hazard — arXiv/JSTOR/author pages may be needed), and
seed CLAIM-0004 (the Zipf-collapse result, UNKNOWN until read) before any generator code. Then set
the next NEXT-SINGLE-ACTION to the design of that reproduction (which generator produces the
random-typing text, which Zipf detectors, what "weak discrimination" means as a pre-declared
number).
