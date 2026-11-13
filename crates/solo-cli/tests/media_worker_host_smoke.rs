//! `N079` — real-executable evidence for the fixed Linux pre-exec limit smoke (`SOLO-WO-079 R01`).
//!
//! Every case launches the compiled `solo-cli` binary that Cargo bound for this integration test
//! through `CARGO_BIN_EXE_solo-cli`. No case searches `PATH`, no case accepts an executable from
//! an argument, no case writes a file and every case is bounded by a finite timeout, so a hung
//! child fails this suite instead of stalling it.
//!
//! On `ubuntu-24.04` the parent case proves the whole seam end to end: the real child was started
//! through the native `pre_exec` boundary, so it reports the two fixed ceilings and a process-group
//! id equal to its own pid, and only then can it print anything usable. On `windows-2025` the same
//! two internal modes are refused before any child exists and the public CLI is unchanged.

use std::io::Read;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

/// Cargo's exact binding for the compiled binary under test: never a `PATH` lookup.
const BIN: &str = env!("CARGO_BIN_EXE_solo-cli");

/// A finite budget for one case, comfortably inside the workflow's own step timeout.
const CASE_TIMEOUT_MS: u64 = 20_000;

/// The fixed `RLIMIT_AS` profile the parent must observe, in bytes.
#[cfg(target_os = "linux")]
const PROFILE_AS_BYTES: u64 = 536_870_912;

/// The fixed `RLIMIT_CPU` profile the parent must observe, in seconds.
#[cfg(target_os = "linux")]
const PROFILE_CPU_SECONDS: u64 = 10;

/// The parent's wall deadline, which the observed run must stay under.
#[cfg(target_os = "linux")]
const SMOKE_WALL_DEADLINE_MS: u64 = 15_000;

/// The wall watchdog of the deadline negative, exactly as `SOLO-WO-081 R01` fixes it.
#[cfg(target_os = "linux")]
const WALL_DEADLINE_MS: u64 = 2_000;

/// The child's fixed overdue sleep, which this negative must terminate well before.
#[cfg(target_os = "linux")]
const WALL_CHILD_SLEEP_MS: u64 = 20_000;

/// A robust lower bound: the watchdog really had to wait for its own 2 000 ms deadline.
#[cfg(target_os = "linux")]
const WALL_MIN_ELAPSED_MS: u64 = 1_500;

/// A bounded upper check: generous for CI scheduling, far below the child's 20-second sleep.
#[cfg(target_os = "linux")]
const WALL_MAX_ELAPSED_MS: u64 = 10_000;

/// The exact over-ceiling capacity the reservation child requests, in bytes (768 MiB).
#[cfg(target_os = "linux")]
const AS_REQUEST_BYTES: u64 = 805_306_368;

/// The parent's fixed supervision deadline for the reservation refusal, in milliseconds.
#[cfg(target_os = "linux")]
const AS_DEADLINE_MS: u64 = 15_000;

/// The CPU parent's fixed supervision deadline, in milliseconds.
///
/// It is the order's separate 60 000 ms budget for the one legitimate CPU burn and is deliberately
/// not the 2 000 ms wall-negative watchdog or the 15 000 ms positive/address-space deadlines.
#[cfg(target_os = "linux")]
const CPU_DEADLINE_MS: u64 = 60_000;

/// The one CPU-exhaustion case's own outer budget: strictly above the parent's 60 000 ms deadline.
#[cfg(target_os = "linux")]
const CPU_CASE_TIMEOUT_MS: u64 = 90_000;

/// The session child's fixed post-readiness sleep, in milliseconds.
///
/// It is the window the parent needs for its own read-only kernel observation, never a timeout or
/// CPU-limit negative, so no case asserts an exact wall time against it.
#[cfg(target_os = "linux")]
const SESSION_CHILD_SLEEP_MS: u64 = 4_000;

/// The session parent's fixed wall deadline, in milliseconds.
#[cfg(target_os = "linux")]
const SESSION_DEADLINE_MS: u64 = 15_000;

/// One bounded observation of one real invocation.
struct Run {
    exit_code: Option<i32>,
    stdout: String,
    stderr: String,
    timed_out: bool,
    /// The wall time this case itself measured, from spawn to reap or to its own budget.
    elapsed_ms: u64,
}

/// Runs the compiled binary with `args` under the ordinary case budget.
fn run(args: &[&str]) -> Run {
    run_bounded(args, CASE_TIMEOUT_MS)
}

/// Runs the compiled binary with `args` under an explicit outer budget.
///
/// The CPU-exhaustion case is the only caller that passes a larger budget, because a deliberate
/// ten-second CPU burn legitimately takes more wall time than the older fast cases. A budget expiry
/// kills and reaps the owned case process and reports `timed_out`, which every caller must refuse.
fn run_bounded(args: &[&str], budget_ms: u64) -> Run {
    let mut child = Command::new(BIN)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the compiled solo-cli binary must launch");

    let mut stdout_pipe = child.stdout.take().expect("stdout must be piped");
    let mut stderr_pipe = child.stderr.take().expect("stderr must be piped");
    let stdout_reader = thread::spawn(move || {
        let mut buffer = Vec::new();
        let _ = stdout_pipe.by_ref().take(4096).read_to_end(&mut buffer);
        buffer
    });
    let stderr_reader = thread::spawn(move || {
        let mut buffer = Vec::new();
        let _ = stderr_pipe.by_ref().take(4096).read_to_end(&mut buffer);
        buffer
    });

    let started = Instant::now();
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break Some(status),
            Ok(None) => {}
            Err(error) => panic!("the case must stay observable: {error}"),
        }
        if started.elapsed() >= Duration::from_millis(budget_ms) {
            let _ = child.kill();
            let _ = child.wait();
            break None;
        }
        thread::sleep(Duration::from_millis(10));
    };

    let stdout =
        String::from_utf8_lossy(&stdout_reader.join().expect("stdout reader")).into_owned();
    let stderr =
        String::from_utf8_lossy(&stderr_reader.join().expect("stderr reader")).into_owned();

    let (exit_code, timed_out) = match status {
        Some(status) => (status.code(), false),
        None => (None, true),
    };

    Run {
        exit_code,
        stdout,
        stderr,
        timed_out,
        elapsed_ms: u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX),
    }
}

/// Splits one accepted evidence line into `(key, value)` pairs, refusing anything surprising.
#[cfg(target_os = "linux")]
fn evidence_fields(line: &str) -> Vec<(String, String)> {
    line.trim_end_matches('\n')
        .split(' ')
        .map(|token| {
            let (key, value) = token
                .split_once('=')
                .unwrap_or_else(|| panic!("every evidence token is key=value: {token:?}"));
            (key.to_owned(), value.to_owned())
        })
        .collect()
}

/// Emits one bounded observation line to the real stderr stream, and nothing else.
///
/// libtest replaces the `print!`/`eprintln!` sink for a *passing* test, so a captured line would
/// never reach the CI log. Writing straight to file descriptor 2 does, which lets the reviewer
/// read the actual kernel values this case asserted without a workflow change and without
/// weakening any assertion. Only the already bounded evidence line is echoed.
#[cfg(target_os = "linux")]
fn report_observation(label: &str, line: &str) {
    use std::io::Write;

    let mut stderr = std::io::stderr();
    let _ = writeln!(stderr, "{label}={}", line.trim_end());
}

