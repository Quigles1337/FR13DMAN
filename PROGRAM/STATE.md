# STATE

- **Updated:** 2026-11-22 UTC (Session 8, close: Al's rulings A–D on Session 7, then the licence
  ADR, then the fifth note of ADR-0003's Amendment 1)
- **Phase:** F1 — REPRODUCTIONS (opened 2026-11-22 UTC, when Gate F0 closed)
- **Gate:** F1 (**open**; nothing started). **Gate F0 is CLOSED — 2026-11-22 UTC, at `edcee64`** —
  by the cold-start exam Session 7 ran: a new conversation launched with the §6 Session-N prompt
  and nothing else (HANDOFF, Session 7). The closing rested on that session's own readings of
  its standing, which were Al's to overrule. **Al ruled on them on 2026-11-22 UTC (rulings A–D,
  below): the closing stands.**
- **Default branch:** `main`. Remote: `origin` = `https://github.com/Quigles1337/FR13DMAN`
  (**public**; created and designated by Al 2026-08-22 — see ADR-0000).
  **ORIENT note (F7):** `git symbolic-ref refs/remotes/origin/HEAD` failed on this working copy at
  the start of Session 2 ("not a symbolic ref") and resolved after `git fetch --all --prune`.
  Source-of-truth fallback: `git remote show origin` (prints `HEAD branch: main`). Do not guess.
- **Launch (R7):** Al launches FRIEDMAN sessions from the repository directory (ruled 2026-11-22
  UTC). It is his practice and not a condition of an exam (his answer, HANDOFF Session 6).
  Sessions 6, 7 and 8 were all started in `C:\Users\LEET`; Sessions 7 and 8 found the repo from
  the location line in the executor's memory, which ADR-0003 D1 allows.
- **Licence:** the **PolyForm Noncommercial License 1.0.0** (SPDX `PolyForm-Noncommercial-1.0.0`)
  for FRIEDMAN's original software and technical documentation, from the commit that adds
  ADR-0005 (ruled by Al, R6; that commit was made only after he accepted the ADR). `LICENSE` at
  the repo root opens with the Required Notice line R6 gives; README § Licence has a
  plain-language summary and Al's contact line for commercial licensing; the crate carries
  `license = "PolyForm-Noncommercial-1.0.0"`, and a crate added later carries the same value;
  the provenance of the text is S-0006. Third-party material keeps its own licence (ADR-0005
  D2). FRIEDMAN carried no licence before that commit: every commit from `eced49d` through
  `92367a6` was published without one, and none is granted for them (ADR-0005 D3).
