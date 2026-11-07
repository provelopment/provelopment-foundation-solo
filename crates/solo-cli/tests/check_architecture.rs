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
//! Exit codes: `0` legal, `1` violation, `2` usage error. Any other argument (including
//! any imagined `--disable`) is a usage error, never a bypass.

mod architecture_checks;

use std::process::ExitCode;

use architecture_checks::{check_real_tree, evaluate_fixture, fixture_by_id, self_test};

const EXIT_LEGAL: u8 = 0;
const EXIT_VIOLATION: u8 = 1;
const EXIT_USAGE: u8 = 2;

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
            ExitCode::from(EXIT_VIOLATION)
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
        ExitCode::from(EXIT_VIOLATION)
    }
}

fn run_one_fixture(id: &str) -> ExitCode {
    let Some(fixture) = fixture_by_id(id) else {
        eprintln!("usage: unknown fixture id; run --self-test to list the matrix");
        return ExitCode::from(EXIT_USAGE);
    };

    match evaluate_fixture(id) {
        None => {
            eprintln!("usage: fixture `{id}` is not registered");
            ExitCode::from(EXIT_USAGE)
        }
        Some(violations) if violations.is_empty() => {
            // A forbidden fixture that passes is a checker failure, reported as such.
            println!(
                "fixture={} class=\"{}\" outcome=accepted expected={}",
                fixture.id,
                fixture.class,
                fixture.expected.as_str()
            );
            eprintln!("error: forbidden fixture `{id}` was accepted");
            ExitCode::from(EXIT_VIOLATION)
        }
        Some(violations) => {
            println!(
                "fixture={} class=\"{}\" outcome=refused expected={} findings={}",
                fixture.id,
                fixture.class,
                fixture.expected.as_str(),
                violations.len()
            );
            report_refusals(&violations);
            // Intentionally nonzero: the fixture is forbidden, so refusal is success for
            // the check and a nonzero status for this deliberately illegal input.
            ExitCode::from(EXIT_VIOLATION)
        }
    }
}

fn report_refusals(violations: &[architecture_checks::Violation]) {
    for violation in violations {
        eprintln!("error: {violation}");
    }
}
