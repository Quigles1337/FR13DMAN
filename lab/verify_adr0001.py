# lab/ has zero result authority (F10). This script re-derives every golden in
# PROGRAM/DECISIONS/ADR-0001.md D6 from the bytes the ADR states and checks the
# example manifest against the D4 rules. Run: python lab/verify_adr0001.py
# The same goldens become an engine/ test at F0.4+ (that test is the authoritative one).
import glob
import hashlib
import json
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
ADR = os.path.join(ROOT, "PROGRAM", "DECISIONS", "ADR-0001.md")
raw = open(ADR, "rb").read()
fails = []


def check(name, cond, detail=""):
    print(("PASS " if cond else "FAIL ") + name + (("  " + detail) if detail else ""))
    if not cond:
        fails.append(name)


# 1. line endings / BOM on every PROGRAM file
for p in sorted(glob.glob(os.path.join(ROOT, "PROGRAM", "**", "*.md"), recursive=True)):
    b = open(p, "rb").read()
    rel = os.path.relpath(p, ROOT).replace(os.sep, "/")
    check("LF-only: " + rel, b"\r" not in b and not b.startswith(b"\xef\xbb\xbf"))

text = raw.decode("utf-8")

# 2. golden corpus bytes -> content_hash
hexbytes = "62 61 20 6b 61 20 62 61 0a 6b 61 0a"
corpus = bytes.fromhex(hexbytes.replace(" ", ""))
check("corpus bytes == 'ba ka ba\\nka\\n'", corpus == b"ba ka ba\nka\n")
ch = hashlib.sha256(corpus).hexdigest()
check("content_hash golden", ch == "8d106b97f8396631fa7e3f0a597622291ec6c773bd1628a82d44fc0c38ec75fd", ch)

# 3. corpus format friedman-tokens/1 conformance of the golden corpus
lines = corpus.split(b"\n")
check("corpus ends with single LF", corpus.endswith(b"\n") and not corpus.endswith(b"\n\n"))
docs = lines[:-1]
check("no empty documents", all(len(d) > 0 for d in docs))
check(
    "single-space separated, no edge spaces",
    all(not d.startswith(b" ") and not d.endswith(b" ") and b"  " not in d for d in docs),
)
toks = [t for d in docs for t in d.split(b" ")]
check("n_documents=2, n_tokens=4, n_types=2", (len(docs), len(toks), len(set(toks))) == (2, 4, 2))
dl = [len(d.split(b" ")) for d in docs]
check("doc_length min=1 max=3 mean=2.0", (min(dl), max(dl), sum(dl) / len(dl)) == (1, 3, 2.0))

# 4. config JCS -> config_hash (Python sort_keys coincides with JCS for this ASCII/short-number input)
cfg_authored = json.loads('{"order": 1, "transition_bias": 0.75, "alphabet": ["ba", "ka"]}')
jcs = json.dumps(cfg_authored, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode("utf-8")
check("JCS form matches ADR text", jcs == b'{"alphabet":["ba","ka"],"order":1,"transition_bias":0.75}', jcs.decode())
check("JCS length 57", len(jcs) == 57, str(len(jcs)))
cfgh = hashlib.sha256(jcs).hexdigest()
check("config_hash golden", cfgh == "22cdc154338b0b3c5aa84e767452853343cdebf204e84595aaa2c7feed0cdc33", cfgh)
check("corpus_id derivation", f"markov-v1.0.0-{cfgh[:12]}-s7" == "markov-v1.0.0-22cdc154338b-s7")

# 5. example manifest parses, has every required field, keys in JCS order, D4 verdict rule holds
m = re.search(r"```json\n(.*?)\n```", text, re.S)
check("json block found", m is not None)
man = json.loads(m.group(1))
required = [
    "manifest_schema", "corpus_id", "origin", "generator", "engine_commit", "seed", "config",
    "config_hash", "ground_truth", "corpus", "commanded", "achieved", "fidelity",
]
missing = [k for k in required if k not in man]
check("all required top-level fields present", not missing, str(missing))
check("manifest_schema string", man["manifest_schema"] == "friedman-manifest/1")
check("config_hash in manifest == recomputed", man["config_hash"] == cfgh)
check("content_hash in manifest == recomputed", man["corpus"]["content_hash"] == ch)
check("corpus_id in manifest == derived", man["corpus_id"] == f"markov-v1.0.0-{cfgh[:12]}-s7")
check(
    "config in manifest == golden config",
    json.dumps(man["config"], sort_keys=True, separators=(",", ":")).encode() == jcs,
)
check(
    "ground_truth.label == mechanism, family == generator.id",
    man["ground_truth"]["label"] == "mechanism" and man["ground_truth"]["family"] == man["generator"]["id"],
)
check("engine_commit is 40 hex", re.fullmatch(r"[0-9a-f]{40}", man["engine_commit"]) is not None)
check(
    "corpus counts match golden",
    (man["corpus"]["n_documents"], man["corpus"]["n_tokens"], man["corpus"]["n_types"]) == (2, 4, 2),
)
check("doc_length matches golden", man["corpus"]["doc_length"] == {"min": 1, "max": 3, "mean": 2.0})

ps = man["fidelity"]["per_statistic"]
check("per_statistic covers every commanded key", set(ps) == set(man["commanded"]))


def verdict(k):
    a = man["achieved"].get(k)
    if a is None or a["value"] is None:
        return "FAIL"
    c = man["commanded"][k]
    t = c["tolerance"]
    if "abs" in t:
        return "PASS" if abs(a["value"] - c["value"]) <= t["abs"] else "FAIL"
    if "rel" in t:
        return "PASS" if abs(a["value"] - c["value"]) <= t["rel"] * abs(c["value"]) else "FAIL"
    if t.get("exact"):
        return "PASS" if a["value"] == c["value"] else "FAIL"
    return "FAIL"


check("per_statistic verdicts follow the D4 rule", all(ps[k] == verdict(k) for k in ps))
check(
    "overall verdict follows the D4 rule",
    man["fidelity"]["verdict"] == ("PASS" if all(v == "PASS" for v in ps.values()) else "FAIL"),
)
check(
    "null achieved value carries a reason",
    all(("reason" in v) == (v["value"] is None) for v in man["achieved"].values()),
)


def keys_sorted(o):
    if isinstance(o, dict):
        ks = list(o)
        return ks == sorted(ks, key=lambda s: s.encode("utf-16-be")) and all(keys_sorted(v) for v in o.values())
    if isinstance(o, list):
        return all(keys_sorted(v) for v in o)
    return True


# key order as written in the ADR text (json.loads preserves insertion order)
check("example manifest keys are in JCS order (recursively)", keys_sorted(man))
check("no timestamp field", not any("time" in k or "date" in k or "created" in k for k in man))

print()
print("RESULT:", "ALL PASS" if not fails else f"{len(fails)} FAIL -> {fails}")
sys.exit(1 if fails else 0)
