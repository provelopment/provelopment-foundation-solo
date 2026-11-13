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
//!
//! # The wall-deadline negative (`SOLO-WO-081 R01`)
//!
//! The smoke above proves that a child *received* the fixed ceilings and exited promptly on its
//! own. It proves nothing about a child that does **not** exit, because the 15 000 ms deadline was
//! never exercised. This module therefore also owns one deliberately overdue, non-descending
//! negative on the same Linux seam:
//!
//! * `wall-parent` re-executes this same executable in the fixed `wall-child` mode through the
//!   **unchanged** pre-exec installer, then supervises it under a **2 000 ms** watchdog;
//! * `wall-child` performs the identical kernel readback first — all four ceiling halves and a
//!   positive `pgid == pid` — and only then writes one fixed readiness line, flushes it and
//!   sleeps exactly 20 seconds of pure wall time. It forks nothing, allocates nothing, burns no
//!   CPU, opens nothing and launches nothing;
//! * when the deadline expires the parent kills **only its own** `Child` handle, waits for the
//!   real status, requires a genuine `SIGKILL`, and reaps the leader.
//!
//! What it still does not prove: a child-**tree** sweep, any process-group signal, resistance to a
//! hostile child, `RLIMIT_AS` enforcement under stress, `RLIMIT_CPU` exhaustion (a sleep consumes
//! no CPU), executable identity, filesystem or network confinement, any Windows process control,
//! and nothing at all about media correctness or publication. The one signal number named below is
//! the Linux `SIGKILL` value the order requires in the evidence; every runtime part that touches a
//! process stays inside the Linux-gated sections.
//!
//! # The address-space reservation refusal (`SOLO-WO-083 R01`)
//!
//! Reading ceilings and terminating an overdue child still prove nothing about *enforcement of the
//! ceiling itself*. This module therefore owns exactly one further negative on the same seam:
//!
//! * `as-parent` re-executes this same executable in the fixed `as-child` mode through the
//!   **unchanged** pre-exec installer, supervises it under the existing 15 000 ms deadline and
//!   validates one strictly parsed denial line;
//! * `as-child` performs the identical kernel readback first — all four ceiling halves and a
//!   positive `pgid == pid` — and only then makes **one** fallible capacity reservation of
//!   805 306 368 bytes (768 MiB), which is larger than the whole 512 MiB `RLIMIT_AS` ceiling. The
//!   reservation is a capacity request only: no byte is initialised, indexed, filled, read, written
//!   or otherwise touched, no second attempt is made and no size is derived at runtime;
//! * a returned `Err` from that one request is the kernel/allocator refusal the negative exists to
//!   prove, and only then does the child print its single denial line and exit zero. An unexpected
//!   success is dropped untouched and refused with its own distinct status, so a host whose ceiling
//!   did not hold can never produce accepted evidence;
//! * the parent requires a normally exited, reaped leader inside its deadline, re-validates every
//!   value, and emits one bounded `media_host_as=limit_enforced … reserve=denied …` record. Every
//!   other outcome is a typed refusal with no evidence;
//! * Windows and every other non-Linux target refuse both modes as `HOST_UNSUPPORTED` before any
//!   child exists and without attempting any reservation.
//!
//! This proves one **virtual-address-space capacity request** was refused under the child's exact
//! address-space ceiling. It proves nothing about page touching, resident memory or RSS, aggregate
//! memory, the OOM killer, arbitrary allocators, `RLIMIT_CPU` exhaustion, a child tree, a sandbox
//! or any release artifact.
//!
//! # The CPU-limit kernel termination (`SOLO-WO-085 R01`)
//!
//! A refused reservation says nothing about *consumed CPU time*. This module therefore owns one
//! further, deliberately narrow proof on the same unchanged seam:
//!
//! * `cpu-parent` re-executes this same executable in the fixed `cpu-child` mode through the
//!   **unchanged** pre-exec installer (`setsid`, `RLIMIT_AS` 512 MiB soft/hard, `RLIMIT_CPU` 10 s
//!   soft/hard), supervises it with an independent **60 000 ms** wall deadline and a 10 ms poll, and
//!   never signals it while it is inside that deadline;
//! * `cpu-child` performs the identical kernel readback first — all four ceiling halves and a
//!   positive `pgid == pid` — writes and flushes one fixed readiness line, and only then enters one
//!   single-threaded, constant-space, allocation-free, deterministic arithmetic loop guarded by
//!   [`std::hint::black_box`]. It registers **no** signal handler, opens nothing, allocates nothing,
//!   sleeps nowhere, forks nothing and runs no second executable, so the only thing that can end
//!   that loop is the kernel's own CPU-time enforcement;
//! * the parent accepts one fixed evidence line only when the reaped leader's **real wait status**
//!   came from `try_wait` before any parent kill, `ExitStatusExt::signal()` returned `SIGXCPU` (24)
//!   or `SIGKILL` (9), the status was not a normal exit, the child's readiness line was captured in
//!   full and re-validated, and the parent's own measurement is inside its deadline;
//! * if the deadline expires first, the parent signals **only its own** `Child` handle, reaps it and
//!   refuses with `HOST_WALL_TIMEOUT`. That refusal is never degraded evidence, even when the
//!   resulting status happens to be `SIGKILL`;
//! * Windows and every other non-Linux target refuse both modes as `HOST_UNSUPPORTED` before any
//!   child or CPU loop exists.
//!
//! What this proves: one trusted, pre-exec-limited, descendant-free child of this executable really
//! was terminated by the kernel because its consumed CPU time reached the ceiling it had verified.
//! What it does **not** prove: aggregate CPU across a tree, resilience against a hostile child, a
//! sandbox, executable identity, wall-time precision (wall and CPU seconds are different clocks),
//! any Windows process control, memory containment, media correctness or publication.

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

/// The one fixed sub-mode token that asks for the wall-deadline supervising parent.
pub const INTERNAL_PROBE_WALL_PARENT_TOKEN_V1: &str = "wall-parent";

/// The one fixed sub-mode token that asks for the deliberately overdue wall child.
pub const INTERNAL_PROBE_WALL_CHILD_TOKEN_V1: &str = "wall-child";

/// The fixed wall watchdog of the deadline negative: **2 000 milliseconds**.
///
/// Deliberately much shorter than the child's fixed 20-second sleep and than the positive smoke's
/// 15 000 ms deadline, so the negative is decided by the watchdog rather than by the child.
pub const WALL_DEADLINE_MS: u64 = 2_000;

/// The child's fixed overdue interval: 20 000 milliseconds of pure wall time.
///
/// `std::thread::sleep` consumes no CPU, so this exercises **wall time only**: it is not an
/// `RLIMIT_CPU` exhaustion proof and not a `RLIMIT_AS` stress proof.
pub const WALL_CHILD_SLEEP_MS: u64 = 20_000;

/// The exact marker that begins the wall child's single readiness line.
pub const WALL_CHILD_READY_MARKER_V1: &str = "SOLO-MEDIA-HOST-WALL-V1";

/// The exact marker that begins the wall parent's single terminated evidence line.
///
/// It is deliberately **not** `media_host_probe=accepted`: a killed child never completed, so no
/// accepted-run wording may ever appear on this path.
pub const WALL_EVIDENCE_MARKER_V1: &str = "media_host_wall=terminated";

/// The Linux `SIGKILL` number a proven watchdog termination must report.
pub const WALL_TERMINATION_SIGNAL_V1: i32 = 9;

/// The one fixed sub-mode token that asks for the address-space reservation parent.
pub const INTERNAL_PROBE_AS_PARENT_TOKEN_V1: &str = "as-parent";

/// The one fixed sub-mode token that asks for the reservation-refusal child.
pub const INTERNAL_PROBE_AS_CHILD_TOKEN_V1: &str = "as-child";

/// The one fixed capacity the child requests: 805 306 368 bytes (768 MiB).
///
/// It is deliberately larger than the **whole** `RLIMIT_AS` ceiling the child verified, so one
/// fallible reservation must be refused by the kernel rather than by an arithmetic check here.
pub const AS_REQUEST_BYTES_V1: u64 = 805_306_368;

/// The exact marker that begins the reserved child's single denial line.
pub const AS_CHILD_MARKER_V1: &str = "SOLO-MEDIA-HOST-AS-V1";

/// The literal refusal result the child is required to report.
///
/// The only accepted value is this literal. A reservation that unexpectedly succeeded reports
/// `reserve=granted`, which is never accepted evidence.
pub const AS_DENIED_TOKEN_V1: &str = "reserve=denied";

/// The exact marker that begins the parent's single address-space evidence line.
pub const AS_EVIDENCE_MARKER_V1: &str = "media_host_as=limit_enforced";

/// The fixed parent watchdog of the reservation refusal: 15 000 milliseconds.
///
/// It is the same bounded interval as the positive smoke, reused rather than re-tuned: a child
/// that cannot reach its own denial inside this interval is a refusal, never accepted evidence.
pub const AS_DEADLINE_MS: u64 = SMOKE_WALL_DEADLINE_MS;

/// The one fixed sub-mode token that asks for the CPU-limit supervising parent.
pub const INTERNAL_PROBE_CPU_PARENT_TOKEN_V1: &str = "cpu-parent";

/// The one fixed sub-mode token that asks for the CPU-burning child.
pub const INTERNAL_PROBE_CPU_CHILD_TOKEN_V1: &str = "cpu-child";

/// The exact marker that begins the CPU child's single pre-burn readiness line.
pub const CPU_CHILD_READY_MARKER_V1: &str = "SOLO-MEDIA-HOST-CPU-V1";

/// The exact marker that begins the parent's single kernel-termination evidence line.
///
/// It is deliberately neither `media_host_probe=accepted`, `media_host_wall=terminated` nor
/// `media_host_as=limit_enforced`: a kernel CPU-limit death is a distinct result, and the parent's
/// own kill can never borrow this wording.
pub const CPU_EVIDENCE_MARKER_V1: &str = "media_host_cpu=kernel_terminated";

/// The independent parent wall deadline of the CPU-limit proof: **60 000 milliseconds**.
///
/// It is deliberately separate from — and much longer than — the 2 000 ms wall-negative watchdog
/// and the 15 000 ms positive/address-space deadlines, because the workload it bounds is the one
/// legitimate CPU burn and only the kernel may end it. Wall milliseconds are **not** consumed CPU
/// seconds, so no exact wall duration is ever asserted.
pub const CPU_DEADLINE_MS: u64 = 60_000;

/// The Linux `SIGXCPU` number: the kernel's signal at the **soft** `RLIMIT_CPU` ceiling.
pub const CPU_TERMINATION_SIGNAL_SOFT_V1: i32 = 24;

/// The Linux `SIGKILL` number: the kernel's signal at the **hard** `RLIMIT_CPU` ceiling.
pub const CPU_TERMINATION_SIGNAL_HARD_V1: i32 = 9;

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

/// Child exit status: the fixed over-ceiling reservation unexpectedly **succeeded**.
///
/// The child drops that reservation without touching a byte and refuses with this distinct
/// status, so a host whose ceiling did not hold can never produce accepted evidence.
const CHILD_EXIT_RESERVE_GRANTED: u8 = 74;

/// Child exit status: the fixed CPU loop unexpectedly **returned**.
///
/// Under the verified 10-second `RLIMIT_CPU` the kernel ends that loop with `SIGXCPU` or `SIGKILL`,
/// so a return is impossible on a conforming host. If it ever happened the child refuses with this
/// distinct status and no success line, so a loop that did not run to the ceiling can never be
/// reported as kernel enforcement.
const CHILD_EXIT_CPU_LOOP_RETURNED: u8 = 75;

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
    /// Run the one fixed supervising parent of the wall-deadline negative.
    WallParent,
    /// Run the one fixed, deliberately overdue child of the wall-deadline negative.
    WallChild,
    /// Run the one fixed supervising parent of the address-space reservation refusal.
    AsParent,
    /// Run the one fixed child that attempts the single over-ceiling reservation.
    AsChild,
    /// Run the one fixed supervising parent of the CPU-limit kernel termination.
    CpuParent,
    /// Run the one fixed child that burns CPU until the kernel ends it.
    CpuChild,
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
    } else if mode == OsStr::new(INTERNAL_PROBE_WALL_PARENT_TOKEN_V1) {
        Ok(InternalProbeModeV1::WallParent)
    } else if mode == OsStr::new(INTERNAL_PROBE_WALL_CHILD_TOKEN_V1) {
        Ok(InternalProbeModeV1::WallChild)
    } else if mode == OsStr::new(INTERNAL_PROBE_AS_PARENT_TOKEN_V1) {
        Ok(InternalProbeModeV1::AsParent)
    } else if mode == OsStr::new(INTERNAL_PROBE_AS_CHILD_TOKEN_V1) {
        Ok(InternalProbeModeV1::AsChild)
    } else if mode == OsStr::new(INTERNAL_PROBE_CPU_PARENT_TOKEN_V1) {
        Ok(InternalProbeModeV1::CpuParent)
    } else if mode == OsStr::new(INTERNAL_PROBE_CPU_CHILD_TOKEN_V1) {
        Ok(InternalProbeModeV1::CpuChild)
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

/// One validated wall-deadline termination: the reaped leader of an already limited child.
///
/// It exists only after the parent reached its own deadline, killed the one `Child` handle it
/// created, obtained that leader's real wait status and re-read the child's own readiness line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WallTerminationV1 {
    /// The child's own observation: four ceiling halves, its pid and its matching pgid.
    pub observation: SmokeObservationV1,
    /// The signal the kernel reported for the reaped leader.
    pub signal: i32,
    /// Whether the leader was actually reaped by `Child::wait`, as `1` for the evidence.
    pub reaped: bool,
}

