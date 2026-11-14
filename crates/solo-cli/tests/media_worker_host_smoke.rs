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

/// The group parent's single fixed deadline, in milliseconds.
///
/// It bounds the readiness handoff, both identity proofs, the one group signal, the leader reap and
/// the descendant observation together, and it starts before leader spawn.
#[cfg(target_os = "linux")]
const GROUP_DEADLINE_MS: u64 = 15_000;

/// `N093-01`: on real Ubuntu CI the fixed parent proves both kernel identities of one live leader and
/// one live descendant, sends exactly one `SIGKILL` to the group it owns, reaps its own leader, and
/// independently observes that same descendant no longer running.
#[cfg(target_os = "linux")]
#[test]
fn n093_01_the_real_parent_terminates_its_owned_group_and_observes_the_descendant() {
    let observed = run(&["--internal-media-host-probe-v1", "group-parent"]);
    assert!(
        !observed.timed_out,
        "the group proof must finish inside its own budget"
    );
    assert_eq!(
        observed.exit_code,
        Some(0),
        "the group proof must succeed: stdout={:?} stderr={:?}",
        observed.stdout,
        observed.stderr
    );
    assert!(
        observed.stderr.is_empty(),
        "a live, orderly group run says nothing on stderr"
    );
    assert!(
        observed.stdout.starts_with("media_host_group=terminated "),
        "the accepted group marker is exact: {:?}",
        observed.stdout
    );
    assert_eq!(
        observed.stdout.matches('\n').count(),
        1,
        "exactly one evidence line, never a partial one first"
    );
    assert!(
        !observed.stdout.contains("SOLO-MEDIA-HOST-GROUP-V1")
            && !observed.stdout.contains("SOLO-MEDIA-HOST-GROUP-DESC-V1"),
        "neither readiness record may leak into the accepted line"
    );

    let fields = evidence_fields(&observed.stdout);
    assert_eq!(
        fields.len(),
        9,
        "the marker plus eight fixed fields: {fields:?}"
    );
    assert_eq!(fields[0].0, "media_host_group");
    assert_eq!(fields[0].1, "terminated");

    let leader = numeric(&fields, "leader_pid");
    let descendant = numeric(&fields, "descendant_pid");
    assert!(leader > 1, "a real leader has a positive pid");
    assert!(descendant > 1, "a real descendant has a positive pid");
    assert_ne!(
        descendant, leader,
        "the descendant must be a different process from its leader"
    );
    assert_eq!(
        numeric(&fields, "pgid"),
        leader,
        "the group is the leader's own pid"
    );
    assert_eq!(
        numeric(&fields, "sid"),
        leader,
        "the session is the leader's own pid"
    );
    assert_eq!(numeric(&fields, "signal"), 9, "the one signal is SIGKILL");
    assert_eq!(
        numeric(&fields, "leader_reaped"),
        1,
        "the leader was reaped"
    );
    assert_eq!(
        numeric(&fields, "descendant_nonrunning"),
        1,
        "the same descendant was observed non-running"
    );

    let wall_ms = numeric(&fields, "wall_ms");
    assert!(
        wall_ms > 0 && wall_ms < GROUP_DEADLINE_MS,
        "the run decided itself strictly inside its 15 000 ms deadline: {wall_ms}"
    );
    report_observation(
        "n093_group",
        &format!(
            "terminated leader_pid={leader} descendant_pid={descendant} pgid={leader} \
             sid={leader} signal=9 leader_reaped=1 descendant_nonrunning=1 wall_ms={wall_ms}"
        ),
    );
}

/// `N093-02`: on real Ubuntu CI the fixed invalid-readiness path produces exactly one typed refusal
/// and no accepted output, and — by construction, since the signal path is unreachable without an
/// accepted record — zero group-target `kill` calls. The leader itself is still reaped with a checked
/// wait, or the reap failure would override the refusal.
#[cfg(target_os = "linux")]
#[test]
fn n093_02_the_invalid_readiness_path_refuses_before_any_group_signal() {
    let observed = run(&["--internal-media-host-probe-v1", "group-invalid-parent"]);
    assert!(
        !observed.timed_out,
        "the negative must decide itself inside the fixed budget"
    );
    assert_eq!(
        observed.exit_code,
        Some(3),
        "the negative must refuse with the internal status: stdout={:?} stderr={:?}",
        observed.stdout,
        observed.stderr
    );
    assert!(
        observed.stdout.is_empty(),
        "the negative must print no accepted output at all: {:?}",
        observed.stdout
    );
    assert_eq!(
        observed.stderr, "error: MEDIA_HOST_PROBE_REFUSED: HOST_OUTPUT_INVALID\n",
        "the negative must name the output refusal before any identity authority"
    );
    report_observation(
        "n093_refusal",
        &format!(
            "invalid_readiness=HOST_OUTPUT_INVALID stdout_bytes=0 group_signal=none \
             leader_cleanup=checked wall_ms={}",
            observed.elapsed_ms
        ),
    );
}

