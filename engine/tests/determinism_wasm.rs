//! Cross-target determinism gate — wasm32 integration test.
//!
//! This is the Atom B.2 moment of truth: run the **exact same**
//! `compute_determinism_fingerprint()` function that the native test
//! runs, but compiled to `wasm32-unknown-unknown` and executed via
//! `wasm-pack test --node`. If the resulting bytes match
//! `EXPECTED_FINGERPRINT` (pinned by the native test), the cross-target
//! determinism contract is proven. If they diverge, we have a real
//! determinism bug and the byte offset of the first mismatch tells us
//! exactly which computation drifted.
//!
//! Run with: `wasm-pack test --node engine`

#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::wasm_bindgen_test;

use stars2026_engine::determinism::test_support::{
    compute_determinism_fingerprint, EXPECTED_FINGERPRINT,
};

/// The definitive cross-target determinism assertion. Same function,
/// same constant, different target. Pass = determinism contract holds.
#[wasm_bindgen_test]
fn cross_target_determinism_fingerprint_matches() {
    let actual = compute_determinism_fingerprint();
    assert_eq!(
        actual.len(),
        EXPECTED_FINGERPRINT.len(),
        "Fingerprint length mismatch: wasm produced {} bytes, native pinned {} bytes",
        actual.len(),
        EXPECTED_FINGERPRINT.len()
    );
    // Find the first divergent byte for diagnostic purposes.
    for (i, (a, e)) in actual.iter().zip(EXPECTED_FINGERPRINT.iter()).enumerate() {
        assert_eq!(
            a, e,
            "Cross-target determinism FAILURE at byte {i}: wasm=0x{a:02X}, native=0x{e:02X}. \
             The engine produces different bytes on wasm32 vs native. Investigate before updating \
             the pinned constant — this is a real determinism bug, not a constant to change."
        );
    }
}
