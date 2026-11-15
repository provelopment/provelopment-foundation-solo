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
/// `SOLO-WO-113 R02` — shared deterministic GIF fixtures for the real four-format host cases.
///
/// These are test-only byte builders, copied from the accepted `n108` fixtures so `mod n097` can
/// build the same canonical specimens. They are inputs to the **real** confined worker; they are
/// never a decoding path, they grant no authority and they change neither `n108` nor `n097`'s
/// existing cases.
mod n113_fixtures {
    /// A 1×1 GIF87a whose only pixel is palette index 0 of the global red/blue table.
    ///
    /// Code width is three bits and the codes are (clear 4, index 0, end 5), packed little-endian
    /// as `0x0144`.
    pub fn gif87a_red_pixel() -> Vec<u8> {
        let mut bytes = b"GIF87a".to_vec();
        bytes.extend_from_slice(&[1, 0, 1, 0, 0x80, 0, 0]);
        bytes.extend_from_slice(&[255, 0, 0, 0, 0, 255]);
        bytes.extend_from_slice(&[0x2c, 0, 0, 0, 0, 1, 0, 1, 0, 0]);
        bytes.extend_from_slice(&[2, 2, 0x44, 0x01, 0, 0x3b]);
        bytes
    }

    /// The same 1×1 canvas whose only pixel is palette index 1, the blue entry.
    ///
    /// The codes are (clear 4, index 1, end 5), packed little-endian as `0x014c`.
    pub fn gif87a_blue_pixel() -> Vec<u8> {
        let mut bytes = b"GIF87a".to_vec();
        bytes.extend_from_slice(&[1, 0, 1, 0, 0x80, 0, 0]);
        bytes.extend_from_slice(&[255, 0, 0, 0, 0, 255]);
        bytes.extend_from_slice(&[0x2c, 0, 0, 0, 0, 1, 0, 1, 0, 0]);
        bytes.extend_from_slice(&[2, 2, 0x4c, 0x01, 0, 0x3b]);
        bytes
    }

    /// Deterministic three-frame 1×1 GIF89a: NETSCAPE2.0 infinite loop and three Graphic Control
    /// Extensions (10 cs keep / 20 cs background+transparent / 5 cs keep+transparent) sharing the
    /// global red/blue palette.
    pub fn gif89a_three_frames() -> Vec<u8> {
        let mut bytes = b"GIF89a".to_vec();
        bytes.extend_from_slice(&[1, 0, 1, 0, 0x80, 0, 0]);
        bytes.extend_from_slice(&[255, 0, 0, 0, 0, 255]);
        bytes.extend_from_slice(&[0x21, 0xff, 11]);
        bytes.extend_from_slice(b"NETSCAPE2.0");
        bytes.extend_from_slice(&[3, 1, 0, 0, 0]);
        for (delay, disposal, transparent) in [(10u8, 1u8, false), (20, 2, true), (5, 1, true)] {
            let gce = (disposal << 2) | u8::from(transparent);
            bytes.extend_from_slice(&[0x21, 0xf9, 4, gce, delay, 0, 0, 0]);
            bytes.extend_from_slice(&[0x2c, 0, 0, 0, 0, 1, 0, 1, 0, 0]);
            bytes.extend_from_slice(&[2, 2, 0x44, 0x01, 0]);
        }
        bytes.push(0x3b);
        bytes
    }
}

mod n097 {
    use std::fs;
    use std::path::{Path, PathBuf};

