# HANDOFF — append-only session log

Schema per §4.5. Newest entry at the bottom. Never edit prior entries.

---

SESSION: 2026-08-22 @ bee91d9 (Session 1; the commit adding this entry + the STATE update lands
on top of bee91d9)
PHASE / GATE: F0 — FOUNDATION / Gate F0 open
ADVANCED:
- Bootstrap (`eced49d`): constitution placed verbatim at PROGRAM/FRIEDMAN-PROGRAM.md; registers
  created (STATE, CLAIMS seeded with CLAIM-0001..0003 UNKNOWN, SOURCES, HANDOFF); ADR-0000
  (home = github.com/Quigles1337/FR13DMAN public, designated by Al; branch `main` discovered;
  LF pinned repo-wide); pushed, origin wired.
- F0.2 core COMPLETE: CLAIM-0001 UNKNOWN→VERIFIED (`6649990`; Nair 2026 arXiv:2604.17828 read in
  full — percentile scorecard confirmed, tunable heraldic/administrative generators confirmed, NO
  ROC/AUC/FPR anywhere); CLAIM-0002 UNKNOWN→VERIFIED and CLAIM-0003 UNKNOWN→VERIFIED (`bee91d9`;
  Lin & Tegmark Theorem 1 + Fig. 1 read first-hand; Timm generator repo MIT-confirmed from
  in-repo LICENSE, pinned a6ede220). S-0001..S-0003 recorded with licenses and SHA-256 digests.
BELIEF DELTAS:
- The sweep's arXiv ID transcription (2604.17828) was CORRECT — session-start me treated it as
  probably-garbled; evidence: abs page + full 13-page read match the believed description exactly.
- Nair's abstract claims "all code and data are publicly available" but his availability section
  says upon-request only, public repo "upon acceptance" — F1.3's "his released generators" do not
  publicly exist as of 2026-08-22. R3 will need: the release appearing, an Al-gated author
  contact, or reimplementation from his §4.2 spec (dials + calibration Table 1 are published).
- Nair's own Table 2 shows his generators MISS their commanded targets (heraldic Zipf −0.97
  achieved vs −1.46 commanded) — prior art independently motivates F5. Cite S-0001 when the
  manifest ADR justifies carrying commanded AND achieved stats.
- Timm's code is MIT (believed unknown) — F2.1(iii) wrap-vs-reimplement is unconstrained by
  license; decide on engineering merits.
- Publisher pages (MDPI, Taylor & Francis) bot-block plain fetches from this environment with
  403; arXiv and the GitHub API do not. Plan retrievals accordingly.
CONFUSIONS:
- Session start pointed this program at the KOBER repo URL. Cost: one orientation detour + one
  escalation round-trip before any FRIEDMAN work. Resolution: Al designated FR13DMAN when asked.
  Doc fix: ADR-0000 records the home decision and WHY the KOBER repo was impossible (identical
  PROGRAM/ schema collides) — successors need not re-litigate.
- During orientation, the sibling KOBER working tree changed mid-observation (its own Session 3
  was live in another window, wiring its remote). Lesson recorded: never touch the sibling's
  working tree; treat any cross-repo observation as instantly stale.
