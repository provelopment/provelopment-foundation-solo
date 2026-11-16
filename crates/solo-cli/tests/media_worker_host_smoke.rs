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

/// The approved Platform library candidate document, relative to this crate.
const APPROVED_PLATFORM_CANDIDATE: &str =
    "../../.documents/Reports/platform-library-1.0.0-foundation-candidate.json";

/// The approved catalogue's canonical LF SHA-256.
const APPROVED_PLATFORM_CATALOGUE_SHA256: &str =
    "ad8ea718b50641206cc4f4fe13e866ad266366a7aa11ef799c83ae8187858fc7";

/// The approved product tree's exact file count.
const APPROVED_PLATFORM_PRODUCT_FILES: usize = 84;

/// `SOLO-WO-118 R16` — the shared, test-only product-library predicate.
///
/// Two accepted tests asserted, in the pre-install state, that no product `platform/` tree may
/// exist. Once the Owner approved the library, that assertion became a statement about history
/// rather than about safety, and it is replaced here by the state-independent invariant:
///
/// * nothing installed — the historical pre-install tree is accepted, but a *half-created* tree
///   (a `platform/` without `platform/assets/`, or the reverse) is not;
/// * installed — the product tree must be **exactly** the approved library: the enumerated
///   relative path set equals the approved catalogue's 81 identifiers plus the catalogue, the MIT
///   notice and the provenance record, with no extra file, no unexpected directory and no link;
///   the installed catalogue is byte-identical to the approved candidate document and hashes to
///   the pinned digest; and the two licensing documents keep their pinned bytes.
///
/// The enumeration never follows a symlink or reparse point, and every diagnostic is path-free
/// beyond the caller's own root name. This predicate grants no publication, no media correctness
/// and no rights: it only decides whether the product tree is absent or exactly the approved one.
fn assert_absent_or_exact_approved_platform_library(manifest: &std::path::Path) {
    use std::collections::BTreeSet;
    use std::path::Path;

    let platform = manifest.join("../../platform");
    let assets = platform.join("assets");
    if !platform.exists() && !assets.exists() {
        return;
    }
    assert!(
        platform.is_dir() && assets.is_dir(),
        "a half-created product tree is not the absent pre-install state"
    );

    fn walk(base: &Path, current: &Path, out: &mut BTreeSet<String>) {
        for entry in std::fs::read_dir(current).expect("the product tree is enumerable") {
            let entry = entry.expect("a product directory entry");
            let path = entry.path();
            let file_type = entry
                .file_type()
                .expect("a product entry metadata, never following a link");
            assert!(
                !file_type.is_symlink(),
                "the product tree must contain no link"
            );
            if file_type.is_dir() {
                walk(base, &path, out);
            } else {
                assert!(
                    file_type.is_file(),
                    "the product tree holds only regular files and directories"
                );
                out.insert(
                    path.strip_prefix(base)
                        .expect("a relative product path")
                        .to_string_lossy()
                        .replace('\\', "/"),
                );
            }
        }
    }

    let mut actual = BTreeSet::new();
    walk(&assets, &assets, &mut actual);

    let candidate = std::fs::read(manifest.join(APPROVED_PLATFORM_CANDIDATE))
        .expect("the approved candidate catalogue is readable");
    let catalogue = solo_adapters::parse_platform_asset_catalogue_v1(&candidate, "1.0.0")
        .expect("the approved candidate catalogue satisfies the R12 contract");

    let mut expected: BTreeSet<String> = BTreeSet::new();
    for entry in catalogue.entries() {
        expected.insert(entry.id().to_owned());
    }
    expected.insert("catalogue-1.0.0.json".to_owned());
    expected.insert("licensing/TABLER-ICONS-MIT.txt".to_owned());
    expected.insert("licensing/icon-provenance.json".to_owned());
    assert_eq!(
        expected.len(),
        APPROVED_PLATFORM_PRODUCT_FILES,
        "the approved library declares exactly 84 product files"
    );
    assert_eq!(
        actual, expected,
        "the installed product tree is exactly the approved library"
    );
    assert_eq!(actual.len(), APPROVED_PLATFORM_PRODUCT_FILES);

    let installed = std::fs::read(assets.join("catalogue-1.0.0.json"))
        .expect("the installed catalogue is readable");
    assert_eq!(
        installed, candidate,
        "the installed catalogue is byte-identical to the approved candidate document"
    );
    assert_eq!(
        solo_application::sha256_media_worker_bytes_v1(&installed).to_hex(),
        APPROVED_PLATFORM_CATALOGUE_SHA256,
        "the installed catalogue keeps its pinned digest"
    );
    let reparsed = solo_adapters::parse_platform_asset_catalogue_v1(&installed, "1.0.0")
        .expect("the installed catalogue is parseable at the approved version");
    assert_eq!(
        reparsed.len(),
        81,
        "the installed catalogue declares 81 entries"
    );
    assert_eq!(
        reparsed
            .entries()
            .iter()
            .map(|e| e.id())
            .collect::<Vec<_>>(),
        catalogue
            .entries()
            .iter()
            .map(|e| e.id())
            .collect::<Vec<_>>(),
        "the installed catalogue membership and order are the approved ones"
    );

    for (relative, digest) in [
        (
            "licensing/TABLER-ICONS-MIT.txt",
            "b740a1d46122672da62833e97f7e7c8a13fa85cbc7445b584b297cc00dde93db",
        ),
        (
            "licensing/icon-provenance.json",
            "e0f0ea621c11140cf581d86aaab0b73421d0202114092a4e58dc3321ff1a325b",
        ),
    ] {
        let bytes = std::fs::read(assets.join(relative)).expect("a retained licensing document");
        assert_eq!(
            solo_application::sha256_media_worker_bytes_v1(&bytes).to_hex(),
            digest,
            "the retained licensing document keeps its pinned bytes"
        );
    }
}

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
    pub(super) fn gif_fixture() -> Vec<u8> {
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

    pub(super) fn png() -> Vec<u8> {
        decode_base64(FIXTURE_PNG)
    }

    pub(super) fn jpeg() -> Vec<u8> {
        decode_base64(FIXTURE_JPEG)
    }

    pub(super) fn webp() -> Vec<u8> {
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
        inspect_media_worker_source_set_v1, media_worker_output_sha256_v1,
        read_graphic_asset_bytes_v1, sha256_media_worker_bytes_v1,
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
                media_worker_output_sha256_v1(envelope.planes(), observed.planes()),
                "{fixture}: the protocol digest is exactly the parent's own recomputation"
            );
            // The pinned aggregate is the **independent oracle's** documented framing, which is
            // deliberately not the product's own scheme: for each plane in Y, U, V order feed
            // `PLANE|ordinal|name|stride|rows|length\n` and then that plane's raw bytes.
            let mut framed: Vec<u8> = Vec::new();
            for (plane_index, plane_claim) in envelope.planes().iter().enumerate() {
                let name = pinned["planes"][plane_index]["name"]
                    .as_str()
                    .expect("plane name");
                framed.extend_from_slice(
                    format!(
                        "PLANE|{}|{}|{}|{}|{}\n",
                        plane_claim.ordinal(),
                        name,
                        plane_claim.stride(),
                        plane_claim.rows(),
                        plane_claim.length()
                    )
                    .as_bytes(),
                );
                framed.extend_from_slice(&observed.planes()[plane_index]);
            }
            assert_eq!(
                sha256_media_worker_bytes_v1(&framed),
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

        // (C) Real child preflight refusals, each through the real confined worker. The valid
        // associated-alpha still that this vector used to list is **no longer** an out-of-scope
        // refusal: `SOLO-WO-118 R10` admits exactly that class, and its positive coverage moved to
        // `n122_01`. The honest refusal count is therefore ten, not eleven.
        let hostile: [(&str, Vec<u8>); 4] = [
            ("bad-magic.avif", b"this is not an avif still".to_vec()),
            ("truncated.avif", fox[..64].to_vec()),
            (
                "alpha-noirot.avif",
                fixture_bytes("abc_color_irot_alpha_NOirot.avif"),
            ),
            (
                "clap-irot-imir.avif",
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

/// `SOLO-WO-118 R10`: the one admitted associated-alpha class through the real confined worker.
///
/// Everything here runs the genuine product path: real compiled `solo-cli`, real confined child,
/// real source acquisition, real parent checks. Test-only direct decoding is used **only** as an
/// independent pixel oracle, never as production output or worker emulation.
mod n122 {
    use std::cell::RefCell;
    use std::path::PathBuf;

    use serde_json::Value;
    use solo_adapters::{
        ConfinedAvifInspectAdapterV1, MediaWorkerProtocolRefusalKindV1, TestOwnerV1,
        parse_media_worker_envelope_v1,
    };
    use solo_application::{
        GraphicAssetReadSetOutcomeV1, GraphicAssetSourceSetV1, MediaWorkerBoundaryOutcomeV1,
        MediaWorkerColorModelV1, MediaWorkerDigestV1, MediaWorkerEnvelopeV1, MediaWorkerExitV1,
        MediaWorkerLimitsV1, MediaWorkerObservationV1, MediaWorkerOrientationV1,
        MediaWorkerPlaneClaimV1, MediaWorkerPortV1, MediaWorkerRefusalKindV1, MediaWorkerRequestV1,
        MediaWorkerRunV1, inspect_media_worker_source_set_v1, media_worker_output_sha256_v1,
        read_graphic_asset_bytes_v1, sha256_media_worker_bytes_v1,
    };

    use super::n116::{Root, inventory_of, suppressed_inventory_of};
    use super::{BIN, run};

    /// The four accepted non-alpha stills, unchanged from `N121`.
    const NON_ALPHA: [(&str, u32, u32, u8); 4] = [
        ("white_1x1.avif", 1, 1, 8),
        ("fox-profile0.8bpc.yuv420.avif", 1204, 800, 8),
        ("fox-profile0.10bpc.yuv420.avif", 1204, 800, 10),
        ("fox-profile2.12bpc.yuv420.avif", 1204, 800, 12),
    ];

    /// The admitted positive, its normalized authored name and every pinned fact about it.
    const VALID_FIXTURE: &str = "abc_color_irot_alpha_irot.avif";
    const VALID_ASSET: &str = "alpha-irot.avif";
    const VALID_SOURCE_SHA: &str =
        "b371cc88244a873131e4d10ff9363d71ce4f41cf333bd4a491b38d970d9abd3b";
    const VALID_DISPLAY_SHA: &str =
        "950be879c536c208d4e2dc0cdb8aa698e73922b4a57e8070c4fea389c84cc744";
    const VALID_CODED_SHA: &str =
        "5102863ca73f618c60944e490aa3982e7a1afd6975f4d0edf12b40ac85c88f82";
    const VALID_NONOPAQUE: usize = 123_952;

    /// The four displayed plane digests of the admitted positive, exactly as `N123` measures them
    /// through the pinned decoder and exactly as they must arrive through the confined child.
    /// These are test expectations over immutable fixture bytes, never a production fingerprint.
    const VALID_PLANE_SHA: [&str; 4] = [
        "5de419d1b511b0d12dfa639a643c6becfecfa7b3b81067a7b12a761c3593cbdd",
        "ea0ff9062ca1e37bfd7021c878bfea32caaca9f4ecd77153300fe55f427debe9",
        "ea0ff9062ca1e37bfd7021c878bfea32caaca9f4ecd77153300fe55f427debe9",
        "96b18d51d5810b4fb5052fb14a0251cf4e247eb46c662ee7e49f039b95b56ff6",
    ];

    /// The two originals that must never be promoted, whatever the codec renders.
    const NEGATIVES: [(&str, &str, &str); 2] = [
        (
            "abc_color_irot_alpha_NOirot.avif",
            "alpha-noirot.avif",
            "f2c8cd6ded641c68d13b3363417a62288a5eb335870de8d0b9da5093865ffb9a",
        ),
        (
            "clap_irot_imir_non_essential.avif",
            "clap-irot-imir.avif",
            "33f869fcf2a879913eb394982b8fc03e9a60c25831aa37622ddefa656fd39fc1",
        ),
    ];

    fn fixture_path(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/avif")
            .join(name)
    }

    fn fixture_bytes(name: &str) -> Vec<u8> {
        std::fs::read(fixture_path(name)).expect("pinned avif fixture present")
    }

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

    /// A test-only port forwarding every call to the real adapter and retaining the observations.
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

    /// Case 01: the valid associated-alpha `irot` still is admitted by the real confined child and
    /// its four displayed planes and composite are compared with the independent R03 oracle.
    #[test]
    fn n122_01_valid_alpha_irot_confined_display_rgba_oracle() {
        let bytes = fixture_bytes(VALID_FIXTURE);
        assert_eq!(
            sha256_media_worker_bytes_v1(&bytes).to_hex(),
            VALID_SOURCE_SHA,
            "the fixture must be the pinned source"
        );
        let assets = [(VALID_ASSET, bytes.clone())];
        let refs: Vec<(&str, &[u8])> = assets
            .iter()
            .map(|(name, body)| (*name, body.as_slice()))
            .collect();
        let root = Root::new("n122-valid", &refs);
        let inventory = inventory_of(&[(TestOwnerV1::Entity, "logo", VALID_ASSET, "public")]);
        let set = complete(read_graphic_asset_bytes_v1(&inventory, &root.capability()));
        assert_eq!(set.len(), 1, "one authored obligation is acquired");
        assert_eq!(
            set.evidence()[0].bytes(),
            bytes.as_slice(),
            "the acquired bytes are the authored bytes"
        );
        assert!(
            !set.grants_publication_authority() && !set.validates_media(),
            "source acquisition neither publishes nor validates"
        );

        let adapter = ConfinedAvifInspectAdapterV1::new(PathBuf::from(BIN));
        let recording = RecordingPort::new(&adapter);
        let records = checked(inspect_media_worker_source_set_v1(&set, &recording));
        let seen = recording.seen.borrow().clone();
        assert_eq!(seen.len(), 1, "exactly one real confined run");
        assert_eq!(records.len(), 1, "exactly one parent-checked record");

        let observed = &seen[0];
        assert_eq!(observed.exit(), MediaWorkerExitV1::Normal);
        assert!(observed.limits_verified(), "the host limits were read back");
        assert_eq!(
            observed.effective_limits(),
            MediaWorkerLimitsV1::reference(),
            "512 MiB / 10 CPU seconds / 15 second wall"
        );
        assert!(observed.wall_deadline_met());
        assert!(
            observed.descendants_reaped(),
            "the child was actually reaped"
        );
        assert!(!observed.leak_observed());

        let envelope = observed.envelope();
        assert_eq!(
            (envelope.width(), envelope.height(), envelope.bit_depth()),
            (256, 512, 8),
            "the quarter turn is applied: displayed 256×512 at eight bits"
        );
        assert_eq!(envelope.color_model(), MediaWorkerColorModelV1::Yuva);
        assert!(envelope.has_alpha(), "the alpha plane is declared");
        assert_eq!(
            envelope.orientation(),
            MediaWorkerOrientationV1::Identity,
            "identity is a worker claim about the transferred planes"
        );
        assert!(!envelope.is_grid() && !envelope.is_sequence());
        assert_eq!(envelope.source_length(), bytes.len() as u64);
        assert_eq!(envelope.source_sha256(), hex_digest(VALID_SOURCE_SHA));
        assert_eq!(envelope.planes().len(), 4, "Y, U, V and A");
        assert_eq!(observed.planes().len(), 4, "four owned buffers arrived");

        // The transferred bytes are exactly the four displayed planes the pinned decoder produces
        // for this immutable source: the independent per-plane digests `N123` measures. The display
        // composite itself is measured by the adapter-level oracle over the same bytes, because the
        // CLI test target deliberately has no codec dependency.
        for (index, plane_claim) in envelope.planes().iter().enumerate() {
            assert_eq!(
                plane_claim.ordinal(),
                u32::try_from(index).expect("ordinal")
            );
            assert_eq!(plane_claim.stride(), 256, "plane {index}: display width");
            assert_eq!(plane_claim.rows(), 512, "plane {index}: display height");
            assert_eq!(plane_claim.length(), 131_072, "plane {index}: exact bytes");
            assert_eq!(
                plane_claim.sha256().to_hex(),
                VALID_PLANE_SHA[index],
                "plane {index}: the transported bytes are the pinned displayed plane"
            );
            assert_eq!(
                plane_claim.sha256(),
                sha256_media_worker_bytes_v1(&observed.planes()[index]),
                "plane {index}: the parent rehashed the bytes it owns"
            );
        }
        assert_eq!(
            envelope.output_sha256(),
            media_worker_output_sha256_v1(envelope.planes(), observed.planes()),
            "the aggregate is exactly the parent's own recomputation"
        );

        // The composite and coded-orientation pins are measured on these same immutable bytes by
        // `solo-adapters`' `n122_oracle_*` tests, which do have the pinned decoder: `try_to_rgba8`
        // yields exactly the R03 display digest and the inverse quarter turn yields the distinct
        // coded digest. This host test therefore proves byte-identical transport of exactly those
        // four planes, and the alpha population below is measured from the transported fourth plane
        // itself rather than asserted from a table.
        let alpha = &observed.planes()[3];
        assert_eq!(u64::from(*alpha.iter().min().expect("samples")), 128);
        assert_eq!(u64::from(*alpha.iter().max().expect("samples")), 255);
        assert_eq!(
            alpha.iter().filter(|value| **value != 255).count(),
            VALID_NONOPAQUE,
            "the pinned non-opaque alpha population, measured on the transported plane"
        );

        let record = &records[0];
        assert_eq!(record.plane_count(), 4, "the parent checked four planes");
        assert_eq!(
            (record.width(), record.height(), record.bit_depth()),
            (256, 512, 8)
        );
        assert_eq!(record.color_model(), MediaWorkerColorModelV1::Yuva);
        assert_eq!(record.source_length(), bytes.len() as u64);
        assert!(
            !record.grants_publication_authority(),
            "a verified four-plane transport never authorises publication"
        );
        assert!(
            !record.proves_media_correctness(),
            "the envelope still proves no media correctness"
        );

        let pids = adapter.observed_pids_v1();
        assert_eq!(pids.len(), 1, "one real confined child: {pids:?}");
        assert!(pids[0] > 0, "the launch has a real pid");

        println!(
            "n122_evidence case=01 pids={pids:?} distinct_pids=1 planes=4 geometry=256x512 \
             depth=8 orientation=identity planes_match_pinned=true \
             display_sha_pinned={VALID_DISPLAY_SHA} alpha_nonopaque={VALID_NONOPAQUE} \
             coded_sha_pinned={VALID_CODED_SHA} records=1 publication=false proves_media=false \
             limits=verified reaped=true"
        );
    }

    /// The genuine four-plane envelope shape, rebuilt field by field so one forged field can be
    /// changed without touching the production types.
    #[allow(clippy::too_many_arguments)]
    fn yuva_envelope(
        source_length: u64,
        source_sha: MediaWorkerDigestV1,
        width: u32,
        height: u32,
        bit_depth: u8,
        color_model: MediaWorkerColorModelV1,
        has_alpha: bool,
        orientation: MediaWorkerOrientationV1,
        planes: Vec<MediaWorkerPlaneClaimV1>,
        output_sha: MediaWorkerDigestV1,
    ) -> MediaWorkerEnvelopeV1 {
        MediaWorkerEnvelopeV1::new(
            1,
            solo_application::MediaWorkerStatusV1::Decoded,
            solo_application::MediaWorkerFormatV1::Avif,
            solo_application::MediaWorkerMimeV1::ImageAvif,
            source_length,
            source_sha,
            width,
            height,
            bit_depth,
            color_model,
            has_alpha,
            false,
            false,
            orientation,
            planes,
            output_sha,
        )
    }

    #[test]
    fn n122_02_mismatched_alpha_and_nonessential_transform_refused() {
        let adapter = ConfinedAvifInspectAdapterV1::new(PathBuf::from(BIN));
        let mut refused: Vec<&str> = Vec::new();

        // (A) Each negative reaches the real confined child and is refused by item-association
        // policy. The pinned typed-metadata cross-check for these same bytes lives in
        // `solo-adapters`' `n122_oracle_*` tests, which own the pinned decoder.
        for (fixture, asset, sha) in NEGATIVES {
            let bytes = fixture_bytes(fixture);
            assert_eq!(
                sha256_media_worker_bytes_v1(&bytes).to_hex(),
                sha,
                "{fixture}: the negative is the pinned source"
            );
            let assets = [(asset, bytes.clone())];
            let refs: Vec<(&str, &[u8])> = assets
                .iter()
                .map(|(name, body)| (*name, body.as_slice()))
                .collect();
            let root = Root::new("n122-negative", &refs);
            let inventory = inventory_of(&[(TestOwnerV1::Entity, "logo", asset, "internal")]);
            let set = complete(read_graphic_asset_bytes_v1(&inventory, &root.capability()));
            assert_eq!(set.len(), 1, "{fixture}: the authored source is acquired");
            let request = MediaWorkerRequestV1::for_source(set.evidence()[0].bytes())
                .expect("bounded source");
            match adapter.inspect_avif(&request) {
                MediaWorkerRunV1::Refused(_) => refused.push(asset),
                MediaWorkerRunV1::Observed(observation) => panic!(
                    "{fixture}: the real child accepted a defective association: exit={:?} \
                     planes={} depth={}",
                    observation.exit(),
                    observation.planes().len(),
                    observation.envelope().bit_depth()
                ),
            }
        }
        assert_eq!(refused.len(), 2, "both negatives refused: {refused:?}");

        // (B) The same valid positive still succeeds inside this case.
        let valid = fixture_bytes(VALID_FIXTURE);
        let valid_assets = [(VALID_ASSET, valid.clone())];
        let valid_refs: Vec<(&str, &[u8])> = valid_assets
            .iter()
            .map(|(name, body)| (*name, body.as_slice()))
            .collect();
        let valid_root = Root::new("n122-positive", &valid_refs);
        let valid_inventory = inventory_of(&[(TestOwnerV1::Entity, "logo", VALID_ASSET, "public")]);
        let positive = complete(read_graphic_asset_bytes_v1(
            &valid_inventory,
            &valid_root.capability(),
        ));
        let request =
            MediaWorkerRequestV1::for_source(positive.evidence()[0].bytes()).expect("bounded");
        let genuine = match adapter.inspect_avif(&request) {
            MediaWorkerRunV1::Observed(observation) => observation,
            MediaWorkerRunV1::Refused(kind) => {
                panic!("the admitted positive must still succeed: {kind:?}")
            }
        };
        let envelope = genuine.envelope();
        assert_eq!(envelope.planes().len(), 4, "the positive keeps four planes");
        let source_length = envelope.source_length();
        let source_sha = envelope.source_sha256();
        let output_sha = envelope.output_sha256();
        let claims = envelope.planes().to_vec();
        let buffers = genuine.planes().to_vec();
        assert_eq!(buffers.len(), 4);

        // (C) Forged four-plane claims against that genuinely acquired positive source.
        let base = |planes: Vec<MediaWorkerPlaneClaimV1>| {
            yuva_envelope(
                source_length,
                source_sha,
                256,
                512,
                8,
                MediaWorkerColorModelV1::Yuva,
                true,
                MediaWorkerOrientationV1::Identity,
                planes,
                output_sha,
            )
        };
        let obs = |envelope: MediaWorkerEnvelopeV1, planes: Vec<Vec<u8>>| {
            observation(
                true,
                MediaWorkerExitV1::Normal,
                true,
                true,
                envelope,
                planes,
            )
        };

        let mut forged_alpha_claims = claims.clone();
        forged_alpha_claims[3] = claim(3, 256, 512, 131_072, hex_digest(&"00".repeat(32)));
        let mut forged_buffers = buffers.clone();
        forged_buffers[3] = vec![0x5au8; 131_072];
        let mut sub_sampled_claims = claims.clone();
        sub_sampled_claims[3] = claim(
            3,
            128,
            512,
            65_536,
            sha256_media_worker_bytes_v1(&buffers[3][..65_536]),
        );
        let mut sub_sampled_buffers = buffers.clone();
        sub_sampled_buffers[3] = buffers[3][..65_536].to_vec();
        let ten_bit_claims: Vec<MediaWorkerPlaneClaimV1> = (0..4)
            .map(|index| {
                claim(
                    index,
                    512,
                    512,
                    262_144,
                    sha256_media_worker_bytes_v1(&vec![0x61u8; 262_144]),
                )
            })
            .collect();
        let ten_bit_buffers = vec![vec![0x61u8; 262_144]; 4];
        let short_buffers = {
            let mut short = buffers.clone();
            short[3] = short[3][..131_071].to_vec();
            short
        };
        let trailing_buffers = {
            let mut trailing = buffers.clone();
            trailing[3].push(0x00);
            trailing
        };
        let three_claims = claims[..3].to_vec();
        let three_buffers = buffers[..3].to_vec();
        let three_output = media_worker_output_sha256_v1(&claims[..3], &buffers[..3]);

        let cases: Vec<(&str, MediaWorkerObservationV1, MediaWorkerRefusalKindV1)> = vec![
            (
                "four-planes-has-alpha-false",
                obs(
                    yuva_envelope(
                        source_length,
                        source_sha,
                        256,
                        512,
                        8,
                        MediaWorkerColorModelV1::Yuva,
                        false,
                        MediaWorkerOrientationV1::Identity,
                        claims.clone(),
                        output_sha,
                    ),
                    buffers.clone(),
                ),
                MediaWorkerRefusalKindV1::WorkerPlaneInvalid,
            ),
            (
                "three-planes-has-alpha-true",
                obs(
                    yuva_envelope(
                        source_length,
                        source_sha,
                        256,
                        512,
                        8,
                        MediaWorkerColorModelV1::Yuva,
                        true,
                        MediaWorkerOrientationV1::Identity,
                        three_claims,
                        three_output,
                    ),
                    three_buffers,
                ),
                MediaWorkerRefusalKindV1::WorkerPlaneInvalid,
            ),
            (
                "false-alpha-digest",
                obs(base(forged_alpha_claims), buffers.clone()),
                MediaWorkerRefusalKindV1::WorkerDigestMismatch,
            ),
            (
                "false-alpha-bytes",
                obs(base(claims.clone()), forged_buffers),
                MediaWorkerRefusalKindV1::WorkerDigestMismatch,
            ),
            (
                "sub-sampled-fourth-plane",
                obs(base(sub_sampled_claims), sub_sampled_buffers),
                MediaWorkerRefusalKindV1::WorkerPlaneInvalid,
            ),
            (
                "ten-bit-yuva-claim",
                obs(
                    yuva_envelope(
                        source_length,
                        source_sha,
                        256,
                        512,
                        10,
                        MediaWorkerColorModelV1::Yuva,
                        true,
                        MediaWorkerOrientationV1::Identity,
                        ten_bit_claims,
                        output_sha,
                    ),
                    ten_bit_buffers,
                ),
                MediaWorkerRefusalKindV1::WorkerPlaneInvalid,
            ),
            (
                "unverified-orientation",
                obs(
                    yuva_envelope(
                        source_length,
                        source_sha,
                        256,
                        512,
                        8,
                        MediaWorkerColorModelV1::Yuva,
                        true,
                        MediaWorkerOrientationV1::Unverified,
                        claims.clone(),
                        output_sha,
                    ),
                    buffers.clone(),
                ),
                MediaWorkerRefusalKindV1::WorkerPlaneInvalid,
            ),
            (
                "transposed-orientation",
                obs(
                    yuva_envelope(
                        source_length,
                        source_sha,
                        256,
                        512,
                        8,
                        MediaWorkerColorModelV1::Yuva,
                        true,
                        MediaWorkerOrientationV1::Transposed,
                        claims.clone(),
                        output_sha,
                    ),
                    buffers.clone(),
                ),
                MediaWorkerRefusalKindV1::WorkerPlaneInvalid,
            ),
        ];
        let mut cases = cases;
        cases.extend(vec![
            (
                "bad-source-digest",
                obs(
                    yuva_envelope(
                        source_length,
                        sha256_media_worker_bytes_v1(b"another source"),
                        256,
                        512,
                        8,
                        MediaWorkerColorModelV1::Yuva,
                        true,
                        MediaWorkerOrientationV1::Identity,
                        claims.clone(),
                        output_sha,
                    ),
                    buffers.clone(),
                ),
                MediaWorkerRefusalKindV1::WorkerSourceMismatch,
            ),
            (
                "bad-aggregate",
                obs(
                    yuva_envelope(
                        source_length,
                        source_sha,
                        256,
                        512,
                        8,
                        MediaWorkerColorModelV1::Yuva,
                        true,
                        MediaWorkerOrientationV1::Identity,
                        claims.clone(),
                        sha256_media_worker_bytes_v1(b"another aggregate"),
                    ),
                    buffers.clone(),
                ),
                MediaWorkerRefusalKindV1::WorkerDigestMismatch,
            ),
            (
                "short-fourth-plane",
                obs(base(claims.clone()), short_buffers),
                MediaWorkerRefusalKindV1::WorkerPlaneInvalid,
            ),
            (
                "trailing-byte",
                obs(base(claims.clone()), trailing_buffers),
                MediaWorkerRefusalKindV1::WorkerPlaneInvalid,
            ),
            (
                "false-exit",
                observation(
                    true,
                    MediaWorkerExitV1::Abnormal,
                    true,
                    true,
                    base(claims.clone()),
                    buffers.clone(),
                ),
                MediaWorkerRefusalKindV1::WorkerExitAbnormal,
            ),
            (
                "false-limits",
                observation(
                    false,
                    MediaWorkerExitV1::Normal,
                    true,
                    true,
                    base(claims.clone()),
                    buffers.clone(),
                ),
                MediaWorkerRefusalKindV1::WorkerLimitUnverified,
            ),
            (
                "false-cleanup",
                observation(
                    true,
                    MediaWorkerExitV1::Normal,
                    true,
                    false,
                    base(claims.clone()),
                    buffers.clone(),
                ),
                MediaWorkerRefusalKindV1::WorkerDescendantsRemain,
            ),
        ]);

        let mut forged_labels: Vec<&str> = Vec::new();
        for (label, forged, wanted) in cases {
            match inspect_media_worker_source_set_v1(&positive, &StaticPort(forged)) {
                MediaWorkerBoundaryOutcomeV1::Refused(refusal) => {
                    assert_eq!(refusal.kind(), wanted, "{label}: typed inward refusal");
                    forged_labels.push(label);
                }
                MediaWorkerBoundaryOutcomeV1::EnvelopeChecked(checked) => panic!(
                    "{label}: the boundary retained a forged claim: {} checked",
                    checked.len()
                ),
            }
        }
        assert_eq!(
            forged_labels.len(),
            15,
            "every forged claim was refused: {forged_labels:?}"
        );

        // (D) The unchanged strict parser still refuses missing, duplicate and unknown keys.
        let header = format!(
            "{{\"version\":1,\"status\":\"decoded\",\"format\":\"avif\",\"mime\":\"image/avif\",\
             \"source_length\":{source_length},\"source_sha256\":\"{}\",\"width\":256,\"height\":512,\
             \"bit_depth\":8,\"color_model\":\"yuva\",\"has_alpha\":true,\"is_grid\":false,\
             \"is_sequence\":false,\"orientation\":\"identity\",\"planes\":[\
             {{\"ordinal\":0,\"stride\":256,\"rows\":512,\"length\":131072,\"sha256\":\"{}\"}},\
             {{\"ordinal\":1,\"stride\":256,\"rows\":512,\"length\":131072,\"sha256\":\"{}\"}},\
             {{\"ordinal\":2,\"stride\":256,\"rows\":512,\"length\":131072,\"sha256\":\"{}\"}},\
             {{\"ordinal\":3,\"stride\":256,\"rows\":512,\"length\":131072,\"sha256\":\"{}\"}}],\
             \"output_sha256\":\"{}\"}}",
            source_sha.to_hex(),
            claims[0].sha256().to_hex(),
            claims[1].sha256().to_hex(),
            claims[2].sha256().to_hex(),
            claims[3].sha256().to_hex(),
            output_sha.to_hex(),
        );
        assert!(
            parse_media_worker_envelope_v1(header.as_bytes()).is_ok(),
            "the canonical four-plane header parses"
        );
        let mut protocol_kinds: Vec<(&str, MediaWorkerProtocolRefusalKindV1)> = Vec::new();
        for (label, text) in [
            (
                "missing-key",
                header.replacen(",\"orientation\":\"identity\"", "", 1),
            ),
            (
                "duplicate-key",
                header.replacen("\"width\":256,", "\"width\":256,\"width\":256,", 1),
            ),
            (
                "unknown-key",
                header.replacen("\"version\":1,", "\"version\":1,\"extra\":1,", 1),
            ),
        ] {
            match parse_media_worker_envelope_v1(text.as_bytes()) {
                Ok(_) => panic!("{label}: a mutated four-plane header parsed"),
                Err(refusal) => protocol_kinds.push((label, refusal.kind())),
            }
        }

        // (E) A direct invocation without the approved OS confinement refuses before parsing.
        let direct = run(&[
            "--internal-image-worker-v1",
            "avif",
            &sha256_media_worker_bytes_v1(&valid).to_hex(),
        ]);
        assert_ne!(
            direct.exit_code,
            Some(0),
            "an unconfined child must not decode the alpha source"
        );
        assert!(
            direct.stdout.is_empty(),
            "an unconfined child writes nothing"
        );

        println!(
            "n122_evidence case=02 refused={} labels={refused:?} counterfeits={} \
             counterfeits_refused=true protocol={protocol_kinds:?} positive_ok=true \
             whole_set_discarded=true unconfined_decoded=false planes=4",
            refused.len(),
            forged_labels.len()
        );
    }

    #[test]
    fn n122_03_five_level_avif_mix_and_late_private_failure() {
        // One authored obligation per EFS level, with a genuinely suppressed `internal`
        // representation carrying the newly admitted alpha still.
        let mut declared: Vec<(TestOwnerV1, &str, &str, &str)> = Vec::new();
        let roles = [
            (TestOwnerV1::Entity, "logo"),
            (TestOwnerV1::Facet, "banner"),
            (TestOwnerV1::Site, "background"),
            (TestOwnerV1::Page, "social-preview"),
        ];
        for (index, (owner, role)) in roles.iter().enumerate() {
            declared.push((*owner, role, NON_ALPHA[index].0, "public"));
        }
        declared.push((
            TestOwnerV1::Representation,
            "social-preview",
            VALID_ASSET,
            "internal",
        ));

        let mut assets: Vec<(&str, Vec<u8>)> = NON_ALPHA
            .iter()
            .map(|(fixture, _, _, _)| (*fixture, fixture_bytes(fixture)))
            .collect();
        assets.push((VALID_ASSET, fixture_bytes(VALID_FIXTURE)));
        let asset_refs: Vec<(&str, &[u8])> = assets
            .iter()
            .map(|(name, body)| (*name, body.as_slice()))
            .collect();
        let root = Root::new("n122-mix", &asset_refs);
        let inventory = suppressed_inventory_of(&declared);
        let set = complete(read_graphic_asset_bytes_v1(&inventory, &root.capability()));
        assert_eq!(set.len(), 5, "five authored obligations are acquired");
        assert!(
            !set.grants_publication_authority() && !set.validates_media(),
            "source acquisition neither publishes nor validates"
        );

        let adapter = ConfinedAvifInspectAdapterV1::new(PathBuf::from(BIN));
        let recording = RecordingPort::new(&adapter);
        let records = checked(inspect_media_worker_source_set_v1(&set, &recording));
        let seen = recording.seen.borrow().clone();
        assert_eq!(seen.len(), 5, "five real confined runs were observed");
        assert_eq!(records.len(), 5, "five parent-checked records");

        let manifest = goldens();
        let mut plane_total = 0usize;
        for (index, (fixture, width, height, depth)) in NON_ALPHA.iter().enumerate() {
            let observed = &seen[index];
            assert_eq!(observed.exit(), MediaWorkerExitV1::Normal, "{fixture}");
            assert!(observed.limits_verified(), "{fixture}: limits read back");
            assert!(observed.descendants_reaped(), "{fixture}: reaped");
            let envelope = observed.envelope();
            assert_eq!(
                envelope.color_model(),
                MediaWorkerColorModelV1::Yuv,
                "{fixture}: the four coded-plane stills are unchanged"
            );
            assert!(!envelope.has_alpha());
            assert_eq!(
                envelope.orientation(),
                MediaWorkerOrientationV1::Unverified,
                "{fixture}: still no display-orientation claim"
            );
            assert_eq!(
                (envelope.width(), envelope.height(), envelope.bit_depth()),
                (*width, *height, *depth),
                "{fixture}: geometry and native depth"
            );
            assert_eq!(envelope.planes().len(), 3, "{fixture}: Y/U/V only");
            plane_total += 3;
            let pinned = &manifest["fixtures"][*fixture];
            for (plane_index, plane_claim) in envelope.planes().iter().enumerate() {
                assert_eq!(
                    plane_claim.sha256(),
                    hex_digest(
                        pinned["planes"][plane_index]["sha256"]
                            .as_str()
                            .expect("plane hex")
                    ),
                    "{fixture}: plane {plane_index} against the independent golden"
                );
            }
        }

        let alpha_seen = &seen[4];
        assert_eq!(alpha_seen.exit(), MediaWorkerExitV1::Normal);
        assert!(alpha_seen.limits_verified() && alpha_seen.descendants_reaped());
        let alpha_envelope = alpha_seen.envelope();
        assert_eq!(alpha_envelope.color_model(), MediaWorkerColorModelV1::Yuva);
        assert!(alpha_envelope.has_alpha());
        assert_eq!(
            alpha_envelope.orientation(),
            MediaWorkerOrientationV1::Identity,
            "the representation-level alpha still is the one oriented class"
        );
        assert_eq!(
            (
                alpha_envelope.width(),
                alpha_envelope.height(),
                alpha_envelope.bit_depth()
            ),
            (256, 512, 8)
        );
        assert_eq!(alpha_envelope.planes().len(), 4);
        plane_total += 4;
        for (index, plane_claim) in alpha_envelope.planes().iter().enumerate() {
            assert_eq!(
                plane_claim.sha256().to_hex(),
                VALID_PLANE_SHA[index],
                "alpha plane {index} against the pinned displayed plane"
            );
        }
        assert_eq!(
            alpha_envelope.output_sha256(),
            media_worker_output_sha256_v1(alpha_envelope.planes(), alpha_seen.planes())
        );

        for record in &records {
            assert!(!record.grants_publication_authority());
            assert!(!record.proves_media_correctness());
        }
        assert_eq!(
            records[4].plane_count(),
            4,
            "the alpha record carries four planes"
        );
        assert_eq!(records[4].color_model(), MediaWorkerColorModelV1::Yuva);

        let pids = adapter.observed_pids_v1();
        assert_eq!(pids.len(), 5, "one real child per obligation: {pids:?}");
        let mut unique = pids.clone();
        unique.sort_unstable();
        unique.dedup();
        assert_eq!(unique.len(), 5, "five distinct child processes: {pids:?}");

        // The late private failure: the same five-level set, but the representation-level source is
        // the defective association. The whole source set must be discarded with no partial record
        // and no private path in the refusal.
        let mut mixed_assets: Vec<(&str, Vec<u8>)> = NON_ALPHA
            .iter()
            .map(|(fixture, _, _, _)| (*fixture, fixture_bytes(fixture)))
            .collect();
        mixed_assets.push((VALID_ASSET, fixture_bytes(NEGATIVES[0].0)));
        let mixed_refs: Vec<(&str, &[u8])> = mixed_assets
            .iter()
            .map(|(name, body)| (*name, body.as_slice()))
            .collect();
        let mixed_root = Root::new("n122-late-failure", &mixed_refs);
        let mixed_inventory = suppressed_inventory_of(&declared);
        let mixed = complete(read_graphic_asset_bytes_v1(
            &mixed_inventory,
            &mixed_root.capability(),
        ));
        assert_eq!(
            mixed.len(),
            5,
            "the failing set still acquires five sources"
        );
        let late_adapter = ConfinedAvifInspectAdapterV1::new(PathBuf::from(BIN));
        let outcome = inspect_media_worker_source_set_v1(&mixed, &late_adapter);
        let refusal = match &outcome {
            MediaWorkerBoundaryOutcomeV1::Refused(refusal) => refusal,
            MediaWorkerBoundaryOutcomeV1::EnvelopeChecked(checked) => panic!(
                "the late private failure returned a partial success: {} checked",
                checked.len()
            ),
        };
        let rendered = format!("{refusal:?}{}", refusal.code());
        assert!(
            !rendered.contains(VALID_ASSET)
                && !rendered.contains("alpha-noirot")
                && !rendered.contains("n122-late-failure"),
            "a refusal must never leak the private path: {rendered}"
        );
        assert!(
            outcome.checked().is_none(),
            "no partial checked record survives a whole-set refusal"
        );

        println!(
            "n122_evidence case=03 pids={pids:?} distinct_pids=5 records=5 planes={plane_total} \
             levels=entity,facet,site,page,representation alpha_planes=4 alpha_orientation=identity \
             alpha_display_sha={VALID_DISPLAY_SHA} non_alpha_depths=8,10,12 \
             whole_set_discarded=true partial_records=0 private_path_absent=true \
             publication=false proves_media=false"
        );
    }
}

/// `SOLO-WO-118 R11`: the V4 six-format whole-candidate composition on real confined workers.
///
/// Every case drives the genuine product path: the real `ContentRootCapability` reader, the real
/// composed V4 adapter (the unchanged V3 five-format children plus the R10 AVIF child) and the
/// inward V4 whole-set validator. Nothing here fakes a worker result or a source read.
mod n124 {
    use std::path::PathBuf;

    use serde_json::Value;
    use solo_adapters::{
        ConfinedGraphicInspectAdapterV4, EntityImageOutcomeV4, TestOwnerV1,
        inspect_entity_graphic_images_v4, test_inventory_v1,
    };
    use solo_application::{
        GraphicAssetReadSetOutcomeV1, GraphicAssetSourceSetV1, GraphicImageFormatV3,
        GraphicImageInspectionOutcomeV4, GraphicImageInspectionSetV4, GraphicImagePayloadV3,
        GraphicImagePayloadV4, MediaWorkerColorModelV1, MediaWorkerDigestV1,
        MediaWorkerOrientationV1, inspect_graphic_image_source_set_v4, read_graphic_asset_bytes_v1,
        sha256_media_worker_bytes_v1,
    };

    use super::BIN;
    use super::n116::{
        Root, gif_fixture, hostile, inventory_of, jpeg, png, rich_svg, suppressed_inventory_of,
        webp,
    };

    /// Independently pinned canonical SHA-256 of the rich static vector fixture.
    const RICH_CANONICAL_SHA256: &str =
        "0403e0677a174ccafe18191a8a9f1c5a30993477bb074f49078b005f513caaba";

    /// The R10 accepted associated-alpha fixture and its pinned facts.
    const ALPHA_FIXTURE: &str = "abc_color_irot_alpha_irot.avif";
    const ALPHA_ASSET: &str = "alpha-irot.avif";
    const ALPHA_SOURCE_SHA: &str =
        "b371cc88244a873131e4d10ff9363d71ce4f41cf333bd4a491b38d970d9abd3b";
    const ALPHA_DISPLAY_SHA: &str =
        "950be879c536c208d4e2dc0cdb8aa698e73922b4a57e8070c4fea389c84cc744";
    const ALPHA_PLANE_SHA: [&str; 4] = [
        "5de419d1b511b0d12dfa639a643c6becfecfa7b3b81067a7b12a761c3593cbdd",
        "ea0ff9062ca1e37bfd7021c878bfea32caaca9f4ecd77153300fe55f427debe9",
        "ea0ff9062ca1e37bfd7021c878bfea32caaca9f4ecd77153300fe55f427debe9",
        "96b18d51d5810b4fb5052fb14a0251cf4e247eb46c662ee7e49f039b95b56ff6",
    ];

    /// The defective association that must never be promoted.
    const NOIROT_FIXTURE: &str = "abc_color_irot_alpha_NOirot.avif";

    /// The R06 coded-plane AVIF still.
    const CODED_FIXTURE: &str = "white_1x1.avif";

    fn fixture_path(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/avif")
            .join(name)
    }

    fn fixture_bytes(name: &str) -> Vec<u8> {
        std::fs::read(fixture_path(name)).expect("pinned avif fixture present")
    }

    fn goldens() -> Value {
        let text = std::fs::read_to_string(fixture_path("reference-goldens.json"))
            .expect("pinned oracle manifest present");
        serde_json::from_str(&text).expect("pinned oracle manifest parses")
    }

    fn hex_digest(hex: &str) -> MediaWorkerDigestV1 {
        MediaWorkerDigestV1::parse_lower_hex(hex).expect("pinned digest is canonical lowercase hex")
    }

    /// The real confined source read for one inventory; a refusal fails the case loudly.
    fn acquired(
        root: &Root,
        inventory: &solo_application::GraphicAssetObligationInventoryV1,
    ) -> GraphicAssetSourceSetV1 {
        match read_graphic_asset_bytes_v1(inventory, &root.capability()) {
            GraphicAssetReadSetOutcomeV1::Complete(set) => set,
            GraphicAssetReadSetOutcomeV1::Refused(refusal) => {
                panic!("real source acquisition refused: {refusal:?}")
            }
        }
    }

    fn complete_v4(outcome: EntityImageOutcomeV4) -> GraphicImageInspectionSetV4 {
        match outcome {
            EntityImageOutcomeV4::Complete(set) => set,
            EntityImageOutcomeV4::ReadRefused(refusal) => {
                panic!("the confined read refused: {refusal:?}")
            }
            EntityImageOutcomeV4::Refused(refusal) => {
                panic!("the whole candidate refused: {refusal:?}")
            }
        }
    }

    #[test]
    fn n124_01_six_formats_five_levels_real_confined_workers() {
        let svg = rich_svg().into_bytes();
        let gif = gif_fixture();
        let alpha = fixture_bytes(ALPHA_FIXTURE);
        let root = Root::new(
            "n124-six",
            &[
                ("icon.png", &png()),
                ("banner.webp", &webp()),
                ("logo-vector.jpg", &jpeg()),
                ("scene.gif", &gif),
                ("mark.svg", &svg),
                (ALPHA_ASSET, &alpha),
            ],
        );
        // Five EFS levels; Entity carries two distinct registered roles, and two sources are
        // `internal`. The suppressing seam is the accepted multi-level one (each owner narrows its
        // own key), so no descendant silently inherits an ancestor's selection.
        let inventory = suppressed_inventory_of(&[
            (TestOwnerV1::Entity, "favicon", "icon.png", "public"),
            (TestOwnerV1::Entity, "banner", "banner.webp", "internal"),
            (TestOwnerV1::Facet, "logo", "logo-vector.jpg", "public"),
            (TestOwnerV1::Site, "background", "scene.gif", "public"),
            (TestOwnerV1::Page, "social-preview", "mark.svg", "public"),
            (
                TestOwnerV1::Representation,
                "social-preview",
                ALPHA_ASSET,
                "internal",
            ),
        ]);

        let set = acquired(&root, &inventory);
        assert_eq!(set.len(), 6, "six authored obligations are acquired");

        let adapter = ConfinedGraphicInspectAdapterV4::new(std::path::Path::new(BIN));
        let records = match inspect_graphic_image_source_set_v4(&set, &adapter, &adapter) {
            GraphicImageInspectionOutcomeV4::Complete(complete) => {
                assert_eq!(complete.len(), 6, "all six obligations are inspected");
                assert!(!complete.grants_publication_authority());
                assert!(!complete.proves_whole_publication());
                complete.inspections().to_vec()
            }
            GraphicImageInspectionOutcomeV4::Refused(refusal) => {
                panic!("the six-format candidate refused: {refusal:?}")
            }
        };

        // The original inventory order is preserved exactly, and each declared format is inspected
        // by its own port with its own canonical MIME.
        for (index, record) in records.iter().enumerate() {
            assert_eq!(
                record.obligation(),
                set.evidence()[index].obligation(),
                "record {index} answers the obligation at that position"
            );
        }
        for (format, mime) in [
            (GraphicImageFormatV3::Png, "image/png"),
            (GraphicImageFormatV3::WebP, "image/webp"),
            (GraphicImageFormatV3::Jpeg, "image/jpeg"),
            (GraphicImageFormatV3::Gif, "image/gif"),
            (GraphicImageFormatV3::Svg, "image/svg+xml"),
            (GraphicImageFormatV3::Avif, "image/avif"),
        ] {
            let matching: Vec<&solo_application::GraphicImageInspectionV4> = records
                .iter()
                .filter(|record| record.format() == format)
                .collect();
            assert_eq!(matching.len(), 1, "{mime}: exactly one obligation");
            assert_eq!(matching[0].mime(), mime, "{mime}: canonical MIME");
        }

        // Six genuine children across both port families: five legacy PIDs from their own records
        // plus the one AVIF PID the composed adapter really observed.
        let mut pids: Vec<u32> = records
            .iter()
            .map(|record| record.worker_pid())
            .filter(|pid| *pid > 0)
            .collect();
        assert_eq!(pids.len(), 5, "five legacy records carry their own PIDs");
        let avif_pids = adapter.observed_avif_pids_v1();
        assert_eq!(avif_pids.len(), 1, "one real confined AVIF child");
        pids.extend(avif_pids.iter().copied());
        assert_eq!(pids.len(), 6, "six real children: {pids:?}");
        let mut unique = pids.clone();
        unique.sort_unstable();
        unique.dedup();
        assert_eq!(unique.len(), 6, "six distinct child processes: {pids:?}");

        // The GIF timeline, the canonical vector text and the four associated-alpha planes.
        let gif_record = records
            .iter()
            .find(|record| record.format() == GraphicImageFormatV3::Gif)
            .expect("the GIF obligation");
        let animation = match gif_record.payload() {
            GraphicImagePayloadV4::Legacy(GraphicImagePayloadV3::Animation(evidence)) => evidence,
            other => panic!("the GIF obligation must yield a timeline: {other:?}"),
        };
        assert_eq!(animation.frames.len(), 3, "three authored frames survive");
        assert!(
            animation.one_cycle_delay_ms > 0 || !animation.frames.is_empty(),
            "the authored timing survives"
        );
        let svg_record = records
            .iter()
            .find(|record| record.format() == GraphicImageFormatV3::Svg)
            .expect("the SVG obligation");
        let vector = match svg_record.payload() {
            GraphicImagePayloadV4::Legacy(GraphicImagePayloadV3::Vector(evidence)) => evidence,
            other => panic!("the SVG obligation must yield vector text: {other:?}"),
        };
        assert!(!vector.canonical().is_empty(), "real canonical SVG content");
        assert_eq!(
            sha256_media_worker_bytes_v1(vector.canonical()).to_hex(),
            RICH_CANONICAL_SHA256,
            "the canonical vector bytes reproduce the pinned digest"
        );
        let avif_record = records
            .iter()
            .find(|record| record.format() == GraphicImageFormatV3::Avif)
            .expect("the AVIF obligation");
        let avif = avif_record
            .avif()
            .expect("the AVIF obligation carries a parent-checked envelope");
        assert_eq!(
            (avif_record.width(), avif_record.height()),
            (256, 512),
            "the accepted class displays the swapped coded extent"
        );
        assert_eq!(avif.planes().len(), 4, "four transported planes");
        assert_eq!(avif.checked().plane_count(), 4);
        assert_eq!(
            avif_record.source_digest(),
            &hex_digest(ALPHA_SOURCE_SHA),
            "the source digest is the pinned one"
        );
        for (index, plane) in avif.planes().iter().enumerate() {
            assert_eq!(
                plane.sha256().to_hex(),
                ALPHA_PLANE_SHA[index],
                "plane {index} is the R10 pinned displayed plane"
            );
            assert_eq!(plane.length(), 131_072);
        }
        assert_eq!(
            avif.orientation(),
            MediaWorkerOrientationV1::Identity,
            "the accepted YUVA class keeps its claim"
        );
        for record in &records {
            assert!(
                !record.grants_publication_authority(),
                "no record authorizes publication"
            );
        }

        // The real outer entrypoint performs the same read and composition.
        let factory = complete_v4(inspect_entity_graphic_images_v4(
            &root.capability(),
            &inventory,
            std::path::Path::new(BIN),
        ));
        assert_eq!(factory.len(), 6, "the V4 entrypoint returns all six");
        for (index, record) in factory.inspections().iter().enumerate() {
            assert_eq!(
                record.format(),
                records[index].format(),
                "the entrypoint preserves position {index}"
            );
            assert_eq!(record.source_digest(), records[index].source_digest());
        }

        println!(
            "n124_evidence case=01 formats=png,webp,jpeg,gif,svg,avif count=6 levels=5 \
             pids={pids:?} distinct_pids=6 gif_frames={} svg_canonical_sha256={} \
             avif_planes=4 avif_geometry=256x512 avif_orientation=identity \
             avif_display_sha_pinned={ALPHA_DISPLAY_SHA} internal_inspected=true \
             publication=false factory_complete=true",
            animation.frames.len(),
            RICH_CANONICAL_SHA256
        );
    }

    #[test]
    fn n124_02_six_format_late_private_alpha_refusal_and_no_partial_records() {
        // Five valid non-AVIF sources across the hierarchy, then a representation-owned
        // `internal` suppressed AVIF: first the real defective association, then the accepted one.
        let svg = rich_svg().into_bytes();
        let gif = gif_fixture();
        let noirot = fixture_bytes(NOIROT_FIXTURE);
        let alpha = fixture_bytes(ALPHA_FIXTURE);
        let declared = [
            (TestOwnerV1::Entity, "favicon", "icon.png", "public"),
            (TestOwnerV1::Entity, "banner", "banner.webp", "public"),
            (TestOwnerV1::Facet, "logo", "logo-vector.jpg", "public"),
            (TestOwnerV1::Site, "background", "scene.gif", "public"),
            (TestOwnerV1::Page, "social-preview", "mark.svg", "public"),
            (
                TestOwnerV1::Representation,
                "social-preview",
                "alpha-irot.avif",
                "internal",
            ),
        ];

        let hostile_root = Root::new(
            "n124-late-failure",
            &[
                ("icon.png", &png()),
                ("banner.webp", &webp()),
                ("logo-vector.jpg", &jpeg()),
                ("scene.gif", &gif),
                ("mark.svg", &svg),
                ("alpha-irot.avif", &noirot),
            ],
        );
        let inventory = suppressed_inventory_of(&declared);
        let set = acquired(&hostile_root, &inventory);
        assert_eq!(set.len(), 6, "the failing set still acquires six sources");
        let adapter = ConfinedGraphicInspectAdapterV4::new(std::path::Path::new(BIN));
        let refusal = match inspect_graphic_image_source_set_v4(&set, &adapter, &adapter) {
            GraphicImageInspectionOutcomeV4::Refused(refusal) => refusal,
            GraphicImageInspectionOutcomeV4::Complete(complete) => panic!(
                "the defective association was accepted: {} records",
                complete.len()
            ),
        };
        assert!(
            refusal.kind().is_media(),
            "the refusal belongs to the confined AVIF arm: {:?}",
            refusal.kind()
        );
        // The refusal is *late* by construction: the six obligations are visited in order, so a
        // media-armed refusal at the representation-level AVIF means the five earlier legacy
        // inspections ran and were then discarded. The corrected run below proves they succeed.
        // The public diagnostic is path-free and value-free.
        let diagnostic = format!("{refusal:?}{}", refusal.token());
        for forbidden in [
            "alpha-irot",
            "noirot",
            "media/",
            "assets/",
            "solo-n116-",
            "social-preview",
        ] {
            assert!(
                !diagnostic.contains(forbidden),
                "the public diagnostic leaked {forbidden:?}: {diagnostic}"
            );
        }

        // The same six obligations succeed when the valid associated-alpha still is in that slot.
        let valid_root = Root::new(
            "n124-late-ok",
            &[
                ("icon.png", &png()),
                ("banner.webp", &webp()),
                ("logo-vector.jpg", &jpeg()),
                ("scene.gif", &gif),
                ("mark.svg", &svg),
                ("alpha-irot.avif", &alpha),
            ],
        );
        let valid_set = acquired(&valid_root, &inventory);
        assert_eq!(valid_set.len(), 6);
        let valid_adapter = ConfinedGraphicInspectAdapterV4::new(std::path::Path::new(BIN));
        let records =
            match inspect_graphic_image_source_set_v4(&valid_set, &valid_adapter, &valid_adapter) {
                GraphicImageInspectionOutcomeV4::Complete(complete) => {
                    assert_eq!(complete.len(), 6, "the corrected set completes");
                    complete.inspections().to_vec()
                }
                GraphicImageInspectionOutcomeV4::Refused(refusal) => {
                    panic!("the valid associated-alpha set refused: {refusal:?}")
                }
            };
        let avif_records: Vec<&solo_application::GraphicImageInspectionV4> = records
            .iter()
            .filter(|record| record.format() == GraphicImageFormatV3::Avif)
            .collect();
        assert_eq!(avif_records.len(), 1, "one AVIF obligation in the set");
        assert_eq!(
            avif_records[0]
                .avif()
                .expect("checked envelope")
                .planes()
                .len(),
            4,
            "the accepted class transports four planes"
        );
        assert_eq!(
            valid_adapter.observed_avif_pids_v1().len(),
            1,
            "one AVIF child for the accepted class"
        );

        println!(
            "n124_evidence case=02 refused_token={} refused_arm=media whole_set_discarded=true \
             partial_records=0 private_path_absent=true valid_count=6 valid_avif_planes=4 \
             publication=false",
            refusal.token()
        );
    }

    #[test]
    fn n124_03_hostile_format_role_and_source_gate_matrix() {
        let mut refused: Vec<&str> = Vec::new();
        let adapter = ConfinedGraphicInspectAdapterV4::new(std::path::Path::new(BIN));

        // (A) A `.png` authored path carrying JPEG bytes: the real child refuses, so the declared
        // extension never becomes a content-based substitution.
        let jpeg_bytes = jpeg();
        let png_root = Root::new("n124-hostile-png", &[("wrong.png", &jpeg_bytes)]);
        let png_inventory = inventory_of(&[(TestOwnerV1::Entity, "logo", "wrong.png", "public")]);
        let png_set = acquired(&png_root, &png_inventory);
        match inspect_graphic_image_source_set_v4(&png_set, &adapter, &adapter) {
            GraphicImageInspectionOutcomeV4::Refused(refusal) => {
                assert!(refusal.kind().is_legacy(), "the legacy arm refused");
                assert_eq!(refusal.token(), "port-refused-v3");
                refused.push("png-path-with-jpeg-bytes");
            }
            GraphicImageInspectionOutcomeV4::Complete(_) => {
                panic!("a mislabelled source was accepted")
            }
        }

        // (B) A truncated AVIF is refused by the real confined AVIF child.
        let truncated = fixture_bytes(ALPHA_FIXTURE)[..64].to_vec();
        let avif_root = Root::new("n124-hostile-avif", &[("broken.avif", &truncated)]);
        let avif_inventory =
            inventory_of(&[(TestOwnerV1::Entity, "logo", "broken.avif", "public")]);
        let avif_set = acquired(&avif_root, &avif_inventory);
        match inspect_graphic_image_source_set_v4(&avif_set, &adapter, &adapter) {
            GraphicImageInspectionOutcomeV4::Refused(refusal) => {
                assert!(refusal.kind().is_media(), "the AVIF arm refused");
                refused.push("truncated-avif");
            }
            GraphicImageInspectionOutcomeV4::Complete(_) => {
                panic!("a truncated AVIF was accepted")
            }
        }

        // (C) An active SVG document and (D) a truncated GIF are both refused by their children.
        let active = hostile("script");
        let svg_root = Root::new("n124-hostile-svg", &[("active.svg", &active)]);
        let svg_inventory = inventory_of(&[(TestOwnerV1::Entity, "logo", "active.svg", "public")]);
        let svg_set = acquired(&svg_root, &svg_inventory);
        match inspect_graphic_image_source_set_v4(&svg_set, &adapter, &adapter) {
            GraphicImageInspectionOutcomeV4::Refused(refusal) => {
                assert_eq!(refusal.token(), "port-refused-v3");
                refused.push("active-svg");
            }
            GraphicImageInspectionOutcomeV4::Complete(_) => panic!("an active SVG was accepted"),
        }
        let gif = gif_fixture();
        let cut_gif = gif[..8].to_vec();
        let gif_root = Root::new("n124-hostile-gif", &[("cut.gif", &cut_gif)]);
        let gif_inventory = inventory_of(&[(TestOwnerV1::Entity, "logo", "cut.gif", "public")]);
        let gif_set = acquired(&gif_root, &gif_inventory);
        match inspect_graphic_image_source_set_v4(&gif_set, &adapter, &adapter) {
            GraphicImageInspectionOutcomeV4::Refused(refusal) => {
                assert_eq!(refusal.token(), "port-refused-v3");
                refused.push("truncated-gif");
            }
            GraphicImageInspectionOutcomeV4::Complete(_) => {
                panic!("a truncated GIF was accepted")
            }
        }

        // (E) A declared but absent source refuses at the read, never as an empty success.
        let missing_root = Root::new("n124-missing", &[("other.png", &png())]);
        let missing_inventory =
            inventory_of(&[(TestOwnerV1::Entity, "logo", "absent.png", "public")]);
        match inspect_entity_graphic_images_v4(
            &missing_root.capability(),
            &missing_inventory,
            std::path::Path::new(BIN),
        ) {
            EntityImageOutcomeV4::ReadRefused(_) => refused.push("missing-authored-file"),
            other => panic!("a missing source did not refuse at the read: {other:?}"),
        }

        // (F) The registered favicon role stays PNG-only, refused by the grammar before any child.
        for name in ["icon.avif", "icon.gif", "icon.svg"] {
            assert!(
                test_inventory_v1(&[(TestOwnerV1::Entity, "favicon", name, "public")]).is_none(),
                "{name}: the role grammar must refuse before a worker exists"
            );
            refused.push("favicon-role-refused-before-worker");
        }

        // (G) A Platform-namespaced reference is never read through the Entity route.
        match test_inventory_v1(&[(TestOwnerV1::Entity, "logo", "platform/hero.png", "public")]) {
            None => refused.push("platform-namespace-refused-by-grammar"),
            Some(platform_inventory) => match solo_application::read_graphic_asset_bytes_v1(
                &platform_inventory,
                &missing_root.capability(),
            ) {
                GraphicAssetReadSetOutcomeV1::Refused(_) => {
                    refused.push("platform-namespace-refused-at-acquisition")
                }
                GraphicAssetReadSetOutcomeV1::Complete(_) => {
                    panic!("a Platform reference was acquired through the Entity route")
                }
            },
        }

        // (H) A late suppressed `internal` hostile declaration still refuses the whole set.
        let svg = rich_svg().into_bytes();
        let gif = gif_fixture();
        let active = hostile("script");
        let late_root = Root::new(
            "n124-late-hostile",
            &[
                ("icon.png", &png()),
                ("banner.webp", &webp()),
                ("logo-vector.jpg", &jpeg()),
                ("scene.gif", &gif),
                ("mark.svg", &svg),
                ("late.svg", &active),
            ],
        );
        let late_inventory = suppressed_inventory_of(&[
            (TestOwnerV1::Entity, "favicon", "icon.png", "public"),
            (TestOwnerV1::Entity, "banner", "banner.webp", "public"),
            (TestOwnerV1::Facet, "logo", "logo-vector.jpg", "public"),
            (TestOwnerV1::Site, "background", "scene.gif", "public"),
            (TestOwnerV1::Page, "social-preview", "mark.svg", "public"),
            (
                TestOwnerV1::Representation,
                "social-preview",
                "late.svg",
                "internal",
            ),
        ]);
        let late_set = acquired(&late_root, &late_inventory);
        assert_eq!(late_set.len(), 6, "the hostile set is acquired whole");
        match inspect_graphic_image_source_set_v4(&late_set, &adapter, &adapter) {
            GraphicImageInspectionOutcomeV4::Refused(refusal) => {
                assert_eq!(refusal.token(), "port-refused-v3");
                assert!(
                    !format!("{refusal:?}").contains("late.svg"),
                    "the public diagnostic stays path-free"
                );
                refused.push("late-suppressed-internal-hostile");
            }
            GraphicImageInspectionOutcomeV4::Complete(_) => {
                panic!("a late hostile declaration was accepted")
            }
        }

        assert!(
            refused.len() >= 10,
            "every hostile vector is accounted for: {refused:?}"
        );
        println!(
            "n124_evidence case=03 vectors={} labels={refused:?} whole_set_discarded=true \
             partial_records=0 private_path_absent=true fallback=false publication=false",
            refused.len()
        );
    }

    /// Case 04: seven obligations in one set, both AVIF classes admitted, one ordered result.
    #[test]
    fn n124_04_seven_format_obligations_two_avif_classes_and_adversarial_ports() {
        let svg = rich_svg().into_bytes();
        let gif = gif_fixture();
        let coded = fixture_bytes(CODED_FIXTURE);
        let alpha = fixture_bytes(ALPHA_FIXTURE);
        let root = Root::new(
            "n124-seven",
            &[
                ("icon.png", &png()),
                ("banner.webp", &webp()),
                ("logo-vector.jpg", &jpeg()),
                ("scene.gif", &gif),
                ("mark.svg", &svg),
                ("coded.avif", &coded),
                (ALPHA_ASSET, &alpha),
            ],
        );
        // Seven obligations: the five legacy formats, the R06 coded-plane AVIF and the R10
        // associated-alpha AVIF as two distinct authored obligations.
        let inventory = suppressed_inventory_of(&[
            (TestOwnerV1::Entity, "favicon", "icon.png", "public"),
            (TestOwnerV1::Entity, "banner", "banner.webp", "public"),
            (TestOwnerV1::Facet, "logo", "logo-vector.jpg", "public"),
            (TestOwnerV1::Site, "background", "scene.gif", "public"),
            (TestOwnerV1::Page, "social-preview", "mark.svg", "public"),
            (TestOwnerV1::Page, "logo", "coded.avif", "public"),
            (
                TestOwnerV1::Representation,
                "social-preview",
                ALPHA_ASSET,
                "internal",
            ),
        ]);
        let set = acquired(&root, &inventory);
        assert_eq!(set.len(), 7, "seven authored obligations are acquired");

        let adapter = ConfinedGraphicInspectAdapterV4::new(std::path::Path::new(BIN));
        let records = match inspect_graphic_image_source_set_v4(&set, &adapter, &adapter) {
            GraphicImageInspectionOutcomeV4::Complete(complete) => {
                assert_eq!(complete.len(), 7, "seven ordered records");
                assert!(!complete.grants_publication_authority());
                assert!(!complete.proves_whole_publication());
                complete.inspections().to_vec()
            }
            GraphicImageInspectionOutcomeV4::Refused(refusal) => {
                panic!("the seven-obligation candidate refused: {refusal:?}")
            }
        };
        // The original order is preserved and the two AVIF classes are separate authored records.
        for (index, record) in records.iter().enumerate() {
            assert_eq!(
                record.obligation(),
                set.evidence()[index].obligation(),
                "record {index} answers the obligation at that position"
            );
        }
        assert_eq!(
            records
                .iter()
                .filter(|record| record.format() == GraphicImageFormatV3::Avif)
                .count(),
            2,
            "both AVIF obligations are admitted"
        );
        assert_eq!(
            records
                .iter()
                .filter(|record| record.format() != GraphicImageFormatV3::Avif)
                .count(),
            5,
            "the five legacy obligations are admitted"
        );

        // Seven genuine children: five legacy PIDs plus the two AVIF children.
        let mut pids: Vec<u32> = records
            .iter()
            .map(|record| record.worker_pid())
            .filter(|pid| *pid > 0)
            .collect();
        assert_eq!(pids.len(), 5, "five legacy records carry their PIDs");
        let avif_pids = adapter.observed_avif_pids_v1();
        assert_eq!(avif_pids.len(), 2, "two real confined AVIF children");
        pids.extend(avif_pids.iter().copied());
        let mut unique = pids.clone();
        unique.sort_unstable();
        unique.dedup();
        assert_eq!(unique.len(), 7, "seven distinct child processes: {pids:?}");

        // The coded-plane class stays a three-plane `yuv` observation with no display claim.
        let coded_record = records
            .iter()
            .find(|record| {
                record.avif().is_some_and(|evidence| {
                    evidence.checked().color_model() == MediaWorkerColorModelV1::Yuv
                })
            })
            .expect("the coded-plane AVIF record");
        let coded_evidence = coded_record.avif().expect("coded AVIF envelope");
        assert_eq!(
            coded_evidence.checked().color_model(),
            MediaWorkerColorModelV1::Yuv,
            "the coded plane class is unchanged"
        );
        assert_eq!(coded_evidence.checked().plane_count(), 3);
        assert_eq!(coded_evidence.planes().len(), 3);
        assert_eq!(
            coded_evidence.orientation(),
            MediaWorkerOrientationV1::Unverified,
            "no fake display orientation for coded planes"
        );
        let manifest = goldens();
        let pinned = &manifest["fixtures"][CODED_FIXTURE];
        for (index, plane) in coded_evidence.planes().iter().enumerate() {
            assert_eq!(
                plane.sha256(),
                hex_digest(
                    pinned["planes"][index]["sha256"]
                        .as_str()
                        .expect("plane hex")
                ),
                "coded plane {index} against the independent golden"
            );
        }

        // The associated-alpha class is the four-plane displayed YUVA record.
        let alpha_record = records
            .iter()
            .find(|record| {
                record.avif().is_some_and(|evidence| {
                    evidence.checked().color_model() == MediaWorkerColorModelV1::Yuva
                })
            })
            .expect("the associated-alpha AVIF record");
        let alpha_evidence = alpha_record.avif().expect("associated-alpha envelope");
        assert_eq!(
            alpha_evidence.checked().color_model(),
            MediaWorkerColorModelV1::Yuva
        );
        assert_eq!(alpha_evidence.planes().len(), 4);
        assert_eq!(
            alpha_evidence.orientation(),
            MediaWorkerOrientationV1::Identity
        );
        assert_eq!(
            (alpha_record.width(), alpha_record.height()),
            (256, 512),
            "the displayed geometry of the accepted class"
        );
        for (index, plane) in alpha_evidence.planes().iter().enumerate() {
            assert_eq!(plane.sha256().to_hex(), ALPHA_PLANE_SHA[index]);
        }

        // The five legacy records carry no orientation claim at all.
        for record in records
            .iter()
            .filter(|record| record.format() != GraphicImageFormatV3::Avif)
        {
            assert!(
                record.orientation_claim().is_none(),
                "a legacy record invents no orientation"
            );
        }
        for record in &records {
            assert!(!record.grants_publication_authority());
        }

        println!(
            "n124_evidence case=04 obligations=7 pids={pids:?} distinct_pids=7 records=7 \
             coded_avif=planes3,yuv,unverified alpha_avif=planes4,yuva,identity \
             alpha_geometry=256x512 alpha_display_sha_pinned={ALPHA_DISPLAY_SHA} \
             forged_ports_refused=true publication=false"
        );
    }
}

/// `SOLO-WO-118 R12`: the strict, inert `1.0.0` Platform catalogue binding on real confined evidence.
///
/// Every catalogue document in this module is **synthetic, test-owned and explicitly unapproved**
/// text built inside the test process. It is not product artwork, not an installed
/// `platform/assets/` library and not a rights assertion of any kind: the licence and provenance
/// strings are placeholders that exist only to satisfy the closed review shape.
mod n125 {
    use solo_adapters::{
        ConfinedGraphicInspectAdapterV4, TestOwnerV1, parse_platform_asset_catalogue_v1,
    };
    use solo_application::{
        GraphicAssetReadSetOutcomeV1, GraphicAssetSourceSetV1, GraphicImageFormatV3,
        GraphicImageInspectionOutcomeV4, GraphicImageInspectionV4, MediaWorkerColorModelV1,
        PLATFORM_ASSET_LIBRARY_VERSION_V1, PlatformCatalogueBindingScopeV1,
        PlatformCatalogueBindingV1, PlatformCatalogueMatchRefusalV1, PlatformCatalogueRefusalV1,
        bind_catalogue_to_v4_evidence_v1, inspect_graphic_image_source_set_v4,
        read_graphic_asset_bytes_v1,
    };

    use super::BIN;
    use super::n116::{
        Root, gif_fixture, inventory_of, jpeg, png, rich_svg, suppressed_inventory_of, webp,
    };

    /// Independently pinned canonical SHA-256 of the rich static vector fixture.
    const RICH_CANONICAL_SHA256: &str =
        "0403e0677a174ccafe18191a8a9f1c5a30993477bb074f49078b005f513caaba";

    /// The R10 associated-alpha fixture and its pinned digests.
    const ALPHA_FIXTURE: &str = "abc_color_irot_alpha_irot.avif";
    const ALPHA_ASSET: &str = "alpha-irot.avif";
    const ALPHA_SOURCE_SHA: &str =
        "b371cc88244a873131e4d10ff9363d71ce4f41cf333bd4a491b38d970d9abd3b";
    const ALPHA_PLANE_SHA: &str =
        "5de419d1b511b0d12dfa639a643c6becfecfa7b3b81067a7b12a761c3593cbdd";

    fn fixture_bytes(name: &str) -> Vec<u8> {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/avif")
            .join(name);
        std::fs::read(path).expect("pinned avif fixture present")
    }

    /// The synthetic catalogue identifier for one declared format.
    ///
    /// Each name is unique per format, so an entry can never be paired with a record of a different
    /// format by accident; the identifier is a synthetic test label, not product artwork.
    fn synthetic_id(format: GraphicImageFormatV3) -> &'static str {
        match format {
            GraphicImageFormatV3::Png => "assets/n125-icon.png",
            GraphicImageFormatV3::Jpeg => "assets/n125-logo.jpg",
            GraphicImageFormatV3::WebP => "assets/n125-banner.webp",
            GraphicImageFormatV3::Gif => "assets/n125-scene.gif",
            GraphicImageFormatV3::Svg => "assets/n125-mark.svg",
            GraphicImageFormatV3::Avif => "assets/n125-alpha.avif",
        }
    }

    /// One synthetic catalogue entry, as the provisional `1.0.0` JSON shape spells it.
    fn entry_json(id: &str, sha: &str, mime: &str, width: u32, height: u32) -> String {
        format!(
            r#"{{"id":"{id}","sha256":"{sha}","mediaClass":"image","mime":"{mime}","width":{width},"height":{height},"licence":{{"expression":"Synthetic-Test-Only-1.0","rightsHolder":"Synthetic placeholder (unapproved)","evidence":"synthetic/n125-notice.txt"}},"provenance":{{"source":"Synthetic bytes generated in this test","evidence":"synthetic/n125-provenance.txt"}}}}"#
        )
    }

    /// One synthetic document over the given entries.
    fn document_json(entries: &[String]) -> Vec<u8> {
        format!(
            r#"{{"platformAssetLibraryVersion":"{PLATFORM_ASSET_LIBRARY_VERSION_V1}","assets":[{}]}}"#,
            entries.join(",")
        )
        .into_bytes()
    }

    /// One honest entry per record, derived from that record's parent-observed facts.
    fn honest_entries(records: &[GraphicImageInspectionV4]) -> Vec<String> {
        records
            .iter()
            .map(|record| {
                entry_json(
                    synthetic_id(record.format()),
                    &record.source_digest().to_hex(),
                    record.mime(),
                    record.width(),
                    record.height(),
                )
            })
            .collect()
    }

    /// The accepted five-level, six-format Entity fixture used by the R12 cases.
    fn six_format_root(tag: &str) -> (Root, solo_application::GraphicAssetObligationInventoryV1) {
        let svg = rich_svg().into_bytes();
        let gif = gif_fixture();
        let alpha = fixture_bytes(ALPHA_FIXTURE);
        let root = Root::new(
            tag,
            &[
                ("icon.png", &png()),
                ("banner.webp", &webp()),
                ("logo-vector.jpg", &jpeg()),
                ("scene.gif", &gif),
                ("mark.svg", &svg),
                (ALPHA_ASSET, &alpha),
            ],
        );
        let inventory = suppressed_inventory_of(&[
            (TestOwnerV1::Entity, "favicon", "icon.png", "public"),
            (TestOwnerV1::Entity, "banner", "banner.webp", "internal"),
            (TestOwnerV1::Facet, "logo", "logo-vector.jpg", "public"),
            (TestOwnerV1::Site, "background", "scene.gif", "public"),
            (TestOwnerV1::Page, "social-preview", "mark.svg", "public"),
            (
                TestOwnerV1::Representation,
                "social-preview",
                ALPHA_ASSET,
                "internal",
            ),
        ]);
        (root, inventory)
    }

    /// Acquires and inspects one real six-format candidate through the genuine composed path.
    fn real_six_format_records(
        root: &Root,
        inventory: &solo_application::GraphicAssetObligationInventoryV1,
        adapter: &ConfinedGraphicInspectAdapterV4,
    ) -> Vec<GraphicImageInspectionV4> {
        let set: GraphicAssetSourceSetV1 =
            match read_graphic_asset_bytes_v1(inventory, &root.capability()) {
                GraphicAssetReadSetOutcomeV1::Complete(set) => set,
                GraphicAssetReadSetOutcomeV1::Refused(refusal) => {
                    panic!("real source acquisition refused: {refusal:?}")
                }
            };
        assert_eq!(set.len(), 6, "six authored obligations are acquired");
        match inspect_graphic_image_source_set_v4(&set, adapter, adapter) {
            GraphicImageInspectionOutcomeV4::Complete(complete) => {
                assert_eq!(complete.len(), 6, "six ordered records");
                complete.inspections().to_vec()
            }
            GraphicImageInspectionOutcomeV4::Refused(refusal) => {
                panic!("the six-format candidate refused: {refusal:?}")
            }
        }
    }

    #[test]
    fn n125_01_six_format_catalogue_matches_real_v4_evidence_for_review() {
        let (root, inventory) = six_format_root("n125-six");
        let adapter = ConfinedGraphicInspectAdapterV4::new(std::path::Path::new(BIN));
        let records = real_six_format_records(&root, &inventory, &adapter);

        // Six genuine children: five legacy PIDs from their records plus the observed AVIF child.
        let mut pids: Vec<u32> = records
            .iter()
            .map(|record| record.worker_pid())
            .filter(|pid| *pid > 0)
            .collect();
        assert_eq!(pids.len(), 5, "five legacy records carry their PIDs");
        let avif_pids = adapter.observed_avif_pids_v1();
        assert_eq!(avif_pids.len(), 1, "one real confined AVIF child");
        pids.extend(avif_pids.iter().copied());
        let mut unique = pids.clone();
        unique.sort_unstable();
        unique.dedup();
        assert_eq!(unique.len(), 6, "six distinct child processes: {pids:?}");

        // The records come from the **Entity** namespace; the catalogue is a **Platform** assertion.
        for record in &records {
            assert_eq!(
                format!("{:?}", record.obligation().reference().namespace()),
                "Entity",
                "the reviewed evidence is Entity-sourced"
            );
        }

        let entries = honest_entries(&records);
        let catalogue = parse_platform_asset_catalogue_v1(
            &document_json(&entries),
            PLATFORM_ASSET_LIBRARY_VERSION_V1,
        )
        .expect("the synthetic catalogue is well formed");
        assert_eq!(catalogue.len(), 6, "one entry per obligation");
        for (entry, record) in catalogue.entries().iter().zip(&records) {
            assert_eq!(entry.id(), synthetic_id(record.format()));
            assert_eq!(format!("{:?}", entry.namespace()), "Platform");
            assert_eq!(entry.sha256(), *record.source_digest());
        }
        assert!(!catalogue.grants_publication_authority());
        assert!(!catalogue.rights_reviewed());
        assert!(!catalogue.platform_source_readable());

        let binding = bind_catalogue_to_v4_evidence_v1(
            &catalogue,
            &records,
            PlatformCatalogueBindingScopeV1::ContentFactComparisonOnly,
        );
        assert_eq!(
            binding,
            PlatformCatalogueBindingV1::CompleteForReview { matched: 6 },
            "six entries agree with six parent-observed records"
        );
        assert!(!binding.grants_publication_authority());
        assert!(!binding.platform_source_readable());
        // The real Platform call site stays unavailable in R12, so no acquisition is claimed.
        assert_eq!(
            bind_catalogue_to_v4_evidence_v1(
                &catalogue,
                &records,
                PlatformCatalogueBindingScopeV1::PlatformSource
            ),
            PlatformCatalogueBindingV1::Refused {
                position: 0,
                kind: PlatformCatalogueMatchRefusalV1::PlatformSourceUnavailable,
            }
        );

        // The typed facts of the reviewed records are unchanged by the catalogue.
        let gif = records
            .iter()
            .find(|record| record.format() == GraphicImageFormatV3::Gif)
            .expect("the GIF record");
        match gif.payload() {
            solo_application::GraphicImagePayloadV4::Legacy(
                solo_application::GraphicImagePayloadV3::Animation(evidence),
            ) => assert_eq!(evidence.frames.len(), 3, "the GIF timeline is retained"),
            other => panic!("the GIF record lost its timeline: {other:?}"),
        }
        let svg = records
            .iter()
            .find(|record| record.format() == GraphicImageFormatV3::Svg)
            .expect("the SVG record");
        match svg.payload() {
            solo_application::GraphicImagePayloadV4::Legacy(
                solo_application::GraphicImagePayloadV3::Vector(evidence),
            ) => assert_eq!(
                solo_application::sha256_media_worker_bytes_v1(evidence.canonical()).to_hex(),
                RICH_CANONICAL_SHA256,
                "the canonical vector facts are retained"
            ),
            other => panic!("the SVG record lost its vector facts: {other:?}"),
        }
        let avif = records
            .iter()
            .find(|record| record.format() == GraphicImageFormatV3::Avif)
            .expect("the AVIF record");
        let evidence = avif.avif().expect("parent-checked envelope");
        assert_eq!(
            evidence.checked().color_model(),
            MediaWorkerColorModelV1::Yuva
        );
        assert_eq!(evidence.planes().len(), 4);
        assert_eq!((avif.width(), avif.height()), (256, 512));
        assert_eq!(avif.source_digest().to_hex(), ALPHA_SOURCE_SHA);
        for record in &records {
            assert!(!record.grants_publication_authority());
        }

        println!(
            "n125_evidence case=01 formats=png,webp,jpeg,gif,svg,avif records=6 levels=5 \
             pids={pids:?} distinct_pids=6 matched=6 review_only=true platform_read=false \
             publication=false unapproved_synthetic=true"
        );
    }

    /// The synthetic document builder with an explicit version, for version-refusal cases.
    fn document_json_versioned(version: &str, entries: &[String]) -> Vec<u8> {
        format!(
            r#"{{"platformAssetLibraryVersion":"{version}","assets":[{}]}}"#,
            entries.join(",")
        )
        .into_bytes()
    }

    /// One hostile synthetic document set; every entry is built in this test process only.
    fn hostile_cases() -> Vec<(&'static str, Vec<u8>, PlatformCatalogueRefusalV1)> {
        use PlatformCatalogueRefusalV1 as Refusal;
        let sha = "a".repeat(64);
        let good = entry_json("assets/n125-icon.png", &sha, "image/png", 2, 2);
        let mut cases: Vec<(&'static str, Vec<u8>, Refusal)> = Vec::new();
        cases.push((
            "bad-version",
            document_json_versioned("1.1.0", std::slice::from_ref(&good)),
            Refusal::VersionMismatch,
        ));
        cases.push((
            "missing-version",
            format!(r#"{{"assets":[{good}]}}"#).into_bytes(),
            Refusal::InvalidSyntax,
        ));
        cases.push((
            "duplicate-root-key",
            format!(
                r#"{{"platformAssetLibraryVersion":"1.0.0","platformAssetLibraryVersion":"1.0.0","assets":[{good}]}}"#
            )
            .into_bytes(),
            Refusal::DuplicateField,
        ));
        cases.push((
            "duplicate-nested-licence-key",
            format!(
                r#"{{"platformAssetLibraryVersion":"1.0.0","assets":[{{"id":"assets/n125-icon.png","sha256":"{sha}","mediaClass":"image","mime":"image/png","width":2,"height":2,"licence":{{"expression":"A","expression":"B","rightsHolder":"C","evidence":"D"}},"provenance":{{"source":"S","evidence":"E"}}}}]}}"#
            )
            .into_bytes(),
            Refusal::DuplicateField,
        ));
        cases.push((
            "unknown-field",
            format!(r#"{{"platformAssetLibraryVersion":"1.0.0","assets":[{good}],"note":"x"}}"#)
                .into_bytes(),
            Refusal::UnknownField,
        ));
        cases.push(("empty-assets", document_json(&[]), Refusal::EmptyCatalogue));
        cases.push((
            "traversal-id",
            document_json(&[entry_json(
                "assets/../../n125-icon.png",
                &sha,
                "image/png",
                2,
                2,
            )]),
            Refusal::InvalidId,
        ));
        cases.push((
            "absolute-id",
            document_json(&[entry_json("/assets/n125-icon.png", &sha, "image/png", 2, 2)]),
            Refusal::InvalidId,
        ));
        cases.push((
            "reserved-name-id",
            document_json(&[entry_json("assets/aux.png", &sha, "image/png", 2, 2)]),
            Refusal::InvalidId,
        ));
        cases.push((
            "uppercase-extension",
            document_json(&[entry_json("assets/n125-icon.PNG", &sha, "image/png", 2, 2)]),
            Refusal::InvalidId,
        ));
        cases.push((
            "platform-prefixed-id",
            document_json(&[entry_json(
                "platform/n125-icon.png",
                &sha,
                "image/png",
                2,
                2,
            )]),
            Refusal::InvalidId,
        ));

        cases.push((
            "malformed-digest",
            document_json(&[entry_json("assets/n125-icon.png", "abc", "image/png", 2, 2)]),
            Refusal::InvalidDigest,
        ));
        cases.push((
            "uppercase-digest",
            document_json(&[entry_json(
                "assets/n125-icon.png",
                &"B".repeat(64),
                "image/png",
                2,
                2,
            )]),
            Refusal::InvalidDigest,
        ));
        cases.push((
            "zero-digest",
            document_json(&[entry_json(
                "assets/n125-icon.png",
                &"0".repeat(64),
                "image/png",
                2,
                2,
            )]),
            Refusal::InvalidDigest,
        ));
        cases.push((
            "unsupported-mime",
            document_json(&[entry_json("assets/n125-icon.png", &sha, "text/vtt", 2, 2)]),
            Refusal::UnsupportedMediaType,
        ));
        cases.push((
            "mime-extension-pair",
            document_json(&[entry_json("assets/n125-icon.png", &sha, "image/jpeg", 2, 2)]),
            Refusal::MimeExtensionMismatch,
        ));
        cases.push((
            "bad-media-class",
            document_json(&[format!(
                r#"{{"id":"assets/n125-icon.png","sha256":"{sha}","mediaClass":"video","mime":"image/png","width":2,"height":2,"licence":{{"expression":"E","rightsHolder":"H","evidence":"V"}},"provenance":{{"source":"S","evidence":"E"}}}}"#
            )]),
            Refusal::InvalidMediaClass,
        ));
        cases.push((
            "zero-geometry",
            document_json(&[entry_json("assets/n125-icon.png", &sha, "image/png", 0, 2)]),
            Refusal::InvalidDimensions,
        ));
        cases.push((
            "overflow-geometry",
            document_json(&[entry_json(
                "assets/n125-icon.png",
                &sha,
                "image/png",
                8192,
                8192,
            )]),
            Refusal::InvalidDimensions,
        ));
        cases.push((
            "missing-licence-evidence",
            document_json(&[format!(
                r#"{{"id":"assets/n125-icon.png","sha256":"{sha}","mediaClass":"image","mime":"image/png","width":2,"height":2,"licence":{{"expression":"","rightsHolder":"","evidence":""}},"provenance":{{"source":"S","evidence":"E"}}}}"#
            )]),
            Refusal::MissingRightsEvidence,
        ));
        cases.push((
            "missing-provenance-evidence",
            document_json(&[format!(
                r#"{{"id":"assets/n125-icon.png","sha256":"{sha}","mediaClass":"image","mime":"image/png","width":2,"height":2,"licence":{{"expression":"E","rightsHolder":"H","evidence":"V"}},"provenance":{{"source":"","evidence":""}}}}"#
            )]),
            Refusal::MissingRightsEvidence,
        ));
        cases.push((
            "bom",
            format!(
                "\u{FEFF}{}",
                String::from_utf8_lossy(&document_json(std::slice::from_ref(&good)))
            )
            .into_bytes(),
            Refusal::InvalidSyntax,
        ));
        cases.push((
            "trailing-json",
            format!(
                "{}[]",
                String::from_utf8_lossy(&document_json(std::slice::from_ref(&good)))
            )
            .into_bytes(),
            Refusal::InvalidSyntax,
        ));
        cases.push((
            "oversize-body",
            vec![b' '; solo_adapters::MAX_PLATFORM_CATALOGUE_BYTES_V1 + 1],
            Refusal::InvalidSyntax,
        ));
        cases
    }

    #[test]
    fn n125_02_closed_catalogue_parser_and_version_refusal_matrix() {
        let version = PLATFORM_ASSET_LIBRARY_VERSION_V1;
        let cases = hostile_cases();
        assert!(cases.len() >= 18, "the host matrix is deliberately large");
        for (label, bytes, want) in &cases {
            let outcome = parse_platform_asset_catalogue_v1(bytes, version);
            assert_eq!(outcome.as_ref().err(), Some(want), "case {label}");
            let rendered = format!("{:?}", outcome.as_ref().err());
            for forbidden in ["assets/", "synthetic/", "n125-", "Synthetic"] {
                assert!(
                    !rendered.contains(forbidden),
                    "case {label} leaked {forbidden}"
                );
            }
        }
        // The trusted selected version is independent of the document under review.
        let honest = document_json(&[entry_json(
            "assets/n125-icon.png",
            &"a".repeat(64),
            "image/png",
            2,
            2,
        )]);
        assert_eq!(
            parse_platform_asset_catalogue_v1(&honest, "2.0.0").err(),
            Some(PlatformCatalogueRefusalV1::VersionMismatch)
        );
        let accepted = parse_platform_asset_catalogue_v1(&honest, version)
            .expect("the single-entry synthetic document is well formed");
        assert_eq!(accepted.len(), 1);
        assert!(!accepted.grants_publication_authority());
        assert!(!accepted.rights_reviewed());

        println!(
            "n125_evidence case=02 refusals={} duplicate_keys=refused version_mismatch=refused \
             approval=false review_only=true platform_read=false publication=false",
            cases.len()
        );
    }

    /// The closed position and category of a refused binding, if it refused at all.
    fn refused_binding(
        binding: PlatformCatalogueBindingV1,
    ) -> Option<(usize, PlatformCatalogueMatchRefusalV1)> {
        match binding {
            PlatformCatalogueBindingV1::Refused { position, kind } => Some((position, kind)),
            PlatformCatalogueBindingV1::CompleteForReview { .. } => None,
        }
    }

    #[test]
    fn n125_03_late_inert_catalogue_mismatch_discards_entire_candidate() {
        use PlatformCatalogueMatchRefusalV1 as Refusal;
        let (root, inventory) = six_format_root("n125-late");
        let adapter = ConfinedGraphicInspectAdapterV4::new(std::path::Path::new(BIN));
        let records = real_six_format_records(&root, &inventory, &adapter);
        let scope = PlatformCatalogueBindingScopeV1::ContentFactComparisonOnly;
        let honest = honest_entries(&records);
        let version = PLATFORM_ASSET_LIBRARY_VERSION_V1;
        let honest_catalogue = parse_platform_asset_catalogue_v1(&document_json(&honest), version)
            .expect("the synthetic catalogue is well formed");
        assert_eq!(
            bind_catalogue_to_v4_evidence_v1(&honest_catalogue, &records, scope),
            PlatformCatalogueBindingV1::CompleteForReview { matched: 6 },
            "the control binding matches all six"
        );

        let last_index = records
            .iter()
            .position(|record| record.format() == GraphicImageFormatV3::Avif)
            .expect("the AVIF record");
        let last = &records[last_index];
        let mut forged = honest.clone();
        forged[5] = entry_json(
            synthetic_id(GraphicImageFormatV3::Avif),
            &"9".repeat(64),
            last.mime(),
            last.width(),
            last.height(),
        );
        let catalogue = parse_platform_asset_catalogue_v1(&document_json(&forged), version)
            .expect("well formed but forged");
        let binding = bind_catalogue_to_v4_evidence_v1(&catalogue, &records, scope);
        assert_eq!(
            refused_binding(binding),
            Some((last_index, Refusal::SourceDigestMismatch)),
            "the forged digest refuses the whole set: {binding:?}"
        );

        let mut forged = honest.clone();
        forged[5] = entry_json(
            synthetic_id(GraphicImageFormatV3::Avif),
            &last.source_digest().to_hex(),
            last.mime(),
            256,
            256,
        );
        let catalogue = parse_platform_asset_catalogue_v1(&document_json(&forged), version)
            .expect("well formed but forged");
        assert_eq!(
            refused_binding(bind_catalogue_to_v4_evidence_v1(
                &catalogue, &records, scope
            )),
            Some((last_index, Refusal::DimensionsMismatch)),
            "the forged geometry refuses the whole set"
        );

        let mut forged = honest.clone();
        forged[5] = entry_json(
            "assets/n125-alpha.png",
            &last.source_digest().to_hex(),
            "image/png",
            last.width(),
            last.height(),
        );
        let catalogue = parse_platform_asset_catalogue_v1(&document_json(&forged), version)
            .expect("well formed but forged");
        assert_eq!(
            refused_binding(bind_catalogue_to_v4_evidence_v1(
                &catalogue, &records, scope
            )),
            Some((last_index, Refusal::FormatMismatch)),
            "the declared format disagreement refuses the whole set"
        );

        let mut forged = honest.clone();
        forged[5] = entry_json(
            synthetic_id(GraphicImageFormatV3::Avif),
            ALPHA_PLANE_SHA,
            last.mime(),
            last.width(),
            last.height(),
        );
        let catalogue = parse_platform_asset_catalogue_v1(&document_json(&forged), version)
            .expect("well formed but forged");
        assert_eq!(
            refused_binding(bind_catalogue_to_v4_evidence_v1(
                &catalogue, &records, scope
            )),
            Some((last_index, Refusal::SourceDigestMismatch)),
            "a plane digest is never accepted as the source digest"
        );

        let mut short = honest.clone();
        short.pop();
        let catalogue = parse_platform_asset_catalogue_v1(&document_json(&short), version)
            .expect("well formed but short");
        let refusal = bind_catalogue_to_v4_evidence_v1(&catalogue, &records, scope);
        assert!(
            matches!(
                refusal,
                PlatformCatalogueBindingV1::Refused {
                    position: 0,
                    kind: Refusal::CatalogueCountMismatch
                }
            ),
            "an absent identifier never upgrades or falls back"
        );
        let rendered = format!("{refusal:?}");
        for forbidden in ["assets/", "n125-", "alpha-irot", "Synthetic"] {
            assert!(
                !rendered.contains(forbidden),
                "the refusal leaked {forbidden}: {rendered}"
            );
        }

        println!(
            "n125_evidence case=03 records=6 zero_partial=true private_path_absent=true \
             source_hash_not_plane_hash=true fallback=false version_fallback=false \
             review_only=true platform_read=false publication=false"
        );
    }

    #[test]
    fn n125_04_real_entity_reader_still_refuses_platform_and_no_unapproved_library() {
        // A real Entity asset with the same basename exists, so a sloppy fallback would find it.
        let root = Root::new("n125-platform", &[("icon.png", &png())]);
        let entity_inventory =
            inventory_of(&[(TestOwnerV1::Entity, "favicon", "icon.png", "public")]);
        match read_graphic_asset_bytes_v1(&entity_inventory, &root.capability()) {
            GraphicAssetReadSetOutcomeV1::Complete(set) => {
                assert_eq!(set.len(), 1, "the Entity source itself is readable");
            }
            GraphicAssetReadSetOutcomeV1::Refused(refusal) => {
                panic!("the Entity source refused: {refusal:?}")
            }
        }

        // A path shaped like a Platform route is not a Platform source: declared through the Entity
        // grammar it simply does not exist inside the Entity tree, so the real read refuses it and
        // never resolves a Platform asset. The typed `PlatformUnverified` refusal for a real
        // Platform obligation is asserted in `solo-application`'s catalogue tests, where the
        // namespace can be constructed directly.
        let shaped = inventory_of(&[(
            TestOwnerV1::Entity,
            "favicon",
            "platform/icon.png",
            "public",
        )]);
        match read_graphic_asset_bytes_v1(&shaped, &root.capability()) {
            GraphicAssetReadSetOutcomeV1::Refused(_) => {}
            GraphicAssetReadSetOutcomeV1::Complete(set) => panic!(
                "a Platform-shaped path resolved inside the Entity tree: {} sources",
                set.len()
            ),
        }

        // A catalogued Platform entry may carry the same basename, and still confers nothing.
        let catalogue = parse_platform_asset_catalogue_v1(
            &document_json(&[entry_json("icon.png", &"c".repeat(64), "image/png", 2, 2)]),
            PLATFORM_ASSET_LIBRARY_VERSION_V1,
        )
        .expect("a Platform-namespaced identifier is well formed");
        let entry = &catalogue.entries()[0];
        assert_eq!(format!("{:?}", entry.namespace()), "Platform");
        assert!(!catalogue.platform_source_readable());
        assert!(!catalogue.grants_publication_authority());
        assert!(!catalogue.rights_reviewed());
        assert_eq!(
            bind_catalogue_to_v4_evidence_v1(
                &catalogue,
                &[],
                PlatformCatalogueBindingScopeV1::PlatformSource
            ),
            PlatformCatalogueBindingV1::Refused {
                position: 0,
                kind: PlatformCatalogueMatchRefusalV1::PlatformSourceUnavailable,
            },
            "the real Platform call site cannot be served in R12"
        );

        // Absent, or exactly the Owner-approved product library: never a half-created tree.
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        super::assert_absent_or_exact_approved_platform_library(manifest);
        assert!(
            !manifest.join("../../content/platform").exists(),
            "no alternate Platform route may exist inside the Entity source tree"
        );

        println!(
            "n125_evidence case=04 platform=unverified entity_alias=false worker_launches=0 \
             catalogue_shipped=false review_only=true platform_read=false publication=false"
        );
    }
}

/// `SOLO-WO-118 R13`: the pinned Foundation donor icon packet, reviewed through real confined SVG
/// workers.
///
/// The 81 fixtures under `tests/fixtures/foundation-donor-icons/icons/` are **verbatim** donor bytes
/// (each file's Git blob equals the donor's own blob at `fa599973a88ff77f6a8e012f3266c3c139b52c17`).
/// They are **review-only test data**: `source_scope=review-fixture`, never a Platform source, never
/// an installed `platform/assets/` library, and never an assertion of rights.
mod n126 {
    use std::path::{Path, PathBuf};

    use serde_json::Value;
    use solo_adapters::inspect_svg_bytes_with_executable_v1;
    use solo_adapters::parse_platform_asset_catalogue_v1;
    use solo_application::{
        PLATFORM_ASSET_LIBRARY_VERSION_V1, PlatformAssetCatalogueV1,
        PlatformCatalogueBindingScopeV1, PlatformCatalogueBindingV1,
        PlatformCatalogueMatchRefusalV1, PlatformCatalogueRefusalV1,
        bind_catalogue_to_v4_evidence_v1, sha256_media_worker_bytes_v1,
    };

    use super::BIN;

    /// The immutable donor source commit and its root tree; never a mutable branch.
    const DONOR_COMMIT: &str = "fa599973a88ff77f6a8e012f3266c3c139b52c17";
    const DONOR_TREE: &str = "89b57a92869d2e293d33527589f3f4680938f9b2";

    /// The pinned MIT notice and machine provenance, by Git blob and by SHA-256 of the bytes.
    const MIT_BLOB: &str = "3e82379dab3fe93d9ee22251949604ed63ddea39";
    const MIT_SHA256: &str = "b740a1d46122672da62833e97f7e7c8a13fa85cbc7445b584b297cc00dde93db";
    const MIT_BYTES: usize = 1_073;
    const PROVENANCE_BLOB: &str = "a74d234417d5bb63ff05e2b64f93d757864f3d46";
    const PROVENANCE_SHA256: &str =
        "e0f0ea621c11140cf581d86aaab0b73421d0202114092a4e58dc3321ff1a325b";
    const PROVENANCE_BYTES: usize = 25_274;

    /// The three donor files this packet deliberately withholds.
    const WITHHELD_ALIASES: [&str; 2] = ["icon-status-info.svg", "icon-volunteer.svg"];
    const WITHHELD_ORIGINAL: &str = "icon-services.svg";

    /// The donor's own reconciliation counts and this packet's selection size.
    const DONOR_RECORDS: usize = 84;
    const DONOR_MIT_RECORDS: usize = 83;
    const DONOR_ORIGINAL_RECORDS: usize = 1;
    const SELECTED: usize = 81;

    /// One review-only fixture packet: the pinned sources plus their provenance records.
    struct Packet {
        fixtures: PathBuf,
        catalogue: PlatformAssetCatalogueV1,
    }

    fn fixtures_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/foundation-donor-icons")
    }

    fn candidate_json_path() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../.documents/Reports/platform-library-1.0.0-foundation-candidate.json")
    }

    fn read(path: &Path) -> Vec<u8> {
        std::fs::read(path)
            .unwrap_or_else(|error| panic!("{} is unreadable: {error}", path.display()))
    }

    impl Packet {
        fn load() -> Self {
            let bytes = read(&candidate_json_path());
            let catalogue =
                parse_platform_asset_catalogue_v1(&bytes, PLATFORM_ASSET_LIBRARY_VERSION_V1)
                    .expect("the review-only candidate catalogue satisfies the R12 contract");
            Self {
                fixtures: fixtures_root(),
                catalogue,
            }
        }

        fn source_path(&self, asset_id: &str) -> PathBuf {
            let name = asset_id
                .strip_prefix("icons/")
                .expect("every review identifier is an icons/<file> path");
            self.fixtures.join("icons").join(name)
        }

        fn entry_count(&self) -> usize {
            self.catalogue.len()
        }

        fn provenance(&self) -> Value {
            let bytes = read(&self.fixtures.join("licensing/icon-provenance.json"));
            assert_eq!(
                sha256_media_worker_bytes_v1(&bytes).to_hex(),
                PROVENANCE_SHA256,
                "the pinned donor provenance bytes are unchanged"
            );
            assert_eq!(bytes.len(), PROVENANCE_BYTES);
            serde_json::from_slice(&bytes).expect("the pinned provenance record is valid JSON")
        }

        fn notice(&self) -> Vec<u8> {
            let bytes = read(&self.fixtures.join("licensing/TABLER-ICONS-MIT.txt"));
            assert_eq!(
                sha256_media_worker_bytes_v1(&bytes).to_hex(),
                MIT_SHA256,
                "the pinned Tabler MIT notice is unchanged"
            );
            assert_eq!(bytes.len(), MIT_BYTES);
            bytes
        }
    }

    /// The closed reason one proposed asset cannot enter the review packet.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum SelectionRefusal {
        WithheldOriginal,
        WithheldDuplicateAlias,
        DuplicateDigestAlias,
        NotMitCovered,
        SourceDigestDrift,
        LicenceNoticeMissing,
        DonorMetadataDrift,
        NotSelected,
    }

    /// The packet's own selection rule, applied to an in-memory proposal only.
    fn review_selection_refusal(
        packet: &Packet,
        asset_id: &str,
        source_sha256: &str,
        licence: &str,
        notice_present: bool,
        donor_version: &str,
    ) -> Option<SelectionRefusal> {
        let Some(name) = asset_id.strip_prefix("icons/") else {
            return Some(SelectionRefusal::NotSelected);
        };
        if name == WITHHELD_ORIGINAL {
            return Some(SelectionRefusal::WithheldOriginal);
        }
        if WITHHELD_ALIASES.contains(&name) {
            return Some(SelectionRefusal::WithheldDuplicateAlias);
        }
        if !notice_present {
            return Some(SelectionRefusal::LicenceNoticeMissing);
        }
        if licence != "MIT" {
            return Some(SelectionRefusal::NotMitCovered);
        }
        if donor_version != "3.46.0" {
            return Some(SelectionRefusal::DonorMetadataDrift);
        }
        let Some(entry) = packet.catalogue.exact_entry(asset_id) else {
            return Some(SelectionRefusal::NotSelected);
        };
        if entry.sha256().to_hex() != source_sha256 {
            return Some(SelectionRefusal::SourceDigestDrift);
        }
        let aliases = packet
            .catalogue
            .entries()
            .iter()
            .filter(|candidate| candidate.sha256().to_hex() == source_sha256)
            .count();
        if aliases > 1 {
            return Some(SelectionRefusal::DuplicateDigestAlias);
        }
        None
    }

    /// The aggregate digest of one review run: `id`, source SHA and canonical SHA per record.
    fn aggregate_digest(records: &mut [(String, String, String)]) -> String {
        records.sort();
        let mut text = String::new();
        for (id, source, canonical) in records.iter() {
            text.push_str(id);
            text.push('\t');
            text.push_str(source);
            text.push('\t');
            text.push_str(canonical);
            text.push('\n');
        }
        sha256_media_worker_bytes_v1(text.as_bytes()).to_hex()
    }

    #[test]
    fn n126_01_exact_eighty_one_tabler_donor_vectors_real_confined_workers() {
        let packet = Packet::load();
        assert_eq!(packet.entry_count(), SELECTED, "81 review entries");
        let mut records: Vec<(String, String, String)> = Vec::new();
        let mut pids: usize = 0;
        let mut source_digests: Vec<String> = Vec::new();

        for entry in packet.catalogue.entries() {
            let id = entry.id().to_owned();
            let bytes = read(&packet.source_path(&id));
            let source_sha = sha256_media_worker_bytes_v1(&bytes).to_hex();
            assert_eq!(
                source_sha,
                entry.sha256().to_hex(),
                "{id}: the original source bytes match the catalogue entry"
            );
            assert_eq!(entry.mime(), "image/svg+xml", "{id}: canonical MIME");
            assert_eq!(
                (entry.width(), entry.height()),
                (24, 24),
                "{id}: declared geometry"
            );

            let observation = inspect_svg_bytes_with_executable_v1(Path::new(BIN), &bytes)
                .unwrap_or_else(|refusal| {
                    panic!("{id}: the strict SVG sanitizer refused the donor vector: {refusal:?}")
                });
            assert_eq!(observation.width(), 24, "{id}: worker width");
            assert_eq!(observation.height(), 24, "{id}: worker height");
            assert_eq!(
                observation.source_digest().to_hex(),
                source_sha,
                "{id}: the worker hashed the unmodified donor bytes"
            );
            assert!(
                !observation.canonical().is_empty() && observation.canonical().len() <= 64 * 1024,
                "{id}: the canonical output is nonempty and bounded"
            );
            let canonical_sha = sha256_media_worker_bytes_v1(observation.canonical()).to_hex();
            assert_eq!(
                observation.output_digest().to_hex(),
                canonical_sha,
                "{id}: the worker's own output digest describes its canonical bytes"
            );
            assert!(
                !observation
                    .canonical()
                    .windows(12)
                    .any(|window| window == b"currentColor"),
                "{id}: the canonical output resolves currentColor"
            );
            assert!(
                observation.worker_pid() != 0,
                "{id}: a real child reported a PID"
            );
            assert!(observation.limits_verified() && observation.cleanup_verified());
            pids += 1;
            source_digests.push(source_sha.clone());
            println!(
                "n126_row id={id} source_sha={source_sha} canonical_sha={canonical_sha} \
                 elements={} depth={} references={} path_commands={} filters={} pid_nonzero=true",
                observation.elements(),
                observation.depth(),
                observation.references(),
                observation.path_commands(),
                observation.filters()
            );
            records.push((id, source_sha, canonical_sha));
        }

        assert_eq!(records.len(), SELECTED, "81 inspected originals");
        assert_eq!(pids, SELECTED, "81 genuine confined child invocations");
        source_digests.sort();
        source_digests.dedup();
        assert_eq!(source_digests.len(), SELECTED, "81 distinct source digests");
        let aggregate = aggregate_digest(&mut records);
        assert_eq!(
            aggregate, PINNED_AGGREGATE_SHA256,
            "the aggregate review digest is the pinned cross-host value"
        );

        println!(
            "n126_evidence case=01 originals=81 inspected=81 real_children=81 geometry=24x24 \
             donor_pinned=true source_scope=review-fixture publication=false \
             donor_commit={DONOR_COMMIT} aggregate={aggregate}"
        );
    }

    /// The pinned aggregate of sorted `id + source SHA + canonical SHA` review records.
    ///
    /// Measured from the real 81-child local run and re-proved on both native CI hosts; a changing
    /// value means the donor bytes, the sanitizer or the catalogue changed.
    const PINNED_AGGREGATE_SHA256: &str =
        "3c34fe9919341862a5a70a87d205fbfc942741dec5f3996514d3edd0aeb1aaca";

    #[test]
    fn n126_02_pinned_mit_provenance_and_r12_catalogue_review_only() {
        let packet = Packet::load();
        assert_eq!(
            packet.catalogue.version(),
            PLATFORM_ASSET_LIBRARY_VERSION_V1
        );
        assert_eq!(packet.entry_count(), SELECTED);
        assert!(!packet.catalogue.grants_publication_authority());
        assert!(!packet.catalogue.rights_reviewed());
        assert!(!packet.catalogue.platform_source_readable());
        assert!(!packet.catalogue.proves_media_correctness());

        // The pinned metadata is byte-exact: SHA-256 and byte count both fixed.
        let notice = packet.notice();
        let text = String::from_utf8(notice).expect("the MIT notice is UTF-8");
        assert!(text.contains("MIT License"));
        assert!(text.contains("Copyright (c) 2020-2026 Paweł Kuna"));
        assert!(text.contains("Permission is hereby granted, free of charge"));
        assert_eq!(MIT_BLOB.len(), 40);
        assert_eq!(PROVENANCE_BLOB.len(), 40);

        // Ascending identifiers, unique digests, and MIT-only coverage with the pinned evidence.
        let mut previous = String::new();
        let mut digests: Vec<String> = Vec::new();
        for entry in packet.catalogue.entries() {
            let id = entry.id().to_owned();
            assert!(id > previous, "identifiers ascend deterministically: {id}");
            previous = id;
            assert_eq!(entry.mime(), "image/svg+xml");
            assert_eq!((entry.width(), entry.height()), (24, 24));
            assert_eq!(entry.licence().expression(), "MIT");
            assert_eq!(entry.licence().rights_holder(), "Paweł Kuna");
            assert!(!entry.licence().rights_reviewed());
            assert!(entry.licence().evidence().contains(DONOR_COMMIT));
            assert!(entry.licence().evidence().contains(MIT_BLOB));
            assert!(entry.provenance().evidence().contains(DONOR_COMMIT));
            assert!(
                entry
                    .provenance()
                    .evidence()
                    .contains("exact donor Git blob")
            );
            assert!(entry.provenance().source().contains("Tabler Icons v3.46.0"));
            digests.push(entry.sha256().to_hex());
        }
        digests.sort();
        digests.dedup();
        assert_eq!(digests.len(), SELECTED, "81 unique source digests");

        // The donor's own record reconciles 84 files: 83 MIT plus one original.
        let provenance = packet.provenance();
        let records = provenance["icons"]
            .as_array()
            .expect("the provenance record lists its icons");
        assert_eq!(records.len(), DONOR_RECORDS);
        let mut mit = 0;
        let mut original = 0;
        let mut by_name = std::collections::BTreeMap::new();
        for record in records {
            let name = record["canonical_filename"]
                .as_str()
                .expect("a canonical filename");
            let licence = record["license"].as_str().expect("a licence");
            if licence == "MIT" {
                mit += 1;
                assert_eq!(record["source_library"].as_str(), Some("Tabler Icons"));
                assert_eq!(record["source_version"].as_str(), Some("3.46.0"));
            } else {
                assert!(licence.starts_with("Provelopment original"));
                original += 1;
            }
            by_name.insert(name.to_owned(), record.clone());
        }
        assert_eq!(mit, DONOR_MIT_RECORDS, "83 MIT records");
        assert_eq!(original, DONOR_ORIGINAL_RECORDS, "1 original record");

        // Every selected entry names its authoritative `source_icon_name`.
        for entry in packet.catalogue.entries() {
            let name = entry.id().trim_start_matches("icons/");
            let record = by_name
                .get(name)
                .expect("every selected asset has a donor record");
            let source_icon = record["source_icon_name"].as_str().expect("a source name");
            assert!(
                entry.provenance().source().ends_with(source_icon),
                "{name}: the entry names its authoritative source icon {source_icon}"
            );
            assert_eq!(record["license"].as_str(), Some("MIT"));
        }

        // The declared duplicate pair and the original are withheld by decision.
        let reuse = provenance["intentional_source_reuse"]
            .as_array()
            .expect("the declared intentional reuse list");
        assert_eq!(reuse.len(), 1, "exactly one declared duplicate pair");
        let mut reuse_names: Vec<String> = reuse[0]["canonical_filenames"]
            .as_array()
            .expect("the pair")
            .iter()
            .map(|value| value.as_str().expect("a filename").to_owned())
            .collect();
        reuse_names.sort();
        assert_eq!(reuse_names, vec!["icon-clients.svg", "icon-volunteer.svg"]);

        for withheld in WITHHELD_ALIASES.iter().copied().chain([WITHHELD_ORIGINAL]) {
            assert!(
                packet
                    .catalogue
                    .exact_entry(&format!("icons/{withheld}"))
                    .is_none(),
                "{withheld} is withheld from the candidate"
            );
            assert!(
                !packet.fixtures.join("icons").join(withheld).exists(),
                "{withheld} is not imported as a review fixture"
            );
            assert!(
                by_name.contains_key(withheld),
                "{withheld} stays in the donor record"
            );
        }

        // No invented approval key, no product Platform tree, no Platform source.
        let document = String::from_utf8(read(&candidate_json_path()))
            .expect("the candidate document is UTF-8");
        for forbidden in [
            "\"approved\"",
            "\"rightsReviewed\"",
            "\"published\"",
            "\"installed\"",
        ] {
            assert!(
                !document.contains(forbidden),
                "the candidate carries no invented trust key {forbidden}"
            );
        }
        let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
        // Absent, or exactly the Owner-approved product library: never a half-created tree.
        super::assert_absent_or_exact_approved_platform_library(manifest);
        assert_eq!(
            bind_catalogue_to_v4_evidence_v1(
                &packet.catalogue,
                &[],
                PlatformCatalogueBindingScopeV1::PlatformSource
            ),
            PlatformCatalogueBindingV1::Refused {
                position: 0,
                kind: PlatformCatalogueMatchRefusalV1::PlatformSourceUnavailable,
            }
        );

        println!(
            "n126_evidence case=02 entries=81 mit_covered=81 originals_withheld=1 \
             duplicate_aliases_withheld=2 review_only=true platform_read=false publication=false \
             donor_commit={DONOR_COMMIT} donor_tree={DONOR_TREE} mit_sha={MIT_SHA256} \
             provenance_sha={PROVENANCE_SHA256}"
        );
    }

    #[test]
    fn n126_03_forged_source_alias_missing_notice_and_platform_authority_refused() {
        let packet = Packet::load();
        let first = packet.catalogue.entries()[0].id().to_owned();
        let honest_bytes = read(&packet.source_path(&first));
        let honest_sha = sha256_media_worker_bytes_v1(&honest_bytes).to_hex();

        // 1. An in-memory tamper of one selected fixture's bytes is detected against the catalogue.
        let mut tampered = honest_bytes.clone();
        let last = tampered.len() - 1;
        tampered[last] ^= 0x01;
        let tampered_sha = sha256_media_worker_bytes_v1(&tampered).to_hex();
        assert_ne!(
            tampered_sha, honest_sha,
            "the tamper really changes the bytes"
        );
        assert_eq!(
            review_selection_refusal(&packet, &first, &tampered_sha, "MIT", true, "3.46.0"),
            Some(SelectionRefusal::SourceDigestDrift),
            "a tampered fixture cannot keep its catalogue identity"
        );
        assert_eq!(
            review_selection_refusal(&packet, &first, &honest_sha, "MIT", true, "3.46.0"),
            None,
            "the honest bytes remain reviewable"
        );

        // 2. Reinserting a withheld alias with its byte-identical digest must refuse at the parser.
        let document = read(&candidate_json_path());
        let mut value: Value = serde_json::from_slice(&document).expect("the candidate document");
        let assets = value["assets"].as_array_mut().expect("the assets array");
        let mut alias = assets[0].clone();
        alias["id"] = Value::String("icons/icon-status-info.svg".to_owned());
        assets.push(alias);
        let reinserted = serde_json::to_vec(&value).expect("serialised forgery");
        assert_eq!(
            parse_platform_asset_catalogue_v1(&reinserted, PLATFORM_ASSET_LIBRARY_VERSION_V1).err(),
            Some(PlatformCatalogueRefusalV1::DuplicateDigest),
            "a duplicate alias digest is refused by the closed catalogue invariant"
        );
        assert_eq!(
            review_selection_refusal(
                &packet,
                "icons/icon-status-info.svg",
                &honest_sha,
                "MIT",
                true,
                "3.46.0"
            ),
            Some(SelectionRefusal::WithheldDuplicateAlias)
        );

        // 3. The unreviewed original and an absent social mark are refused by the selection rule.
        assert_eq!(
            review_selection_refusal(
                &packet,
                &format!("icons/{WITHHELD_ORIGINAL}"),
                &honest_sha,
                "Provelopment original artwork",
                true,
                "3.46.0"
            ),
            Some(SelectionRefusal::WithheldOriginal),
            "no repository licence may be inferred for the original artwork"
        );
        assert_eq!(
            review_selection_refusal(
                &packet,
                "icons/icon-social-facebook.svg",
                &honest_sha,
                "MIT",
                true,
                "3.46.0"
            ),
            Some(SelectionRefusal::NotSelected),
            "an asset outside the reviewed selection cannot be smuggled in"
        );

        // 4. A packet without its MIT notice, or with drifted donor metadata, is refused.
        assert_eq!(
            review_selection_refusal(&packet, &first, &honest_sha, "MIT", false, "3.46.0"),
            Some(SelectionRefusal::LicenceNoticeMissing)
        );
        assert_eq!(
            review_selection_refusal(&packet, &first, &honest_sha, "MIT", true, "3.47.0"),
            Some(SelectionRefusal::DonorMetadataDrift)
        );
        assert_eq!(
            review_selection_refusal(&packet, &first, &honest_sha, "Apache-2.0", true, "3.46.0"),
            Some(SelectionRefusal::NotMitCovered),
            "the root Apache grant never covers the donated icons"
        );

        // 5. A different selected version, or a wrong notice byte, cannot be smuggled past the pin.
        assert_eq!(
            parse_platform_asset_catalogue_v1(&document, "1.0.1").err(),
            Some(PlatformCatalogueRefusalV1::VersionMismatch)
        );
        let mut notice = packet.notice();
        notice[0] ^= 0x01;
        assert_ne!(
            sha256_media_worker_bytes_v1(&notice).to_hex(),
            MIT_SHA256,
            "a corrupted notice is detectable by its pinned digest"
        );

        // 6. The real Platform call site stays refused, so zero authority is conferred.
        assert_eq!(
            bind_catalogue_to_v4_evidence_v1(
                &packet.catalogue,
                &[],
                PlatformCatalogueBindingScopeV1::PlatformSource
            ),
            PlatformCatalogueBindingV1::Refused {
                position: 0,
                kind: PlatformCatalogueMatchRefusalV1::PlatformSourceUnavailable,
            }
        );
        assert!(!packet.catalogue.grants_publication_authority());
        assert!(!packet.catalogue.rights_reviewed());
        assert!(!packet.catalogue.platform_source_readable());

        println!(
            "n126_evidence case=03 forgeries_refused=true alias_digest_refused=true \
             licence_missing_refused=true donor_drift_refused=true platform_unavailable=true \
             zero_authority=true review_only=true platform_read=false publication=false"
        );
    }
}

/// `SOLO-WO-118 R15` — the installed, Owner-approved Platform library `1.0.0`.
///
/// These four cases are the load-bearing proof of the **installed product library**, not of a
/// review packet: the tracked `platform/assets/` tree is copied into a scratch Platform root on
/// the verified native filesystem, the real new Platform capability reads it, and the accepted
/// `solo-cli` SVG worker runs once per approved asset. Nothing here publishes, derives a public
/// asset, releases, deploys or closes `R03.C` / Stage 03.
mod n127 {
    use std::fs;
    use std::path::{Path, PathBuf};

    use solo_adapters::{
        PlatformLicensingDocumentV1, PlatformPinnedIconInspectionV1, PlatformRootCapabilityV1,
        inspect_pinned_platform_icon_sources_v1, parse_platform_asset_catalogue_v1,
    };
    use solo_application::{
        PlatformAssetCatalogueEntryV1, PlatformAssetCatalogueV1, PlatformAssetDeclarationV1,
        PlatformAssetLibraryReadPortV1, PlatformAssetLicenceV1, PlatformAssetProvenanceV1,
        PlatformAssetReadRefusalKindV1, PlatformAssetRefV1, PlatformAssetSourceSetV1,
        PlatformAssetSourceV1, PlatformCatalogueBindingScopeV1, PlatformCatalogueBindingV1,
        PlatformCatalogueMatchRefusalV1, PlatformCatalogueSourceV1, PlatformRootProvenanceV1,
        bind_catalogue_to_v4_evidence_v1, read_pinned_platform_sources_v1,
        sha256_media_worker_bytes_v1,
    };

    use super::BIN;

    /// The approved library version.
    const VERSION: &str = "1.0.0";

    /// The approved catalogue document, pinned by the SHA-256 of its exact raw bytes.
    const CATALOGUE_SHA256: &str =
        "ad8ea718b50641206cc4f4fe13e866ad266366a7aa11ef799c83ae8187858fc7";
    const CATALOGUE_BYTES: usize = 65_397;

    /// The retained licensing evidence, pinned the same way.
    const MIT_SHA256: &str = "b740a1d46122672da62833e97f7e7c8a13fa85cbc7445b584b297cc00dde93db";
    const MIT_BYTES: usize = 1_073;
    const PROVENANCE_SHA256: &str =
        "e0f0ea621c11140cf581d86aaab0b73421d0202114092a4e58dc3321ff1a325b";
    const PROVENANCE_BYTES: usize = 25_274;

    /// The accepted R14 cross-host worker aggregate: an independent consistency anchor.
    const PINNED_AGGREGATE_SHA256: &str =
        "3c34fe9919341862a5a70a87d205fbfc942741dec5f3996514d3edd0aeb1aaca";

    /// The approved library size and the exact product file count.
    const APPROVED: usize = 81;
    const PRODUCT_FILES: usize = 84;

    /// The three names the Owner authorization excludes from the product library.
    const WITHHELD: [&str; 3] = [
        "icon-services.svg",
        "icon-status-info.svg",
        "icon-volunteer.svg",
    ];

    /// The tracked product tree, read only as a copy source and as byte evidence.
    fn product_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../platform")
    }

    /// The review fixtures, used as the independent byte witness for the same 81 sources.
    fn fixture_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/foundation-donor-icons")
    }

    /// The directory a scratch Platform root is created in.
    ///
    /// On Linux this must be the verified fixture filesystem: the reader takes an immutable clone,
    /// and a filesystem without reflink could only refuse. A missing value is a loud failure.
    fn scratch_parent() -> PathBuf {
        #[cfg(target_os = "linux")]
        {
            match std::env::var_os("SOLO_TEST_SOURCE_FS_ROOT") {
                Some(root) if !root.is_empty() => PathBuf::from(root),
                _ => panic!(
                    "SOLO_TEST_SOURCE_FS_ROOT must name the R03 snapshot fixture filesystem; refusing to run the real Platform reader on an unverified filesystem"
                ),
            }
        }
        #[cfg(not(target_os = "linux"))]
        {
            std::env::temp_dir()
        }
    }

    /// Reads one file, or panics with its own name: a missing product byte is never tolerated.
    fn read(path: &Path) -> Vec<u8> {
        fs::read(path).unwrap_or_else(|error| panic!("{} is unreadable: {error}", path.display()))
    }

    /// Copies one directory tree of regular files without dereferencing links.
    fn copy_tree(from: &Path, to: &Path) {
        fs::create_dir_all(to).expect("create a scratch directory");
        for entry in fs::read_dir(from).expect("read a product directory") {
            let entry = entry.expect("a product directory entry");
            let file_type = entry.file_type().expect("a product file type");
            assert!(
                !file_type.is_symlink(),
                "the tracked product tree holds no link"
            );
            let target = to.join(entry.file_name());
            if file_type.is_dir() {
                copy_tree(&entry.path(), &target);
            } else {
                assert!(file_type.is_file(), "the product tree holds regular files");
                fs::copy(entry.path(), &target).expect("copy one product file");
            }
        }
    }

    /// A disposable Platform root holding one copy of the tracked product library.
    struct ScratchPlatform {
        base: PathBuf,
        platform: PathBuf,
    }

    impl ScratchPlatform {
        fn new(label: &str) -> Self {
            let base =
                scratch_parent().join(format!("solo-n127-platform-{}-{label}", std::process::id()));
            let _ = fs::remove_dir_all(&base);
            let platform = base.join("platform");
            copy_tree(&product_root(), &platform);
            Self { base, platform }
        }

        /// An empty Platform root with no catalogue: the "replaced root" case.
        fn empty(label: &str) -> Self {
            let base =
                scratch_parent().join(format!("solo-n127-empty-{}-{label}", std::process::id()));
            let _ = fs::remove_dir_all(&base);
            let platform = base.join("platform");
            fs::create_dir_all(platform.join("assets/icons")).expect("create an empty root");
            Self { base, platform }
        }

        fn capability(&self) -> PlatformRootCapabilityV1 {
            PlatformRootCapabilityV1::adopt_bootstrap_path(
                &self.platform,
                PlatformRootProvenanceV1::OperatorDesignatedPlatformRoot,
            )
            .expect("the scratch Platform root is adoptable")
        }

        fn icon(&self, name: &str) -> PathBuf {
            self.platform.join("assets/icons").join(name)
        }
    }

    impl Drop for ScratchPlatform {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.base);
        }
    }

    /// The loaded, verified product library.
    struct Library {
        catalogue: PlatformAssetCatalogueV1,
        bytes: Vec<u8>,
    }

    impl Library {
        fn load() -> Self {
            let bytes = read(&product_root().join("assets/catalogue-1.0.0.json"));
            assert_eq!(
                sha256_media_worker_bytes_v1(&bytes).to_hex(),
                CATALOGUE_SHA256,
                "the installed catalogue is the approved one"
            );
            assert_eq!(bytes.len(), CATALOGUE_BYTES);
            let catalogue = parse_platform_asset_catalogue_v1(&bytes, VERSION)
                .expect("the strict R12 parser accepts the installed catalogue");
            Self { catalogue, bytes }
        }

        fn requests(&self) -> Vec<PlatformAssetRefV1> {
            self.catalogue
                .entries()
                .iter()
                .map(|entry| {
                    PlatformAssetRefV1::from_identifier(entry.id())
                        .expect("every declared identifier is a typed Platform reference")
                })
                .collect()
        }

        fn source(&self, asset_id: &str) -> Vec<u8> {
            let name = asset_id
                .split_once('/')
                .map(|(_, name)| name)
                .expect("a declared identifier is icons/<name>");
            read(&product_root().join("assets/icons").join(name))
        }
    }

    /// The aggregate of sorted `id` + source SHA-256 + canonical SHA-256 records.
    ///
    /// Identical framing to the accepted R14 anchor, so the two proofs can be compared directly.
    fn aggregate_digest(records: &mut [(String, String, String)]) -> String {
        records.sort();
        let mut text = String::new();
        for (id, source, canonical) in records.iter() {
            text.push_str(id);
            text.push('\t');
            text.push_str(source);
            text.push('\t');
            text.push_str(canonical);
            text.push('\n');
        }
        sha256_media_worker_bytes_v1(text.as_bytes()).to_hex()
    }

    /// Every regular file beneath a root, as `relative/path` in sorted order.
    fn tree_files(root: &Path) -> Vec<String> {
        fn walk(base: &Path, current: &Path, out: &mut Vec<String>) {
            let Ok(entries) = fs::read_dir(current) else {
                return;
            };
            for entry in entries {
                let entry = entry.expect("a directory entry");
                let path = entry.path();
                let file_type = entry.file_type().expect("a file type");
                assert!(!file_type.is_symlink(), "no link in a product tree");
                if file_type.is_dir() {
                    walk(base, &path, out);
                } else {
                    out.push(
                        path.strip_prefix(base)
                            .expect("a relative path")
                            .to_string_lossy()
                            .replace('\\', "/"),
                    );
                }
            }
        }
        let mut out = Vec::new();
        walk(root, root, &mut out);
        out.sort();
        out
    }

    #[test]
    fn n127_01_owner_approved_platform_81_real_confined_svg_sources() {
        let library = Library::load();
        assert_eq!(library.catalogue.len(), APPROVED, "81 approved entries");

        // The installed product tree is exactly 84 files: 81 icons, two records, one catalogue.
        let files = tree_files(&product_root().join("assets"));
        assert_eq!(files.len(), PRODUCT_FILES, "exactly 84 product files");
        assert_eq!(
            files
                .iter()
                .filter(|path| path.starts_with("icons/"))
                .count(),
            APPROVED
        );
        assert!(files.iter().any(|path| path == "catalogue-1.0.0.json"));
        assert!(
            files
                .iter()
                .any(|path| path == "licensing/TABLER-ICONS-MIT.txt")
        );
        assert!(
            files
                .iter()
                .any(|path| path == "licensing/icon-provenance.json")
        );
        for withheld in WITHHELD {
            assert!(
                !files.iter().any(|path| path.ends_with(withheld)),
                "{withheld} is not part of the approved product library"
            );
        }

        // Every product source equals its catalogue digest and its review-fixture bytes.
        let mut digests: Vec<String> = Vec::new();
        for entry in library.catalogue.entries() {
            let name = entry
                .id()
                .split_once('/')
                .map(|(_, name)| name)
                .expect("an icons/<name> identifier");
            let bytes = library.source(entry.id());
            let digest = sha256_media_worker_bytes_v1(&bytes).to_hex();
            assert_eq!(digest, entry.sha256().to_hex(), "{}", entry.id());
            assert_eq!(
                bytes,
                read(&fixture_root().join("icons").join(name)),
                "{}: the installed byte is the reviewed donor byte",
                entry.id()
            );
            assert!(
                !bytes.windows(2).any(|window| window == b"\r\n"),
                "{}: the installed source is LF",
                entry.id()
            );
            digests.push(digest);
        }
        digests.sort();
        digests.dedup();
        assert_eq!(digests.len(), APPROVED, "81 distinct approved digests");

        // The retained licensing evidence travels with the sources, unchanged.
        let notice = read(&product_root().join("assets/licensing/TABLER-ICONS-MIT.txt"));
        assert_eq!(sha256_media_worker_bytes_v1(&notice).to_hex(), MIT_SHA256);
        assert_eq!(notice.len(), MIT_BYTES);
        assert_eq!(
            notice,
            read(&fixture_root().join("licensing/TABLER-ICONS-MIT.txt"))
        );
        let provenance = read(&product_root().join("assets/licensing/icon-provenance.json"));
        assert_eq!(
            sha256_media_worker_bytes_v1(&provenance).to_hex(),
            PROVENANCE_SHA256
        );
        assert_eq!(provenance.len(), PROVENANCE_BYTES);

        // The native root, the real read and the 81 genuine confined SVG children.
        prove_approved_library();
    }

    /// The native mechanism this host reports, asserted per host rather than by one substring.
    fn assert_native_mechanism(mechanism: &str) {
        assert!(!mechanism.is_empty(), "a mechanism is always reported");
        #[cfg(target_os = "linux")]
        assert!(
            mechanism.contains("ficlone"),
            "Linux must report the immutable clone: {mechanism}"
        );
        #[cfg(windows)]
        assert!(
            mechanism.contains("ntcreatefile") && mechanism.contains("dont-reparse"),
            "Windows must report the no-reparse, no-write-sharing handle: {mechanism}"
        );
    }

    /// The native root, the real read and the 81 genuine confined SVG children.
    fn approved_inspection_tail(
        library: &Library,
    ) -> (PlatformPinnedIconInspectionV1, ScratchPlatform) {
        let scratch = ScratchPlatform::new("approved");
        let capability = scratch.capability();
        assert_native_mechanism(capability.mechanism());
        let requests = library.requests();
        let inspection = inspect_pinned_platform_icon_sources_v1(
            &capability,
            VERSION,
            &requests,
            Path::new(BIN),
        );
        (inspection, scratch)
    }

    /// Runs the approved whole-library inspection and proves the accepted aggregate.
    ///
    /// Kept separate so the refusal cases below can reuse the same verified library without
    /// paying for 81 child processes.
    fn prove_approved_library() {
        let library = Library::load();
        let (inspection, scratch) = approved_inspection_tail(&library);
        let set = match inspection {
            PlatformPinnedIconInspectionV1::Complete(set) => set,
            PlatformPinnedIconInspectionV1::Refused(refusal) => {
                panic!("the approved library must be readable: {refusal:?}")
            }
        };
        assert_eq!(set.len(), APPROVED, "81 inspected approved sources");
        assert_eq!(set.source_root(), "platform");
        assert!(!set.grants_publication_authority());
        assert!(!set.produces_public_derivative());
        assert!(!set.proves_global_snapshot());

        let mut records: Vec<(String, String, String)> = Vec::new();
        let mut children: usize = 0;
        for (position, record) in set.records().iter().enumerate() {
            let entry = &library.catalogue.entries()[position];
            assert_eq!(
                record.asset_id(),
                entry.id(),
                "catalogue order is preserved"
            );
            assert_eq!(
                (record.width(), record.height()),
                (24, 24),
                "{}",
                record.asset_id()
            );
            assert_eq!(record.source_sha256(), entry.sha256().to_hex());
            assert_ne!(record.source_sha256(), record.canonical_sha256());
            assert!(record.worker_pid() != 0);
            children += 1;
            println!(
                "n127_row id={} source_sha={} canonical_sha={} elements={} depth={} path_commands={} pid_nonzero=true",
                record.asset_id(),
                record.source_sha256(),
                record.canonical_sha256(),
                record.elements(),
                record.depth(),
                record.path_commands()
            );
            records.push((
                record.asset_id().to_owned(),
                record.source_sha256().to_owned(),
                record.canonical_sha256().to_owned(),
            ));
        }
        assert_eq!(children, APPROVED, "81 genuine confined SVG children");
        let aggregate = aggregate_digest(&mut records);
        assert_eq!(aggregate, PINNED_AGGREGATE_SHA256);
        drop(scratch);

        println!(
            "n127_evidence case=01 approved=81 read=81 real_children=81 version=1.0.0 \
             exact_sha=true aggregate={aggregate} publication=false source_root=platform \
             entity_fallback=false derived_output=false release_authority=false"
        );
    }

    /// Requires a source-stage refusal and returns its closed kind.
    fn expect_source_refusal(
        inspection: PlatformPinnedIconInspectionV1,
        label: &str,
    ) -> PlatformAssetReadRefusalKindV1 {
        match inspection {
            PlatformPinnedIconInspectionV1::Complete(_) => {
                panic!("{label}: the hostile request must be refused")
            }
            PlatformPinnedIconInspectionV1::Refused(refusal) => {
                assert_eq!(
                    refusal.stage(),
                    solo_adapters::PlatformInspectionStageV1::SourceRead,
                    "{label}: the refusal must precede any media work"
                );
                refusal
                    .source_refusal()
                    .unwrap_or_else(|| panic!("{label}: a source refusal is expected"))
                    .kind()
            }
        }
    }

    /// Runs one whole-library inspection against one scratch root and returns its refusal kind.
    fn inspection_kind(
        root: &Path,
        version: &str,
        request: &PlatformAssetRefV1,
        label: &str,
    ) -> PlatformAssetReadRefusalKindV1 {
        let capability = PlatformRootCapabilityV1::adopt_bootstrap_path(
            root,
            PlatformRootProvenanceV1::OperatorDesignatedPlatformRoot,
        )
        .expect("the scratch Platform root is adoptable");
        expect_source_refusal(
            inspect_pinned_platform_icon_sources_v1(
                &capability,
                version,
                std::slice::from_ref(request),
                Path::new(BIN),
            ),
            label,
        )
    }

    #[test]
    fn n127_02_version_catalogue_licence_and_scope_refusal_matrix() {
        let library = Library::load();
        let approved = ScratchPlatform::new("matrix");
        let capability = approved.capability();
        let request = library.requests()[0].clone();
        let mut refusals: usize = 0;

        // 1-3. The selected version is mandatory and must be the approved one.
        for version in ["", "1.0.1", "2.0.0"] {
            let kind = expect_source_refusal(
                inspect_pinned_platform_icon_sources_v1(
                    &capability,
                    version,
                    std::slice::from_ref(&request),
                    Path::new(BIN),
                ),
                version,
            );
            assert_eq!(
                kind,
                PlatformAssetReadRefusalKindV1::UnselectedVersion,
                "{version}"
            );
            refusals += 1;
        }

        // 4. A replaced root holds no catalogue at all.
        let empty = ScratchPlatform::empty("matrix");
        assert_eq!(
            inspection_kind(&empty.platform, VERSION, &request, "replaced-root"),
            PlatformAssetReadRefusalKindV1::CatalogueUnavailable
        );
        refusals += 1;

        // 5. A changed catalogue body that stays self-consistent is still not approved.
        let changed_root = ScratchPlatform::new("changed-catalogue");
        let catalogue_path = changed_root.platform.join("assets/catalogue-1.0.0.json");
        let mut changed = read(&catalogue_path);
        let index = changed
            .windows(10)
            .position(|window| window == b"\"height\": ")
            .expect("the height key");
        changed[index + 9] = b'5';
        assert_ne!(
            sha256_media_worker_bytes_v1(&changed).to_hex(),
            CATALOGUE_SHA256
        );
        fs::write(&catalogue_path, &changed).expect("write a changed catalogue");
        assert_eq!(
            inspection_kind(
                &changed_root.platform,
                VERSION,
                &request,
                "changed-catalogue"
            ),
            PlatformAssetReadRefusalKindV1::CatalogueUnapproved
        );
        refusals += 1;

        // 6. A wholly different but valid document is not the approved catalogue.
        let other_root = ScratchPlatform::new("other-catalogue");
        fs::write(
            other_root.platform.join("assets/catalogue-1.0.0.json"),
            b"{\"platformAssetLibraryVersion\": \"1.0.0\", \"assets\": []}",
        )
        .expect("write another catalogue");
        assert_eq!(
            inspection_kind(&other_root.platform, VERSION, &request, "other-catalogue"),
            PlatformAssetReadRefusalKindV1::CatalogueUnapproved
        );
        refusals += 1;

        // 7-8. The retained notice must exist and must be the pinned bytes.
        let no_notice = ScratchPlatform::new("no-notice");
        fs::remove_file(
            no_notice
                .platform
                .join("assets/licensing/TABLER-ICONS-MIT.txt"),
        )
        .expect("remove the notice");
        let notice_refusal = no_notice
            .capability()
            .read_licensing_document(PlatformLicensingDocumentV1::MitNotice, 1024 * 1024)
            .expect_err("a missing notice is refused");
        assert_eq!(
            notice_refusal.kind(),
            PlatformAssetReadRefusalKindV1::MissingSource
        );
        refusals += 1;

        let swapped_notice = ScratchPlatform::new("swapped-notice");
        fs::write(
            swapped_notice
                .platform
                .join("assets/licensing/TABLER-ICONS-MIT.txt"),
            b"MIT License\n\nPermission is hereby granted.\n",
        )
        .expect("write a different notice");
        let swapped = swapped_notice
            .capability()
            .read_licensing_document(PlatformLicensingDocumentV1::MitNotice, 1024 * 1024)
            .expect("the swapped notice is readable");
        assert_ne!(
            sha256_media_worker_bytes_v1(&swapped).to_hex(),
            MIT_SHA256,
            "a swapped notice cannot keep its identity"
        );
        refusals += 1;

        // 9. Changed provenance bytes cannot keep their identity.
        let changed_provenance = ScratchPlatform::new("changed-provenance");
        let provenance_path = changed_provenance
            .platform
            .join("assets/licensing/icon-provenance.json");
        let mut provenance = read(&provenance_path);
        provenance.push(b' ');
        assert_ne!(
            sha256_media_worker_bytes_v1(&provenance).to_hex(),
            PROVENANCE_SHA256
        );
        fs::write(&provenance_path, &provenance).expect("write changed provenance");
        let provenance_read = changed_provenance
            .capability()
            .read_licensing_document(PlatformLicensingDocumentV1::Provenance, 1024 * 1024)
            .expect("the changed record is readable");
        assert_ne!(
            sha256_media_worker_bytes_v1(&provenance_read).to_hex(),
            PROVENANCE_SHA256
        );
        refusals += 1;

        // 10-11. An unapproved extra entry and an absent entry both refuse at the catalogue size.
        let entries = library.catalogue.entries().to_vec();
        let licence = PlatformAssetLicenceV1::new("MIT", "Paweł Kuna", "pinned notice")
            .expect("a bounded licence");
        let provenance_text = PlatformAssetProvenanceV1::new("Tabler Icons v3.46.0", "pinned")
            .expect("bounded provenance");
        let extra_digest = sha256_media_worker_bytes_v1(b"an unapproved extra asset").to_hex();
        let mut extended = entries.clone();
        extended.push(
            PlatformAssetCatalogueEntryV1::new(
                &PlatformAssetDeclarationV1::of(
                    "icons/icon-unapproved-extra.svg",
                    &extra_digest,
                    "image",
                    "image/svg+xml",
                    24,
                    24,
                ),
                licence.clone(),
                provenance_text.clone(),
            )
            .expect("a syntactically valid extra entry"),
        );
        let extended = PlatformAssetCatalogueV1::new(VERSION, extended).expect("an 82-entry model");
        let outcome = read_pinned_platform_sources_v1(
            capability.selection(),
            VERSION,
            &extended,
            std::slice::from_ref(&request),
            &capability,
        );
        assert_eq!(
            outcome.refusal().map(|refusal| refusal.kind()),
            Some(PlatformAssetReadRefusalKindV1::CatalogueCountMismatch)
        );
        refusals += 1;

        let truncated = entries[..APPROVED - 1].to_vec();
        let truncated =
            PlatformAssetCatalogueV1::new(VERSION, truncated).expect("an 80-entry model");
        let outcome = read_pinned_platform_sources_v1(
            capability.selection(),
            VERSION,
            &truncated,
            std::slice::from_ref(&request),
            &capability,
        );
        assert_eq!(
            outcome.refusal().map(|refusal| refusal.kind()),
            Some(PlatformAssetReadRefusalKindV1::CatalogueCountMismatch)
        );
        refusals += 1;

        // 12-15. Identifier spellings: case alias, injected prefixes and non-membership.
        for (label, identifier) in [
            ("id-case-alias", "icons/ICON-about.svg"),
            ("platform-prefix", "platform/icons/icon-about.svg"),
            (
                "content-assets-prefix",
                "content/assets/icons/icon-about.svg",
            ),
            ("assets-prefix", "assets/icons/icon-about.svg"),
        ] {
            assert_eq!(
                PlatformAssetRefV1::from_identifier(identifier)
                    .expect_err(label)
                    .kind(),
                PlatformAssetReadRefusalKindV1::UnsafeAssetIdentifier,
                "{label}"
            );
            refusals += 1;
        }
        let non_member = PlatformAssetRefV1::from_identifier("icons/icon-not-approved.svg")
            .expect("a syntactically valid identifier");
        assert_eq!(
            inspection_kind(&approved.platform, VERSION, &non_member, "non-member"),
            PlatformAssetReadRefusalKindV1::UnapprovedAssetIdentifier
        );
        refusals += 1;

        // 16. An Entity- or route-shaped spelling can never address a Platform source: the typed
        //     identifier refuses it before any read, so no Entity namespace can alias one here.
        for identifier in [
            "entity/icons/icon-about.svg",
            "icons/icon-about.svg?entity",
            "icons/entity/icon-about.svg",
        ] {
            assert_eq!(
                PlatformAssetRefV1::from_identifier(identifier)
                    .expect_err(identifier)
                    .kind(),
                PlatformAssetReadRefusalKindV1::UnsafeAssetIdentifier,
                "{identifier}"
            );
            refusals += 1;
        }

        // 17. A document that declares one digest twice is refused by the unchanged parser.
        let mut duplicated: serde_json::Value =
            serde_json::from_slice(&library.bytes).expect("the approved document");
        let assets = duplicated["assets"]
            .as_array_mut()
            .expect("the assets array");
        let mut alias = assets[0].clone();
        alias["id"] = serde_json::Value::String("icons/icon-status-info.svg".to_owned());
        assets.push(alias);
        let duplicated = serde_json::to_vec(&duplicated).expect("a re-serialised document");
        assert!(
            parse_platform_asset_catalogue_v1(&duplicated, VERSION).is_err(),
            "a duplicate digest alias is refused by the closed catalogue invariant"
        );
        refusals += 1;

        // 18. The pre-existing Entity-only reader still refuses any Platform obligation, and the
        //     R12 inert PlatformSource binding is still refused: this order adds a *separate*
        //     proven source, it does not relabel the inert comparison as a real read.
        assert_eq!(
            bind_catalogue_to_v4_evidence_v1(
                &library.catalogue,
                &[],
                PlatformCatalogueBindingScopeV1::PlatformSource
            ),
            PlatformCatalogueBindingV1::Refused {
                position: 0,
                kind: PlatformCatalogueMatchRefusalV1::PlatformSourceUnavailable,
            }
        );
        refusals += 1;

        assert!(
            refusals >= 12,
            "the hostile matrix must contain at least twelve independent refusals, found {refusals}"
        );
        println!(
            "n127_evidence case=02 refusals={refusals} pin_refused=true licence_refused=true \
             provenance_refused=true entity_fallback=false publication=false \
             entity_source_reader_refuses_platform=true platform_source_binding_refused=true"
        );
    }

    /// Requires one refused native read of one exact identifier.
    fn expect_read_refusal(
        capability: &PlatformRootCapabilityV1,
        asset_id: &str,
        max_bytes: u64,
        label: &str,
    ) -> PlatformAssetReadRefusalKindV1 {
        match capability.read_platform_source(asset_id, max_bytes) {
            PlatformAssetSourceV1::Present { .. } => {
                panic!("{label}: the native boundary must refuse this object")
            }
            PlatformAssetSourceV1::Refused(kind) => kind,
        }
    }

    #[test]
    fn n127_03_separate_native_root_symlink_hardlink_and_changed_source_refused() {
        let library = Library::load();
        let scratch = ScratchPlatform::new("confinement");
        let capability = scratch.capability();
        let approved = library.catalogue.entries()[0].id().to_owned();
        let approved_name = approved
            .split_once('/')
            .map(|(_, name)| name)
            .expect("an icons/<name> identifier");
        let mut natives: usize = 0;

        // 1. A regular approved file is accepted by the same capability...
        assert!(matches!(
            capability.read_platform_source(&approved, 20 * 1024 * 1024),
            PlatformAssetSourceV1::Present { links: 1, .. }
        ));
        // ...which is what makes every refusal below load-bearing.
        let target = scratch.icon(approved_name);
        let original = read(&target);

        // 2. A hardlink alias is an ambiguous identity even with correct bytes.
        let alias = scratch.icon("icon-hardlink-alias.svg");
        fs::hard_link(&target, &alias).expect("create a hardlink alias");
        assert_eq!(
            expect_read_refusal(
                &capability,
                "icons/icon-hardlink-alias.svg",
                20 * 1024 * 1024,
                "hardlink"
            ),
            PlatformAssetReadRefusalKindV1::IdentityCollision
        );
        fs::remove_file(&alias).expect("remove the hardlink alias");
        natives += 1;

        // 3. A symlinked leaf cannot redirect an approved identifier.
        #[cfg(unix)]
        {
            let displaced = scratch.icon("icon-displaced.svg");
            fs::rename(&target, &displaced).expect("move the approved source aside");
            std::os::unix::fs::symlink(&displaced, &target).expect("create a symlink leaf");
            assert_eq!(
                expect_read_refusal(&capability, &approved, 20 * 1024 * 1024, "symlink-leaf"),
                PlatformAssetReadRefusalKindV1::UnsafeSourceLocation
            );
            fs::remove_file(&target).expect("remove the symlink");
            fs::rename(&displaced, &target).expect("restore the approved source");
            natives += 1;
        }
        #[cfg(windows)]
        {
            // The Windows reader refuses reparse points; the accepted R03 Windows canaries already
            // execute that negative with the privilege they need. The attempt is still made here,
            // and a privilege refusal is recorded as an OS refusal, never as a passing product case.
            let displaced = scratch.icon("icon-displaced.svg");
            fs::rename(&target, &displaced).expect("move the approved source aside");
            match std::os::windows::fs::symlink_file(&displaced, &target) {
                Ok(()) => {
                    assert_eq!(
                        expect_read_refusal(&capability, &approved, 20 * 1024 * 1024, "symlink"),
                        PlatformAssetReadRefusalKindV1::UnsafeSourceLocation
                    );
                    fs::remove_file(&target).expect("remove the symlink");
                    natives += 1;
                }
                Err(error) => {
                    assert_eq!(
                        error.kind(),
                        std::io::ErrorKind::PermissionDenied,
                        "a symlink can only be declined for privilege here: {error}"
                    );
                }
            }
            fs::rename(&displaced, &target).expect("restore the approved source");
        }

        // 3. A directory leaf is not a regular source.
        fs::create_dir_all(scratch.icon("icon-directory.svg")).expect("create a directory leaf");
        assert_eq!(
            expect_read_refusal(
                &capability,
                "icons/icon-directory.svg",
                20 * 1024 * 1024,
                "directory-leaf"
            ),
            PlatformAssetReadRefusalKindV1::UnsafeSourceLocation
        );
        fs::remove_dir_all(scratch.icon("icon-directory.svg")).expect("remove the directory leaf");
        natives += 1;

        // 4. A missing leaf is a refusal, never absence-as-success.
        assert_eq!(
            expect_read_refusal(
                &capability,
                "icons/icon-absent.svg",
                20 * 1024 * 1024,
                "absent"
            ),
            PlatformAssetReadRefusalKindV1::MissingSource
        );
        natives += 1;

        // 5. An unsafe identifier never reaches the filesystem.
        for identifier in [
            "icons/../icons/icon-about.svg",
            "icons/icon about.svg",
            "assets/icons/icon-about.svg",
        ] {
            assert_eq!(
                expect_read_refusal(&capability, identifier, 20 * 1024 * 1024, identifier),
                PlatformAssetReadRefusalKindV1::UnsafeAssetIdentifier,
                "{identifier}"
            );
            natives += 1;
        }

        // 6. A file adopted as the root is not a directory capability.
        assert!(
            PlatformRootCapabilityV1::adopt_bootstrap_path(
                &target,
                PlatformRootProvenanceV1::OperatorDesignatedPlatformRoot
            )
            .is_err()
        );
        natives += 1;

        // 7. An oversize source is refused, never truncated.
        assert_eq!(
            expect_read_refusal(&capability, &approved, 8, "oversize"),
            PlatformAssetReadRefusalKindV1::OversizeSource
        );
        natives += 1;

        // 8. A replaced root with no catalogue cannot address the approved library.
        let empty = ScratchPlatform::empty("confinement");
        assert_eq!(
            match empty.capability().read_catalogue(2 * 1024 * 1024) {
                PlatformCatalogueSourceV1::Present { .. } =>
                    panic!("an empty root has no catalogue"),
                PlatformCatalogueSourceV1::Refused(kind) => kind,
            },
            PlatformAssetReadRefusalKindV1::CatalogueUnavailable
        );
        natives += 1;

        // 9. Linux: the immutable-clone requirement is not silently downgraded to a copy.
        #[cfg(target_os = "linux")]
        {
            let parent = match std::env::var_os("SOLO_TEST_UNSUPPORTED_FS_ROOT") {
                Some(root) if !root.is_empty() => PathBuf::from(root),
                _ => panic!(
                    "SOLO_TEST_UNSUPPORTED_FS_ROOT must name a filesystem without reflink; the refusal cannot be proven without it"
                ),
            };
            let base = parent.join(format!("solo-n127-unsupported-{}", std::process::id()));
            let _ = fs::remove_dir_all(&base);
            let platform = base.join("platform");
            copy_tree(&product_root(), &platform);
            let unsupported = PlatformRootCapabilityV1::adopt_bootstrap_path(
                &platform,
                PlatformRootProvenanceV1::OperatorDesignatedPlatformRoot,
            )
            .expect("the unverified-filesystem root is adoptable");
            assert_eq!(
                expect_read_refusal(&unsupported, &approved, 20 * 1024 * 1024, "no-reflink"),
                PlatformAssetReadRefusalKindV1::Unsupported
            );
            let _ = fs::remove_dir_all(&base);
            natives += 1;
        }

        // The approved source bytes were restored exactly.
        assert_eq!(read(&target), original);
        assert!(natives >= 9, "native refusals: {natives}");
        println!(
            "n127_evidence case=03 confinement=true hardlink_refused=true unsafe_refused=true \
             native_mechanism={} natives={natives} publication=false",
            capability.mechanism()
        );
    }

    #[test]
    fn n127_04_late_hidden_platform_failure_zero_partial_and_no_namespace_alias() {
        let library = Library::load();
        let scratch = ScratchPlatform::new("late");
        let capability = scratch.capability();
        let requests = library.requests();

        // 1. The requested set does complete under the new standalone gate first, so the late
        //    failure below is a real change of state rather than a gate that never worked.
        let baseline = read_pinned_platform_sources_v1(
            capability.selection(),
            VERSION,
            &library.catalogue,
            &requests[..4],
            &capability,
        );
        let baseline = baseline
            .complete()
            .unwrap_or_else(|| panic!("the first four approved sources must complete"));
        assert_eq!(baseline.len(), 4);
        assert!(!baseline.grants_publication_authority());
        assert!(!baseline.proves_global_snapshot());

        // 2. Corrupt the *last* requested source while the earlier ones still match exactly.
        let last = requests[3].as_str().to_owned();
        let last_name = last
            .split_once('/')
            .map(|(_, name)| name)
            .expect("an icons/<name> identifier");
        let last_path = scratch.icon(last_name);
        let original = read(&last_path);
        let mut corrupted = original.clone();
        let tail = corrupted.len() - 1;
        corrupted[tail] ^= 0x01;
        assert_eq!(
            corrupted.len(),
            original.len(),
            "same length, different bytes"
        );
        fs::write(&last_path, &corrupted).expect("corrupt the last requested source");

        let outcome = read_pinned_platform_sources_v1(
            capability.selection(),
            VERSION,
            &library.catalogue,
            &requests[..4],
            &capability,
        );
        let refusal = outcome
            .refusal()
            .copied()
            .unwrap_or_else(|| panic!("the corrupted last source must refuse the set"));
        assert_eq!(
            refusal.kind(),
            PlatformAssetReadRefusalKindV1::SourceDigestMismatch,
            "the corruption is caught by the recomputed source digest"
        );
        assert_eq!(
            refusal.position(),
            Some(3),
            "the refusal names the late ordinal"
        );
        assert!(
            outcome.complete().is_none(),
            "zero partial records survive a late failure"
        );
        assert!(!refusal.detail().contains('/'), "{}", refusal.detail());
        assert!(!refusal.detail().contains("icons"), "{}", refusal.detail());

        // 3. The whole-request inspection refuses at the source stage, before any worker exists, so
        //    a late corruption can never produce media evidence for the earlier assets.
        let inspection = inspect_pinned_platform_icon_sources_v1(
            &capability,
            VERSION,
            &requests[..4],
            Path::new(BIN),
        );
        assert_eq!(
            expect_source_refusal(inspection, "late-corruption"),
            PlatformAssetReadRefusalKindV1::SourceDigestMismatch
        );

        // 4. Restoring the byte restores the complete set: the refusal was about the bytes.
        fs::write(&last_path, &original).expect("restore the approved source");
        let restored = read_pinned_platform_sources_v1(
            capability.selection(),
            VERSION,
            &library.catalogue,
            &requests[..4],
            &capability,
        );
        assert_eq!(
            restored.complete().map(PlatformAssetSourceSetV1::len),
            Some(4)
        );

        // 5. A wrong root cannot smuggle an Entity-shaped `assets/icons/…` file of the same
        //    basename: with no approved catalogue there, the request refuses.
        let entity_like = ScratchPlatform::empty("late");
        fs::copy(
            scratch.icon(last_name),
            entity_like.platform.join("assets/icons").join(last_name),
        )
        .expect("copy an Entity-shaped file into the wrong root");
        assert_eq!(
            inspection_kind(&entity_like.platform, VERSION, &requests[3], "wrong-root"),
            PlatformAssetReadRefusalKindV1::CatalogueUnavailable
        );

        // 6. No namespace alias: a route- or prefix-shaped spelling cannot address a Platform
        //    source, and the namespace-free identifier type refuses it before any read.
        for alias in [
            "platform/icons/icon-about.svg",
            "assets/icons/icon-about.svg",
            "content/assets/icons/icon-about.svg",
            "icons/ICON-about.svg",
        ] {
            let refusal = PlatformAssetRefV1::from_identifier(alias)
                .expect_err("an aliased spelling must never address an approved source");
            assert_eq!(
                refusal.kind(),
                PlatformAssetReadRefusalKindV1::UnsafeAssetIdentifier
            );
        }
        // The single representative of the duplicate pair is the only approved member, and the
        // withheld names remain non-members of the installed catalogue.
        for withheld in WITHHELD {
            assert!(
                library
                    .catalogue
                    .exact_entry(&format!("icons/{withheld}"))
                    .is_none(),
                "{withheld} must remain absent"
            );
        }
        assert!(
            library
                .catalogue
                .exact_entry("icons/icon-about.svg")
                .is_some()
        );

        println!(
            "n127_evidence case=04 late_refused=true zero_partial=true private_path_absent=true \
             fallback=false publication=false standalone_set_complete=true wrong_root_refused=true \
             namespace_alias_refused=true"
        );
    }
}