    use solo_adapters::{
        ConfinedGraphicInspectAdapterV2, ContentRootCapability, EntityImageOutcomeV1,
        EntityImageOutcomeV2, EntityImageRefusalV1, EntityImageRefusalV2, GifWorkerRefusalV1,
        GifWorkerTestModeV1, IMAGE_WORKER_MODE_REFUSE_V1, IMAGE_WORKER_MODE_SHORT_OUTPUT_V1,
        IMAGE_WORKER_MODE_TRAILING_V1, IMAGE_WORKER_MODE_WRONG_OUTPUT_V1,
        IMAGE_WORKER_MODE_WRONG_SOURCE_V1, RootProvenanceV1, TestOwnerV1,
        inspect_entity_graphic_images_v1, inspect_entity_graphic_images_v2,
        inspect_gif_bytes_with_executable_v1, inspect_gif_bytes_with_test_mode_v1,
        inspect_image_bytes_with_mode_v1, test_inventory_suppressed_v1, test_inventory_v1,
    };
    use solo_application::{
        GifDisposalV2, GifLoopV2, GraphicAssetObligationInventoryV1, GraphicAssetReadSetOutcomeV1,
        GraphicImageFormatV1, GraphicImageFormatV2, GraphicImageInspectPortOutcomeV1,
        GraphicImageInspectRefusalKindV1, GraphicImageInspectRefusalKindV2,
        GraphicImageInspectionOutcomeV1, GraphicImageInspectionOutcomeV2,
        MAX_IMAGE_SOURCE_BYTES_V1, inspect_graphic_image_source_set_v1,
        inspect_graphic_image_source_set_v2, read_graphic_asset_bytes_v1,
        sha256_media_worker_bytes_v1,
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

    // ---- `SOLO-WO-113 R02`: the real four-format V2 composition ----

    fn complete_v2(outcome: EntityImageOutcomeV2) -> solo_application::GraphicImageInspectionSetV2 {
        match outcome {
            EntityImageOutcomeV2::Complete(set) => set,
            EntityImageOutcomeV2::Refused(refusal) => {
                panic!("expected a complete V2 inspection, got {refusal:?}")
            }
        }
    }

    fn inspection_refusal_v2(outcome: EntityImageOutcomeV2) -> EntityImageRefusalV2 {
        match outcome {
            EntityImageOutcomeV2::Complete(set) => {
                panic!("expected a V2 refusal, got {} inspections", set.len())
            }
            EntityImageOutcomeV2::Refused(refusal) => refusal,
        }
    }

    #[test]
    fn n113_01_mixed_four_formats_real_host() {
        let png_bytes = png();
        let jpeg_bytes = jpeg();
        let webp_bytes = webp();
        let gif_bytes = super::n113_fixtures::gif89a_three_frames();
        let root = Root::new(
            "n113-01",
            &[
                ("favicon.png", &png_bytes),
                ("logo.jpg", &jpeg_bytes),
                ("hero.webp", &webp_bytes),
                ("scene.gif", &gif_bytes),
            ],
        );
        // Four distinct real EFS declaring owners, each with its own registered role, so no singular
        // inherited cardinality is violated and every level contributes an independent obligation.
        let inventory = inventory_of(&[
            (TestOwnerV1::Entity, "favicon", "favicon.png", "public"),
            (TestOwnerV1::Facet, "logo", "logo.jpg", "public"),
            (TestOwnerV1::Site, "banner", "hero.webp", "internal"),
            (TestOwnerV1::Page, "background", "scene.gif", "public"),
        ]);
        assert_eq!(inventory.len(), 4);

        let set = complete_v2(inspect_entity_graphic_images_v2(
            &root.capability(),
            &inventory,
            Path::new(BIN),
        ));
        assert_eq!(set.len(), 4);
        assert!(!set.grants_publication_authority());
        assert!(!set.proves_whole_publication());

        let expected = [
            (
                GraphicImageFormatV2::Png,
                "image/png",
                2u32,
                2u32,
                &png_bytes,
            ),
            (GraphicImageFormatV2::Jpeg, "image/jpeg", 2, 2, &jpeg_bytes),
            (GraphicImageFormatV2::WebP, "image/webp", 2, 2, &webp_bytes),
            (GraphicImageFormatV2::Gif, "image/gif", 1, 1, &gif_bytes),
        ];
        let references = ["favicon.png", "logo.jpg", "hero.webp", "scene.gif"];
        let mut owners = Vec::new();
        let mut pids = Vec::new();
        for (position, inspection) in set.inspections().iter().enumerate() {
            let (format, mime, width, height, bytes) = expected[position];
            assert_eq!(inspection.format(), format, "inventory order is preserved");
            assert_eq!(inspection.mime(), mime);
            assert_eq!((inspection.width(), inspection.height()), (width, height));
            assert_eq!(inspection.pixels(), u64::from(width) * u64::from(height));
            assert_eq!(
                inspection.source_digest(),
                &sha256_media_worker_bytes_v1(bytes)
            );
            assert_eq!(
                inspection.obligation().reference_text(),
                references[position]
            );
            assert_eq!(inspection.obligation().local_index(), 0);
            assert!(inspection.logical_path().starts_with("assets/"));
            assert!(inspection.logical_path().ends_with(references[position]));
            assert_ne!(inspection.worker_pid(), 0);
            assert!(!inspection.grants_publication_authority());
            owners.push(inspection.obligation().declaring_owner().clone());
            pids.push(inspection.worker_pid());
            match inspection.still() {
                Some(still) => {
                    assert!(!inspection.is_animation());
                    assert_eq!(
                        still.decoded_bytes,
                        u64::from(width) * u64::from(height) * 4
                    );
                    assert_ne!(still.decoded_digest, sha256_media_worker_bytes_v1(b""));
                }
                None => assert!(inspection.is_animation(), "the GIF row is the timeline"),
            }
        }
        for (left, right) in [(0usize, 1usize), (2, 3)] {
            assert_ne!(
                owners[left], owners[right],
                "each result keeps its own declaring owner"
            );
        }
        assert_eq!(
            owners.len(),
            4,
            "four distinct declaring owners must be retained"
        );
        assert_ne!(
            pids[0], pids[3],
            "the PNG and GIF rows are separate children"
        );

        let gif_row = &set.inspections()[3];
        let gif = gif_row.animation().expect("the GIF row is a timeline");
        assert_eq!(gif.signature, "GIF89a");
        assert_eq!(gif.mime, "image/gif");
        assert_eq!((gif.width, gif.height), (1, 1));
        assert_eq!(gif.frame_count(), 3);
        assert!(gif.is_animated());
        assert_eq!(gif.loop_semantics, GifLoopV2::Infinite);
        assert_eq!(gif.one_cycle_delay_ms, 350);
        assert_eq!(gif.total_composited_pixels, 3);
        assert_eq!(
            gif.frames
                .iter()
                .map(|frame| frame.index)
                .collect::<Vec<_>>(),
            [0, 1, 2]
        );
        assert_eq!(
            gif.frames
                .iter()
                .map(|frame| frame.delay_centiseconds)
                .collect::<Vec<_>>(),
            [10, 20, 5]
        );
        assert_eq!(
            gif.frames
                .iter()
                .map(|frame| frame.disposal)
                .collect::<Vec<_>>(),
            [
                GifDisposalV2::Keep,
                GifDisposalV2::Background,
                GifDisposalV2::Keep
            ]
        );
        let red = sha256_media_worker_bytes_v1(&[255, 0, 0, 255]);
        let clear = sha256_media_worker_bytes_v1(&[0, 0, 0, 0]);
        assert_eq!(gif.frames[0].composite_digest, red);
        assert_eq!(gif.frames[1].composite_digest, red);
        assert_eq!(gif.frames[2].composite_digest, clear);
        assert_eq!(gif.source_digest, sha256_media_worker_bytes_v1(&gif_bytes));
        assert!(gif.transcript_len > 0);
        assert_ne!(gif.transcript_digest, gif.source_digest);
        assert!(gif.limits_verified && gif.cleanup_verified);
        assert_eq!(gif.worker_pid, pids[3]);

        println!(
            "n113_mixed formats=4 owners=4 order=png,jpeg,webp,gif pids={pids:?} gif_frames=3 \
             gif_loop=infinite gif_cycle_ms=350 gif_src={} gif_transcript={} red={} clear={}",
            gif.source_digest.to_hex(),
            gif.transcript_digest.to_hex(),
            red.to_hex(),
            clear.to_hex()
        );
    }

    #[test]
    fn n113_02_hidden_obligation_refuses_whole_candidate() {
        let png_bytes = png();
        let jpeg_bytes = jpeg();
        let webp_bytes = webp();
        let gif_bytes = super::n113_fixtures::gif89a_three_frames();
        let mut corrupt_gif = gif_bytes.clone();
        corrupt_gif.truncate(corrupt_gif.len() - 3);

        // (1) A representation-owned declaration is a full obligation of its own, alongside an
        // `internal` exposure that the graph keeps for verification but never publishes.
        let root = Root::new(
            "n113-02a",
            &[
                ("favicon.png", &png_bytes),
                ("logo.jpg", &jpeg_bytes),
                ("hero.webp", &webp_bytes),
                ("scene.gif", &gif_bytes),
            ],
        );
        let inventory = inventory_of(&[
            (TestOwnerV1::Entity, "favicon", "favicon.png", "public"),
            (TestOwnerV1::Facet, "logo", "logo.jpg", "public"),
            (TestOwnerV1::Site, "banner", "hero.webp", "internal"),
            (
                TestOwnerV1::Representation,
                "social-preview",
                "scene.gif",
                "public",
            ),
        ]);
        let set = complete_v2(inspect_entity_graphic_images_v2(
            &root.capability(),
            &inventory,
            Path::new(BIN),
        ));
        assert_eq!(
            set.len(),
            4,
            "every declared owner is verified independently"
        );
        let owners: Vec<_> = set
            .inspections()
            .iter()
            .map(|inspection| inspection.obligation().declaring_owner().clone())
            .collect();
        assert_ne!(owners[2], owners[3]);
        assert!(set.inspections()[3].is_animation());
        println!(
            "n113_hidden representation_included=true internal_included=true owners=4 \
             gif_owner_distinct=true"
        );

        // (2) An invalid `internal` obligation in the middle aborts the whole candidate: the two
        // earlier successes are discarded and the caller receives one refusal only.
        let root = Root::new(
            "n113-02b",
            &[
                ("favicon.png", &png_bytes),
                ("logo.jpg", &jpeg_bytes),
                ("broken.gif", &corrupt_gif),
                ("scene.gif", &gif_bytes),
            ],
        );
        let inventory = inventory_of(&[
            (TestOwnerV1::Entity, "favicon", "favicon.png", "public"),
            (TestOwnerV1::Facet, "logo", "logo.jpg", "public"),
            (TestOwnerV1::Site, "banner", "broken.gif", "internal"),
            (TestOwnerV1::Page, "background", "scene.gif", "public"),
        ]);
        match inspection_refusal_v2(inspect_entity_graphic_images_v2(
            &root.capability(),
            &inventory,
            Path::new(BIN),
        )) {
            EntityImageRefusalV2::Inspection(refusal) => {
                assert_eq!(
                    refusal.kind(),
                    GraphicImageInspectRefusalKindV2::PortRefused
                );
                assert!(refusal.declaring_owner().is_some());
                assert!(refusal.role().is_some());
                assert_eq!(refusal.local_index(), Some(0));
            }
            other => panic!("expected an inspection refusal, got {other:?}"),
        }
        println!("n113_hidden refused_whole=true discarded=2 partial=false");

        // (3) A suppressing descendant still contributes its own authored obligation.
        let root = Root::new("n113-02c", &[("scene.gif", &gif_bytes)]);
        let suppressed = test_inventory_suppressed_v1(&[(
            TestOwnerV1::Page,
            "background",
            "scene.gif",
            "public",
        )])
        .expect("a suppressing declaration is valid authored input");
        assert_eq!(suppressed.len(), 1);
        let set = complete_v2(inspect_entity_graphic_images_v2(
            &root.capability(),
            &suppressed,
            Path::new(BIN),
        ));
        assert_eq!(set.len(), 1);
        assert!(set.inspections()[0].is_animation());
        println!("n113_hidden suppressed_declaration_inspected=true");
    }

    #[test]
    fn n113_03_declared_format_and_favicon_refusals() {
        let png_bytes = png();
        let gif_bytes = super::n113_fixtures::gif89a_three_frames();
        let mut corrupt_gif = gif_bytes.clone();
        corrupt_gif.truncate(corrupt_gif.len() - 6);

        // A PNG claimed as a GIF, and a GIF claimed as a PNG: both are refused by the real worker
        // behind the V2 adapter, and neither yields a candidate.
        for (tag, name, bytes) in [
            ("png-as-gif", "scene.gif", &png_bytes),
            ("gif-as-png", "hero.png", &gif_bytes),
            ("corrupt-gif", "broken.gif", &corrupt_gif),
        ] {
            let root = Root::new(&format!("n113-03-{tag}"), &[(name, bytes)]);
            let inventory = one_logo(name);
            match inspection_refusal_v2(inspect_entity_graphic_images_v2(
                &root.capability(),
                &inventory,
                Path::new(BIN),
            )) {
                EntityImageRefusalV2::Inspection(refusal) => {
                    assert_eq!(
                        refusal.kind(),
                        GraphicImageInspectRefusalKindV2::PortRefused
                    );
                }
                other => panic!("{tag}: expected an inspection refusal, got {other:?}"),
            }
            println!("n113_declared case={tag} refused=true kind=port-refused");
        }

        // Declaration-level refusals happen at the real property-validation stage, before any read.
        // Lowercase `.svg` is admitted *syntax* since the accepted S2 grammar, so the property-stage
        // case is the uppercase extension, which stays outside the authored token set.
        for (tag, role, reference) in [
            ("uppercase", "logo", "hero.PNG"),
            ("uppercase-svg", "logo", "vector.SVG"),
            ("unknown", "logo", "hero.bmp"),
            ("favicon-gif", "favicon", "favicon.gif"),
            ("favicon-webp", "favicon", "favicon.webp"),
        ] {
            assert!(
                test_inventory_v1(&[(TestOwnerV1::Page, role, reference, "public")]).is_none(),
                "{tag} must be refused by the authored declaration validators"
            );
            println!("n113_declared case={tag} refused=true stage=property-validation");
        }

        // The positive counterpart: lowercase `.svg` passes a valid non-favicon declaration, and the
        // four-format V2 inspector then refuses it as an unsupported format for *this* gate. The same
        // `.svg` under the registered PNG-only favicon role is refused by the role, not the grammar.
        let vector_svg = b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"4\" height=\"4\"/>";
        assert!(
            test_inventory_v1(&[(TestOwnerV1::Page, "logo", "vector.svg", "public")]).is_some(),
            "lowercase .svg must be admitted as declared syntax"
        );
        let svg_root = Root::new("n113-03-svg-admitted", &[("vector.svg", &vector_svg[..])]);
        match inspection_refusal_v2(inspect_entity_graphic_images_v2(
            &svg_root.capability(),
            &one_logo("vector.svg"),
            Path::new(BIN),
        )) {
            EntityImageRefusalV2::Inspection(refusal) => {
                assert_eq!(
                    refusal.kind(),
                    GraphicImageInspectRefusalKindV2::UnsupportedFormat,
                    "the four-format V2 inspector must still refuse an .svg source"
                );
            }
            other => panic!("svg: expected an inspection refusal, got {other:?}"),
        }
        println!(
            "n113_declared case=svg-admitted-then-unsupported refused=true kind=unsupported-format"
        );

        // An `.svg` favicon never reaches inspection at all: the registered PNG-only role refuses
        // the declaration itself, which is the strictly stronger guarantee.
        assert!(
            test_inventory_v1(&[(TestOwnerV1::Page, "favicon", "favicon.svg", "public")]).is_none(),
            "an .svg favicon must be refused by the PNG-only registered role at declaration"
        );
        println!("n113_declared case=svg-favicon refused=true stage=property-validation");

        // AVIF is a valid *reference* but is outside this four-format gate, so it refuses here.
        let root = Root::new("n113-03-avif", &[("hero.avif", &png_bytes)]);
        match inspection_refusal_v2(inspect_entity_graphic_images_v2(
            &root.capability(),
            &one_logo("hero.avif"),
            Path::new(BIN),
        )) {
            EntityImageRefusalV2::Inspection(refusal) => {
                assert_eq!(
                    refusal.kind(),
                    GraphicImageInspectRefusalKindV2::UnsupportedFormat
                );
            }
            other => panic!("avif: expected an inspection refusal, got {other:?}"),
        }
        println!("n113_declared case=avif refused=true kind=unsupported-format");

        // An empty source refuses rather than decoding nothing.
        let root = Root::new("n113-03-empty", &[("hero.png", &[][..])]);
        match inspection_refusal_v2(inspect_entity_graphic_images_v2(
            &root.capability(),
            &one_logo("hero.png"),
            Path::new(BIN),
        )) {
            EntityImageRefusalV2::Inspection(refusal) => {
                assert!(matches!(
                    refusal.kind(),
                    GraphicImageInspectRefusalKindV2::EmptySource
                        | GraphicImageInspectRefusalKindV2::PortRefused
                ));
            }
            EntityImageRefusalV2::SourceRead(refusal) => {
                println!("n113_declared case=empty refused_at=source-read kind={refusal:?}");
            }
        }

        // A source above the 20 MiB ceiling refuses, whichever stage enforces it first.
        let oversize = vec![0u8; usize::try_from(MAX_IMAGE_SOURCE_BYTES_V1).unwrap() + 1];
        let root = Root::new("n113-03-oversize", &[("huge.png", &oversize)]);
        match inspect_entity_graphic_images_v2(
            &root.capability(),
            &one_logo("huge.png"),
            Path::new(BIN),
        ) {
            EntityImageOutcomeV2::Refused(EntityImageRefusalV2::Inspection(refusal)) => {
                assert_eq!(
                    refusal.kind(),
                    GraphicImageInspectRefusalKindV2::OversizeSource
                );
            }
            EntityImageOutcomeV2::Refused(EntityImageRefusalV2::SourceRead(refusal)) => {
                println!("n113_declared case=oversize refused_at=source-read kind={refusal:?}");
            }
            other => panic!("oversize: expected a refusal, got {other:?}"),
        }

        // A declared source that does not exist refuses at the confined read.
        let empty_root = Root::new("n113-03-absent", &[]);
        match inspection_refusal_v2(inspect_entity_graphic_images_v2(
            &empty_root.capability(),
            &one_logo("absent.png"),
            Path::new(BIN),
        )) {
            EntityImageRefusalV2::SourceRead(refusal) => {
                println!("n113_declared case=absent refused_at=source-read kind={refusal:?}");
            }
            EntityImageRefusalV2::Inspection(refusal) => {
                panic!("absent: refused at the wrong stage: {refusal:?}")
            }
        }
        println!("n113_declared all_cases_refused=true");
    }

    #[test]
    fn n113_04_gif_forgery_and_red_oracle_sensitivity() {
        let red_gif = super::n113_fixtures::gif87a_red_pixel();
        let blue_gif = super::n113_fixtures::gif87a_blue_pixel();
        let red_pixel = sha256_media_worker_bytes_v1(&[255, 0, 0, 255]);
        let blue_pixel = sha256_media_worker_bytes_v1(&[0, 0, 255, 255]);

        // Phase 1 — the real worker, independent pixel oracle, no filesystem fixture needed.
        let red_observation = inspect_gif_bytes_with_executable_v1(Path::new(BIN), &red_gif)
            .expect("the real confined worker must decode the RED asset");
        let blue_observation = inspect_gif_bytes_with_executable_v1(Path::new(BIN), &blue_gif)
            .expect("the real confined worker must decode the BLUE asset");
        assert_eq!(red_observation.frames()[0].composite_digest(), &red_pixel);
        assert_eq!(blue_observation.frames()[0].composite_digest(), &blue_pixel);
        assert_ne!(
            red_observation.source_digest(),
            blue_observation.source_digest()
        );
        assert_ne!(
            red_observation.transcript_digest(),
            blue_observation.transcript_digest()
        );

        // The narrow deliberate-red switch: it may only choose the *wrong expected* digest, and it
        // is never set by the committed workflow, so the default expectation is always the true RED
        // oracle. Setting it must make this assertion fail.
        let expected_red = match std::env::var("SOLO_N113_RED_ORACLE").as_deref() {
            Ok("wrong") => blue_pixel,
            _ => red_pixel,
        };
        assert_eq!(
            red_observation.frames()[0].composite_digest(),
            &expected_red,
            "the RED asset's independent pixel oracle"
        );
        assert_ne!(
            red_observation.frames()[0].composite_digest(),
            &blue_pixel,
            "a RED asset is never equal to the BLUE oracle"
        );
        println!(
            "n113_red red={} blue={} oracle=red source_distinct=true transcript_distinct=true",
            red_pixel.to_hex(),
            blue_pixel.to_hex()
        );

        // Phase 2 — the two closed forged-evidence modes, through the same parent validator.
        match inspect_gif_bytes_with_test_mode_v1(
            Path::new(BIN),
            &red_gif,
            GifWorkerTestModeV1::WrongSourceDigest,
        ) {
            Err(GifWorkerRefusalV1::SourceDigestMismatch) => {}
            other => panic!("a forged source digest must refuse, got {other:?}"),
        }
        match inspect_gif_bytes_with_test_mode_v1(
            Path::new(BIN),
            &red_gif,
            GifWorkerTestModeV1::WrongTranscriptDigest,
        ) {
            Err(GifWorkerRefusalV1::TranscriptDigestMismatch) => {}
            other => panic!("a forged transcript digest must refuse, got {other:?}"),
        }
        println!("n113_forged wrong_source=refused wrong_transcript=refused");

        // Phase 3 — both assets through the whole-candidate V2 gate, and the same forged modes
        // through the real adapter over a real acquired source set: no candidate either way.
        let root = Root::new("n113-04", &[("red.gif", &red_gif), ("blue.gif", &blue_gif)]);
        let inventory = inventory_of(&[
            (TestOwnerV1::Site, "banner", "red.gif", "public"),
            (TestOwnerV1::Page, "background", "blue.gif", "public"),
        ]);
        let set = complete_v2(inspect_entity_graphic_images_v2(
            &root.capability(),
            &inventory,
            Path::new(BIN),
        ));
        assert_eq!(set.len(), 2);
        let red_row = set.inspections()[0]
            .animation()
            .expect("the RED row is a timeline");
        let blue_row = set.inspections()[1]
            .animation()
            .expect("the BLUE row is a timeline");
        assert_eq!(red_row.frames[0].composite_digest, red_pixel);
        assert_eq!(blue_row.frames[0].composite_digest, blue_pixel);
        assert_ne!(red_row.source_digest, blue_row.source_digest);
        assert_ne!(red_row.transcript_digest, blue_row.transcript_digest);
        assert_ne!(red_row.worker_pid, blue_row.worker_pid);

        let source_set = match read_graphic_asset_bytes_v1(&inventory, &root.capability()) {
            GraphicAssetReadSetOutcomeV1::Complete(set) => set,
            GraphicAssetReadSetOutcomeV1::Refused(refusal) => {
                panic!("the real source read must succeed: {refusal:?}")
            }
        };
        for mode in [
            GifWorkerTestModeV1::WrongSourceDigest,
            GifWorkerTestModeV1::WrongTranscriptDigest,
        ] {
            let adapter = ConfinedGraphicInspectAdapterV2::with_test_mode(BIN, mode);
            match inspect_graphic_image_source_set_v2(&source_set, &adapter) {
                GraphicImageInspectionOutcomeV2::Refused(refusal) => {
                    assert_eq!(
                        refusal.kind(),
                        GraphicImageInspectRefusalKindV2::PortRefused
                    );
                }
                GraphicImageInspectionOutcomeV2::Complete(set) => {
                    panic!("forged mode {mode:?} produced {} inspections", set.len())
                }
            }
        }
        println!(
            "n113_forged adapter_wrong_source=refused adapter_wrong_transcript=refused \
             partial=false gif_pids={},{}",
            red_row.worker_pid, blue_row.worker_pid
        );
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
        run_n102_event_probe_v1, run_n102_handle_probe_v1,
    };

    use super::BIN;

    /// The single parent-only canary variable the helper parent is launched with.
    const PARENT_CANARY: &str = "SOLO_N102_PARENT_CANARY";

    /// A bounded, deterministic, non-secret canary value. It is never printed.
    const PARENT_CANARY_VALUE: &str = "solo-n102-canary-bounded-value";

    /// A numeric handle value the child can never legitimately own.
    const IMPOSSIBLE_HANDLE: u32 = 0xffff_fff0;

    /// `n102_01`: one extra real inheritable parent event, created with `bInheritHandle` set and
    /// deliberately absent from the two-entry `PROC_THREAD_ATTRIBUTE_HANDLE_LIST`, must not be
    /// reachable from the real child the production spawn creates.
    ///
    /// The evidence is object identity rather than a numeric coincidence: the child signals the
    /// transmitted candidate values, and this parent then asks the kernel about its **own** original
    /// event objects. A reused integer naming a different object cannot signal them; a genuinely
    /// inherited event handle would signal the very same kernel object and fail this test.
    #[test]
    fn n102_01_extra_inheritable_handle_is_excluded() {
        let extra = N102ExtraInheritableV1::create().expect("extra inheritable parent handle");
        // A second event in the same parent table created with `bInheritHandle = 0`: its numeric
        // value can never reach the child by inheritance, so it is the ambiguity control that tells
        // a real allowlist exclusion apart from a coincidental reuse of the same numeric value.
        let control = N102ExtraInheritableV1::create_control()
            .expect("non-inheritable control event in the same parent table");
        // Both parent events start nonsignaled and remain owned and alive across the whole probe.
        assert_eq!(
            extra.signaled_v1(),
            Some(false),
            "the extra inheritable event was already signaled before the probe"
        );
        assert_eq!(
            control.signaled_v1(),
            Some(false),
            "the control event was already signaled before the probe"
        );
        let answer = run_n102_event_probe_v1(Path::new(BIN), extra.value(), control.value())
            .expect("bounded event probe answer from the real child");
        assert!(
            answer.attempted,
            "the child did not execute the SetEvent attempts"
        );
        assert!(
            answer.witness,
            "the child could not prove SetEvent plus a zero-timeout wait on its own event"
        );
        assert_ne!(answer.pid, 0, "the child reported no process identifier");
        // The child is fully reaped here, so these report the state of the parent's own original
        // kernel objects: a genuinely inherited handle would have signaled the same object.
        let extra_signaled = extra.signaled_v1();
        let control_signaled = control.signaled_v1();
        assert_eq!(
            extra_signaled,
            Some(false),
            "the extra inheritable parent event object was signaled by the child"
        );
        assert_eq!(
            control_signaled,
            Some(false),
            "the non-inheritable control event object was signaled"
        );
        // Parent-side positive control: the same two primitives must observe a real state change on
        // a fresh parent event, otherwise a `false` above would be indistinguishable from a broken
        // wait or a failed `SetEvent`.
        let positive =
            N102ExtraInheritableV1::create_control().expect("parent-side positive-control event");
        assert_eq!(
            positive.signaled_v1(),
            Some(false),
            "the positive-control event was already signaled"
        );
        assert!(
            positive.signal_parent_v1(),
            "SetEvent failed on a live parent-owned event"
        );
        assert_eq!(
            positive.signaled_v1(),
            Some(true),
            "a zero-timeout wait did not observe a signaled parent event"
        );
        println!(
            "n102_01 extra_event_signaled={} control_event_signaled={} local_witness={} parent_positive_control=true child_pid={}",
            extra_signaled == Some(true),
            control_signaled == Some(true),
            answer.witness,
            answer.pid
        );
        drop((extra, control, positive));
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
        let mut checked = Vec::new();
        for step in 1..=solo_adapters::N102_LAST_FAULT_STEP_V1 {
            let (fault, outcome) = solo_adapters::run_n102_fault_attempt_v1(Path::new(BIN), step);
            assert_eq!(
                fault.requested(),
                Some(step),
                "step {step} was not requested"
            );
            assert_eq!(
                fault.reached(),
                Some(step),
                "step {step} never reached its real native boundary"
            );
            assert!(
                outcome.is_err(),
                "step {step} produced an observation instead of a typed refusal"
            );
            if step >= 9 {
                assert!(
                    fault.cleanup_verified(),
                    "step {step} did not observe termination of the owned child"
                );
            } else {
                assert!(
                    !fault.cleanup_verified(),
                    "step {step} claimed cleanup before any child existed"
                );
            }
            checked.push((step, fault.cleanup_verified()));
        }
        assert_eq!(
            checked.len(),
            usize::from(solo_adapters::N102_LAST_FAULT_STEP_V1)
        );
        println!(
            "n102_03 injected_steps_reached={} cleanup_observed={:?}",
            checked.len(),
            checked
        );
    }

    /// `n102_04`: a per-run fault can never reach a concurrently running worker. The value is
    /// constructed per invocation and threaded by reference, so no shared or thread-local state
    /// exists to leak it.
    #[test]
    fn n102_04_a_concurrent_worker_is_unaffected_by_a_fault() {
        let faults = std::thread::spawn(|| {
            for step in 1..=solo_adapters::N102_LAST_FAULT_STEP_V1 {
                let (_fault, outcome) =
                    solo_adapters::run_n102_fault_attempt_v1(Path::new(BIN), step);
                assert!(outcome.is_err(), "injected step {step} did not refuse");
            }
        });
        let clean = run_n102_handle_probe_v1(Path::new(BIN), IMPOSSIBLE_HANDLE, IMPOSSIBLE_HANDLE)
            .expect("a concurrent fault must not disturb a normal confined probe");
        assert!(
            !clean.control_live,
            "the child handle-table control resolved"
        );
        assert!(
            !clean.handle_live,
            "the extra parent handle reached a concurrent child"
        );
        faults.join().expect("fault thread");
        println!("n102_04 concurrent_probe_pid={} unaffected=true", clean.pid);
    }
}

#[cfg(any(target_os = "linux", windows))]
mod n108 {
    use solo_adapters::{GifDisposalV1, GifLoopV1, inspect_gif_bytes_with_executable_v1};
    use solo_application::sha256_media_worker_bytes_v1;
    use std::path::Path;

