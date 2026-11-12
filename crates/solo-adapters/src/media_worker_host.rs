//! Safe façade for the one fixed, harmless Linux **pre-exec limit smoke probe** (`R03.C3`).
//!
//! `SOLO-WO-078 R01` proved that this crate can *read* the calling process's existing
//! `RLIMIT_AS` and `RLIMIT_CPU` ceilings. This module is the matching, deliberately tiny
//! second half: it starts **one** trusted child — this same `solo-cli` executable in one exact
//! private mode — and proves that the child really received two fixed ceilings *before* its
//! image was executed, by having that child read the ceilings back from its own kernel view.
//!
//! What it is:
//!
//! * one compile-time-fixed configuration. No argument, environment variable, manifest or
//!   authored document can name an executable, a ceiling, a command, a script, a destination
//!   or a mode;
//! * one child, which is this process's own executable, launched with exactly one internal flag
//!   and one fixed sub-mode token and closed stdin;
//! * a bounded parent: captured output capped at [`MAX_CHILD_STDOUT_BYTES`] and
//!   [`MAX_CHILD_STDERR_BYTES`], a real [`SMOKE_WALL_DEADLINE_MS`] millisecond watchdog, an
//!   exact zero-exit requirement and an exact single-line schema for the child's observation;
//! * a child that performs no stress, no deliberate allocation, no AVIF work, no source read,
//!   no network use, no filesystem output, no worker-protocol work and no media parsing.
//!
//! What it emphatically is not:
//!
//! * not a container, sandbox, supervisor service, process-tree sweeper or Windows Job Object.
//!   The child is forbidden to spawn a descendant, so no descendant cleanup is proved here;
//! * not proof of limit *enforcement*: a stress allocation and a CPU-burning loop are not
//!   attempted, so "an over-large request was actually rejected" is not claimed;
//! * [`std::env::current_exe`] is used only as the trusted *test* assumption that an executable
//!   can re-execute itself. It is not immutable executable identity, not anti-replacement and
//!   not a production trust root;
//! * the internal modes are a harness, not a credential. A process that arranged the same
//!   ceilings itself would satisfy the child's own check, so no cryptographic parent
//!   attestation is claimed;
//! * `RLIMIT_AS` bounds **virtual address space**, not resident memory and not an aggregate
//!   across a child tree; `RLIMIT_CPU` bounds **consumed CPU time**, not elapsed wall time.
//!
//! Every platform-native call lives in the separately gated `media_worker_host_native` module.
//! This file holds no `unsafe`, no `libc`, no `windows_sys`, no raw OS handle and no OS constant,
//! which is what lets the whole façade be reviewed as ordinary safe Rust.

use std::ffi::{OsStr, OsString};

/// The single reserved internal flag.
///
/// It belongs to no public product contract: it is absent from help and completion output, and
/// the ordinary `bootstrap-preflight` parser never sees it, because the composition root
/// dispatches it first.
pub const INTERNAL_PROBE_FLAG_V1: &str = "--internal-media-host-probe-v1";

/// The one fixed sub-mode token that asks for the supervising parent.
pub const INTERNAL_PROBE_PARENT_TOKEN_V1: &str = "parent";

/// The one fixed sub-mode token that asks for the child's own readback.
pub const INTERNAL_PROBE_CHILD_TOKEN_V1: &str = "child";

/// The fixed `RLIMIT_AS` profile: 536 870 912 bytes (512 MiB), soft **and** hard.
///
/// This is the single source of truth: the Linux pre-exec closure in
/// `media_worker_host_native` installs exactly these values, and the child verifies the kernel's
/// answer against them, so a drift between the two layers cannot pass unnoticed.
pub const PROFILE_V1_AS_BYTES: u64 = 536_870_912;

/// The fixed `RLIMIT_CPU` profile: 10 seconds of consumed CPU time, soft **and** hard.
pub const PROFILE_V1_CPU_SECONDS: u64 = 10;

/// The fixed parent watchdog: 15 000 milliseconds from spawn to a reaped leader.
pub const SMOKE_WALL_DEADLINE_MS: u64 = 15_000;

/// The most child stdout the parent will accept: one short, fixed line.
pub const MAX_CHILD_STDOUT_BYTES: usize = 1024;

/// The most child stderr the parent will capture, and then deliberately discard.
pub const MAX_CHILD_STDERR_BYTES: usize = 512;

/// The exact marker that begins the child's single observation line.
pub const CHILD_OBSERVATION_MARKER_V1: &str = "SOLO-MEDIA-HOST-PROBE-V1";