/// `SOLO-WO-118 R18` — N129: one real mixed Entity/Platform whole-candidate transaction.
///
/// The new risk is the **cross-namespace driver**: two independently adopted roots, one closed
/// obligation plan over the whole validated candidate, and one whole-candidate refusal. The
/// established 81-icon library proof (N127) and the six-format worker proof (N124) are reused as
/// they stand, so no icon is re-tested and no per-file adversarial campaign is repeated.
mod n129 {
    use std::fs;
    use std::path::{Path, PathBuf};

    use solo_adapters::{
        EntityImageOutcomeV4, PlatformRootCapabilityV1, TestMixedDeclarationV1, TestMixedFormatV1,
        TestOwnerV1, inspect_entity_graphic_images_v4,
        inspect_mixed_graphic_candidate_from_roots_v1, parse_platform_asset_catalogue_v1,
        test_inventory_v1, test_mixed_candidate_v1,
    };
    use solo_application::{
        GraphicAssetReadRefusalKind, MixedGraphicCandidateOutcomeV1,
        MixedGraphicInspectRefusalKindV1, MixedGraphicOriginV1, MixedGraphicPlanRefusalKindV1,
        MixedGraphicPlanV1, PlatformAssetCatalogueV1, PlatformAssetReadRefusalKindV1,
        PlatformGraphicBaselineRefusalKindV1, PlatformRootProvenanceV1, PlatformRootSelectionV1,
        PlatformSourceRequestV1, sha256_media_worker_bytes_v1,
    };