    /// Valid extension-free GIF87a: 1x1 canvas, global palette red/blue, one image
    /// descriptor, LZW codes clear / palette-index-0 / end. Code width is three bits:
    /// codes (4, 0, 5) => 0x0144, little-endian packed.
    fn gif87a_red_pixel() -> Vec<u8> {
        let mut bytes = b"GIF87a".to_vec();
        bytes.extend_from_slice(&[1, 0, 1, 0, 0x80, 0, 0]);
        bytes.extend_from_slice(&[255, 0, 0, 0, 0, 255]);
        bytes.extend_from_slice(&[0x2c, 0, 0, 0, 0, 1, 0, 1, 0, 0]);
        bytes.extend_from_slice(&[2, 2, 0x44, 0x01, 0, 0x3b]);
        bytes
    }

    #[test]
    fn n108_01_gif87a_still_confined_and_parent_verified() {
        let bytes = gif87a_red_pixel();
        let obs = inspect_gif_bytes_with_executable_v1(Path::new(super::BIN), &bytes)
            .expect("real confined GIF87a worker must succeed");
        assert_eq!(obs.signature(), "GIF87a");
        assert_eq!((obs.width(), obs.height()), (1, 1));
        assert_eq!(obs.frame_count(), 1);
        assert!(!obs.is_animated());
        assert_eq!(obs.loop_semantics(), GifLoopV1::Absent);
        assert_eq!(obs.source_digest(), &sha256_media_worker_bytes_v1(&bytes));
        assert_eq!(
            obs.frames()[0].composite_digest(),
            &sha256_media_worker_bytes_v1(&[255, 0, 0, 255])
        );
        assert!(obs.transcript_len() > 0);
        assert_ne!(obs.worker_pid(), 0);
        assert!(obs.limits_verified() && obs.cleanup_verified());
        println!(
            "n108_gif=still frames=1 pid={} src={} transcript={}",
            obs.worker_pid(),
            obs.source_digest().to_hex(),
            obs.transcript_digest().to_hex()
        );
    }

    /// Deterministic three-frame 1x1 GIF89a: NETSCAPE2.0 infinite loop, three Graphic
    /// Control Extensions (10 cs keep / 20 cs background+transparent / 5 cs keep+transparent)
    /// and three image descriptors sharing the global red/blue palette.
    fn gif89a_three_frames() -> Vec<u8> {
        let mut bytes = b"GIF89a".to_vec();
        bytes.extend_from_slice(&[1, 0, 1, 0, 0x80, 0, 0]);
        bytes.extend_from_slice(&[255, 0, 0, 0, 0, 255]);
        bytes.extend_from_slice(&[0x21, 0xff, 11]);
        bytes.extend_from_slice(b"NETSCAPE2.0");
        bytes.extend_from_slice(&[3, 1, 0, 0, 0]);
        for (delay, disposal, transparent) in [(10u8, 1u8, false), (20, 2, true), (5, 1, true)] {
            let gce = (disposal << 2) | u8::from(transparent);
            bytes.extend_from_slice(&[0x21, 0xf9, 4, gce, delay, 0, 0, 0]);
            bytes.extend_from_slice(&[0x2c, 0, 0, 0, 0, 1, 0, 1, 0, 0]);
            bytes.extend_from_slice(&[2, 2, 0x44, 0x01, 0]);
        }
        bytes.push(0x3b);
        bytes
    }

    #[test]
    fn n108_02_gif89a_animation_keeps_frames_delays_and_loop() {
        let bytes = gif89a_three_frames();
        let obs = inspect_gif_bytes_with_executable_v1(Path::new(super::BIN), &bytes)
            .expect("real confined animated GIF89a worker must succeed");
        assert_eq!(obs.signature(), "GIF89a");
        assert_eq!((obs.width(), obs.height()), (1, 1));
        assert_eq!(obs.frame_count(), 3);
        assert!(obs.is_animated());
        assert_eq!(obs.loop_semantics(), GifLoopV1::Infinite);
        assert_eq!(obs.one_cycle_delay_ms(), 350);
        let frames = obs.frames();
        assert_eq!(
            frames.iter().map(|f| f.index()).collect::<Vec<_>>(),
            [0, 1, 2]
        );
        assert_eq!(
            frames
                .iter()
                .map(|f| f.delay_centiseconds())
                .collect::<Vec<_>>(),
            [10, 20, 5]
        );
        assert_eq!(
            frames.iter().map(|f| f.disposal()).collect::<Vec<_>>(),
            [
                GifDisposalV1::Keep,
                GifDisposalV1::Background,
                GifDisposalV1::Keep
            ]
        );
        let red = sha256_media_worker_bytes_v1(&[255, 0, 0, 255]);
        let clear = sha256_media_worker_bytes_v1(&[0, 0, 0, 0]);
        assert_eq!(frames[0].composite_digest(), &red);
        assert_eq!(frames[1].composite_digest(), &red);
        assert_eq!(frames[2].composite_digest(), &clear);
        assert_eq!(obs.source_digest(), &sha256_media_worker_bytes_v1(&bytes));
        assert!(obs.transcript_len() > 0 && obs.worker_pid() != 0);
        assert!(obs.limits_verified() && obs.cleanup_verified());
        println!(
            "n108_gif=animation frames=3 duration_ms=350 pid={} src={} transcript={}",
            obs.worker_pid(),
            obs.source_digest().to_hex(),
            obs.transcript_digest().to_hex()
        );
    }

    // ---- Test-only deterministic GIF fixture encoder, ported from the accepted codec tests. ----
    // It exists only to construct exact inputs for the real child; it is never a decoding path and
    // it grants no authority. Min code size is pinned to eight bits so dictionary growth cannot mask
    // a framing bug in these tiny images.
    const RED: [u8; 3] = [255, 0, 0];
    const GREEN: [u8; 3] = [0, 255, 0];
    const BLUE: [u8; 3] = [0, 0, 255];
    const WHITE: [u8; 3] = [255, 255, 255];

    struct BitPackerV1 {
        bytes: Vec<u8>,
        accumulator: u32,
        bits: u8,
    }

    impl BitPackerV1 {
        fn new() -> Self {
            Self {
                bytes: Vec::new(),
                accumulator: 0,
                bits: 0,
            }
        }

        fn write(&mut self, code: u16, width: u8) {
            self.accumulator |= u32::from(code) << self.bits;
            self.bits += width;
            while self.bits >= 8 {
                self.bytes.push((self.accumulator & 0xFF) as u8);
                self.accumulator >>= 8;
                self.bits -= 8;
            }
        }

        fn finish(mut self) -> Vec<u8> {
            if self.bits > 0 {
                self.bytes.push((self.accumulator & 0xFF) as u8);
            }
            self.bytes
        }
    }

    fn lzw_encode_v1(min_code_size: u8, indices: &[u8]) -> Vec<u8> {
        let clear = 1u16 << min_code_size;
        let end = clear + 1;
        let mut packer = BitPackerV1::new();
        let mut code_size = min_code_size + 1;
        packer.write(clear, code_size);
        let mut table: std::collections::HashMap<(u16, u8), u16> = std::collections::HashMap::new();
        let mut next = end + 1;
        let mut current: Option<u16> = None;
        for &pixel in indices {
            match current {
                None => current = Some(u16::from(pixel)),
                Some(prefix) => {
                    if let Some(&known) = table.get(&(prefix, pixel)) {
                        current = Some(known);
                    } else {
                        packer.write(prefix, code_size);
                        table.insert((prefix, pixel), next);
                        next += 1;
                        if next == (1u16 << code_size) && code_size < 12 {
                            code_size += 1;
                        }
                        current = Some(u16::from(pixel));
                    }
                }
            }
        }
        if let Some(prefix) = current {
            packer.write(prefix, code_size);
        }
        packer.write(end, code_size);
        packer.finish()
    }

    fn table_bits_v1(entries: usize) -> (u8, usize) {
        let mut size = 2usize;
        while size < entries {
            size *= 2;
        }
        (
            u8::try_from(size.trailing_zeros())
                .unwrap_or(1)
                .saturating_sub(1),
            size,
        )
    }

    fn table_bytes_v1(palette: &[[u8; 3]]) -> (u8, Vec<u8>) {
        let (bits, entries) = table_bits_v1(palette.len().max(2));
        let mut bytes = Vec::with_capacity(entries * 3);
        for entry in palette {
            bytes.extend_from_slice(entry);
        }
        while bytes.len() < entries * 3 {
            bytes.push(0);
        }
        (bits, bytes)
    }

    fn write_sub_blocks_v1(out: &mut Vec<u8>, data: &[u8]) {
        for chunk in data.chunks(255) {
            out.push(u8::try_from(chunk.len()).unwrap_or(255));
            out.extend_from_slice(chunk);
        }
        out.push(0);
    }

    struct FrameSpecV1 {
        left: u16,
        top: u16,
        width: u16,
        height: u16,
        delay: u16,
        disposal: u8,
        transparent: Option<u8>,
        local_palette: Option<Vec<[u8; 3]>>,
        interlaced: bool,
        indices: Vec<u8>,
    }

    fn frame_v1(width: u16, height: u16, indices: &[u8]) -> FrameSpecV1 {
        FrameSpecV1 {
            left: 0,
            top: 0,
            width,
            height,
            delay: 0,
            disposal: 1,
            transparent: None,
            local_palette: None,
            interlaced: false,
            indices: indices.to_vec(),
        }
    }

    struct GifSpecV1 {
        version: &'static [u8; 6],
        width: u16,
        height: u16,
        background_index: u8,
        global_palette: Option<Vec<[u8; 3]>>,
        loop_count: Option<u16>,
        graphic_control: bool,
        frames: Vec<FrameSpecV1>,
    }

    impl GifSpecV1 {
        fn to_bytes(&self) -> Vec<u8> {
            let mut out = Vec::new();
            out.extend_from_slice(self.version);
            out.extend_from_slice(&self.width.to_le_bytes());
            out.extend_from_slice(&self.height.to_le_bytes());
            let mut packed = 0u8;
            let mut table = Vec::new();
            let mut global_entries = 2usize;
            if let Some(palette) = &self.global_palette {
                let (bits, bytes) = table_bytes_v1(palette);
                let (_, entries) = table_bits_v1(palette.len().max(2));
                global_entries = entries;
                packed |= 0x80 | bits;
                table = bytes;
            }
            out.push(packed);
            out.push(self.background_index);
            out.push(0);
            out.extend_from_slice(&table);
            if let Some(count) = self.loop_count {
                out.extend_from_slice(&[0x21, 0xFF, 11]);
                out.extend_from_slice(b"NETSCAPE2.0");
                out.extend_from_slice(&[3, 1]);
                out.extend_from_slice(&count.to_le_bytes());
                out.push(0);
            }
            for frame in &self.frames {
                if self.graphic_control {
                    let gce = (frame.disposal << 2) | u8::from(frame.transparent.is_some());
                    out.extend_from_slice(&[
                        0x21,
                        0xF9,
                        4,
                        gce,
                        (frame.delay & 0xFF) as u8,
                        (frame.delay >> 8) as u8,
                        frame.transparent.unwrap_or(0),
                        0,
                    ]);
                }
                out.push(0x2C);
                out.extend_from_slice(&frame.left.to_le_bytes());
                out.extend_from_slice(&frame.top.to_le_bytes());
                out.extend_from_slice(&frame.width.to_le_bytes());
                out.extend_from_slice(&frame.height.to_le_bytes());
                let mut descriptor = 0u8;
                let mut local = Vec::new();
                if let Some(palette) = &frame.local_palette {
                    let (bits, bytes) = table_bytes_v1(palette);
                    descriptor |= 0x80 | bits;
                    local = bytes;
                }
                if frame.interlaced {
                    descriptor |= 0x40;
                }
                out.push(descriptor);
                out.extend_from_slice(&local);
                let _ = global_entries;
                let min_code_size = 8u8;
                out.push(min_code_size);
                let data = lzw_encode_v1(min_code_size, &frame.indices);
                write_sub_blocks_v1(&mut out, &data);
            }
            out.push(0x3B);
            out
        }
    }

    fn canvas_v1(pixels: &[[u8; 4]]) -> Vec<u8> {
        let mut out = Vec::with_capacity(pixels.len() * 4);
        for pixel in pixels {
            out.extend_from_slice(pixel);
        }
        out
    }
    #[test]
    fn n108_03_gif89a_previous_background_palette_interlace_offset() {
        let red = [255, 0, 0, 255];
        let blue = [0, 0, 255, 255];
        let green = [0, 255, 0, 255];
        let white = [255, 255, 255, 255];
        let clear = [0, 0, 0, 0];

        // 1. RestorePrevious: opaque red canvas, opaque blue frame disposed Previous, transparent
        //    reveal. The reveal must show the restored red canvas, not the blue frame.
        let mut fill_red = frame_v1(2, 2, &[0, 0, 0, 0]);
        fill_red.delay = 7;
        let mut fill_blue = frame_v1(2, 2, &[1, 1, 1, 1]);
        fill_blue.delay = 7;
        fill_blue.disposal = 3;
        let mut reveal = frame_v1(1, 1, &[0]);
        reveal.delay = 7;
        reveal.transparent = Some(0);
        let previous = GifSpecV1 {
            version: b"GIF89a",
            width: 2,
            height: 2,
            background_index: 0,
            global_palette: Some(vec![RED, BLUE]),
            loop_count: None,
            graphic_control: true,
            frames: vec![fill_red, fill_blue, reveal],
        }
        .to_bytes();
        let obs = inspect_gif_bytes_with_executable_v1(Path::new(super::BIN), &previous)
            .expect("real confined restore-previous worker must succeed");
        assert_eq!(obs.signature(), "GIF89a");
        assert_eq!((obs.width(), obs.height()), (2, 2));
        assert_eq!(obs.frame_count(), 3);
        assert_eq!(obs.frames()[1].disposal(), GifDisposalV1::Previous);
        assert_eq!(
            obs.frames()[0].composite_digest(),
            &sha256_media_worker_bytes_v1(&canvas_v1(&[red, red, red, red]))
        );
        assert_eq!(
            obs.frames()[1].composite_digest(),
            &sha256_media_worker_bytes_v1(&canvas_v1(&[blue, blue, blue, blue]))
        );
        assert_eq!(
            obs.frames()[2].composite_digest(),
            &sha256_media_worker_bytes_v1(&canvas_v1(&[red, red, red, red]))
        );
        assert!(obs.transcript_len() > 0 && obs.worker_pid() != 0);
        assert!(obs.limits_verified() && obs.cleanup_verified());
        assert_eq!(
            obs.source_digest(),
            &sha256_media_worker_bytes_v1(&previous)
        );
        println!(
            "n108_gif=restore-previous frames=3 pid={} src={} transcript={}",
            obs.worker_pid(),
            obs.source_digest().to_hex(),
            obs.transcript_digest().to_hex()
        );

        // 2. RestoreBackground with a NONZERO (blue) global background index: the disposed region
        //    must become the real background colour, never transparent and never a stale red pixel.
        let fill = frame_v1(2, 2, &[0, 0, 0, 0]);
        let mut over = frame_v1(1, 1, &[0]);
        over.left = 1;
        over.top = 1;
        over.disposal = 2;
        let mut reveal_bg = frame_v1(1, 1, &[0]);
        reveal_bg.transparent = Some(0);
        let background = GifSpecV1 {
            version: b"GIF89a",
            width: 2,
            height: 2,
            background_index: 1,
            global_palette: Some(vec![RED, BLUE]),
            loop_count: None,
            graphic_control: true,
            frames: vec![fill, over, reveal_bg],
        }
        .to_bytes();
        let obs = inspect_gif_bytes_with_executable_v1(Path::new(super::BIN), &background)
            .expect("real confined opaque-background worker must succeed");
        assert_eq!(obs.frames()[1].disposal(), GifDisposalV1::Background);
        let all_red = sha256_media_worker_bytes_v1(&canvas_v1(&[red, red, red, red]));
        assert_eq!(obs.frames()[0].composite_digest(), &all_red);
        assert_eq!(obs.frames()[1].composite_digest(), &all_red);
        assert_eq!(
            obs.frames()[2].composite_digest(),
            &sha256_media_worker_bytes_v1(&canvas_v1(&[red, red, red, blue]))
        );
        assert_ne!(
            obs.frames()[2].composite_digest(),
            &sha256_media_worker_bytes_v1(&canvas_v1(&[red, red, red, clear]))
        );
        assert!(obs.transcript_len() > 0 && obs.worker_pid() != 0);
        assert!(obs.limits_verified() && obs.cleanup_verified());
        assert_eq!(
            obs.source_digest(),
            &sha256_media_worker_bytes_v1(&background)
        );
        println!(
            "n108_gif=background frames=3 pid={} src={} transcript={}",
            obs.worker_pid(),
            obs.source_digest().to_hex(),
            obs.transcript_digest().to_hex()
        );

        // 3. Local palette: a GREEN/WHITE local table must override the RED/BLUE global table.
        let mut local = frame_v1(2, 2, &[0, 0, 0, 0]);
        local.local_palette = Some(vec![GREEN, WHITE]);
        let palette = GifSpecV1 {
            version: b"GIF89a",
            width: 2,
            height: 2,
            background_index: 0,
            global_palette: Some(vec![RED, BLUE]),
            loop_count: None,
            graphic_control: true,
            frames: vec![local],
        }
        .to_bytes();
        let obs = inspect_gif_bytes_with_executable_v1(Path::new(super::BIN), &palette)
            .expect("real confined local-palette worker must succeed");
        assert!(obs.frames()[0].local_palette());
        assert_eq!(
            obs.frames()[0].composite_digest(),
            &sha256_media_worker_bytes_v1(&canvas_v1(&[green, green, green, green]))
        );
        assert!(obs.transcript_len() > 0 && obs.worker_pid() != 0);
        assert!(obs.limits_verified() && obs.cleanup_verified());
        assert_eq!(obs.source_digest(), &sha256_media_worker_bytes_v1(&palette));
        println!(
            "n108_gif=local-palette frames=1 pid={} src={} transcript={}",
            obs.worker_pid(),
            obs.source_digest().to_hex(),
            obs.transcript_digest().to_hex()
        );

        // 4. Interlaced 1x4 frame at left=1 on a 2x4 screen; stored order [0,2,1,3] is the
        //    interlace pass order (rows 0, 2, 1, 3), so the right column must be RED, GREEN,
        //    BLUE, WHITE and the untouched left column must stay zero-alpha.
        let mut interlaced = frame_v1(1, 4, &[0, 2, 1, 3]);
        interlaced.left = 1;
        interlaced.interlaced = true;
        let woven = GifSpecV1 {
            version: b"GIF89a",
            width: 2,
            height: 4,
            background_index: 0,
            global_palette: Some(vec![RED, GREEN, BLUE, WHITE]),
            loop_count: None,
            graphic_control: true,
            frames: vec![interlaced],
        }
        .to_bytes();
        let obs = inspect_gif_bytes_with_executable_v1(Path::new(super::BIN), &woven)
            .expect("real confined interlaced-offset worker must succeed");
        assert!(obs.frames()[0].interlaced());
        assert_eq!(
            (
                obs.frames()[0].left(),
                obs.frames()[0].top(),
                obs.frames()[0].width(),
                obs.frames()[0].height()
            ),
            (1, 0, 1, 4)
        );
        assert_eq!(
            obs.frames()[0].composite_digest(),
            &sha256_media_worker_bytes_v1(&canvas_v1(&[
                clear, red, clear, green, clear, blue, clear, white
            ]))
        );
        assert!(obs.transcript_len() > 0 && obs.worker_pid() != 0);
        assert!(obs.limits_verified() && obs.cleanup_verified());
        assert_eq!(obs.source_digest(), &sha256_media_worker_bytes_v1(&woven));
        println!(
            "n108_gif=interlace-offset frames=1 pid={} src={} transcript={}",
            obs.worker_pid(),
            obs.source_digest().to_hex(),
            obs.transcript_digest().to_hex()
        );
    }