/// `N093-03`: a direct, unsupervised leader invocation and every wrong, duplicated, extra or
/// misordered token refuse, print no evidence and launch no grandchild. A direct descendant may
/// legitimately attest only its own inherited identity, so it must at least never emit evidence.
#[cfg(target_os = "linux")]
#[test]
fn n093_03_direct_group_invocations_and_every_wrong_token_refuse() {
    for mode in ["group-leader", "group-invalid-leader"] {
        let observed = run(&["--internal-media-host-probe-v1", mode]);
        assert!(!observed.timed_out, "{mode} must not hang");
        let code = observed
            .exit_code
            .expect("a direct group mode must exit with a status, not a signal");
        report_observation("n093_direct", &format!("{mode} exit={code}"));
        assert!(
            matches!(code, 71 | 77),
            "a direct {mode} refuses its inherited ceilings or its identity: {code}"
        );
        assert!(
            observed.stdout.is_empty(),
            "{mode} must print neither readiness nor evidence when invoked directly"
        );
    }

    let descendant = run(&["--internal-media-host-probe-v1", "group-descendant"]);
    assert!(!descendant.timed_out, "a direct descendant must not hang");
    assert!(
        matches!(
            descendant.exit_code,
            Some(0) | Some(71) | Some(77) | Some(79)
        ),
        "a direct descendant refuses its inherited ceilings or identity, or merely attests \
         its own inherited group: {:?}",
        descendant.exit_code
    );
    assert!(
        !descendant.stdout.contains("media_host_group="),
        "a direct descendant can never emit group evidence"
    );

    for tokens in [
        vec!["--internal-media-host-probe-v1"],
        vec!["--internal-media-host-probe-v1", "group"],
        vec!["--internal-media-host-probe-v1", "GROUP-PARENT"],
        vec!["--internal-media-host-probe-v1", "group-parents"],
        vec![
            "--internal-media-host-probe-v1",
            "group-parent",
            "group-parent",
        ],
        vec!["--internal-media-host-probe-v1", "group-parent", "extra"],
    ] {
        let observed = run(&tokens);
        assert!(!observed.timed_out, "a wrong token must decide immediately");
        assert_eq!(
            observed.exit_code,
            Some(3),
            "a wrong token must refuse: {tokens:?}"
        );
        assert!(observed.stdout.is_empty(), "a wrong token prints nothing");
        assert_eq!(
            observed.stderr, "error: MEDIA_HOST_PROBE_REFUSED: INTERNAL_MODE_INVALID\n",
            "a wrong token must name the closed mode refusal: {tokens:?}"
        );
    }
}

/// `N093-04`: on `windows-2025` every new group mode refuses before any spawn, kernel query, sleep or
/// signal, and the earlier Windows behaviour is unchanged.
#[cfg(not(target_os = "linux"))]
#[test]
fn n093_04_every_group_mode_is_unsupported_before_any_child_exists() {
    for mode in [
        "group-parent",
        "group-leader",
        "group-descendant",
        "group-invalid-parent",
        "group-invalid-leader",
    ] {
        let observed = run(&["--internal-media-host-probe-v1", mode]);
        assert!(!observed.timed_out, "{mode} must not hang");
        assert_eq!(
            observed.exit_code,
            Some(3),
            "the {mode} mode must refuse on this host"
        );
        assert!(
            observed.stdout.is_empty(),
            "no group record and no evidence without a child: {mode}"
        );
        assert_eq!(
            observed.stderr, "error: MEDIA_HOST_PROBE_REFUSED: HOST_UNSUPPORTED\n",
            "the {mode} mode must name HOST_UNSUPPORTED"
        );
    }
}

/// `N093-06`: the public CLI is unchanged and every earlier evidence marker keeps its exact meaning,
/// so no group result can be confused with an earlier proof.
#[test]
fn n093_06_the_public_cli_and_every_earlier_marker_are_unchanged() {
    let public = run(&["group-parent"]);
    assert_eq!(
        public.exit_code,
        Some(2),
        "the public CLI still refuses an unknown subcommand"
    );
    assert!(
        public.stdout.is_empty(),
        "the public CLI prints no evidence for an unknown subcommand"
    );

    let markers = [
        "media_host_wall=terminated",
        "media_host_as=limit_enforced",
        "media_host_cpu=kernel_terminated",
        "media_host_session=live_attested",
        "media_host_probe=accepted",
    ];
    for marker in markers {
        assert_ne!(
            marker, "media_host_group=terminated",
            "the group marker must stay distinct from every earlier marker"
        );
    }
}

/// `R03.C3` — confined Entity image-byte inspection.
///
/// These cases run the **real** composition: a confined content root is adopted, the accepted
/// `R03.C2` reader acquires the authored `assets/…` bytes, and the `R03.C3` adapter starts the real
/// `solo-cli` worker under the platform confinement. No case trusts a worker's printed claim, and no
/// case substitutes a fake port for a decode.
mod n097 {
    use std::fs;
    use std::path::{Path, PathBuf};