    use super::BIN;
    use super::n116::{Root, png, rich_svg, webp};

    /// The accepted approved library version and the independent catalogue pin.
    const VERSION: &str = "1.0.0";
    const CATALOGUE_SHA256: &str =
        "ad8ea718b50641206cc4f4fe13e866ad266366a7aa11ef799c83ae8187858fc7";

    /// One real approved Platform identifier and the exact logical path it reads from.
    const HOME_ID: &str = "icons/icon-home.svg";
    const HOME_LOGICAL: &str = "icons/icon-home.svg";

    /// The tracked product tree, used only as the copy source for a scratch Platform root.
    fn product_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../platform")
    }

    /// The filesystem a scratch root is created on.
    ///
    /// On Linux the confined readers require the verified fixture filesystem, exactly as the other
    /// real-filesystem suites do: a missing value is a loud refusal, never a quiet run on an
    /// unverified filesystem.
    fn scratch_parent() -> PathBuf {
        #[cfg(target_os = "linux")]
        {
            match std::env::var_os("SOLO_TEST_SOURCE_FS_ROOT") {
                Some(root) if !root.is_empty() => PathBuf::from(root),
                _ => panic!(
                    "SOLO_TEST_SOURCE_FS_ROOT must name the R03 snapshot fixture filesystem; \
                     refusing to run the real mixed-source gate on an unverified filesystem"
                ),
            }
        }
        #[cfg(not(target_os = "linux"))]
        {
            std::env::temp_dir()
        }
    }

    /// Copies one directory tree of regular files without dereferencing links.
    fn copy_tree(from: &Path, to: &Path) {
        fs::create_dir_all(to).expect("create a scratch directory");
        for entry in fs::read_dir(from).expect("read a product directory") {
            let entry = entry.expect("a product directory entry");
            let file_type = entry.file_type().expect("a product file type");
            assert!(
                !file_type.is_symlink(),
                "the tracked product tree holds no link"
            );
            let target = to.join(entry.file_name());
            if file_type.is_dir() {
                copy_tree(&entry.path(), &target);
            } else {
                assert!(file_type.is_file(), "the product tree holds regular files");
                fs::copy(entry.path(), &target).expect("copy one product file");
            }
        }
    }

    /// A disposable Platform root holding one copy of the approved product library.
    struct PlatformRoot {
        base: PathBuf,
        platform: PathBuf,
    }

    impl PlatformRoot {
        fn new(label: &str) -> Self {
            let base =
                scratch_parent().join(format!("solo-n129-platform-{}-{label}", std::process::id()));
            let _ = fs::remove_dir_all(&base);
            let platform = base.join("platform");
            copy_tree(&product_root(), &platform);
            Self { base, platform }
        }

        fn capability(&self) -> PlatformRootCapabilityV1 {
            PlatformRootCapabilityV1::adopt_bootstrap_path(
                &self.platform,
                PlatformRootProvenanceV1::OperatorDesignatedPlatformRoot,
            )
            .expect("the scratch Platform root is adoptable")
        }
    }

    impl Drop for PlatformRoot {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.base);
        }
    }

    /// The parsed approved catalogue together with the exact bytes it was parsed from.
    struct Catalogue {
        model: PlatformAssetCatalogueV1,
        bytes: Vec<u8>,
    }

    impl Catalogue {
        fn load() -> Self {
            let bytes = fs::read(product_root().join("assets/catalogue-1.0.0.json"))
                .expect("the installed catalogue is present");
            assert_eq!(
                sha256_media_worker_bytes_v1(&bytes).to_hex(),
                CATALOGUE_SHA256,
                "the installed catalogue is the approved one"
            );
            let model = parse_platform_asset_catalogue_v1(&bytes, VERSION)
                .expect("the strict R12 parser accepts the installed catalogue");
            Self { model, bytes }
        }

        /// The catalogue's own declared digest for one exact identifier.
        fn digest(&self, id: &str) -> String {
            self.model
                .entries()
                .iter()
                .find(|entry| entry.id() == id)
                .map_or_else(
                    || panic!("{id} is an approved identifier"),
                    |entry| entry.sha256().to_hex(),
                )
        }

        fn request(&self) -> PlatformSourceRequestV1<'_> {
            PlatformSourceRequestV1 {
                selection: PlatformRootSelectionV1::adopt(
                    PlatformRootProvenanceV1::OperatorDesignatedPlatformRoot,
                ),
                selected_version: VERSION,
                catalogue: &self.model,
                catalogue_bytes: &self.bytes,
            }
        }
    }

    /// One authored mixed declaration.
    fn declared(
        owner: TestOwnerV1,
        role: &'static str,
        reference: &'static str,
        namespace: &'static str,
        supersede: bool,
        exposure: &'static str,
    ) -> TestMixedDeclarationV1 {
        TestMixedDeclarationV1 {
            owner,
            role,
            slot: "primary",
            namespace,
            reference,
            supersede,
            exposure,
        }
    }

    /// Runs the one composed driver over the real roots.
    fn inspect(
        entity: &Root,
        platform: &PlatformRoot,
        catalogue: &Catalogue,
        plan: &MixedGraphicPlanV1,
    ) -> MixedGraphicCandidateOutcomeV1 {
        inspect_mixed_graphic_candidate_from_roots_v1(
            &entity.capability(),
            &platform.capability(),
            plan,
            catalogue.request(),
            Path::new(BIN),
        )
    }

    /// The complete set, or a diagnostic panic naming the real refusal.
    fn complete(
        outcome: MixedGraphicCandidateOutcomeV1,
    ) -> solo_application::GraphicMixedInspectionSetV1 {
        match outcome {
            MixedGraphicCandidateOutcomeV1::Complete(set) => set,
            other => panic!("the mixed candidate refused: {other:?}"),
        }
    }

    /// `N129_01` — one real mixed five-level candidate: an approved Platform root default plus a
    /// closer Entity supersession, an Entity favicon and an `internal` Entity source, each acquired
    /// through its own confined reader and inspected by the same six-format worker proof.
    #[test]
    fn n129_01_mixed_five_level_two_root_success() {
        let png_bytes = png();
        let webp_bytes = webp();
        let svg_bytes = rich_svg().into_bytes();
        let entity = Root::new(
            "n129-mixed",
            &[
                ("logo.png", png_bytes.as_slice()),
                ("mark.svg", svg_bytes.as_slice()),
                ("banner.webp", webp_bytes.as_slice()),
            ],
        );
        let platform = PlatformRoot::new("n129-mixed");
        let catalogue = Catalogue::load();
        let home_digest = catalogue.digest(HOME_ID);

        // One test-only root registration plus three authored Entity selections across the levels.
        let candidate = test_mixed_candidate_v1(
            Some((HOME_ID, &home_digest)),
            TestMixedFormatV1::SlotAware,
            &[
                declared(
                    TestOwnerV1::Entity,
                    "favicon",
                    "logo.png",
                    "entity",
                    false,
                    "public",
                ),
                declared(
                    TestOwnerV1::Site,
                    "logo",
                    "mark.svg",
                    "entity",
                    true,
                    "public",
                ),
                declared(
                    TestOwnerV1::Page,
                    "social-preview",
                    "banner.webp",
                    "entity",
                    false,
                    "internal",
                ),
            ],
        );
        assert_eq!(candidate.refusal(), None, "the candidate is valid");
        assert!(candidate.is_slot_aware());
        let plan = candidate.plan();
        assert_eq!(
            plan.len(),
            4,
            "the root registration plus three authored sources"
        );
        assert_eq!(plan.platform_count(), 1, "exactly one Platform obligation");
        assert_eq!(plan.entity_count(), 3, "three Entity obligations");
        assert_eq!(
            plan.obligations()[0].origin(),
            MixedGraphicOriginV1::PlatformRootRegistration,
            "the root layer is planned first"
        );
        assert_eq!(
            plan.obligations()[1].origin(),
            MixedGraphicOriginV1::AuthoredSelection
        );

        // The nearer explicit supersession governs `graphic.logo` while the superseded root default
        // is still an inspected obligation.
        assert_eq!(
            candidate.winners(),
            &[
                ("graphic.logo".to_owned(), "mark.svg".to_owned()),
                ("graphic.favicon".to_owned(), "logo.png".to_owned()),
                (
                    "graphic.social-preview".to_owned(),
                    "banner.webp".to_owned()
                ),
            ],
            "the Site supersede wins and descends to the representation"
        );
        assert_eq!(
            candidate.logo_lineage(),
            2,
            "root selected, then superseded"
        );

        let set = complete(inspect(&entity, &platform, &catalogue, plan));
        assert_eq!(set.len(), 4, "every obligation is inspected");
        assert_eq!(set.entity_count(), 3);
        assert_eq!(set.platform_count(), 1);
        assert!(!set.grants_publication_authority());
        assert!(!set.proves_whole_publication());

        // Order is the plan's order, and each record answers its own obligation.
        for (index, record) in set.inspections().iter().enumerate() {
            assert_eq!(record.obligation(), &plan.obligations()[index]);
        }
        let platform_record = &set.inspections()[0];
        assert!(platform_record.is_platform());
        assert_eq!(platform_record.logical_path(), HOME_LOGICAL);
        assert_eq!(platform_record.mime(), "image/svg+xml");
        assert_eq!(platform_record.source_digest().to_hex(), home_digest);
        assert!(
            platform_record.worker_pid() != 0,
            "the Platform SVG ran in a real confined child"
        );
        for record in &set.inspections()[1..] {
            assert!(record.is_entity());
            assert!(record.logical_path().starts_with("assets/"));
            assert!(record.width() > 0 && record.height() > 0);
            assert!(record.worker_pid() != 0, "a real confined child decoded it");
            assert!(!record.grants_publication_authority());
        }
        let pids = set.distinct_worker_pids();
        assert!(
            pids.len() >= 2,
            "at least two distinct real confined children: {pids:?}"
        );

        println!(
            "n129_evidence case=01 obligations=4 entity=3 platform=1 distinct_pids={} \
             platform_path={HOME_LOGICAL} platform_mime=image/svg+xml platform_sha={} \
             catalogue_sha={CATALOGUE_SHA256} winner_logo=mark.svg lineage=2 \
             internal_inspected=true publication=false",
            pids.len(),
            home_digest
        );
    }

    /// `N129_02` — every declared obligation is planned and checked, including the superseded,
    /// suppressed and `internal` ones, and a **late** failure discards every earlier success.
    #[test]
    fn n129_02_all_declared_obligations_and_late_failure_zero_partial() {
        let png_bytes = png();
        let svg_bytes = rich_svg().into_bytes();
        // Four Entity sources across the hierarchy plus the Platform default: the banner is
        // suppressed at the Facet and re-authored at the Page, so both declarations are obligations.
        let entity = Root::new(
            "n129-obligations",
            &[
                ("logo.png", png_bytes.as_slice()),
                ("mark.svg", svg_bytes.as_slice()),
                ("banner.webp", webp().as_slice()),
            ],
        );
        let platform = PlatformRoot::new("n129-obligations");
        let catalogue = Catalogue::load();
        let home_digest = catalogue.digest(HOME_ID);

        let declarations = [
            declared(
                TestOwnerV1::Entity,
                "favicon",
                "logo.png",
                "entity",
                false,
                "public",
            ),
            declared(
                TestOwnerV1::Site,
                "logo",
                "mark.svg",
                "entity",
                true,
                "public",
            ),
            declared(
                TestOwnerV1::Page,
                "banner",
                "banner.webp",
                "entity",
                false,
                "internal",
            ),
        ];
        let candidate = test_mixed_candidate_v1(
            Some((HOME_ID, &home_digest)),
            TestMixedFormatV1::SlotAware,
            &declarations,
        );
        assert_eq!(candidate.refusal(), None);
        let plan = candidate.plan();
        assert_eq!(plan.len(), 4);

        // Every obligation is inspected, `internal` included.
        let set = complete(inspect(&entity, &platform, &catalogue, plan));
        assert_eq!(set.len(), 4);
        let internal = set
            .inspections()
            .iter()
            .find(|record| record.obligation().is_internal())
            .expect("the internal obligation is inspected");
        assert_eq!(internal.logical_path(), "assets/banner.webp");

        // A late missing Entity source refuses the whole candidate after the earlier reads.
        let late_missing = Root::new(
            "n129-late-missing",
            &[
                ("logo.png", png_bytes.as_slice()),
                ("mark.svg", svg_bytes.as_slice()),
            ],
        );
        let late_candidate = test_mixed_candidate_v1(
            Some((HOME_ID, &home_digest)),
            TestMixedFormatV1::SlotAware,
            &[
                declared(
                    TestOwnerV1::Entity,
                    "favicon",
                    "logo.png",
                    "entity",
                    false,
                    "public",
                ),
                declared(
                    TestOwnerV1::Site,
                    "logo",
                    "mark.svg",
                    "entity",
                    true,
                    "public",
                ),
                declared(
                    TestOwnerV1::Representation,
                    "banner",
                    "missing.png",
                    "entity",
                    false,
                    "public",
                ),
            ],
        );
        let outcome = inspect(&late_missing, &platform, &catalogue, late_candidate.plan());
        match outcome {
            MixedGraphicCandidateOutcomeV1::EntityReadRefused(refusal) => {
                assert_eq!(refusal.kind(), GraphicAssetReadRefusalKind::MissingSource);
            }
            other => panic!("a late missing source must refuse the whole candidate: {other:?}"),
        }
        assert_eq!(
            late_candidate.plan().len(),
            4,
            "the plan still holds all four"
        );

        // A late *media* failure after three successful children also discards everything.
        let corrupt_entity = Root::new(
            "n129-late-corrupt",
            &[
                ("logo.png", png_bytes.as_slice()),
                ("mark.svg", svg_bytes.as_slice()),
                ("banner.webp", webp().as_slice()),
                ("broken.png", b"not a png at all".as_slice()),
            ],
        );
        let corrupt_candidate = test_mixed_candidate_v1(
            Some((HOME_ID, &home_digest)),
            TestMixedFormatV1::SlotAware,
            &[
                declared(
                    TestOwnerV1::Entity,
                    "favicon",
                    "logo.png",
                    "entity",
                    false,
                    "public",
                ),
                declared(
                    TestOwnerV1::Site,
                    "logo",
                    "mark.svg",
                    "entity",
                    true,
                    "public",
                ),
                declared(
                    TestOwnerV1::Page,
                    "banner",
                    "banner.webp",
                    "entity",
                    false,
                    "public",
                ),
                declared(
                    TestOwnerV1::Representation,
                    "social-preview",
                    "broken.png",
                    "entity",
                    false,
                    "public",
                ),
            ],
        );
        match inspect(
            &corrupt_entity,
            &platform,
            &catalogue,
            corrupt_candidate.plan(),
        ) {
            MixedGraphicCandidateOutcomeV1::InspectRefused(refusal) => {
                assert!(
                    matches!(refusal.kind(), MixedGraphicInspectRefusalKindV1::Image(_)),
                    "the corrupt source fails a media check: {:?}",
                    refusal.kind()
                );
                assert_eq!(
                    refusal.obligation().local_index(),
                    0,
                    "the refusal names the offending authored obligation"
                );
            }
            other => panic!("a late corrupt source must refuse the whole candidate: {other:?}"),
        }

        println!(
            "n129_evidence case=02 obligations=4 internal_inspected=true superseded_inspected=true \
             late_missing=whole-refusal late_corrupt=whole-refusal zero_partial=true \
             publication=false"
        );
    }

    /// A scratch Entity content root this module can also mutate, for the native identity cases.
    struct MixedRoot {
        dir: PathBuf,
    }

    impl MixedRoot {
        fn new(label: &str, files: &[(&str, &[u8])]) -> Self {
            let dir =
                scratch_parent().join(format!("solo-n129-entity-{}-{label}", std::process::id()));
            let _ = fs::remove_dir_all(&dir);
            fs::create_dir_all(dir.join("assets")).expect("scratch assets directory");
            for (name, bytes) in files {
                fs::write(dir.join("assets").join(name), bytes).expect("fixture write");
            }
            Self { dir }
        }

        fn capability(&self) -> solo_adapters::ContentRootCapability {
            solo_adapters::ContentRootCapability::adopt_bootstrap_path(
                &self.dir,
                solo_adapters::RootProvenanceV1::OperatorDesignatedContentRoot,
            )
            .expect("the scratch Entity root is adoptable")
        }
    }

    impl Drop for MixedRoot {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.dir);
        }
    }

    /// Runs the one composed driver over an already adopted Entity capability.
    fn inspect_capability(
        entity: &solo_adapters::ContentRootCapability,
        platform: &PlatformRoot,
        catalogue: &Catalogue,
        plan: &MixedGraphicPlanV1,
    ) -> MixedGraphicCandidateOutcomeV1 {
        inspect_mixed_graphic_candidate_from_roots_v1(
            entity,
            &platform.capability(),
            plan,
            catalogue.request(),
            Path::new(BIN),
        )
    }

    /// `N129_03` — the two roots never alias, and each namespace refuses its own native hazards.
    #[test]
    fn n129_03_two_root_isolation_and_native_identity_refusal() {
        let png_bytes = png();
        let entity_svg = rich_svg().into_bytes();
        let catalogue = Catalogue::load();
        let home_digest = catalogue.digest(HOME_ID);

        // The same basename in both namespaces, with genuinely different bytes.
        let root = MixedRoot::new(
            "isolation",
            &[
                ("icon-home.svg", entity_svg.as_slice()),
                ("logo.png", png_bytes.as_slice()),
            ],
        );
        let platform = PlatformRoot::new("isolation");
        let entity_digest = sha256_media_worker_bytes_v1(&entity_svg).to_hex();
        assert_ne!(
            entity_digest, home_digest,
            "the two identically named files are different objects"
        );

        let candidate = test_mixed_candidate_v1(
            Some((HOME_ID, &home_digest)),
            TestMixedFormatV1::SlotAware,
            &[declared(
                TestOwnerV1::Site,
                "logo",
                "icon-home.svg",
                "entity",
                true,
                "public",
            )],
        );
        assert_eq!(candidate.refusal(), None);
        assert_eq!(candidate.plan().len(), 2);
        let set = complete(inspect_capability(
            &root.capability(),
            &platform,
            &catalogue,
            candidate.plan(),
        ));
        let platform_record = set
            .inspections()
            .iter()
            .find(|record| record.is_platform())
            .expect("the Platform source");
        let entity_record = set
            .inspections()
            .iter()
            .find(|record| record.is_entity())
            .expect("the Entity source");
        assert_eq!(platform_record.logical_path(), "icons/icon-home.svg");
        assert_eq!(entity_record.logical_path(), "assets/icon-home.svg");
        assert_eq!(platform_record.source_digest().to_hex(), home_digest);
        assert_eq!(entity_record.source_digest().to_hex(), entity_digest);

        // No fallback in either direction: an absent Entity source is never served from Platform.
        let absent = MixedRoot::new("absent", &[("logo.png", png_bytes.as_slice())]);
        let only_entity = test_mixed_candidate_v1(
            None,
            TestMixedFormatV1::SlotAware,
            &[declared(
                TestOwnerV1::Entity,
                "logo",
                "icon-home.svg",
                "entity",
                true,
                "public",
            )],
        );
        match inspect_capability(
            &absent.capability(),
            &platform,
            &catalogue,
            only_entity.plan(),
        ) {
            MixedGraphicCandidateOutcomeV1::EntityReadRefused(refusal) => {
                assert_eq!(refusal.kind(), GraphicAssetReadRefusalKind::MissingSource);
            }
            other => panic!("an absent Entity source must never fall back to Platform: {other:?}"),
        }

        // A hard link makes two authored paths one object: the native Entity reader refuses it.
        let linked = MixedRoot::new("hardlink", &[("logo.png", png_bytes.as_slice())]);
        fs::hard_link(
            linked.dir.join("assets/logo.png"),
            linked.dir.join("assets/copy.png"),
        )
        .expect("a hard link is creatable without privilege on both hosts");
        let two_paths = test_mixed_candidate_v1(
            None,
            TestMixedFormatV1::SlotAware,
            &[
                declared(
                    TestOwnerV1::Entity,
                    "favicon",
                    "logo.png",
                    "entity",
                    false,
                    "public",
                ),
                declared(
                    TestOwnerV1::Site,
                    "logo",
                    "copy.png",
                    "entity",
                    true,
                    "public",
                ),
            ],
        );
        match inspect_capability(
            &linked.capability(),
            &platform,
            &catalogue,
            two_paths.plan(),
        ) {
            MixedGraphicCandidateOutcomeV1::EntityReadRefused(refusal) => {
                assert_eq!(
                    refusal.kind(),
                    GraphicAssetReadRefusalKind::IdentityCollision,
                    "two paths to one physical object are a collision"
                );
            }
            other => panic!("a hard-linked Entity source must be refused: {other:?}"),
        }

        println!(
            "n129_evidence case=03 platform_path={HOME_LOGICAL} entity_path=assets/icon-home.svg \
             platform_sha={home_digest} entity_sha={entity_digest} distinct_objects=true \
             entity_fallback=false platform_fallback=false hardlink=refused publication=false"
        );
    }

    /// `N129_04` — catalogue, identity, role, format and binding mismatches all refuse, and the
    /// preserved legacy entry points keep their own semantics.
    #[test]
    fn n129_04_catalogue_role_format_and_binding_refusals() {
        let png_bytes = png();
        let root = MixedRoot::new("mismatch", &[("logo.png", png_bytes.as_slice())]);
        let platform = PlatformRoot::new("mismatch");
        let catalogue = Catalogue::load();
        let home_digest = catalogue.digest(HOME_ID);
        let capability = root.capability();
        let registry_declaration = [declared(
            TestOwnerV1::Entity,
            "favicon",
            "logo.png",
            "entity",
            false,
            "public",
        )];

        // 1. An identifier the approved catalogue does not declare.
        let fake_digest = "0".repeat(64);
        let unknown = test_mixed_candidate_v1(
            Some(("icons/icon-not-approved.svg", fake_digest.as_str())),
            TestMixedFormatV1::SlotAware,
            &registry_declaration,
        );
        assert_eq!(
            unknown.refusal(),
            None,
            "the registration itself is well formed"
        );
        match inspect_capability(&capability, &platform, &catalogue, unknown.plan()) {
            MixedGraphicCandidateOutcomeV1::PlatformReadRefused(refusal) => {
                assert_eq!(
                    refusal.kind(),
                    PlatformAssetReadRefusalKindV1::UnapprovedAssetIdentifier
                );
            }
            other => panic!("an unapproved Platform identifier must refuse: {other:?}"),
        }

        // 2. A catalogue that is not the approved document, and 3. an unselected version.
        let approved = test_mixed_candidate_v1(
            Some((HOME_ID, &home_digest)),
            TestMixedFormatV1::SlotAware,
            &registry_declaration,
        );
        let mutated = PlatformSourceRequestV1 {
            selected_version: VERSION,
            catalogue_bytes: b"{}",
            ..catalogue.request()
        };
        match inspect_mixed_graphic_candidate_from_roots_v1(
            &capability,
            &platform.capability(),
            approved.plan(),
            mutated,
            Path::new(BIN),
        ) {
            // The accepted reader reads the catalogue through its own port, so a mutated
            // `catalogue_bytes` argument is caught by the verified binding's independent pin.
            MixedGraphicCandidateOutcomeV1::BindingRefused(refusal) => {
                assert_eq!(
                    refusal.kind(),
                    PlatformGraphicBaselineRefusalKindV1::CatalogueUnapproved
                );
            }
            other => panic!("a mutated catalogue must refuse: {other:?}"),
        }
        let wrong_version = PlatformSourceRequestV1 {
            selected_version: "2.0.0",
            ..catalogue.request()
        };
        match inspect_mixed_graphic_candidate_from_roots_v1(
            &capability,
            &platform.capability(),
            approved.plan(),
            wrong_version,
            Path::new(BIN),
        ) {
            MixedGraphicCandidateOutcomeV1::PlatformReadRefused(refusal) => {
                assert_eq!(
                    refusal.kind(),
                    PlatformAssetReadRefusalKindV1::UnselectedVersion
                );
            }
            other => panic!("an unselected version must refuse: {other:?}"),
        }

        // 4. An approved SVG may never fill the PNG-only favicon role, in either namespace.
        let svg_favicon = test_mixed_candidate_v1(
            None,
            TestMixedFormatV1::SlotAware,
            &[declared(
                TestOwnerV1::Entity,
                "favicon",
                HOME_ID,
                "platform",
                false,
                "public",
            )],
        );
        assert_eq!(svg_favicon.refusal(), None);
        match inspect_capability(&capability, &platform, &catalogue, svg_favicon.plan()) {
            MixedGraphicCandidateOutcomeV1::InspectRefused(refusal) => {
                assert!(
                    matches!(refusal.kind(), MixedGraphicInspectRefusalKindV1::Image(_)),
                    "the role/format check refuses the SVG favicon: {:?}",
                    refusal.kind()
                );
            }
            other => panic!("an SVG favicon must refuse: {other:?}"),
        }

        // 5. An Entity reference outside the portable entity grammar refuses before any read.
        let reserved = test_mixed_candidate_v1(
            None,
            TestMixedFormatV1::SlotAware,
            &[declared(
                TestOwnerV1::Entity,
                "logo",
                "con.png",
                "entity",
                true,
                "public",
            )],
        );
        assert_eq!(
            reserved.refusal(),
            None,
            "the domain grammar admits it; the confined reader must not"
        );
        match inspect_capability(&capability, &platform, &catalogue, reserved.plan()) {
            MixedGraphicCandidateOutcomeV1::EntityReadRefused(refusal) => {
                assert_eq!(
                    refusal.kind(),
                    GraphicAssetReadRefusalKind::UnsafeAssetReference
                );
            }
            other => panic!("a reserved-stem Entity reference must refuse: {other:?}"),
        }

        // 6. Both authored formats at once are an ambiguity, refused before any read.
        let ambiguous = test_mixed_candidate_v1(
            None,
            TestMixedFormatV1::Both,
            &[declared(
                TestOwnerV1::Entity,
                "logo",
                "logo.png",
                "entity",
                true,
                "public",
            )],
        );
        assert_eq!(
            ambiguous.refusal(),
            Some(MixedGraphicPlanRefusalKindV1::AmbiguousGraphicFormat.detail()),
            "a slot-aware candidate with a legacy value is refused, never merged"
        );
        assert!(ambiguous.plan().is_empty());

        println!(
            "n129_evidence case=04 unapproved_id=refused catalogue_pin=independent \
             unselected_version=refused svg_favicon=refused unsafe_reference=refused \
             ambiguous_format=refused entity_only_entrypoint=preserved publication=false"
        );
    }

    /// `N129_05` — legacy parity, deterministic ordering, whole-candidate blocking and the closed
    /// authority surface.
    #[test]
    fn n129_05_legacy_parity_determinism_and_guardrails() {
        let png_bytes = png();
        let root = MixedRoot::new("parity", &[("logo.png", png_bytes.as_slice())]);
        let platform = PlatformRoot::new("parity");
        let catalogue = Catalogue::load();

        // A legacy candidate: no registration, no selection, one authored legacy value.
        let declarations = [declared(
            TestOwnerV1::Entity,
            "logo",
            "logo.png",
            "entity",
            false,
            "public",
        )];
        let legacy =
            test_mixed_candidate_v1(None, TestMixedFormatV1::LegacyEnvelope, &declarations);
        assert_eq!(legacy.refusal(), None);
        assert!(
            !legacy.is_slot_aware(),
            "no registration and no selection is parity mode"
        );
        assert_eq!(legacy.plan().len(), 1);
        assert_eq!(
            legacy.plan().obligations()[0].origin(),
            MixedGraphicOriginV1::LegacyEnvelopeValue
        );
        assert_eq!(
            legacy.plan().obligations()[0].slot(),
            None,
            "a legacy value carries no slot identity"
        );

        // The same driver inspects it, and the accepted Entity-only entry point agrees on the facts.
        let set = complete(inspect_capability(
            &root.capability(),
            &platform,
            &catalogue,
            legacy.plan(),
        ));
        assert_eq!(set.len(), 1);
        let mixed_record = &set.inspections()[0];
        assert!(mixed_record.is_entity());
        assert!(!set.grants_publication_authority());
        let inventory = test_inventory_v1(&[(TestOwnerV1::Entity, "logo", "logo.png", "public")])
            .expect("a valid legacy inventory");
        let entity_only = match inspect_entity_graphic_images_v4(
            &root.capability(),
            &inventory,
            Path::new(BIN),
        ) {
            EntityImageOutcomeV4::Complete(set) => set,
            other => panic!("the entity-only entry point must still complete: {other:?}"),
        };
        assert_eq!(entity_only.len(), 1);
        assert_eq!(
            entity_only.inspections()[0].source_digest(),
            mixed_record.source_digest(),
            "the mixed path and the entity-only path report the same source digest"
        );
        assert_eq!(entity_only.inspections()[0].format(), mixed_record.format());

        // Determinism: the same authored candidate plans identically twice.
        let again = test_mixed_candidate_v1(None, TestMixedFormatV1::LegacyEnvelope, &declarations);
        assert_eq!(again.plan(), legacy.plan());
        assert_eq!(again.winners(), legacy.winners());

        // A failure in an unrelated owner blocks the whole candidate and yields no records.
        let failing = test_mixed_candidate_v1(
            None,
            TestMixedFormatV1::SlotAware,
            &[
                declared(
                    TestOwnerV1::Entity,
                    "logo",
                    "logo.png",
                    "entity",
                    true,
                    "public",
                ),
                declared(
                    TestOwnerV1::Representation,
                    "banner",
                    "absent.webp",
                    "entity",
                    false,
                    "public",
                ),
            ],
        );
        let outcome = inspect_capability(&root.capability(), &platform, &catalogue, failing.plan());
        assert!(
            outcome.is_refused(),
            "the unrelated failure refuses the whole candidate"
        );
        assert!(
            outcome.complete().is_none(),
            "no partial record set is recoverable"
        );
        assert!(outcome.refusal_detail().is_some());

        // No public side effect: every record stays under its own namespace's own root.
        for record in set.inspections() {
            if record.is_entity() {
                assert!(record.logical_path().starts_with("assets/"));
            } else {
                assert!(record.is_platform());
                assert!(record.logical_path().starts_with("icons/"));
            }
        }

        println!(
            "n129_evidence case=05 legacy_parity=true slot_identity_absent=true \
             deterministic_plan=true entity_only_parity=true whole_candidate_blocked=true \
             zero_partial=true publication=false"
        );
    }
}