    #[test]
    fn n108_04_gif_extension_signature_trailer_and_mislabeled_input_refuse() {
        // Positive controls: the known-good fixtures must still be accepted by the real child.
        let good87 = gif87a_red_pixel();
        let good89 = gif89a_three_frames();
        let control87 = inspect_gif_bytes_with_executable_v1(Path::new(super::BIN), &good87)
            .expect("positive GIF87a control must still be accepted");
        let control89 = inspect_gif_bytes_with_executable_v1(Path::new(super::BIN), &good89)
            .expect("positive GIF89a control must still be accepted");
        assert_eq!(control87.frame_count(), 1);
        assert_eq!(control89.frame_count(), 3);
        println!(
            "n108_gif=refusal-controls pids={},{} src87={} src89={}",
            control87.worker_pid(),
            control89.worker_pid(),
            control87.source_digest().to_hex(),
            control89.source_digest().to_hex()
        );

        let mut wrong_magic = good87.clone();
        wrong_magic[..6].copy_from_slice(b"BAD87a");
        let mut extra_after_trailer = good87.clone();
        extra_after_trailer.push(0);
        let truncated = good87[..good87.len() - 2].to_vec();
        let mut gif87a_with_gce = good87.clone();
        // Offset 19 is the first image descriptor for this exact fixture: six signature bytes,
        // seven logical-screen bytes, six global-colour-table bytes (two entries).
        assert_eq!(gif87a_with_gce[19], 0x2C);
        gif87a_with_gce.splice(19..19, [0x21, 0xF9, 4, 0x04, 0, 0, 0, 0]);

        // Unsupported extension block (plain text, 0x21 0x01) inserted at the same offset into an
        // otherwise valid GIF89a.
        let mut unsupported = gif87a_red_pixel();
        unsupported[..6].copy_from_slice(b"GIF89a");
        assert_eq!(unsupported[19], 0x2C);
        unsupported.splice(
            19..19,
            [0x21, 0x01, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        // Logical-screen background index outside the two-entry global colour table.
        let background_out_of_range = GifSpecV1 {
            version: b"GIF89a",
            width: 1,
            height: 1,
            background_index: 5,
            global_palette: Some(vec![RED, BLUE]),
            loop_count: None,
            graphic_control: true,
            frames: vec![frame_v1(1, 1, &[0])],
        }
        .to_bytes();

        let png = [
            0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A, 0, 0, 0, 0x0D, b'I', b'H', b'D', b'R',
        ];
        let jpeg = [
            0xFF, 0xD8, 0xFF, 0xE0, 0, 0x10, b'J', b'F', b'I', b'F', 0, 1, 1, 0, 0, 1, 0, 1, 0, 0,
            0xFF, 0xD9,
        ];
        let mut webp = b"RIFF".to_vec();
        webp.extend_from_slice(&[0x1A, 0, 0, 0]);
        webp.extend_from_slice(b"WEBPVP8 ");
        webp.extend_from_slice(&[0, 0, 0, 0]);

        let cases: [(&str, &[u8], &str); 8] = [
            ("wrong-magic", &wrong_magic, "gif-worker-source-rejected"),
            (
                "gce-in-gif87a",
                &gif87a_with_gce,
                "gif-worker-supervisor-refused",
            ),
            ("truncated", &truncated, "gif-worker-supervisor-refused"),
            (
                "extra-after-trailer",
                &extra_after_trailer,
                "gif-worker-supervisor-refused",
            ),
            (
                "unsupported-extension",
                &unsupported,
                "gif-worker-supervisor-refused",
            ),
            (
                "background-out-of-range",
                &background_out_of_range,
                "gif-worker-supervisor-refused",
            ),
            ("png-as-gif", &png, "gif-worker-source-rejected"),
            ("webp-as-gif", &webp, "gif-worker-source-rejected"),
        ];
        for (label, bytes, expected) in cases {
            match inspect_gif_bytes_with_executable_v1(Path::new(super::BIN), bytes) {
                Ok(observation) => panic!(
                    "{label}: malformed GIF was accepted with {} frames",
                    observation.frame_count()
                ),
                Err(refusal) => {
                    println!("n108_gif=refusal case={label} token={}", refusal.token());
                    assert_eq!(
                        refusal.token(),
                        expected,
                        "{label}: unexpected refusal token"
                    );
                }
            }
        }
        match inspect_gif_bytes_with_executable_v1(Path::new(super::BIN), &jpeg) {
            Ok(_) => panic!("jpeg-as-gif: mislabeled JPEG was accepted"),
            Err(refusal) => {
                println!(
                    "n108_gif=refusal case=jpeg-as-gif token={}",
                    refusal.token()
                );
                assert_eq!(refusal.token(), "gif-worker-source-rejected");
            }
        }
    }

    /// One 1x1 GIF89a with the shared RED/BLUE palette and one RED frame per authored delay, so the
    /// loop extension and per-frame timing are the only variables.
    fn gif89a_red_delays(loop_count: Option<u16>, delays: &[u16]) -> Vec<u8> {
        let frames = delays
            .iter()
            .map(|delay| {
                let mut frame = frame_v1(1, 1, &[0]);
                frame.delay = *delay;
                frame
            })
            .collect();
        GifSpecV1 {
            version: b"GIF89a",
            width: 1,
            height: 1,
            background_index: 0,
            global_palette: Some(vec![RED, BLUE]),
            loop_count,
            graphic_control: true,
            frames,
        }
        .to_bytes()
    }

    /// Every accepted observation must prove the real supervised child ran and stays internally
    /// consistent: matching source digest, bounded nonzero transcript and a reaped, confined PID.
    fn assert_real_child_evidence(obs: &solo_adapters::GifWorkerObservationV1, bytes: &[u8]) {
        assert_eq!(obs.source_digest(), &sha256_media_worker_bytes_v1(bytes));
        assert!(obs.transcript_len() > 0);
        assert_ne!(obs.worker_pid(), 0);
        assert!(obs.limits_verified() && obs.cleanup_verified());
    }

    #[test]
    fn n108_05_gif89a_absent_finite_infinite_loops_and_zero_delay() {
        let red = sha256_media_worker_bytes_v1(&canvas_v1(&[[255, 0, 0, 255]]));

        // 1. No repeat extension at all: absent, not implicit infinite, and zero delay stays zero.
        let absent = gif89a_red_delays(None, &[0]);
        let obs = inspect_gif_bytes_with_executable_v1(Path::new(super::BIN), &absent)
            .expect("extension-free GIF89a must pass the real confined worker");
        assert_eq!(obs.loop_semantics(), GifLoopV1::Absent);
        assert_eq!(obs.one_cycle_delay_ms(), 0);
        assert_eq!(obs.frame_count(), 1);
        assert!(!obs.is_animated());
        assert_eq!(obs.frames()[0].delay_centiseconds(), 0);
        assert_eq!(obs.frames()[0].composite_digest(), &red);
        assert_real_child_evidence(&obs, &absent);
        println!(
            "n108_gif=loop case=absent loop={} cycle_ms={} frames={} pid={} src={} transcript={}",
            obs.loop_semantics().token(),
            obs.one_cycle_delay_ms(),
            obs.frame_count(),
            obs.worker_pid(),
            obs.source_digest().to_hex(),
            obs.transcript_digest().to_hex()
        );

        // 2. Finite one: an authored single repeat is finite, never collapsed into "absent".
        let one = gif89a_red_delays(Some(1), &[1]);
        let obs = inspect_gif_bytes_with_executable_v1(Path::new(super::BIN), &one)
            .expect("finite-one GIF89a must pass the real confined worker");
        assert_eq!(obs.loop_semantics(), GifLoopV1::Finite(1));
        assert_eq!(obs.one_cycle_delay_ms(), 10);
        assert_eq!(obs.frames()[0].delay_centiseconds(), 1);
        assert_eq!(obs.frames()[0].composite_digest(), &red);
        assert_real_child_evidence(&obs, &one);
        println!(
            "n108_gif=loop case=finite-one loop={} cycle_ms={} frames={} pid={} src={} transcript={}",
            obs.loop_semantics().token(),
            obs.one_cycle_delay_ms(),
            obs.frame_count(),
            obs.worker_pid(),
            obs.source_digest().to_hex(),
            obs.transcript_digest().to_hex()
        );

        // 3. Finite two with a five-centisecond frame delay.
        let two = gif89a_red_delays(Some(2), &[5]);
        let obs = inspect_gif_bytes_with_executable_v1(Path::new(super::BIN), &two)
            .expect("finite-two GIF89a must pass the real confined worker");
        assert_eq!(obs.loop_semantics(), GifLoopV1::Finite(2));
        assert_eq!(obs.one_cycle_delay_ms(), 50);
        assert_eq!(obs.frames()[0].delay_centiseconds(), 5);
        assert_real_child_evidence(&obs, &two);
        println!(
            "n108_gif=loop case=finite-two loop={} cycle_ms={} frames={} pid={} src={} transcript={}",
            obs.loop_semantics().token(),
            obs.one_cycle_delay_ms(),
            obs.frame_count(),
            obs.worker_pid(),
            obs.source_digest().to_hex(),
            obs.transcript_digest().to_hex()
        );

        // 4. Repeat count zero is infinite: one authored frame, so the timeline is still a still.
        let infinite = gif89a_red_delays(Some(0), &[0]);
        let obs = inspect_gif_bytes_with_executable_v1(Path::new(super::BIN), &infinite)
            .expect("infinite-loop GIF89a must pass the real confined worker");
        assert_eq!(obs.loop_semantics(), GifLoopV1::Infinite);
        assert_eq!(obs.one_cycle_delay_ms(), 0);
        assert_eq!(obs.frame_count(), 1);
        assert!(!obs.is_animated());
        assert_real_child_evidence(&obs, &infinite);
        println!(
            "n108_gif=loop case=infinite loop={} cycle_ms={} frames={} pid={} src={} transcript={}",
            obs.loop_semantics().token(),
            obs.one_cycle_delay_ms(),
            obs.frame_count(),
            obs.worker_pid(),
            obs.source_digest().to_hex(),
            obs.transcript_digest().to_hex()
        );

        // 5. Mixed authored delays are kept exactly, never normalised to a browser minimum.
        let mixed = gif89a_red_delays(None, &[0, 7, 3]);
        let obs = inspect_gif_bytes_with_executable_v1(Path::new(super::BIN), &mixed)
            .expect("multi-frame GIF89a must pass the real confined worker");
        assert_eq!(obs.frame_count(), 3);
        assert!(obs.is_animated());
        assert_eq!(obs.one_cycle_delay_ms(), 100);
        assert_eq!(
            obs.frames()
                .iter()
                .map(|frame| frame.delay_centiseconds())
                .collect::<Vec<_>>(),
            [0, 7, 3]
        );
        assert_real_child_evidence(&obs, &mixed);
        println!(
            "n108_gif=loop case=mixed-delays loop={} cycle_ms={} frames={} delays={:?} pid={} src={} transcript={}",
            obs.loop_semantics().token(),
            obs.one_cycle_delay_ms(),
            obs.frame_count(),
            obs.frames()
                .iter()
                .map(|frame| frame.delay_centiseconds())
                .collect::<Vec<_>>(),
            obs.worker_pid(),
            obs.source_digest().to_hex(),
            obs.transcript_digest().to_hex()
        );
    }

    #[test]
    fn n108_06_gif_resource_budget_and_frame_rectangle_refusals() {
        let control = gif89a_red_delays(None, &[0]);
        let accepted = inspect_gif_bytes_with_executable_v1(Path::new(super::BIN), &control)
            .expect("the 1x1 GIF89a control must stay accepted");
        assert_eq!(accepted.frame_count(), 1);
        println!(
            "n108_gif=budget control frames=1 pid={} src={}",
            accepted.worker_pid(),
            accepted.source_digest().to_hex()
        );

        // 1. 1025 frames: exactly one over MAX_GIF_FRAMES_V1. The whole animation must refuse; no
        // partial timeline may survive.
        let frames: Vec<FrameSpecV1> = (0..=1024).map(|_| frame_v1(1, 1, &[0])).collect();
        let too_many_frames = GifSpecV1 {
            version: b"GIF89a",
            width: 1,
            height: 1,
            background_index: 0,
            global_palette: Some(vec![RED, BLUE]),
            loop_count: None,
            graphic_control: true,
            frames,
        }
        .to_bytes();

        // 2. Logical screen side 8193 with a compact 1x1 frame: the canvas side limit is eight
        // thousand one hundred and ninety-two.
        let over_wide_canvas = GifSpecV1 {
            version: b"GIF89a",
            width: 8193,
            height: 1,
            background_index: 0,
            global_palette: Some(vec![RED, BLUE]),
            loop_count: None,
            graphic_control: true,
            frames: vec![frame_v1(1, 1, &[0])],
        }
        .to_bytes();

        // 3. 8192 x 8192 logical screen with two 1x1 frames: 2 x 67,108,864 composed pixels exceeds
        // 128,000,000 and must refuse from the container scan before any canvas allocation. The
        // fixture stays compact because no large bitmap is ever built.
        let composed_pixels_exceeded = GifSpecV1 {
            version: b"GIF89a",
            width: 8192,
            height: 8192,
            background_index: 0,
            global_palette: Some(vec![RED, BLUE]),
            loop_count: None,
            graphic_control: true,
            frames: vec![frame_v1(1, 1, &[0]), frame_v1(1, 1, &[0])],
        }
        .to_bytes();
        assert_eq!(2u64 * 8192 * 8192, 134_217_728);

        // 4. Six frames of 65,535 centiseconds: 3,932,100 ms exceeds the 3,600,000 ms cycle limit.
        let duration_exceeded = gif89a_red_delays(None, &[u16::MAX; 6]);
        assert_eq!(6u64 * 655_350, 3_932_100);

        // 5. Authored frame rectangle outside a 2x2 canvas: no clipping, no partial success.
        let mut outside = frame_v1(2, 2, &[0, 0, 0, 0]);
        outside.left = 1;
        let rectangle_outside = GifSpecV1 {
            version: b"GIF89a",
            width: 2,
            height: 2,
            background_index: 0,
            global_palette: Some(vec![RED, BLUE]),
            loop_count: None,
            graphic_control: true,
            frames: vec![outside],
        }
        .to_bytes();

        // 6. Exactly one byte over the source ceiling: refused by the parent before any child spawn.
        let ceiling = usize::try_from(solo_adapters::MAX_GIF_SOURCE_BYTES_V1)
            .expect("the GIF source ceiling fits usize")
            + 1;
        let mut over_source = b"GIF89a".to_vec();
        over_source.resize(ceiling, 0);
        assert_eq!(over_source.len(), ceiling);

        let cases: [(&str, &[u8], &str); 6] = [
            (
                "frames-1025",
                &too_many_frames,
                "gif-worker-supervisor-refused",
            ),
            (
                "canvas-8193",
                &over_wide_canvas,
                "gif-worker-supervisor-refused",
            ),
            (
                "composed-134217728",
                &composed_pixels_exceeded,
                "gif-worker-supervisor-refused",
            ),
            (
                "duration-3932100",
                &duration_exceeded,
                "gif-worker-supervisor-refused",
            ),
            (
                "frame-outside-canvas",
                &rectangle_outside,
                "gif-worker-supervisor-refused",
            ),
            (
                "source-over-ceiling",
                &over_source,
                "gif-worker-source-rejected",
            ),
        ];
        for (label, bytes, expected) in cases {
            match inspect_gif_bytes_with_executable_v1(Path::new(super::BIN), bytes) {
                Ok(observation) => panic!(
                    "{label}: refused specimen was accepted with {} frames",
                    observation.frame_count()
                ),
                Err(refusal) => {
                    println!("n108_gif=budget case={label} token={}", refusal.token());
                    assert_eq!(
                        refusal.token(),
                        expected,
                        "{label}: unexpected refusal token"
                    );
                }
            }
        }
    }

    #[test]
    fn n108_07_gif_application_repeat_and_lzw_corruption_refusals() {
        // Positive controls: the known-good fixtures must still be accepted by the real child.
        let good87 = gif87a_red_pixel();
        let good89 = gif89a_three_frames();
        let control87 = inspect_gif_bytes_with_executable_v1(Path::new(super::BIN), &good87)
            .expect("positive GIF87a control must still be accepted");
        let control89 = inspect_gif_bytes_with_executable_v1(Path::new(super::BIN), &good89)
            .expect("positive GIF89a control must still be accepted");
        assert_eq!(control87.frame_count(), 1);
        assert_eq!(control89.frame_count(), 3);
        println!(
            "n108_gif=malformed control87 pid={} src={} control89 pid={} src={}",
            control87.worker_pid(),
            control87.source_digest().to_hex(),
            control89.worker_pid(),
            control89.source_digest().to_hex()
        );

        // Duplicate NETSCAPE repeat block: the exact verified nineteen-byte block sits at offset
        // nineteen, so a second identical copy can be spliced immediately after it.
        let mut duplicated_repeat = good89.clone();
        let repeat = duplicated_repeat[19..38].to_vec();
        assert_eq!(repeat.len(), 19);
        assert!(repeat.starts_with(&[0x21, 0xff, 11]));
        assert_eq!(&repeat[3..14], b"NETSCAPE2.0");
        assert_eq!(&repeat[14..19], &[3, 1, 0, 0, 0]);
        duplicated_repeat.splice(38..38, repeat);

        // Unknown application identifier with intact block framing.
        let mut unknown_application = good89.clone();
        assert_eq!(&unknown_application[22..33], b"NETSCAPE2.0");
        unknown_application[22..33].copy_from_slice(b"NOTSCAPE2.0");

        // Malformed Graphic Control Extension: block size four mutated to three.
        let mut short_control = good89.clone();
        assert_eq!(&short_control[38..41], &[0x21, 0xf9, 4]);
        short_control[40] = 3;

        // LZW without an end code: the exact one-pixel payload of the GIF87a control is replaced by
        // a deterministic clear-plus-index stream that never emits the end code the strict decoder
        // requires (`check_lzw_end_code(true)`).
        let mut unterminated_lzw = good87.clone();
        assert_eq!(unterminated_lzw.len(), 35);
        assert_eq!(unterminated_lzw[29], 2);
        assert_eq!(unterminated_lzw[30], 2);
        assert_eq!(&unterminated_lzw[31..33], &[0x44, 0x01]);
        assert_eq!(unterminated_lzw[33], 0);
        assert_eq!(unterminated_lzw[34], 0x3b);
        unterminated_lzw[31] = 0x04;
        unterminated_lzw[32] = 0x00;

        // Missing sub-block terminator: the verified terminator byte becomes a one-byte block.
        let mut missing_terminator = good87.clone();
        assert_eq!(missing_terminator[33], 0);
        missing_terminator[33] = 1;

        // Compressed sub-block claiming more bytes than the stream still holds.
        let mut truncated_block = good87.clone();
        assert_eq!(truncated_block[30], 2);
        truncated_block[30] = 4;

        let cases: [(&str, &[u8]); 6] = [
            ("duplicate-repeat", &duplicated_repeat),
            ("unknown-application", &unknown_application),
            ("short-graphic-control", &short_control),
            ("unterminated-lzw", &unterminated_lzw),
            ("missing-sub-block-terminator", &missing_terminator),
            ("truncated-compressed-block", &truncated_block),
        ];
        for (label, bytes) in cases {
            match inspect_gif_bytes_with_executable_v1(Path::new(super::BIN), bytes) {
                Ok(observation) => panic!(
                    "{label}: corrupt GIF was accepted with {} frames",
                    observation.frame_count()
                ),
                Err(refusal) => {
                    println!("n108_gif=malformed case={label} token={}", refusal.token());
                    assert_eq!(
                        refusal.token(),
                        "gif-worker-supervisor-refused",
                        "{label}: unexpected refusal token"
                    );
                }
            }
        }
    }

    /// One 1x1 GIF89a selecting `index` from the shared RED/BLUE palette with an authored delay.
    fn gif89a_index_delay(index: u8, delay: u16) -> Vec<u8> {
        let mut frame = frame_v1(1, 1, &[index]);
        frame.delay = delay;
        GifSpecV1 {
            version: b"GIF89a",
            width: 1,
            height: 1,
            background_index: 0,
            global_palette: Some(vec![RED, BLUE]),
            loop_count: None,
            graphic_control: true,
            frames: vec![frame],
        }
        .to_bytes()
    }

    #[test]
    fn n108_08_gif_source_transcript_and_pixel_change_evidence() {
        let red = sha256_media_worker_bytes_v1(&canvas_v1(&[[255, 0, 0, 255]]));
        let blue = sha256_media_worker_bytes_v1(&canvas_v1(&[[0, 0, 255, 255]]));

        let a_bytes = gif89a_index_delay(0, 0);
        let b_bytes = gif89a_index_delay(1, 0);
        let c_bytes = gif89a_index_delay(0, 5);
        assert_ne!(a_bytes, b_bytes, "B must differ from A only by pixel index");
        assert_ne!(a_bytes, c_bytes, "C must differ from A only by delay");

        let run = |bytes: &Vec<u8>, label: &str| {
            let obs = inspect_gif_bytes_with_executable_v1(Path::new(super::BIN), bytes)
                .expect("every valid GIF89a specimen must pass the real confined worker");
            assert_real_child_evidence(&obs, bytes);
            assert_eq!(obs.frame_count(), 1);
            println!(
                "n108_gif=evidence case={label} pid={} src={} transcript={} pixel={} cycle_ms={}",
                obs.worker_pid(),
                obs.source_digest().to_hex(),
                obs.transcript_digest().to_hex(),
                obs.frames()[0].composite_digest().to_hex(),
                obs.one_cycle_delay_ms()
            );
            obs
        };

        // A is inspected twice through separate supervised children.
        let a1 = run(&a_bytes, "A1");
        let a2 = run(&a_bytes, "A2");
        let b = run(&b_bytes, "B");
        let c = run(&c_bytes, "C");

        // Pixel evidence is asserted independently of the decoder's own digest arithmetic.
        assert_eq!(a1.frames()[0].composite_digest(), &red);
        assert_eq!(a2.frames()[0].composite_digest(), &red);
        assert_eq!(b.frames()[0].composite_digest(), &blue);
        assert_eq!(c.frames()[0].composite_digest(), &red);

        // A different authored asset changes both the source and the transcript binding.
        assert_ne!(a1.source_digest(), b.source_digest());
        assert_ne!(a1.transcript_digest(), b.transcript_digest());
        assert_ne!(a1.source_digest(), c.source_digest());
        assert_ne!(a1.transcript_digest(), c.transcript_digest());

        // C keeps A's composited pixels while changing only the timing contract.
        assert_eq!(
            a1.frames()[0].composite_digest(),
            c.frames()[0].composite_digest()
        );
        assert_eq!(a1.one_cycle_delay_ms(), 0);
        assert_eq!(c.one_cycle_delay_ms(), 50);

        // Two separate real children agree on every piece of evidence except their own identity.
        assert_eq!(a1.source_digest(), a2.source_digest());
        assert_eq!(a1.transcript_digest(), a2.transcript_digest());
        assert_eq!(a1.transcript_len(), a2.transcript_len());
        assert_eq!(
            a1.frames()[0].composite_digest(),
            a2.frames()[0].composite_digest()
        );
        assert_eq!(a1.one_cycle_delay_ms(), a2.one_cycle_delay_ms());
        assert_ne!(a1.worker_pid(), a2.worker_pid());
    }
}

// `SOLO-WO-116 R03` — the four mandatory five-format real-host cases.
//
// They live in a sibling support module of `n097` because that is the existing legitimate
// real-fixture seam: the same `SOLO_TEST_SOURCE_FS_ROOT` Btrfs root, the same real confined source
// reader and the same real supervised worker binary. Nothing here is a mock port, a fabricated
// source set or a synthetic completed inspection.
#[cfg(test)]
mod n116 {
    use std::fs;
    use std::path::PathBuf;

    use solo_adapters::{
        ContentRootCapability, EntityImageOutcomeV3, RootProvenanceV1, SvgWorkerRefusalV1,
        SvgWorkerTestModeV1, TestOwnerV1, inspect_entity_graphic_images_v3,
        inspect_svg_bytes_with_executable_v1, inspect_svg_bytes_with_test_mode_v1,
        test_inventory_suppressed_v1, test_inventory_v1,
    };
    use solo_application::{
        GraphicAssetObligationInventoryV1, GraphicImageFormatV3, GraphicImageInspectRefusalKindV3,
        GraphicImageInspectionSetV3, IMAGE_MIME_SVG_V3,
    };

    /// The one worker binary these cases start.
    const BIN: &str = super::BIN;

    /// The trusted provenance marker for an adopted root.
    const TRUSTED: RootProvenanceV1 = RootProvenanceV1::OperatorDesignatedContentRoot;

    /// Independently pinned SHA-256 of the canonical vector for [`rich_svg`].
    ///
    /// This literal is the expected value authored for review; both hosts must reproduce it.
    const RICH_CANONICAL_SHA256: &str =
        "0403e0677a174ccafe18191a8a9f1c5a30993477bb074f49078b005f513caaba";

    /// Independently pinned SHA-256 of the canonical vector for the pinned Foundation icon.
    const ICON_CANONICAL_SHA256: &str =
        "ce9ea48a3d7bc41673f969727c49eb5dd111d27451d27adab403faf3fa3362d5";

    /// A four-pixel 2×2 PNG (red/green/blue/white), the accepted `n097` fixture.
    const FIXTURE_PNG: &str = "iVBORw0KGgoAAAANSUhEUgAAAAIAAAACCAYAAABytg0kAAAAFklEQVR4AWL6z8DwHwSZGEEkw38GAAAAAP//4+5legAAAAZJREFUAwBBDwb/5c5bAAAAAABJRU5ErkJggg==";

    /// The same pixels as a 2×2 baseline JPEG, the accepted `n097` fixture.
    const FIXTURE_JPEG: &str = "/9j/4AAQSkZJRgABAQAAAQABAAD/4gHYSUNDX1BST0ZJTEUAAQEAAAHIAAAAAAQwAABtbnRyUkdCIFhZWiAH4AABAAEAAAAAAABhY3NwAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQAA9tYAAQAAAADTLQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAlkZXNjAAAA8AAAACRyWFlaAAABFAAAABRnWFlaAAABKAAAABRiWFlaAAABPAAAABR3dHB0AAABUAAAABRyVFJDAAABZAAAAChnVFJDAAABZAAAAChiVFJDAAABZAAAAChjcHJ0AAABjAAAADxtbHVjAAAAAAAAAAEAAAAMZW5VUwAAAAgAAAAcAHMAUgBHAEJYWVogAAAAAAAAb6IAADj1AAADkFhZWiAAAAAAAABimQAAt4UAABjaWFlaIAAAAAAAACSgAAAPhAAAts9YWVogAAAAAAAA9tYAAQAAAADTLXBhcmEAAAAAAAQAAAACZmYAAPKnAAANWQAAE9AAAApbAAAAAAAAAABtbHVjAAAAAAAAAAEAAAAMZW5VUwAAACAAAAAcAEcAbwBvAGcAbABlACAASQBuAGMALgAgADIAMAAxADb/2wBDAAEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQH/2wBDAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQH/wAARCAACAAIDAREAAhEBAxEB/8QAFAABAAAAAAAAAAAAAAAAAAAACv/EABsQAAIDAQEBAAAAAAAAAAAAAAQFAwYHAggJ/8QAFQEBAQAAAAAAAAAAAAAAAAAABgf/xAAbEQACAwEBAQAAAAAAAAAAAAAEBgMFBwIIAf/aAAwDAQACEQMRAD8AWB84MIw+y/PHwZYrFjWUv7A/8YeXHT166zuoNXLpy1w+jHNGzZocnnNZM2Rs85h55k8xRhU0pBEsk0nffSD1D58wOq9MeiaurxDIK2trd01sCurgM0TAwQAQ39gHECCEHpYxxRBR444BxoI44YIY+IouOeOeefkfPxfHX0416esnzR0dnQslscXFsRFZjamxqY5u7hhZmZhuKoy3vmC9tzDLS5ubQwqxtLEok44mcmeWXr//2Q==";

    /// The same pixels as a 2×2 WebP still container, the accepted `n097` fixture.
    const FIXTURE_WEBP: &str = "UklGRg4CAABXRUJQVlA4WAoAAAAgAAAAAQAAAQAASUNDUMgBAAAAAAHIAAAAAAQwAABtbnRyUkdCIFhZWiAH4AABAAEAAAAAAABhY3NwAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQAA9tYAAQAAAADTLQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAlkZXNjAAAA8AAAACRyWFlaAAABFAAAABRnWFlaAAABKAAAABRiWFlaAAABPAAAABR3dHB0AAABUAAAABRyVFJDAAABZAAAAChnVFJDAAABZAAAAChiVFJDAAABZAAAAChjcHJ0AAABjAAAADxtbHVjAAAAAAAAAAEAAAAMZW5VUwAAAAgAAAAcAHMAUgBHAEJYWVogAAAAAAAAb6IAADj1AAADkFhZWiAAAAAAAABimQAAt4UAABjaWFlaIAAAAAAAACSgAAAPhAAAts9YWVogAAAAAAAA9tYAAQAAAADTLXBhcmEAAAAAAAQAAAACZmYAAPKnAAANWQAAE9AAAApbAAAAAAAAAABtbHVjAAAAAAAAAAEAAAAMZW5VUwAAACAAAAAcAEcAbwBvAGcAbABlACAASQBuAGMALgAgADIAMAAxADZWUDhMHwAAAC8BQAAAHyAQSN4fOo35FxAU/B/d/EdkD+AGDBH9DwEA";

    /// The accepted deterministic three-frame GIF89a fixture, with an infinite NETSCAPE loop and
    /// three Graphic Control extensions, shared with the N113 cases.
    fn gif_fixture() -> Vec<u8> {
        super::n113_fixtures::gif89a_three_frames()
    }

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

    /// A disposable confined content root holding exactly the files a case declares.
    pub(super) struct Root {
        dir: PathBuf,
    }

    impl Root {
        fn parent() -> PathBuf {
            #[cfg(target_os = "linux")]
            {
                match std::env::var_os("SOLO_TEST_SOURCE_FS_ROOT") {
                    Some(root) if !root.is_empty() => PathBuf::from(root),
                    _ => panic!(
                        "SOLO_TEST_SOURCE_FS_ROOT must name the R03 snapshot fixture filesystem; \
                         refusing to run the real five-format suite on an unverified filesystem"
                    ),
                }
            }
            #[cfg(not(target_os = "linux"))]
            {
                std::env::temp_dir()
            }
        }

        pub(super) fn new(tag: &str, assets: &[(&str, &[u8])]) -> Self {
            let dir = Self::parent().join(format!("solo-n116-{}-{tag}", std::process::id()));
            let _ = fs::remove_dir_all(&dir);
            fs::create_dir_all(dir.join("assets")).expect("scratch assets directory");
            for (name, bytes) in assets {
                fs::write(dir.join("assets").join(name), bytes).expect("fixture write");
            }
            Self { dir }
        }

        pub(super) fn capability(&self) -> ContentRootCapability {
            ContentRootCapability::adopt_bootstrap_path(&self.dir, TRUSTED).expect("adopted root")
        }
    }

    impl Drop for Root {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.dir);
        }
    }