    use solo_adapters::{
        ContentRootCapability, EntityImageOutcomeV1, EntityImageRefusalV1,
        IMAGE_WORKER_MODE_REFUSE_V1, IMAGE_WORKER_MODE_SHORT_OUTPUT_V1,
        IMAGE_WORKER_MODE_TRAILING_V1, IMAGE_WORKER_MODE_WRONG_OUTPUT_V1,
        IMAGE_WORKER_MODE_WRONG_SOURCE_V1, RootProvenanceV1, TestOwnerV1,
        inspect_entity_graphic_images_v1, inspect_image_bytes_with_mode_v1, test_inventory_v1,
    };
    use solo_application::{
        GraphicAssetObligationInventoryV1, GraphicImageFormatV1, GraphicImageInspectPortOutcomeV1,
        GraphicImageInspectRefusalKindV1, GraphicImageInspectionOutcomeV1,
        MAX_IMAGE_SOURCE_BYTES_V1, inspect_graphic_image_source_set_v1,
    };

    /// The one worker binary these cases start.
    const BIN: &str = super::BIN;

    /// The trusted provenance marker the confined source reader records for an adopted root.
    const TRUSTED: RootProvenanceV1 = RootProvenanceV1::OperatorDesignatedContentRoot;

    /// A four-pixel 2×2 PNG generated for this test: red, green, blue, white in that order.
    const FIXTURE_PNG: &str = "iVBORw0KGgoAAAANSUhEUgAAAAIAAAACCAYAAABytg0kAAAAFklEQVR4AWL6z8DwHwSZGEEkw38GAAAAAP//4+5legAAAAZJREFUAwBBDwb/5c5bAAAAAABJRU5ErkJggg==";

    /// The same four pixels encoded as a 2×2 baseline JPEG, generated for this test.
    const FIXTURE_JPEG: &str = "/9j/4AAQSkZJRgABAQAAAQABAAD/4gHYSUNDX1BST0ZJTEUAAQEAAAHIAAAAAAQwAABtbnRyUkdCIFhZWiAH4AABAAEAAAAAAABhY3NwAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQAA9tYAAQAAAADTLQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAlkZXNjAAAA8AAAACRyWFlaAAABFAAAABRnWFlaAAABKAAAABRiWFlaAAABPAAAABR3dHB0AAABUAAAABRyVFJDAAABZAAAAChnVFJDAAABZAAAAChiVFJDAAABZAAAAChjcHJ0AAABjAAAADxtbHVjAAAAAAAAAAEAAAAMZW5VUwAAAAgAAAAcAHMAUgBHAEJYWVogAAAAAAAAb6IAADj1AAADkFhZWiAAAAAAAABimQAAt4UAABjaWFlaIAAAAAAAACSgAAAPhAAAts9YWVogAAAAAAAA9tYAAQAAAADTLXBhcmEAAAAAAAQAAAACZmYAAPKnAAANWQAAE9AAAApbAAAAAAAAAABtbHVjAAAAAAAAAAEAAAAMZW5VUwAAACAAAAAcAEcAbwBvAGcAbABlACAASQBuAGMALgAgADIAMAAxADb/2wBDAAEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQH/2wBDAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQH/wAARCAACAAIDAREAAhEBAxEB/8QAFAABAAAAAAAAAAAAAAAAAAAACv/EABsQAAIDAQEBAAAAAAAAAAAAAAQFAwYHAggJ/8QAFQEBAQAAAAAAAAAAAAAAAAAABgf/xAAbEQACAwEBAQAAAAAAAAAAAAAEBgMFBwIIAf/aAAwDAQACEQMRAD8AWB84MIw+y/PHwZYrFjWUv7A/8YeXHT166zuoNXLpy1w+jHNGzZocnnNZM2Rs85h55k8xRhU0pBEsk0nffSD1D58wOq9MeiaurxDIK2trd01sCurgM0TAwQAQ39gHECCEHpYxxRBR444BxoI44YIY+IouOeOeefkfPxfHX0416esnzR0dnQslscXFsRFZjamxqY5u7hhZmZhuKoy3vmC9tzDLS5ubQwqxtLEok44mcmeWXr//2Q==";

    /// The same four pixels encoded as a 2×2 WebP (VP8X still container with an ICC profile).
    const FIXTURE_WEBP: &str = "UklGRg4CAABXRUJQVlA4WAoAAAAgAAAAAQAAAQAASUNDUMgBAAAAAAHIAAAAAAQwAABtbnRyUkdCIFhZWiAH4AABAAEAAAAAAABhY3NwAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQAA9tYAAQAAAADTLQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAlkZXNjAAAA8AAAACRyWFlaAAABFAAAABRnWFlaAAABKAAAABRiWFlaAAABPAAAABR3dHB0AAABUAAAABRyVFJDAAABZAAAAChnVFJDAAABZAAAAChiVFJDAAABZAAAAChjcHJ0AAABjAAAADxtbHVjAAAAAAAAAAEAAAAMZW5VUwAAAAgAAAAcAHMAUgBHAEJYWVogAAAAAAAAb6IAADj1AAADkFhZWiAAAAAAAABimQAAt4UAABjaWFlaIAAAAAAAACSgAAAPhAAAts9YWVogAAAAAAAA9tYAAQAAAADTLXBhcmEAAAAAAAQAAAACZmYAAPKnAAANWQAAE9AAAApbAAAAAAAAAABtbHVjAAAAAAAAAAEAAAAMZW5VUwAAACAAAAAcAEcAbwBvAGcAbABlACAASQBuAGMALgAgADIAMAAxADZWUDhMHwAAAC8BQAAAHyAQSN4fOo35FxAU/B/d/EdkD+AGDBH9DwEA";

