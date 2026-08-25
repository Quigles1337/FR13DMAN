#!/usr/bin/env bash
# FRIEDMAN local gate (F9): the default branch must pass this clean.
# The same gate CI runs (.github/workflows/check.yml; ADR-0002). Run from anywhere.
# Toolchain comes from rust-toolchain.toml at the repo root (F7: the pin, never a machine default).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT/engine"
echo "== toolchain (pinned by $ROOT/rust-toolchain.toml) =="
rustc --version
cargo --version
echo "== cargo fmt --check =="
cargo fmt --all -- --check
echo "== cargo test =="
cargo test --workspace
echo "== cargo clippy (deny warnings) =="
cargo clippy --workspace --all-targets -- -D warnings
echo "== lab verifier (zero result authority, F10; guards ADR-0001 goldens until superseded) =="
PY="$(command -v python3 || command -v python)"
"$PY" "$ROOT/lab/verify_adr0001.py" | tail -1
echo "ALL GREEN"