- **Build status:** **green** = `bash scripts/check.sh` prints `ALL GREEN` (ADR-0002 D6: fmt
  --check + `cargo test` + clippy `-D warnings` + lab verifier). Last verified locally 2026-11-22
  UTC by Session 8: at `31e503f` with its HANDOFF append in the tree, on the tree of `92367a6`,
  on the tree of `eb572ea` (ADR-0005, with the crate's new `license` field), and on the tree of
  the commit that adds the fifth note to ADR-0003 (16/16 tests; verifier `RESULT: ALL PASS`).
  CI: `.github/workflows/check.yml` runs the same script on push to `main` and on PRs; last
  known CI success on `main` @ `eb572ea` (run 45051459260; one job, 8 steps). Check
  `gh run list -R Quigles1337/FR13DMAN --branch main` at ORIENT; this line lags the last push by
  one run.
  **Never make the workflow a required status check** (ADR-0002 D6). On 2026-11-22 UTC `main` had
  no branch protection and the repository no ruleset (GitHub API).
- **Toolchain (pinned, ADR-0002 D1):** `rust-toolchain.toml` at repo root → `1.91.0` + clippy +
  rustfmt; edition 2024. Observed alongside, not pinned (re-observed 2026-11-22 UTC by Session
  7): Python 3.11.9 (lab verifier), `gh 2.74.0` authenticated as Quigles1337, cargo-audit 0.22.2.

## Gate F1 exit criteria

The constitution's §2 labels the three reproductions "R1", "R2" and "R3". This file calls them
F1.1, F1.2 and F1.3, because here "R1" to "R7" mean Al's rulings below.

| Criterion | Status | Evidence |
|---|---|---|
| F1.1 Zipf collapse — a REPRODUCED claim with seeds + SHAs | not started | NEXT-SINGLE-ACTION is its first step. The sources (Miller; Li) are on F0.2's "verify on first use" list and have not been retrieved |
| F1.2 MI decay — a REPRODUCED claim with seeds + SHAs | not started | The source claim, CLAIM-0002, is VERIFIED (S-0002); the caveats F1.2 must carry are in that entry |
| F1.3 scorecard → ROC — a REPRODUCED claim with seeds + SHAs | not started — **known obstacle** | CLAIM-0001: on 2026-08-22 Nair's generators were not publicly released. The entry lists the three ways through; contacting the author is Al's decision (§4.6). Re-check the release before anything else. Indus-derived data goes to `quarantine/` under F8's frozen-as-published exemption and informs no design |
| Battery's qualitative detector ranking consistent with the literature (conditional entropy and MI-decay strong, Zipf-slope weak) | not started | — |
| Discrepancies documented | nothing to document yet | — |
| Cold-start test | not attempted | *The cold-start exam procedure*, at the end of this file |

## Gate F0 exit criteria — CLOSED 2026-11-22 UTC at `edcee64`

Rows one to five are as Session 6 left them. Session 7 confirmed every one of them from the repo
on 2026-11-22 UTC (HANDOFF Session 7, exam record, step (3)).

| Criterion | Status | Evidence |
|---|---|---|
| F0.1 sweep report at `PROGRAM/SWEEP-2026-08.md` with F11 banner | **done** | Ferried by Al 2026-08-25; identity verified against Al's stated digest before the move (SHA-256 `6ba3eb341ba2ecd325b68eb31d4bfafcbb46a47e6669c4d98374ec771e56275c`, 35,207 bytes, LF-only UTF-8, no BOM); F11 banner at line 1; ferry note at line 129 records that the sweep's "publicly released" claim about Nair's generators is superseded by S-0001. Commit `6c97b53`. Re-verified from the repo in Sessions 4 and 5 |
| F0.2 CLAIM-0001..0003 resolved VERIFIED or honestly UNKNOWN with consequences | **done (core)** | all three VERIFIED 2026-08-22 (commits `6649990`, `bee91d9`; S-0001..S-0003). "Verify on first use" list (Miller/Li, Ferrer-i-Cancho, Takahashi & Tanaka-Ishii, Rugg) remains open by design |
| F0.3 manifest schema ADR merged | **done** | ADR-0001 (`7288a49`) + verifier (`2e9bef0`); enforced by the canonical tier (`27dd471`); comparison arithmetic pinned by ADR-0004 (`a3f42d2`) with seven conformance vectors identical in Rust and Python |
| F0.4 scaffold (`engine/`, `lab/`, `corpora/`, `gallery/`, `quarantine/` + F8 README, CI) | **done** | ADR-0002 (`27dd471`): `rust-toolchain.toml` 1.91.0; `engine/` workspace, crate `friedman-manifest` (16 tests incl. RFC 8785 §3.2 sample bytes and all 24 finite Appendix B rows); `scripts/check.sh` gate; `.github/workflows/check.yml` (actions at v5, `91faff1`); `corpora/` `gallery/` `quarantine/` READMEs — F8 quotation checked verbatim programmatically in Session 5; `.gitignore`. CI success on `main` @ `766a095` |
| Licenses recorded for anything touched | **done for everything touched so far** | S-0001..S-0004 (papers/RFC: hashed, not committed; S-0004 digest re-reproduced from the URL in Session 5); S-0005 (all 24 locked crates, licenses before ingestion; `cargo audit` exit 0 re-run 2026-08-25 Session 5). Standing rule: new crate ⇒ S-0005 row in the same commit |
| Cold-start test | **passed 2026-11-22 UTC** by Session 7, at `edcee64`; **the pass stands by Al's rulings A–C** | A new conversation launched with the §6 Session-N prompt and nothing else; memory disclosed first, as the prompt orders; launching message compared with §6 by script; no prior turn; `ALL GREEN`; CI `success` for the tip; every row above confirmed; no value of program state guessed or taken from memory (HANDOFF Session 7). Al's rulings of 2026-11-22 UTC on the session's own readings: the disclosure order was not a breach (A), the memory reading is accepted (B), the four places where the repo did not say how are accepted (C). Earlier attempts: Session 4 DISQUALIFIED (memory held state); Session 5 void (R1) |

What Session 7 added to the evidence above, all on 2026-11-22 UTC: CLAIM-0003's transition and
S-0003 are in `bee91d9`, whose subject names only CLAIM-0002, so the two commits the F0.2 row
cites do cover all three claims; the stored-copy digests of S-0001, S-0002 and S-0004 reproduce
from their URLs, byte counts included; S-0003's upstream is still at the pinned commit under MIT;
the S-0005 table equals `engine/Cargo.lock`'s 24 registry crates by name and version (script);
`cargo audit` exit 0 with 1,555 advisories loaded (1,226 in August).

## Blockers

- None.

## Al's rulings R1–R7 of 2026-11-22 UTC (verbatim)

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

## Al's rulings A–D on Session 7, 2026-11-22 UTC (verbatim)

Given in the message that launched Session 8 (received 2026-11-22T17:04:30Z by the harness's
record), on Session 7's exam and on the state at `31e503f`. Copied here by script from the
harness's record of that message, not typed; each ruling is one line, as Al sent it. The whole
message is in HANDOFF (Session 8). The letters A to D mean these rulings everywhere in this file.

```text
A. The disclosure order. Not a breach. The prompt allows a script, the script read none of the file's bytes, and Session 7 learned the size only after the append was written. The closing of Gate F0 stands. From now on the append asks the file system nothing, as Session 7's note 2 has it.
B. The memory reading. Accepted, all four calls. None of them is program state.
C. The four places where the repo did not say how. Accepted. None was a guess about the program.
D. Session 7's four notes on the exam procedure stay, with one addition to note 3, in my words: a session opens its own conversation record and no other, by its exact name, and lists nothing in that directory. If the record is not where the note puts it, the session does not look for it: it records the launching message from its own context, states that it did, and I judge. A session that opens another conversation's record has taken on that conversation's context and is not cold (R1).
```

What the rulings change, in the executor's words. Where these and the rulings differ, the
rulings govern.

- **A.** Session 7's query of `PROGRAM/HANDOFF.md`'s existence and size, made by its append
  script before the disclosure was written, is not a breach of the disclosure order. The closing
  of Gate F0 stands. From now on the append asks the file system nothing (note 2 of *Executor's
  notes from Session 7*, below).
- **B.** Session 7's reading that its disclosed memory holds no program state is accepted, all
  four of the closest calls it named (HANDOFF Session 7, STANDING (f)).
- **C.** The four places where the repo did not say how to do a thing are accepted (HANDOFF
  Session 7, exam record, item (4)): none was a guess about the program.
- **D.** Session 7's four notes on the exam procedure stay, at the end of this file. Note 3
  carries Al's addition, in his words.

## Standing escalations

- **Open:** none.
- **Closed by Al's ruling of 2026-11-22 UTC (his answer to Session 8, item 5), recorded in
  HANDOFF Session 8:** the disclosure append. Ruling A orders that "from now on the append asks
  the file system nothing, as Session 7's note 2 has it". Note 2 is in this file, and a session
  launched with the §6 Session-N prompt appends its disclosure before it may read this file.
  Session 8's append asked the file system nothing, but its call would have created the file
  had it been missing (HANDOFF Session 8, LAUNCH (e)). Al ordered a fifth note in ADR-0003's
  Amendment 1, in his words, as a commit of its own. It is there: ADR-0003 is read before the
  append, so the rule now reaches a session in time.
- **Closed by ADR-0005 (2026-11-22 UTC), recorded in HANDOFF Session 8:** the licence (R6). Al
  asked for the ADR in the message that launched Session 8 and gave the commercial-licence
  contact line there. Session 8 prepared it in the working tree and showed it to him. He
  accepted it with changes, which were made before the commit: SYNDEOM's D2(b) is not followed
  and was struck, with the README sentence that mirrored it; D3 and the README say that no
  licence is granted for earlier versions; of the executor's five readings, 1, 3, 4 and 5
  stand and 2 is changed. His answer is in HANDOFF (Session 8) whole, and in ADR-0005 as far
  as it concerns the licence. The *Licence* bullet at the top of this file is the result.
- **Closed by rulings A–D (2026-11-22 UTC), recorded in HANDOFF Session 8:** Session 7's
  disclosure order (A: not a breach; the closing of Gate F0 stands); Session 7's reading of its
  memory (B: accepted); the four how-to points (C: accepted); Session 7's four notes on the exam
  procedure (D: they stay, with Al's addition to note 3).
- **Closed, recorded in HANDOFF Session 6:** the ADR-0003 corollary (R1: ratified and extended to
  the launch) and the pre-purge memory copies (R5: carried out; nothing was left to delete).

## NEXT-SINGLE-ACTION

**F1.1, first step: retrieve the two random-typing sources and seed CLAIM-0004.** F1.1 is the
Zipf-collapse reproduction. It begins with "verify on first use" (F0.2): retrieve the Miller
random-typing source and the Li random-typing source, record each in SOURCES.md (citation, URL,
access date, license, SHA-256 of any local copy; publisher 403s are a known hazard —
arXiv/JSTOR/author pages may be needed), and seed CLAIM-0004 (the Zipf-collapse result, UNKNOWN
until read) before any generator code. `PROGRAM/SWEEP-2026-08.md` may say where to look; it is a
map, not evidence, and nothing enters a register from it or from executor memory (F11). Then set
the next NEXT-SINGLE-ACTION to the design of that reproduction (which generator produces the
random-typing text, which Zipf detectors, what "weak discrimination" means as a pre-declared
number).

A session launched with the §6 Session-N prompt makes its memory disclosure first, whatever its
task (ADR-0003, Amendment 1). The session that does this step is not an exam.

## The cold-start exam procedure

Every gate ends in a cold-start test (§2). This is the procedure Gate F0's exam ran, kept so that
the next gate's exam does not have to recover it from history. Steps (0) to (5) are the text Al
accepted on 2026-11-22 UTC (HANDOFF Session 6), unchanged; they name Gate F0 and the F1.1 first
step because that was the exam they were written for. Whether a later gate's exam uses them as
they stand is Al's to say when that exam becomes the next action. Only a cold session runs an
exam: a new conversation launched with the §6 Session-N prompt verbatim and nothing that carries
program state (ADR-0003, Amendment 1, R1). The launch directory is not a condition (Al's answer,
HANDOFF Session 6).

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

**Executor's notes from Session 7** — not rulings; what the session that ran the procedure wishes
it had been told. Al has seen them and ruled that they stay, with one addition to note 3, which
is his (ruling D, 2026-11-22 UTC). The four notes are as Session 7 wrote them; what Session 8
added under notes 2 and 3 is marked.

1. *When step (0) is done.* A session learns that it is an exam, and that step (0) exists, only
   by reading this file. Session 7 ran the §4.1 commands, read STATE, did step (0), and then
   read the rest of what step (1) lists.
2. *The append in the disclosure.* Open `PROGRAM/HANDOFF.md` write-only, for appending, without
   creating it (in Python, `os.open(path, os.O_WRONLY | os.O_APPEND | os.O_BINARY)`), and ask the
   file system nothing else about it: no existence test, no size. Session 7's script took both
   and had to report it to Al.

   *Added by Session 8.* Al ruled on this (ruling A): Session 7's query was not a breach, and
   from now on the append asks the file system nothing. A session reads this note only after
   it has appended, so Al put the rule where it is read first: ADR-0003, Amendment 1, note 5,
   in his words. In Node, `fs.openSync(path, fs.constants.O_WRONLY | fs.constants.O_APPEND)`
   opens for appending without creating: tested by Session 8 on scratch files under Node
   22.23.1, it appends to an existing file and fails with `ENOENT` on a missing one, which it
   does not create. `fs.appendFileSync(path, …)` creates a missing file; Session 8 used it for
   its own append and reported it.
3. *The launching message, verbatim.* On Al's machine on 2026-11-22 UTC (harness version
   2.1.322) the harness kept its record of a conversation as one JSON-lines file, named for the
   session identifier, in the directory that holds the memory directory; the session identifier
   is the folder above `scratchpad` in the scratch path the harness names. The repo did not say
   so; Session 7 inferred it and tested it. Open that one file by name and list nothing: the
   same directory holds the records of earlier conversations, and those carry program state.
   Confirm the file by its `sessionId` field. The launching message is the first record of type
   `user`: its `parentUuid` is null and its `turnPosition` is prompt 1, turn 1. Copy the message
   and compare it with §6 by script.

   *Al's addition (ruling D, 2026-11-22 UTC), in his words, copied by script from ruling D with
   line breaks added:*

   > a session opens its own conversation record and no other, by its exact name, and lists
   > nothing in that directory. If the record is not where the note puts it, the session does not
   > look for it: it records the launching message from its own context, states that it did, and I
   > judge. A session that opens another conversation's record has taken on that conversation's
   > context and is not cold (R1).
4. *Completing the provisional header.* Nothing says what a completed header looks like. Session
   7 replaced the one provisional header line with a header in the §4.5 form and checked by
   script that every byte before and after that line was unchanged.