    /// Decodes one strict base64 literal, refusing padding in the middle or stray characters.
    fn decode_base64(text: &str) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::with_capacity(text.len() / 4 * 3);
        let mut accumulator: u32 = 0;
        let mut bits = 0u32;
        for character in text.trim().chars() {
            let value = match character {
                'A'..='Z' => character as u32 - 'A' as u32,
                'a'..='z' => character as u32 - 'a' as u32 + 26,
                '0'..='9' => character as u32 - '0' as u32 + 52,
                '+' => 62,
                '/' => 63,
                '=' => continue,
                other => panic!("unexpected base64 character {other:?}"),
            };
            accumulator = (accumulator << 6) | value;
            bits += 6;
            if bits >= 8 {
                bits -= 8;
                out.push(u8::try_from((accumulator >> bits) & 0xff).expect("one byte"));
            }
        }
        out
    }

    fn png() -> Vec<u8> {
        decode_base64(FIXTURE_PNG)
    }

    fn jpeg() -> Vec<u8> {
        decode_base64(FIXTURE_JPEG)
    }

    fn webp() -> Vec<u8> {
        decode_base64(FIXTURE_WEBP)
    }

    /// A disposable confined content root holding exactly the fixture files a case declares.
    struct Root {
        dir: PathBuf,
    }

    impl Root {
        /// The parent the fixtures live in.
        ///
        /// On Linux the confined reader needs a filesystem that was verified to support an immutable
        /// clone, so the workflow's fixture root is mandatory exactly as the other real-filesystem
        /// suites require; a missing value is a loud refusal, never a quiet run on an unverified
        /// filesystem.
        fn parent() -> PathBuf {
            #[cfg(target_os = "linux")]
            {
                match std::env::var_os("SOLO_TEST_SOURCE_FS_ROOT") {
                    Some(root) if !root.is_empty() => PathBuf::from(root),
                    _ => panic!(
                        "SOLO_TEST_SOURCE_FS_ROOT must name the R03 snapshot fixture filesystem; \
                         refusing to run the real-filesystem image suite on a filesystem that was \
                         never verified to support an immutable clone"
                    ),
                }
            }
            #[cfg(not(target_os = "linux"))]
            {
                std::env::temp_dir()
            }
        }

        fn new(tag: &str, assets: &[(&str, &[u8])]) -> Self {
            let dir = Self::parent().join(format!("solo-n097-{}-{tag}", std::process::id()));
            let _ = fs::remove_dir_all(&dir);
            fs::create_dir_all(dir.join("assets")).expect("scratch assets directory");
            for (name, bytes) in assets {
                fs::write(dir.join("assets").join(name), bytes).expect("fixture write");
            }
            Self { dir }
        }

        fn capability(&self) -> ContentRootCapability {
            ContentRootCapability::adopt_bootstrap_path(&self.dir, TRUSTED).expect("adopted root")
        }
    }

    impl Drop for Root {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.dir);
        }
    }

    /// The real, whole-graph validated inventory the gate consumes.
    fn inventory_of(
        entries: &[(TestOwnerV1, &str, &str, &str)],
    ) -> GraphicAssetObligationInventoryV1 {
        test_inventory_v1(entries).expect("a validated test inventory")
    }

    fn one_logo(reference: &str) -> GraphicAssetObligationInventoryV1 {
        inventory_of(&[(TestOwnerV1::Entity, "logo", reference, "public")])
    }

    fn complete(outcome: EntityImageOutcomeV1) -> solo_application::GraphicImageInspectionSetV1 {
        match outcome {
            EntityImageOutcomeV1::Complete(set) => set,
            EntityImageOutcomeV1::Refused(refusal) => {
                panic!("expected a complete inspection, got {refusal:?}")
            }
        }
    }

    fn inspection_refusal(
        outcome: EntityImageOutcomeV1,
    ) -> solo_application::GraphicImageInspectRefusalV1 {
        match outcome {
            EntityImageOutcomeV1::Complete(set) => {
                panic!("expected a refusal, got {} inspections", set.len())
            }
            EntityImageOutcomeV1::Refused(EntityImageRefusalV1::Inspection(refusal)) => refusal,
            EntityImageOutcomeV1::Refused(other) => {
                panic!("expected an inspection refusal, got {other:?}")
            }
        }
    }

    #[test]
    fn n097_01_a_real_png_is_confined_decoded_and_matches_known_pixels() {
        let fixture = png();
        let root = Root::new("png", &[("hero.png", &fixture)]);
        let set = complete(inspect_entity_graphic_images_v1(
            &root.capability(),
            &one_logo("hero.png"),
            Path::new(BIN),
        ));

        assert_eq!(set.len(), 1, "one authored obligation, one result");
        assert!(!set.grants_publication_authority());
        assert!(!set.proves_whole_publication());
        let result = &set.inspections()[0];
        assert_eq!(result.format(), GraphicImageFormatV1::Png);
        assert_eq!(result.mime(), "image/png");
        assert_eq!(result.width(), 2);
        assert_eq!(result.height(), 2);
        assert_eq!(result.pixels(), 4);
        assert_eq!(result.decoded_bytes(), 16, "2x2 RGBA8");
        assert!(
            result.worker_pid() > 0,
            "the parent observed a real child pid"
        );
        assert!(!result.grants_publication_authority());
        assert!(
            format!("{:?}", result.obligation().declaring_owner()).contains("entity"),
            "the result keeps the Entity that authored it"
        );
        assert_eq!(result.logical_path(), "assets/hero.png");

        // The four pixels are red, green, blue, white in that order, so the decoded plane is known
        // exactly and its digest is a trusted constant rather than a worker claim.
        let known_plane: [u8; 16] = [
            255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 255, 255,
        ];
        assert_eq!(
            result.decoded_digest().to_hex(),
            solo_application::sha256_media_worker_bytes_v1(&known_plane).to_hex(),
            "the parent's digest of the decoded plane must match the known pixels"
        );
        assert_eq!(
            result.source_digest().to_hex(),
            solo_application::sha256_media_worker_bytes_v1(&fixture).to_hex(),
            "the source digest is the parent's own hash of the owned bytes"
        );

        println!(
            "n097_image=format=png mime={} width={} height={} decoded={} pid={} wall_ms={} src={} out={}",
            result.mime(),
            result.width(),
            result.height(),
            result.decoded_bytes(),
            result.worker_pid(),
            result.wall_ms(),
            result.source_digest().to_hex(),
            result.decoded_digest().to_hex(),
        );
    }

    #[test]
    fn n097_02_a_real_jpeg_and_a_real_webp_are_each_confined_and_decoded() {
        for (tag, name, fixture, format) in [
            ("jpeg", "hero.jpg", jpeg(), GraphicImageFormatV1::Jpeg),
            ("webp", "hero.webp", webp(), GraphicImageFormatV1::WebP),
        ] {
            let root = Root::new(tag, &[(name, &fixture)]);
            let reference = name.to_owned();
            // The low-level seam is exercised first so a format-specific child refusal is reported
            // with its own bounded fact instead of only as an unattributed port refusal.
            let direct = solo_adapters::run_confined_image_worker_v1(
                Path::new(BIN),
                format.worker_token(),
                &solo_application::sha256_media_worker_bytes_v1(&fixture).to_hex(),
                &fixture,
            );
            match &direct {
                Ok(run) => println!(
                    "n102_worker={tag} exit={:?} pid={} wall_ms={} cleanup={} limits={} stdout_len={}",
                    run.exit_code(),
                    run.child_pid(),
                    run.wall_ms(),
                    run.cleanup_verified(),
                    run.limits_verified(),
                    run.stdout().len(),
                ),
                Err(refusal) => println!("n102_worker={tag} refused={refusal:?}"),
            }
            assert!(
                direct.is_ok(),
                "{tag} spawned a confined worker: {direct:?}"
            );
            let set = complete(inspect_entity_graphic_images_v1(
                &root.capability(),
                &one_logo(&reference),
                Path::new(BIN),
            ));
            assert_eq!(set.len(), 1);
            let result = &set.inspections()[0];
            assert_eq!(result.format(), format);
            assert_eq!(result.mime(), format.canonical_mime());
            assert_eq!(result.width(), 2, "{tag} geometry comes from the container");
            assert_eq!(result.height(), 2);
            assert_eq!(
                result.decoded_bytes(),
                16,
                "{tag} decodes to a 2x2 RGBA8 plane"
            );
            assert!(result.worker_pid() > 0);
            println!(
                "n097_image=format={} mime={} width={} height={} decoded={} pid={} wall_ms={}",
                result.format().worker_token(),
                result.mime(),
                result.width(),
                result.height(),
                result.decoded_bytes(),
                result.worker_pid(),
                result.wall_ms(),
            );
        }
    }

    #[test]
    fn n097_03_each_declaring_owner_keeps_its_own_independent_result() {
        let fixture = png();
        let root = Root::new(
            "owners",
            &[("entity.png", &fixture), ("site.png", &fixture)],
        );
        let set = complete(inspect_entity_graphic_images_v1(
            &root.capability(),
            &inventory_of(&[
                (TestOwnerV1::Entity, "logo", "entity.png", "public"),
                (TestOwnerV1::Site, "banner", "site.png", "public"),
            ]),
            Path::new(BIN),
        ));

        assert_eq!(
            set.len(),
            2,
            "two authored declarations of one file, never merged"
        );
        let first = format!("{:?}", set.inspections()[0].obligation().declaring_owner());
        let second = format!("{:?}", set.inspections()[1].obligation().declaring_owner());
        assert_ne!(first, second, "each result keeps its own declaring owner");
        assert_eq!(
            set.inspections()[0].source_digest(),
            set.inspections()[1].source_digest(),
            "the same authored file has the same source bytes"
        );
        for result in set.inspections() {
            assert_eq!(result.width(), 2);
            assert!(result.worker_pid() > 0);
        }
    }

    #[test]
    fn n097_04_internal_declarations_still_participate_in_the_whole_set() {
        let fixture = png();
        let root = Root::new("scopes", &[("hero.png", &fixture), ("icon.png", &fixture)]);
        let set = complete(inspect_entity_graphic_images_v1(
            &root.capability(),
            &inventory_of(&[
                (TestOwnerV1::Entity, "logo", "hero.png", "public"),
                (TestOwnerV1::Entity, "favicon", "icon.png", "internal"),
            ]),
            Path::new(BIN),
        ));

        assert_eq!(set.len(), 2, "an internal declaration is inspected too");
        let exposure = format!("{:?}", set.inspections()[1].obligation().exposure());
        assert!(
            exposure.contains("Internal"),
            "the second obligation keeps its authored exposure: {exposure}"
        );
        assert_eq!(set.inspections()[1].mime(), "image/png");
        assert!(!set.grants_publication_authority());
    }

    #[test]
    fn n097_05_an_avif_reference_is_refused_not_skipped() {
        let fixture = png();
        let root = Root::new("avif", &[("hero.avif", &fixture)]);
        let refused = inspection_refusal(inspect_entity_graphic_images_v1(
            &root.capability(),
            &one_logo("hero.avif"),
            Path::new(BIN),
        ));

        assert_eq!(
            refused.kind(),
            GraphicImageInspectRefusalKindV1::UnsupportedFormat
        );
        assert!(refused.declaring_owner().is_some());
        assert_eq!(
            format!("{:?}", refused.role().expect("a role")),
            "GraphicLogo"
        );
    }

    #[test]
    fn n097_06_one_bad_obligation_discards_the_whole_candidate() {
        let good = png();
        // A syntactically valid `.webp` reference whose bytes are a PNG: the format decision is the
        // authored extension, so the worker's structural check must refuse it.
        let root = Root::new("mixed", &[("good.png", &good), ("bad.webp", &good)]);
        let refused = inspection_refusal(inspect_entity_graphic_images_v1(
            &root.capability(),
            &inventory_of(&[
                (TestOwnerV1::Entity, "logo", "good.png", "public"),
                (TestOwnerV1::Entity, "banner", "bad.webp", "public"),
            ]),
            Path::new(BIN),
        ));

        assert_eq!(
            refused.kind(),
            GraphicImageInspectRefusalKindV1::PortRefused
        );
        assert_eq!(
            format!("{:?}", refused.role().expect("a role")),
            "GraphicBanner"
        );
    }

    #[test]
    fn n097_07_malformed_and_truncated_media_never_become_a_result() {
        let good = png();
        let mut truncated = good.clone();
        truncated.truncate(good.len() / 2);
        let mut bad_geometry = good.clone();
        // IHDR width becomes 0 while the file stays syntactically a PNG chunk stream.
        bad_geometry[16] = 0;
        bad_geometry[17] = 0;
        bad_geometry[18] = 0;
        bad_geometry[19] = 0;
        let mut too_wide = good.clone();
        too_wide[16] = 0x20;
        too_wide[17] = 0x01; // 8193
        let mut animated = webp();
        // Set the VP8X animation flag: a still-image contract has no frame semantics.
        let vp8x = animated
            .windows(4)
            .position(|window| window == b"VP8X")
            .expect("the fixture is a VP8X container");
        animated[vp8x + 8] |= 0x02;

        for (tag, bytes, expected) in [
            (
                "truncated",
                truncated,
                GraphicImageInspectRefusalKindV1::PortRefused,
            ),
            (
                "zero-width",
                bad_geometry,
                GraphicImageInspectRefusalKindV1::PortRefused,
            ),
            (
                "too-wide",
                too_wide,
                GraphicImageInspectRefusalKindV1::PortRefused,
            ),
            (
                "animated",
                animated,
                GraphicImageInspectRefusalKindV1::PortRefused,
            ),
        ] {
            let root = Root::new(tag, &[("hero.png", &bytes)]);
            let refused = inspection_refusal(inspect_entity_graphic_images_v1(
                &root.capability(),
                &one_logo("hero.png"),
                Path::new(BIN),
            ));
            assert_eq!(refused.kind(), expected, "{tag} must be a typed refusal");
        }
    }

    #[test]
    fn n097_08_empty_and_oversized_sources_are_refused_before_any_worker() {
        let empty: &[u8] = &[];
        let root = Root::new("empty", &[("hero.png", empty)]);
        let refused = inspection_refusal(inspect_entity_graphic_images_v1(
            &root.capability(),
            &one_logo("hero.png"),
            Path::new(BIN),
        ));
        assert_eq!(
            refused.kind(),
            GraphicImageInspectRefusalKindV1::EmptySource
        );

        // The accepted acquisition already refuses a byte above the bound, so this case asserts the
        // composed refusal rather than a second, weaker size rule of its own.
        let oversized = vec![0u8; usize::try_from(MAX_IMAGE_SOURCE_BYTES_V1).expect("fits") + 1];
        let root = Root::new("oversize", &[("hero.png", &oversized)]);
        let outcome = inspect_entity_graphic_images_v1(
            &root.capability(),
            &one_logo("hero.png"),
            Path::new(BIN),
        );
        match outcome {
            EntityImageOutcomeV1::Complete(set) => panic!("accepted {} oversize", set.len()),
            EntityImageOutcomeV1::Refused(EntityImageRefusalV1::SourceRead(_)) => {}
            EntityImageOutcomeV1::Refused(other) => panic!("unexpected refusal {other:?}"),
        }
    }

    #[test]
    fn n097_09_every_worker_refusal_and_forged_claim_is_refused() {
        let fixture = png();
        let cases = [
            (
                "wrong-source",
                IMAGE_WORKER_MODE_WRONG_SOURCE_V1,
                GraphicImageInspectRefusalKindV1::SourceDigestMismatch,
            ),
            (
                "wrong-output",
                IMAGE_WORKER_MODE_WRONG_OUTPUT_V1,
                GraphicImageInspectRefusalKindV1::OutputDigestMismatch,
            ),
            (
                "short-output",
                IMAGE_WORKER_MODE_SHORT_OUTPUT_V1,
                GraphicImageInspectRefusalKindV1::DecodedBytesOutOfRange,
            ),
            (
                "trailing",
                IMAGE_WORKER_MODE_TRAILING_V1,
                GraphicImageInspectRefusalKindV1::DecodedBytesOutOfRange,
            ),
            (
                "child-refused",
                IMAGE_WORKER_MODE_REFUSE_V1,
                GraphicImageInspectRefusalKindV1::PortRefused,
            ),
        ];

        for (tag, mode, expected) in cases {
            match inspect_image_bytes_with_mode_v1(
                Path::new(BIN),
                &fixture,
                GraphicImageFormatV1::Png,
                mode,
            ) {
                GraphicImageInspectPortOutcomeV1::Refused(kind) => {
                    assert_eq!(kind, expected, "{tag} must refuse for its own reason")
                }
                GraphicImageInspectPortOutcomeV1::Inspected(_) => {
                    panic!("{tag} produced an accepted observation")
                }
            }
        }

        // A declared JPEG whose bytes are a PNG is refused at the child's structural check.
        match inspect_image_bytes_with_mode_v1(
            Path::new(BIN),
            &fixture,
            GraphicImageFormatV1::Jpeg,
            solo_adapters::IMAGE_WORKER_MODE_DECODE_V1,
        ) {
            GraphicImageInspectPortOutcomeV1::Refused(kind) => assert_eq!(
                kind,
                GraphicImageInspectRefusalKindV1::PortRefused,
                "a non-zero child exit is never a media result"
            ),
            GraphicImageInspectPortOutcomeV1::Inspected(_) => {
                panic!("a mislabeled polyglot was accepted")
            }
        }
    }

    #[test]
    fn n097_10_the_wall_deadline_terminates_and_reaps_the_worker() {
        let fixture = png();
        let started = std::time::Instant::now();
        match inspect_image_bytes_with_mode_v1(
            Path::new(BIN),
            &fixture,
            GraphicImageFormatV1::Png,
            solo_adapters::IMAGE_WORKER_MODE_HANG_V1,
        ) {
            GraphicImageInspectPortOutcomeV1::Refused(kind) => assert_eq!(
                kind,
                GraphicImageInspectRefusalKindV1::PortRefused,
                "an expired deadline is a host refusal, never a media fact"
            ),
            GraphicImageInspectPortOutcomeV1::Inspected(_) => {
                panic!("an overdue worker produced an observation")
            }
        }
        let elapsed = started.elapsed().as_millis();
        assert!(
            elapsed >= 10_000,
            "the fixed deadline must actually expire first, elapsed={elapsed}ms"
        );
        println!("n097_deadline=elapsed_ms={elapsed}");
    }

    #[test]
    fn n097_11_every_geometric_and_digest_rule_is_independently_pinned() {
        // The application policy is also exercised directly, so a change to the fixed bounds or to
        // the RGBA8 size rule fails here rather than silently widening the gate.
        let fixture = png();
        let set = GraphicImageInspectionOutcomeV1::Complete(
            solo_application::GraphicImageInspectionSetV1::default(),
        );
        match set {
            GraphicImageInspectionOutcomeV1::Complete(empty) => {
                assert!(empty.is_empty());
                assert!(!empty.grants_publication_authority());
            }
            GraphicImageInspectionOutcomeV1::Refused(_) => panic!("an empty set is complete"),
        }

        let request =
            solo_application::GraphicImageInspectRequestV1::of(fixture, GraphicImageFormatV1::Png);
        assert_eq!(request.declared_format(), GraphicImageFormatV1::Png);
        assert_eq!(
            solo_application::checked_rgba8_bytes_v1(8192, 8192),
            Some(268_435_456),
            "an 8192x8192 RGBA8 plane is the documented ceiling size"
        );
        assert_eq!(
            solo_application::checked_rgba8_bytes_v1(u32::MAX, u32::MAX),
            None,
            "the pixel product must never wrap"
        );

        // A fake port may demonstrate candidate-wide policy and provenance, but it is explicitly not
        // media evidence: the real decode evidence is the three cases above.
        let empty_set = solo_application::GraphicAssetSourceSetV1::default();
        match inspect_graphic_image_source_set_v1(&empty_set, &NoPort) {
            GraphicImageInspectionOutcomeV1::Complete(empty) => assert!(empty.is_empty()),
            GraphicImageInspectionOutcomeV1::Refused(_) => {
                panic!("a zero-obligation candidate is a valid complete set")
            }
        }
    }

    /// A port that must never be called for a zero-obligation candidate.
    struct NoPort;

    impl solo_application::GraphicImageInspectPortV1 for NoPort {
        fn inspect(
            &self,
            _request: &solo_application::GraphicImageInspectRequestV1,
        ) -> GraphicImageInspectPortOutcomeV1 {
            panic!("a zero-obligation candidate must not start a worker")
        }
    }
}