/// Parses the wall child's single readiness line, strictly.
///
/// Accepted form, exactly one line and nothing else:
///
/// ```text
/// SOLO-MEDIA-HOST-WALL-V1 as_soft=<u64> as_hard=<u64> cpu_soft=<u64> cpu_hard=<u64> pid=<u64> pgid=<u64>
/// ```
///
/// The shape is the positive smoke's shape under a different marker, so the same closed field
/// reader, the same refusal vocabulary and the same profile check apply. On top of that the pid
/// must be positive: `0` can never be a real child of this parent.
///
/// # Errors
///
/// Returns [`SmokeRefusalV1::HostOutputInvalid`] for any shape defect and
/// [`SmokeRefusalV1::HostLimitUnavailable`] for a well-formed line outside the fixed profile.
pub fn parse_wall_child_readiness_v1(text: &str) -> Result<SmokeObservationV1, SmokeRefusalV1> {
    let body = text
        .strip_suffix('\n')
        .ok_or(SmokeRefusalV1::HostOutputInvalid)?;
    if body.contains('\n') || body.contains('\r') || body.contains('\t') {
        return Err(SmokeRefusalV1::HostOutputInvalid);
    }

    let tokens: Vec<&str> = body.split(' ').collect();
    if tokens.len() != 7 || tokens[0] != WALL_CHILD_READY_MARKER_V1 {
        return Err(SmokeRefusalV1::HostOutputInvalid);
    }

    let observation = SmokeObservationV1 {
        address_space_soft: field_value(tokens[1], "as_soft=")?,
        address_space_hard: field_value(tokens[2], "as_hard=")?,
        cpu_soft: field_value(tokens[3], "cpu_soft=")?,
        cpu_hard: field_value(tokens[4], "cpu_hard=")?,
        pid: field_value(tokens[5], "pid=")?,
        pgid: field_value(tokens[6], "pgid=")?,
        wall_ms: 0,
    };
    if observation.pid == 0 {
        return Err(SmokeRefusalV1::HostLimitUnavailable);
    }
    validate_observation_v1(observation)
}

/// Classifies the leader's real termination, and refuses everything that is not a proven kill.
///
/// A normal exit — with any code, including `0` — is never a watchdog termination, and a signal
/// other than `SIGKILL` is never accepted either: a child that died of `SIGSEGV` or `SIGTERM` was
/// not stopped by this parent's `Child::kill()`.
///
/// # Errors
///
/// [`SmokeRefusalV1::HostReapFailed`] when no reap happened, otherwise
/// [`SmokeRefusalV1::HostExitAbnormal`].
pub fn classify_wall_termination_v1(
    signal: Option<i32>,
    reaped: bool,
) -> Result<i32, SmokeRefusalV1> {
    if !reaped {
        return Err(SmokeRefusalV1::HostReapFailed);
    }
    match signal {
        Some(value) if value == WALL_TERMINATION_SIGNAL_V1 => Ok(value),
        Some(_) | None => Err(SmokeRefusalV1::HostExitAbnormal),
    }
}

/// Validates a complete wall termination against the fixed profile and the required signal.
///
/// # Errors
///
/// Any refusal from [`classify_wall_termination_v1`] or [`validate_observation_v1`], plus
/// [`SmokeRefusalV1::HostLimitUnavailable`] for a non-positive pid.
pub fn validate_wall_termination_v1(
    termination: WallTerminationV1,
) -> Result<WallTerminationV1, SmokeRefusalV1> {
    classify_wall_termination_v1(Some(termination.signal), termination.reaped)?;
    let validated = validate_observation_v1(termination.observation)?;
    if validated.pid == 0 {
        return Err(SmokeRefusalV1::HostLimitUnavailable);
    }
    // The parent's own measurement must be consistent with the fixed deadline: a watchdog that
    // fired before its 2 000 ms cannot be accepted, and a proven kill lands far inside the
    // child's 20-second overdue sleep, so a longer interval cannot be this negative at all.
    if validated.wall_ms < WALL_DEADLINE_MS || validated.wall_ms >= WALL_CHILD_SLEEP_MS {
        return Err(SmokeRefusalV1::HostWallTimeout);
    }
    Ok(WallTerminationV1 {
        observation: validated,
        ..termination
    })
}

/// The raw, still-unvalidated material one wall supervision produced.
///
/// It carries inert data only. Nothing in it is believed until [`validate_wall_attempt_v1`] has
/// parsed the readiness bytes, bounded the measured interval and re-checked the termination, which
/// is also why that validation lives here in the platform-neutral layer instead of next to the
/// Linux process code.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WallAttemptV1 {
    /// The child's captured stdout bytes, or `None` when the reader failed, overflowed or panicked.
    pub stdout: Option<Vec<u8>>,
    /// The signal the kernel reported for the reaped leader, if it reported one.
    pub signal: Option<i32>,
    /// Whether `Child::wait` actually reaped the leader.
    pub reaped: bool,
    /// The wall time the parent itself measured around spawn and reap.
    pub wall_ms: u64,
}

/// Parses, bounds and validates one raw wall attempt into accepted evidence, or refuses.
///
/// # Errors
///
/// [`SmokeRefusalV1::HostOutputInvalid`] for unreadable, over-cap, non-UTF-8 or off-schema
/// readiness bytes; otherwise the refusal from [`validate_wall_termination_v1`].
pub fn validate_wall_attempt_v1(
    attempt: WallAttemptV1,
) -> Result<WallTerminationV1, SmokeRefusalV1> {
    let signal = classify_wall_termination_v1(attempt.signal, attempt.reaped)?;
    let bytes = attempt.stdout.ok_or(SmokeRefusalV1::HostOutputInvalid)?;
    let text = String::from_utf8(bytes).map_err(|_| SmokeRefusalV1::HostOutputInvalid)?;
    let observation = parse_wall_child_readiness_v1(&text)?;
    validate_wall_termination_v1(WallTerminationV1 {
        observation: SmokeObservationV1 {
            wall_ms: attempt.wall_ms,
            ..observation
        },
        signal,
        reaped: attempt.reaped,
    })
}

/// One validated address-space reservation refusal: the child's own denial observation.
///
/// It exists only after the child exited normally with status zero, its single denial line parsed
/// exactly, every value matched the fixed profile, the literal `reserve=denied` was present, the
/// fixed request size was present and the leader was reaped inside the parent's deadline.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsRefusalV1 {
    /// The child's own observation: four ceiling halves, its pid, its matching pgid and wall time.
    pub observation: SmokeObservationV1,
    /// The exact capacity the child requested when the kernel refused it.
    pub request_bytes: u64,
}

/// The raw, still-unvalidated material one address-space supervision produced.
///
/// It carries inert data only. Nothing in it is believed until [`validate_as_attempt_v1`] has
/// classified the exit status, parsed the denial bytes and bounded the measured interval, which is
/// why that validation lives in the platform-neutral layer rather than next to the Linux code.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AsAttemptV1 {
    /// The child's captured stdout bytes, or `None` when the reader failed, overflowed or panicked.
    pub stdout: Option<Vec<u8>>,
    /// The reaped leader's normal exit code, or `None` when it died of a signal.
    pub exit_code: Option<i32>,
    /// Whether the leader was actually reaped by this parent.
    pub reaped: bool,
    /// The wall time the parent itself measured around spawn and reap.
    pub wall_ms: u64,
}

/// Parses the child's single denial line, strictly.
///
/// Accepted form, exactly one line and nothing else:
///
/// ```text
/// SOLO-MEDIA-HOST-AS-V1 as_soft=<u64> as_hard=<u64> cpu_soft=<u64> cpu_hard=<u64> pid=<u64> pgid=<u64> request_bytes=<u64> reserve=denied
/// ```
///
/// The marker, the eight field names and their order are fixed; every numeric value is plain ASCII
/// decimal digits with no sign, no prefix and no whitespace; a trailing newline is required and a
/// second line, a carriage return, a tab, a doubled space or any unknown, duplicated, missing or
/// reordered field is refused. `request_bytes` must be exactly [`AS_REQUEST_BYTES_V1`] and the
/// result token must be exactly [`AS_DENIED_TOKEN_V1`]; `reserve=granted` is never accepted.
///
/// # Errors
///
/// [`SmokeRefusalV1::HostOutputInvalid`] for any shape or literal defect, and
/// [`SmokeRefusalV1::HostLimitUnavailable`] for a well-formed line outside the fixed profile.
pub fn parse_as_child_refusal_v1(text: &str) -> Result<AsRefusalV1, SmokeRefusalV1> {
    let body = text
        .strip_suffix('\n')
        .ok_or(SmokeRefusalV1::HostOutputInvalid)?;
    if body.contains('\n') || body.contains('\r') || body.contains('\t') {
        return Err(SmokeRefusalV1::HostOutputInvalid);
    }

    let tokens: Vec<&str> = body.split(' ').collect();
    if tokens.len() != 9 || tokens[0] != AS_CHILD_MARKER_V1 {
        return Err(SmokeRefusalV1::HostOutputInvalid);
    }

    let observation = SmokeObservationV1 {
        address_space_soft: field_value(tokens[1], "as_soft=")?,
        address_space_hard: field_value(tokens[2], "as_hard=")?,
        cpu_soft: field_value(tokens[3], "cpu_soft=")?,
        cpu_hard: field_value(tokens[4], "cpu_hard=")?,
        pid: field_value(tokens[5], "pid=")?,
        pgid: field_value(tokens[6], "pgid=")?,
        wall_ms: 0,
    };
    let request_bytes = field_value(tokens[7], "request_bytes=")?;
    if tokens[8] != AS_DENIED_TOKEN_V1 {
        return Err(SmokeRefusalV1::HostOutputInvalid);
    }
    if request_bytes != AS_REQUEST_BYTES_V1 {
        return Err(SmokeRefusalV1::HostOutputInvalid);
    }
    if observation.pid == 0 {
        return Err(SmokeRefusalV1::HostLimitUnavailable);
    }
    Ok(AsRefusalV1 {
        observation: validate_observation_v1(observation)?,
        request_bytes,
    })
}

/// Classifies the reaped reservation child's exit status.
///
/// Only a normal status zero means the child reached its single reservation and observed the
/// kernel's denial. The three child verification statuses and the distinct
/// [`CHILD_EXIT_RESERVE_GRANTED`] status all mean the fixed ceilings were not proved, so they map
/// to [`SmokeRefusalV1::HostLimitUnavailable`] — a reservation that succeeded above the observed
/// ceiling is exactly a limit that did not hold. A signal death or any other status is
/// [`SmokeRefusalV1::HostExitAbnormal`].
///
/// # Errors
///
/// Returns the fixed refusal for a non-zero, signal or unrecognised status.
pub fn classify_as_child_exit_v1(code: Option<i32>) -> Result<(), SmokeRefusalV1> {
    match code {
        Some(0) => Ok(()),
        Some(value)
            if value == i32::from(CHILD_EXIT_LIMITS_MISMATCH)
                || value == i32::from(CHILD_EXIT_GROUP_MISMATCH)
                || value == i32::from(CHILD_EXIT_READBACK_FAILED)
                || value == i32::from(CHILD_EXIT_RESERVE_GRANTED) =>
        {
            Err(SmokeRefusalV1::HostLimitUnavailable)
        }
        _ => Err(SmokeRefusalV1::HostExitAbnormal),
    }
}

/// Parses, bounds and validates one raw address-space attempt into accepted evidence, or refuses.
///
/// The parent re-checks everything: an un-reaped leader, a non-zero or signal exit, absent,
/// over-cap, non-UTF-8 or off-schema bytes, a wrong request size, a literal other than
/// `reserve=denied` and an interval that is not strictly below [`AS_DEADLINE_MS`] are all typed
/// refusals with no accepted evidence.
///
/// # Errors
///
/// [`SmokeRefusalV1::HostReapFailed`] for an un-reaped leader,
/// [`SmokeRefusalV1::HostOutputInvalid`] for unusable bytes, and otherwise the refusal from
/// [`classify_as_child_exit_v1`] or [`parse_as_child_refusal_v1`].
pub fn validate_as_attempt_v1(attempt: AsAttemptV1) -> Result<AsRefusalV1, SmokeRefusalV1> {
    if !attempt.reaped {
        return Err(SmokeRefusalV1::HostReapFailed);
    }
    classify_as_child_exit_v1(attempt.exit_code)?;
    let bytes = attempt.stdout.ok_or(SmokeRefusalV1::HostOutputInvalid)?;
    let text = String::from_utf8(bytes).map_err(|_| SmokeRefusalV1::HostOutputInvalid)?;
    let refusal = parse_as_child_refusal_v1(&text)?;
    if attempt.wall_ms >= AS_DEADLINE_MS {
        return Err(SmokeRefusalV1::HostWallTimeout);
    }
    Ok(AsRefusalV1 {
        observation: SmokeObservationV1 {
            wall_ms: attempt.wall_ms,
            ..refusal.observation
        },
        request_bytes: refusal.request_bytes,
    })
}