/// The exact marker that begins the parent's single accepted evidence line.
pub const PARENT_EVIDENCE_MARKER_V1: &str = "media_host_probe=accepted";

/// The exact, fixed prefix of the one refusal line an internal mode may print.
pub const INTERNAL_REFUSAL_PREFIX_V1: &str = "error: MEDIA_HOST_PROBE_REFUSED: ";

/// Exit status of a typed internal-mode refusal. It never collides with the public `0/1/2`.
pub const EXIT_INTERNAL_PROBE_REFUSED: u8 = 3;

/// Child exit status: the child's own readback did not equal the fixed profile.
const CHILD_EXIT_LIMITS_MISMATCH: u8 = 71;

/// Child exit status: the child's process-group identity was not its own pid.
const CHILD_EXIT_GROUP_MISMATCH: u8 = 72;

/// Child exit status: a required `getrlimit`/`getpgid` observation failed.
const CHILD_EXIT_READBACK_FAILED: u8 = 73;

/// The closed refusal vocabulary of the limit smoke probe.
///
/// Every variant is a fixed token with no OS error text, no path, no environment string and no
/// child output, so nothing host-specific can be leaked to an operator through it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SmokeRefusalV1 {
    /// This host has no approved Linux pre-exec boundary, so nothing was created.
    HostUnsupported,
    /// The current executable could not be resolved, or is not a usable file.
    HostExecutableUnavailable,
    /// The required fixed limits, or the required group identity, were not proved.
    HostLimitUnavailable,
    /// The child process could not be launched.
    HostSpawnFailed,
    /// The single child did not finish and reap inside the wall deadline.
    HostWallTimeout,
    /// The child exited abnormally, by signal, or with an unrecognised status.
    HostExitAbnormal,
    /// The child's output was absent, over-long, non-UTF-8 or off-schema.
    HostOutputInvalid,
    /// The single child could not be reaped, so no result may be claimed.
    HostReapFailed,
    /// The reserved internal flag was present but its token shape was wrong.
    InternalModeInvalid,
}

impl SmokeRefusalV1 {
    /// The stable, fixed token for this refusal.
    pub const fn code(self) -> &'static str {
        match self {
            Self::HostUnsupported => "HOST_UNSUPPORTED",
            Self::HostExecutableUnavailable => "HOST_EXECUTABLE_UNAVAILABLE",
            Self::HostLimitUnavailable => "HOST_LIMIT_UNAVAILABLE",
            Self::HostSpawnFailed => "HOST_SPAWN_FAILED",
            Self::HostWallTimeout => "HOST_WALL_TIMEOUT",
            Self::HostExitAbnormal => "HOST_EXIT_ABNORMAL",
            Self::HostOutputInvalid => "HOST_OUTPUT_INVALID",
            Self::HostReapFailed => "HOST_REAP_FAILED",
            Self::InternalModeInvalid => "INTERNAL_MODE_INVALID",
        }
    }
}

/// The complete, validated observation of one smoke run.
///
/// It is only ever produced after the child exited zero *and* every field was parsed strictly
/// *and* every value matched the fixed profile *and* the leader was reaped.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SmokeObservationV1 {
    /// The child's `RLIMIT_AS` soft ceiling, in bytes.
    pub address_space_soft: u64,
    /// The child's `RLIMIT_AS` hard ceiling, in bytes.
    pub address_space_hard: u64,
    /// The child's `RLIMIT_CPU` soft ceiling, in seconds.
    pub cpu_soft: u64,
    /// The child's `RLIMIT_CPU` hard ceiling, in seconds.
    pub cpu_hard: u64,
    /// The child's own process id, as the child reported it.
    pub pid: u64,
    /// The child's process-group id, which `setsid` must have made equal to `pid`.
    pub pgid: u64,
    /// The parent-measured wall time in milliseconds; `0` until the parent fills it in.
    pub wall_ms: u64,
}

/// Which reserved internal mode the composition root must run, if any.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InternalProbeModeV1 {
    /// The reserved flag is absent, so the ordinary public CLI must run unchanged.
    NotRequested,
    /// Run the one fixed supervising parent.
    Parent,
    /// Run the child's own fixed readback.
    Child,
}

/// What the composition root must emit for an internal mode, and with which exit status.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InternalProbeReportV1 {
    /// The process exit status: `0` for accepted evidence, or a fixed refusal status.
    pub exit_code: u8,
    /// The single accepted evidence line, if any.
    pub stdout: Option<String>,
    /// The single fixed refusal line, if any.
    pub stderr: Option<String>,
}