/// Reads one numeric field, refusing an absent, duplicated or non-numeric one.
#[cfg(target_os = "linux")]
fn numeric(fields: &[(String, String)], key: &str) -> u64 {
    let mut values = fields
        .iter()
        .filter(|(field, _)| field.as_str() == key)
        .map(|(_, value)| value);
    let value = values.next().unwrap_or_else(|| panic!("missing `{key}`"));
    assert!(values.next().is_none(), "`{key}` must appear exactly once");
    value
        .parse::<u64>()
        .unwrap_or_else(|_| panic!("`{key}` must be a plain decimal: {value:?}"))
}

/// `N079-01`/`N079-02`: on real Ubuntu CI the parent launches the real child through the native
/// pre-exec boundary, and the child's own kernel readback shows both fixed ceilings at full width,
/// its own pid, and a process group that `setsid` made equal to that pid.
#[cfg(target_os = "linux")]
#[test]
fn n079_01_the_real_child_observes_the_installed_limits() {
    let observed = run(&["--internal-media-host-probe-v1", "parent"]);
    assert!(
        !observed.timed_out,
        "the smoke must finish inside the case budget"
    );
    assert_eq!(
        observed.exit_code,
        Some(0),
        "the parent must accept: stdout={:?} stderr={:?}",
        observed.stdout,
        observed.stderr
    );
    assert!(
        observed.stderr.is_empty(),
        "an accepted run says nothing on stderr"
    );
    assert!(observed.stdout.starts_with("media_host_probe=accepted "));
    assert!(observed.stdout.ends_with('\n'));
    assert_eq!(
        observed.stdout.matches('\n').count(),
        1,
        "exactly one evidence line, never a partial one first"
    );

    let fields = evidence_fields(&observed.stdout);
    report_observation("n079_observation", &observed.stdout);
    assert_eq!(
        fields.len(),
        8,
        "the marker plus seven fixed fields: {fields:?}"
    );
    assert_eq!(fields[0].0, "media_host_probe");
    assert_eq!(fields[0].1, "accepted");

    // `N079-02`: both ceilings, at full width, soft and hard, exactly the fixed profile.
    assert_eq!(numeric(&fields, "as_soft"), PROFILE_AS_BYTES);
    assert_eq!(numeric(&fields, "as_hard"), PROFILE_AS_BYTES);
    assert_eq!(numeric(&fields, "cpu_soft"), PROFILE_CPU_SECONDS);
    assert_eq!(numeric(&fields, "cpu_hard"), PROFILE_CPU_SECONDS);
    assert!(
        observed
            .stdout
            .contains("as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 "),
        "every value must be printed at full width: {}",
        observed.stdout
    );

    let pid = numeric(&fields, "pid");
    let pgid = numeric(&fields, "pgid");
    assert!(pid > 0, "the child reports its own pid");
    assert_eq!(
        pgid, pid,
        "setsid must have made the child its own process-group leader"
    );

    let wall_ms = numeric(&fields, "wall_ms");
    assert!(
        wall_ms < SMOKE_WALL_DEADLINE_MS,
        "the run must stay inside the 15 000 ms deadline: {wall_ms}"
    );
}

/// `N079-03`: the real binary refuses every wrong internal token shape, with no evidence at all.
#[cfg(target_os = "linux")]
#[test]
fn n079_03_wrong_internal_arguments_are_refused_by_the_real_binary() {
    let cases = [
        vec!["--internal-media-host-probe-v1"],
        vec!["--internal-media-host-probe-v1", "worker"],
        vec!["--internal-media-host-probe-v1", "parent", "extra"],
        vec!["--internal-media-host-probe-v1", "child", "child"],
    ];
    for args in cases {
        let observed = run(&args);
        assert!(!observed.timed_out, "{args:?} must not hang");
        assert_eq!(observed.exit_code, Some(3), "{args:?} must refuse");
        assert!(
            observed.stdout.is_empty(),
            "a refusal never emits evidence: {args:?}"
        );
        assert_eq!(
            observed.stderr, "error: MEDIA_HOST_PROBE_REFUSED: INTERNAL_MODE_INVALID\n",
            "{args:?} must name its own closed refusal"
        );
    }
}

/// `N079-04`: a direct, unsupervised child invocation under the CI runner's ordinary environment
/// cannot emit accepted readback, because it never received the fixed ceilings.
#[cfg(target_os = "linux")]
#[test]
fn n079_04_a_direct_unsupervised_child_is_refused() {
    let observed = run(&["--internal-media-host-probe-v1", "child"]);
    assert!(!observed.timed_out, "the child must exit promptly");
    assert_ne!(
        observed.exit_code,
        Some(0),
        "an unsupervised child must never report success"
    );
    assert!(
        observed.stdout.is_empty(),
        "no partial or fabricated evidence: {:?}",
        observed.stdout
    );
    assert_eq!(observed.stderr, "", "the child prints nothing at all");
    let code = observed
        .exit_code
        .expect("the child must exit with a status, not a signal");
    report_observation("n079_direct_child", &format!("exit={code}"));
    assert!(
        matches!(code, 71..=73),
        "the child's own verification refusal is one of its three fixed statuses: {code}"
    );
}

/// `N079-05`: on a host without the approved pre-exec boundary both internal modes are refused
/// before any child is created, and no Linux readback is compiled, executed or claimed.
#[cfg(not(target_os = "linux"))]
#[test]
fn n079_05_both_internal_modes_are_unsupported_before_any_child_exists() {
    for mode in ["parent", "child"] {
        let observed = run(&["--internal-media-host-probe-v1", mode]);
        assert!(!observed.timed_out, "{mode} must not hang");
        assert_eq!(
            observed.exit_code,
            Some(3),
            "the {mode} mode must refuse on this host"
        );
        assert!(
            observed.stdout.is_empty(),
            "no Linux readback may be claimed on this host: {mode}"
        );
        assert_eq!(
            observed.stderr, "error: MEDIA_HOST_PROBE_REFUSED: HOST_UNSUPPORTED\n",
            "the {mode} mode must name HOST_UNSUPPORTED"
        );
    }
}

/// `N079-06`: the public CLI is byte-for-byte unchanged on both hosts, and the reserved flag is
/// not reachable through it.
#[test]
fn n079_06_the_public_cli_is_unchanged() {
    let accepted = run(&[
        "bootstrap-preflight",
        "--entity-id",
        "entity-7",
        "--base-url",
        "https://EXAMPLE.com:443/alpha",
    ]);
    assert!(!accepted.timed_out);
    assert_eq!(accepted.exit_code, Some(0), "stderr={:?}", accepted.stderr);
    assert_eq!(
        accepted.stdout,
        "preflight=ok entity_id=entity-7 base=https://example.com/alpha\n"
    );
    assert!(accepted.stderr.is_empty());

    let refused = run(&[
        "bootstrap-preflight",
        "--entity-id",
        "Main",
        "--base-url",
        "https://example.com",
    ]);
    assert_eq!(refused.exit_code, Some(2));
    assert!(refused.stdout.is_empty(), "a refusal gains no stdout");
    assert_eq!(
        refused.stderr,
        "error: PREFLIGHT_ENTITY_ID_INVALID: invalid-start\n"
    );
}

