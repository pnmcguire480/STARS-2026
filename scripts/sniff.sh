#!/usr/bin/env bash
#
# scripts/sniff.sh — STARS 2026 sniff-test runner
#
# This script is the SOURCE OF TRUTH for the per-atom sniff test.
# Both local development and CI must run it verbatim. If a check
# would run in CI, it MUST run here. If a check runs here but not
# in CI, that is a CI gap. The two are kept identical on purpose.
#
# Background:
#   On 2026-04-07, the Phase 1 Task 1 close commit (2180c78) passed
#   `cargo test` and `cargo clippy` locally but failed CI on
#   `cargo fmt --check`. Root cause: the local sniff-test discipline
#   was author-memory-bound — three commands tracked in a checklist
#   that the human had to remember. ADR-0001 logged the gap. ADR-0002
#   (the hardening pass) made it impossible by encoding all checks
#   into this script.
#
# Usage:
#   bash scripts/sniff.sh                  # run all checks for the engine crate
#   bash scripts/sniff.sh --verbose        # show full output (default: tail only)
#
# Exit code:
#   0 — all checks passed
#   1 — at least one check failed
#
# Add a check? Add it BOTH here AND in .github/workflows/ci.yml.
# That coupling is the whole point.

set -euo pipefail

# ─── Configuration ──────────────────────────────────────────────────────────
ENGINE_CRATE="stars2026-engine"
WASM_TARGET="wasm32-unknown-unknown"
VERBOSE="${1:-}"

# ─── ANSI helpers (no emoji per project policy) ─────────────────────────────
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
RESET='\033[0m'

pass() { printf "${GREEN}[PASS]${RESET} %s\n" "$1"; }
fail() { printf "${RED}[FAIL]${RESET} %s\n" "$1"; exit 1; }
step() { printf "\n${YELLOW}▶${RESET} %s\n" "$1"; }

# ─── Step 1: Test ───────────────────────────────────────────────────────────
step "cargo test -p ${ENGINE_CRATE}"
if [ "$VERBOSE" = "--verbose" ]; then
    cargo test -p "$ENGINE_CRATE" || fail "cargo test"
else
    cargo test -p "$ENGINE_CRATE" 2>&1 | tail -8 || fail "cargo test"
fi
pass "cargo test"

# ─── Step 2: Clippy (pedantic, deny warnings) ───────────────────────────────
step "cargo clippy -p ${ENGINE_CRATE} --all-targets -- -D warnings"
if [ "$VERBOSE" = "--verbose" ]; then
    cargo clippy -p "$ENGINE_CRATE" --all-targets -- -D warnings || fail "cargo clippy"
else
    cargo clippy -p "$ENGINE_CRATE" --all-targets -- -D warnings 2>&1 | tail -5 || fail "cargo clippy"
fi
pass "cargo clippy"

# ─── Step 3: Format check ───────────────────────────────────────────────────
step "cargo fmt --check -p ${ENGINE_CRATE}"
if cargo fmt --check -p "$ENGINE_CRATE" 2>&1; then
    pass "cargo fmt --check"
else
    fail "cargo fmt --check (run 'cargo fmt -p ${ENGINE_CRATE}' to fix, then re-run sniff)"
fi

# ─── Step 4: WASM compile (dual-target gate) ────────────────────────────────
# This is the architectural premise of STARS 2026: one source compiles to
# both browser (wasm32) and native (x86_64). If this step fails, the project
# has lost its reason to exist regardless of how green the native side is.
step "cargo check -p ${ENGINE_CRATE} --target ${WASM_TARGET}"
if [ "$VERBOSE" = "--verbose" ]; then
    cargo check -p "$ENGINE_CRATE" --target "$WASM_TARGET" || fail "wasm32 check"
else
    cargo check -p "$ENGINE_CRATE" --target "$WASM_TARGET" 2>&1 | tail -3 || fail "wasm32 check"
fi
pass "cargo check (wasm32)"

# ─── Done ───────────────────────────────────────────────────────────────────
printf "\n${GREEN}═══════════════════════════════════════════════════════════════${RESET}\n"
printf "${GREEN}  SNIFF TEST: ALL CHECKS PASSED${RESET}\n"
printf "${GREEN}═══════════════════════════════════════════════════════════════${RESET}\n"
printf "  ✓ test       (native)\n"
printf "  ✓ clippy     (pedantic, deny warnings)\n"
printf "  ✓ fmt        (rustfmt --check)\n"
printf "  ✓ wasm32     (dual-target gate)\n"
printf "\nNext: STOP for approval per SNIFFTEST.md.\n"