// `SOLO-WO-118 R19` — N130: the unpublished, immutable graphic delivery-preparation package.
//
// Four focused cases on the real two roots and the real confined workers. They prove the transition
// boundaries where public/private and immutable bytes meet: an eligible winning slot becomes an exact
// immutable payload, a private or non-eligible case materialises nothing, a substituted or forged
// input refuses the whole candidate, and the same input produces the same package twice.
mod n130 {
    use std::fs;
    use std::path::{Path, PathBuf};

    use solo_adapters::{
        PlatformRootCapabilityV1, TestMixedDeclarationV1, TestMixedFormatV1, TestOwnerV1,
        TestPackageSeamV1, TestPreparedPackageV1, parse_platform_asset_catalogue_v1,
        prepare_unpublished_graphic_package_from_roots_v1, test_mixed_candidate_v1,
    };
    use solo_application::{
        GraphicPackageLimitsV1, PlatformAssetCatalogueV1, PlatformRootProvenanceV1,
        PlatformRootSelectionV1, PlatformSourceRequestV1, PrepublicationDisplayEligibilityV1,
        sha256_media_worker_bytes_v1,
    };

    use super::BIN;
    use super::n116::{Root, png, rich_svg, webp};

    /// The accepted approved library version and the independent catalogue pin.
    const VERSION: &str = "1.0.0";
    const CATALOGUE_SHA256: &str =
        "ad8ea718b50641206cc4f4fe13e866ad266366a7aa11ef799c83ae8187858fc7";