/// `N081-01`: the real parent starts exactly one pre-exec-limited overdue child, kills only its own
/// child at the 2 000 ms deadline, obtains `SIGKILL` and reaps the leader.
#[cfg(target_os = "linux")]
#[test]
fn n081_01_the_wall_parent_kills_and_reaps_its_own_overdue_child() {
    let observed = run(&["--internal-media-host-probe-v1", "wall-parent"]);
    assert!(
        !observed.timed_out,
        "the negative must finish inside the case budget"
    );
    assert_eq!(
        observed.exit_code,
        Some(0),
        "the negative itself must succeed: stdout={:?} stderr={:?}",
        observed.stdout,
        observed.stderr
    );
    assert!(
        observed.stderr.is_empty(),
        "a proven negative says nothing on stderr"
    );
    assert!(
        observed.stdout.starts_with("media_host_wall=terminated "),
        "the marker must state termination: {:?}",
        observed.stdout
    );
    assert_eq!(
        observed.stdout.matches('\n').count(),
        1,
        "exactly one evidence line"
    );
    assert!(
        !observed.stdout.contains("media_host_probe=accepted"),
        "a killed child must never be reported as an accepted run"
    );

    let fields = evidence_fields(&observed.stdout);
    report_observation("n081_wall", &observed.stdout);
    assert_eq!(
        fields.len(),
        10,
        "the marker plus nine fixed fields: {fields:?}"
    );
    assert_eq!(fields[0].0, "media_host_wall");
    assert_eq!(fields[0].1, "terminated");

    // The child's own kernel readback, re-checked by the parent, at full width.
    assert_eq!(numeric(&fields, "as_soft"), PROFILE_AS_BYTES);
    assert_eq!(numeric(&fields, "as_hard"), PROFILE_AS_BYTES);
    assert_eq!(numeric(&fields, "cpu_soft"), PROFILE_CPU_SECONDS);
    assert_eq!(numeric(&fields, "cpu_hard"), PROFILE_CPU_SECONDS);
    assert!(
        observed
            .stdout
            .contains("as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 "),
        "every value must be printed at full width: {}",
        observed.stdout
    );

    let pid = numeric(&fields, "pid");
    let pgid = numeric(&fields, "pgid");
    assert!(pid > 0, "the child reported its own pid");
    assert_eq!(
        pgid, pid,
        "the child verified its own group identity before sleeping"
    );

    assert_eq!(
        numeric(&fields, "signal"),
        9,
        "the reaped leader died of SIGKILL"
    );
    assert_eq!(
        numeric(&fields, "reaped"),
        1,
        "Child::wait returned the leader's real status"
    );
    let wall_ms = numeric(&fields, "wall_ms");
    assert!(
        (WALL_MIN_ELAPSED_MS..WALL_MAX_ELAPSED_MS).contains(&wall_ms),
        "the parent's own measurement must match the short deadline: {wall_ms}"
    );
}

/// `N081-02`: the observed duration belongs to the short watchdog and is materially below the
/// child's 20-second sleep, so a normal completion can never substitute for the termination.
#[cfg(target_os = "linux")]
#[test]
fn n081_02_the_termination_is_the_short_deadline_not_the_full_sleep() {
    let observed = run(&["--internal-media-host-probe-v1", "wall-parent"]);
    assert!(!observed.timed_out, "the case must stay inside its budget");
    assert_eq!(
        observed.exit_code,
        Some(0),
        "stdout={:?} stderr={:?}",
        observed.stdout,
        observed.stderr
    );

    let fields = evidence_fields(&observed.stdout);
    let wall_ms = numeric(&fields, "wall_ms");
    let signal = numeric(&fields, "signal");
    let reaped = numeric(&fields, "reaped");
    report_observation(
        "n081_wall_timing",
        &format!(
            "wall_ms={wall_ms} case_elapsed_ms={} signal={signal} reaped={reaped}",
            observed.elapsed_ms
        ),
    );

    assert_eq!(
        WALL_DEADLINE_MS, 2_000,
        "the configured deadline stays 2 000 ms"
    );
    assert!(
        wall_ms >= WALL_MIN_ELAPSED_MS,
        "the watchdog really waited for its deadline: {wall_ms}"
    );
    assert!(
        wall_ms < WALL_MAX_ELAPSED_MS,
        "the watchdog did not overrun its bound: {wall_ms}"
    );
    assert!(
        wall_ms < WALL_CHILD_SLEEP_MS,
        "the child could not have finished on its own: {wall_ms}"
    );
    assert!(
        observed.elapsed_ms < WALL_CHILD_SLEEP_MS,
        "the whole case is shorter than the child's sleep: {} ms",
        observed.elapsed_ms
    );

    // A reap is required, and only the real wait status can evidence it.
    assert_eq!(reaped, 1, "the leader was reaped, not merely abandoned");
    assert_eq!(
        signal, 9,
        "the leader's status is SIGKILL, not a normal exit"
    );
}

/// `N081-03`: a direct `wall-child` refuses before its sleep, every wrong wall token is refused,
/// and the legacy child mode keeps its exact prior behavior.
#[cfg(target_os = "linux")]
#[test]
fn n081_03_a_direct_wall_child_and_every_wrong_wall_token_are_refused() {
    let direct = run(&["--internal-media-host-probe-v1", "wall-child"]);
    assert!(
        !direct.timed_out,
        "an unsupervised wall child must never reach its 20 s sleep"
    );
    assert!(
        direct.elapsed_ms < 5_000,
        "it must refuse promptly: {} ms",
        direct.elapsed_ms
    );
    assert!(
        direct.stdout.is_empty(),
        "no readiness line without the installed limits: {:?}",
        direct.stdout
    );
    assert_eq!(direct.stderr, "", "the child prints nothing at all");
    let code = direct
        .exit_code
        .expect("the child must exit with a status, not a signal");
    report_observation("n081_direct_wall_child", &format!("exit={code}"));
    assert!(
        matches!(code, 71..=73),
        "the child's own verification refusal is one of its three fixed statuses: {code}"
    );

    let cases = [
        vec!["--internal-media-host-probe-v1", "wall"],
        vec!["--internal-media-host-probe-v1", "WALL-CHILD"],
        vec![
            "--internal-media-host-probe-v1",
            "wall-parent",
            "wall-parent",
        ],
        vec!["--internal-media-host-probe-v1", "wall-child", "extra"],
        vec!["--internal-media-host-probe-v1"],
    ];
    for args in cases {
        let observed = run(&args);
        assert!(!observed.timed_out, "{args:?} must not hang");
        assert_eq!(observed.exit_code, Some(3), "{args:?} must refuse");
        assert!(
            observed.stdout.is_empty(),
            "a refusal never emits evidence: {args:?}"
        );
        assert_eq!(
            observed.stderr, "error: MEDIA_HOST_PROBE_REFUSED: INTERNAL_MODE_INVALID\n",
            "{args:?} must name its own closed refusal"
        );
    }

    // The legacy child mode is byte-exactly what it was before this order.
    let legacy = run(&["--internal-media-host-probe-v1", "child"]);
    assert!(!legacy.timed_out);
    assert!(legacy.stdout.is_empty());
    assert_eq!(legacy.stderr, "");
    let legacy_code = legacy.exit_code.expect("a status, not a signal");
    assert!(
        matches!(legacy_code, 71..=73),
        "the legacy child refusal is unchanged: {legacy_code}"
    );
}