/// The one fixed address-space enforcement line, built only from an already validated refusal.
///
/// `reserve=denied` and the fixed request size are written from the validated values, never from
/// the child's text, and the marker stays `media_host_as=limit_enforced` so a reservation refusal
/// can never be confused with the positive smoke's accepted run.
fn as_evidence_line(refusal: AsRefusalV1) -> String {
    format!(
        "{AS_EVIDENCE_MARKER_V1} as_soft={} as_hard={} cpu_soft={} cpu_hard={} request_bytes={} \
         {AS_DENIED_TOKEN_V1} pid={} pgid={} wall_ms={}\n",
        refusal.observation.address_space_soft,
        refusal.observation.address_space_hard,
        refusal.observation.cpu_soft,
        refusal.observation.cpu_hard,
        refusal.request_bytes,
        refusal.observation.pid,
        refusal.observation.pgid,
        refusal.observation.wall_ms
    )
}

/// The one fixed terminated-evidence line, built only from an already validated termination.
///
/// `signal=9 reaped=1` is written from the validated values, never from the child's text, and the
/// marker stays `media_host_wall=terminated` so a killed child can never be mistaken for a
/// completed one.
fn wall_evidence_line(termination: WallTerminationV1) -> String {
    format!(
        "{WALL_EVIDENCE_MARKER_V1} as_soft={} as_hard={} cpu_soft={} cpu_hard={} pid={} pgid={} \
         signal={} reaped={} wall_ms={}\n",
        termination.observation.address_space_soft,
        termination.observation.address_space_hard,
        termination.observation.cpu_soft,
        termination.observation.cpu_hard,
        termination.observation.pid,
        termination.observation.pgid,
        termination.signal,
        u8::from(termination.reaped),
        termination.observation.wall_ms
    )
}

/// One validated kernel CPU-limit termination of an already limited child.
///
/// It exists only after the parent observed the leader's **real wait status** from `try_wait`
/// before any parent kill, that status named `SIGXCPU` or `SIGKILL`, the child's readiness line was
/// captured in full and re-validated, and the parent's own measurement stayed inside its deadline.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CpuTerminationV1 {
    /// The child's own pre-burn observation: four ceiling halves, its pid and its matching pgid.
    pub observation: SmokeObservationV1,
    /// The signal the kernel reported for the reaped leader.
    pub signal: i32,
    /// Whether the leader was actually reaped, as `1` for the evidence.
    pub reaped: bool,
    /// Whether this parent sent the signal. It must stay `false`: a parent kill is a refusal.
    pub parent_killed: bool,
}

/// Parses the CPU child's single pre-burn readiness line, strictly.
///
/// Accepted form, exactly one line and nothing else:
///
/// ```text
/// SOLO-MEDIA-HOST-CPU-V1 as_soft=<u64> as_hard=<u64> cpu_soft=<u64> cpu_hard=<u64> pid=<u64> pgid=<u64>
/// ```
///
/// The shape is the positive smoke's shape under the CPU marker, so the same closed field reader,
/// the same refusal vocabulary and the same profile check apply. A negative or zero pid is refused
/// because it cannot be a real child of this parent.
///
/// # Errors
///
/// [`SmokeRefusalV1::HostOutputInvalid`] for any shape defect and
/// [`SmokeRefusalV1::HostLimitUnavailable`] for a well-formed line outside the fixed profile.
pub fn parse_cpu_child_readiness_v1(text: &str) -> Result<SmokeObservationV1, SmokeRefusalV1> {
    let body = text
        .strip_suffix('\n')
        .ok_or(SmokeRefusalV1::HostOutputInvalid)?;
    if body.contains('\n') || body.contains('\r') || body.contains('\t') {
        return Err(SmokeRefusalV1::HostOutputInvalid);
    }

    let tokens: Vec<&str> = body.split(' ').collect();
    if tokens.len() != 7 || tokens[0] != CPU_CHILD_READY_MARKER_V1 {
        return Err(SmokeRefusalV1::HostOutputInvalid);
    }

    let observation = SmokeObservationV1 {
        address_space_soft: field_value(tokens[1], "as_soft=")?,
        address_space_hard: field_value(tokens[2], "as_hard=")?,
        cpu_soft: field_value(tokens[3], "cpu_soft=")?,
        cpu_hard: field_value(tokens[4], "cpu_hard=")?,
        pid: field_value(tokens[5], "pid=")?,
        pgid: field_value(tokens[6], "pgid=")?,
        wall_ms: 0,
    };
    if observation.pid == 0 {
        return Err(SmokeRefusalV1::HostLimitUnavailable);
    }
    validate_observation_v1(observation)
}

/// Classifies the leader's real termination, and refuses everything that is not kernel CPU death.
///
/// A normal exit — with any code, including `0` — is never a CPU-limit termination, a signal other
/// than `SIGXCPU` or `SIGKILL` is never accepted, and a status this parent produced itself is a
/// refusal even when it happens to be `SIGKILL`: the order requires that the parent's own watchdog
/// can never be reported as kernel enforcement.
///
/// # Errors
///
/// [`SmokeRefusalV1::HostReapFailed`] when no reap happened,
/// [`SmokeRefusalV1::HostWallTimeout`] when this parent killed the child, otherwise
/// [`SmokeRefusalV1::HostExitAbnormal`].
pub fn classify_cpu_termination_v1(
    signal: Option<i32>,
    reaped: bool,
    parent_killed: bool,
) -> Result<i32, SmokeRefusalV1> {
    if !reaped {
        return Err(SmokeRefusalV1::HostReapFailed);
    }
    if parent_killed {
        return Err(SmokeRefusalV1::HostWallTimeout);
    }
    match signal {
        Some(value)
            if value == CPU_TERMINATION_SIGNAL_SOFT_V1
                || value == CPU_TERMINATION_SIGNAL_HARD_V1 =>
        {
            Ok(value)
        }
        Some(_) | None => Err(SmokeRefusalV1::HostExitAbnormal),
    }
}

/// Validates a complete CPU termination against the fixed profile and the required signals.
///
/// # Errors
///
/// Any refusal from [`classify_cpu_termination_v1`] or [`validate_observation_v1`], plus
/// [`SmokeRefusalV1::HostLimitUnavailable`] for a zero pid and
/// [`SmokeRefusalV1::HostWallTimeout`] for a measurement at or beyond the fixed 60 000 ms deadline.
pub fn validate_cpu_termination_v1(
    termination: CpuTerminationV1,
) -> Result<CpuTerminationV1, SmokeRefusalV1> {
    let signal = classify_cpu_termination_v1(
        Some(termination.signal),
        termination.reaped,
        termination.parent_killed,
    )?;
    let validated = validate_observation_v1(termination.observation)?;
    if validated.pid == 0 {
        return Err(SmokeRefusalV1::HostLimitUnavailable);
    }
    // The parent measures wall time, not CPU time: a measurement at or past its own deadline means
    // the watchdog decided this run, which is a refusal regardless of the status it produced.
    if validated.wall_ms == 0 || validated.wall_ms >= CPU_DEADLINE_MS {
        return Err(SmokeRefusalV1::HostWallTimeout);
    }
    Ok(CpuTerminationV1 {
        observation: validated,
        signal,
        reaped: true,
        parent_killed: false,
    })
}

/// The raw, still-unvalidated material one CPU supervision produced.
///
/// It carries inert data only. Nothing in it is believed until [`validate_cpu_attempt_v1`] has
/// classified the true wait status, parsed the readiness bytes and bounded the measured interval,
/// which is also why that validation lives in the platform-neutral layer instead of next to the
/// Linux process code.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CpuAttemptV1 {
    /// The child's captured stdout bytes, or `None` when the reader failed, overflowed or panicked.
    pub stdout: Option<Vec<u8>>,
    /// The signal of the leader's real wait status, or `None` when it exited normally.
    pub signal: Option<i32>,
    /// Whether the leader was actually reaped by this parent.
    pub reaped: bool,
    /// Whether this parent sent the terminating signal. `true` can never become evidence.
    pub parent_killed: bool,
    /// The wall time the parent itself measured around spawn and reap.
    pub wall_ms: u64,
}

/// Parses, bounds and validates one raw CPU attempt into accepted evidence, or refuses.
///
/// Every refusal path — a normal exit, an unexpected signal, a parent kill, a failed reap, unreadable
/// or over-cap readiness bytes, an off-profile value or an over-deadline measurement — returns a
/// typed refusal and therefore no evidence line at all.
///
/// # Errors
///
/// [`SmokeRefusalV1::HostOutputInvalid`] for unreadable, over-cap, non-UTF-8 or off-schema readiness
/// bytes; otherwise the refusal from [`validate_cpu_termination_v1`].
pub fn validate_cpu_attempt_v1(attempt: CpuAttemptV1) -> Result<CpuTerminationV1, SmokeRefusalV1> {
    let signal =
        classify_cpu_termination_v1(attempt.signal, attempt.reaped, attempt.parent_killed)?;
    let bytes = attempt.stdout.ok_or(SmokeRefusalV1::HostOutputInvalid)?;
    let text = String::from_utf8(bytes).map_err(|_| SmokeRefusalV1::HostOutputInvalid)?;
    let observation = parse_cpu_child_readiness_v1(&text)?;
    validate_cpu_termination_v1(CpuTerminationV1 {
        observation: SmokeObservationV1 {
            wall_ms: attempt.wall_ms,
            ..observation
        },
        signal,
        reaped: attempt.reaped,
        parent_killed: attempt.parent_killed,
    })
}

/// The one fixed kernel-termination line, built only from an already validated termination.
///
/// `signal`, `reaped` and `parent_killed` are written from the parent's own lifecycle values, never
/// from any child text: the child reports only its pre-burn readiness and cannot author a
/// termination claim. The marker is distinct from every earlier result, so a kernel CPU death can
/// never be read as an accepted probe, a watchdog kill or a reservation refusal.
fn cpu_evidence_line(termination: CpuTerminationV1) -> String {
    format!(
        "{CPU_EVIDENCE_MARKER_V1} as_soft={} as_hard={} cpu_soft={} cpu_hard={} pid={} pgid={} \
         signal={} reaped={} parent_killed={} wall_ms={}\n",
        termination.observation.address_space_soft,
        termination.observation.address_space_hard,
        termination.observation.cpu_soft,
        termination.observation.cpu_hard,
        termination.observation.pid,
        termination.observation.pgid,
        termination.signal,
        u8::from(termination.reaped),
        u8::from(termination.parent_killed),
        termination.observation.wall_ms
    )
}