    pub(super) fn inventory_of(
        entries: &[(TestOwnerV1, &str, &str, &str)],
    ) -> GraphicAssetObligationInventoryV1 {
        test_inventory_v1(entries).expect("a validated test inventory")
    }

    /// The same validation with every declaration **suppressing** inheritance, so a declared
    /// source is present but not publishable — it must still be inspected.
    pub(super) fn suppressed_inventory_of(
        entries: &[(TestOwnerV1, &str, &str, &str)],
    ) -> GraphicAssetObligationInventoryV1 {
        test_inventory_suppressed_v1(entries).expect("a validated suppressing inventory")
    }

    /// The rich static-vector positive: every mandated construct in one document.
    pub(super) fn rich_svg() -> String {
        let mut svg = String::new();
        svg.push_str("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"24\" height=\"24\" ");
        svg.push_str("viewBox=\"0 0 24 24\">");
        svg.push_str("<defs>");
        svg.push_str("<linearGradient id=\"lg\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"1\">");
        svg.push_str("<stop offset=\"0\" stop-color=\"#ff0000\"/>");
        svg.push_str("<stop offset=\"100%\" stop-color=\"#0000ff\"/></linearGradient>");
        svg.push_str("<radialGradient id=\"rg\" cx=\"0.5\" cy=\"0.5\" r=\"0.5\">");
        svg.push_str("<stop offset=\"0.5\" stop-color=\"#00ff00\"/></radialGradient>");
        svg.push_str("<clipPath id=\"cp\"><rect x=\"0\" y=\"0\" width=\"8\" height=\"8\"/>");
        svg.push_str("</clipPath>");
        svg.push_str("<mask id=\"mk\"><circle cx=\"4\" cy=\"4\" r=\"3\"/></mask>");
        svg.push_str("<filter id=\"fl\">");
        svg.push_str("<feGaussianBlur in=\"SourceGraphic\" stdDeviation=\"1.5\" result=\"b\"/>");
        svg.push_str("<feOffset in=\"b\" dx=\"1\" dy=\"1\" result=\"o\"/>");
        svg.push_str("<feComposite in=\"o\" in2=\"SourceAlpha\" operator=\"over\" result=\"c\"/>");
        svg.push_str("<feMerge><feMergeNode in=\"c\"/></feMerge></filter></defs>");
        svg.push_str("<g transform=\"translate(1 2) rotate(30)\" clip-path=\"url(#cp)\" ");
        svg.push_str("mask=\"url(#mk)\" filter=\"url(#fl)\">");
        svg.push_str("<path d=\"M1 1L2 2H3V4C5 5 6 6 7 7S8 8 9 9Q10 10 11 11T12 12");
        svg.push_str("A3 3 0 0 1 13 13Z\" fill=\"url(#lg)\"/>");
        svg.push_str(
            "<rect x=\"1\" y=\"1\" width=\"2\" height=\"3\" rx=\"1\" stroke=\"url(#rg)\"/>",
        );
        svg.push_str("<circle cx=\"4\" cy=\"4\" r=\"2\"/>");
        svg.push_str("<ellipse cx=\"4\" cy=\"4\" rx=\"2\" ry=\"3\"/>");
        svg.push_str("<line x1=\"0\" y1=\"0\" x2=\"4\" y2=\"4\"/>");
        svg.push_str("<polyline points=\"0,0 1,1 2,0\"/>");
        svg.push_str("<polygon points=\"0,0 2,0 1,2\"/></g>");
        svg.push_str("<text x=\"2\" y=\"3\" font-size=\"14\" fill=\"currentColor\">Hi 42</text>");
        svg.push_str("</svg>");
        svg
    }