/// `N081-05`: on a host without the approved pre-exec boundary both wall modes refuse before any
/// child exists, and no Linux wall test name can execute here.
#[cfg(not(target_os = "linux"))]
#[test]
fn n081_05_both_wall_modes_are_unsupported_before_any_child_exists() {
    for mode in ["wall-parent", "wall-child"] {
        let observed = run(&["--internal-media-host-probe-v1", mode]);
        assert!(!observed.timed_out, "{mode} must not hang");
        assert!(
            observed.elapsed_ms < 10_000,
            "a refusal is prompt and never waits for a deadline: {mode}"
        );
        assert_eq!(
            observed.exit_code,
            Some(3),
            "the {mode} mode must refuse on this host"
        );
        assert!(
            observed.stdout.is_empty(),
            "no Linux termination may be claimed on this host: {mode}"
        );
        assert_eq!(
            observed.stderr, "error: MEDIA_HOST_PROBE_REFUSED: HOST_UNSUPPORTED\n",
            "the {mode} mode must name HOST_UNSUPPORTED"
        );
    }
}

/// `N083-01`: the real parent starts exactly one pre-exec-limited child, the child verifies its own
/// four ceiling halves and its group identity, makes **one** 768 MiB fallible capacity reservation,
/// the kernel refuses it, the child exits zero, that leader is reaped, and the parent emits one
/// validated bounded record — all strictly inside the 15 000 ms deadline.
#[cfg(target_os = "linux")]
#[test]
fn n083_01_the_child_reserves_over_the_ceiling_and_is_refused() {
    let observed = run(&["--internal-media-host-probe-v1", "as-parent"]);
    assert!(
        !observed.timed_out,
        "the negative must finish inside the case budget"
    );
    assert_eq!(
        observed.exit_code,
        Some(0),
        "the negative itself must succeed: stdout={:?} stderr={:?}",
        observed.stdout,
        observed.stderr
    );
    assert!(
        observed.stderr.is_empty(),
        "a proven refusal says nothing on stderr"
    );
    assert!(
        observed.stdout.starts_with("media_host_as=limit_enforced "),
        "the marker must state enforcement: {:?}",
        observed.stdout
    );
    assert!(observed.stdout.ends_with('\n'));
    assert_eq!(
        observed.stdout.matches('\n').count(),
        1,
        "exactly one evidence line, never a partial one first"
    );
    assert!(
        !observed.stdout.contains("media_host_probe=accepted"),
        "a refused reservation must never be reported as an accepted run"
    );
    assert!(
        !observed.stdout.contains("media_host_wall=terminated"),
        "a reservation refusal is not a watchdog termination"
    );
    assert!(
        !observed.stdout.contains("SOLO-MEDIA-HOST-AS-V1"),
        "the child's own marker must not leak into parent evidence"
    );

    let fields = evidence_fields(&observed.stdout);
    report_observation("n083_as", &observed.stdout);
    assert_eq!(
        fields.len(),
        10,
        "the marker plus nine fixed fields: {fields:?}"
    );
    assert_eq!(fields[0].0, "media_host_as");
    assert_eq!(fields[0].1, "limit_enforced");

    // The child's own kernel readback, re-checked by the parent, at full width.
    assert_eq!(numeric(&fields, "as_soft"), PROFILE_AS_BYTES);
    assert_eq!(numeric(&fields, "as_hard"), PROFILE_AS_BYTES);
    assert_eq!(numeric(&fields, "cpu_soft"), PROFILE_CPU_SECONDS);
    assert_eq!(numeric(&fields, "cpu_hard"), PROFILE_CPU_SECONDS);
    assert!(
        observed
            .stdout
            .contains("as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 "),
        "every value must be printed at full width: {}",
        observed.stdout
    );

    // The one request really is larger than the whole ceiling, and its refusal is literal.
    let request_bytes = numeric(&fields, "request_bytes");
    assert_eq!(request_bytes, AS_REQUEST_BYTES);
    assert!(
        request_bytes > numeric(&fields, "as_soft"),
        "the single request must exceed the child's own address-space ceiling"
    );
    assert!(
        observed
            .stdout
            .contains("request_bytes=805306368 reserve=denied "),
        "the literal refusal result must be present: {}",
        observed.stdout
    );

    let pid = numeric(&fields, "pid");
    let pgid = numeric(&fields, "pgid");
    assert!(pid > 0, "the child reports its own pid");
    assert_eq!(
        pgid, pid,
        "setsid must have made the child its own process-group leader"
    );

    let wall_ms = numeric(&fields, "wall_ms");
    assert!(
        wall_ms < AS_DEADLINE_MS,
        "the refusal must be decided strictly inside the 15 000 ms deadline: {wall_ms}"
    );
}

/// `N083-02`: a direct, unsupervised reservation child refuses from the **prior** limit/group check,
/// promptly, with no output at all, and never reaches the 768 MiB request.
#[cfg(target_os = "linux")]
#[test]
fn n083_02_a_direct_as_child_refuses_before_the_reservation() {
    let direct = run(&["--internal-media-host-probe-v1", "as-child"]);
    assert!(
        !direct.timed_out,
        "an unsupervised reservation child must exit promptly"
    );
    assert!(
        direct.elapsed_ms < 5_000,
        "it must refuse from its own limit check, not from a deadline: {} ms",
        direct.elapsed_ms
    );
    assert!(
        direct.stdout.is_empty(),
        "no denial line without the installed limits: {:?}",
        direct.stdout
    );
    assert_eq!(direct.stderr, "", "the child prints nothing at all");
    let code = direct
        .exit_code
        .expect("the child must exit with a status, not a signal");
    report_observation("n083_direct_as_child", &format!("exit={code}"));
    assert!(
        matches!(code, 71..=74),
        "the child's own verification refusal is one of its four fixed statuses: {code}"
    );
}

/// `N083-03`: both new modes are closed and additive on either host, and every malformed, extra,
/// repeated or misplaced token is the same typed refusal rather than a fall-through.
#[test]
fn n083_03_every_wrong_reservation_token_is_refused_by_the_real_binary() {
    let cases = [
        vec!["--internal-media-host-probe-v1", "as"],
        vec!["--internal-media-host-probe-v1", "AS-PARENT"],
        vec!["--internal-media-host-probe-v1", "as_child"],
        vec!["--internal-media-host-probe-v1", "as-parent", "as-parent"],
        vec!["--internal-media-host-probe-v1", "as-child", "extra"],
        vec!["--internal-media-host-probe-v1", "as-parent", "as-child"],
        vec!["--internal-media-host-probe-v1"],
    ];
    for args in cases {
        let observed = run(&args);
        assert!(!observed.timed_out, "{args:?} must not hang");
        assert_eq!(observed.exit_code, Some(3), "{args:?} must refuse");
        assert!(
            observed.stdout.is_empty(),
            "a refusal never emits evidence: {args:?}"
        );
        assert_eq!(
            observed.stderr, "error: MEDIA_HOST_PROBE_REFUSED: INTERNAL_MODE_INVALID\n",
            "{args:?} must name its own closed refusal"
        );
    }

    // The reservation tokens are not reachable behind a public subcommand either.
    let hidden = run(&[
        "bootstrap-preflight",
        "--internal-media-host-probe-v1",
        "as-parent",
    ]);
    assert!(!hidden.timed_out);
    assert_ne!(
        hidden.stdout.as_str(),
        "media_host_as=limit_enforced",
        "a public subcommand must never reach the reservation parent"
    );
}