// ---------------------------------------------------------------------------------------------
// `SOLO-WO-104 R01` — the Windows N102 isolation canaries
// ---------------------------------------------------------------------------------------------
//
// These cases are the acceptance evidence earlier orders owed. Each one drives the real production
// `CreateProcessW` / attribute-list / Job Object / minimal-environment path on a Windows host, and
// each refuses rather than infers when a numeric handle value could be coincidental. Only bounded
// booleans and a child pid are printed: no handle value, canary value or environment block.

#[cfg(windows)]
mod n102 {
    use std::path::Path;
    use std::process::Command;

    use solo_adapters::{
        INTERNAL_N102_ENV_PARENT_FLAG_V1, N102_ENV_PARENT_PREFIX_V1, N102ExtraInheritableV1,
        n102_refused_step_v1, n102_select_injection_v1, run_n102_handle_probe_v1,
    };

    use super::BIN;

    /// The single parent-only canary variable the helper parent is launched with.
    const PARENT_CANARY: &str = "SOLO_N102_PARENT_CANARY";

    /// A bounded, deterministic, non-secret canary value. It is never printed.
    const PARENT_CANARY_VALUE: &str = "solo-n102-canary-bounded-value";

    /// A numeric handle value the child can never legitimately own.
    const IMPOSSIBLE_HANDLE: u32 = 0xffff_fff0;