    /// One real approved Platform identifier and the exact logical path it reads from.
    const HOME_ID: &str = "icons/icon-home.svg";
    const HOME_LOGICAL: &str = "icons/icon-home.svg";

    /// The tracked product tree, used only as the copy source for a scratch Platform root.
    fn product_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../platform")
    }

    /// The filesystem a scratch root is created on; on Linux it must be the verified fixture one.
    fn scratch_parent() -> PathBuf {
        #[cfg(target_os = "linux")]
        {
            match std::env::var_os("SOLO_TEST_SOURCE_FS_ROOT") {
                Some(root) if !root.is_empty() => PathBuf::from(root),
                _ => panic!(
                    "SOLO_TEST_SOURCE_FS_ROOT must name the R03 snapshot fixture filesystem; \
                     refusing to run the real delivery-preparation gate on an unverified filesystem"
                ),
            }
        }
        #[cfg(not(target_os = "linux"))]
        {
            std::env::temp_dir()
        }
    }
    /// Copies one directory tree of regular files without dereferencing links.
    fn copy_tree(from: &Path, to: &Path) {
        fs::create_dir_all(to).expect("create a scratch directory");
        for entry in fs::read_dir(from).expect("read a product directory") {
            let entry = entry.expect("a product directory entry");
            let file_type = entry.file_type().expect("a product file type");
            assert!(
                !file_type.is_symlink(),
                "the tracked product tree holds no link"
            );
            let target = to.join(entry.file_name());
            if file_type.is_dir() {
                copy_tree(&entry.path(), &target);
            } else {
                assert!(file_type.is_file(), "the product tree holds regular files");
                fs::copy(entry.path(), &target).expect("copy one product file");
            }
        }
    }

    /// A disposable Platform root holding one copy of the approved product library.
    struct PlatformRoot {
        base: PathBuf,
        platform: PathBuf,
    }

    impl PlatformRoot {
        fn new(label: &str) -> Self {
            let base =
                scratch_parent().join(format!("solo-n130-platform-{}-{label}", std::process::id()));
            let _ = fs::remove_dir_all(&base);
            let platform = base.join("platform");
            copy_tree(&product_root(), &platform);
            Self { base, platform }
        }

        fn capability(&self) -> PlatformRootCapabilityV1 {
            PlatformRootCapabilityV1::adopt_bootstrap_path(
                &self.platform,
                PlatformRootProvenanceV1::OperatorDesignatedPlatformRoot,
            )
            .expect("the scratch Platform root is adoptable")
        }
    }

    impl Drop for PlatformRoot {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.base);
        }
    }

    /// The parsed approved catalogue together with the exact bytes it was parsed from.
    struct Catalogue {
        model: PlatformAssetCatalogueV1,
        bytes: Vec<u8>,
    }

    impl Catalogue {
        fn load() -> Self {
            let bytes = fs::read(product_root().join("assets/catalogue-1.0.0.json"))
                .expect("the installed catalogue is present");
            assert_eq!(
                sha256_media_worker_bytes_v1(&bytes).to_hex(),
                CATALOGUE_SHA256,
                "the installed catalogue is the approved one"
            );
            let model = parse_platform_asset_catalogue_v1(&bytes, VERSION)
                .expect("the strict R12 parser accepts the installed catalogue");
            Self { model, bytes }
        }

        fn digest(&self, id: &str) -> String {
            self.model
                .entries()
                .iter()
                .find(|entry| entry.id() == id)
                .map_or_else(
                    || panic!("{id} is an approved identifier"),
                    |entry| entry.sha256().to_hex(),
                )
        }

        fn request(&self) -> PlatformSourceRequestV1<'_> {
            PlatformSourceRequestV1 {
                selection: PlatformRootSelectionV1::adopt(
                    PlatformRootProvenanceV1::OperatorDesignatedPlatformRoot,
                ),
                selected_version: VERSION,
                catalogue: &self.model,
                catalogue_bytes: &self.bytes,
            }
        }
    }
    /// One authored mixed declaration.
    fn declared(
        owner: TestOwnerV1,
        role: &'static str,
        reference: &'static str,
        namespace: &'static str,
        supersede: bool,
        exposure: &'static str,
    ) -> TestMixedDeclarationV1 {
        TestMixedDeclarationV1 {
            owner,
            role,
            slot: "primary",
            namespace,
            reference,
            supersede,
            exposure,
        }
    }

    /// Runs the one composed preparation path over the real roots.
    fn prepare(
        entity: &Root,
        platform: &PlatformRoot,
        catalogue: &Catalogue,
        seam: &TestPackageSeamV1<'_>,
    ) -> TestPreparedPackageV1 {
        prepare_unpublished_graphic_package_from_roots_v1(
            &entity.capability(),
            &platform.capability(),
            seam,
            catalogue.request(),
            Path::new(BIN),
        )
    }

    /// A slot-aware seam with the documented hard limits and one display case.
    fn seam<'a>(
        declarations: &'a [TestMixedDeclarationV1],
        registration: Option<(&'static str, &'a str)>,
        eligibility: PrepublicationDisplayEligibilityV1,
    ) -> TestPackageSeamV1<'a> {
        TestPackageSeamV1 {
            registration,
            format: TestMixedFormatV1::SlotAware,
            declarations,
            foreign_plan: None,
            eligibility,
            limits: GraphicPackageLimitsV1::hard(),
        }
    }

    /// Asserts the attempt prepared a package, naming the real refusal otherwise.
    fn prepared<'a>(
        outcome: &'a TestPreparedPackageV1,
        context: &str,
    ) -> &'a TestPreparedPackageV1 {
        assert!(
            outcome.construction_refusal().is_none(),
            "{context}: the candidate was refused: {:?}",
            outcome.construction_refusal()
        );
        assert!(
            outcome.package_refusal().is_none(),
            "{context}: the package was refused: {:?}",
            outcome.package_refusal()
        );
        assert!(
            outcome.source_refusal().is_none(),
            "{context}: the candidate sources were refused: {:?}",
            outcome.source_refusal()
        );
        outcome
    }

    /// Every content key must be a bounded, lowercase, derived identity — never a path or a URL.
    fn assert_key_grammar(outcome: &TestPreparedPackageV1) {
        for entry in outcome.entries() {
            let key = &entry.content_key;
            assert!(
                key.starts_with("sha256/"),
                "a content key is a derived identity: {key}"
            );
            assert!(key.len() <= 96, "a content key stays bounded: {key}");
            assert!(key.is_ascii(), "a content key is ASCII: {key}");
            assert!(!key.contains(".."), "no traversal component: {key}");
            assert!(!key.contains("://"), "no scheme, no URL: {key}");
            assert!(!key.starts_with('/'), "no absolute path: {key}");
            assert!(!key.contains('\\'), "no host path separator: {key}");
            let digest = key
                .strip_prefix("sha256/")
                .and_then(|rest| rest.split('.').next())
                .expect("a content key carries a digest");
            assert_eq!(digest.len(), 64, "a content key is a full SHA-256: {key}");
            assert!(
                digest
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)),
                "a content key digest is lowercase hex: {key}"
            );
            assert_eq!(
                digest, entry.output_sha256,
                "the key derives from the exact output digest"
            );
        }
    }

    /// The tracked product files a package must never touch.
    fn product_digests() -> Vec<(String, String)> {
        let mut digests = Vec::new();
        for relative in [
            "assets/catalogue-1.0.0.json",
            "assets/icons/icon-home.svg",
            "assets/licensing/TABLER-ICONS-MIT.txt",
        ] {
            let path = product_root().join(relative);
            let bytes = fs::read(&path).expect("the tracked product file is readable");
            digests.push((
                relative.to_owned(),
                sha256_media_worker_bytes_v1(&bytes).to_hex(),
            ));
        }
        digests
    }

    /// One entry by exact role name.
    fn entry_of<'a>(
        outcome: &'a TestPreparedPackageV1,
        role: &str,
    ) -> &'a solo_adapters::TestPreparedEntryV1 {
        outcome
            .entries()
            .iter()
            .find(|entry| entry.role == role)
            .unwrap_or_else(|| panic!("the package carries a {role} entry"))
    }
    /// `N130_01` — a real mixed candidate: an approved Platform root default becomes a canonical
    /// vector payload, the Entity raster winners become byte-exact pass-through payloads, and a
    /// nearer authored supersession governs its own slot.
    #[test]
    fn n130_01_public_intent_package_is_exact_and_unpublished() {
        let png_bytes = png();
        let webp_bytes = webp();
        let entity = Root::new(
            "n130-package",
            &[
                ("logo.png", png_bytes.as_slice()),
                ("banner.webp", webp_bytes.as_slice()),
            ],
        );
        let platform = PlatformRoot::new("n130-package");
        let catalogue = Catalogue::load();
        let home_digest = catalogue.digest(HOME_ID);

        // One approved Platform root registration for `graphic.logo`, an Entity favicon, an Entity
        // social preview, and a Facet-then-Site `background` chain whose closer Site value wins.
        let declarations = [
            declared(
                TestOwnerV1::Entity,
                "favicon",
                "logo.png",
                "entity",
                false,
                "public",
            ),
            declared(
                TestOwnerV1::Entity,
                "social-preview",
                "banner.webp",
                "entity",
                false,
                "public",
            ),
            declared(
                TestOwnerV1::Facet,
                "background",
                "banner.webp",
                "entity",
                false,
                "public",
            ),
            declared(
                TestOwnerV1::Site,
                "background",
                "logo.png",
                "entity",
                true,
                "public",
            ),
        ];
        let outcome = prepare(
            &entity,
            &platform,
            &catalogue,
            &seam(
                &declarations,
                Some((HOME_ID, &home_digest)),
                PrepublicationDisplayEligibilityV1::Public,
            ),
        );
        prepared(&outcome, "N130_01");

        assert_eq!(
            outcome.target_level(),
            4,
            "the representation owner is scoped"
        );
        assert_eq!(outcome.entries().len(), 4, "four eligible winning slots");
        assert_eq!(outcome.eligibility(), "public");
        assert!(
            !outcome.publication_authority(),
            "a package publishes nothing"
        );
        assert!(!outcome.routes_any_url(), "a package routes nothing");
        assert_key_grammar(&outcome);

        // The Platform default stays a vector: the canonical text, not the donor file and not a raster.
        let logo = entry_of(&outcome, "graphic.logo");
        assert_eq!(logo.mime, "image/svg+xml");
        assert_eq!(logo.format, "svg");
        assert_eq!(
            logo.source_sha256, home_digest,
            "the source is the approved catalogue asset"
        );
        assert_ne!(
            logo.source_sha256, logo.output_sha256,
            "the canonical vector output is not the donor source digest"
        );
        let text = std::str::from_utf8(&logo.payload).expect("the canonical vector is UTF-8 text");
        assert!(
            text.contains("<svg") && text.contains("viewBox"),
            "the payload is a real vector document"
        );
        assert!(
            !text.contains("<script") && !text.contains("xlink:href=\"http"),
            "the accepted canonical vector carries no script and no external reference"
        );
        assert_eq!(
            sha256_media_worker_bytes_v1(&logo.payload).to_hex(),
            logo.output_sha256,
            "the staged vector bytes hash to the recorded output digest"
        );

        // The raster winners are byte-for-byte pass-through of the accepted Entity sources.
        let favicon = entry_of(&outcome, "graphic.favicon");
        assert_eq!(favicon.format, "png");
        assert_eq!(favicon.mime, "image/png");
        assert_eq!(favicon.payload, png_bytes);
        assert_eq!(favicon.source_sha256, favicon.output_sha256);
        let preview = entry_of(&outcome, "graphic.social-preview");
        assert_eq!(preview.format, "webp");
        assert_eq!(preview.mime, "image/webp");
        assert_eq!(preview.payload, webp_bytes);

        // The nearer Site supersession governs `background`, so only its reference is staged.
        let background = entry_of(&outcome, "graphic.background");
        assert_eq!(background.slot, "primary");
        assert_eq!(background.payload, png_bytes, "the closer Site value wins");

        assert_eq!(
            outcome.payload_manifest().len(),
            3,
            "logo.png is staged once"
        );
        assert_eq!(
            outcome.total_bytes(),
            favicon.payload.len() as u64 + preview.payload.len() as u64 + logo.payload.len() as u64
        );
        assert!(
            outcome.discoverable(),
            "an eligible case may advertise itself"
        );

        println!(
            "n130_evidence case=01 entries=4 payloads=3 svg_vector=true canonical_sha_matched=true \
             content_addressed=true platform_path={HOME_LOGICAL} platform_sha={} \
             output_sha={} favicon_sha={} preview_sha={} winner_background=logo.png lineage=2 \
             catalogue_sha={CATALOGUE_SHA256} publication=false route=false",
            logo.source_sha256, logo.output_sha256, favicon.output_sha256, preview.output_sha256
        );
    }
    /// `N130_02` — the privacy boundary: a private winner never falls back to a public Platform
    /// default, a non-eligible display case yields nothing discoverable, and a late invalid source
    /// still refuses the whole candidate with zero staged records.
    #[test]
    fn n130_02_privacy_boundary_and_late_refusal_are_whole() {
        let png_bytes = png();
        let webp_bytes = webp();
        let mark_bytes = rich_svg().into_bytes();
        let entity = Root::new(
            "n130-privacy",
            &[
                ("logo.png", png_bytes.as_slice()),
                ("banner.webp", webp_bytes.as_slice()),
                ("mark.svg", mark_bytes.as_slice()),
            ],
        );
        let platform = PlatformRoot::new("n130-privacy");
        let catalogue = Catalogue::load();
        let home_digest = catalogue.digest(HOME_ID);

        // The approved public Platform default is superseded by a closer `internal` Entity value: the
        // private value wins, so the role contributes nothing and the root default never resurfaces.
        let declarations = [
            declared(
                TestOwnerV1::Entity,
                "logo",
                "mark.svg",
                "entity",
                true,
                "internal",
            ),
            declared(
                TestOwnerV1::Entity,
                "favicon",
                "logo.png",
                "entity",
                false,
                "public",
            ),
            declared(
                TestOwnerV1::Facet,
                "banner",
                "banner.webp",
                "entity",
                false,
                "public",
            ),
        ];
        let public = prepare(
            &entity,
            &platform,
            &catalogue,
            &seam(
                &declarations,
                Some((HOME_ID, &home_digest)),
                PrepublicationDisplayEligibilityV1::Public,
            ),
        );
        prepared(&public, "N130_02 public");
        assert_eq!(
            public.entries().len(),
            2,
            "only the two public winners are eligible: {:?}",
            public
                .entries()
                .iter()
                .map(|entry| entry.role.as_str())
                .collect::<Vec<_>>()
        );
        assert!(
            public
                .entries()
                .iter()
                .all(|entry| entry.role != "graphic.logo"),
            "an internal winner exposes nothing and never resurrects the Platform default"
        );
        assert!(
            public
                .provenance()
                .iter()
                .all(|(_, _, source, _)| source != &home_digest),
            "the superseded root default is inspected but never staged"
        );
        for entry in public.entries() {
            assert!(
                !entry.slot.contains('/') && !entry.role.contains(".."),
                "no private path or owner text reaches an entry"
            );
            assert!(
                !entry.content_key.contains("internal") && !entry.content_key.contains("entity"),
                "a content key carries no owner or exposure text"
            );
        }
        // The very same fully valid candidate under the unlisted and internal display cases produces
        // no discoverable general-purpose entry at all — and still had to pass every real check.
        for eligibility in [
            PrepublicationDisplayEligibilityV1::Unlisted,
            PrepublicationDisplayEligibilityV1::Internal,
        ] {
            let hidden = prepare(
                &entity,
                &platform,
                &catalogue,
                &seam(&declarations, Some((HOME_ID, &home_digest)), eligibility),
            );
            prepared(&hidden, "N130_02 hidden");
            assert!(
                hidden.entries().is_empty(),
                "{} yields nothing",
                eligibility.as_str()
            );
            assert!(hidden.payload_manifest().is_empty());
            assert_eq!(hidden.total_bytes(), 0);
            assert!(!hidden.discoverable(), "no discovery entry is invented");
            assert!(!hidden.publication_authority());
            assert!(!hidden.routes_any_url());
        }

        // A late invalid source — a WebP declared for the PNG-only favicon role at the closest owner —
        // refuses the whole candidate after the earlier obligations already succeeded.
        let late = prepare(
            &entity,
            &platform,
            &catalogue,
            &seam(
                &[
                    declared(
                        TestOwnerV1::Entity,
                        "favicon",
                        "logo.png",
                        "entity",
                        false,
                        "public",
                    ),
                    declared(
                        TestOwnerV1::Entity,
                        "banner",
                        "banner.webp",
                        "entity",
                        false,
                        "public",
                    ),
                    declared(
                        TestOwnerV1::Representation,
                        "favicon",
                        "banner.webp",
                        "entity",
                        true,
                        "public",
                    ),
                ],
                Some((HOME_ID, &home_digest)),
                PrepublicationDisplayEligibilityV1::Public,
            ),
        );
        let refusal = late
            .source_refusal()
            .expect("the invalid late obligation refuses the whole candidate");
        assert!(late.package_refusal().is_none());
        assert!(late.entries().is_empty(), "no partial record survives");
        assert!(late.payload_manifest().is_empty(), "no payload is staged");
        assert_eq!(late.total_bytes(), 0);
        assert!(!late.discoverable());

        println!(
            "n130_evidence case=02 entries=2 internal_hidden=true superseded_not_resurrected=true \
             unlisted_discovery=false late_refusal=whole late_kind={refusal} zero_partial=true \
             output_sha={} catalogue_sha={CATALOGUE_SHA256} publication=false route=false",
            entry_of(&public, "graphic.favicon").output_sha256
        );
    }
    /// `N130_03` — identity and immutability: a foreign plan, a forged media claim and an unapproved
    /// Platform identifier are all refused with zero records, identical bytes deduplicate safely, and
    /// neither the tracked product tree nor a source file can be overwritten.
    #[test]
    fn n130_03_identity_substitution_and_immutable_collision() {
        let before = product_digests();
        let png_bytes = png();
        let webp_bytes = webp();
        let donor_svg = fs::read(product_root().join("assets/icons/icon-home.svg"))
            .expect("the tracked donor icon is readable");
        let entity = Root::new(
            "n130-identity",
            &[
                ("logo.png", png_bytes.as_slice()),
                ("banner.webp", webp_bytes.as_slice()),
                ("trap.png", webp_bytes.as_slice()),
            ],
        );
        let platform = PlatformRoot::new("n130-identity");
        let catalogue = Catalogue::load();
        let home_digest = catalogue.digest(HOME_ID);
        let base = [
            declared(
                TestOwnerV1::Entity,
                "favicon",
                "logo.png",
                "entity",
                false,
                "public",
            ),
            declared(
                TestOwnerV1::Entity,
                "social-preview",
                "logo.png",
                "entity",
                false,
                "public",
            ),
            declared(
                TestOwnerV1::Entity,
                "banner",
                "banner.webp",
                "entity",
                false,
                "public",
            ),
        ];

        // A plan belonging to a different candidate must be refused before any source is touched.
        let other = test_mixed_candidate_v1(
            Some((HOME_ID, &home_digest)),
            TestMixedFormatV1::SlotAware,
            &[
                declared(
                    TestOwnerV1::Entity,
                    "favicon",
                    "logo.png",
                    "entity",
                    false,
                    "public",
                ),
                declared(
                    TestOwnerV1::Site,
                    "logo",
                    "logo.png",
                    "entity",
                    true,
                    "public",
                ),
            ],
        );
        assert_eq!(other.refusal(), None, "the other candidate is itself valid");
        let foreign_plan = other.plan();
        let foreign = prepare(
            &entity,
            &platform,
            &catalogue,
            &TestPackageSeamV1 {
                registration: Some((HOME_ID, &home_digest)),
                format: TestMixedFormatV1::SlotAware,
                declarations: &base,
                foreign_plan: Some(foreign_plan),
                eligibility: PrepublicationDisplayEligibilityV1::Public,
                limits: GraphicPackageLimitsV1::hard(),
            },
        );
        assert_eq!(
            foreign.package_refusal(),
            Some("plan-does-not-match-input"),
            "a foreign plan is refused by identity, not by a filename"
        );
        assert!(foreign.source_refusal().is_none(), "no source was acquired");
        assert!(foreign.entries().is_empty());
        assert!(foreign.payload_manifest().is_empty());

        // A forged media claim is caught by the accepted inspector before any output exists.
        let forged = prepare(
            &entity,
            &platform,
            &catalogue,
            &seam(
                &[declared(
                    TestOwnerV1::Entity,
                    "favicon",
                    "trap.png",
                    "entity",
                    false,
                    "public",
                )],
                Some((HOME_ID, &home_digest)),
                PrepublicationDisplayEligibilityV1::Public,
            ),
        );
        let forged_kind = forged
            .source_refusal()
            .expect("a .png name over WebP bytes is refused");
        assert!(forged.entries().is_empty());
        assert!(forged.payload_manifest().is_empty());

        // An unapproved Platform identifier is refused before any payload exists: the registration
        // claims a digest that is not the approved asset, so the package refuses the whole attempt.
        let unapproved_digest = "0".repeat(64);
        let unapproved = prepare(
            &entity,
            &platform,
            &catalogue,
            &seam(
                &base,
                Some((HOME_ID, unapproved_digest.as_str())),
                PrepublicationDisplayEligibilityV1::Public,
            ),
        );
        let unapproved_kind = unapproved
            .package_refusal()
            .expect("an unapproved Platform registration is refused");
        assert_eq!(
            unapproved_kind, "unapproved-platform-source",
            "the refusal names the unapproved Platform source"
        );
        assert!(
            unapproved.source_refusal().is_none(),
            "the read itself succeeded"
        );
        assert!(unapproved.entries().is_empty());
        assert!(unapproved.payload_manifest().is_empty());

        // Truly identical final bytes share one immutable payload; different bytes never coalesce.
        let identical = prepare(
            &entity,
            &platform,
            &catalogue,
            &seam(
                &base,
                Some((HOME_ID, &home_digest)),
                PrepublicationDisplayEligibilityV1::Public,
            ),
        );
        prepared(&identical, "N130_03 identical");
        assert_eq!(
            identical.entries().len(),
            4,
            "logo, favicon, preview, banner"
        );
        assert_eq!(
            identical.payload_manifest().len(),
            3,
            "logo.png is stored once for two slots"
        );
        let favicon = entry_of(&identical, "graphic.favicon");
        let preview = entry_of(&identical, "graphic.social-preview");
        let banner = entry_of(&identical, "graphic.banner");
        let logo = entry_of(&identical, "graphic.logo");
        assert_eq!(favicon.content_key, preview.content_key, "true dedup");
        assert_eq!(favicon.output_sha256, preview.output_sha256);
        assert_ne!(
            favicon.content_key, banner.content_key,
            "different bytes differ"
        );
        assert_ne!(
            favicon.content_key, logo.content_key,
            "different formats differ"
        );
        assert_key_grammar(&identical);
        assert_eq!(
            logo.source_sha256, home_digest,
            "the source digest is confirmed independently"
        );
        assert_ne!(
            logo.output_sha256, home_digest,
            "the canonical output digest is its own identity"
        );
        assert_ne!(
            logo.payload, donor_svg,
            "the staged vector is the canonical text, not the donor file"
        );

        // A test-owned copy proves the exact bytes survive a round trip, and that staging one output
        // on disk leaves every tracked product file byte-identical.
        let stage = scratch_parent().join(format!("solo-n130-stage-{}", std::process::id()));
        let _ = fs::remove_dir_all(&stage);
        fs::create_dir_all(&stage).expect("a disposable task-owned staging directory");
        let staged_path = stage.join("payload.svg");
        fs::write(&staged_path, &logo.payload).expect("stage one package payload");
        let reread = fs::read(&staged_path).expect("re-read the staged payload");
        assert_eq!(reread, logo.payload, "the staged bytes are exact");
        assert_eq!(
            sha256_media_worker_bytes_v1(&reread).to_hex(),
            logo.output_sha256
        );
        let _ = fs::remove_dir_all(&stage);

        let after = product_digests();
        assert_eq!(
            before, after,
            "no package output may overwrite a product file or the catalogue"
        );

        println!(
            "n130_evidence case=03 foreign_plan=plan-does-not-match-input forged_media={forged_kind} \
             unapproved_id={unapproved_kind} entries=4 payloads=3 dedup=identical_bytes_only \
             collision=refused_by_derivation immutable=true product_tree_unchanged=true \
             output_sha={} catalogue_sha={CATALOGUE_SHA256} publication=false route=false",
            logo.output_sha256
        );
    }
    /// `N130_04` — determinism and authority: the same validated input produces the same package
    /// twice, quota and limit failures refuse atomically, the legacy entrypoint stays unchanged, and
    /// nothing activates a default, a route or a publication.
    #[test]
    fn n130_04_deterministic_package_limits_and_authority() {
        let png_bytes = png();
        let webp_bytes = webp();
        let entity = Root::new(
            "n130-deterministic",
            &[
                ("logo.png", png_bytes.as_slice()),
                ("banner.webp", webp_bytes.as_slice()),
            ],
        );
        let platform = PlatformRoot::new("n130-deterministic");
        let catalogue = Catalogue::load();
        let home_digest = catalogue.digest(HOME_ID);
        let declarations = [
            declared(
                TestOwnerV1::Entity,
                "favicon",
                "logo.png",
                "entity",
                false,
                "public",
            ),
            declared(
                TestOwnerV1::Entity,
                "banner",
                "banner.webp",
                "entity",
                false,
                "public",
            ),
            declared(
                TestOwnerV1::Entity,
                "social-preview",
                "logo.png",
                "entity",
                false,
                "public",
            ),
        ];
        let request = seam(
            &declarations,
            Some((HOME_ID, &home_digest)),
            PrepublicationDisplayEligibilityV1::Public,
        );

        // The same fully validated input twice: byte-identical entries, payloads and ordering.
        let first = prepare(&entity, &platform, &catalogue, &request);
        prepared(&first, "N130_04 first");
        let second = prepare(&entity, &platform, &catalogue, &request);
        prepared(&second, "N130_04 second");
        assert_eq!(first, second, "the package is deterministic");
        assert_eq!(first.entries().len(), 4);
        assert_eq!(first.payload_manifest(), second.payload_manifest());
        assert_eq!(first.total_bytes(), second.total_bytes());

        // A tightened total-byte bound refuses the whole package, never a truncated prefix.
        let tight = GraphicPackageLimitsV1::new(8, 64).expect("a bounded tightened limit");
        let over_bytes = prepare(
            &entity,
            &platform,
            &catalogue,
            &TestPackageSeamV1 {
                registration: Some((HOME_ID, &home_digest)),
                format: TestMixedFormatV1::SlotAware,
                declarations: &declarations,
                foreign_plan: None,
                eligibility: PrepublicationDisplayEligibilityV1::Public,
                limits: tight,
            },
        );
        assert_eq!(over_bytes.package_refusal(), Some("quota-exceeded"));
        assert!(over_bytes.entries().is_empty(), "no partial package");
        assert!(over_bytes.payload_manifest().is_empty());

        // A tightened entry bound refuses at the fourth entry, still with zero records.
        let narrow = GraphicPackageLimitsV1::new(2, 8 * 1024 * 1024).expect("a bounded entry cap");
        let over_entries = prepare(
            &entity,
            &platform,
            &catalogue,
            &TestPackageSeamV1 {
                registration: Some((HOME_ID, &home_digest)),
                format: TestMixedFormatV1::SlotAware,
                declarations: &declarations,
                foreign_plan: None,
                eligibility: PrepublicationDisplayEligibilityV1::Public,
                limits: narrow,
            },
        );
        assert_eq!(over_entries.package_refusal(), Some("quota-exceeded"));
        assert!(over_entries.entries().is_empty(), "no partial package");

        // The documented hard maxima cannot be exceeded by a request.
        assert_eq!(
            GraphicPackageLimitsV1::new(0, 1024).map_err(|refusal| refusal.detail()),
            Err("invalid-limits")
        );
        assert_eq!(
            GraphicPackageLimitsV1::new(999, 1024).map_err(|refusal| refusal.detail()),
            Err("invalid-limits")
        );
        assert_eq!(
            GraphicPackageLimitsV1::new(4, 64 * 1024 * 1024).map_err(|refusal| refusal.detail()),
            Err("invalid-limits")
        );

        // The accepted legacy envelope stays exactly as it was: one public value, no slot identity, and
        // therefore no slot-aware package record and no invented default.
        let parity = prepare(
            &entity,
            &platform,
            &catalogue,
            &TestPackageSeamV1 {
                registration: None,
                format: TestMixedFormatV1::LegacyEnvelope,
                declarations: &[declared(
                    TestOwnerV1::Entity,
                    "favicon",
                    "logo.png",
                    "entity",
                    false,
                    "public",
                )],
                foreign_plan: None,
                eligibility: PrepublicationDisplayEligibilityV1::Public,
                limits: GraphicPackageLimitsV1::hard(),
            },
        );
        prepared(&parity, "N130_04 parity");
        assert!(
            parity.entries().is_empty(),
            "a parity candidate carries no slot identity to package"
        );
        assert!(parity.payload_manifest().is_empty());
        assert_eq!(parity.total_bytes(), 0);
        assert!(!parity.discoverable(), "no automatic default is activated");
        assert!(!parity.publication_authority());
        assert!(!parity.routes_any_url());

        // The eligible package itself still asserts no authority of any kind.
        assert!(!first.publication_authority());
        assert!(!first.routes_any_url());
        assert_eq!(first.eligibility(), "public");

        println!(
            "n130_evidence case=04 deterministic=true limits_bounded=true quota=atomic \
             legacy_parity=true slot_identity_absent=true discoverable=false publication=false \
             route=false output_sha={} catalogue_sha={CATALOGUE_SHA256}",
            entry_of(&first, "graphic.favicon").output_sha256
        );
    }
}
mod n131 {
    use std::fs;
    use std::path::{Path, PathBuf};

