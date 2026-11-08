//! CI failure-sensitivity tripwire — one transparent, **test-only** probe.
//!
//! Purpose: prove that a genuinely failing Rust test is surfaced by the real
//! `cargo test` process, so a green CI status cannot silently hide a broken or
//! unexecuted test. It carries no product behaviour:
//!
//! * in the ordinary environment the single test passes;
//! * when the **test-only** environment variable `SOLO_CI_INJECT_TEST_FAILURE` is
//!   exactly `1`, the same test fails with [`INJECTED_FAILURE_DIAGNOSTIC`], and the
//!   workflow requires that specific failure (nonzero exit **and** this diagnostic
//!   **and** exactly one failed test) instead of accepting any nonzero exit.
//!
//! This is not an application switch, not an architecture-checker switch and not a
//! way to weaken a mandatory gate: no production code, no checker routine and no
//! other test reads this variable. The injected invocation is expected to fail, so
//! the CI step that runs it succeeds only after verifying that expected failure.

/// Fixed, distinctive marker asserted by the CI probe.
///
/// Deliberately ASCII, single-line and free of paths, user input or OS text, so it
/// is stable to grep and safe to print in a public job log.
const INJECTED_FAILURE_DIAGNOSTIC: &str = "SOLO_CI_TRIPWIRE_INJECTED_FAILURE: deliberate test-only fault for CI failure-sensitivity evidence";

/// The tripwire test: passes normally, fails only under the injected variable.
#[test]
fn the_ci_tripwire_reports_an_injected_fault() {
    let injected = std::env::var("SOLO_CI_INJECT_TEST_FAILURE").as_deref() == Ok("1");

    assert!(!injected, "{INJECTED_FAILURE_DIAGNOSTIC}");
}
