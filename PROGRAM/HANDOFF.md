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
