//! Foundation Solo `solo-cli` — the single binary and the one composition root.
//!
//! The root wires the real CLI adapter to the real application service and turns the
//! typed outcome into a process exit status. It contains no validation rule, no
//! formatting rule and no business decision of its own: everything meaningful
//! happens inward of this file, which is what keeps the Hexagonal boundary honest.
//!
//! Exit status: `0` accepted, `2` refused, `1` internal error.

#![forbid(unsafe_code)]

use std::io::Write;
use std::process::ExitCode;

use solo_adapters::run;
use solo_application::BootstrapPreflightService;

fn main() -> ExitCode {
    let outcome = run(std::env::args_os(), &BootstrapPreflightService);

    if let Some(payload) = outcome.stdout() {
        let mut stdout = std::io::stdout().lock();
        let _ = stdout.write_all(payload.as_bytes());
        let _ = stdout.flush();
    }
    if let Some(payload) = outcome.stderr() {
        let mut stderr = std::io::stderr().lock();
        let _ = stderr.write_all(payload.as_bytes());
        let _ = stderr.flush();
    }

    ExitCode::from(outcome.exit_code())
}
