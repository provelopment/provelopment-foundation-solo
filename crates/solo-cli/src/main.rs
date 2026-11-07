//! Foundation Solo `solo-cli` — the single binary and the one composition root.
//!
//! The root wires the real CLI adapter to the real application service and turns the
//! typed outcome into a process exit status. It contains no validation rule, no
//! formatting rule and no business decision of its own: everything meaningful
//! happens inward of this file, which is what keeps the Hexagonal boundary honest.
//!
//! Exit status: `0` accepted, `2` refused, `1` internal error. A required output
//! write or flush that fails is an internal error, never a silent success and never
//! a false input refusal — a lost report must not be reported as a clean result.

#![forbid(unsafe_code)]

use std::io::Write;
use std::process::ExitCode;

use solo_adapters::{CliOutcome, EXIT_INTERNAL, run};
use solo_application::BootstrapPreflightService;

/// Fixed, safe diagnostic emitted when a required output could not be written.
///
/// It is a constant ASCII line: no OS error text, path, token or authored input can
/// reach the operator through it.
const OUTPUT_FAILURE_DIAGNOSTIC: &str = "error: PREFLIGHT_INTERNAL: output-failure\n";

/// The narrow output capability this root needs from a stream.
///
/// Keeping it a trait lets failure injection exercise the real emission path in
/// unit tests without a second production composition root or extra dependency.
trait OutputSink {
    fn write_all(&mut self, bytes: &[u8]) -> std::io::Result<()>;
    fn flush(&mut self) -> std::io::Result<()>;
}

/// Adapter binding the sink trait to a real standard-library stream.
struct StreamSink<W: Write> {
    writer: W,
}

