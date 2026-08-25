# quarantine/ — contested corpora (F8)

This directory is **empty by design until Phase F4** and governed by rule F8 of the constitution
(`PROGRAM/FRIEDMAN-PROGRAM.md` §1), reproduced verbatim:

> **F8 — Pre-registration and quarantine.** The battery is frozen (tagged commit, SHA in CLAIMS)
> before contested corpora are analyzed with it. Contested corpora live under `quarantine/` and are
> read-only test subjects for frozen, pre-registered analyses (published reproductions, or our
> post-freeze battery). **They never inform design:** any ADR citing contested-corpus behavior as
> design rationale pre-freeze is a leakage violation. Post-freeze battery changes create a new
> version; superseded results are RETRACTED with a trail, never edited.

House rules that follow from it:

1. Nothing lands here before the `battery-freeze-v1` tag exists and its SHA is in `CLAIMS.md`
   (F3.4), except the F1.3 exemption: Indus-derived data for the frozen-as-published Nair
   reproduction, which sits here and informs no design.
2. Every file here carries provenance in `SOURCES.md` (F11) — citation, URL, access date, license,
   SHA-256 — **before** it is added. License ambiguity is an escalation to Al, not a judgment call.
3. Read-only: code reads from here; nothing writes here except the ingestion commit itself.
4. No ADR, generator dial, detector choice, tolerance, or statistic registry entry may cite anything
   in this directory as rationale. The HANDOFF `LEAKAGE CHECK` is where each session proves this
   was checked, not assumed.
5. Lifting or modifying the quarantine in any way not already prescribed by a passed gate is Al's
   decision (§4.6), never the executor's.

FRIEDMAN never issues verdicts on contested artifacts (§0.2). Files here are positions to be
measured on a calibrated surface, not questions to be answered.
