//! `check-architecture` — the single test-only architecture checker entry point.
//!
//! Invocations (there is no disable, relax or partial-scan mode):
//!
//! * no arguments — inspect **the real checkout** (Cargo graph + source AST) and exit
//!   `0` only when both are legal;
//! * `--self-test` — run the whole negative/positive matrix and exit `0` only when every
//!   forbidden fixture is refused and every legal twin is accepted;
//! * `--fixture <id>` — run one forbidden fixture through the same routines; the
//!   expected outcome is a **nonzero** exit with a bounded diagnostic.
//!
//! Exit codes distinguish a correct policy refusal from the harness failing to refuse:
//!
//! * `0` — the real tree is legal (`--self-test` also exits `0` only when the whole
//!   matrix is correct);
//! * `1` — a deliberate violation was refused: the real tree is illegal, or a forbidden
//!   fixture was correctly refused by the policy;
//! * `2` — the command line was not understood; no flag can weaken the checker;
//! * `3` — a forbidden fixture was **accepted**: a checker failure, never evidence of a
//!   refusal.

mod architecture_checks;

use std::process::ExitCode;

use architecture_checks::{
    EXIT_HARNESS_FAILURE, EXIT_LEGAL, EXIT_REFUSED, EXIT_USAGE, check_real_tree,
    classify_fixture_result, evaluate_fixture, fixture_by_id, fixture_exit_code,
    fixture_outcome_text, self_test,
};

fn main() -> ExitCode {
    let arguments: Vec<String> = std::env::args().skip(1).collect();

    match arguments.as_slice() {
        [] => check_the_real_tree(),
        [flag] if flag == "--self-test" => run_self_test(),
        [flag, id] if flag == "--fixture" => run_one_fixture(id),
        _ => {
            eprintln!(
                "usage: check-architecture [--self-test | --fixture <id>]\n\
                 no other flag exists: there is no disable, relax or partial-scan mode"
            );
            ExitCode::from(EXIT_USAGE)
        }
    }
}

fn check_the_real_tree() -> ExitCode {
    match check_real_tree() {
        Ok(report) => {
            println!(
                "architecture=legal packages={} targets={} files={} edges={}",
                report.packages, report.units, report.files, report.edges
            );
            ExitCode::from(EXIT_LEGAL)
        }
        Err(violations) => {
            report_refusals(&violations);
            ExitCode::from(EXIT_REFUSED)
        }
    }
}

fn run_self_test() -> ExitCode {
    let cases = self_test();
    for case in &cases {
        println!(
            "{} {:<26} {}",
            if case.passed { "ok  " } else { "FAIL" },
            case.id,
            case.detail
        );
    }

    let failed = cases.iter().filter(|case| !case.passed).count();
    if failed == 0 {
        println!("self-check=passed cases={}", cases.len());
        ExitCode::from(EXIT_LEGAL)
    } else {
        println!("self-check=failed cases={} failed={failed}", cases.len());
        ExitCode::from(EXIT_REFUSED)
    }
}

fn run_one_fixture(id: &str) -> ExitCode {
    let Some(fixture) = fixture_by_id(id) else {
        eprintln!("usage: unknown fixture id; run --self-test to list the matrix");
        return ExitCode::from(EXIT_USAGE);
    };

    let Some(violations) = evaluate_fixture(id) else {
        eprintln!("usage: fixture `{id}` is not registered");
        return ExitCode::from(EXIT_USAGE);
    };

    let outcome = classify_fixture_result(&violations);
    println!("{}", fixture_outcome_text(fixture, &violations));

    match outcome {
        architecture_checks::FixtureOutcome::Refused => {
            report_refusals(&violations);
            // Correctly refused: the fixture is deliberately illegal, so a policy refusal
            // is success for the check and exit 1 for this input.
            ExitCode::from(fixture_exit_code(outcome))
        }
        architecture_checks::FixtureOutcome::Accepted => {
            eprintln!(
                "error: forbidden fixture `{id}` was accepted; the checker is not load-bearing"
            );
            ExitCode::from(EXIT_HARNESS_FAILURE)
        }
    }
}

fn report_refusals(violations: &[architecture_checks::Violation]) {
    for violation in violations {
        eprintln!("error: {violation}");
    }
}