    /// The pinned Foundation donor icon, normalised from the checkout's line endings to the exact
    /// pinned LF fixture bytes **before** hashing. The pinned file itself is never modified.
    pub(super) fn icon_home_lf() -> Vec<u8> {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/svg/icon-home.svg"
        );
        let raw = fs::read(path).expect("pinned Foundation icon present");
        let mut out = Vec::with_capacity(raw.len());
        let mut index = 0;
        while index < raw.len() {
            if raw[index] == b'\r' && raw.get(index + 1) == Some(&b'\n') {
                index += 1;
                continue;
            }
            out.push(raw[index]);
            index += 1;
        }
        out
    }

    /// The closed set of hostile documents the refusal cases draw from.
    pub(super) fn hostile(name: &str) -> Vec<u8> {
        let body = match name {
            "script" => "<svg xmlns=\"http://www.w3.org/2000/svg\"><script>alert(1)</script></svg>",
            "dtd" => {
                "<!DOCTYPE svg [<!ENTITY x SYSTEM \"file:///e\">]><svg xmlns=\"http://www.w3.org/2000/svg\"><path d=\"M0 0\"/></svg>"
            }
            "pi" => {
                "<?xml version=\"1.0\"?><svg xmlns=\"http://www.w3.org/2000/svg\"><path d=\"M0 0\"/></svg>"
            }
            "comment" => {
                "<svg xmlns=\"http://www.w3.org/2000/svg\"><!-- c --><path d=\"M0 0\"/></svg>"
            }
            "foreign" => {
                "<svg xmlns=\"http://www.w3.org/2000/svg\"><foreignObject><div/></foreignObject></svg>"
            }
            "event" => {
                "<svg xmlns=\"http://www.w3.org/2000/svg\"><path d=\"M0 0\" onclick=\"x()\"/></svg>"
            }
            "css" => "<svg xmlns=\"http://www.w3.org/2000/svg\"><style>.a{fill:red}</style></svg>",
            "remote" => {
                "<svg xmlns=\"http://www.w3.org/2000/svg\"><rect fill=\"url(http://e.com/a)\"/></svg>"
            }
            "cycle" => {
                "<svg xmlns=\"http://www.w3.org/2000/svg\"><defs><clipPath id=\"a\"><rect clip-path=\"url(#a)\"/></clipPath></defs></svg>"
            }
            "dup" => {
                "<svg xmlns=\"http://www.w3.org/2000/svg\"><defs><clipPath id=\"a\"/><mask id=\"a\"/></defs></svg>"
            }
            "wrongns" => "<svg xmlns=\"http://example.com/not-svg\"><path d=\"M0 0\"/></svg>",
            "text" => {
                "<svg xmlns=\"http://www.w3.org/2000/svg\"><text font-family=\"Helvetica\">x</text></svg>"
            }
            "animate" => {
                "<svg xmlns=\"http://www.w3.org/2000/svg\"><path d=\"M0 0\"><animate attributeName=\"d\"/></path></svg>"
            }
            other => panic!("unknown hostile document {other:?}"),
        };
        body.as_bytes().to_vec()
    }

    /// Whether the deliberate-RED switch is engaged. It changes only this test's expectation.
    pub(super) fn red_oracle() -> bool {
        std::env::var("SOLO_N116_RED_ORACLE").is_ok_and(|value| value == "wrong")
    }

    // ---- the four mandatory cases -----------------------------------------------------------

    fn hex(digest: &solo_application::MediaWorkerDigestV1) -> String {
        digest.to_hex()
    }

    fn complete(outcome: EntityImageOutcomeV3) -> GraphicImageInspectionSetV3 {
        match outcome {
            EntityImageOutcomeV3::Complete(set) => set,
            EntityImageOutcomeV3::ReadRefused(refusal) => {
                panic!("source read refused: {refusal:?}")
            }
            EntityImageOutcomeV3::Refused(refusal) => {
                panic!("expected a complete inspection, got {refusal:?}")
            }
        }
    }

    fn refusal_kind(outcome: EntityImageOutcomeV3) -> GraphicImageInspectRefusalKindV3 {
        match outcome {
            EntityImageOutcomeV3::Complete(set) => {
                panic!("expected a refusal, got {} inspections", set.len())
            }
            EntityImageOutcomeV3::ReadRefused(refusal) => {
                panic!("source read refused: {refusal:?}")
            }
            EntityImageOutcomeV3::Refused(refusal) => refusal.kind(),
        }
    }

    /// `n116_01` — one real Entity inventory with five distinct original declarations across the
    /// whole EFS chain, inspected through the real confined reader and five real child workers.
    #[test]
    fn n116_01_mixed_five_formats_real_host() {
        let svg_bytes = rich_svg().into_bytes();
        let root = Root::new(
            "mixed",
            &[
                ("icon.png", &png()),
                ("logo.jpg", &jpeg()),
                ("banner.webp", &webp()),
                ("scene.gif", &gif_fixture()),
                ("mark.svg", &svg_bytes),
            ],
        );
        let inventory = inventory_of(&[
            (TestOwnerV1::Entity, "favicon", "icon.png", "public"),
            (TestOwnerV1::Facet, "logo", "logo.jpg", "public"),
            (TestOwnerV1::Site, "banner", "banner.webp", "internal"),
            (TestOwnerV1::Page, "background", "scene.gif", "public"),
            (
                TestOwnerV1::Representation,
                "social-preview",
                "mark.svg",
                "public",
            ),
        ]);

        let set = complete(inspect_entity_graphic_images_v3(
            &root.capability(),
            &inventory,
            std::path::Path::new(BIN),
        ));

        assert_eq!(set.len(), 5, "five distinct obligations are inspected");
        let formats: Vec<GraphicImageFormatV3> =
            set.inspections().iter().map(|item| item.format()).collect();
        assert_eq!(
            formats,
            vec![
                GraphicImageFormatV3::Png,
                GraphicImageFormatV3::Jpeg,
                GraphicImageFormatV3::WebP,
                GraphicImageFormatV3::Gif,
                GraphicImageFormatV3::Svg,
            ],
            "the accepted order is the declared order"
        );

        let mut pids: Vec<u32> = Vec::new();
        for item in set.inspections() {
            assert!(item.worker_pid() > 0, "a real child was observed");
            assert_ne!(
                item.source_digest().as_bytes(),
                &[0u8; 32],
                "a real source digest exists"
            );
            pids.push(item.worker_pid());
        }
        let mut unique = pids.clone();
        unique.sort_unstable();
        unique.dedup();
        assert_eq!(unique.len(), 5, "five distinct worker processes: {pids:?}");

        let animation = set
            .inspections()
            .iter()
            .find_map(|item| item.animation())
            .expect("the GIF obligation yields a timeline");
        assert_eq!(animation.frames.len(), 3, "three authored frames");
        assert_eq!(animation.width, 1);
        assert_eq!(animation.height, 1);

        let vector = set
            .inspections()
            .iter()
            .find_map(|item| item.vector())
            .expect("the SVG obligation yields a vector result");
        assert_eq!(vector.width, 24);
        assert_eq!(vector.height, 24);
        assert!(!vector.canonical().is_empty());
        assert_eq!(
            *set.inspections()[4].source_digest(),
            vector.source_digest,
            "the vector source digest is the one the adapter read"
        );
        let canonical = String::from_utf8_lossy(vector.canonical()).to_string();
        assert!(canonical.starts_with("<svg"), "canonical SVG text");
        assert!(!canonical.contains("<text"), "no text element survives");
        assert_eq!(set.inspections()[4].mime(), IMAGE_MIME_SVG_V3);
        assert!(!set.inspections()[4].is_raster(), "not a raster substitute");

        assert!(!set.grants_publication_authority());
        assert!(!set.proves_whole_publication());

        // AVIF is named by the grammar and refused before any worker exists.
        let avif_root = Root::new(
            "avif",
            &[(
                "mark.avif",
                b"\x00\x00\x00\x20ftypavif\x00\x00\x00\x00avifmif1miafMA1B".as_slice(),
            )],
        );
        let avif_inventory = inventory_of(&[(TestOwnerV1::Entity, "logo", "mark.avif", "public")]);
        assert_eq!(
            refusal_kind(inspect_entity_graphic_images_v3(
                &avif_root.capability(),
                &avif_inventory,
                std::path::Path::new(BIN)
            )),
            GraphicImageInspectRefusalKindV3::AvifUnsupported
        );

        println!(
            "n116_evidence case=n116_01 formats=png,jpeg,webp,gif,svg pids={pids:?} \
             gif_frames={} vector_canonical_sha256={} avif=avif-unsupported-v3 \
             partial=false publication=false",
            animation.frames.len(),
            hex(&vector.output_digest)
        );
    }
    /// `n116_02` — every mandated static-vector construct plus the pinned Foundation icon, with an
    /// independently authored canonical-sha oracle and cross-host equality.
    #[test]
    fn n116_02_safe_vector_semantics_and_cross_host_digest() {
        let rich = rich_svg().into_bytes();
        let observation = inspect_svg_bytes_with_executable_v1(std::path::Path::new(BIN), &rich)
            .expect("the rich static vector is accepted");
        assert_eq!(observation.width(), 24);
        assert_eq!(observation.height(), 24);
        assert!(
            observation.references() >= 3,
            "linear, radial and clip refs"
        );
        assert!(
            observation.path_commands() >= 12,
            "M/L/H/V/C/S/Q/T/A/Z cover"
        );
        assert!(observation.filters() >= 1);
        assert!(observation.limits_verified() && observation.cleanup_verified());

        let canonical = String::from_utf8_lossy(observation.canonical()).to_string();
        // Independently authored semantic oracle: nothing active, ambient or raster survives.
        for forbidden in [
            "<text",
            "font-family",
            "font-face",
            "<style",
            "<script",
            "foreignObject",
            "<image",
            "xlink:href=\"http",
            "url(http",
            "onclick",
            "<animate",
            "<?xml",
            "<!DOCTYPE",
            "<!--",
        ] {
            assert!(
                !canonical.contains(forbidden),
                "canonical output must not contain {forbidden:?}"
            );
        }
        assert!(canonical.starts_with("<svg"));
        assert!(canonical.ends_with("</svg>") || canonical.ends_with("</svg>\n"));
        assert!(!canonical.contains('\r'), "canonical form is LF only");
        // `currentColor` is resolved to a fixed literal, never left ambient.
        assert!(!canonical.contains("currentColor"));
        // The geometric text became real path sub-paths.
        let paths = canonical.matches("<path").count();
        assert!(paths >= 2, "glyph outlines are emitted as paths: {paths}");

        // Idempotence: re-sanitising the canonical bytes reproduces them exactly.
        let second = inspect_svg_bytes_with_executable_v1(
            std::path::Path::new(BIN),
            observation.canonical(),
        )
        .expect("canonical bytes are themselves accepted");
        assert_eq!(
            second.canonical(),
            observation.canonical(),
            "canonical serialisation is idempotent"
        );
        assert_eq!(
            hex(second.output_digest()),
            hex(observation.output_digest())
        );

        // The pinned Foundation donor icon, normalised to its pinned LF bytes before hashing.
        let icon = icon_home_lf();
        assert!(icon.windows(4).all(|w| w != b"\r\n"), "test source is LF");
        let icon_observation =
            inspect_svg_bytes_with_executable_v1(std::path::Path::new(BIN), &icon)
                .expect("the pinned Foundation icon is accepted");
        assert_eq!(icon_observation.width(), 24);
        assert_eq!(icon_observation.height(), 24);

        // The independently pinned oracle: one literal per document, identical on both hosts.
        let expected_rich = if red_oracle() {
            "00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff"
        } else {
            RICH_CANONICAL_SHA256
        };
        assert_eq!(
            hex(observation.output_digest()),
            expected_rich,
            "rich canonical sha-256 must equal the independently pinned reference"
        );
        assert_eq!(
            hex(icon_observation.output_digest()),
            ICON_CANONICAL_SHA256,
            "pinned Foundation icon canonical sha-256"
        );
        println!(
            "n116_evidence case=n116_02 rich_canonical_sha256={} icon_canonical_sha256={} \
             elements={} references={} path_commands={} filters={} oracle={}",
            hex(observation.output_digest()),
            hex(icon_observation.output_digest()),
            observation.elements(),
            observation.references(),
            observation.path_commands(),
            observation.filters(),
            if red_oracle() { "RED" } else { "GREEN" }
        );
    }
    /// `n116_03` — active SVG, hidden obligations and later corrupt assets refuse the whole set.
    #[test]
    fn n116_03_active_svg_and_hidden_obligation_refuse_whole() {
        let mut refusals: Vec<(&str, GraphicImageInspectRefusalKindV3)> = Vec::new();

        // Every hostile document refuses whole, one at a time.
        for name in [
            "script", "dtd", "pi", "comment", "foreign", "event", "css", "remote", "cycle", "dup",
            "wrongns", "text", "animate",
        ] {
            let bad = hostile(name);
            let root = Root::new(name, &[("mark.svg", &bad)]);
            let inventory = inventory_of(&[(TestOwnerV1::Entity, "logo", "mark.svg", "public")]);
            let kind = refusal_kind(inspect_entity_graphic_images_v3(
                &root.capability(),
                &inventory,
                std::path::Path::new(BIN),
            ));
            refusals.push((name, kind));
            assert_ne!(
                kind,
                GraphicImageInspectRefusalKindV3::UnsupportedFormat,
                "{name} is admitted by the grammar and refused by inspection"
            );
        }

        // A later corrupt SVG discards every earlier success: four good, one bad, no partial set.
        let good = rich_svg().into_bytes();
        let late = Root::new(
            "late",
            &[
                ("icon.png", &png()),
                ("logo.jpg", &jpeg()),
                ("banner.webp", &webp()),
                ("scene.gif", &gif_fixture()),
                ("z-mark.svg", &hostile("script")),
            ],
        );
        let late_inventory = inventory_of(&[
            (TestOwnerV1::Entity, "favicon", "icon.png", "public"),
            (TestOwnerV1::Facet, "logo", "logo.jpg", "public"),
            (TestOwnerV1::Site, "banner", "banner.webp", "public"),
            (TestOwnerV1::Page, "background", "scene.gif", "public"),
            (
                TestOwnerV1::Representation,
                "social-preview",
                "z-mark.svg",
                "public",
            ),
        ]);
        let late_outcome = inspect_entity_graphic_images_v3(
            &late.capability(),
            &late_inventory,
            std::path::Path::new(BIN),
        );
        assert!(
            matches!(late_outcome, EntityImageOutcomeV3::Refused(_)),
            "one bad later obligation refuses the whole candidate"
        );
        let late_kind = refusal_kind(late_outcome);
        refusals.push(("late", late_kind));

        // A suppressed (hidden) declaration cannot escape validation by being unpublishable.
        let hidden = Root::new(
            "hidden",
            &[("mark.svg", &hostile("dtd")), ("logo.jpg", &jpeg())],
        );
        let hidden_inventory = suppressed_inventory_of(&[
            (TestOwnerV1::Entity, "logo", "logo.jpg", "public"),
            (
                TestOwnerV1::Representation,
                "social-preview",
                "mark.svg",
                "internal",
            ),
        ]);
        let hidden_outcome = inspect_entity_graphic_images_v3(
            &hidden.capability(),
            &hidden_inventory,
            std::path::Path::new(BIN),
        );
        assert!(
            matches!(hidden_outcome, EntityImageOutcomeV3::Refused(_)),
            "a suppressed declaration is still inspected and still refuses"
        );

        // Registered role policy. The favicon role is PNG-only in the **authored declaration
        // validator**, which is stricter than the inspection stage: an `.svg` or `.gif` favicon is
        // refused before a source read or a worker can exist. Measured with the real validator, so a
        // future widening of the role cannot pass this case silently.
        let svg_bytes = good.clone();
        for (tag, reference) in [("icon.svg", "icon.svg"), ("icon.gif", "icon.gif")] {
            assert!(
                test_inventory_v1(&[(TestOwnerV1::Entity, "favicon", reference, "public")])
                    .is_none(),
                "{tag} must be refused by the registered PNG-only favicon role at declaration"
            );
        }
        // The same declared bytes under a non-favicon role reach the inspector, which accepts the
        // SVG — proving the refusal above is the role, not the format gate.
        let role_root = Root::new("roles", &[("mark.svg", &svg_bytes)]);
        let role_set = complete(inspect_entity_graphic_images_v3(
            &role_root.capability(),
            &inventory_of(&[(TestOwnerV1::Entity, "logo", "mark.svg", "public")]),
            std::path::Path::new(BIN),
        ));
        assert_eq!(
            role_set.len(),
            1,
            "an .svg under a non-favicon role inspects"
        );
        assert!(role_set.inspections()[0].vector().is_some());

        // A missing file is a real source-read refusal, never a silent skip.
        let missing_root = Root::new("missing", &[("logo.jpg", &jpeg())]);
        let missing = inspect_entity_graphic_images_v3(
            &missing_root.capability(),
            &inventory_of(&[
                (TestOwnerV1::Entity, "logo", "logo.jpg", "public"),
                (TestOwnerV1::Facet, "banner", "absent.png", "public"),
            ]),
            std::path::Path::new(BIN),
        );
        assert!(
            matches!(missing, EntityImageOutcomeV3::ReadRefused(_)),
            "an absent declared source refuses the read"
        );

        assert!(refusals.len() >= 14);
        println!(
            "n116_evidence case=n116_03 refusals={} late_refusal={} hidden_refused=true \
             favicon_svg=declaration-refused favicon_gif=declaration-refused svg_nonfavicon_role=inspected \
             missing=read-refused partial=false fallback=none",
            refusals.len(),
            late_kind.token()
        );
    }
    /// `n116_04` — genuine worker forgery refused, plus the deliberate-RED oracle.
    #[test]
    fn n116_04_svg_forgery_and_deliberate_red_oracle() {
        let source = rich_svg().into_bytes();

        // A child that lies about the source digest cannot produce a result.
        assert_eq!(
            inspect_svg_bytes_with_test_mode_v1(
                std::path::Path::new(BIN),
                &source,
                SvgWorkerTestModeV1::WrongSource
            )
            .expect_err("a forged source digest refuses"),
            SvgWorkerRefusalV1::SourceDigestMismatch
        );

        // A child that lies about its canonical output digest cannot produce a result.
        assert_eq!(
            inspect_svg_bytes_with_test_mode_v1(
                std::path::Path::new(BIN),
                &source,
                SvgWorkerTestModeV1::WrongOutput
            )
            .expect_err("a forged output digest refuses"),
            SvgWorkerRefusalV1::OutputDigestMismatch
        );

        // The genuine path still succeeds on the same bytes.
        let genuine = inspect_svg_bytes_with_executable_v1(std::path::Path::new(BIN), &source)
            .expect("the genuine worker path still succeeds");

        // Deliberate RED: the switch changes only this expectation, never production code.
        let expected = if red_oracle() {
            "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
        } else {
            RICH_CANONICAL_SHA256
        };
        assert_eq!(
            hex(genuine.output_digest()),
            expected,
            "canonical sha-256 oracle (RED when SOLO_N116_RED_ORACLE=wrong)"
        );

        println!(
            "n116_evidence case=n116_04 forged_source=source-digest-mismatch \
             forged_output=output-digest-mismatch genuine_canonical_sha256={} oracle={}",
            hex(genuine.output_digest()),
            if red_oracle() { "RED" } else { "GREEN" }
        );
    }
}

