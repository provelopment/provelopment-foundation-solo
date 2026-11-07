//! Ordinary `#[test]` gate for the architecture checker, run by
//! `cargo test --workspace --all-targets --locked`.
//!
//! This target is the reason a violation can never be "only reported": the real tree must
//! satisfy the boundary contract, every forbidden fixture must be refused by its expected
//! category, and every legal twin must be accepted — as ordinary test cases.

mod architecture_checks;

use architecture_checks::{
    FIXTURES, POSITIVES, check_real_tree, evaluate_fixture, evaluate_positive, fixture_ids,
    self_test,
};

#[test]
fn the_real_tree_satisfies_the_source_boundary_contract() {
    let report = check_real_tree().unwrap_or_else(|violations| {
        panic!("the real tree violates the boundary contract: {violations:#?}")
    });

    assert_eq!(report.packages, 4, "exactly four workspace packages");
    assert!(
        report.units >= 8,
        "every declared target must be checked, saw {}",
        report.units
    );
    assert!(
        report.files >= 14,
        "every reachable source file must be parsed, saw {}",
        report.files
    );
    assert!(report.edges >= 6, "the real graph must be non-trivial");
}

#[test]
fn every_forbidden_fixture_is_refused_by_its_expected_category() {
    for fixture in &FIXTURES {
        let found = evaluate_fixture(fixture.id)
            .unwrap_or_else(|| panic!("fixture `{}` is not registered", fixture.id));

        assert!(
            !found.is_empty(),
            "fixture `{}` ({}) was accepted: the guard is not load-bearing",
            fixture.id,
            fixture.class
        );
        assert!(
            found
                .iter()
                .any(|violation| violation.category == fixture.expected),
            "fixture `{}` must be refused as {}, saw {:#?}",
            fixture.id,
            fixture.expected.as_str(),
            found
                .iter()
                .map(|violation| violation.category.as_str())
                .collect::<Vec<_>>()
        );
    }
}

#[test]
fn legal_twins_are_accepted() {
    for id in POSITIVES {
        let found = evaluate_positive(id).unwrap_or_else(|| panic!("`{id}` is not registered"));
        assert!(found.is_empty(), "`{id}` must be accepted, saw {found:#?}");
    }
}

#[test]
fn the_self_test_matrix_is_complete_and_passing() {
    let ids = fixture_ids();
    assert_eq!(
        ids.len(),
        FIXTURES.len(),
        "every required negative class must have a fixture"
    );
    assert!(
        ids.contains(&"checker-bypass"),
        "the bypass probe must exist"
    );

    let cases = self_test();
    let failed: Vec<&str> = cases
        .iter()
        .filter(|case| !case.passed)
        .map(|case| case.id.as_str())
        .collect();
    assert!(failed.is_empty(), "self-test failures: {failed:?}");
    assert_eq!(
        cases.len(),
        FIXTURES.len() + POSITIVES.len() + 1,
        "the matrix covers the real tree, every fixture and every legal twin"
    );
}