impl<W: Write> StreamSink<W> {
    fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl<W: Write> OutputSink for StreamSink<W> {
    fn write_all(&mut self, bytes: &[u8]) -> std::io::Result<()> {
        self.writer.write_all(bytes)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

/// Emits the outcome payloads and returns the process exit status.
///
/// Required output is written and flushed for both streams. Any failure of that
/// required output yields [`EXIT_INTERNAL`]; the best-effort follow-up diagnostic is
/// fixed and its own failure cannot change the status.
fn emit(outcome: &CliOutcome, stdout: &mut impl OutputSink, stderr: &mut impl OutputSink) -> u8 {
    if let Some(payload) = outcome.stdout()
        && stdout
            .write_all(payload.as_bytes())
            .and_then(|()| stdout.flush())
            .is_err()
    {
        let _ = stderr
            .write_all(OUTPUT_FAILURE_DIAGNOSTIC.as_bytes())
            .and_then(|()| stderr.flush());
        return EXIT_INTERNAL;
    }

    if let Some(payload) = outcome.stderr()
        && stderr
            .write_all(payload.as_bytes())
            .and_then(|()| stderr.flush())
            .is_err()
    {
        return EXIT_INTERNAL;
    }

    outcome.exit_code()
}

fn main() -> ExitCode {
    let outcome = run(std::env::args_os(), &BootstrapPreflightService);

    let mut stdout = StreamSink::new(std::io::stdout().lock());
    let mut stderr = StreamSink::new(std::io::stderr().lock());

    ExitCode::from(emit(&outcome, &mut stdout, &mut stderr))
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;

    use super::*;

    const EXPECTED_SUCCESS_LINE: &str =
        "preflight=ok entity_id=entity-7 base=https://example.com/alpha\n";
    const EXPECTED_REFUSAL_LINE: &str = "error: PREFLIGHT_ENTITY_ID_INVALID: invalid-start\n";

    /// Deterministic sink that records accepted bytes and injects the requested
    /// failure.
    #[derive(Default)]
    struct MockSink {
        fail_write: bool,
        fail_flush: bool,
        written: Vec<u8>,
        flushes: usize,
    }

    impl MockSink {
        fn working() -> Self {
            Self::default()
        }

        fn write_fails() -> Self {
            Self {
                fail_write: true,
                ..Self::default()
            }
        }

        fn flush_fails() -> Self {
            Self {
                fail_flush: true,
                ..Self::default()
            }
        }

        fn text(&self) -> String {
            String::from_utf8(self.written.clone()).expect("recorded bytes must be UTF-8")
        }
    }

    impl OutputSink for MockSink {
        fn write_all(&mut self, bytes: &[u8]) -> std::io::Result<()> {
            if self.fail_write {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::BrokenPipe,
                    "injected write failure",
                ));
            }
            self.written.extend_from_slice(bytes);
            Ok(())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            self.flushes += 1;
            if self.fail_flush {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::BrokenPipe,
                    "injected flush failure",
                ));
            }
            Ok(())
        }
    }

    fn argv(tokens: &[&str]) -> Vec<OsString> {
        let mut args = vec![OsString::from("solo-cli")];
        args.extend(tokens.iter().map(OsString::from));
        args
    }

    fn accepted_outcome() -> CliOutcome {
        run(
            argv(&[
                "bootstrap-preflight",
                "--entity-id",
                "entity-7",
                "--base-url",
                "https://EXAMPLE.com:443/alpha",
            ]),
            &BootstrapPreflightService,
        )
    }

    fn refused_outcome() -> CliOutcome {
        run(
            argv(&[
                "bootstrap-preflight",
                "--entity-id",
                "Main",
                "--base-url",
                "https://example.com",
            ]),
            &BootstrapPreflightService,
        )
    }

    /// One injected-output failure case: the two sinks and the outcome factory.
    type FailureCase = (MockSink, MockSink, fn() -> CliOutcome);

    #[test]
    fn accepted_output_is_byte_for_byte_unchanged() {
        let mut stdout = MockSink::working();
        let mut stderr = MockSink::working();

        let status = emit(&accepted_outcome(), &mut stdout, &mut stderr);

        assert_eq!(status, 0);
        assert_eq!(stdout.text(), EXPECTED_SUCCESS_LINE);
        assert!(stdout.flushes >= 1, "required output must be flushed");
        assert_eq!(stderr.text(), "");
    }

    #[test]
    fn refusal_output_is_byte_for_byte_unchanged() {
        let mut stdout = MockSink::working();
        let mut stderr = MockSink::working();

        let status = emit(&refused_outcome(), &mut stdout, &mut stderr);

        assert_eq!(status, 2);
        assert_eq!(stdout.text(), "");
        assert_eq!(stderr.text(), EXPECTED_REFUSAL_LINE);
        assert!(stderr.flushes >= 1, "required diagnostic must be flushed");
    }

    #[test]
    fn a_failed_stdout_write_never_reports_success() {
        let mut stdout = MockSink::write_fails();
        let mut stderr = MockSink::working();

        let status = emit(&accepted_outcome(), &mut stdout, &mut stderr);

        assert_eq!(status, EXIT_INTERNAL, "a lost success line cannot exit 0");
        assert_eq!(stdout.text(), "", "no partial success line may be claimed");
        assert_eq!(stderr.text(), OUTPUT_FAILURE_DIAGNOSTIC);
    }

    #[test]
    fn a_failed_stdout_flush_never_reports_success() {
        let mut stdout = MockSink::flush_fails();
        let mut stderr = MockSink::working();

        let status = emit(&accepted_outcome(), &mut stdout, &mut stderr);

        assert_eq!(status, EXIT_INTERNAL, "an unflushed line cannot exit 0");
        assert_eq!(stderr.text(), OUTPUT_FAILURE_DIAGNOSTIC);
    }

    #[test]
    fn a_failed_stderr_write_never_reports_a_plain_refusal() {
        let mut stdout = MockSink::working();
        let mut stderr = MockSink::write_fails();

        let status = emit(&refused_outcome(), &mut stdout, &mut stderr);

        assert_eq!(status, EXIT_INTERNAL, "a lost diagnostic cannot exit 2");
        assert_eq!(stdout.text(), "", "the refusal must not gain stdout");
    }

    #[test]
    fn a_failed_stderr_flush_never_reports_a_plain_refusal() {
        let mut stdout = MockSink::working();
        let mut stderr = MockSink::flush_fails();

        let status = emit(&refused_outcome(), &mut stdout, &mut stderr);

        assert_eq!(
            status, EXIT_INTERNAL,
            "an unflushed diagnostic cannot exit 2"
        );
    }

    #[test]
    fn the_output_failure_diagnostic_is_fixed_and_safe() {
        assert!(OUTPUT_FAILURE_DIAGNOSTIC.is_ascii());
        assert!(OUTPUT_FAILURE_DIAGNOSTIC.ends_with('\n'));
        assert!(
            !OUTPUT_FAILURE_DIAGNOSTIC
                .chars()
                .any(|character| character.is_control() && character != '\n')
        );
        assert!(
            !OUTPUT_FAILURE_DIAGNOSTIC.contains("injected"),
            "OS error text must never be echoed"
        );
    }

    #[test]
    fn output_failures_are_never_silently_ignored() {
        // If output-error handling were removed, these four cases would return the
        // ordinary 0 / 2 statuses instead of the internal failure status.
        let cases: [FailureCase; 4] = [
            (
                MockSink::write_fails(),
                MockSink::working(),
                accepted_outcome,
            ),
            (
                MockSink::flush_fails(),
                MockSink::working(),
                accepted_outcome,
            ),
            (
                MockSink::working(),
                MockSink::write_fails(),
                refused_outcome,
            ),
            (
                MockSink::working(),
                MockSink::flush_fails(),
                refused_outcome,
            ),
        ];

        for (mut stdout, mut stderr, make_outcome) in cases {
            let status = emit(&make_outcome(), &mut stdout, &mut stderr);
            assert_eq!(status, EXIT_INTERNAL);
            assert_ne!(status, 0);
            assert_ne!(status, 2);
        }
    }
}