/// Runs the reserved internal mode, if — and only if — the arguments ask for it.
///
/// `None` means the reserved flag was absent, so the ordinary public CLI must run unchanged.
pub fn run_internal_media_host_probe_v1(args: &[OsString]) -> Option<InternalProbeReportV1> {
    match classify_internal_arguments_v1(args) {
        Ok(InternalProbeModeV1::NotRequested) => None,
        Ok(InternalProbeModeV1::Parent) => Some(parent_report()),
        Ok(InternalProbeModeV1::Child) => Some(child_report()),
        Ok(InternalProbeModeV1::WallParent) => Some(wall_parent_report()),
        Ok(InternalProbeModeV1::WallChild) => Some(wall_child_report()),
        Ok(InternalProbeModeV1::AsParent) => Some(as_parent_report()),
        Ok(InternalProbeModeV1::AsChild) => Some(as_child_report()),
        Ok(InternalProbeModeV1::CpuParent) => Some(cpu_parent_report()),
        Ok(InternalProbeModeV1::CpuChild) => Some(cpu_child_report()),
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

/// Runs the one fixed Linux wall-deadline negative and returns its raw, unvalidated material.
///
/// On Linux this starts exactly one `wall-child` of this executable under the **unchanged**
/// pre-exec installer, supervises it with the fixed 2 000 ms watchdog, kills only that handle and
/// reaps the leader. On every other host it refuses before creating anything.
///
/// # Errors
///
/// Returns a [`SmokeRefusalV1`] for every launch, early-exit, signal, deadline or reap failure.
#[cfg(target_os = "linux")]
pub fn run_internal_wall_attempt_v1() -> Result<WallAttemptV1, SmokeRefusalV1> {
    linux::run_wall_probe()
}

/// Runs the one fixed Linux wall-deadline negative, which this host cannot provide.
///
/// # Errors
///
/// Always [`SmokeRefusalV1::HostUnsupported`].
#[cfg(not(target_os = "linux"))]
pub fn run_internal_wall_attempt_v1() -> Result<WallAttemptV1, SmokeRefusalV1> {
    Err(SmokeRefusalV1::HostUnsupported)
}

/// The wall parent mode's whole result, as the composition root must print it.
///
/// The validation is deliberately performed here, in the platform-neutral layer, so a refusal is
/// decided by ordinary checked code rather than by the platform branch.
fn wall_parent_report() -> InternalProbeReportV1 {
    match run_internal_wall_attempt_v1().and_then(validate_wall_attempt_v1) {
        Ok(termination) => InternalProbeReportV1 {
            exit_code: 0,
            stdout: Some(wall_evidence_line(termination)),
            stderr: None,
        },
        Err(refusal) => refusal_report(refusal),
    }
}

/// The wall child mode's whole result: it prints its own readiness line before sleeping, so the
/// report itself carries no payload.
#[cfg(target_os = "linux")]
fn wall_child_report() -> InternalProbeReportV1 {
    match linux::run_wall_child() {
        Ok(()) => InternalProbeReportV1 {
            exit_code: 0,
            stdout: None,
            stderr: None,
        },
        Err(code) => InternalProbeReportV1 {
            exit_code: code,
            stdout: None,
            stderr: None,
        },
    }
}

/// The wall child mode is refused on a host with no approved pre-exec boundary, before any child
/// of this process could ever exist.
#[cfg(not(target_os = "linux"))]
fn wall_child_report() -> InternalProbeReportV1 {
    refusal_report(SmokeRefusalV1::HostUnsupported)
}

/// Runs the one fixed Linux address-space reservation attempt and returns its raw material.
///
/// On Linux this starts exactly one `as-child` of this executable under the **unchanged** pre-exec
/// installer, supervises it with the fixed 15 000 ms watchdog, and reaps the leader. On every other
/// host it refuses before creating anything, so no reservation is ever attempted there.
///
/// # Errors
///
/// Returns a [`SmokeRefusalV1`] for every launch, exit, deadline or reap failure.
#[cfg(target_os = "linux")]
pub fn run_internal_as_attempt_v1() -> Result<AsAttemptV1, SmokeRefusalV1> {
    linux::run_as_probe()
}

/// Runs the one fixed Linux address-space reservation attempt, which this host cannot provide.
///
/// # Errors
///
/// Always [`SmokeRefusalV1::HostUnsupported`]; nothing is spawned and nothing is reserved.
#[cfg(not(target_os = "linux"))]
pub fn run_internal_as_attempt_v1() -> Result<AsAttemptV1, SmokeRefusalV1> {
    Err(SmokeRefusalV1::HostUnsupported)
}

/// Runs the one fixed Linux CPU-limit termination attempt and returns its raw, unvalidated material.
///
/// On Linux this starts exactly one `cpu-child` of this executable under the **unchanged** pre-exec
/// installer, supervises it with the fixed 60 000 ms deadline, and never signals it inside that
/// deadline. On every other host it refuses before creating anything.
///
/// # Errors
///
/// Returns a [`SmokeRefusalV1`] for every launch, unobservable-status or failed-reap failure.
#[cfg(target_os = "linux")]
pub fn run_internal_cpu_attempt_v1() -> Result<CpuAttemptV1, SmokeRefusalV1> {
    linux::run_cpu_probe()
}

/// Runs the one fixed Linux CPU-limit termination attempt, which this host cannot provide.
///
/// # Errors
///
/// Always [`SmokeRefusalV1::HostUnsupported`]; no child is created and no loop runs.
#[cfg(not(target_os = "linux"))]
pub fn run_internal_cpu_attempt_v1() -> Result<CpuAttemptV1, SmokeRefusalV1> {
    Err(SmokeRefusalV1::HostUnsupported)
}

/// The address-space parent mode's whole result, as the composition root must print it.
///
/// As with the wall negative, the validation happens here in the platform-neutral layer, so a
/// refusal is decided by ordinary checked code rather than by the platform branch.
fn as_parent_report() -> InternalProbeReportV1 {
    match run_internal_as_attempt_v1().and_then(validate_as_attempt_v1) {
        Ok(refusal) => InternalProbeReportV1 {
            exit_code: 0,
            stdout: Some(as_evidence_line(refusal)),
            stderr: None,
        },
        Err(refusal) => refusal_report(refusal),
    }
}

/// The reservation child mode's whole result.
///
/// The child itself only ever returns its single denial line or a fixed refusal status, so this
/// wrapper carries either that already bounded line or the child's own status.
#[cfg(target_os = "linux")]
fn as_child_report() -> InternalProbeReportV1 {
    match linux::run_as_child() {
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

/// The reservation child mode is refused on a host with no approved pre-exec boundary: no child is
/// created and no reservation is attempted.
#[cfg(not(target_os = "linux"))]
fn as_child_report() -> InternalProbeReportV1 {
    refusal_report(SmokeRefusalV1::HostUnsupported)
}

/// The CPU parent mode's whole result, as the composition root must print it.
///
/// As with the earlier negatives, the validation happens here in the platform-neutral layer, so a
/// refusal — including a deadline kill by this parent — is decided by ordinary checked code rather
/// than by the platform branch.
fn cpu_parent_report() -> InternalProbeReportV1 {
    match run_internal_cpu_attempt_v1().and_then(validate_cpu_attempt_v1) {
        Ok(termination) => InternalProbeReportV1 {
            exit_code: 0,
            stdout: Some(cpu_evidence_line(termination)),
            stderr: None,
        },
        Err(refusal) => refusal_report(refusal),
    }
}

/// The CPU child mode's whole result.
///
/// The child writes only its pre-burn readiness to its own stdout, so this wrapper never carries a
/// payload: a return from the CPU loop is a non-zero status with no line, and a verification
/// refusal is the child's own fixed status with no line either.
#[cfg(target_os = "linux")]
fn cpu_child_report() -> InternalProbeReportV1 {
    match linux::run_cpu_child() {
        Ok(()) => InternalProbeReportV1 {
            exit_code: 0,
            stdout: None,
            stderr: None,
        },
        Err(code) => InternalProbeReportV1 {
            exit_code: code,
            stdout: None,
            stderr: None,
        },
    }
}

/// The CPU child mode is refused on a host with no approved pre-exec boundary: no child is created
/// and no CPU workload is ever started.
#[cfg(not(target_os = "linux"))]
fn cpu_child_report() -> InternalProbeReportV1 {
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
    use std::io::{Read, Write};
    use std::os::unix::process::ExitStatusExt;
    use std::process::{Command, Stdio};
    use std::time::{Duration, Instant};

    use super::{
        AS_CHILD_MARKER_V1, AS_DEADLINE_MS, AS_DENIED_TOKEN_V1, AS_REQUEST_BYTES_V1, AsAttemptV1,
        CHILD_EXIT_CPU_LOOP_RETURNED, CHILD_EXIT_GROUP_MISMATCH, CHILD_EXIT_LIMITS_MISMATCH,
        CHILD_EXIT_READBACK_FAILED, CHILD_EXIT_RESERVE_GRANTED, CHILD_OBSERVATION_MARKER_V1,
        CPU_CHILD_READY_MARKER_V1, CPU_DEADLINE_MS, CpuAttemptV1, INTERNAL_PROBE_AS_CHILD_TOKEN_V1,
        INTERNAL_PROBE_CHILD_TOKEN_V1, INTERNAL_PROBE_CPU_CHILD_TOKEN_V1, INTERNAL_PROBE_FLAG_V1,
        INTERNAL_PROBE_WALL_CHILD_TOKEN_V1, MAX_CHILD_STDERR_BYTES, MAX_CHILD_STDOUT_BYTES,
        PROFILE_V1_AS_BYTES, PROFILE_V1_CPU_SECONDS, SMOKE_WALL_DEADLINE_MS, SmokeObservationV1,
        SmokeRefusalV1, WALL_CHILD_READY_MARKER_V1, WALL_CHILD_SLEEP_MS, WALL_DEADLINE_MS,
        WallAttemptV1, classify_child_exit_v1, parse_child_observation_v1, validate_observation_v1,
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

    /// What the 2 000 ms watchdog decided about the one owned child.
    ///
    /// The variants exist so the exit-versus-timeout race can never be reported as a kill: only
    /// [`WallOutcome::Killed`] is produced by an accepted `Child::kill()` followed by a successful
    /// `Child::wait()`, and it is the only variant that can carry a signal.
    enum WallOutcome {
        /// The child was still live at this parent's deadline, `kill` was accepted, and the
        /// leader's real termination status came back from `wait`.
        Killed(Option<i32>),
        /// The child exited by itself before the deadline. Never a proven termination.
        CompletedBeforeDeadline,
        /// The child's status could not be observed, so nothing may be claimed.
        Unobservable,
        /// The kill request was rejected, so no termination may be claimed.
        KillFailed,
        /// The leader could not be reaped after the kill.
        ReapFailed,
    }

    /// Supervises the one fixed `wall-child` and proves a real, reaped `SIGKILL` termination.
    ///
    /// The timer starts **before** `Command::spawn`, only this parent's own `Child` handle is ever
    /// signalled, and the two readers are joined only after the leader is gone.
    pub(super) fn run_wall_probe() -> Result<WallAttemptV1, SmokeRefusalV1> {
        let executable =
            std::env::current_exe().map_err(|_| SmokeRefusalV1::HostExecutableUnavailable)?;
        if !executable.is_file() {
            return Err(SmokeRefusalV1::HostExecutableUnavailable);
        }

        let mut command = Command::new(&executable);
        command
            .arg(INTERNAL_PROBE_FLAG_V1)
            .arg(INTERNAL_PROBE_WALL_CHILD_TOKEN_V1)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        // The same, unchanged pre-exec boundary: a new session and both ceilings before `exec`.
        install_linux_child_limits_v1(&mut command);

        let started = Instant::now();
        let mut child = command
            .spawn()
            .map_err(|_| SmokeRefusalV1::HostSpawnFailed)?;

        let out_reader = child
            .stdout
            .take()
            .map(|stream| std::thread::spawn(move || read_bounded(stream, MAX_CHILD_STDOUT_BYTES)));
        let err_reader = child
            .stderr
            .take()
            .map(|stream| std::thread::spawn(move || read_bounded(stream, MAX_CHILD_STDERR_BYTES)));

        let deadline = Duration::from_millis(WALL_DEADLINE_MS);
        let outcome = loop {
            match child.try_wait() {
                Ok(Some(_)) => {
                    // It finished on its own. A normal exit — even a zero exit from the child's
                    // own deadline fallback — is never this parent's termination.
                    break WallOutcome::CompletedBeforeDeadline;
                }
                Ok(None) => {}
                Err(_) => {
                    let _ = child.kill();
                    let _ = child.wait();
                    break WallOutcome::Unobservable;
                }
            }
            if started.elapsed() >= deadline {
                // Only this owned handle, only after this parent's own deadline, only while live.
                // `Child::kill` is the safe owned-handle API: no PID, no group and no other
                // process is ever signalled.
                if child.kill().is_err() {
                    let _ = child.wait();
                    break WallOutcome::KillFailed;
                }
                break match child.wait() {
                    Ok(status) => WallOutcome::Killed(status.signal()),
                    Err(_) => WallOutcome::ReapFailed,
                };
            }
            std::thread::sleep(POLL_INTERVAL);
        };
        let wall_ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX);

        // Joined only now, after the leader is gone: a live child keeps its own pipe ends open, so
        // an earlier join could block. Each reader stays capped, so the cost is bounded.
        let stdout = join_reader(out_reader);
        let _stderr = join_reader(err_reader);

        let signal = match outcome {
            WallOutcome::Killed(signal) => signal,
            WallOutcome::CompletedBeforeDeadline => {
                return Err(SmokeRefusalV1::HostExitAbnormal);
            }
            WallOutcome::Unobservable | WallOutcome::KillFailed | WallOutcome::ReapFailed => {
                return Err(SmokeRefusalV1::HostReapFailed);
            }
        };

        // Hand over raw, still-unvalidated material: the neutral layer above parses the readiness
        // bytes, bounds the interval and re-checks that the signal really is `SIGKILL`.
        Ok(WallAttemptV1 {
            stdout: stdout.and_then(Result::ok),
            signal,
            reaped: true,
            wall_ms,
        })
    }

    /// The fixed overdue child: prove the limits and the group identity, then sleep.
    ///
    /// The kernel readback and the profile check happen **before** any sleep and before any byte is
    /// written, so a direct, unsupervised invocation refuses promptly, prints nothing and never
    /// reaches the 20-second workload. The returned `Err` value is the fixed child exit status.
    pub(super) fn run_wall_child() -> Result<(), u8> {
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

        // One fixed readiness line, written and flushed before the deliberate sleep, so the parent
        // can observe the child's own kernel answer while the child is still overdue.
        let line = format!(
            "{WALL_CHILD_READY_MARKER_V1} as_soft={} as_hard={} cpu_soft={} cpu_hard={} \
             pid={pid} pgid={pgid}\n",
            readback.address_space.soft,
            readback.address_space.hard,
            readback.cpu.soft,
            readback.cpu.hard
        );
        {
            let mut stdout = std::io::stdout();
            stdout
                .write_all(line.as_bytes())
                .and_then(|()| stdout.flush())
                .map_err(|_| CHILD_EXIT_READBACK_FAILED)?;
        }

        // The deliberate overdue work: pure wall time. No descendant, no allocation, no CPU, no
        // media, no source, no network and no second executable.
        std::thread::sleep(Duration::from_millis(WALL_CHILD_SLEEP_MS));
        Ok(())
    }

    /// What the 15 000 ms supervision decided about the one owned reservation child.
    ///
    /// The variants keep the exit-versus-timeout race honest: only [`AsSupervision::Exited`] can
    /// carry a status, and only a normal zero status can become accepted evidence.
    enum AsSupervision {
        /// The child finished on its own; the value is its normal exit code, if it had one.
        Exited(Option<i32>),
        /// The child was still live at the parent's deadline; it was killed and reaped.
        TimedOut,
        /// The child's status could not be observed, so nothing may be claimed.
        Unobservable,
    }

    /// Supervises the one fixed `as-child` and hands back its raw, unvalidated material.
    ///
    /// The timer starts **before** `Command::spawn`, only the two capped readers ever touch the
    /// child's output, the leader is reaped by this parent in every path, and the neutral layer
    /// above decides whether anything may be claimed.
    pub(super) fn run_as_probe() -> Result<AsAttemptV1, SmokeRefusalV1> {
        let executable =
            std::env::current_exe().map_err(|_| SmokeRefusalV1::HostExecutableUnavailable)?;
        if !executable.is_file() {
            return Err(SmokeRefusalV1::HostExecutableUnavailable);
        }

        let mut command = Command::new(&executable);
        command
            .arg(INTERNAL_PROBE_FLAG_V1)
            .arg(INTERNAL_PROBE_AS_CHILD_TOKEN_V1)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        // The same, unchanged pre-exec boundary: a new session and both ceilings before `exec`.
        install_linux_child_limits_v1(&mut command);

        let started = Instant::now();
        let mut child = command
            .spawn()
            .map_err(|_| SmokeRefusalV1::HostSpawnFailed)?;

        let out_reader = child
            .stdout
            .take()
            .map(|stream| std::thread::spawn(move || read_bounded(stream, MAX_CHILD_STDOUT_BYTES)));
        let err_reader = child
            .stderr
            .take()
            .map(|stream| std::thread::spawn(move || read_bounded(stream, MAX_CHILD_STDERR_BYTES)));

        let deadline = Duration::from_millis(AS_DEADLINE_MS);
        let outcome = loop {
            match child.try_wait() {
                Ok(Some(status)) => break AsSupervision::Exited(status.code()),
                Ok(None) => {}
                Err(_) => {
                    let _ = child.kill();
                    let _ = child.wait();
                    break AsSupervision::Unobservable;
                }
            }
            if started.elapsed() >= deadline {
                // A hung child is killed through this parent's own handle and reaped here, so no
                // unreaped leader can outlive the run.
                let _ = child.kill();
                let _ = child.wait();
                break AsSupervision::TimedOut;
            }
            std::thread::sleep(POLL_INTERVAL);
        };
        let wall_ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX);

        // Joined only after the leader is gone, so a live child's open pipe ends cannot block.
        let stdout = join_reader(out_reader);
        let _stderr = join_reader(err_reader);

        let exit_code = match outcome {
            AsSupervision::Exited(code) => code,
            AsSupervision::TimedOut => return Err(SmokeRefusalV1::HostWallTimeout),
            AsSupervision::Unobservable => return Err(SmokeRefusalV1::HostReapFailed),
        };

        Ok(AsAttemptV1 {
            stdout: stdout.and_then(Result::ok),
            exit_code,
            reaped: true,
            wall_ms,
        })
    }

    /// The fixed reservation child: prove the own limits and group identity, then try once.
    ///
    /// The four ceiling halves and the process-group identity are verified **before** the single
    /// reservation, so a direct, unsupervised invocation refuses promptly and never reaches the
    /// 768 MiB request. When the reservation is refused — the only expected outcome under the
    /// installed 512 MiB ceiling — exactly one line is produced. If it unexpectedly succeeds, the
    /// reservation is dropped without a byte being read, written or touched, and the child refuses
    /// with its own distinct status and no accepted output.
    pub(super) fn run_as_child() -> Result<String, u8> {
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

        // Exactly one fallible capacity reservation, above the whole verified ceiling. The request
        // is a capacity reservation only: nothing here initialises, indexes, fills, reads, writes
        // or otherwise touches the requested bytes, and the vector is dropped in both branches.
        let capacity =
            usize::try_from(AS_REQUEST_BYTES_V1).map_err(|_| CHILD_EXIT_READBACK_FAILED)?;
        let mut reservation = Vec::<u8>::new();
        let reserved = reservation.try_reserve_exact(capacity);
        drop(reservation);
        if reserved.is_ok() {
            // The ceiling did not hold. Refuse without accepted evidence rather than touching it.
            return Err(CHILD_EXIT_RESERVE_GRANTED);
        }

        Ok(format!(
            "{AS_CHILD_MARKER_V1} as_soft={} as_hard={} cpu_soft={} cpu_hard={} pid={pid} \
             pgid={pgid} request_bytes={} {AS_DENIED_TOKEN_V1}\n",
            readback.address_space.soft,
            readback.address_space.hard,
            readback.cpu.soft,
            readback.cpu.hard,
            AS_REQUEST_BYTES_V1
        ))
    }

    /// What the 60 000 ms supervision decided about the one owned CPU child.
    ///
    /// The variants keep the two possible signal sources apart: only [`CpuSupervision::Observed`]
    /// can carry kernel evidence, while [`CpuSupervision::ParentKilled`] records that this parent
    /// produced the status itself and must therefore never be reported as enforcement.
    enum CpuSupervision {
        /// The child exited on its own before this parent's deadline; the value is its real signal.
        Observed(Option<i32>),
        /// The deadline expired: this parent signalled only its owned handle, and whether it reaped.
        ParentKilled(Option<i32>, bool),
        /// Neither the status nor a clean reap could be observed, so nothing may be claimed.
        Unobservable,
    }

    /// Supervises the one fixed `cpu-child` and hands back its raw, unvalidated material.
    ///
    /// The timer starts **before** `Command::spawn`, the child is launched through the unchanged
    /// pre-exec installer, only this parent's own `Child` handle is ever signalled, and the two
    /// capped readers are joined only after the leader is gone. No process group, descendant or
    /// foreign process is touched.
    pub(super) fn run_cpu_probe() -> Result<CpuAttemptV1, SmokeRefusalV1> {
        let executable =
            std::env::current_exe().map_err(|_| SmokeRefusalV1::HostExecutableUnavailable)?;
        if !executable.is_file() {
            return Err(SmokeRefusalV1::HostExecutableUnavailable);
        }

        let mut command = Command::new(&executable);
        command
            .arg(INTERNAL_PROBE_FLAG_V1)
            .arg(INTERNAL_PROBE_CPU_CHILD_TOKEN_V1)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        // The same, unchanged pre-exec boundary: a new session and both ceilings before `exec`.
        install_linux_child_limits_v1(&mut command);

        let started = Instant::now();
        let mut child = command
            .spawn()
            .map_err(|_| SmokeRefusalV1::HostSpawnFailed)?;

        let out_reader = child
            .stdout
            .take()
            .map(|stream| std::thread::spawn(move || read_bounded(stream, MAX_CHILD_STDOUT_BYTES)));
        let err_reader = child
            .stderr
            .take()
            .map(|stream| std::thread::spawn(move || read_bounded(stream, MAX_CHILD_STDERR_BYTES)));

        let deadline = Duration::from_millis(CPU_DEADLINE_MS);
        let outcome = loop {
            match child.try_wait() {
                // The true wait status of an exit that happened before any kill by this parent.
                Ok(Some(status)) => break CpuSupervision::Observed(status.signal()),
                Ok(None) => {}
                Err(_) => {
                    let _ = child.kill();
                    let _ = child.wait();
                    break CpuSupervision::Unobservable;
                }
            }
            if started.elapsed() >= deadline {
                // This parent's own watchdog expired. It signals only its owned handle, reaps the
                // leader and records that the status is its own: that is a refusal, never evidence.
                if child.kill().is_err() {
                    let _ = child.wait();
                    break CpuSupervision::Unobservable;
                }
                break match child.wait() {
                    Ok(status) => CpuSupervision::ParentKilled(status.signal(), true),
                    Err(_) => CpuSupervision::Unobservable,
                };
            }
            std::thread::sleep(POLL_INTERVAL);
        };
        let wall_ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX);

        // Joined only after the leader is gone, so a live child's open pipe ends cannot block.
        let stdout = join_reader(out_reader);
        let _stderr = join_reader(err_reader);

        match outcome {
            CpuSupervision::Observed(signal) => Ok(CpuAttemptV1 {
                stdout: stdout.and_then(Result::ok),
                signal,
                reaped: true,
                parent_killed: false,
                wall_ms,
            }),
            CpuSupervision::ParentKilled(signal, reaped) => Ok(CpuAttemptV1 {
                stdout: stdout.and_then(Result::ok),
                signal,
                reaped,
                parent_killed: true,
                wall_ms,
            }),
            CpuSupervision::Unobservable => Err(SmokeRefusalV1::HostReapFailed),
        }
    }

    /// The fixed CPU child: prove the own limits and group identity, then burn CPU until the kernel
    /// ends this process.
    ///
    /// The four ceiling halves and the process-group identity are verified **before** the readiness
    /// line and before any loop iteration, so a direct, unsupervised invocation refuses promptly,
    /// prints nothing and never reaches the workload. The loop itself is single-threaded, constant
    /// space, allocation-free and syscall-free, and it installs no signal handler: only the kernel's
    /// own `RLIMIT_CPU` enforcement can end it. A return from it is refused with its own fixed
    /// status and no success line.
    pub(super) fn run_cpu_child() -> Result<(), u8> {
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

        // One fixed readiness line, written and flushed before the burn, so the parent can validate
        // the child's own kernel answer even though the child can never report its own death.
        let line = format!(
            "{CPU_CHILD_READY_MARKER_V1} as_soft={} as_hard={} cpu_soft={} cpu_hard={} \
             pid={pid} pgid={pgid}\n",
            readback.address_space.soft,
            readback.address_space.hard,
            readback.cpu.soft,
            readback.cpu.hard
        );
        {
            let mut stdout = std::io::stdout();
            stdout
                .write_all(line.as_bytes())
                .and_then(|()| stdout.flush())
                .map_err(|_| CHILD_EXIT_READBACK_FAILED)?;
        }

        // The one authorized CPU workload, reached only with the ceilings verified above.
        burn_cpu();

        // Reachable only if that loop returned, which a conforming kernel never allows under the
        // verified 10-second ceiling: refuse with a distinct status and without any success line.
        Err(CHILD_EXIT_CPU_LOOP_RETURNED)
    }

    /// The one fixed CPU-only workload: single-threaded, constant space, deterministic.
    ///
    /// It allocates nothing, sleeps nowhere, yields nowhere, opens nothing, forks nothing and calls
    /// no OS service, so the only thing that can end it is the kernel's own CPU-time enforcement.
    /// `wrapping_*` arithmetic keeps it deterministic and [`std::hint::black_box`] keeps it alive
    /// through optimisation; the guarded `return` documents that this function is expected never to
    /// return while still letting the caller refuse if it somehow does.
    fn burn_cpu() {
        let mut state: u64 = 0x9E37_79B9_7F4A_7C15;
        loop {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            state ^= state.rotate_left(17);
            state = std::hint::black_box(state);
            if std::hint::black_box(false) {
                return;
            }
        }
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

    /// The canonical readiness line the fixed `wall-child` writes before it sleeps.
    ///
    /// It is built from the real marker constant, so a changed marker cannot pass unobserved.
    fn wall_ready_line() -> String {
        format!(
            "{WALL_CHILD_READY_MARKER_V1} as_soft=536870912 as_hard=536870912 cpu_soft=10 \
             cpu_hard=10 pid=5252 pgid=5252\n"
        )
    }

    fn wall_observation() -> SmokeObservationV1 {
        SmokeObservationV1 {
            address_space_soft: PROFILE_V1_AS_BYTES,
            address_space_hard: PROFILE_V1_AS_BYTES,
            cpu_soft: PROFILE_V1_CPU_SECONDS,
            cpu_hard: PROFILE_V1_CPU_SECONDS,
            pid: 5252,
            pgid: 5252,
            wall_ms: 2_004,
        }
    }

    fn wall_termination() -> WallTerminationV1 {
        WallTerminationV1 {
            observation: wall_observation(),
            signal: WALL_TERMINATION_SIGNAL_V1,
            reaped: true,
        }
    }

    /// `N081-03`: the two wall tokens are additive and closed, and the legacy tokens are unchanged.
    #[test]
    fn n081_03_the_wall_tokens_are_closed_and_additive() {
        assert_eq!(
            classify_internal_arguments_v1(&argv(&[
                INTERNAL_PROBE_FLAG_V1,
                INTERNAL_PROBE_WALL_PARENT_TOKEN_V1,
            ])),
            Ok(InternalProbeModeV1::WallParent)
        );
        assert_eq!(
            classify_internal_arguments_v1(&argv(&[
                INTERNAL_PROBE_FLAG_V1,
                INTERNAL_PROBE_WALL_CHILD_TOKEN_V1,
            ])),
            Ok(InternalProbeModeV1::WallChild)
        );

        // The legacy pair still classifies exactly as before this order.
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

        let refusals: [&[&str]; 8] = [
            &[INTERNAL_PROBE_FLAG_V1, "wall"],
            &[INTERNAL_PROBE_FLAG_V1, "WALL-PARENT"],
            &[INTERNAL_PROBE_FLAG_V1, "wall-parent", "wall-parent"],
            &[INTERNAL_PROBE_FLAG_V1, "wall-child", "extra"],
            &[INTERNAL_PROBE_FLAG_V1, "wall_child"],
            &[INTERNAL_PROBE_FLAG_V1, "wall-parent "],
            &[INTERNAL_PROBE_FLAG_V1, ""],
            &[INTERNAL_PROBE_FLAG_V1],
        ];
        for tokens in refusals {
            assert_eq!(
                classify_internal_arguments_v1(&argv(tokens)),
                Err(SmokeRefusalV1::InternalModeInvalid),
                "{tokens:?} must be refused, never fallen through"
            );
        }

        // Neither wall token is reachable behind a public subcommand.
        for token in [
            INTERNAL_PROBE_WALL_PARENT_TOKEN_V1,
            INTERNAL_PROBE_WALL_CHILD_TOKEN_V1,
        ] {
            assert_eq!(
                classify_internal_arguments_v1(&argv(&["bootstrap-preflight", token])),
                Ok(InternalProbeModeV1::NotRequested)
            );
        }
    }

    /// `N081-01`: the fixed numbers this order requires cannot drift unnoticed.
    ///
    /// These are `const` assertions, so the compiler evaluates them for every build of this crate
    /// on every host; the same numbers are also checked against what the real child reported in
    /// the integration suite.
    #[test]
    fn n081_01_the_wall_negative_uses_the_fixed_deadline_and_profile() {
        const {
            assert!(
                WALL_DEADLINE_MS == 2_000,
                "the watchdog is exactly 2 000 ms"
            );
            assert!(WALL_CHILD_SLEEP_MS == 20_000, "the child sleeps 20 seconds");
            assert!(WALL_DEADLINE_MS < WALL_CHILD_SLEEP_MS);
            assert!(
                SMOKE_WALL_DEADLINE_MS == 15_000,
                "the positive smoke deadline is unchanged"
            );
            assert!(PROFILE_V1_AS_BYTES == 536_870_912);
            assert!(PROFILE_V1_CPU_SECONDS == 10);
            assert!(
                WALL_TERMINATION_SIGNAL_V1 == 9,
                "a proven kill reports SIGKILL"
            );
            assert!(MAX_CHILD_STDOUT_BYTES == 1024);
            assert!(MAX_CHILD_STDERR_BYTES == 512);
        }
    }

    /// `N081-04`: every malformed, duplicated, oversized, negative, overflowing or out-of-profile
    /// readiness line is a closed refusal, and a good one is accepted unchanged.
    #[test]
    fn n081_04_malformed_wall_readiness_is_refused() {
        assert_eq!(
            parse_wall_child_readiness_v1(&wall_ready_line()),
            Ok(SmokeObservationV1 {
                wall_ms: 0,
                ..wall_observation()
            }),
            "the canonical line must be accepted, with the parent's own wall time still unset"
        );

        let malformed: [&str; 14] = [
            "",
            "\n",
            "SOLO-MEDIA-HOST-WALL-V1\n",
            // the positive smoke's marker is not this one
            "SOLO-MEDIA-HOST-PROBE-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 \
             cpu_hard=10 pid=5252 pgid=5252\n",
            // missing trailing newline
            "SOLO-MEDIA-HOST-WALL-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 \
             cpu_hard=10 pid=5252 pgid=5252",
            // missing field
            "SOLO-MEDIA-HOST-WALL-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 \
             cpu_hard=10 pid=5252\n",
            // extra field
            "SOLO-MEDIA-HOST-WALL-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 \
             cpu_hard=10 pid=5252 pgid=5252 signal=9\n",
            // duplicated field
            "SOLO-MEDIA-HOST-WALL-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 \
             cpu_hard=10 pid=5252 pid=5252\n",
            // negative value
            "SOLO-MEDIA-HOST-WALL-V1 as_soft=-536870912 as_hard=536870912 cpu_soft=10 \
             cpu_hard=10 pid=5252 pgid=5252\n",
            // overflow past u64
            "SOLO-MEDIA-HOST-WALL-V1 as_soft=99999999999999999999 as_hard=536870912 cpu_soft=10 \
             cpu_hard=10 pid=5252 pgid=5252\n",
            // a second line
            "SOLO-MEDIA-HOST-WALL-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 \
             cpu_hard=10 pid=5252 pgid=5252\nextra\n",
            // a carriage return
            "SOLO-MEDIA-HOST-WALL-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 \
             cpu_hard=10 pid=5252 pgid=5252\r\n",
            // a doubled space
            "SOLO-MEDIA-HOST-WALL-V1  as_soft=536870912 as_hard=536870912 cpu_soft=10 \
             cpu_hard=10 pid=5252 pgid=5252\n",
            // reordered fields
            "SOLO-MEDIA-HOST-WALL-V1 as_hard=536870912 as_soft=536870912 cpu_soft=10 \
             cpu_hard=10 pid=5252 pgid=5252\n",
        ];
        for line in malformed {
            assert_eq!(
                parse_wall_child_readiness_v1(line),
                Err(SmokeRefusalV1::HostOutputInvalid),
                "{line:?} must be refused as invalid output"
            );
        }
    }

    /// `N081-04`: a well-formed line outside the fixed profile, or with an impossible identity, is
    /// refused rather than accepted as proof.
    #[test]
    fn n081_04_out_of_profile_wall_readiness_is_refused() {
        let off_profile: [(&str, SmokeRefusalV1); 5] = [
            (
                "SOLO-MEDIA-HOST-WALL-V1 as_soft=536870913 as_hard=536870912 cpu_soft=10 \
                 cpu_hard=10 pid=5252 pgid=5252\n",
                SmokeRefusalV1::HostLimitUnavailable,
            ),
            (
                "SOLO-MEDIA-HOST-WALL-V1 as_soft=536870912 as_hard=536870912 cpu_soft=11 \
                 cpu_hard=10 pid=5252 pgid=5252\n",
                SmokeRefusalV1::HostLimitUnavailable,
            ),
            (
                "SOLO-MEDIA-HOST-WALL-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 \
                 cpu_hard=10 pid=5252 pgid=5253\n",
                SmokeRefusalV1::HostLimitUnavailable,
            ),
            (
                "SOLO-MEDIA-HOST-WALL-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 \
                 cpu_hard=10 pid=0 pgid=0\n",
                SmokeRefusalV1::HostLimitUnavailable,
            ),
            (
                "SOLO-MEDIA-HOST-WALL-V1 as_soft=18446744073709551615 \
                 as_hard=18446744073709551615 cpu_soft=10 cpu_hard=10 pid=5252 pgid=5252\n",
                SmokeRefusalV1::HostLimitUnavailable,
            ),
        ];
        for (line, refusal) in off_profile {
            assert_eq!(
                parse_wall_child_readiness_v1(line),
                Err(refusal),
                "{line:?} must be refused as out of profile"
            );
        }
    }

    /// `N081-04`: only a reaped `SIGKILL` is a termination; nothing else may ever be accepted.
    #[test]
    fn n081_04_only_a_reaped_sigkill_is_a_termination() {
        assert_eq!(
            classify_wall_termination_v1(Some(WALL_TERMINATION_SIGNAL_V1), true),
            Ok(WALL_TERMINATION_SIGNAL_V1)
        );
        for signal in [None, Some(0), Some(1), Some(2), Some(11), Some(15)] {
            assert_eq!(
                classify_wall_termination_v1(signal, true),
                Err(SmokeRefusalV1::HostExitAbnormal),
                "{signal:?} is not a proven watchdog kill"
            );
        }
        assert_eq!(
            classify_wall_termination_v1(Some(WALL_TERMINATION_SIGNAL_V1), false),
            Err(SmokeRefusalV1::HostReapFailed),
            "an un-reaped leader can never be accepted"
        );

        assert_eq!(
            validate_wall_termination_v1(wall_termination()),
            Ok(wall_termination())
        );

        let mut wrong_signal = wall_termination();
        wrong_signal.signal = 15;
        assert_eq!(
            validate_wall_termination_v1(wrong_signal),
            Err(SmokeRefusalV1::HostExitAbnormal)
        );

        let mut unreaped = wall_termination();
        unreaped.reaped = false;
        assert_eq!(
            validate_wall_termination_v1(unreaped),
            Err(SmokeRefusalV1::HostReapFailed)
        );

        let mut wrong_limits = wall_termination();
        wrong_limits.observation.address_space_soft = 536_870_913;
        assert_eq!(
            validate_wall_termination_v1(wrong_limits),
            Err(SmokeRefusalV1::HostLimitUnavailable)
        );

        let mut wrong_group = wall_termination();
        wrong_group.observation.pgid = 5253;
        assert_eq!(
            validate_wall_termination_v1(wrong_group),
            Err(SmokeRefusalV1::HostLimitUnavailable)
        );

        let mut too_early = wall_termination();
        too_early.observation.wall_ms = WALL_DEADLINE_MS - 1;
        assert_eq!(
            validate_wall_termination_v1(too_early),
            Err(SmokeRefusalV1::HostWallTimeout),
            "a watchdog that fired before its own deadline is not a termination"
        );

        let mut too_late = wall_termination();
        too_late.observation.wall_ms = WALL_CHILD_SLEEP_MS;
        assert_eq!(
            validate_wall_termination_v1(too_late),
            Err(SmokeRefusalV1::HostWallTimeout),
            "an interval the child could have survived is not this negative"
        );
    }

    /// `N081-01`/`N081-08`: the evidence line is fixed, terminated-only and never accepted wording.
    #[test]
    fn n081_04_the_terminated_evidence_line_is_fixed_and_never_accepted_wording() {
        let line = wall_evidence_line(wall_termination());
        assert_eq!(
            line,
            "media_host_wall=terminated as_soft=536870912 as_hard=536870912 cpu_soft=10 \
             cpu_hard=10 pid=5252 pgid=5252 signal=9 reaped=1 wall_ms=2004\n"
        );
        assert!(line.ends_with('\n'), "exactly one terminated line");
        assert_eq!(line.matches('\n').count(), 1);
        assert!(
            !line.contains(PARENT_EVIDENCE_MARKER_V1),
            "a killed child must never be reported as an accepted run"
        );
        assert!(
            !line.contains(CHILD_OBSERVATION_MARKER_V1),
            "the child's own marker must not leak into parent evidence"
        );
        assert!(line.is_ascii());
    }

    /// The canonical denial line the fixed `as-child` writes when the kernel refuses the request.
    ///
    /// It is built from the real marker and literal constants, so a changed marker or a changed
    /// request size cannot pass unobserved.
    fn as_denial_line() -> String {
        format!(
            "{AS_CHILD_MARKER_V1} as_soft=536870912 as_hard=536870912 cpu_soft=10 \
             cpu_hard=10 pid=6262 pgid=6262 request_bytes=805306368 {AS_DENIED_TOKEN_V1}\n"
        )
    }

    fn as_refusal() -> AsRefusalV1 {
        AsRefusalV1 {
            observation: SmokeObservationV1 {
                address_space_soft: PROFILE_V1_AS_BYTES,
                address_space_hard: PROFILE_V1_AS_BYTES,
                cpu_soft: PROFILE_V1_CPU_SECONDS,
                cpu_hard: PROFILE_V1_CPU_SECONDS,
                pid: 6262,
                pgid: 6262,
                wall_ms: 12,
            },
            request_bytes: AS_REQUEST_BYTES_V1,
        }
    }

    fn as_attempt(stdout: Option<Vec<u8>>) -> AsAttemptV1 {
        AsAttemptV1 {
            stdout,
            exit_code: Some(0),
            reaped: true,
            wall_ms: 12,
        }
    }

    /// `N083-03`: the two reservation tokens are additive and closed, and every earlier token keeps
    /// its exact classification.
    #[test]
    fn n083_03_the_reservation_tokens_are_closed_and_additive() {
        assert_eq!(
            classify_internal_arguments_v1(&argv(&[
                INTERNAL_PROBE_FLAG_V1,
                INTERNAL_PROBE_AS_PARENT_TOKEN_V1,
            ])),
            Ok(InternalProbeModeV1::AsParent)
        );
        assert_eq!(
            classify_internal_arguments_v1(&argv(&[
                INTERNAL_PROBE_FLAG_V1,
                INTERNAL_PROBE_AS_CHILD_TOKEN_V1,
            ])),
            Ok(InternalProbeModeV1::AsChild)
        );

        // Every earlier mode still classifies exactly as it did before this order.
        for (token, mode) in [
            (INTERNAL_PROBE_PARENT_TOKEN_V1, InternalProbeModeV1::Parent),
            (INTERNAL_PROBE_CHILD_TOKEN_V1, InternalProbeModeV1::Child),
            (
                INTERNAL_PROBE_WALL_PARENT_TOKEN_V1,
                InternalProbeModeV1::WallParent,
            ),
            (
                INTERNAL_PROBE_WALL_CHILD_TOKEN_V1,
                InternalProbeModeV1::WallChild,
            ),
        ] {
            assert_eq!(
                classify_internal_arguments_v1(&argv(&[INTERNAL_PROBE_FLAG_V1, token])),
                Ok(mode),
                "{token} must keep its classification"
            );
        }

        let refusals: [&[&str]; 8] = [
            &[INTERNAL_PROBE_FLAG_V1, "as"],
            &[INTERNAL_PROBE_FLAG_V1, "AS-CHILD"],
            &[INTERNAL_PROBE_FLAG_V1, "as_child"],
            &[INTERNAL_PROBE_FLAG_V1, "as-parent", "as-parent"],
            &[INTERNAL_PROBE_FLAG_V1, "as-child", "extra"],
            &[INTERNAL_PROBE_FLAG_V1, "as-child "],
            &[INTERNAL_PROBE_FLAG_V1, ""],
            &[INTERNAL_PROBE_FLAG_V1],
        ];
        for tokens in refusals {
            assert_eq!(
                classify_internal_arguments_v1(&argv(tokens)),
                Err(SmokeRefusalV1::InternalModeInvalid),
                "{tokens:?} must be refused, never fallen through"
            );
        }

        // Neither reservation token is reachable behind a public subcommand.
        for token in [
            INTERNAL_PROBE_AS_PARENT_TOKEN_V1,
            INTERNAL_PROBE_AS_CHILD_TOKEN_V1,
        ] {
            assert_eq!(
                classify_internal_arguments_v1(&argv(&[
                    "bootstrap-preflight",
                    INTERNAL_PROBE_FLAG_V1,
                    token,
                ])),
                Ok(InternalProbeModeV1::NotRequested),
                "{token} must never be picked up behind a public subcommand"
            );
        }
    }

    /// `N083-03`: exactly one well-formed denial line is accepted, with every value at full width.
    #[test]
    fn n083_03_only_the_exact_denial_line_is_accepted() {
        let parsed = parse_as_child_refusal_v1(&as_denial_line())
            .expect("the exact denial line is accepted");
        assert_eq!(
            parsed,
            AsRefusalV1 {
                observation: SmokeObservationV1 {
                    wall_ms: 0,
                    ..as_refusal().observation
                },
                request_bytes: AS_REQUEST_BYTES_V1,
            },
            "only the parent fills in wall time"
        );
        assert_eq!(parsed.request_bytes, AS_REQUEST_BYTES_V1);
    }

    /// `N083-03`: no malformed, partial, duplicated, reordered or trailing denial is accepted.
    #[test]
    fn n083_03_malformed_denials_are_refused_without_partial_values() {
        let cases: [&str; 17] = [
            "",
            "\n",
            "SOLO-MEDIA-HOST-AS-V1\n",
            // no trailing newline
            "SOLO-MEDIA-HOST-AS-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 \
             pid=6262 pgid=6262 request_bytes=805306368 reserve=denied",
            // a second line
            "SOLO-MEDIA-HOST-AS-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 \
             pid=6262 pgid=6262 request_bytes=805306368 reserve=denied\nextra\n",
            // CRLF
            "SOLO-MEDIA-HOST-AS-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 \
             pid=6262 pgid=6262 request_bytes=805306368 reserve=denied\r\n",
            // tab separator
            "SOLO-MEDIA-HOST-AS-V1\tas_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 \
             pid=6262 pgid=6262 request_bytes=805306368 reserve=denied\n",
            // doubled space
            "SOLO-MEDIA-HOST-AS-V1  as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 \
             pid=6262 pgid=6262 request_bytes=805306368 reserve=denied\n",
            // the wall marker instead of the reservation marker
            "SOLO-MEDIA-HOST-WALL-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 \
             pid=6262 pgid=6262 request_bytes=805306368 reserve=denied\n",
            // reordered fields
            "SOLO-MEDIA-HOST-AS-V1 as_hard=536870912 as_soft=536870912 cpu_soft=10 cpu_hard=10 \
             pid=6262 pgid=6262 request_bytes=805306368 reserve=denied\n",
            // signed value
            "SOLO-MEDIA-HOST-AS-V1 as_soft=+536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 \
             pid=6262 pgid=6262 request_bytes=805306368 reserve=denied\n",
            // overflow
            "SOLO-MEDIA-HOST-AS-V1 as_soft=99999999999999999999 as_hard=536870912 cpu_soft=10 \
             cpu_hard=10 pid=6262 pgid=6262 request_bytes=805306368 reserve=denied\n",
            // a request that is not the fixed one
            "SOLO-MEDIA-HOST-AS-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 \
             pid=6262 pgid=6262 request_bytes=805306369 reserve=denied\n",
            // the ceiling itself instead of the over-ceiling request
            "SOLO-MEDIA-HOST-AS-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 \
             pid=6262 pgid=6262 request_bytes=536870912 reserve=denied\n",
            // an unexpected success is never accepted evidence
            "SOLO-MEDIA-HOST-AS-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 \
             pid=6262 pgid=6262 request_bytes=805306368 reserve=granted\n",
            // an extra trailing field
            "SOLO-MEDIA-HOST-AS-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 \
             pid=6262 pgid=6262 request_bytes=805306368 reserve=denied wall_ms=12\n",
            // a missing field
            "SOLO-MEDIA-HOST-AS-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 \
             pid=6262 pgid=6262 reserve=denied\n",
        ];
        for case in cases {
            assert_eq!(
                parse_as_child_refusal_v1(case),
                Err(SmokeRefusalV1::HostOutputInvalid),
                "{case:?} must be refused"
            );
        }
    }

    /// `N083-03`: every abnormal, un-reaped, non-UTF-8 or over-deadline supervision is refused.
    #[test]
    fn n083_03_every_abnormal_supervision_is_refused() {
        let accepted = as_attempt(Some(as_denial_line().into_bytes()));
        assert_eq!(validate_as_attempt_v1(accepted), Ok(as_refusal()));

        // An un-reaped leader can never be accepted.
        let mut unreaped = as_attempt(Some(as_denial_line().into_bytes()));
        unreaped.reaped = false;
        assert_eq!(
            validate_as_attempt_v1(unreaped),
            Err(SmokeRefusalV1::HostReapFailed)
        );

        // The three verification statuses and an unexpected grant all mean "limits not proved".
        for code in [71, 72, 73, 74] {
            let mut attempt = as_attempt(Some(as_denial_line().into_bytes()));
            attempt.exit_code = Some(code);
            assert_eq!(
                validate_as_attempt_v1(attempt),
                Err(SmokeRefusalV1::HostLimitUnavailable),
                "status {code} must mean the ceilings were not proved"
            );
            assert_eq!(
                classify_as_child_exit_v1(Some(code)),
                Err(SmokeRefusalV1::HostLimitUnavailable)
            );
        }

        // Any other status, and any signal death, is abnormal rather than a limit refusal.
        for code in [1, 2, 3, 70, 75, 137, 255] {
            let mut attempt = as_attempt(Some(as_denial_line().into_bytes()));
            attempt.exit_code = Some(code);
            assert_eq!(
                validate_as_attempt_v1(attempt),
                Err(SmokeRefusalV1::HostExitAbnormal),
                "status {code} is not this negative"
            );
        }
        let mut signalled = as_attempt(Some(as_denial_line().into_bytes()));
        signalled.exit_code = None;
        assert_eq!(
            validate_as_attempt_v1(signalled),
            Err(SmokeRefusalV1::HostExitAbnormal),
            "a signal death is never success"
        );
        assert_eq!(
            classify_as_child_exit_v1(None),
            Err(SmokeRefusalV1::HostExitAbnormal)
        );
        assert_eq!(
            classify_as_child_exit_v1(Some(0)),
            Ok(()),
            "only a normal zero status can carry the child's denial"
        );

        // Absent or non-UTF-8 bytes are unusable.
        assert_eq!(
            validate_as_attempt_v1(as_attempt(None)),
            Err(SmokeRefusalV1::HostOutputInvalid)
        );
        assert_eq!(
            validate_as_attempt_v1(as_attempt(Some(Vec::from([0xff_u8, 0xfe, b'\n'])))),
            Err(SmokeRefusalV1::HostOutputInvalid)
        );

        // The measured interval must be strictly below the fixed deadline.
        let mut at_deadline = as_attempt(Some(as_denial_line().into_bytes()));
        at_deadline.wall_ms = AS_DEADLINE_MS;
        assert_eq!(
            validate_as_attempt_v1(at_deadline),
            Err(SmokeRefusalV1::HostWallTimeout),
            "an interval at the deadline is not inside it"
        );
        let mut over_deadline = as_attempt(Some(as_denial_line().into_bytes()));
        over_deadline.wall_ms = AS_DEADLINE_MS + 1;
        assert_eq!(
            validate_as_attempt_v1(over_deadline),
            Err(SmokeRefusalV1::HostWallTimeout)
        );
    }

    /// `N083-03`: the enforcement line is fixed, bounded and never uses accepted-run wording.
    #[test]
    fn n083_03_the_evidence_line_is_fixed_and_bounded() {
        let line = as_evidence_line(as_refusal());
        assert_eq!(
            line,
            "media_host_as=limit_enforced as_soft=536870912 as_hard=536870912 cpu_soft=10 \
             cpu_hard=10 request_bytes=805306368 reserve=denied pid=6262 pgid=6262 wall_ms=12\n"
        );
        assert!(line.ends_with('\n'), "exactly one evidence line");
        assert_eq!(line.matches('\n').count(), 1);
        assert!(line.is_ascii());
        assert!(
            !line.contains(PARENT_EVIDENCE_MARKER_V1),
            "a refused reservation must never be reported as an accepted run"
        );
        assert!(
            !line.contains(WALL_EVIDENCE_MARKER_V1),
            "a reservation refusal is not a watchdog termination"
        );
        assert!(
            !line.contains(AS_CHILD_MARKER_V1),
            "the child's own marker must not leak into parent evidence"
        );

        // The child's single line fits the parent's existing stdout cap with room to spare.
        let denial = as_denial_line();
        assert_eq!(denial.matches('\n').count(), 1);
        assert!(
            denial.len() < MAX_CHILD_STDOUT_BYTES,
            "the one denial line must fit the bounded pipe cap: {} bytes",
            denial.len()
        );
    }

    /// `N083-03`: the one fixed request really is larger than the whole address-space ceiling, and
    /// the fixed supervision deadline is the required 15 000 ms.
    ///
    /// The values are read through a function so the assertion compares observed runtime values
    /// rather than restating compile-time literals.
    #[test]
    fn n083_03_the_fixed_request_exceeds_the_whole_address_space_ceiling() {
        let (ceiling, request, deadline) = fixed_as_profile_v1();
        assert!(
            request > ceiling,
            "the single reservation must exceed the whole ceiling: {request} vs {ceiling}"
        );
        assert_eq!(
            request - ceiling,
            268_435_456,
            "the request clears the whole 512 MiB ceiling by 256 MiB"
        );
        assert_eq!(deadline, 15_000, "the supervision deadline is 15 000 ms");
    }

    /// The fixed numbers this negative rests on, read at runtime by the pure case above.
    fn fixed_as_profile_v1() -> (u64, u64, u64) {
        (PROFILE_V1_AS_BYTES, AS_REQUEST_BYTES_V1, AS_DEADLINE_MS)
    }

    /// The canonical readiness line the fixed `cpu-child` writes before it burns CPU.
    fn cpu_ready_line() -> String {
        format!(
            "{CPU_CHILD_READY_MARKER_V1} as_soft=536870912 as_hard=536870912 cpu_soft=10 \
             cpu_hard=10 pid=7272 pgid=7272\n"
        )
    }

    fn cpu_observation() -> SmokeObservationV1 {
        SmokeObservationV1 {
            address_space_soft: PROFILE_V1_AS_BYTES,
            address_space_hard: PROFILE_V1_AS_BYTES,
            cpu_soft: PROFILE_V1_CPU_SECONDS,
            cpu_hard: PROFILE_V1_CPU_SECONDS,
            pid: 7272,
            pgid: 7272,
            wall_ms: 10_012,
        }
    }

    fn cpu_termination() -> CpuTerminationV1 {
        CpuTerminationV1 {
            observation: cpu_observation(),
            signal: CPU_TERMINATION_SIGNAL_HARD_V1,
            reaped: true,
            parent_killed: false,
        }
    }

    fn cpu_attempt(stdout: Option<Vec<u8>>) -> CpuAttemptV1 {
        CpuAttemptV1 {
            stdout,
            signal: Some(CPU_TERMINATION_SIGNAL_SOFT_V1),
            reaped: true,
            parent_killed: false,
            wall_ms: 10_012,
        }
    }

    /// `N085-03`: no malformed, duplicated, reordered, negative or oversized readiness is accepted.
    #[test]
    fn n085_03_malformed_cpu_readiness_is_refused() {
        let malformed: [&str; 13] = [
            "",
            "\n",
            "SOLO-MEDIA-HOST-CPU-V1\n",
            // no trailing newline
            "SOLO-MEDIA-HOST-CPU-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 \
             cpu_hard=10 pid=7272 pgid=7272",
            // a second line
            "SOLO-MEDIA-HOST-CPU-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 \
             cpu_hard=10 pid=7272 pgid=7272\nextra\n",
            // CRLF
            "SOLO-MEDIA-HOST-CPU-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 \
             cpu_hard=10 pid=7272 pgid=7272\r\n",
            // tab separator
            "SOLO-MEDIA-HOST-CPU-V1\tas_soft=536870912 as_hard=536870912 cpu_soft=10 \
             cpu_hard=10 pid=7272 pgid=7272\n",
            // doubled space
            "SOLO-MEDIA-HOST-CPU-V1  as_soft=536870912 as_hard=536870912 cpu_soft=10 \
             cpu_hard=10 pid=7272 pgid=7272\n",
            // another mode's marker
            "SOLO-MEDIA-HOST-WALL-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 \
             cpu_hard=10 pid=7272 pgid=7272\n",
            // reordered fields
            "SOLO-MEDIA-HOST-CPU-V1 as_hard=536870912 as_soft=536870912 cpu_soft=10 \
             cpu_hard=10 pid=7272 pgid=7272\n",
            // signed value
            "SOLO-MEDIA-HOST-CPU-V1 as_soft=+536870912 as_hard=536870912 cpu_soft=10 \
             cpu_hard=10 pid=7272 pgid=7272\n",
            // a missing field
            "SOLO-MEDIA-HOST-CPU-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 \
             cpu_hard=10 pid=7272\n",
            // an extra field
            "SOLO-MEDIA-HOST-CPU-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 \
             cpu_hard=10 pid=7272 pgid=7272 signal=9\n",
        ];
        for line in malformed {
            assert_eq!(
                parse_cpu_child_readiness_v1(line),
                Err(SmokeRefusalV1::HostOutputInvalid),
                "{line:?} must be refused as invalid output"
            );
        }

        // A well-formed line outside the fixed profile, or with an impossible identity, is refused.
        let off_profile: [(&str, SmokeRefusalV1); 4] = [
            (
                "SOLO-MEDIA-HOST-CPU-V1 as_soft=536870913 as_hard=536870912 cpu_soft=10 \
                 cpu_hard=10 pid=7272 pgid=7272\n",
                SmokeRefusalV1::HostLimitUnavailable,
            ),
            (
                "SOLO-MEDIA-HOST-CPU-V1 as_soft=536870912 as_hard=536870912 cpu_soft=11 \
                 cpu_hard=10 pid=7272 pgid=7272\n",
                SmokeRefusalV1::HostLimitUnavailable,
            ),
            (
                "SOLO-MEDIA-HOST-CPU-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 \
                 cpu_hard=10 pid=7272 pgid=7273\n",
                SmokeRefusalV1::HostLimitUnavailable,
            ),
            (
                "SOLO-MEDIA-HOST-CPU-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 \
                 cpu_hard=10 pid=0 pgid=0\n",
                SmokeRefusalV1::HostLimitUnavailable,
            ),
        ];
        for (line, refusal) in off_profile {
            assert_eq!(
                parse_cpu_child_readiness_v1(line),
                Err(refusal),
                "{line:?} must be refused rather than accepted as proof"
            );
        }
    }

    /// `N085-03`: only a reaped, pre-deadline kernel `SIGXCPU`/`SIGKILL` ever becomes evidence.
    #[test]
    fn n085_03_every_unexpected_cpu_termination_is_refused() {
        for signal in [
            CPU_TERMINATION_SIGNAL_SOFT_V1,
            CPU_TERMINATION_SIGNAL_HARD_V1,
        ] {
            assert_eq!(
                classify_cpu_termination_v1(Some(signal), true, false),
                Ok(signal),
                "signal {signal} is the kernel's own RLIMIT_CPU enforcement"
            );
        }

        // A normal exit is never a kill, and an absent status is never evidence either.
        for code in [None, Some(0), Some(71), Some(73), Some(75), Some(137)] {
            assert_eq!(
                classify_cpu_termination_v1(code, true, false),
                Err(SmokeRefusalV1::HostExitAbnormal),
                "{code:?} must not be read as kernel enforcement"
            );
        }

        // No other signal is this proof.
        for signal in [1, 2, 6, 11, 15, 31] {
            assert_eq!(
                classify_cpu_termination_v1(Some(signal), true, false),
                Err(SmokeRefusalV1::HostExitAbnormal),
                "signal {signal} is not RLIMIT_CPU enforcement"
            );
        }

        // This parent's own kill is a refusal even when the status it produced is `SIGKILL`.
        for signal in [
            CPU_TERMINATION_SIGNAL_SOFT_V1,
            CPU_TERMINATION_SIGNAL_HARD_V1,
        ] {
            assert_eq!(
                classify_cpu_termination_v1(Some(signal), true, true),
                Err(SmokeRefusalV1::HostWallTimeout),
                "a watchdog kill must never be reported as kernel enforcement"
            );
        }

        // Without a reap nothing may be claimed, whatever the signal looked like.
        assert_eq!(
            classify_cpu_termination_v1(Some(CPU_TERMINATION_SIGNAL_HARD_V1), false, false),
            Err(SmokeRefusalV1::HostReapFailed)
        );

        // A complete, well-formed attempt is the only accepted shape.
        let accepted = validate_cpu_attempt_v1(cpu_attempt(Some(cpu_ready_line().into_bytes())))
            .expect("a complete, well-formed attempt is the only accepted shape");
        assert_eq!(accepted.signal, CPU_TERMINATION_SIGNAL_SOFT_V1);
        assert!(accepted.reaped);
        assert!(!accepted.parent_killed);
        assert_eq!(accepted.observation.wall_ms, 10_012);
    }

    /// `N085-03`: the two CPU tokens are additive and closed, and every legacy token is unchanged.
    #[test]
    fn n085_03_the_cpu_tokens_are_closed_and_additive() {
        assert_eq!(
            classify_internal_arguments_v1(&argv(&[
                INTERNAL_PROBE_FLAG_V1,
                INTERNAL_PROBE_CPU_PARENT_TOKEN_V1,
            ])),
            Ok(InternalProbeModeV1::CpuParent)
        );
        assert_eq!(
            classify_internal_arguments_v1(&argv(&[
                INTERNAL_PROBE_FLAG_V1,
                INTERNAL_PROBE_CPU_CHILD_TOKEN_V1,
            ])),
            Ok(InternalProbeModeV1::CpuChild)
        );

        // Every earlier pair still classifies exactly as it did before this order.
        let legacy = [
            (INTERNAL_PROBE_PARENT_TOKEN_V1, InternalProbeModeV1::Parent),
            (INTERNAL_PROBE_CHILD_TOKEN_V1, InternalProbeModeV1::Child),
            (
                INTERNAL_PROBE_WALL_PARENT_TOKEN_V1,
                InternalProbeModeV1::WallParent,
            ),
            (
                INTERNAL_PROBE_WALL_CHILD_TOKEN_V1,
                InternalProbeModeV1::WallChild,
            ),
            (
                INTERNAL_PROBE_AS_PARENT_TOKEN_V1,
                InternalProbeModeV1::AsParent,
            ),
            (
                INTERNAL_PROBE_AS_CHILD_TOKEN_V1,
                InternalProbeModeV1::AsChild,
            ),
        ];
        for (token, mode) in legacy {
            assert_eq!(
                classify_internal_arguments_v1(&argv(&[INTERNAL_PROBE_FLAG_V1, token])),
                Ok(mode),
                "{token} must keep classifying as before"
            );
        }

        let refusals: [&[&str]; 8] = [
            &[INTERNAL_PROBE_FLAG_V1, "cpu"],
            &[INTERNAL_PROBE_FLAG_V1, "CPU-PARENT"],
            &[INTERNAL_PROBE_FLAG_V1, "cpu_child"],
            &[INTERNAL_PROBE_FLAG_V1, "cpu-parent", "cpu-parent"],
            &[INTERNAL_PROBE_FLAG_V1, "cpu-child", "extra"],
            &[INTERNAL_PROBE_FLAG_V1, "cpu-parent", "cpu-child"],
            &[INTERNAL_PROBE_FLAG_V1, " cpu-parent"],
            &[INTERNAL_PROBE_FLAG_V1],
        ];
        for tokens in refusals {
            assert_eq!(
                classify_internal_arguments_v1(&argv(tokens)),
                Err(SmokeRefusalV1::InternalModeInvalid),
                "{tokens:?} must be refused, never fallen through"
            );
        }

        // Neither CPU token is reachable behind a public subcommand.
        for token in [
            INTERNAL_PROBE_CPU_PARENT_TOKEN_V1,
            INTERNAL_PROBE_CPU_CHILD_TOKEN_V1,
        ] {
            assert_eq!(
                classify_internal_arguments_v1(&argv(&["bootstrap-preflight", token])),
                Ok(InternalProbeModeV1::NotRequested),
                "{token} must never be honoured behind a public subcommand"
            );
        }
    }

    /// `N085-03`: exactly one well-formed readiness line is accepted, with nothing else.
    #[test]
    fn n085_03_only_the_exact_cpu_readiness_line_is_accepted() {
        let parsed = parse_cpu_child_readiness_v1(&cpu_ready_line())
            .expect("the canonical readiness line is accepted");
        assert_eq!(
            parsed,
            SmokeObservationV1 {
                wall_ms: 0,
                ..cpu_observation()
            },
            "only the parent fills in wall time"
        );
        assert!(
            cpu_ready_line().len() < MAX_CHILD_STDOUT_BYTES,
            "the one readiness line must fit the bounded pipe cap"
        );
    }

    /// `N085-03`: absent, non-UTF-8, over-cap, parent-killed, over-deadline and off-profile attempts
    /// are all refused, so no refused run can ever produce a success line.
    #[test]
    fn n085_03_refused_cpu_attempts_never_become_evidence() {
        assert_eq!(
            validate_cpu_attempt_v1(cpu_attempt(None)),
            Err(SmokeRefusalV1::HostOutputInvalid)
        );
        assert_eq!(
            validate_cpu_attempt_v1(cpu_attempt(Some(Vec::from([0xff_u8, 0xfe, b'\n'])))),
            Err(SmokeRefusalV1::HostOutputInvalid)
        );
        assert_eq!(
            validate_cpu_attempt_v1(cpu_attempt(Some(vec![b'x'; MAX_CHILD_STDOUT_BYTES + 1]))),
            Err(SmokeRefusalV1::HostOutputInvalid)
        );

        // This parent's own kill refuses the whole attempt, even at an accepted signal number.
        let mut killed = cpu_attempt(Some(cpu_ready_line().into_bytes()));
        killed.signal = Some(CPU_TERMINATION_SIGNAL_HARD_V1);
        killed.parent_killed = true;
        assert_eq!(
            validate_cpu_attempt_v1(killed),
            Err(SmokeRefusalV1::HostWallTimeout)
        );

        // An exit without any signal is never enforcement, whatever the readiness said.
        let mut normal_exit = cpu_attempt(Some(cpu_ready_line().into_bytes()));
        normal_exit.signal = None;
        assert_eq!(
            validate_cpu_attempt_v1(normal_exit),
            Err(SmokeRefusalV1::HostExitAbnormal)
        );

        // A reap that never happened refuses too.
        let mut unreaped = cpu_attempt(Some(cpu_ready_line().into_bytes()));
        unreaped.reaped = false;
        assert_eq!(
            validate_cpu_attempt_v1(unreaped),
            Err(SmokeRefusalV1::HostReapFailed)
        );

        // The measured interval must sit strictly inside the 60 000 ms deadline.
        for wall_ms in [0, CPU_DEADLINE_MS, CPU_DEADLINE_MS + 1] {
            let mut bounded = cpu_attempt(Some(cpu_ready_line().into_bytes()));
            bounded.wall_ms = wall_ms;
            assert_eq!(
                validate_cpu_attempt_v1(bounded),
                Err(SmokeRefusalV1::HostWallTimeout),
                "wall_ms {wall_ms} is not inside the deadline"
            );
        }

        // Evidence is refused when any value drifts off the fixed profile.
        let mut off_profile_termination = cpu_termination();
        off_profile_termination.observation.cpu_hard = 11;
        assert_eq!(
            validate_cpu_termination_v1(off_profile_termination),
            Err(SmokeRefusalV1::HostLimitUnavailable)
        );
        let mut no_identity = cpu_termination();
        no_identity.observation.pid = 0;
        no_identity.observation.pgid = 0;
        assert_eq!(
            validate_cpu_termination_v1(no_identity),
            Err(SmokeRefusalV1::HostLimitUnavailable)
        );
    }

    /// `N085-03`: the termination line is fixed, bounded and never borrows another result's wording.
    #[test]
    fn n085_03_the_cpu_evidence_line_is_fixed_and_bounded() {
        let line = cpu_evidence_line(cpu_termination());
        assert_eq!(
            line,
            "media_host_cpu=kernel_terminated as_soft=536870912 as_hard=536870912 cpu_soft=10 \
             cpu_hard=10 pid=7272 pgid=7272 signal=9 reaped=1 parent_killed=0 wall_ms=10012\n"
        );
        assert!(line.is_ascii());
        assert!(line.ends_with('\n'), "exactly one evidence line");
        assert_eq!(line.matches('\n').count(), 1);
        for foreign in [
            PARENT_EVIDENCE_MARKER_V1,
            WALL_EVIDENCE_MARKER_V1,
            AS_EVIDENCE_MARKER_V1,
            CPU_CHILD_READY_MARKER_V1,
            AS_CHILD_MARKER_V1,
        ] {
            assert!(
                !line.contains(foreign),
                "the termination line must not borrow {foreign}"
            );
        }
        assert!(line.len() < MAX_CHILD_STDOUT_BYTES);
    }

    /// `N085-03`: the fixed CPU profile really is the verified 10-second kernel limit, and the parent
    /// deadline is the separate 60 000 ms budget rather than any earlier watchdog.
    #[test]
    fn n085_03_the_fixed_cpu_profile_is_the_ten_second_kernel_limit() {
        let (cpu_seconds, deadline, soft, hard, wall_deadline) = fixed_cpu_profile_v1();
        assert_eq!(cpu_seconds, 10, "RLIMIT_CPU soft and hard are both 10 s");
        assert_eq!(deadline, 60_000, "the CPU parent deadline is 60 000 ms");
        assert_eq!(soft, 24, "SIGXCPU is the soft-limit signal");
        assert_eq!(hard, 9, "SIGKILL is the hard-limit signal");
        assert_ne!(
            soft, hard,
            "the two kernel signals must stay distinguishable"
        );
        assert!(
            deadline > wall_deadline && deadline > SMOKE_WALL_DEADLINE_MS,
            "the CPU budget must exceed both earlier watchdogs"
        );
    }

    /// The fixed numbers this proof rests on, read at runtime by the pure case above.
    fn fixed_cpu_profile_v1() -> (u64, u64, i32, i32, u64) {
        (
            PROFILE_V1_CPU_SECONDS,
            CPU_DEADLINE_MS,
            CPU_TERMINATION_SIGNAL_SOFT_V1,
            CPU_TERMINATION_SIGNAL_HARD_V1,
            WALL_DEADLINE_MS,
        )
    }
}