/// `SOLO-WO-118 R06`: the real confined AVIF coded-plane worker proven on both native hosts.
///
/// Every positive starts the actual compiled `solo-cli` binary through the real confined
/// supervisor, acquires bytes through the real `ContentRootCapability` reader and validates the
/// result with the inward parent verifier. No positive is fabricated and no fixture digest is used
/// to predict worker output.
#[cfg(test)]
mod n121 {
    use std::cell::RefCell;
    use std::path::PathBuf;

    use serde_json::Value;
    use solo_adapters::{
        ConfinedAvifInspectAdapterV1, MediaWorkerProtocolRefusalKindV1, TestOwnerV1,
        parse_media_worker_envelope_v1, test_inventory_v1,
    };
    use solo_application::{
        GraphicAssetReadSetOutcomeV1, GraphicAssetSourceSetV1, MediaWorkerBoundaryOutcomeV1,
        MediaWorkerColorModelV1, MediaWorkerDigestV1, MediaWorkerEnvelopeV1, MediaWorkerExitV1,
        MediaWorkerFormatV1, MediaWorkerLimitsV1, MediaWorkerMimeV1, MediaWorkerObservationV1,
        MediaWorkerOrientationV1, MediaWorkerPlaneClaimV1, MediaWorkerPortV1,
        MediaWorkerRefusalKindV1, MediaWorkerRequestV1, MediaWorkerRunV1, MediaWorkerStatusV1,
        inspect_media_worker_source_set_v1, read_graphic_asset_bytes_v1,
        sha256_media_worker_bytes_v1,
    };

    use super::n116::{Root, inventory_of, suppressed_inventory_of};
    use super::{BIN, run};

    /// The four authentic stills, as `(fixture, width, height, depth)`.
    const POSITIVES: [(&str, u32, u32, u8); 4] = [
        ("white_1x1.avif", 1, 1, 8),
        ("fox-profile0.8bpc.yuv420.avif", 1204, 800, 8),
        ("fox-profile0.10bpc.yuv420.avif", 1204, 800, 10),
        ("fox-profile2.12bpc.yuv420.avif", 1204, 800, 12),
    ];

    fn fixture_path(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/avif")
            .join(name)
    }

    fn fixture_bytes(name: &str) -> Vec<u8> {
        std::fs::read(fixture_path(name)).expect("pinned avif fixture present")
    }

    /// The unchanged independent R03 manifest: a fixed test-only path, opened read-only.
    fn goldens() -> Value {
        let text = std::fs::read_to_string(fixture_path("reference-goldens.json"))
            .expect("pinned oracle manifest present");
        serde_json::from_str(&text).expect("pinned oracle manifest parses")
    }

    fn hex_digest(hex: &str) -> MediaWorkerDigestV1 {
        MediaWorkerDigestV1::parse_lower_hex(hex).expect("pinned digest is canonical lowercase hex")
    }

    fn complete(outcome: GraphicAssetReadSetOutcomeV1) -> GraphicAssetSourceSetV1 {
        match outcome {
            GraphicAssetReadSetOutcomeV1::Complete(set) => set,
            GraphicAssetReadSetOutcomeV1::Refused(refusal) => {
                panic!("real source acquisition refused: {refusal:?}")
            }
        }
    }

    fn checked(
        outcome: MediaWorkerBoundaryOutcomeV1,
    ) -> Vec<solo_application::MediaWorkerCheckedEnvelopeV1> {
        match outcome {
            MediaWorkerBoundaryOutcomeV1::EnvelopeChecked(checked) => checked,
            MediaWorkerBoundaryOutcomeV1::Refused(refusal) => {
                panic!("the inward boundary refused: {refusal:?}")
            }
        }
    }

    fn refusal_kind(
        set: &GraphicAssetSourceSetV1,
        port: &impl MediaWorkerPortV1,
    ) -> MediaWorkerRefusalKindV1 {
        match inspect_media_worker_source_set_v1(set, port) {
            MediaWorkerBoundaryOutcomeV1::Refused(refusal) => refusal.kind(),
            MediaWorkerBoundaryOutcomeV1::EnvelopeChecked(checked) => panic!(
                "the boundary accepted a hostile claim set: {} checked",
                checked.len()
            ),
        }
    }