/// Classifies the reserved internal arguments, and nothing else.
///
/// The reserved flag is honoured **only** in second position, so it can never be smuggled in
/// behind a public subcommand. When the flag is present the token shape is closed: exactly one
/// sub-mode token, no duplicate, no extra token, no alternative spelling. Anything else is a
/// typed refusal rather than a fall-through, so a mistyped internal call can never silently run
/// a workload.
pub fn classify_internal_arguments_v1(
    args: &[OsString],
) -> Result<InternalProbeModeV1, SmokeRefusalV1> {
    if args.len() < 2 || args[1].as_os_str() != OsStr::new(INTERNAL_PROBE_FLAG_V1) {
        return Ok(InternalProbeModeV1::NotRequested);
    }
    if args.len() != 3 {
        return Err(SmokeRefusalV1::InternalModeInvalid);
    }
    let mode = args[2].as_os_str();
    if mode == OsStr::new(INTERNAL_PROBE_PARENT_TOKEN_V1) {
        Ok(InternalProbeModeV1::Parent)
    } else if mode == OsStr::new(INTERNAL_PROBE_CHILD_TOKEN_V1) {
        Ok(InternalProbeModeV1::Child)
    } else {
        Err(SmokeRefusalV1::InternalModeInvalid)
    }
}

/// Parses the child's single observation line, strictly.
///
/// Accepted form, exactly one line and nothing else:
///
/// ```text
/// SOLO-MEDIA-HOST-PROBE-V1 as_soft=<u64> as_hard=<u64> cpu_soft=<u64> cpu_hard=<u64> pid=<u64> pgid=<u64>
/// ```
///
/// The marker, the six field names and their order are fixed; every value is plain ASCII decimal
/// digits with no sign, no prefix, no separator and no whitespace; a trailing newline is required
/// and a second line, a carriage return, a tab, a doubled space or any unknown, duplicated,
/// missing or reordered field is refused. [`SmokeObservationV1::wall_ms`] is left at `0` because
/// only the parent measures wall time.
pub fn parse_child_observation_v1(text: &str) -> Result<SmokeObservationV1, SmokeRefusalV1> {
    let body = text
        .strip_suffix('\n')
        .ok_or(SmokeRefusalV1::HostOutputInvalid)?;
    if body.contains('\n') || body.contains('\r') || body.contains('\t') {
        return Err(SmokeRefusalV1::HostOutputInvalid);
    }

    let tokens: Vec<&str> = body.split(' ').collect();
    if tokens.len() != 7 || tokens[0] != CHILD_OBSERVATION_MARKER_V1 {
        return Err(SmokeRefusalV1::HostOutputInvalid);
    }

    Ok(SmokeObservationV1 {
        address_space_soft: field_value(tokens[1], "as_soft=")?,
        address_space_hard: field_value(tokens[2], "as_hard=")?,
        cpu_soft: field_value(tokens[3], "cpu_soft=")?,
        cpu_hard: field_value(tokens[4], "cpu_hard=")?,
        pid: field_value(tokens[5], "pid=")?,
        pgid: field_value(tokens[6], "pgid=")?,
        wall_ms: 0,
    })
}

/// Reads one `<prefix><decimal>` token, or refuses.
fn field_value(token: &str, prefix: &str) -> Result<u64, SmokeRefusalV1> {
    let raw = token
        .strip_prefix(prefix)
        .ok_or(SmokeRefusalV1::HostOutputInvalid)?;
    if raw.is_empty() || raw.len() > 20 || !raw.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(SmokeRefusalV1::HostOutputInvalid);
    }
    raw.parse::<u64>()
        .map_err(|_| SmokeRefusalV1::HostOutputInvalid)
}

/// Classifies the reaped child's exit status.
///
/// A missing code means the child was terminated by a signal, which is never success. The three
/// fixed child refusal statuses are mapped to [`SmokeRefusalV1::HostLimitUnavailable`] because
/// they mean exactly one thing — the child could not verify the required fixed limits, the
/// required group identity, or the readback syscalls that establish them. Any other non-zero
/// status is [`SmokeRefusalV1::HostExitAbnormal`].
///
/// # Errors
///
/// Returns the fixed refusal for a non-zero, signal or unrecognised status.
pub fn classify_child_exit_v1(code: Option<i32>) -> Result<(), SmokeRefusalV1> {
    match code {
        Some(0) => Ok(()),
        Some(value)
            if value == i32::from(CHILD_EXIT_LIMITS_MISMATCH)
                || value == i32::from(CHILD_EXIT_GROUP_MISMATCH)
                || value == i32::from(CHILD_EXIT_READBACK_FAILED) =>
        {
            Err(SmokeRefusalV1::HostLimitUnavailable)
        }
        _ => Err(SmokeRefusalV1::HostExitAbnormal),
    }
}