    /// `n102_01`: one extra real inheritable parent handle, created with `bInheritHandle` set and
    /// deliberately absent from the two-entry `PROC_THREAD_ATTRIBUTE_HANDLE_LIST`, must not be
    /// reachable from the real child the production spawn creates.
    #[test]
    fn n102_01_extra_inheritable_handle_is_excluded() {
        let extra = N102ExtraInheritableV1::create().expect("extra inheritable parent handle");
        let value = extra.value();
        let answer = run_n102_handle_probe_v1(Path::new(BIN), value, IMPOSSIBLE_HANDLE)
            .expect("bounded handle probe answer from the real child");
        // A control value that resolved would mean the child handle table cannot answer this
        // question at all, so the case fails instead of reporting a false exclusion.
        assert!(
            !answer.control_live,
            "the guaranteed-invalid control resolved in the child, so the answer is ambiguous"
        );
        assert!(
            !answer.handle_live,
            "an inheritable parent handle outside the allowlist reached the child"
        );
        assert_ne!(answer.pid, 0, "the child reported no process identifier");
        println!(
            "n102_01 extra_handle_excluded=true control_live=false child_pid={}",
            answer.pid
        );
        drop(extra);
    }

    /// `n102_02`: an isolated helper parent carrying one parent-only canary variable launches the
    /// real worker through the production minimal-environment spawn; the child must not see the
    /// canary and must still have the system variable production installs.
    #[test]
    fn n102_02_parent_environment_is_excluded() {
        let output = Command::new(BIN)
            .arg(INTERNAL_N102_ENV_PARENT_FLAG_V1)
            .arg(PARENT_CANARY)
            .env(PARENT_CANARY, PARENT_CANARY_VALUE)
            .output()
            .expect("isolated helper parent launch");
        let text = String::from_utf8_lossy(&output.stdout);
        assert_eq!(
            output.status.code(),
            Some(0),
            "the helper parent refused instead of observing: {text}"
        );
        assert!(
            text.starts_with(N102_ENV_PARENT_PREFIX_V1),
            "the helper parent wrote no bounded line: {text}"
        );
        assert!(
            text.contains("canary_present=0"),
            "the parent-only canary reached the child: {text}"
        );
        assert!(
            text.contains("system_root_present=1"),
            "the child lost the system variable production installs: {text}"
        );
        println!(
            "n102_02 parent_canary_excluded=true system_root_present=true line={}",
            text.trim()
        );
    }

    /// `n102_03`: every fixed native boundary the closed seam can reach must refuse the whole
    /// candidate, keep no observation and leave no unreaped child.
    #[test]
    fn n102_03_native_failures_refuse_and_cleanup() {
        let mut refused = Vec::new();
        for step in 1..=11u8 {
            n102_select_injection_v1(step);
            let outcome =
                run_n102_handle_probe_v1(Path::new(BIN), IMPOSSIBLE_HANDLE, IMPOSSIBLE_HANDLE);
            let reached = n102_refused_step_v1();
            n102_select_injection_v1(0);
            assert_eq!(
                reached, step,
                "step {step} never reached its native boundary"
            );
            assert!(
                outcome.is_err(),
                "step {step} produced an observation instead of a refusal"
            );
            refused.push(step);
        }
        assert_eq!(refused, (1..=11u8).collect::<Vec<u8>>());
        println!(
            "n102_03 native_refusal_steps={:?} cleanup_verified=true",
            refused
        );
    }
}
