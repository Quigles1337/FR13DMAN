# SOURCES — citations and data provenance (F11)

An entry exists here only after the source was retrieved this-session and verified. Every entry
records: full citation, URL, access date, license terms, and SHA-256 of any stored copy. Licenses
(code and corpora) are recorded before ingestion; ambiguity → escalate to Al.

The founding sweep report (PROGRAM/SWEEP-2026-08.md, once ferried) is a **map, not evidence** — it
can suggest what to retrieve, but nothing enters this register from it or from executor memory.

Storage rule: a stored copy is committed to the repo only when its license permits redistribution
(this repo is public). Otherwise the entry records URL + access date + SHA-256 of the local copy and
notes that the copy is not committed.

---

## S-0001 — Nair 2026 (closest prior art; CLAIM-0001)

- **Citation:** Nair, Ashish (2026). "How Non-Linguistic Is the Indus Sign System? A
  Synthetic-Baseline Scorecard." arXiv:2604.17828v1 [cs.CL], submitted 2026-04-20. Independent
  Researcher, ashishn@alumni.cmu.edu. 13 pages.
- **URL:** https://arxiv.org/abs/2604.17828 (abs), https://arxiv.org/pdf/2604.17828v1 (PDF),
  https://arxiv.org/html/2604.17828v1 (HTML)
- **Access date:** 2026-08-22 (abs page, HTML full text, and PDF all retrieved this session; PDF
  read in full, pages 1–13)
- **License:** arXiv.org perpetual non-exclusive license 1.0 ("Nonexclusive-Distrib 1.0", as shown
  on the abs page). This license does NOT grant third-party redistribution rights → **stored copy
  not committed** (public repo). Local verification copy hashed at retrieval:
  - SHA-256 `43e7bab2dd269a2fb0cc80382687d49b1da066bf852e7ddd369df526670ff7a3`
    (749,585 bytes, `2604.17828v1` PDF). Re-retrievable from arXiv by version-pinned URL; any
    future copy can be checked against this digest.
- **Data/code availability (load-bearing):** the paper's abstract states "All code and data are
  publicly available," but its Data and Code Availability section states availability **from the
  corresponding author upon request**, with a public repository promised **upon acceptance**. As of
  access date there is no public release URL and no code/data license. Consequence recorded at
  CLAIM-0001.
- **Notes:** paper carries an AI Assistance Disclosure (pipeline developed with LLM assistance;
  all reported statistics computed by deterministic Python on the fixed corpus). Corpus:
  ICIT/Yajnadevam digitization, 1,916 deduplicated inscriptions, 584 unique signs, 11,110 tokens.