    /// A test-only port that forwards every call to the real adapter and retains the observation
    /// the adapter actually returned. It fabricates nothing and launches nothing itself.
    struct RecordingPort<'a> {
        inner: &'a ConfinedAvifInspectAdapterV1,
        seen: RefCell<Vec<MediaWorkerObservationV1>>,
    }

    impl<'a> RecordingPort<'a> {
        fn new(inner: &'a ConfinedAvifInspectAdapterV1) -> Self {
            Self {
                inner,
                seen: RefCell::new(Vec::new()),
            }
        }
    }

    impl MediaWorkerPortV1 for RecordingPort<'_> {
        fn inspect_avif(&self, request: &MediaWorkerRequestV1<'_>) -> MediaWorkerRunV1 {
            let run = self.inner.inspect_avif(request);
            if let MediaWorkerRunV1::Observed(observation) = &run {
                self.seen.borrow_mut().push(observation.clone());
            }
            run
        }
    }

    /// A test-only port replaying one constructed claim; it can only prove inward refusal rules.
    struct StaticPort(MediaWorkerObservationV1);

    impl MediaWorkerPortV1 for StaticPort {
        fn inspect_avif(&self, _request: &MediaWorkerRequestV1<'_>) -> MediaWorkerRunV1 {
            MediaWorkerRunV1::Observed(self.0.clone())
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn rebuild(
        source_length: u64,
        source_sha256: MediaWorkerDigestV1,
        width: u32,
        height: u32,
        bit_depth: u8,
        planes: Vec<MediaWorkerPlaneClaimV1>,
        output_sha256: MediaWorkerDigestV1,
    ) -> MediaWorkerEnvelopeV1 {
        MediaWorkerEnvelopeV1::new(
            1,
            MediaWorkerStatusV1::Decoded,
            MediaWorkerFormatV1::Avif,
            MediaWorkerMimeV1::ImageAvif,
            source_length,
            source_sha256,
            width,
            height,
            bit_depth,
            MediaWorkerColorModelV1::Yuv,
            false,
            false,
            false,
            MediaWorkerOrientationV1::Unverified,
            planes,
            output_sha256,
        )
    }

    fn claim(
        ordinal: u32,
        stride: u64,
        rows: u64,
        length: u64,
        sha256: MediaWorkerDigestV1,
    ) -> MediaWorkerPlaneClaimV1 {
        MediaWorkerPlaneClaimV1::new(ordinal, stride, rows, length, sha256)
    }

    fn observation(
        limits_verified: bool,
        exit: MediaWorkerExitV1,
        wall_deadline_met: bool,
        descendants_reaped: bool,
        envelope: MediaWorkerEnvelopeV1,
        planes: Vec<Vec<u8>>,
    ) -> MediaWorkerObservationV1 {
        MediaWorkerObservationV1::new(
            limits_verified,
            exit,
            wall_deadline_met,
            descendants_reaped,
            !descendants_reaped,
            MediaWorkerLimitsV1::reference(),
            envelope,
            planes,
        )
    }

    #[test]
    fn n121_01_four_coded_plane_goldens_cross_worker_and_parent() {
        // Four distinct owners, four distinct registered roles, four distinct authentic stills.
        let declared = [
            (TestOwnerV1::Entity, "logo", POSITIVES[0].0, "public"),
            (TestOwnerV1::Facet, "banner", POSITIVES[1].0, "public"),
            (TestOwnerV1::Site, "background", POSITIVES[2].0, "public"),
            (
                TestOwnerV1::Page,
                "social-preview",
                POSITIVES[3].0,
                "public",
            ),
        ];
        let assets: Vec<(&str, Vec<u8>)> = POSITIVES
            .iter()
            .map(|(fixture, _, _, _)| (*fixture, fixture_bytes(fixture)))
            .collect();
        let asset_refs: Vec<(&str, &[u8])> = assets
            .iter()
            .map(|(name, bytes)| (*name, bytes.as_slice()))
            .collect();
        let root = Root::new("n121-four", &asset_refs);
        let inventory = inventory_of(&declared);

        let set = complete(read_graphic_asset_bytes_v1(&inventory, &root.capability()));
        assert_eq!(set.len(), 4, "four authored obligations are acquired");
        for (index, (fixture, _, _, _)) in POSITIVES.iter().enumerate() {
            assert_eq!(
                u64::try_from(set.evidence()[index].bytes().len()).expect("length"),
                u64::try_from(fixture_bytes(fixture).len()).expect("fixture length"),
                "{fixture}: the acquired bytes are the authored bytes"
            );
        }
        assert!(
            !set.grants_publication_authority() && !set.validates_media(),
            "source acquisition neither publishes nor validates"
        );

        let adapter = ConfinedAvifInspectAdapterV1::new(PathBuf::from(BIN));
        let recording = RecordingPort::new(&adapter);
        let records = checked(inspect_media_worker_source_set_v1(&set, &recording));
        let seen = recording.seen.borrow().clone();

        assert_eq!(seen.len(), 4, "four real confined runs were observed");
        assert_eq!(records.len(), 4, "four parent-checked records");
        let manifest = goldens();
        let mut plane_total = 0usize;
        for (index, (fixture, width, height, depth)) in POSITIVES.iter().enumerate() {
            let observed = &seen[index];
            assert_eq!(
                observed.exit(),
                MediaWorkerExitV1::Normal,
                "{fixture}: the child must exit normally"
            );
            assert!(observed.limits_verified(), "{fixture}: limits read back");
            assert!(
                observed.wall_deadline_met(),
                "{fixture}: the wall deadline was met"
            );
            assert!(
                observed.descendants_reaped(),
                "{fixture}: the child is reaped"
            );
            assert!(!observed.leak_observed(), "{fixture}: no leak observed");

            let envelope = observed.envelope();
            assert_eq!(envelope.format(), MediaWorkerFormatV1::Avif);
            assert_eq!(
                envelope.orientation(),
                MediaWorkerOrientationV1::Unverified,
                "{fixture}: coded planes only, never a display-orientation claim"
            );
            assert_eq!(
                (envelope.width(), envelope.height(), envelope.bit_depth()),
                (*width, *height, *depth),
                "{fixture}: geometry and native depth"
            );
            assert_eq!(envelope.planes().len(), 3, "{fixture}: Y/U/V only");
            assert_eq!(observed.planes().len(), 3, "{fixture}: three owned buffers");
            plane_total += 3;

            let pinned = &manifest["fixtures"][*fixture];
            assert_eq!(
                envelope.source_sha256(),
                hex_digest(pinned["source_sha256"].as_str().expect("source hex")),
                "{fixture}: source identity"
            );
            for (plane_index, plane_claim) in envelope.planes().iter().enumerate() {
                let pinned_plane = &pinned["planes"][plane_index];
                assert_eq!(
                    plane_claim.stride(),
                    pinned_plane["stride"].as_u64().expect("stride"),
                    "{fixture}: plane {plane_index} canonical stride"
                );
                assert_eq!(
                    plane_claim.rows(),
                    pinned_plane["rows"].as_u64().expect("rows"),
                    "{fixture}: plane {plane_index} canonical rows"
                );
                assert_eq!(
                    plane_claim.length(),
                    u64::try_from(observed.planes()[plane_index].len()).expect("length"),
                    "{fixture}: plane {plane_index} received length"
                );
                let pinned_hex = pinned_plane["sha256"].as_str().expect("plane hex");
                assert_eq!(
                    plane_claim.sha256(),
                    hex_digest(pinned_hex),
                    "{fixture}: plane {plane_index} claim against the independent golden"
                );
                assert_eq!(
                    sha256_media_worker_bytes_v1(&observed.planes()[plane_index]),
                    hex_digest(pinned_hex),
                    "{fixture}: plane {plane_index} received bytes against the independent golden"
                );
            }
            assert_eq!(
                envelope.output_sha256(),
                hex_digest(pinned["aggregate_sha256"].as_str().expect("aggregate hex")),
                "{fixture}: aggregate against the independent golden"
            );
        }

        for (index, record) in records.iter().enumerate() {
            let (fixture, width, height, depth) = POSITIVES[index];
            assert_eq!(
                record.plane_count(),
                3,
                "{fixture}: the parent checked three planes"
            );
            assert_eq!(
                (record.width(), record.height(), record.bit_depth()),
                (width, height, depth),
                "{fixture}: the parent checked geometry and depth"
            );
            assert_eq!(
                record.source_length(),
                u64::try_from(set.evidence()[index].bytes().len()).expect("length"),
                "{fixture}: the parent checked the source length"
            );
            assert!(
                !record.grants_publication_authority(),
                "{fixture}: a coded-plane check never authorises publication"
            );
            assert!(
                !record.proves_media_correctness(),
                "{fixture}: a coded-plane check never proves the media"
            );
        }

        let pids = adapter.observed_pids_v1();
        assert_eq!(pids.len(), 4, "one real child per obligation: {pids:?}");
        assert!(
            pids.iter().all(|pid| *pid > 0),
            "every launch has a real pid"
        );
        let mut unique = pids.clone();
        unique.sort_unstable();
        unique.dedup();
        assert_eq!(unique.len(), 4, "four distinct child processes: {pids:?}");

        println!(
            "n121_evidence case=01 pids={pids:?} distinct_pids=4 fixtures=4 planes={plane_total} \
             depths=8,10,12 orientation=unverified publication=false records=4"
        );
    }

    #[test]
    fn n121_02_real_worker_and_hostile_claim_refusals() {
        let adapter = ConfinedAvifInspectAdapterV1::new(PathBuf::from(BIN));
        let fox = fixture_bytes(POSITIVES[1].0);
        let mut refused: Vec<&str> = Vec::new();

        // (A) The parent-side source gate refuses before any launch.
        assert!(
            MediaWorkerRequestV1::for_source(&[]).is_err(),
            "zero source bytes refuse before spawn"
        );
        refused.push("source-empty-before-launch");
        let oversize = vec![0u8; (20 * 1024 * 1024) + 1];
        assert!(
            MediaWorkerRequestV1::for_source(&oversize).is_err(),
            "an over-20-MiB source refuses before spawn"
        );
        refused.push("source-oversize-before-launch");

        // (B) Authored property grammar: the favicon role is PNG-only, so an AVIF favicon is
        // refused while the reference grammar itself admits the `.avif` extension.
        assert!(
            test_inventory_v1(&[(TestOwnerV1::Entity, "favicon", POSITIVES[0].0, "public")])
                .is_none(),
            "an AVIF favicon is refused by the authored property grammar"
        );
        refused.push("graphic-favicon-not-png");

        // (C) Real child preflight refusals, each through the real confined worker.
        let hostile: [(&str, Vec<u8>); 5] = [
            ("bad-magic.avif", b"this is not an avif still".to_vec()),
            ("truncated.avif", fox[..64].to_vec()),
            (
                "alpha-irot",
                fixture_bytes("abc_color_irot_alpha_irot.avif"),
            ),
            (
                "alpha-noirot",
                fixture_bytes("abc_color_irot_alpha_NOirot.avif"),
            ),
            (
                "clap-irot-imir",
                fixture_bytes("clap_irot_imir_non_essential.avif"),
            ),
        ];
        let hostile_assets: Vec<(&str, &[u8])> = hostile
            .iter()
            .map(|(name, bytes)| (*name, bytes.as_slice()))
            .collect();
        let hostile_root = Root::new("n121-hostile", &hostile_assets);
        for (name, _) in &hostile {
            let inventory = inventory_of(&[(TestOwnerV1::Entity, "logo", name, "internal")]);
            let set = complete(read_graphic_asset_bytes_v1(
                &inventory,
                &hostile_root.capability(),
            ));
            let request =
                MediaWorkerRequestV1::for_source(set.evidence()[0].bytes()).expect("bounded");
            match adapter.inspect_avif(&request) {
                MediaWorkerRunV1::Refused(_) => refused.push(name),
                MediaWorkerRunV1::Observed(observation) => panic!(
                    "{name}: the real worker accepted an out-of-scope source: exit={:?} planes={}",
                    observation.exit(),
                    observation.planes().len()
                ),
            }
        }

        // (D) A complete earlier success is discarded when a later obligation is bad. The second
        // obligation is a representation-level `internal` declaration.
        let mixed_assets = [
            (POSITIVES[0].0, fixture_bytes(POSITIVES[0].0)),
            ("truncated.avif", fox[..64].to_vec()),
        ];
        let mixed_refs: Vec<(&str, &[u8])> = mixed_assets
            .iter()
            .map(|(name, bytes)| (*name, bytes.as_slice()))
            .collect();
        let mixed_root = Root::new("n121-mixed", &mixed_refs);
        let mixed_inventory = suppressed_inventory_of(&[
            (TestOwnerV1::Entity, "logo", POSITIVES[0].0, "public"),
            (
                TestOwnerV1::Representation,
                "social-preview",
                "truncated.avif",
                "internal",
            ),
        ]);
        let mixed = complete(read_graphic_asset_bytes_v1(
            &mixed_inventory,
            &mixed_root.capability(),
        ));
        assert_eq!(mixed.len(), 2, "both authored obligations were acquired");
        match inspect_media_worker_source_set_v1(&mixed, &adapter) {
            MediaWorkerBoundaryOutcomeV1::Refused(_) => {
                refused.push("whole-set-discarded-by-representation")
            }
            MediaWorkerBoundaryOutcomeV1::EnvelopeChecked(checked) => panic!(
                "the boundary returned a partial success: {} checked",
                checked.len()
            ),
        }

        // (E) A Platform-namespaced obligation is never acquired by this Entity path.
        match test_inventory_v1(&[(TestOwnerV1::Entity, "logo", "platform/hero.avif", "public")]) {
            None => refused.push("platform-namespace-refused-by-grammar"),
            Some(inventory) => {
                match read_graphic_asset_bytes_v1(&inventory, &mixed_root.capability()) {
                    GraphicAssetReadSetOutcomeV1::Refused(_) => {
                        refused.push("platform-namespace-refused-at-acquisition")
                    }
                    GraphicAssetReadSetOutcomeV1::Complete(_) => {
                        panic!("a Platform-namespaced obligation was acquired")
                    }
                }
            }
        }

        // (F) A direct invocation without the approved OS confinement refuses before parsing.
        let direct = run(&[
            "--internal-image-worker-v1",
            "avif",
            &sha256_media_worker_bytes_v1(&fox).to_hex(),
        ]);
        assert_ne!(
            direct.exit_code,
            Some(0),
            "an unconfined child must not decode"
        );
        assert!(
            direct.stdout.is_empty(),
            "an unconfined child writes nothing"
        );
        refused.push("unconfined-direct-invocation");

        // (G) Counterfeit V1 claims against a genuinely acquired single-source set.
        let single_assets = [(POSITIVES[0].0, fixture_bytes(POSITIVES[0].0))];
        let single_refs: Vec<(&str, &[u8])> = single_assets
            .iter()
            .map(|(name, bytes)| (*name, bytes.as_slice()))
            .collect();
        let single_root = Root::new("n121-single", &single_refs);
        let single_inventory =
            inventory_of(&[(TestOwnerV1::Entity, "logo", POSITIVES[0].0, "public")]);
        let single = complete(read_graphic_asset_bytes_v1(
            &single_inventory,
            &single_root.capability(),
        ));
        let request =
            MediaWorkerRequestV1::for_source(single.evidence()[0].bytes()).expect("bounded");
        let genuine = match adapter.inspect_avif(&request) {
            MediaWorkerRunV1::Observed(observation) => observation,
            MediaWorkerRunV1::Refused(kind) => panic!("the genuine source was refused: {kind:?}"),
        };
        let envelope = genuine.envelope();
        let source_length = envelope.source_length();
        let source_sha = envelope.source_sha256();
        let output_sha = envelope.output_sha256();
        let claims = envelope.planes().to_vec();
        let planes = genuine.planes().to_vec();
        let expect =
            |label: &str, forged: MediaWorkerObservationV1, wanted: MediaWorkerRefusalKindV1| {
                let kind = refusal_kind(&single, &StaticPort(forged));
                assert_eq!(kind, wanted, "{label}: typed inward refusal");
            };
        let forged_digest = |hex: [u8; 32]| MediaWorkerDigestV1::from_bytes(hex);
        let honest = |planes: Vec<Vec<u8>>| {
            observation(
                true,
                MediaWorkerExitV1::Normal,
                true,
                true,
                rebuild(
                    source_length,
                    source_sha,
                    1,
                    1,
                    8,
                    claims.clone(),
                    output_sha,
                ),
                planes,
            )
        };

        expect(
            "forged-source-digest",
            observation(
                true,
                MediaWorkerExitV1::Normal,
                true,
                true,
                rebuild(
                    source_length,
                    forged_digest([9u8; 32]),
                    1,
                    1,
                    8,
                    claims.clone(),
                    output_sha,
                ),
                planes.clone(),
            ),
            MediaWorkerRefusalKindV1::WorkerSourceMismatch,
        );
        expect(
            "forged-aggregate-digest",
            observation(
                true,
                MediaWorkerExitV1::Normal,
                true,
                true,
                rebuild(
                    source_length,
                    source_sha,
                    1,
                    1,
                    8,
                    claims.clone(),
                    forged_digest([7u8; 32]),
                ),
                planes.clone(),
            ),
            MediaWorkerRefusalKindV1::WorkerDigestMismatch,
        );
        let mut wrong_plane_sha = claims.clone();
        wrong_plane_sha[1] = claim(1, 1, 1, 1, forged_digest([5u8; 32]));
        expect(
            "forged-plane-digest",
            observation(
                true,
                MediaWorkerExitV1::Normal,
                true,
                true,
                rebuild(
                    source_length,
                    source_sha,
                    1,
                    1,
                    8,
                    wrong_plane_sha,
                    output_sha,
                ),
                planes.clone(),
            ),
            MediaWorkerRefusalKindV1::WorkerDigestMismatch,
        );
        let mut swapped = claims.clone();
        swapped.swap(1, 2);
        expect(
            "swapped-uv-ordinals",
            observation(
                true,
                MediaWorkerExitV1::Normal,
                true,
                true,
                rebuild(source_length, source_sha, 1, 1, 8, swapped, output_sha),
                planes.clone(),
            ),
            MediaWorkerRefusalKindV1::WorkerPlaneInvalid,
        );
        expect(
            "wrong-coded-depth",
            observation(
                true,
                MediaWorkerExitV1::Normal,
                true,
                true,
                rebuild(
                    source_length,
                    source_sha,
                    1,
                    1,
                    9,
                    claims.clone(),
                    output_sha,
                ),
                planes.clone(),
            ),
            MediaWorkerRefusalKindV1::WorkerMediaUnverified,
        );
        let mut bad_stride = claims.clone();
        bad_stride[0] = claim(0, 2, 1, 1, claims[0].sha256());
        expect(
            "invalid-stride",
            observation(
                true,
                MediaWorkerExitV1::Normal,
                true,
                true,
                rebuild(source_length, source_sha, 1, 1, 8, bad_stride, output_sha),
                planes.clone(),
            ),
            MediaWorkerRefusalKindV1::WorkerPlaneInvalid,
        );
        let mut short = planes.clone();
        short[2].pop();
        expect(
            "short-payload",
            honest(short),
            MediaWorkerRefusalKindV1::WorkerPlaneInvalid,
        );
        let mut trailing = planes.clone();
        trailing[2].push(0);
        expect(
            "trailing-payload",
            honest(trailing),
            MediaWorkerRefusalKindV1::WorkerPlaneInvalid,
        );
        expect(
            "abnormal-exit",
            observation(
                true,
                MediaWorkerExitV1::Abnormal,
                true,
                true,
                rebuild(
                    source_length,
                    source_sha,
                    1,
                    1,
                    8,
                    claims.clone(),
                    output_sha,
                ),
                planes.clone(),
            ),
            MediaWorkerRefusalKindV1::WorkerExitAbnormal,
        );
        expect(
            "false-limits",
            observation(
                false,
                MediaWorkerExitV1::Normal,
                true,
                true,
                rebuild(
                    source_length,
                    source_sha,
                    1,
                    1,
                    8,
                    claims.clone(),
                    output_sha,
                ),
                planes.clone(),
            ),
            MediaWorkerRefusalKindV1::WorkerLimitUnverified,
        );
        expect(
            "false-cleanup",
            observation(
                true,
                MediaWorkerExitV1::Normal,
                true,
                false,
                rebuild(
                    source_length,
                    source_sha,
                    1,
                    1,
                    8,
                    claims.clone(),
                    output_sha,
                ),
                planes.clone(),
            ),
            MediaWorkerRefusalKindV1::WorkerDescendantsRemain,
        );
        expect(
            "wall-timeout",
            observation(
                true,
                MediaWorkerExitV1::Normal,
                false,
                true,
                rebuild(
                    source_length,
                    source_sha,
                    1,
                    1,
                    8,
                    claims.clone(),
                    output_sha,
                ),
                planes.clone(),
            ),
            MediaWorkerRefusalKindV1::WorkerWallTimeout,
        );

        // (H) The unchanged strict parser refuses missing, duplicate and unknown envelope keys.
        let header = format!(
            "{{\"version\":1,\"status\":\"decoded\",\"format\":\"avif\",\"mime\":\"image/avif\",\
             \"source_length\":{source_length},\"source_sha256\":\"{}\",\"width\":1,\"height\":1,\
             \"bit_depth\":8,\"color_model\":\"yuv\",\"has_alpha\":false,\"is_grid\":false,\
             \"is_sequence\":false,\"orientation\":\"unverified\",\"planes\":[\
             {{\"ordinal\":0,\"stride\":1,\"rows\":1,\"length\":1,\"sha256\":\"{}\"}},\
             {{\"ordinal\":1,\"stride\":1,\"rows\":1,\"length\":1,\"sha256\":\"{}\"}},\
             {{\"ordinal\":2,\"stride\":1,\"rows\":1,\"length\":1,\"sha256\":\"{}\"}}],\
             \"output_sha256\":\"{}\"}}",
            source_sha.to_hex(),
            claims[0].sha256().to_hex(),
            claims[1].sha256().to_hex(),
            claims[2].sha256().to_hex(),
            output_sha.to_hex(),
        );
        assert!(
            parse_media_worker_envelope_v1(header.as_bytes()).is_ok(),
            "the canonical header parses"
        );
        let mutations: [(&str, String); 3] = [
            (
                "missing-key",
                header.replacen(",\"orientation\":\"unverified\"", "", 1),
            ),
            (
                "duplicate-key",
                header.replacen("\"width\":1,", "\"width\":1,\"width\":1,", 1),
            ),
            (
                "unknown-key",
                header.replacen("\"version\":1,", "\"version\":1,\"extra\":1,", 1),
            ),
        ];
        let mut protocol_kinds: Vec<(&str, MediaWorkerProtocolRefusalKindV1)> = Vec::new();
        for (label, text) in mutations {
            match parse_media_worker_envelope_v1(text.as_bytes()) {
                Ok(_) => panic!("{label}: a mutated header parsed"),
                Err(refusal) => protocol_kinds.push((label, refusal.kind())),
            }
        }

        println!(
            "n121_evidence case=02 refused={} labels={refused:?} counterfeits=12 \
             counterfeits_refused=true protocol={protocol_kinds:?} whole_set_discarded=true \
             unconfined_decoded=false",
            refused.len()
        );
    }
}