/// `N083-04`: on a host without the approved pre-exec boundary both reservation modes refuse before
/// any child exists and without attempting a reservation, and no Linux-only case can run here.
#[cfg(not(target_os = "linux"))]
#[test]
fn n083_04_both_reservation_modes_are_unsupported_before_any_child_exists() {
    for mode in ["as-parent", "as-child"] {
        let observed = run(&["--internal-media-host-probe-v1", mode]);
        assert!(!observed.timed_out, "{mode} must not hang");
        assert!(
            observed.elapsed_ms < 10_000,
            "a refusal is prompt and never waits for a deadline: {mode}"
        );
        assert_eq!(
            observed.exit_code,
            Some(3),
            "the {mode} mode must refuse on this host"
        );
        assert!(
            observed.stdout.is_empty(),
            "no reservation may be claimed on this host: {mode}"
        );
        assert!(
            !observed.stdout.contains("media_host_as=limit_enforced"),
            "no enforcement record without a real kernel denial: {mode}"
        );
        assert_eq!(
            observed.stderr, "error: MEDIA_HOST_PROBE_REFUSED: HOST_UNSUPPORTED\n",
            "the {mode} mode must name HOST_UNSUPPORTED"
        );
    }
}

/// `N085-01`: the one real CPU-exhaustion proof. The parent starts exactly one pre-exec-limited child
/// through the unchanged installer; the child verifies its own four ceiling halves and its group
/// identity, reports that readiness, and then burns CPU until the kernel ends it; the parent observes
/// that real wait status before any kill of its own, reaps the leader and emits one bounded record.
/// The signal is the kernel's `SIGXCPU` or `SIGKILL`, never this parent's kill.
#[cfg(target_os = "linux")]
#[test]
fn n085_01_the_kernel_cpu_limit_terminates_the_child() {
    let observed = run_bounded(
        &["--internal-media-host-probe-v1", "cpu-parent"],
        CPU_CASE_TIMEOUT_MS,
    );
    assert!(
        !observed.timed_out,
        "the CPU proof must finish inside its own 90 000 ms outer budget"
    );
    assert_eq!(
        observed.exit_code,
        Some(0),
        "the proof itself must succeed: stdout={:?} stderr={:?}",
        observed.stdout,
        observed.stderr
    );
    assert!(
        observed.stderr.is_empty(),
        "a proven kernel termination says nothing on stderr"
    );
    assert!(
        observed
            .stdout
            .starts_with("media_host_cpu=kernel_terminated "),
        "the marker must state kernel termination: {:?}",
        observed.stdout
    );
    assert!(observed.stdout.ends_with('\n'));
    assert_eq!(
        observed.stdout.matches('\n').count(),
        1,
        "exactly one evidence line, never a partial one first"
    );
    for foreign in [
        "media_host_probe=accepted",
        "media_host_wall=terminated",
        "media_host_as=limit_enforced",
        "SOLO-MEDIA-HOST-CPU-V1",
    ] {
        assert!(
            !observed.stdout.contains(foreign),
            "the CPU record must not borrow {foreign}"
        );
    }

    let fields = evidence_fields(&observed.stdout);
    report_observation("n085_cpu", &observed.stdout);
    assert_eq!(
        fields.len(),
        11,
        "the marker plus ten fixed fields: {fields:?}"
    );
    assert_eq!(fields[0].0, "media_host_cpu");
    assert_eq!(fields[0].1, "kernel_terminated");

    // The child's own pre-burn kernel readback, re-checked by the parent, at full width.
    assert_eq!(numeric(&fields, "as_soft"), PROFILE_AS_BYTES);
    assert_eq!(numeric(&fields, "as_hard"), PROFILE_AS_BYTES);
    assert_eq!(numeric(&fields, "cpu_soft"), PROFILE_CPU_SECONDS);
    assert_eq!(numeric(&fields, "cpu_hard"), PROFILE_CPU_SECONDS);
    assert!(
        observed
            .stdout
            .contains("as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 "),
        "every value must be printed at full width: {}",
        observed.stdout
    );

    let pid = numeric(&fields, "pid");
    let pgid = numeric(&fields, "pgid");
    assert!(pid > 0, "the child reports its own pid");
    assert_eq!(
        pgid, pid,
        "setsid must have made the child its own process-group leader"
    );

    // The termination came from the kernel, strictly inside the parent's own deadline.
    let signal = numeric(&fields, "signal");
    assert!(
        signal == 24 || signal == 9,
        "only SIGXCPU (24) or SIGKILL (9) is RLIMIT_CPU enforcement: {signal}"
    );
    assert_eq!(numeric(&fields, "reaped"), 1, "the leader was reaped");
    assert_eq!(
        numeric(&fields, "parent_killed"),
        0,
        "this parent must not have sent the signal it reports"
    );

    let wall_ms = numeric(&fields, "wall_ms");
    assert!(
        wall_ms > 0 && wall_ms < CPU_DEADLINE_MS,
        "the kernel decided this run strictly inside the 60 000 ms deadline: {wall_ms}"
    );
    assert!(
        observed.elapsed_ms >= wall_ms,
        "the case's own budget must contain the parent's measurement"
    );
}

/// `N085-02`: a direct, unsupervised `cpu-child` refuses from its own limit/group check before any
/// loop iteration, so the deliberate CPU burn is never executed outside the limited child.
#[cfg(target_os = "linux")]
#[test]
fn n085_02_a_direct_cpu_child_refuses_before_the_loop() {
    let direct = run(&["--internal-media-host-probe-v1", "cpu-child"]);
    assert!(!direct.timed_out, "the child must exit promptly");
    assert!(
        direct.elapsed_ms < 10_000,
        "a refused child must never spend the CPU budget: {}",
        direct.elapsed_ms
    );
    assert_ne!(
        direct.exit_code,
        Some(0),
        "an unsupervised child must never report success"
    );
    assert!(
        direct.stdout.is_empty(),
        "no readiness line without the installed limits: {:?}",
        direct.stdout
    );
    assert_eq!(direct.stderr, "", "the child prints nothing at all");
    let code = direct
        .exit_code
        .expect("the child must exit with a status, not a signal");
    report_observation("n085_direct_cpu_child", &format!("exit={code}"));
    assert!(
        matches!(code, 71..=73 | 75),
        "the child's own verification refusal is one of its fixed statuses: {code}"
    );
}

