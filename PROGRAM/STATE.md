# STATE

- **Updated:** 2026-08-22 (Session 1, bootstrap)
- **Phase:** F0 — FOUNDATION
- **Gate:** F0 (open)
- **Default branch:** `main`. Remote: `origin` = `https://github.com/Quigles1337/FR13DMAN`
  (**public**; created and designated by Al 2026-08-22 — see ADR-0000)
- **Build status:** no build yet — `engine/` not scaffolded (F0.4 open). "Green" currently means:
  state files consistent with git reality; nothing compiles because nothing exists to compile.

## Gate F0 exit criteria

| Criterion | Status | Evidence |
|---|---|---|
| F0.1 sweep report at `PROGRAM/SWEEP-2026-08.md` with F11 banner | **blocked on Al** | report was not provided at bootstrap; under F11 the executor must not reconstruct it from memory — see escalations |
| F0.2 CLAIM-0001..0003 resolved VERIFIED or honestly UNKNOWN with consequences | in progress | CLAIMS.md seeded (all three UNKNOWN); retrieval work this session |
| F0.3 manifest schema ADR merged | open | — |
| F0.4 scaffold (`engine/`, `lab/`, `corpora/`, `gallery/`, `quarantine/` + F8 README, CI) | open | — |
| Licenses recorded for anything touched | ongoing | SOURCES.md (empty; rule posted) |
| Cold-start test | open | — |

## Blockers

- none (F0.1 is Al-gated but does not block F0.2–F0.4)

## Standing escalations (awaiting Al)

- **F0.1 — founding sweep report.** The Session-1 bootstrap did not include the sweep report text.
  Al: ferry the founding sweep report so it can be committed to `PROGRAM/SWEEP-2026-08.md` with the
  *map, not evidence (F11)* banner. The executor must not regenerate it from memory.

## NEXT-SINGLE-ACTION

(set at session close)