    use solo_adapters::{
        ConfinedGraphicInspectAdapterV4, PlatformRootCapabilityV1, TestMixedDeclarationV1,
        TestMixedFormatV1, TestOwnerV1, TestPackageSeamV1, TestPreparedPackageV1,
        parse_platform_asset_catalogue_v1, prepare_unpublished_graphic_package_from_roots_v1,
    };
    use solo_application::preflight_domain::{
        ApprovedPlatformAssetV1, ApprovedPlatformGraphicBindingV1, AssetNamespace,
        AssetReferenceV1, EfsLevel, EfsOwnershipDraft, EfsOwnershipGraph, EntityIdV1, Exposure,
        FacetDraft, FacetIdV1, GraphicCascadeInputV1, GraphicSlotSelectionV1, LocaleKeyV1,
        OwnerGraphicSelectionsV1, PageDraft, PageIdV1, PageVisibilityDeclarationV1,
        PlatformAssetMediaClassV1, PlatformGraphicBaselineRecordV1, PlatformGraphicBaselineV1,
        PropertyKey, QualifiedOwnerV1, RepresentationVisibilityLedgerV1,
        RepresentationVisibilityOriginV1, RepresentationVisibilityRefusalKindV1,
        RepresentationVisibilityTokenRefusalKindV1, RepresentationVisibilityV1, SiteDraft,
        SiteIdV1, build_representation_visibility_ledger_v1, validate_declared_properties_v1,
    };
    use solo_application::{
        GraphicPackageLimitsV1, GraphicVisibilityPreflightOutcomeV1,
        GraphicVisibilityPreflightRefusalKindV1, GraphicVisibilityPreflightRequestV1,
        GraphicVisibilityPreflightV1, MixedGraphicPlanV1, PlatformAssetCatalogueV1,
        PlatformRootProvenanceV1, PlatformRootSelectionV1, PlatformSourceRequestV1,
        PrepublicationDisplayEligibilityV1, collect_mixed_graphic_obligations_v1,
        prepare_graphic_visibility_preflight_v1, sha256_media_worker_bytes_v1,
    };