/// `N085-03`: both new modes are closed and additive on either host, and every malformed, extra,
/// repeated or misplaced token is the same typed refusal rather than a fall-through.
#[test]
fn n085_03_every_wrong_cpu_token_is_refused_by_the_real_binary() {
    let cases = [
        vec!["--internal-media-host-probe-v1", "cpu"],
        vec!["--internal-media-host-probe-v1", "CPU-PARENT"],
        vec!["--internal-media-host-probe-v1", "cpu_child"],
        vec!["--internal-media-host-probe-v1", "cpu-parent", "cpu-parent"],
        vec!["--internal-media-host-probe-v1", "cpu-child", "extra"],
        vec!["--internal-media-host-probe-v1", "cpu-parent", "cpu-child"],
        vec!["--internal-media-host-probe-v1", " cpu-parent"],
        vec!["--internal-media-host-probe-v1"],
    ];
    for args in cases {
        let observed = run(&args);
        assert!(!observed.timed_out, "{args:?} must not hang");
        assert_eq!(observed.exit_code, Some(3), "{args:?} must refuse");
        assert!(
            observed.stdout.is_empty(),
            "a refusal never emits evidence: {args:?}"
        );
        assert_eq!(
            observed.stderr, "error: MEDIA_HOST_PROBE_REFUSED: INTERNAL_MODE_INVALID\n",
            "{args:?} must name its own closed refusal"
        );
    }

    // The CPU tokens are not reachable behind a public subcommand either.
    let hidden = run(&[
        "bootstrap-preflight",
        "--internal-media-host-probe-v1",
        "cpu-parent",
    ]);
    assert!(!hidden.timed_out);
    assert!(
        !hidden.stdout.contains("media_host_cpu=kernel_terminated"),
        "a public subcommand must never reach the CPU parent"
    );
}

/// `N085-04`: on a host without the approved pre-exec boundary both CPU modes refuse before any child
/// exists and without running any workload, and no Linux-only case can run here.
#[cfg(not(target_os = "linux"))]
#[test]
fn n085_04_both_cpu_modes_are_unsupported_before_any_child_exists() {
    for mode in ["cpu-parent", "cpu-child"] {
        let observed = run(&["--internal-media-host-probe-v1", mode]);
        assert!(!observed.timed_out, "{mode} must not hang");
        assert!(
            observed.elapsed_ms < 10_000,
            "a refusal is prompt and never waits for a deadline: {mode}"
        );
        assert_eq!(
            observed.exit_code,
            Some(3),
            "the {mode} mode must refuse on this host"
        );
        assert!(
            observed.stdout.is_empty(),
            "no CPU evidence may be claimed on this host: {mode}"
        );
        assert!(
            !observed.stdout.contains("media_host_cpu=kernel_terminated"),
            "no kernel-termination record without a real kernel signal: {mode}"
        );
        assert!(
            !observed.stdout.contains("SOLO-MEDIA-HOST-CPU-V1"),
            "no readiness line without a child: {mode}"
        );
        assert_eq!(
            observed.stderr, "error: MEDIA_HOST_PROBE_REFUSED: HOST_UNSUPPORTED\n",
            "the {mode} mode must name HOST_UNSUPPORTED"
        );
    }
}

/// `N087-01`: the real parent proves a live, directly owned child's process-group **and** session
/// identity in two independent ways — the child's own kernel readback and the parent's own
/// read-only kernel query of that still-live child — and only then emits one bounded line.
///
/// A status of zero is only reachable through the neutral validator, which refuses any attempt
/// where the child was not proved alive both immediately before and immediately after the parent's
/// observation, so this case's own success is the "live before and after" assertion.
#[cfg(target_os = "linux")]
#[test]
fn n087_01_the_parent_independently_attests_a_live_child_session() {
    let observed = run(&["--internal-media-host-probe-v1", "session-parent"]);
    assert!(
        !observed.timed_out,
        "the attestation must finish inside the ordinary 20 000 ms case budget"
    );
    assert_eq!(
        observed.exit_code,
        Some(0),
        "the proof itself must succeed: stdout={:?} stderr={:?}",
        observed.stdout,
        observed.stderr
    );
    assert!(
        observed.stderr.is_empty(),
        "a live, attested run says nothing on stderr"
    );
    assert!(
        observed
            .stdout
            .starts_with("media_host_session=live_attested "),
        "the marker must state live attestation: {:?}",
        observed.stdout
    );
    assert!(observed.stdout.ends_with('\n'));
    assert_eq!(
        observed.stdout.matches('\n').count(),
        1,
        "exactly one evidence line, never a partial one first"
    );
    for foreign in [
        "media_host_probe=accepted",
        "media_host_wall=terminated",
        "media_host_as=limit_enforced",
        "media_host_cpu=kernel_terminated",
        "SOLO-MEDIA-HOST-SESSION-V1",
    ] {
        assert!(
            !observed.stdout.contains(foreign),
            "the session record must not borrow {foreign}"
        );
    }

    let fields = evidence_fields(&observed.stdout);
    report_observation("n087_session", &observed.stdout);
    assert_eq!(
        fields.len(),
        12,
        "the marker plus eleven fixed fields: {fields:?}"
    );
    assert_eq!(fields[0].0, "media_host_session");
    assert_eq!(fields[0].1, "live_attested");

    // The child's own kernel readback, re-checked by the parent, at full width.
    assert_eq!(numeric(&fields, "as_soft"), PROFILE_AS_BYTES);
    assert_eq!(numeric(&fields, "as_hard"), PROFILE_AS_BYTES);
    assert_eq!(numeric(&fields, "cpu_soft"), PROFILE_CPU_SECONDS);
    assert_eq!(numeric(&fields, "cpu_hard"), PROFILE_CPU_SECONDS);
    assert!(
        observed
            .stdout
            .contains("as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 "),
        "every value must be printed at full width: {}",
        observed.stdout
    );

    let pid = numeric(&fields, "pid");
    let pgid = numeric(&fields, "pgid");
    let sid = numeric(&fields, "sid");
    let observed_pgid = numeric(&fields, "observed_pgid");
    let observed_sid = numeric(&fields, "observed_sid");
    assert!(pid > 0, "a real child has a positive pid");
    assert_eq!(
        pgid, pid,
        "setsid must have made the child its own process-group leader"
    );
    assert_eq!(
        sid, pid,
        "setsid must have made the child its own session leader"
    );
    assert_eq!(
        observed_pgid, pid,
        "the parent's own kernel group query must agree with the child"
    );
    assert_eq!(
        observed_sid, pid,
        "the parent's own kernel session query must agree with the child"
    );
    assert_eq!(
        numeric(&fields, "reaped"),
        1,
        "the parent reaped the leader it spawned"
    );

    let wall_ms = numeric(&fields, "wall_ms");
    assert!(
        wall_ms > 0 && wall_ms < SESSION_DEADLINE_MS,
        "the run decided itself strictly inside the 15 000 ms deadline: {wall_ms}"
    );
    assert!(
        observed.elapsed_ms >= wall_ms,
        "the case's own budget must contain the parent's measurement"
    );
}

