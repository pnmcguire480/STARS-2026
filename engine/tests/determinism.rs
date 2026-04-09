//! Cross-target determinism gate — native integration test.
//!
//! This file is a thin wrapper that calls the shared fingerprint
//! computation in `stars2026_engine::determinism::test_support`. The
//! same computation is also called by the wasm-bindgen-test in
//! `engine/tests/determinism_wasm.rs` (Atom B.2). Both tests compare
//! against the SAME `EXPECTED_FINGERPRINT` constant so any cross-target
//! divergence is caught.
//!
//! See `engine/src/determinism.rs` for the full computation catalog,
//! and ADR-0002 / ADR-0003 for the rationale behind the gate.

use stars2026_engine::determinism::test_support::{
    compute_determinism_fingerprint, print_and_panic_on_empty, EXPECTED_FINGERPRINT,
};

/// **Determinism gate test.** Computes the fingerprint and asserts it
/// matches the pinned constant.
#[test]
fn determinism_fingerprint_is_pinned() {
    let actual = compute_determinism_fingerprint();
    print_and_panic_on_empty(&actual);
    assert_eq!(
        actual, EXPECTED_FINGERPRINT,
        "Determinism fingerprint mismatch. This is a CRITICAL failure: \
         the engine produced different bytes than the pinned reference. \
         Likely causes: (1) target-specific FP behavior (FMA fusion, ULP drift), \
         (2) HashMap iteration order leaking through serde, (3) a non-deterministic \
         Default impl. Investigate before updating the constant."
    );
}

/// Quick smoke test that the fingerprint is non-empty and stable across
/// two same-target runs.
#[test]
fn determinism_fingerprint_stable_across_runs() {
    let first = compute_determinism_fingerprint();
    let second = compute_determinism_fingerprint();
    assert_eq!(
        first, second,
        "Fingerprint must be stable across two runs of the same target — \
         if this fails, something in the computation is using ambient state \
         (RNG, time, env) instead of fixed inputs."
    );
    assert!(
        !first.is_empty(),
        "Fingerprint must contain at least one byte"
    );
}
