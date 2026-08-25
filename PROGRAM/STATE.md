# STATE

- **Updated:** 2026-08-25 (Session 2, close)
- **Phase:** F0 — FOUNDATION
- **Gate:** F0 (open)
- **Default branch:** `main`. Remote: `origin` = `https://github.com/Quigles1337/FR13DMAN`
  (**public**; created and designated by Al 2026-08-22 — see ADR-0000).
  **ORIENT note (F7):** `git symbolic-ref refs/remotes/origin/HEAD` failed on this working copy at
  the start of Session 2 ("not a symbolic ref") and resolved after `git fetch --all --prune`; the
  sibling program recorded the same symptom. Source-of-truth fallback: `git remote show origin`
  (prints `HEAD branch: main`). Do not guess.
- **Build status:** no build yet — `engine/` not scaffolded (F0.4 open). "Green" currently means:
  state files consistent with git reality, and `python lab/verify_adr0001.py` exits 0 (35 checks;
  Python 3.11.9 observed 2026-08-25). `lab/` has zero result authority (F10); the verifier only
  guards ADR-0001's goldens until the engine test exists.
- **Toolchain observed 2026-08-25 (not yet pinned — F0.4 ADR):** `rustc 1.91.0`, `cargo 1.91.0`,
  Python 3.11.9, Node v22.23.1. Re-discover at F0.4; do not treat this line as the pin.

## Gate F0 exit criteria

| Criterion | Status | Evidence |
|---|---|---|
| F0.1 sweep report at `PROGRAM/SWEEP-2026-08.md` with F11 banner | **blocked on Al — file not found** | Session 2 escalation response said the report was at the repo root as `SWEEP-2026-08.md`; it is not in the working tree, not on `origin/main`, not in any ref or stash, and not in Desktop/Downloads/Documents or the KOBER root (searched 2026-08-25). F11 forbids reconstructing it; escalation kept open below |
| F0.2 CLAIM-0001..0003 resolved VERIFIED or honestly UNKNOWN with consequences | **done (core)** | all three VERIFIED 2026-08-22 (commits `6649990`, `bee91d9`; S-0001..S-0003). "Verify on first use" list (Miller/Li, Ferrer-i-Cancho, Takahashi & Tanaka-Ishii, Rugg) remains open by design |
| F0.3 manifest schema ADR merged | **done** | ADR-0001 (`7288a49`, on `main`): JSON/I-JSON manifests, `friedman-tokens/1` corpus format, JCS config hash (S-0004), commanded+achieved+tolerance verdict, no timestamps, KOBER handshake = two files + version strings; hand-computed goldens (D6) re-derived by `lab/verify_adr0001.py` (`2e9bef0`) |
| F0.4 scaffold (`engine/`, `lab/`, `corpora/`, `gallery/`, `quarantine/` + F8 README, CI) | open | `lab/` exists with one script (created for ADR-0001's verifier; not the scaffold). Everything else unbuilt |
| Licenses recorded for anything touched | ongoing | S-0001..S-0004 all carry license terms; no stored copy committed (arXiv ×2, RFC text — Trust Legal Provisions not retrieved, so redistribution unverified) |
| Cold-start test | open | — |

## Blockers

- none (F0.1 is Al-gated but does not block F0.4)

## Standing escalations (awaiting Al)

- **F0.1 — founding sweep report: NOT FOUND where stated.** Al's Session-2 response placed it at
  the repo root as `SWEEP-2026-08.md`. On 2026-08-25 the executor found no such file in
  `C:\Users\LEET\FR13DMAN`, on `origin/main`, in any ref/stash, or in Desktop / Downloads /
  Documents / the KOBER root. Under F11 the executor must not regenerate it. **Ask:** place the
  file at `C:\Users\LEET\FR13DMAN\SWEEP-2026-08.md` (or commit it on `main`) and, if convenient,
  state its byte size or SHA-256 so the executor can confirm it is the intended file before
  moving it to `PROGRAM/SWEEP-2026-08.md` and applying/confirming the *map, not evidence (F11)*
  banner. If Al has already applied the banner, the executor will verify rather than re-apply.

## NEXT-SINGLE-ACTION

F0.4 — Scaffold the machine, in this order, on a feature branch, merged to `main` only when
`cargo test` is green (F9): (1) discover the toolchain per F7 (`rustc --version`, `cargo --version`;
check whether `rustup` and a `rust-toolchain.toml` convention are available — the sibling KOBER repo
has one, read it as precedent only); (2) write `PROGRAM/DECISIONS/ADR-0002.md` pinning the Rust
toolchain and edition, the workspace layout under `engine/`, the JCS serializer choice (crate vs
in-repo, after checking what exists — ADR-0001 D5), and the `corpus.txt` commit policy (ADR-0001
D2); (3) create `engine/` as a Cargo workspace with one crate holding the ADR-0001 manifest types
(serde) and a test that reproduces **every** ADR-0001 D6 golden (content hash over the 12 bytes,
JCS config hash over the 57 bytes, `corpus_id` derivation) — the engine is wrong if it disagrees;
(4) create `corpora/`, `gallery/`, `quarantine/` (empty; `quarantine/README.md` posts the F8
rules verbatim from the constitution) and keep `lab/`; (5) CI only if discovered available (KOBER
uses `.github/workflows/check.yml` — precedent, not assumption), running `cargo test` + clippy at
deny-warnings on `main`. Deliverable: ADR-0002 + green scaffold on `main`. Then Gate F0 needs only
F0.1 (Al) and the cold-start test.