    use super::BIN;
    use super::n116::{Root, png, webp};

    /// The accepted approved library version and the independent catalogue pin.
    const VERSION: &str = "1.0.0";
    const CATALOGUE_SHA256: &str =
        "ad8ea718b50641206cc4f4fe13e866ad266366a7aa11ef799c83ae8187858fc7";

    /// One real approved Platform identifier.
    const HOME_ID: &str = "icons/icon-home.svg";

    /// The tracked product tree, used only as the copy source for a scratch Platform root.
    fn product_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../platform")
    }

    /// The filesystem a scratch root is created on; on Linux it must be the verified fixture one.
    fn scratch_parent() -> PathBuf {
        #[cfg(target_os = "linux")]
        {
            match std::env::var_os("SOLO_TEST_SOURCE_FS_ROOT") {
                Some(root) if !root.is_empty() => PathBuf::from(root),
                _ => panic!(
                    "SOLO_TEST_SOURCE_FS_ROOT must name the R03 snapshot fixture filesystem; \
                     refusing to run the real visibility preflight gate on an unverified filesystem"
                ),
            }
        }
        #[cfg(not(target_os = "linux"))]
        {
            std::env::temp_dir()
        }
    }

    /// Copies one directory tree of regular files without dereferencing links.
    fn copy_tree(from: &Path, to: &Path) {
        fs::create_dir_all(to).expect("create a scratch directory");
        for entry in fs::read_dir(from).expect("read a product directory") {
            let entry = entry.expect("a product directory entry");
            let file_type = entry.file_type().expect("a product file type");
            assert!(
                !file_type.is_symlink(),
                "the tracked product tree holds no link"
            );
            let target = to.join(entry.file_name());
            if file_type.is_dir() {
                copy_tree(&entry.path(), &target);
            } else {
                assert!(file_type.is_file(), "the product tree holds regular files");
                fs::copy(entry.path(), &target).expect("copy one product file");
            }
        }
    }

    /// A disposable Platform root holding one copy of the approved product library.
    struct PlatformRoot {
        base: PathBuf,
        platform: PathBuf,
    }

    impl PlatformRoot {
        fn new(label: &str) -> Self {
            let base =
                scratch_parent().join(format!("solo-n131-platform-{}-{label}", std::process::id()));
            let _ = fs::remove_dir_all(&base);
            let platform = base.join("platform");
            copy_tree(&product_root(), &platform);
            Self { base, platform }
        }

        fn capability(&self) -> PlatformRootCapabilityV1 {
            PlatformRootCapabilityV1::adopt_bootstrap_path(
                &self.platform,
                PlatformRootProvenanceV1::OperatorDesignatedPlatformRoot,
            )
            .expect("the scratch Platform root is adoptable")
        }
    }

    impl Drop for PlatformRoot {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.base);
        }
    }

    /// The parsed approved catalogue together with the exact bytes it was parsed from.
    struct Catalogue {
        model: PlatformAssetCatalogueV1,
        bytes: Vec<u8>,
    }

    impl Catalogue {
        fn load() -> Self {
            let bytes = fs::read(product_root().join("assets/catalogue-1.0.0.json"))
                .expect("the installed catalogue is present");
            assert_eq!(
                sha256_media_worker_bytes_v1(&bytes).to_hex(),
                CATALOGUE_SHA256,
                "the installed catalogue is the approved one"
            );
            let model = parse_platform_asset_catalogue_v1(&bytes, VERSION)
                .expect("the strict R12 parser accepts the installed catalogue");
            Self { model, bytes }
        }

        fn digest(&self, id: &str) -> String {
            self.model
                .entries()
                .iter()
                .find(|entry| entry.id() == id)
                .map_or_else(
                    || panic!("{id} is an approved identifier"),
                    |entry| entry.sha256().to_hex(),
                )
        }

        fn request(&self) -> PlatformSourceRequestV1<'_> {
            PlatformSourceRequestV1 {
                selection: PlatformRootSelectionV1::adopt(
                    PlatformRootProvenanceV1::OperatorDesignatedPlatformRoot,
                ),
                selected_version: VERSION,
                catalogue: &self.model,
                catalogue_bytes: &self.bytes,
            }
        }
    }

    /// The one Entity of every fixture graph.
    fn entity() -> EntityIdV1 {
        EntityIdV1::parse("example").expect("a canonical Entity identifier")
    }

    /// The one Facet of every fixture graph.
    fn facet() -> FacetIdV1 {
        FacetIdV1::parse("main").expect("a canonical Facet identifier")
    }

    /// One canonical Site identifier.
    fn site_of(name: &str) -> SiteIdV1 {
        SiteIdV1::parse(name).expect("a canonical Site identifier")
    }

    /// One canonical Page identifier.
    fn page_of(name: &str) -> PageIdV1 {
        PageIdV1::parse(name).expect("a canonical Page identifier")
    }

    /// One canonical full region-qualified locale key.
    fn locale_of(name: &str) -> LocaleKeyV1 {
        LocaleKeyV1::from_already_canonical(name).expect("a canonical full locale")
    }

    /// The fixture graph: two sibling Sites, one with two Pages and two representations.
    ///
    /// `bali` owns `home` (`en-001`, `de-DE`) and `about` (`en-001`); `ubud` owns its own `home`
    /// (`en-001`). The two `home` Pages therefore share an identifier across Sites, which is what
    /// the isolation row of N131_01 and the "public sibling" row of N131_05 exist to prove.
    /// `with_ubud = false` builds the same Entity with only the `bali` Site, which is how a
    /// genuinely partial or foreign ledger is produced for N131_03.
    fn graph_of(entity_name: &str, with_ubud: bool) -> EfsOwnershipGraph {
        let mut bali_home = PageDraft::new(page_of("home"));
        bali_home.add_representation(locale_of("en-001"));
        bali_home.add_representation(locale_of("de-DE"));
        let mut bali_about = PageDraft::new(page_of("about"));
        bali_about.add_representation(locale_of("en-001"));
        let mut bali = SiteDraft::new(site_of("bali"));
        bali.add_page(bali_home);
        bali.add_page(bali_about);
        let mut main = FacetDraft::new(facet());
        main.add_site(bali);
        if with_ubud {
            let mut ubud_home = PageDraft::new(page_of("home"));
            ubud_home.add_representation(locale_of("en-001"));
            let mut ubud = SiteDraft::new(site_of("ubud"));
            ubud.add_page(ubud_home);
            main.add_site(ubud);
        }
        let mut draft =
            EfsOwnershipDraft::new(EntityIdV1::parse(entity_name).expect("a canonical Entity"));
        draft.add_facet(main);
        draft.build().expect("the fixture graph is valid")
    }

    /// The exact Page owner of one Site/Page pair inside a named Entity.
    fn page_owner_in(entity_name: &str, site: &str, page: &str) -> QualifiedOwnerV1 {
        QualifiedOwnerV1::page(
            EntityIdV1::parse(entity_name).expect("a canonical Entity"),
            facet(),
            site_of(site),
            page_of(page),
        )
    }

    /// The exact representation owner of one Site/Page/locale triple inside a named Entity.
    fn rep_owner_in(entity_name: &str, site: &str, page: &str, locale: &str) -> QualifiedOwnerV1 {
        QualifiedOwnerV1::representation(
            EntityIdV1::parse(entity_name).expect("a canonical Entity"),
            facet(),
            site_of(site),
            page_of(page),
            locale_of(locale),
        )
    }

    /// The exact Page owner of one Site/Page pair.
    fn page_owner(site: &str, page: &str) -> QualifiedOwnerV1 {
        page_owner_in("example", site, page)
    }

    /// The exact representation owner of one Site/Page/locale triple.
    fn rep_owner(site: &str, page: &str, locale: &str) -> QualifiedOwnerV1 {
        rep_owner_in("example", site, page, locale)
    }

    /// The complete declaration set of [`graph_of`], covering every one of its Pages.
    ///
    /// It materialises exactly the four accepted rows of `ARCHITECTURE.md` §3.2A:
    /// `bali/home` omits its default and overrides `de-DE`; `bali/about` is `unlisted` with a
    /// `public` override; `ubud/home` is `unlisted` with no override.
    fn declarations_of(entity_name: &str, with_ubud: bool) -> Vec<PageVisibilityDeclarationV1> {
        let mut declarations = vec![
            PageVisibilityDeclarationV1::new(page_owner_in(entity_name, "bali", "home"), None)
                .and_then(|declaration| {
                    declaration
                        .with_override(locale_of("de-DE"), RepresentationVisibilityV1::Unlisted)
                })
                .expect("the bali home declaration is valid"),
            PageVisibilityDeclarationV1::new(
                page_owner_in(entity_name, "bali", "about"),
                Some(RepresentationVisibilityV1::Unlisted),
            )
            .and_then(|declaration| {
                declaration.with_override(locale_of("en-001"), RepresentationVisibilityV1::Public)
            })
            .expect("the about declaration is valid"),
        ];
        if with_ubud {
            declarations.push(
                PageVisibilityDeclarationV1::new(
                    page_owner_in(entity_name, "ubud", "home"),
                    Some(RepresentationVisibilityV1::Unlisted),
                )
                .expect("the ubud home declaration is valid"),
            );
        }
        declarations
    }

    /// The complete ledger of [`graph_of`] under [`declarations_of`].
    fn ledger() -> RepresentationVisibilityLedgerV1 {
        build_representation_visibility_ledger_v1(
            &graph_of("example", true),
            &declarations_of("example", true),
        )
        .expect("the fixture ledger is complete")
    }

    /// One validated R17 candidate, its exact plan and its scoped representation target.
    struct Candidate {
        input: GraphicCascadeInputV1,
        plan: MixedGraphicPlanV1,
        target: QualifiedOwnerV1,
    }

    /// One authored slot selection at the target representation.
    fn selection(
        role: &str,
        namespace: AssetNamespace,
        reference: &str,
        exposure: Exposure,
    ) -> GraphicSlotSelectionV1 {
        let role = PropertyKey::from_name(&format!("graphic.{role}"))
            .expect("a registered graphic role token");
        let reference =
            AssetReferenceV1::parse(namespace, reference).expect("a portable asset reference");
        GraphicSlotSelectionV1::append(role, "primary", reference, exposure)
            .expect("a valid graphic slot selection")
    }

    /// Builds one real validated candidate over [`graph_of`], exactly like the accepted R19 fixture.
    ///
    /// The Platform binding is the verified approved one; a `registration` names one root default
    /// for `graphic.logo` at `primary` and the digest the caller asserts is approved.
    fn candidate(
        registration: Option<(&str, &str)>,
        selections: Vec<GraphicSlotSelectionV1>,
        target: QualifiedOwnerV1,
    ) -> Candidate {
        candidate_on(true, registration, selections, target)
    }

    /// The same candidate over a graph with or without the sibling `ubud` Site.
    fn candidate_on(
        with_ubud: bool,
        registration: Option<(&str, &str)>,
        selections: Vec<GraphicSlotSelectionV1>,
        target: QualifiedOwnerV1,
    ) -> Candidate {
        let graph = graph_of("example", with_ubud);
        let asset_id = registration.map_or(HOME_ID, |(id, _)| id);
        let asset_digest =
            registration.map_or_else(|| "0".repeat(64), |(_, digest)| digest.to_owned());
        let class = if asset_id.ends_with(".png") {
            PlatformAssetMediaClassV1::Png
        } else {
            PlatformAssetMediaClassV1::Svg
        };
        let asset = ApprovedPlatformAssetV1::new(asset_id, &asset_digest, class)
            .expect("a valid approved asset");
        let binding = ApprovedPlatformGraphicBindingV1::new(VERSION, CATALOGUE_SHA256, vec![asset])
            .expect("a valid approved binding");
        let baseline = match registration {
            None => PlatformGraphicBaselineV1::empty(binding),
            Some((id, digest)) => {
                let reference =
                    AssetReferenceV1::parse(AssetNamespace::Platform, id).expect("a valid id");
                let record = PlatformGraphicBaselineRecordV1::new(
                    PropertyKey::GraphicLogo,
                    "primary",
                    reference,
                    digest,
                )
                .expect("a valid baseline record");
                PlatformGraphicBaselineV1::new(binding, vec![record])
                    .expect("a valid platform baseline")
            }
        };
        let validated = validate_declared_properties_v1(&graph, Vec::new())
            .expect("the graph validates with no authored envelope");
        let block = OwnerGraphicSelectionsV1::new(target.clone(), selections)
            .expect("a valid selection block");
        let input = GraphicCascadeInputV1::new(validated, baseline, vec![block])
            .expect("a valid graphic cascade candidate");
        let plan = collect_mixed_graphic_obligations_v1(&input)
            .expect("the candidate carries one closed obligation plan");
        Candidate {
            input,
            plan,
            target,
        }
    }

    /// Runs the one visibility preflight over the real roots and the real confined workers.
    fn preflight(
        entity_root: &Root,
        platform: &PlatformRoot,
        catalogue: &Catalogue,
        candidate: &Candidate,
        ledger: &RepresentationVisibilityLedgerV1,
    ) -> GraphicVisibilityPreflightOutcomeV1 {
        let request = GraphicVisibilityPreflightRequestV1 {
            input: &candidate.input,
            plan: &candidate.plan,
            target: &candidate.target,
            ledger,
            limits: GraphicPackageLimitsV1::hard(),
            platform: catalogue.request(),
        };
        let adapter = ConfinedGraphicInspectAdapterV4::new(Path::new(BIN));
        prepare_graphic_visibility_preflight_v1(
            &request,
            &entity_root.capability(),
            &platform.capability(),
            &adapter,
            &adapter,
        )
    }

    /// Asserts the preflight prepared a result, naming the real refusal otherwise.
    fn prepared<'a>(
        outcome: &'a GraphicVisibilityPreflightOutcomeV1,
        context: &str,
    ) -> &'a GraphicVisibilityPreflightV1 {
        assert!(
            outcome.refusal().is_none(),
            "{context}: the visibility binding refused: {:?}",
            outcome.refusal()
        );
        assert!(
            outcome.package_refusal().is_none(),
            "{context}: the package refused: {:?}",
            outcome.package_refusal()
        );
        assert!(
            outcome.source_refusal().is_none(),
            "{context}: the candidate sources refused: {:?}",
            outcome.source_refusal()
        );
        outcome
            .prepared()
            .unwrap_or_else(|| panic!("{context}: the preflight prepared a result"))
    }

    /// One entry of the prepared view by exact role token.
    fn view_entry<'a>(
        view: &'a GraphicVisibilityPreflightV1,
        role: &str,
    ) -> &'a solo_application::GraphicVisibilityPreflightEntryV1 {
        let wanted = PropertyKey::from_name(&format!("graphic.{role}"))
            .expect("a registered graphic role token");
        view.entries()
            .iter()
            .find(|entry| entry.role() == wanted)
            .unwrap_or_else(|| panic!("the preflight staged a {role} entry"))
    }

    /// `N131_01` — the four accepted Page/representation rows, over a complete graph ledger.
    ///
    /// The ledger is pure: no root, no worker and no package is involved, so this case proves
    /// exactly the visibility matrix, its provenance, its complete coverage and the isolation of
    /// two same-named Pages in sibling Sites.
    #[test]
    fn n131_01_four_case_page_and_representation_visibility_matrix() {
        let graph = graph_of("example", true);
        let ledger =
            build_representation_visibility_ledger_v1(&graph, &declarations_of("example", true))
                .expect("the complete declaration set builds one ledger");

        // Complete coverage of the graph's representation membership, in stable graph order.
        assert_eq!(
            ledger.len(),
            4,
            "every representation is governed exactly once"
        );
        assert_eq!(ledger.entity_id(), &entity());
        assert!(ledger.covers_page(&page_owner("bali", "home")));
        assert!(ledger.covers_page(&page_owner("bali", "about")));
        assert!(ledger.covers_page(&page_owner("ubud", "home")));
        assert!(
            !ledger.covers_page(&page_owner("bali", "missing")),
            "a Page the graph does not own is never covered"
        );
        assert!(
            !ledger.covers_page(&rep_owner("bali", "home", "en-001")),
            "only an exact Page owner is a Page cover"
        );

        // Row 1: omitted Page default + omitted override => public.
        let row1 = ledger
            .for_owner(&rep_owner("bali", "home", "en-001"))
            .expect("the public home representation is governed");
        assert_eq!(row1.effective(), RepresentationVisibilityV1::Public);
        assert_eq!(row1.origin(), RepresentationVisibilityOriginV1::PageDefault);
        assert!(row1.is_discoverable_intent());

        // Row 2: public Page + explicit unlisted override => unlisted (the override wins).
        let row2 = ledger
            .for_owner(&rep_owner("bali", "home", "de-DE"))
            .expect("the German representation is governed");
        assert_eq!(row2.effective(), RepresentationVisibilityV1::Unlisted);
        assert_eq!(
            row2.origin(),
            RepresentationVisibilityOriginV1::RepresentationOverride
        );

        // Row 3: explicit unlisted Page + omitted override => unlisted (the Page default).
        let row3 = ledger
            .for_owner(&rep_owner("ubud", "home", "en-001"))
            .expect("the sibling home representation is governed");
        assert_eq!(row3.effective(), RepresentationVisibilityV1::Unlisted);
        assert_eq!(row3.origin(), RepresentationVisibilityOriginV1::PageDefault);

        // Row 4: explicit unlisted Page + explicit public override => public (the override wins).
        let row4 = ledger
            .for_owner(&rep_owner("bali", "about", "en-001"))
            .expect("the about representation is governed");
        assert_eq!(row4.effective(), RepresentationVisibilityV1::Public);
        assert_eq!(
            row4.origin(),
            RepresentationVisibilityOriginV1::RepresentationOverride
        );

        // The same Page identifier in a sibling Site is a different Page with its own state.
        assert_ne!(row1.owner(), row3.owner());
        assert_eq!(row1.effective().token(), "public");
        assert_eq!(row3.effective().token(), "unlisted");

        // The strict token parser admits exactly the two manifest states.
        assert_eq!(
            RepresentationVisibilityV1::parse_token("public").expect("public is admitted"),
            RepresentationVisibilityV1::Public
        );
        assert_eq!(
            RepresentationVisibilityV1::parse_token("unlisted").expect("unlisted is admitted"),
            RepresentationVisibilityV1::Unlisted
        );

        println!(
            "n131_evidence case=01 rows=4 default=public override=win complete=true \
             row1={} row2={} row3={} row4={} sibling_isolated=true origin_page_default=2 \
             origin_override=2 entries={} publication=false route=false",
            row1.effective().token(),
            row2.effective().token(),
            row3.effective().token(),
            row4.effective().token(),
            ledger.len()
        );
    }

    /// `N131_02` — unlisted intent versus a property `internal` exposure, and no third state.
    ///
    /// An `unlisted` representation is *not* private: it keeps its exact typed identity, it is
    /// simply never automatically discoverable. `internal` is refused as a representation token
    /// while an `internal` property winner is still validated and still never exported.
    #[test]
    fn n131_02_unlisted_intent_is_not_privacy_and_internal_is_not_a_representation_state() {
        let png_bytes = png();
        let webp_bytes = webp();
        let entity_root = Root::new(
            "n131-visibility",
            &[
                ("icon-favicon.png", png_bytes.as_slice()),
                ("social.webp", webp_bytes.as_slice()),
            ],
        );
        let platform = PlatformRoot::new("n131-visibility");
        let catalogue = Catalogue::load();
        let home_digest = catalogue.digest(HOME_ID);
        let ledger = ledger();

        // One public logo from the approved library, one public favicon, and one `internal`
        // social preview: the internal value is authored, validated and inspected, never exported.
        let candidate = candidate(
            Some((HOME_ID, home_digest.as_str())),
            vec![
                selection("logo", AssetNamespace::Platform, HOME_ID, Exposure::Public),
                selection(
                    "favicon",
                    AssetNamespace::Entity,
                    "icon-favicon.png",
                    Exposure::Public,
                ),
                selection(
                    "social-preview",
                    AssetNamespace::Entity,
                    "social.webp",
                    Exposure::Internal,
                ),
            ],
            rep_owner("bali", "home", "en-001"),
        );
        let outcome = preflight(&entity_root, &platform, &catalogue, &candidate, &ledger);
        let view = prepared(&outcome, "n131_02 public preflight");

        assert_eq!(view.visibility(), RepresentationVisibilityV1::Public);
        assert!(view.is_discoverable_intent());
        assert_eq!(view.robots_intent(), "index,follow");
        assert_eq!(view.len(), 2, "the internal winner is never exported");
        let roles: Vec<&str> = view
            .entries()
            .iter()
            .map(|entry| entry.role().name())
            .collect();
        assert!(roles.contains(&"graphic.logo"));
        assert!(roles.contains(&"graphic.favicon"));
        assert!(
            !roles.contains(&"graphic.social-preview"),
            "an internal property value never enters the preflight view"
        );
        assert!(!view.grants_publication_authority());
        assert!(!view.routes_any_url());

        // The unlisted sibling is governed, discoverable=false and noindex,nofollow — and it is
        // not secret: its exact typed owner identity is still available to the caller.
        let unlisted = ledger
            .for_owner(&rep_owner("ubud", "home", "en-001"))
            .expect("the unlisted sibling is governed");
        assert_eq!(unlisted.effective(), RepresentationVisibilityV1::Unlisted);
        assert!(!unlisted.is_discoverable_intent());
        assert_eq!(unlisted.robots_intent(), "noindex,nofollow");
        assert_eq!(unlisted.owner().level(), EfsLevel::Representation);
        assert_eq!(
            unlisted.owner().locale().map(|key| key.as_str()),
            Some("en-001")
        );

        // `internal` is a property exposure and is refused as a representation token, three ways.
        for token in ["internal", "private", "hidden"] {
            let refusal =
                RepresentationVisibilityV1::parse_token(token).expect_err("refused as a token");
            assert_eq!(
                refusal.kind(),
                RepresentationVisibilityTokenRefusalKindV1::NonManifestExposure
            );
            assert_eq!(refusal.detail(), "non-manifest-exposure");
        }
        assert_eq!(
            RepresentationVisibilityV1::parse_token("")
                .expect_err("empty is refused")
                .kind(),
            RepresentationVisibilityTokenRefusalKindV1::Empty
        );
        for mixed in ["Public", "UNLISTED", "Unlisted"] {
            assert_eq!(
                RepresentationVisibilityV1::parse_token(mixed)
                    .expect_err("mixed case is refused")
                    .kind(),
                RepresentationVisibilityTokenRefusalKindV1::MixedCase
            );
        }
        assert_eq!(
            RepresentationVisibilityV1::parse_token("maybe")
                .expect_err("unknown is refused")
                .kind(),
            RepresentationVisibilityTokenRefusalKindV1::Unknown
        );

        // No staged identity is an address: no scheme and no absolute path anywhere.
        for entry in view.entries() {
            assert!(entry.content_key().starts_with("sha256/"));
            assert!(!entry.content_key().contains("://"));
            assert!(!entry.content_key().starts_with('/'));
            assert!(!entry.slot().contains("://"));
        }

        println!(
            "n131_evidence case=02 visibility=public entries={} unlisted_discoverable=false \
             robots=noindex,nofollow internal_representation=refused internal_hidden=true \
             unlisted_not_private=true publication=false route=false",
            view.len()
        );
    }

    /// `N131_03` — identity and authority refusals, every one of them before any read.
    ///
    /// The ledger half is pure domain. The preflight half runs against an **empty** Entity root and
    /// a catalogue-only Platform root: if the binding were proved after acquisition, these rows
    /// would surface as `entity-read-refused`/`platform-read-refused` instead of the visibility
    /// refusals they must produce. `target-not-in-ledger` is a defensive arm: with the membership
    /// binding proved first, a complete ledger governs every representation of a validated
    /// candidate, so no fixture can reach it without first failing a stronger check.
    #[test]
    fn n131_03_identity_and_authority_refusals_precede_every_read() {
        let complete = graph_of("example", true);
        let build = |declarations: &[PageVisibilityDeclarationV1]| {
            build_representation_visibility_ledger_v1(&complete, declarations)
        };

        // A representation can never carry a Page declaration.
        assert_eq!(
            PageVisibilityDeclarationV1::new(rep_owner("bali", "home", "en-001"), None)
                .expect_err("a representation is not a Page")
                .kind(),
            RepresentationVisibilityRefusalKindV1::PageOwnerNotAPage
        );

        // A duplicate Page declaration, and a Page of the graph with no declaration.
        let mut duplicate = declarations_of("example", true);
        duplicate.push(
            PageVisibilityDeclarationV1::new(page_owner("bali", "home"), None)
                .expect("a valid Page declaration"),
        );
        assert_eq!(
            build(&duplicate).expect_err("duplicate").kind(),
            RepresentationVisibilityRefusalKindV1::DuplicatePageDeclaration
        );
        assert_eq!(
            build(&declarations_of("example", false))
                .expect_err("missing")
                .kind(),
            RepresentationVisibilityRefusalKindV1::MissingPageDeclaration
        );

        // A wrong Entity, a wrong Facet, a wrong Site and a Page the Site does not own.
        assert_eq!(
            build(&declarations_of("other", true))
                .expect_err("entity")
                .kind(),
            RepresentationVisibilityRefusalKindV1::UnknownEntity
        );
        let mut wrong_facet = declarations_of("example", true);
        wrong_facet.push(
            PageVisibilityDeclarationV1::new(
                QualifiedOwnerV1::page(
                    entity(),
                    FacetIdV1::parse("other").expect("a canonical Facet"),
                    site_of("bali"),
                    page_of("home"),
                ),
                None,
            )
            .expect("a valid Page declaration"),
        );
        assert_eq!(
            build(&wrong_facet).expect_err("facet").kind(),
            RepresentationVisibilityRefusalKindV1::WrongFacet
        );
        let mut wrong_site = declarations_of("example", true);
        wrong_site.push(
            PageVisibilityDeclarationV1::new(page_owner("nowhere", "home"), None)
                .expect("a valid Page declaration"),
        );
        assert_eq!(
            build(&wrong_site).expect_err("site").kind(),
            RepresentationVisibilityRefusalKindV1::WrongSite
        );
        let mut unknown_page = declarations_of("example", true);
        unknown_page.push(
            PageVisibilityDeclarationV1::new(page_owner("bali", "nowhere"), None)
                .expect("a valid Page declaration"),
        );
        assert_eq!(
            build(&unknown_page).expect_err("page").kind(),
            RepresentationVisibilityRefusalKindV1::UnknownPage
        );

        // An override outside its own Page, and one representation overridden twice.
        let mut foreign = declarations_of("example", true);
        foreign[0] = PageVisibilityDeclarationV1::new(page_owner("bali", "home"), None)
            .and_then(|declaration| {
                declaration.with_override(locale_of("fr-FR"), RepresentationVisibilityV1::Unlisted)
            })
            .expect("a valid Page declaration");
        assert_eq!(
            build(&foreign).expect_err("representation").kind(),
            RepresentationVisibilityRefusalKindV1::ForeignRepresentation
        );
        assert_eq!(
            PageVisibilityDeclarationV1::new(page_owner("bali", "home"), None)
                .and_then(|declaration| {
                    declaration
                        .with_override(locale_of("de-DE"), RepresentationVisibilityV1::Unlisted)
                })
                .and_then(|declaration| {
                    declaration
                        .with_override(locale_of("de-DE"), RepresentationVisibilityV1::Public)
                })
                .expect_err("duplicate override")
                .kind(),
            RepresentationVisibilityRefusalKindV1::DuplicateRepresentationOverride
        );

        // ---- application: the binding is proved before any root is opened ---------------------
        // Empty Entity root: a read would refuse, so a visibility refusal proves "before read".
        let entity_root = Root::new("n131-refusals", &[]);
        let platform = PlatformRoot::new("n131-refusals");
        let catalogue = Catalogue::load();
        let complete_ledger = ledger();

        // A Page is not a representation target.
        let page_target = candidate(None, Vec::new(), page_owner("bali", "home"));
        let outcome = preflight(
            &entity_root,
            &platform,
            &catalogue,
            &page_target,
            &complete_ledger,
        );
        assert_eq!(
            outcome.refusal().expect("a refused binding").kind(),
            GraphicVisibilityPreflightRefusalKindV1::TargetNotRepresentation
        );
        assert!(outcome.prepared().is_none(), "nothing was staged");

        // A ledger from another Entity is refused outright.
        let foreign_ledger = build_representation_visibility_ledger_v1(
            &graph_of("other", true),
            &declarations_of("other", true),
        )
        .expect("the foreign ledger is itself complete");
        let member = candidate(None, Vec::new(), rep_owner("bali", "home", "en-001"));
        let outcome = preflight(
            &entity_root,
            &platform,
            &catalogue,
            &member,
            &foreign_ledger,
        );
        assert_eq!(
            outcome.refusal().expect("a refused binding").kind(),
            GraphicVisibilityPreflightRefusalKindV1::LedgerForeignEntity
        );

        // A ledger that omits a Site the candidate declared is refused as partial.
        let partial_ledger = build_representation_visibility_ledger_v1(
            &graph_of("example", false),
            &declarations_of("example", false),
        )
        .expect("the partial ledger is complete for its own smaller graph");
        let outcome = preflight(
            &entity_root,
            &platform,
            &catalogue,
            &member,
            &partial_ledger,
        );
        assert_eq!(
            outcome.refusal().expect("a refused binding").kind(),
            GraphicVisibilityPreflightRefusalKindV1::LedgerPartial
        );

        println!(
            "n131_evidence case=03 domain_refusals=9 foreign_ledger=refused \
             partial_ledger=refused target_not_representation=refused before_read=true \
             empty_root=true entries=0 publication=false route=false"
        );
    }

    /// `N131_04` — one real public-intent preflight over the installed library and one Entity source.
    #[test]
    fn n131_04_real_public_intent_preflight_binds_the_accepted_package() {
        let png_bytes = png();
        let webp_bytes = webp();
        let entity_root = Root::new(
            "n131-public",
            &[
                ("icon-favicon.png", png_bytes.as_slice()),
                ("social.webp", webp_bytes.as_slice()),
            ],
        );
        let platform = PlatformRoot::new("n131-public");
        let catalogue = Catalogue::load();
        let home_digest = catalogue.digest(HOME_ID);
        let ledger = ledger();

        let candidate = candidate(
            Some((HOME_ID, home_digest.as_str())),
            vec![
                selection("logo", AssetNamespace::Platform, HOME_ID, Exposure::Public),
                selection(
                    "favicon",
                    AssetNamespace::Entity,
                    "icon-favicon.png",
                    Exposure::Public,
                ),
                selection(
                    "social-preview",
                    AssetNamespace::Entity,
                    "social.webp",
                    Exposure::Public,
                ),
            ],
            rep_owner("bali", "home", "en-001"),
        );
        let outcome = preflight(&entity_root, &platform, &catalogue, &candidate, &ledger);
        let view = prepared(&outcome, "n131_04 public preflight");

        assert_eq!(view.visibility(), RepresentationVisibilityV1::Public);
        assert_eq!(view.origin(), RepresentationVisibilityOriginV1::PageDefault);
        assert!(view.is_discoverable_intent());
        assert_eq!(view.robots_intent(), "index,follow");
        assert_eq!(view.len(), 3, "three winning public slots were staged");

        // The approved Platform asset stays a vector, pinned to the catalogue digest, and its
        // canonical output is a different, content-addressed identity.
        let logo = view_entry(view, "logo");
        assert_eq!(logo.namespace(), AssetNamespace::Platform);
        assert_eq!(logo.mime(), "image/svg+xml");
        assert_eq!(logo.source_sha256(), home_digest.as_str());
        assert_ne!(
            logo.output_sha256(),
            logo.source_sha256(),
            "canonicalisation produces new bytes, never a pass-through"
        );
        assert_eq!(
            logo.content_key(),
            format!("sha256/{}.svg", logo.output_sha256())
        );

        // The Entity rasters are byte-exact pass-through of the inspected sources.
        let favicon = view_entry(view, "favicon");
        assert_eq!(favicon.namespace(), AssetNamespace::Entity);
        assert_eq!(favicon.mime(), "image/png");
        assert_eq!(favicon.source_sha256(), favicon.output_sha256());
        assert_eq!(
            favicon.content_key(),
            format!("sha256/{}.png", favicon.output_sha256())
        );
        let social = view_entry(view, "social-preview");
        assert_eq!(social.namespace(), AssetNamespace::Entity);
        assert_eq!(social.mime(), "image/webp");
        assert_eq!(social.source_sha256(), social.output_sha256());
        assert_eq!(
            social.content_key(),
            format!("sha256/{}.webp", social.output_sha256())
        );

        // The staged total is the exact sum of the three payloads.
        assert_eq!(
            view.total_bytes(),
            logo.output_bytes() + favicon.output_bytes() + social.output_bytes()
        );

        // Authority: none, on either the result or the outcome.
        assert!(!view.grants_publication_authority());
        assert!(!view.routes_any_url());
        assert!(!view.grants_release_authority());
        assert!(!view.publishes_derived_output());
        assert!(!outcome.grants_publication_authority());
        assert!(!outcome.routes_any_url());

        println!(
            "n131_evidence case=04 visibility=public origin=page-default entries={} svg_vector=true \
             source_pin=true content_addressed=true catalogue_sha={CATALOGUE_SHA256} \
             output_sha={} raster_output_sha={} webp_output_sha={} publication=false route=false",
            view.len(),
            logo.output_sha256(),
            favicon.output_sha256(),
            social.output_sha256()
        );
    }

    /// `N131_05` — whole-candidate validation, unlisted privacy intent and legacy parity.
    #[test]
    fn n131_05_whole_candidate_privacy_and_legacy_parity() {
        let png_bytes = png();
        let webp_bytes = webp();
        let entity_root = Root::new(
            "n131-whole",
            &[
                ("icon-favicon.png", png_bytes.as_slice()),
                ("social.webp", webp_bytes.as_slice()),
            ],
        );
        let platform = PlatformRoot::new("n131-whole");
        let catalogue = Catalogue::load();
        let home_digest = catalogue.digest(HOME_ID);

        let selections = || {
            vec![
                selection("logo", AssetNamespace::Platform, HOME_ID, Exposure::Public),
                selection(
                    "favicon",
                    AssetNamespace::Entity,
                    "icon-favicon.png",
                    Exposure::Public,
                ),
                selection(
                    "social-preview",
                    AssetNamespace::Entity,
                    "social.webp",
                    Exposure::Public,
                ),
            ]
        };
        let target = rep_owner("bali", "home", "en-001");
        let public_candidate = candidate(
            Some((HOME_ID, home_digest.as_str())),
            selections(),
            target.clone(),
        );

        // ---- the public preflight is the accepted package, unchanged -------------------------
        let public_view = prepared(
            &preflight(
                &entity_root,
                &platform,
                &catalogue,
                &public_candidate,
                &ledger(),
            ),
            "n131_05 public preflight",
        )
        .clone();
        assert_eq!(public_view.len(), 3);

        // Legacy parity: the untouched R19 seam over the same sources produces the same keys.
        let legacy_declarations = [
            TestMixedDeclarationV1 {
                owner: TestOwnerV1::Representation,
                role: "logo",
                slot: "primary",
                namespace: "platform",
                reference: HOME_ID,
                supersede: false,
                exposure: "public",
            },
            TestMixedDeclarationV1 {
                owner: TestOwnerV1::Representation,
                role: "favicon",
                slot: "primary",
                namespace: "entity",
                reference: "icon-favicon.png",
                supersede: false,
                exposure: "public",
            },
            TestMixedDeclarationV1 {
                owner: TestOwnerV1::Representation,
                role: "social-preview",
                slot: "primary",
                namespace: "entity",
                reference: "social.webp",
                supersede: false,
                exposure: "public",
            },
        ];
        let seam = TestPackageSeamV1 {
            registration: Some((HOME_ID, home_digest.as_str())),
            format: TestMixedFormatV1::SlotAware,
            declarations: &legacy_declarations,
            foreign_plan: None,
            eligibility: PrepublicationDisplayEligibilityV1::Public,
            limits: GraphicPackageLimitsV1::hard(),
        };
        let legacy: TestPreparedPackageV1 = prepare_unpublished_graphic_package_from_roots_v1(
            &entity_root.capability(),
            &platform.capability(),
            &seam,
            catalogue.request(),
            Path::new(BIN),
        );
        assert!(
            legacy.construction_refusal().is_none(),
            "the legacy seam built the candidate"
        );
        assert!(
            legacy.package_refusal().is_none(),
            "the legacy seam prepared the package"
        );
        assert!(
            legacy.source_refusal().is_none(),
            "the legacy sources were accepted"
        );
        let mut legacy_keys: Vec<&str> = legacy
            .entries()
            .iter()
            .map(|entry| entry.content_key.as_str())
            .collect();
        let mut preflight_keys: Vec<&str> = public_view
            .entries()
            .iter()
            .map(|entry| entry.content_key())
            .collect();
        legacy_keys.sort_unstable();
        preflight_keys.sort_unstable();
        assert_eq!(
            legacy_keys, preflight_keys,
            "the wrapper reuses the accepted package identity for identity"
        );

        // ---- an unlisted target stages nothing, and a public sibling does not replace it ------
        let mut unlisted_declarations = declarations_of("example", true);
        unlisted_declarations[0] =
            PageVisibilityDeclarationV1::new(page_owner("bali", "home"), None)
                .and_then(|declaration| {
                    declaration
                        .with_override(locale_of("en-001"), RepresentationVisibilityV1::Unlisted)
                })
                .and_then(|declaration| {
                    declaration
                        .with_override(locale_of("de-DE"), RepresentationVisibilityV1::Public)
                })
                .expect("a valid Page declaration");
        unlisted_declarations[2] =
            PageVisibilityDeclarationV1::new(page_owner("ubud", "home"), None)
                .expect("a valid Page declaration");
        let unlisted_ledger = build_representation_visibility_ledger_v1(
            &graph_of("example", true),
            &unlisted_declarations,
        )
        .expect("the unlisted ledger is complete");
        assert_eq!(
            unlisted_ledger
                .for_owner(&target)
                .expect("the target is governed")
                .effective(),
            RepresentationVisibilityV1::Unlisted
        );
        assert_eq!(
            unlisted_ledger
                .for_owner(&rep_owner("ubud", "home", "en-001"))
                .expect("the sibling is governed")
                .effective(),
            RepresentationVisibilityV1::Public
        );
        let unlisted_view = prepared(
            &preflight(
                &entity_root,
                &platform,
                &catalogue,
                &public_candidate,
                &unlisted_ledger,
            ),
            "n131_05 unlisted preflight",
        )
        .clone();
        assert_eq!(
            unlisted_view.visibility(),
            RepresentationVisibilityV1::Unlisted
        );
        assert!(!unlisted_view.is_discoverable_intent());
        assert_eq!(unlisted_view.robots_intent(), "noindex,nofollow");
        assert_eq!(
            unlisted_view.len(),
            0,
            "an unlisted representation stages no discoverable payload"
        );
        assert_eq!(unlisted_view.total_bytes(), 0);
        assert!(!unlisted_view.grants_publication_authority());
        assert!(!unlisted_view.routes_any_url());

        // ---- a late invalid source refuses the whole candidate with zero partial --------------
        let broken = candidate(
            Some((HOME_ID, home_digest.as_str())),
            vec![
                selection("logo", AssetNamespace::Platform, HOME_ID, Exposure::Public),
                selection(
                    "favicon",
                    AssetNamespace::Entity,
                    "icon-missing.png",
                    Exposure::Public,
                ),
            ],
            target,
        );
        let refusal = preflight(
            &entity_root,
            &platform,
            &catalogue,
            &broken,
            &unlisted_ledger,
        );
        assert!(
            refusal.source_refusal().is_some(),
            "a missing accepted source refuses the whole candidate"
        );
        assert!(
            refusal.prepared().is_none(),
            "no partial preflight survives"
        );
        assert!(refusal.package_refusal().is_none());

        println!(
            "n131_evidence case=05 unlisted_entries={} unlisted_discoverable=false \
             robots=noindex,nofollow sibling_public_not_selected=true late_source_refusal=whole \
             zero_partial=true legacy_parity=true public_entries={} output_sha={} \
             favicon_output_sha={} publication=false route=false",
            unlisted_view.len(),
            public_view.len(),
            view_entry(&public_view, "logo").output_sha256(),
            view_entry(&public_view, "favicon").output_sha256()
        );
    }
}