HONESTY CHECK: all three transitions carry explicit scope limits — CLAIM-0003 covers existence +
license only (code unread, Zenodo record unfetched, Cryptologia paper corroborated-not-retrieved);
CLAIM-0002 carries the two-point-MI estimator caveat and the Zipf-vs-MI-power-law distinction;
CLAIM-0001 records the availability contradiction rather than repeating the abstract's claim. No
downgrades needed this session; nothing entered the register from memory.
LEAKAGE CHECK: checked, not assumed — the only design decision made was ADR-0000 (repo hygiene),
which cites no corpus behavior. S-0001 is a paper ABOUT a contested corpus, read to verify prior
art's methodology; no Indus data was ingested, computed on, or used to motivate any design choice.
`quarantine/` does not exist yet (F0.4). Verdict: no leakage.
DETERMINISM CHECK: no MEASURED/REPRODUCED results exist yet. Every retrieval is pinned:
version-pinned arXiv URLs + SHA-256 for both PDFs (copies not committed — license), GitHub repo
pinned at commit a6ede220. Nothing to quarantine from CLAIMS.
DRIFT CHECK: no verdict creep (no statement anywhere about what Indus/Voynich/any artifact "is");
no demo drift (zero gallery work, no timebox consumed); no detector-shopping (no detectors exist).
Clean.
NEXT-SINGLE-ACTION: see STATE.md — F0.3 manifest-schema ADR (ADR-0001), fully specified there.
ESCALATIONS:
- F0.1 — Al to ferry the founding sweep report for PROGRAM/SWEEP-2026-08.md (F11 banner: map, not
  evidence). The executor must not reconstruct it from memory.
STOP-REASON: natural boundary — F0.2 core complete and pushed green; F0.3 is a fresh design task
best started cold by a successor with the full constitution in context.

---

SESSION: 2026-08-25 @ 2e9bef0 (Session 2; the commit adding this entry + the STATE update lands
on top of 2e9bef0)
PHASE / GATE: F0 — FOUNDATION / Gate F0 open
ADVANCED:
- F0.3 COMPLETE: ADR-0001 (`7288a49`) decides the corpus manifest schema — JSON restricted to
  I-JSON; one `manifest.json` + `corpus.txt` per corpus under `corpora/<corpus_id>/`;
  `friedman-tokens/1` byte format (one document per line, single-space tokens, LF-only by
  construction); `config_hash` = SHA-256 over RFC 8785 JCS canonical config; `content_hash` =
  SHA-256 over corpus bytes; commanded + achieved statistics with per-statistic tolerance verdict
  (F5); `corpus_id` a pure function of (generator, config, seed); deliberately no timestamps so
  regeneration is byte-identical; KOBER handshake (D7) = the two files + the schema/format
  version strings, nothing more. Hand-computed goldens in D6.
- S-0004 (RFC 8785) retrieved this session, load-bearing sections read first-hand, SHA-256 pinned,
  copy not committed (Trust Legal Provisions not retrieved → redistribution unverified).
- `lab/verify_adr0001.py` (`7288a49`, made repo-relative in `2e9bef0`): 35 checks re-deriving the
  D6 goldens from the bytes the ADR states, parsing the example manifest, enforcing the D4 verdict
  rule and JCS key order, and asserting every PROGRAM file is LF-only. ALL PASS at `2e9bef0`.
- F0.1 escalation response processed: the file Al described is NOT present (see CONFUSIONS and
  ESCALATIONS). Nothing fabricated; escalation re-issued with a precise ask.
BELIEF DELTAS:
- Session-start me expected to spend the session on F0.1 (a placement task). The file does not
  exist anywhere reachable, so the inherited NEXT-SINGLE-ACTION (F0.3) was the right one thing.
  Evidence: working tree, `origin/main`, `git log --all -- 'SWEEP*'`, stash, Desktop, Downloads,
  Documents, KOBER root — all empty of it.
- A manifest needs no wall-clock timestamp and is better without one: byte-identical regeneration
  (F6) is worth more than an in-band creation time that git already records. Evidence: the D6
  goldens are only meaningful because every input is fixed bytes.
- Python's `json.dumps(sort_keys=True, separators=(",", ":"))` coincides with JCS only for
  ASCII keys and short numbers; the engine still needs a real RFC 8785 serializer (number
  serialization per ECMAScript §7.1.12.1 is the part that diverges). Evidence: S-0004 §3.2.2.3
  read first-hand — the RFC explicitly defers the number algorithm to V8/Ryu references.
- The deep `find` over the home directory hangs on OneDrive reparse points in this environment
  (timed out at 120 s); bounded, directory-specific searches complete instantly. Plan searches
  accordingly.