/// Validates a parsed observation against the fixed profile.
///
/// The parent re-checks what the child already checked, on purpose: evidence produced by a
/// child is never taken on trust. All four ceilings must equal the compile-time profile exactly,
/// and the child's process-group id must equal its own pid, which is what proves `setsid` ran
/// inside the pre-exec boundary.
///
/// # Errors
///
/// Returns [`SmokeRefusalV1::HostLimitUnavailable`] when any value differs.
pub fn validate_observation_v1(
    observation: SmokeObservationV1,
) -> Result<SmokeObservationV1, SmokeRefusalV1> {
    let as_ok = observation.address_space_soft == PROFILE_V1_AS_BYTES
        && observation.address_space_hard == PROFILE_V1_AS_BYTES;
    let cpu_ok = observation.cpu_soft == PROFILE_V1_CPU_SECONDS
        && observation.cpu_hard == PROFILE_V1_CPU_SECONDS;
    if !as_ok || !cpu_ok || observation.pgid != observation.pid {
        return Err(SmokeRefusalV1::HostLimitUnavailable);
    }
    Ok(observation)
}

/// Runs the reserved internal mode, if — and only if — the arguments ask for it.
///
/// `None` means the reserved flag was absent, so the ordinary public CLI must run unchanged.
pub fn run_internal_media_host_probe_v1(args: &[OsString]) -> Option<InternalProbeReportV1> {
    match classify_internal_arguments_v1(args) {
        Ok(InternalProbeModeV1::NotRequested) => None,
        Ok(InternalProbeModeV1::Parent) => Some(parent_report()),
        Ok(InternalProbeModeV1::Child) => Some(child_report()),
        Err(refusal) => Some(refusal_report(refusal)),
    }
}

/// Runs the one fixed Linux pre-exec limit smoke probe.
///
/// On Linux this starts exactly one child of this executable, whose pre-exec boundary installs a
/// new session and both hard and soft ceilings before `exec`, and returns the validated
/// observation. On every other host it refuses before creating anything.
///
/// # Errors
///
/// Returns a [`SmokeRefusalV1`] for every launch, limit, output, exit, deadline or reap failure.
#[cfg(target_os = "linux")]
pub fn run_internal_linux_limit_smoke_v1() -> Result<SmokeObservationV1, SmokeRefusalV1> {
    linux::run_smoke()
}

/// Runs the one fixed Linux pre-exec limit smoke probe, which this host cannot provide.
///
/// # Errors
///
/// Always [`SmokeRefusalV1::HostUnsupported`].
#[cfg(not(target_os = "linux"))]
pub fn run_internal_linux_limit_smoke_v1() -> Result<SmokeObservationV1, SmokeRefusalV1> {
    Err(SmokeRefusalV1::HostUnsupported)
}

/// The parent mode's whole result, as the composition root must print it.
fn parent_report() -> InternalProbeReportV1 {
    match run_internal_linux_limit_smoke_v1() {
        Ok(observation) => accepted_report(observation),
        Err(refusal) => refusal_report(refusal),
    }
}

/// The child mode's whole result, as the composition root must print it.
#[cfg(target_os = "linux")]
fn child_report() -> InternalProbeReportV1 {
    match linux::child_observation_line() {
        Ok(line) => InternalProbeReportV1 {
            exit_code: 0,
            stdout: Some(line),
            stderr: None,
        },
        Err(code) => InternalProbeReportV1 {
            exit_code: code,
            stdout: None,
            stderr: None,
        },
    }
}

/// The child mode is refused on a host with no approved pre-exec boundary.
#[cfg(not(target_os = "linux"))]
fn child_report() -> InternalProbeReportV1 {
    refusal_report(SmokeRefusalV1::HostUnsupported)
}

