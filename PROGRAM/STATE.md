# STATE

- **Updated:** 2026-08-22 (Session 1, close)
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
| F0.2 CLAIM-0001..0003 resolved VERIFIED or honestly UNKNOWN with consequences | **done (core)** | all three VERIFIED 2026-08-22 with scope limits stated (commits `6649990`, `bee91d9`; S-0001..S-0003). "Verify on first use" list (Miller/Li, Ferrer-i-Cancho, Takahashi & Tanaka-Ishii, Rugg) remains open by design |
| F0.3 manifest schema ADR merged | open | — |
| F0.4 scaffold (`engine/`, `lab/`, `corpora/`, `gallery/`, `quarantine/` + F8 README, CI) | open | — |
| Licenses recorded for anything touched | ongoing | SOURCES.md S-0001..S-0003 all carry license terms; nothing ingested beyond metadata/reads; both arXiv PDFs hashed, not committed (license) |
| Cold-start test | open | — |

## Blockers

- none (F0.1 is Al-gated but does not block F0.2–F0.4)

## Standing escalations (awaiting Al)

- **F0.1 — founding sweep report.** The Session-1 bootstrap did not include the sweep report text.
  Al: ferry the founding sweep report so it can be committed to `PROGRAM/SWEEP-2026-08.md` with the
  *map, not evidence (F11)* banner. The executor must not regenerate it from memory.

## NEXT-SINGLE-ACTION

F0.3 — Write `PROGRAM/DECISIONS/ADR-0001.md` deciding the corpus **manifest schema**: enumerate
the F1 fields (generator ID + version, full config, seed, commanded statistics, achieved
statistics, tolerance verdict, content hash — content hash computed over LF-normalized bytes per
ADR-0000 §4), choose the serialization format (options to weigh: JSON vs TOML vs YAML; decide in
the ADR), define where manifests live relative to `corpora/`, and state the KOBER handshake
surface (F0.3: interface, not dependency — KOBER's Phase-3 battery may someday consume FRIEDMAN
manifests as decoy populations; neither program blocks the other). The ADR is the deliverable; no
code. Inputs worth citing: S-0001's manifest-relevant lesson (his Table 2 commanded-vs-achieved
mismatch is why the schema carries BOTH commanded and achieved statistics plus a tolerance
verdict, F5).