CONFUSIONS:
- **The escalation response named a file that is not there.** Cost: ~5 tool calls of search
  before concluding, plus this session could not close F0.1 as instructed. Doc fix: STATE.md's
  escalation now asks for an exact path and a size/SHA-256 so the next executor can verify
  identity in one step instead of searching; this entry records every location checked so the
  successor does not repeat the search.
- `git symbolic-ref refs/remotes/origin/HEAD` failed at ORIENT ("not a symbolic ref") and then
  resolved after `git fetch --all --prune`. Cost: one confused minute. Doc fix: STATE.md carries
  the fallback (`git remote show origin`), mirroring the sibling program's identical finding.
- Two attempts to write files through the shell went wrong: a long heredoc failed to parse in
  this harness, and `sed`/`python -c` replacements silently turned `\r`/`\xef` escapes into raw
  bytes, producing a broken line that I committed once (`7288a49` carried two absolute Windows
  paths, an F7 violation). Cost: two extra tool rounds and a fix-up commit (`2e9bef0`). Lesson
  for successors: write multi-line files with the editor tools, then verify with a script; never
  trust a `grep` in an `&&` chain as a guard (a match exits 0 and the chain continues).
HONESTY CHECK: no CLAIM transitions this session — ADR-0001 is engineering, not scholarship, and
S-0004 is recorded as an engineering standard with "no CLAIM depends on it." Re-examined
CLAIM-0001..0003: unchanged, scope limits still accurate. The ADR's example manifest is labelled
as shape-only placeholders that enter no register. Downgrades: none needed. The one thing I got
wrong (absolute paths in a committed script) is recorded above rather than amended away.
LEAKAGE CHECK: checked, not assumed — the only design decision was ADR-0001. Its one citation of
contested-corpus material is S-0001's Table 2 (a *published generator's* commanded-vs-achieved
gap, i.e., prior art's methodology), used to justify carrying achieved statistics — no Indus
data, no Indus statistic, no contested-corpus behaviour informed any field or rule. The KOBER
constitution was read (read-only, its Phase-3 text) to state the handshake accurately; no
Linear A/B data was touched. `quarantine/` still does not exist. Verdict: no leakage.
DETERMINISM CHECK: no MEASURED/REPRODUCED results exist. The D6 goldens are regenerable from
fixed bytes by anyone with `sha256sum` (and by `lab/verify_adr0001.py` from a fresh clone —
`7288a49`'s copy was NOT clone-portable; `2e9bef0`'s is). S-0004 pinned by URL + SHA-256.
Nothing to quarantine from CLAIMS.
DRIFT CHECK: no verdict creep (no statement about what any artifact "is"; `ground_truth.label`
for synthetic corpora is fixed to `"mechanism"` by construction, not judgment); no demo drift
(zero gallery work, no timebox consumed); no detector-shopping (no detectors exist; the ADR fixes
entry *shape*, not the statistic list, and defers CI-aware verdict rules to a future schema
version by ADR so they cannot be tuned silently). One scope note: `lab/` was created one phase
item early to hold the verifier — a directory, not a decision; F0.4 still owns the scaffold.
NEXT-SINGLE-ACTION: see STATE.md — F0.4 scaffold (ADR-0002 toolchain/workspace/JCS/commit-policy
+ `engine/` workspace with the D6 golden test green + `corpora/` `gallery/` `quarantine/` with F8
README + CI if discovered available), fully specified there.
ESCALATIONS:
- **F0.1 — sweep report not found where stated.** Al: the file was not at
  `C:\Users\LEET\FR13DMAN\SWEEP-2026-08.md`, on `origin/main`, or anywhere searched (list in
  STATE.md). Please place it at that path (or commit it on `main`) and, if convenient, state its
  size or SHA-256; the executor will then move it to `PROGRAM/SWEEP-2026-08.md`, verify the F11
  banner, and close the escalation. Not regenerated, per F11.
STOP-REASON: natural boundary — F0.3 complete and green on `main`; F0.4 is a scaffold task that
should start cold with toolchain discovery, and the F0.1 blocker needs Al before Gate F0 can close.