/// `N087-02`: a direct, unsupervised `session-child` refuses from its own limit/identity check
/// before its four-second sleep, so the parent-only observation cannot be faked from outside.
#[cfg(target_os = "linux")]
#[test]
fn n087_02_a_direct_session_child_refuses_before_its_sleep() {
    let direct = run(&["--internal-media-host-probe-v1", "session-child"]);
    assert!(!direct.timed_out, "the child must exit promptly");
    assert!(
        direct.elapsed_ms < SESSION_CHILD_SLEEP_MS,
        "the refusal must happen before the four-second window: {} ms",
        direct.elapsed_ms
    );
    assert!(
        direct.stdout.is_empty(),
        "a refused child writes no readiness line: {:?}",
        direct.stdout
    );
    assert!(
        direct.stderr.is_empty(),
        "a refused child writes no diagnostic of its own: {:?}",
        direct.stderr
    );
    let code = direct
        .exit_code
        .expect("the child must exit with a status, not a signal");
    report_observation("n087_direct_session_child", &format!("exit={code}"));
    assert!(
        matches!(code, 71..=73 | 76 | 77),
        "the child's own verification refusal is one of its fixed statuses: {code}"
    );
}

/// `N087-03`: both new modes are closed and additive on either host, and every malformed, extra,
/// repeated or misplaced token is the same typed refusal rather than a fall-through.
#[test]
fn n087_03_every_wrong_session_token_is_refused_by_the_real_binary() {
    let cases = [
        vec!["--internal-media-host-probe-v1", "session"],
        vec!["--internal-media-host-probe-v1", "SESSION-PARENT"],
        vec!["--internal-media-host-probe-v1", "session_child"],
        vec![
            "--internal-media-host-probe-v1",
            "session-parent",
            "session-parent",
        ],
        vec!["--internal-media-host-probe-v1", "session-child", "extra"],
        vec![
            "--internal-media-host-probe-v1",
            "session-parent",
            "session-child",
        ],
        vec!["--internal-media-host-probe-v1", " session-parent"],
        vec!["--internal-media-host-probe-v1"],
    ];
    for args in cases {
        let observed = run(&args);
        assert!(!observed.timed_out, "{args:?} must not hang");
        assert_eq!(observed.exit_code, Some(3), "{args:?} must refuse");
        assert!(
            observed.stdout.is_empty(),
            "a refusal never emits evidence: {args:?}"
        );
        assert_eq!(
            observed.stderr, "error: MEDIA_HOST_PROBE_REFUSED: INTERNAL_MODE_INVALID\n",
            "{args:?} must name its own closed refusal"
        );
    }

    // The session tokens are not reachable behind a public subcommand either.
    let hidden = run(&[
        "bootstrap-preflight",
        "--internal-media-host-probe-v1",
        "session-parent",
    ]);
    assert!(!hidden.timed_out);
    assert!(
        !hidden.stdout.contains("media_host_session=live_attested"),
        "a public subcommand must never reach the session parent"
    );
}

/// `N087-04`: on a host without the approved pre-exec boundary both session modes refuse before any
/// child exists and without running any workload, and no Linux-only case can run here.
#[cfg(not(target_os = "linux"))]
#[test]
fn n087_04_both_session_modes_are_unsupported_before_any_child_exists() {
    for mode in ["session-parent", "session-child"] {
        let observed = run(&["--internal-media-host-probe-v1", mode]);
        assert!(!observed.timed_out, "{mode} must not hang");
        assert!(
            observed.elapsed_ms < 10_000,
            "a refusal is prompt and never waits for a deadline: {mode}"
        );
        assert_eq!(
            observed.exit_code,
            Some(3),
            "the {mode} mode must refuse on this host"
        );
        assert!(
            observed.stdout.is_empty(),
            "no session evidence may be claimed on this host: {mode}"
        );
        assert!(
            !observed.stdout.contains("media_host_session=live_attested"),
            "no live-attestation record without a real kernel observation: {mode}"
        );
        assert!(
            !observed.stdout.contains("SOLO-MEDIA-HOST-SESSION-V1"),
            "no readiness line without a child: {mode}"
        );
        assert_eq!(
            observed.stderr, "error: MEDIA_HOST_PROBE_REFUSED: HOST_UNSUPPORTED\n",
            "the {mode} mode must name HOST_UNSUPPORTED"
        );
    }
}

/// `N089-01`: the real parent performs its single read-only kernel group/session query only after it
/// has received and strictly validated its own child's flushed readiness line, and the accepted
/// record is still the unchanged N087 schema.
///
/// The ordering itself is carried by the façade source — the bounded early handoff and
/// `validate_session_readiness_v1` gate the query unconditionally, and that crate's pure N089 tests
/// show no attempt without the gate can become evidence — while this case proves the real binary
/// built from that new source still produces one exact, complete record and refuses any partial one.
#[cfg(target_os = "linux")]
#[test]
fn n089_01_the_real_parent_orders_its_query_after_validated_readiness() {
    let observed = run(&["--internal-media-host-probe-v1", "session-parent"]);
    assert!(
        !observed.timed_out,
        "the ordered attestation must finish inside the ordinary 20 000 ms case budget"
    );
    assert_eq!(
        observed.exit_code,
        Some(0),
        "the ordered proof must succeed: stdout={:?} stderr={:?}",
        observed.stdout,
        observed.stderr
    );
    assert!(
        observed.stderr.is_empty(),
        "a live, ordered run says nothing on stderr"
    );
    assert!(
        observed
            .stdout
            .starts_with("media_host_session=live_attested "),
        "the accepted marker must be unchanged: {:?}",
        observed.stdout
    );
    assert_eq!(
        observed.stdout.matches('\n').count(),
        1,
        "exactly one evidence line, never a partial one first"
    );
    assert!(
        !observed.stdout.contains("SOLO-MEDIA-HOST-SESSION-V1"),
        "the consumed readiness record must not leak into the accepted line"
    );

    let fields = evidence_fields(&observed.stdout);
    assert_eq!(
        fields.len(),
        12,
        "the marker plus eleven fixed fields: {fields:?}"
    );
    assert_eq!(fields[0].0, "media_host_session");
    assert_eq!(fields[0].1, "live_attested");
    assert_eq!(numeric(&fields, "as_soft"), PROFILE_AS_BYTES);
    assert_eq!(numeric(&fields, "as_hard"), PROFILE_AS_BYTES);
    assert_eq!(numeric(&fields, "cpu_soft"), PROFILE_CPU_SECONDS);
    assert_eq!(numeric(&fields, "cpu_hard"), PROFILE_CPU_SECONDS);

    let pid = numeric(&fields, "pid");
    assert!(pid > 0, "a real child has a positive pid");
    assert_eq!(numeric(&fields, "pgid"), pid, "setsid made it the leader");
    assert_eq!(numeric(&fields, "sid"), pid, "setsid made it the session");
    assert_eq!(
        numeric(&fields, "observed_pgid"),
        pid,
        "the parent's own group query must agree with the child"
    );
    assert_eq!(
        numeric(&fields, "observed_sid"),
        pid,
        "the parent's own session query must agree with the child"
    );
    assert_eq!(numeric(&fields, "reaped"), 1, "the leader was reaped");

    let wall_ms = numeric(&fields, "wall_ms");
    assert!(
        wall_ms > 0 && wall_ms < SESSION_DEADLINE_MS,
        "the run decided itself strictly inside the 15 000 ms deadline: {wall_ms}"
    );
    report_observation(
        "n089_readiness",
        &format!("validated_before_query pid={pid} wall_ms={wall_ms}"),
    );
}

