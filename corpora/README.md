# corpora/ — manifested synthetic output (F1, ADR-0001, ADR-0002)

Layout (ADR-0001 D2), one directory per corpus:

```
corpora/<corpus_id>/manifest.json   # committed — the F1 carrier, schema friedman-manifest/1
corpora/<corpus_id>/corpus.txt      # NOT committed — regenerated from the manifest (ADR-0002 D3)
```

- `corpus_id` = `<generator.id>-v<generator.version>-<config_hash[0:12]>-s<seed>`; two corpora with
  the same id are byte-identical by construction (F6). That is an engine invariant with a test, not
  a convention.
- `corpus.txt` is `friedman-tokens/1` (ADR-0001 D3): UTF-8, one document per line, single-space
  tokens, LF-only. Its SHA-256 is `corpus.content_hash` in the manifest, so a regenerated file is
  verifiable without the original.
- A corpus **counts** (F5) only when `fidelity.verdict` is `PASS`. FAIL manifests are kept as
  findings, never deleted.
- Nothing in this directory is a result. Results are CLAIMS.md entries with engine SHA + seed +
  config hash + CI (F6); this directory is their raw material.

Empty until the first generator module lands (Phase F2).