/// The one fixed accepted evidence line. Nothing is printed before it, and it is only built from
/// an already validated observation.
fn accepted_report(observation: SmokeObservationV1) -> InternalProbeReportV1 {
    InternalProbeReportV1 {
        exit_code: 0,
        stdout: Some(format!(
            "{PARENT_EVIDENCE_MARKER_V1} as_soft={} as_hard={} cpu_soft={} \
             cpu_hard={} pid={} pgid={} wall_ms={}\n",
            observation.address_space_soft,
            observation.address_space_hard,
            observation.cpu_soft,
            observation.cpu_hard,
            observation.pid,
            observation.pgid,
            observation.wall_ms
        )),
        stderr: None,
    }
}

/// The one fixed refusal line, carrying only the closed token.
fn refusal_report(refusal: SmokeRefusalV1) -> InternalProbeReportV1 {
    InternalProbeReportV1 {
        exit_code: EXIT_INTERNAL_PROBE_REFUSED,
        stdout: None,
        stderr: Some(format!("{INTERNAL_REFUSAL_PREFIX_V1}{}\n", refusal.code())),
    }
}

/// The Linux half: the bounded parent and the fixed child readback.
///
/// Both entry points use ordinary safe Rust only. The `unsafe` pre-exec registration and the
/// native `getrlimit`/`getpgid` calls live in `media_worker_host_native`.
#[cfg(target_os = "linux")]
mod linux {
    use std::io::Read;
    use std::process::{Command, Stdio};
    use std::time::{Duration, Instant};

    use super::{
        CHILD_EXIT_GROUP_MISMATCH, CHILD_EXIT_LIMITS_MISMATCH, CHILD_EXIT_READBACK_FAILED,
        CHILD_OBSERVATION_MARKER_V1, INTERNAL_PROBE_CHILD_TOKEN_V1, INTERNAL_PROBE_FLAG_V1,
        MAX_CHILD_STDERR_BYTES, MAX_CHILD_STDOUT_BYTES, PROFILE_V1_AS_BYTES,
        PROFILE_V1_CPU_SECONDS, SMOKE_WALL_DEADLINE_MS, SmokeObservationV1, SmokeRefusalV1,
        classify_child_exit_v1, parse_child_observation_v1, validate_observation_v1,
    };
    use crate::media_worker_host_native::{
        install_linux_child_limits_v1, own_process_group_v1, read_limit_ceilings_v1,
    };

    /// How often the parent re-checks the single child. Small enough that the 15 000 ms deadline
    /// is enforced promptly, large enough not to spin.
    const POLL_INTERVAL: Duration = Duration::from_millis(10);

    /// One capped pipe reader's outcome: the bytes it read, or the fact that it overflowed,
    /// failed or panicked. It is deliberately a plain unit error: nothing host-specific is ever
    /// carried across the pipe boundary into a report.
    type BoundedRead = Result<Vec<u8>, ()>;

    /// Launches the one fixed child and validates its observation.
    pub(super) fn run_smoke() -> Result<SmokeObservationV1, SmokeRefusalV1> {
        let executable =
            std::env::current_exe().map_err(|_| SmokeRefusalV1::HostExecutableUnavailable)?;
        if !executable.is_file() {
            return Err(SmokeRefusalV1::HostExecutableUnavailable);
        }

        // Every command, argument and pipe is built here, before the fork: the pre-exec closure
        // never resolves a path, allocates or formats anything.
        let mut command = Command::new(&executable);
        command
            .arg(INTERNAL_PROBE_FLAG_V1)
            .arg(INTERNAL_PROBE_CHILD_TOKEN_V1)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        // The limits, the new session and the closed stdin are all in place before `exec`.
        install_linux_child_limits_v1(&mut command);

        let started = Instant::now();
        let mut child = command
            .spawn()
            .map_err(|_| SmokeRefusalV1::HostSpawnFailed)?;

        // The two pipes are drained on their own threads so a child that writes a lot (or nothing
        // at all) can never deadlock the watchdog loop below. Each reader is capped, so the parent
        // memory cost of a hostile child is bounded by the cap plus one byte.
        let out_reader = child
            .stdout
            .take()
            .map(|stream| std::thread::spawn(move || read_bounded(stream, MAX_CHILD_STDOUT_BYTES)));
        let err_reader = child
            .stderr
            .take()
            .map(|stream| std::thread::spawn(move || read_bounded(stream, MAX_CHILD_STDERR_BYTES)));

        let deadline = Duration::from_millis(SMOKE_WALL_DEADLINE_MS);
        let status = loop {
            match child.try_wait() {
                Ok(Some(status)) => break status,
                Ok(None) => {}
                Err(_) => {
                    // An unobservable child is killed and reaped before refusing; a refused reap
                    // is never reported as a mere timeout.
                    let _ = child.kill();
                    let _ = child.wait();
                    return Err(SmokeRefusalV1::HostReapFailed);
                }
            }
            if started.elapsed() >= deadline {
                let _ = child.kill();
                return match child.wait() {
                    Ok(_) => Err(SmokeRefusalV1::HostWallTimeout),
                    Err(_) => Err(SmokeRefusalV1::HostReapFailed),
                };
            }
            std::thread::sleep(POLL_INTERVAL);
        };
        let wall_ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX);