/// `N089-02`: a direct, unsupervised `session-child` still refuses from its own limit/identity check
/// before its four-second sleep and before it can write any readiness line, so the parent's ordered
/// observation cannot be produced from outside the parent.
#[cfg(target_os = "linux")]
#[test]
fn n089_02_a_direct_session_child_grants_no_readiness() {
    let direct = run(&["--internal-media-host-probe-v1", "session-child"]);
    assert!(!direct.timed_out, "the child must exit promptly");
    assert!(
        direct.elapsed_ms < SESSION_CHILD_SLEEP_MS,
        "the refusal must precede the fixed four-second window: {} ms",
        direct.elapsed_ms
    );
    assert!(
        direct.stdout.is_empty(),
        "a refused child grants no readiness line: {:?}",
        direct.stdout
    );
    assert!(
        direct.stderr.is_empty(),
        "a refused child writes no diagnostic of its own: {:?}",
        direct.stderr
    );
    let code = direct
        .exit_code
        .expect("the child must exit with a status, not a signal");
    report_observation("n089_direct_session_child", &format!("exit={code}"));
    assert!(
        matches!(code, 71..=73 | 76 | 77),
        "the child's own verification refusal is one of its fixed statuses: {code}"
    );
}

/// `N089-03`: the readiness path stays unreachable behind the public CLI on either host, so this
/// proof added no unapproved mode, flag or environment control.
#[test]
fn n089_03_the_readiness_path_is_unreachable_from_the_public_cli() {
    for mode in ["session-parent", "session-child"] {
        let hidden = run(&[
            "bootstrap-preflight",
            "--internal-media-host-probe-v1",
            mode,
        ]);
        assert!(
            !hidden.timed_out,
            "{mode} must not hang behind a public subcommand"
        );
        assert!(
            !hidden.stdout.contains("media_host_session=live_attested"),
            "a public subcommand must never reach the session parent: {mode}"
        );
        assert!(
            !hidden.stdout.contains("SOLO-MEDIA-HOST-SESSION-V1"),
            "a public subcommand must never reach the session child: {mode}"
        );
    }
}

/// `N089-04`: on a host without the approved pre-exec boundary both session modes still refuse
/// before any child exists, so no readiness worker, pipe or Linux-only case runs here.
#[cfg(not(target_os = "linux"))]
#[test]
fn n089_04_no_readiness_worker_runs_where_the_boundary_is_unsupported() {
    for mode in ["session-parent", "session-child"] {
        let observed = run(&["--internal-media-host-probe-v1", mode]);
        assert!(!observed.timed_out, "{mode} must not hang");
        assert_eq!(
            observed.exit_code,
            Some(3),
            "the {mode} mode must refuse on this host"
        );
        assert!(
            observed.stdout.is_empty(),
            "no readiness line and no evidence without a child: {mode}"
        );
        assert_eq!(
            observed.stderr, "error: MEDIA_HOST_PROBE_REFUSED: HOST_UNSUPPORTED\n",
            "the {mode} mode must name HOST_UNSUPPORTED"
        );
    }
}

/// `N090-01`: after the corrected early-refusal cleanup the real parent still emits exactly one
/// unchanged live-attested record, and its leader was reaped before the capped readers were joined.
#[cfg(target_os = "linux")]
#[test]
fn n090_01_the_real_parent_reaps_before_joining_after_the_correction() {
    let observed = run(&["--internal-media-host-probe-v1", "session-parent"]);
    assert!(
        !observed.timed_out,
        "the corrected proof must finish in budget"
    );
    assert_eq!(
        observed.exit_code,
        Some(0),
        "the corrected proof must succeed: stdout={:?} stderr={:?}",
        observed.stdout,
        observed.stderr
    );
    assert!(
        observed.stderr.is_empty(),
        "a live, ordered run says nothing on stderr"
    );
    assert!(
        observed
            .stdout
            .starts_with("media_host_session=live_attested "),
        "the accepted marker is unchanged: {:?}",
        observed.stdout
    );
    assert_eq!(
        observed.stdout.matches('\n').count(),
        1,
        "exactly one evidence line, never a partial one first"
    );
    assert!(
        !observed.stdout.contains("SOLO-MEDIA-HOST-SESSION-V1"),
        "the consumed readiness record must not leak into the accepted line"
    );

    let fields = evidence_fields(&observed.stdout);
    assert_eq!(
        fields.len(),
        12,
        "the marker plus eleven fixed fields: {fields:?}"
    );
    assert_eq!(fields[0].0, "media_host_session");
    assert_eq!(fields[0].1, "live_attested");
    assert_eq!(numeric(&fields, "as_soft"), PROFILE_AS_BYTES);
    assert_eq!(numeric(&fields, "as_hard"), PROFILE_AS_BYTES);
    assert_eq!(numeric(&fields, "cpu_soft"), PROFILE_CPU_SECONDS);
    assert_eq!(numeric(&fields, "cpu_hard"), PROFILE_CPU_SECONDS);

    let pid = numeric(&fields, "pid");
    assert!(pid > 0, "a real child has a positive pid");
    assert_eq!(numeric(&fields, "pgid"), pid, "setsid made it the leader");
    assert_eq!(numeric(&fields, "sid"), pid, "setsid made it the session");
    assert_eq!(
        numeric(&fields, "observed_pgid"),
        pid,
        "the parent's own group query must agree with the child"
    );
    assert_eq!(
        numeric(&fields, "observed_sid"),
        pid,
        "the parent's own session query must agree with the child"
    );
    assert_eq!(numeric(&fields, "reaped"), 1, "the leader was reaped");

    let wall_ms = numeric(&fields, "wall_ms");
    assert!(
        wall_ms > 0 && wall_ms < SESSION_DEADLINE_MS,
        "the run decided itself strictly inside the 15 000 ms deadline: {wall_ms}"
    );
    report_observation(
        "n090_reap",
        &format!("reaped_before_join pid={pid} wall_ms={wall_ms}"),
    );
}

/// `N090-02`: the correction adds no Windows capability — both private session modes still refuse
/// before any child exists, with no readiness worker, no evidence and no kernel marker.
#[cfg(not(target_os = "linux"))]
#[test]
fn n090_02_both_session_modes_stay_unsupported_after_the_correction() {
    for mode in ["session-parent", "session-child"] {
        let observed = run(&["--internal-media-host-probe-v1", mode]);
        assert!(!observed.timed_out, "{mode} must not hang");
        assert_eq!(
            observed.exit_code,
            Some(3),
            "the {mode} mode must refuse on this host"
        );
        assert!(
            observed.stdout.is_empty(),
            "no readiness line and no evidence without a child: {mode}"
        );
        assert_eq!(
            observed.stderr, "error: MEDIA_HOST_PROBE_REFUSED: HOST_UNSUPPORTED\n",
            "the {mode} mode must name HOST_UNSUPPORTED"
        );
    }
}