        let stdout = join_reader(out_reader);
        // The child's stderr is captured, capped and discarded: it is never echoed, formatted or
        // turned into evidence.
        let _stderr = join_reader(err_reader);

        classify_child_exit_v1(status.code())?;

        let Some(Ok(bytes)) = stdout else {
            return Err(SmokeRefusalV1::HostOutputInvalid);
        };
        let text = String::from_utf8(bytes).map_err(|_| SmokeRefusalV1::HostOutputInvalid)?;
        let observation = parse_child_observation_v1(&text)?;
        let validated = validate_observation_v1(observation)?;
        Ok(SmokeObservationV1 {
            wall_ms,
            ..validated
        })
    }

    /// The fixed child readback: verify the own ceilings and group identity, then state them.
    ///
    /// The returned `Err` value is the fixed child exit status; nothing is printed on refusal, so
    /// a direct, unsupervised invocation can never produce accepted evidence.
    pub(super) fn child_observation_line() -> Result<String, u8> {
        let readback = read_limit_ceilings_v1().map_err(|_| CHILD_EXIT_READBACK_FAILED)?;

        let limits_match = readback.address_space.soft == PROFILE_V1_AS_BYTES
            && readback.address_space.hard == PROFILE_V1_AS_BYTES
            && readback.cpu.soft == PROFILE_V1_CPU_SECONDS
            && readback.cpu.hard == PROFILE_V1_CPU_SECONDS;
        if !limits_match {
            return Err(CHILD_EXIT_LIMITS_MISMATCH);
        }

        let pid = u64::from(std::process::id());
        let pgid = u64::from(own_process_group_v1().map_err(|_| CHILD_EXIT_READBACK_FAILED)?);
        if pgid != pid {
            return Err(CHILD_EXIT_GROUP_MISMATCH);
        }

        Ok(format!(
            "{CHILD_OBSERVATION_MARKER_V1} as_soft={} as_hard={} cpu_soft={} cpu_hard={} \
             pid={pid} pgid={pgid}\n",
            readback.address_space.soft,
            readback.address_space.hard,
            readback.cpu.soft,
            readback.cpu.hard
        ))
    }

    /// Reads at most `cap` bytes; one byte more is a refusal rather than a silent truncation.
    fn read_bounded(stream: impl Read, cap: usize) -> BoundedRead {
        let limit = u64::try_from(cap).map_err(|_| ())?.saturating_add(1);
        let mut buffer = Vec::new();
        let mut bounded = stream.take(limit);
        let read = bounded.read_to_end(&mut buffer).map_err(|_| ())?;
        if read > cap {
            return Err(());
        }
        Ok(buffer)
    }

    /// Joins one capped reader. A panicked or failed reader is a refusal, never empty success.
    fn join_reader(handle: Option<std::thread::JoinHandle<BoundedRead>>) -> Option<BoundedRead> {
        handle.map(|handle| handle.join().unwrap_or(Err(())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn argv(tokens: &[&str]) -> Vec<OsString> {
        let mut args = vec![OsString::from("solo-cli")];
        args.extend(tokens.iter().map(OsString::from));
        args
    }

    const WELL_FORMED: &str = "SOLO-MEDIA-HOST-PROBE-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 pid=4242 pgid=4242\n";

    fn profile_observation() -> SmokeObservationV1 {
        SmokeObservationV1 {
            address_space_soft: PROFILE_V1_AS_BYTES,
            address_space_hard: PROFILE_V1_AS_BYTES,
            cpu_soft: PROFILE_V1_CPU_SECONDS,
            cpu_hard: PROFILE_V1_CPU_SECONDS,
            pid: 4242,
            pgid: 4242,
            wall_ms: 0,
        }
    }

    /// `N079-03`: the reserved flag is honoured only in second position, and its absence is the
    /// ordinary public CLI rather than a refusal.
    #[test]
    fn n079_03_the_reserved_flag_is_honoured_only_in_second_position() {
        assert_eq!(
            classify_internal_arguments_v1(&argv(&[])),
            Ok(InternalProbeModeV1::NotRequested)
        );
        assert_eq!(
            classify_internal_arguments_v1(&argv(&["bootstrap-preflight"])),
            Ok(InternalProbeModeV1::NotRequested)
        );
        assert_eq!(
            classify_internal_arguments_v1(&argv(&[
                "bootstrap-preflight",
                INTERNAL_PROBE_FLAG_V1,
                INTERNAL_PROBE_CHILD_TOKEN_V1,
            ])),
            Ok(InternalProbeModeV1::NotRequested),
            "the reserved flag must never be picked up behind a public subcommand"
        );
        assert_eq!(
            classify_internal_arguments_v1(&argv(&[
                INTERNAL_PROBE_FLAG_V1,
                INTERNAL_PROBE_PARENT_TOKEN_V1,
            ])),
            Ok(InternalProbeModeV1::Parent)
        );
        assert_eq!(
            classify_internal_arguments_v1(&argv(&[
                INTERNAL_PROBE_FLAG_V1,
                INTERNAL_PROBE_CHILD_TOKEN_V1,
            ])),
            Ok(InternalProbeModeV1::Child)
        );
    }

    /// `N079-03`: every wrong shape of the reserved mode is one stable typed refusal.
    #[test]
    fn n079_03_every_wrong_internal_token_shape_is_a_typed_refusal() {
        let refusals: [&[&str]; 7] = [
            &[INTERNAL_PROBE_FLAG_V1],
            &[INTERNAL_PROBE_FLAG_V1, "PARENT"],
            &[INTERNAL_PROBE_FLAG_V1, "worker"],
            &[INTERNAL_PROBE_FLAG_V1, ""],
            &[INTERNAL_PROBE_FLAG_V1, "parent", "parent"],
            &[INTERNAL_PROBE_FLAG_V1, "child", "extra"],
            &[INTERNAL_PROBE_FLAG_V1, "child", "child"],
        ];
        for tokens in refusals {
            assert_eq!(
                classify_internal_arguments_v1(&argv(tokens)),
                Err(SmokeRefusalV1::InternalModeInvalid),
                "{tokens:?} must be refused, never fallen through"
            );
        }
        assert_eq!(
            SmokeRefusalV1::InternalModeInvalid.code(),
            "INTERNAL_MODE_INVALID"
        );
    }

    /// `N079-03`: exactly one well-formed line is accepted, with every field at full width.
    #[test]
    fn n079_03_only_the_exact_observation_line_is_accepted() {
        let parsed = parse_child_observation_v1(WELL_FORMED).expect("the exact line is accepted");
        assert_eq!(parsed, profile_observation());
        assert_eq!(parsed.wall_ms, 0, "only the parent measures wall time");
    }

    /// `N079-03`: no malformed, partial, duplicated, reordered or trailing evidence is accepted.
    #[test]
    fn n079_03_malformed_observations_are_refused_without_partial_values() {
        let cases: [&str; 14] = [
            "",
            "\n",
            "SOLO-MEDIA-HOST-PROBE-V1\n",
            // no trailing newline
            "SOLO-MEDIA-HOST-PROBE-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 \
             pid=4242 pgid=4242",
            // a second line
            "SOLO-MEDIA-HOST-PROBE-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 \
             pid=4242 pgid=4242\nextra\n",
            // CRLF
            "SOLO-MEDIA-HOST-PROBE-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 \
             pid=4242 pgid=4242\r\n",
            // tab separator
            "SOLO-MEDIA-HOST-PROBE-V1\tas_soft=536870912 as_hard=536870912 cpu_soft=10 \
             cpu_hard=10 pid=4242 pgid=4242\n",
            // doubled space
            "SOLO-MEDIA-HOST-PROBE-V1  as_soft=536870912 as_hard=536870912 cpu_soft=10 \
             cpu_hard=10 pid=4242 pgid=4242\n",
            // unknown marker
            "SOLO-MEDIA-HOST-PROBE-V2 as_soft=536870912 as_hard=536870912 cpu_soft=10 \
             cpu_hard=10 pid=4242 pgid=4242\n",
            // reordered fields
            "SOLO-MEDIA-HOST-PROBE-V1 as_hard=536870912 as_soft=536870912 cpu_soft=10 \
             cpu_hard=10 pid=4242 pgid=4242\n",
            // signed value
            "SOLO-MEDIA-HOST-PROBE-V1 as_soft=+536870912 as_hard=536870912 cpu_soft=10 \
             cpu_hard=10 pid=4242 pgid=4242\n",
            // empty value
            "SOLO-MEDIA-HOST-PROBE-V1 as_soft= as_hard=536870912 cpu_soft=10 cpu_hard=10 \
             pid=4242 pgid=4242\n",
            // overflow
            "SOLO-MEDIA-HOST-PROBE-V1 as_soft=99999999999999999999 as_hard=536870912 cpu_soft=10 \
             cpu_hard=10 pid=4242 pgid=4242\n",
            // an extra trailing field
            "SOLO-MEDIA-HOST-PROBE-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 \
             cpu_hard=10 pid=4242 pgid=4242 wall_ms=1\n",
        ];
        for case in cases {
            assert_eq!(
                parse_child_observation_v1(case),
                Err(SmokeRefusalV1::HostOutputInvalid),
                "{case:?} must never be accepted as evidence"
            );
        }
        assert_eq!(
            parse_child_observation_v1(
                "SOLO-MEDIA-HOST-PROBE-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 \
                 cpu_hard=10 pid=4242\n"
            ),
            Err(SmokeRefusalV1::HostOutputInvalid),
            "a missing field is refused, not defaulted"
        );
    }

    /// `N079-03`: every exit status is classified, including a signal death.
    #[test]
    fn n079_03_child_exit_statuses_are_classified() {
        assert_eq!(classify_child_exit_v1(Some(0)), Ok(()));
        for code in [71, 72, 73] {
            assert_eq!(
                classify_child_exit_v1(Some(code)),
                Err(SmokeRefusalV1::HostLimitUnavailable),
                "the child's own verification refusal must stay distinguishable"
            );
        }
        for code in [1, 2, 3, 70, 74, 137, 255] {
            assert_eq!(
                classify_child_exit_v1(Some(code)),
                Err(SmokeRefusalV1::HostExitAbnormal)
            );
        }
        assert_eq!(
            classify_child_exit_v1(None),
            Err(SmokeRefusalV1::HostExitAbnormal),
            "a signal death is never success"
        );
    }

    /// `N079-03`: a grammatically valid observation outside the fixed profile is refused.
    #[test]
    fn n079_03_values_outside_the_fixed_profile_are_refused() {
        assert_eq!(
            validate_observation_v1(profile_observation()),
            Ok(profile_observation())
        );

        let mut hard_off = profile_observation();
        hard_off.address_space_hard = 536_870_913;
        assert_eq!(
            validate_observation_v1(hard_off),
            Err(SmokeRefusalV1::HostLimitUnavailable)
        );

        let mut cpu_off = profile_observation();
        cpu_off.cpu_soft = 11;
        assert_eq!(
            validate_observation_v1(cpu_off),
            Err(SmokeRefusalV1::HostLimitUnavailable)
        );

        let mut wrong_group = profile_observation();
        wrong_group.pgid = 4243;
        assert_eq!(
            validate_observation_v1(wrong_group),
            Err(SmokeRefusalV1::HostLimitUnavailable),
            "a session that is not its own process group cannot be accepted"
        );
    }

    /// The refusal vocabulary is closed, fixed and free of host text.
    #[test]
    fn the_refusal_vocabulary_is_fixed_and_closed() {
        let codes = [
            SmokeRefusalV1::HostUnsupported,
            SmokeRefusalV1::HostExecutableUnavailable,
            SmokeRefusalV1::HostLimitUnavailable,
            SmokeRefusalV1::HostSpawnFailed,
            SmokeRefusalV1::HostWallTimeout,
            SmokeRefusalV1::HostExitAbnormal,
            SmokeRefusalV1::HostOutputInvalid,
            SmokeRefusalV1::HostReapFailed,
            SmokeRefusalV1::InternalModeInvalid,
        ];
        for refusal in codes {
            let code = refusal.code();
            assert!(code.is_ascii(), "a refusal token must be ASCII");
            assert!(
                code.bytes()
                    .all(|byte| byte.is_ascii_uppercase() || byte == b'_'),
                "a refusal token is uppercase with underscores: {code}"
            );
        }

        let report = refusal_report(SmokeRefusalV1::HostUnsupported);
        assert_eq!(report.exit_code, EXIT_INTERNAL_PROBE_REFUSED);
        assert_eq!(report.stdout, None, "a refusal never emits evidence");
        assert_eq!(
            report.stderr.as_deref(),
            Some("error: MEDIA_HOST_PROBE_REFUSED: HOST_UNSUPPORTED\n")
        );
    }
}
