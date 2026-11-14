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

// The one reserved native seam, under whichever positively gated name this target declared.
#[cfg(target_os = "linux")]
use crate::media_worker_host_native as native_worker;
#[cfg(windows)]
use crate::media_worker_host_native as native_worker;

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

/// The one fixed parent token of the live-child session attestation (`SOLO-WO-087 R01`).
///
/// Honoured only in second position behind [`INTERNAL_PROBE_FLAG_V1`], exactly like every earlier
/// closed mode, so it can never be reached through a public subcommand.
pub const INTERNAL_PROBE_SESSION_PARENT_TOKEN_V1: &str = "session-parent";

/// The one fixed, trusted child token of the live-child session attestation.
pub const INTERNAL_PROBE_SESSION_CHILD_TOKEN_V1: &str = "session-child";

/// The child's single readiness marker: unambiguous and never reused by an earlier proof.
pub const SESSION_CHILD_READY_MARKER_V1: &str = "SOLO-MEDIA-HOST-SESSION-V1";

/// The parent's single evidence marker. It is distinct from every earlier result, so a live-child
/// identity attestation can never be read as an accepted probe, a watchdog kill, a reservation
/// refusal or a kernel CPU termination.
pub const SESSION_EVIDENCE_MARKER_V1: &str = "media_host_session=live_attested";

/// The fixed child's post-readiness sleep: **4 000 milliseconds** of plain wall time.
///
/// It burns no CPU, allocates no large memory, opens nothing, forks nothing and starts no thread.
/// Its only purpose is to keep a descendant-free child alive long enough for the parent's own
/// read-only kernel observation of a still-live handle. It is not a timeout or CPU-limit negative.
///
/// Linux-only, because only the Linux pre-exec boundary creates the session child that sleeps; a
/// non-Linux build has no such child and therefore no such window.
#[cfg(target_os = "linux")]
pub const SESSION_CHILD_SLEEP_MS: u64 = 4_000;

/// The fixed parent wall deadline of the session attestation: **15 000 milliseconds**.
///
/// Measured from before `spawn` to the completed leader reap, exactly like the positive and
/// address-space deadlines, and well inside the integration case's own 20 000 ms budget.
pub const SESSION_DEADLINE_MS: u64 = 15_000;

/// The internal token of the one fixed **group supervising parent** of the `R03.C3` group proof.
///
/// `SOLO-WO-093 R01` adds it, and only it can create the group leader. The token is honoured only
/// in second position behind the reserved flag, so it can never be reached from the public CLI.
pub const INTERNAL_PROBE_GROUP_PARENT_TOKEN_V1: &str = "group-parent";

/// The internal token of the one fixed **group leader**.
///
/// The parent re-executes this same executable into this mode, which becomes its own session and
/// process-group leader under the unchanged pre-exec installer and starts exactly one descendant.
pub const INTERNAL_PROBE_GROUP_LEADER_TOKEN_V1: &str = "group-leader";

/// The internal token of the one fixed **group descendant**.
///
/// It is started only by the group leader, without a second `setsid`, so it inherits that leader's
/// group, session and both ceilings, and it has one fixed finite wall lifetime.
pub const INTERNAL_PROBE_GROUP_DESCENDANT_TOKEN_V1: &str = "group-descendant";

/// The internal token of the fixed **invalid-readiness parent** of the group negative.
///
/// This parent uses the same pre-exec limits and the same deadline, but its leader creates **no**
/// descendant and emits one deliberately malformed readiness record, so the parent must refuse
/// before any group identity authority exists and without issuing any group signal.
pub const INTERNAL_PROBE_GROUP_INVALID_PARENT_TOKEN_V1: &str = "group-invalid-parent";

/// The internal token of the fixed **invalid-readiness leader**.
///
/// It proves the same fixed ceilings and session identity, creates no descendant, then writes one
/// closed-but-wrong record: twelve tokens where the schema has eleven.
pub const INTERNAL_PROBE_GROUP_INVALID_LEADER_TOKEN_V1: &str = "group-invalid-leader";

/// The exact marker that begins the group leader's single readiness record.
///
/// The record carries the leader's own `pid`/`pgid`/`sid`, the descendant's `pid`/`pgid`/`sid` and
/// the four ceiling halves. Every value is re-checked by the parent against its own kernel
/// observations, so no field of this text can authorize anything by itself.
#[cfg(any(target_os = "linux", test))]
pub const GROUP_LEADER_READY_MARKER_V1: &str = "SOLO-MEDIA-HOST-GROUP-V1";

/// The exact marker that begins the fixed descendant's own readiness record.
///
/// Only the leader reads it, and only to prove that the descendant it spawned is really the process
/// that answered, with the leader's group and session.
#[cfg(any(target_os = "linux", test))]
pub const GROUP_DESCENDANT_READY_MARKER_V1: &str = "SOLO-MEDIA-HOST-GROUP-DESC-V1";

/// The exact marker that begins the one accepted group evidence line.
///
/// It is distinct from every earlier marker, so no existing regression can be re-read as a group
/// result and no group result can be mistaken for a session, CPU, address-space or wall proof.
pub const GROUP_EVIDENCE_MARKER_V1: &str = "media_host_group=terminated";

/// The one signal the group proof may send: `SIGKILL`, written as its fixed Linux number.
pub const GROUP_TERMINATION_SIGNAL_V1: i32 = 9;

/// The single group parent deadline: **15 000 milliseconds**, started **before** leader spawn.
///
/// It bounds the readiness handoff, the identity queries, the one group signal, the leader reap and
/// the descendant non-running observation together. No later step opens a second budget.
pub const GROUP_DEADLINE_MS: u64 = 15_000;

/// The leader's bounded wait for the descendant's own readiness record, in milliseconds.
///
/// It is a sub-budget of the single deadline above, never an extra one.
#[cfg(target_os = "linux")]
pub const GROUP_DESCENDANT_READY_DEADLINE_MS: u64 = 5_000;

/// The fixed finite wall lifetime of the one descendant, in milliseconds.
///
/// This is the safety net for a refusal on the positive path: a descendant that is never signalled
/// still ends by itself inside this window, so no long-running test process can be left behind.
#[cfg(target_os = "linux")]
pub const GROUP_DESCENDANT_LIFETIME_MS: u64 = 8_000;

/// How long the leader holds after its readiness record, in milliseconds.
///
/// The parent is expected to signal the group well inside this window. A leader that survives it
/// refuses, so a parent that never signalled can never be reported as a group termination.
#[cfg(target_os = "linux")]
pub const GROUP_LEADER_HOLD_MS: u64 = 10_000;

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
/// reported as kernel enforcement. It is Linux-only because only the Linux child can produce it.
#[cfg(target_os = "linux")]
const CHILD_EXIT_CPU_LOOP_RETURNED: u8 = 75;

/// Child exit status: the session child's own kernel identity readback **failed**.
///
/// Only the Linux child can produce it. It means `getpgid`/`getsid` did not report a usable
/// identity, so nothing was proved and no readiness line may be written.
#[cfg(target_os = "linux")]
const CHILD_EXIT_SESSION_READBACK_FAILED: u8 = 76;

/// Child exit status: the session child is **not** its own process-group and session leader.
///
/// This is the expected result of a direct, unsupervised `session-child` invocation: without the
/// pre-exec installer no `setsid` ran, so the kernel reports the inherited group and session. The
/// child refuses promptly, before its sleep and before any output.
#[cfg(target_os = "linux")]
const CHILD_EXIT_SESSION_MISMATCH: u8 = 77;

/// Child exit status: the group leader could not start its one fixed descendant.
///
/// Only the Linux leader can produce it. Nothing is printed, so a leader that never acquired a
/// descendant can never emit a readiness record.
#[cfg(target_os = "linux")]
const CHILD_EXIT_GROUP_SPAWN_FAILED: u8 = 78;

/// Child exit status: the group descendant could not be positively identified in the leader's group.
///
/// This is also the expected result of a direct, unsupervised `group-descendant` invocation: without
/// the leader's `setsid` the descendant is not inside the leader's group and session, so it refuses
/// before its readiness record and before its finite sleep.
#[cfg(target_os = "linux")]
const CHILD_EXIT_GROUP_DESCENDANT_MISMATCH: u8 = 79;

/// Child exit status: the group leader outlived its own fixed hold, so no group signal ever came.
///
/// A surviving leader is not a group termination. It refuses, and its descendant still ends by
/// itself inside its own finite lifetime.
#[cfg(target_os = "linux")]
const CHILD_EXIT_GROUP_LEADER_SURVIVED: u8 = 80;

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
    /// The fixed descendant's post-signal non-running state could not be established from the
    /// kernel for the **same** observed identity, so no group termination may be claimed.
    ///
    /// It is deliberately distinct from [`Self::HostReapFailed`]: the leader reap and the descendant
    /// observation are separate facts, and collapsing them would let one stand for the other.
    HostDescendantUnverified,
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
            Self::HostDescendantUnverified => "HOST_DESCENDANT_UNVERIFIED",
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
    /// Run the one fixed supervising parent of the live-child session attestation.
    SessionParent,
    /// Run the one fixed, trusted child that attests its own kernel session identity and sleeps.
    SessionChild,
    /// Run the one fixed supervising parent of the bounded group and descendant lifecycle proof.
    GroupParent,
    /// Run the one fixed group leader, which starts exactly one fixed descendant.
    GroupLeader,
    /// Run the one fixed group descendant, which inherits the leader's group and session.
    GroupDescendant,
    /// Run the one fixed supervising parent of the invalid-readiness group negative.
    GroupInvalidParent,
    /// Run the one fixed leader that emits a deliberately malformed readiness record.
    GroupInvalidLeader,
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
    } else if mode == OsStr::new(INTERNAL_PROBE_SESSION_PARENT_TOKEN_V1) {
        Ok(InternalProbeModeV1::SessionParent)
    } else if mode == OsStr::new(INTERNAL_PROBE_SESSION_CHILD_TOKEN_V1) {
        Ok(InternalProbeModeV1::SessionChild)
    } else if mode == OsStr::new(INTERNAL_PROBE_GROUP_PARENT_TOKEN_V1) {
        Ok(InternalProbeModeV1::GroupParent)
    } else if mode == OsStr::new(INTERNAL_PROBE_GROUP_LEADER_TOKEN_V1) {
        Ok(InternalProbeModeV1::GroupLeader)
    } else if mode == OsStr::new(INTERNAL_PROBE_GROUP_DESCENDANT_TOKEN_V1) {
        Ok(InternalProbeModeV1::GroupDescendant)
    } else if mode == OsStr::new(INTERNAL_PROBE_GROUP_INVALID_PARENT_TOKEN_V1) {
        Ok(InternalProbeModeV1::GroupInvalidParent)
    } else if mode == OsStr::new(INTERNAL_PROBE_GROUP_INVALID_LEADER_TOKEN_V1) {
        Ok(InternalProbeModeV1::GroupInvalidLeader)
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

/// The fixed child's own identity attestation, parsed strictly from its one readiness line.
///
/// It carries the four ceiling halves the child read back plus its own kernel `pid`, `pgid` and
/// `sid`. Nothing here is trusted on its own: the parent re-checks every value against its own
/// kernel observation and the compile-time profile before any evidence can be emitted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SessionChildObservationV1 {
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
    /// The child's own process-group id, which `setsid` must have made equal to `pid`.
    pub pgid: u64,
    /// The child's own session id, which `setsid` must have made equal to `pid`.
    pub sid: u64,
}

/// Parses the session child's single readiness line, strictly.
///
/// Accepted form, exactly one line and nothing else:
///
/// ```text
/// SOLO-MEDIA-HOST-SESSION-V1 as_soft=<u64> as_hard=<u64> cpu_soft=<u64> cpu_hard=<u64> pid=<u64> pgid=<u64> sid=<u64>
/// ```
///
/// The marker, the seven field names and their order are fixed; every value is plain ASCII decimal
/// digits with no sign, no prefix, no separator and no whitespace; a single trailing newline is
/// required and a second line, a carriage return, a tab, a doubled space or any unknown,
/// duplicated, missing or reordered field is refused. Identity values are judged by
/// [`validate_session_child_v1`], so the same closed reader serves both hosts' neutral tests.
///
/// # Errors
///
/// [`SmokeRefusalV1::HostOutputInvalid`] for any shape defect.
pub fn parse_session_child_readiness_v1(
    text: &str,
) -> Result<SessionChildObservationV1, SmokeRefusalV1> {
    let body = text
        .strip_suffix('\n')
        .ok_or(SmokeRefusalV1::HostOutputInvalid)?;
    if body.contains('\n') || body.contains('\r') || body.contains('\t') {
        return Err(SmokeRefusalV1::HostOutputInvalid);
    }

    let tokens: Vec<&str> = body.split(' ').collect();
    if tokens.len() != 8 || tokens[0] != SESSION_CHILD_READY_MARKER_V1 {
        return Err(SmokeRefusalV1::HostOutputInvalid);
    }

    Ok(SessionChildObservationV1 {
        address_space_soft: field_value(tokens[1], "as_soft=")?,
        address_space_hard: field_value(tokens[2], "as_hard=")?,
        cpu_soft: field_value(tokens[3], "cpu_soft=")?,
        cpu_hard: field_value(tokens[4], "cpu_hard=")?,
        pid: field_value(tokens[5], "pid=")?,
        pgid: field_value(tokens[6], "pgid=")?,
        sid: field_value(tokens[7], "sid=")?,
    })
}

/// Validates the parsed child line against the fixed profile and its own three identities.
///
/// A child that reports anything but the compile-time ceilings and a strictly positive
/// `pid == pgid == sid` has proved nothing, so it is refused with the same closed
/// [`SmokeRefusalV1::HostLimitUnavailable`] the earlier limit proofs use for "the required fixed
/// limits, or the required group identity, were not proved".
///
/// # Errors
///
/// [`SmokeRefusalV1::HostLimitUnavailable`] for any off-profile ceiling or non-identical identity.
pub fn validate_session_child_v1(
    observation: SessionChildObservationV1,
) -> Result<SessionChildObservationV1, SmokeRefusalV1> {
    let limits_match = observation.address_space_soft == PROFILE_V1_AS_BYTES
        && observation.address_space_hard == PROFILE_V1_AS_BYTES
        && observation.cpu_soft == PROFILE_V1_CPU_SECONDS
        && observation.cpu_hard == PROFILE_V1_CPU_SECONDS;
    let identity_match = observation.pid > 0
        && observation.pgid == observation.pid
        && observation.sid == observation.pid;
    if !limits_match || !identity_match {
        return Err(SmokeRefusalV1::HostLimitUnavailable);
    }
    Ok(observation)
}

/// Validates one **received** readiness line as the sole gate for the parent's kernel query.
///
/// It applies exactly the strict pair the complete stream is judged by —
/// [`parse_session_child_readiness_v1`] and then [`validate_session_child_v1`] — plus the one fact
/// only the parent can know: the line's reported child pid must be the exact positive pid of this
/// parent's own child handle. A parent that has not obtained `Ok` from this function has no
/// authority to ask the kernel anything about that child.
///
/// # Errors
///
/// [`SmokeRefusalV1::HostOutputInvalid`] for non-UTF-8 or any shape defect, and
/// [`SmokeRefusalV1::HostLimitUnavailable`] for an off-profile ceiling or a reported pid that is
/// not the handle's own.
pub fn validate_session_readiness_v1(
    line: &[u8],
    expected_pid: u64,
) -> Result<SessionChildObservationV1, SmokeRefusalV1> {
    let text = std::str::from_utf8(line).map_err(|_| SmokeRefusalV1::HostOutputInvalid)?;
    let observation = validate_session_child_v1(parse_session_child_readiness_v1(text)?)?;
    if expected_pid == 0 || observation.pid != expected_pid {
        return Err(SmokeRefusalV1::HostLimitUnavailable);
    }
    Ok(observation)
}

/// The parent's raw, unvalidated material for one live-child session attestation.
///
/// It carries what the parent itself observed — its live-child kernel query, the two liveness
/// readbacks around that query, the exit status, the reap and the wall time — plus the child's
/// capped output. Nothing in it is evidence until [`validate_session_attempt_v1`] accepts it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SessionAttemptV1 {
    /// The child's captured stdout bytes, or `None` when the reader failed, overflowed or panicked.
    pub stdout: Option<Vec<u8>>,
    /// The exact positive pid this parent's own `Child` handle reported at spawn.
    pub child_pid: u64,
    /// The kernel's `getpgid` answer for the still-live, directly owned child.
    ///
    /// `0` means no observation was made, because a real group id is always positive.
    pub observed_pgid: u64,
    /// The kernel's `getsid` answer for the same live child. `0` likewise means "not observed".
    pub observed_sid: u64,
    /// Whether `try_wait()` reported the child still running immediately **before** the query.
    pub live_before: bool,
    /// Whether `try_wait()` reported the child still running immediately **after** the query.
    pub live_after: bool,
    /// The leader's normal exit code, or `None` when it was signalled or unobservable.
    pub exit_code: Option<i32>,
    /// Whether the leader was actually reaped by this parent.
    pub reaped: bool,
    /// Whether this parent sent any signal to the leader. `true` can never become evidence.
    pub parent_killed: bool,
    /// The wall time the parent measured from before `spawn` to the completed leader reap.
    pub wall_ms: u64,
    /// Whether this parent received and strictly validated a complete readiness line from its own
    /// child **before** it performed the kernel identity query.
    ///
    /// This is the only value that can authorize the ordering claim, and it is set on exactly one
    /// path in [`run_internal_session_attempt_v1`]: after the bounded handoff delivered a line and
    /// after [`validate_session_readiness_v1`] accepted it, and only then before the query.
    pub readiness_validated: bool,
    /// The exact readiness line the parent gated on, or `None` when no complete line ever arrived.
    ///
    /// The parent requires it to equal the whole captured stream byte for byte, so a partial early
    /// prefix, a duplicate line or any trailing output cannot become accepted evidence.
    pub readiness_line: Option<Vec<u8>>,
}

/// The complete, validated live-child session attestation.
///
/// It only ever exists after the child exited normally with code zero, the leader was reaped
/// without any kill by this parent, the child was proved alive both immediately before and
/// immediately after the parent's own kernel query, the one readiness line parsed strictly, and
/// every identity agreed with the parent's independent observation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SessionAttestationV1 {
    /// The child's four ceiling halves and its own three identities, re-checked by the parent.
    pub observation: SessionChildObservationV1,
    /// The process-group id the parent's own kernel query reported for the live child.
    pub observed_pgid: u64,
    /// The session id the same parent query reported.
    pub observed_sid: u64,
    /// Whether the leader was reaped. Always `true` in any value that exists.
    pub reaped: bool,
    /// The parent-measured wall time in milliseconds.
    pub wall_ms: u64,
}

/// Validates one raw session attempt into accepted evidence, or refuses.
///
/// Every refusal path returns a typed refusal and therefore no evidence line at all: a kill by this
/// parent, a missing reap, a signalled or non-zero exit, a child that was not proved alive at the
/// observation, a measurement at or beyond the fixed deadline, unreadable or over-cap readiness
/// bytes, an off-profile ceiling, a non-identical child identity, or any disagreement with the
/// parent's own kernel observation.
///
/// # Errors
///
/// [`SmokeRefusalV1::HostExitAbnormal`] for a parent kill, an abnormal exit or a child not proved
/// live; [`SmokeRefusalV1::HostReapFailed`] for a missing reap;
/// [`SmokeRefusalV1::HostWallTimeout`] for an over-deadline measurement;
/// [`SmokeRefusalV1::HostOutputInvalid`] for unreadable, over-cap, non-UTF-8 or off-schema bytes;
/// otherwise [`SmokeRefusalV1::HostLimitUnavailable`].
pub fn validate_session_attempt_v1(
    attempt: SessionAttemptV1,
) -> Result<SessionAttestationV1, SmokeRefusalV1> {
    if attempt.parent_killed {
        return Err(SmokeRefusalV1::HostExitAbnormal);
    }
    if !attempt.reaped {
        return Err(SmokeRefusalV1::HostReapFailed);
    }
    if attempt.exit_code != Some(0) {
        return Err(SmokeRefusalV1::HostExitAbnormal);
    }
    if !attempt.live_before || !attempt.live_after {
        return Err(SmokeRefusalV1::HostExitAbnormal);
    }
    if attempt.wall_ms >= SESSION_DEADLINE_MS {
        return Err(SmokeRefusalV1::HostWallTimeout);
    }

    // The readiness gate is mandatory: an attempt whose parent never received and validated a
    // complete readiness line cannot describe an ordered observation, so it is refused before any
    // of its text is consulted.
    if !attempt.readiness_validated {
        return Err(SmokeRefusalV1::HostOutputInvalid);
    }
    let early = attempt
        .readiness_line
        .ok_or(SmokeRefusalV1::HostOutputInvalid)?;
    let bytes = attempt.stdout.ok_or(SmokeRefusalV1::HostOutputInvalid)?;
    // The early prefix the parent gated on and the complete stream captured through EOF must be the
    // same bytes, so trailing, duplicated or late output can never be folded into a success.
    if early != bytes {
        return Err(SmokeRefusalV1::HostOutputInvalid);
    }
    let observation = validate_session_readiness_v1(&bytes, attempt.child_pid)?;

    // The child's reported pid must be the exact pid this parent's own handle gave, and the
    // parent's independent kernel observation must agree with it on both identities.
    let ids_agree = attempt.child_pid > 0
        && observation.pid == attempt.child_pid
        && attempt.observed_pgid > 0
        && attempt.observed_sid > 0
        && attempt.observed_pgid == observation.pgid
        && attempt.observed_sid == observation.sid;
    if !ids_agree {
        return Err(SmokeRefusalV1::HostLimitUnavailable);
    }

    Ok(SessionAttestationV1 {
        observation,
        observed_pgid: attempt.observed_pgid,
        observed_sid: attempt.observed_sid,
        reaped: attempt.reaped,
        wall_ms: attempt.wall_ms,
    })
}

/// What one bounded early-readiness refusal cleanup established about its own leader.
///
/// Only a checked `Child::wait()` result can produce [`Self::Reaped`]. A `Child::kill()` result is
/// deliberately absent from this type: an accepted signal says nothing about whether the leader
/// exited, and a rejected one is legitimate when the leader had already exited on its own.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg(any(target_os = "linux", test))]
pub enum SessionReapV1 {
    /// `Child::wait()` returned `Ok`: the directly owned leader is confirmed exited and reaped.
    Reaped,
    /// `Child::wait()` returned `Err`: no leader exit and no reap were established.
    Unreaped,
}

#[cfg(any(target_os = "linux", test))]
impl SessionReapV1 {
    /// Maps the actual checked `Child::wait()` outcome; only `Ok` proves a completed reap.
    ///
    /// Every other observation the cleanup made — including whether the kill was accepted — is
    /// excluded here, so no signal result can be mistaken for a reap.
    fn from_wait_outcome_v1(wait: std::io::Result<std::process::ExitStatus>) -> Self {
        match wait {
            Ok(_) => Self::Reaped,
            Err(_) => Self::Unreaped,
        }
    }
}

/// Classifies the **one real** `Child::wait()` outcome of a supervised group leader.
///
/// This is the production boundary the group supervisor uses immediately after `child.wait()`, so
/// the failed-wait decision is ordinary checked code rather than a branch buried in the supervisor.
/// Only `Ok` yields a signal status; every `Err` becomes [`SmokeRefusalV1::HostReapFailed`], which is
/// what prevents a failed reap from ever reaching a success attempt or a reader join.
///
/// Linux-only because reading a signal from a wait status needs the Unix extension trait; no
/// Windows counterpart exists and none is authorized.
///
/// # Errors
///
/// [`SmokeRefusalV1::HostReapFailed`] for any failed wait.
#[cfg(target_os = "linux")]
fn group_wait_signal_v1(
    waited: std::io::Result<std::process::ExitStatus>,
) -> Result<Option<i32>, SmokeRefusalV1> {
    use std::os::unix::process::ExitStatusExt;

    waited
        .map(|status| status.signal())
        .map_err(|_| SmokeRefusalV1::HostReapFailed)
}

/// The terminal plan for one early-readiness refusal cleanup.
///
/// This is the production decision rule for that path: it decides **both** whether the capped
/// readers may be joined and which refusal is finally reported, so a failed reap can never be
/// masked by the readiness refusal that caused the cleanup. Neither variant can express success,
/// and the caller always returns `Err`, so cleanup can never be converted into evidence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg(any(target_os = "linux", test))]
pub enum SessionCleanupPlanV1 {
    /// A reap was confirmed: the readers may be joined, then the original refusal is reported.
    JoinThenRefuse(SmokeRefusalV1),
    /// No reap was confirmed: nothing is joined and the reap failure overrides the refusal.
    UnverifiedReap,
}

#[cfg(any(target_os = "linux", test))]
impl SessionCleanupPlanV1 {
    /// Classifies one early-refusal cleanup from the checked wait outcome.
    ///
    /// `kill_accepted` is accepted for the record and deliberately excluded from the decision: a
    /// failed kill must not look like a terminated child, and a successful one still requires a
    /// reap before any reader may be joined.
    fn classify_v1(kill_accepted: bool, reap: SessionReapV1, refusal: SmokeRefusalV1) -> Self {
        let _ = kill_accepted;
        match reap {
            SessionReapV1::Reaped => Self::JoinThenRefuse(refusal),
            SessionReapV1::Unreaped => Self::UnverifiedReap,
        }
    }

    /// Whether the capped readers may be joined. Only a confirmed reap permits joining.
    fn joins_readers_v1(self) -> bool {
        matches!(self, Self::JoinThenRefuse(_))
    }

    /// The refusal actually reported for this cleanup.
    fn refusal_v1(self) -> SmokeRefusalV1 {
        match self {
            Self::JoinThenRefuse(refusal) => refusal,
            Self::UnverifiedReap => SmokeRefusalV1::HostReapFailed,
        }
    }
}

/// The one fixed live-attestation line, built only from an already validated attestation.
///
/// The marker states exactly what was proved — a live child observed at this check — and is
/// distinct from every earlier result. `reaped` is written from the parent's own lifecycle value,
/// never from any child text, so a child cannot author a reap or identity claim of its own.
fn session_evidence_line(attestation: SessionAttestationV1) -> String {
    format!(
        "{SESSION_EVIDENCE_MARKER_V1} as_soft={} as_hard={} cpu_soft={} cpu_hard={} pid={} \
         pgid={} sid={} observed_pgid={} observed_sid={} reaped={} wall_ms={}\n",
        attestation.observation.address_space_soft,
        attestation.observation.address_space_hard,
        attestation.observation.cpu_soft,
        attestation.observation.cpu_hard,
        attestation.observation.pid,
        attestation.observation.pgid,
        attestation.observation.sid,
        attestation.observed_pgid,
        attestation.observed_sid,
        u8::from(attestation.reaped),
        attestation.wall_ms
    )
}

/// The leader's single readiness record, exactly as the parent parses it.
///
/// It carries the four ceiling halves, the leader's own `pid`/`pgid`/`sid` and the descendant's
/// `pid`/`pgid`/`sid`. Nothing here is trusted on its own: the parent proves every identity again
/// through its own kernel queries and refuses on any disagreement.
#[cfg(any(target_os = "linux", test))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GroupChildObservationV1 {
    /// The leader's `RLIMIT_AS` soft ceiling, in bytes.
    pub address_space_soft: u64,
    /// The leader's `RLIMIT_AS` hard ceiling, in bytes.
    pub address_space_hard: u64,
    /// The leader's `RLIMIT_CPU` soft ceiling, in seconds.
    pub cpu_soft: u64,
    /// The leader's `RLIMIT_CPU` hard ceiling, in seconds.
    pub cpu_hard: u64,
    /// The leader's own process id.
    pub leader_pid: u64,
    /// The leader's process-group id, which `setsid` must have made equal to `leader_pid`.
    pub leader_pgid: u64,
    /// The leader's session id, which `setsid` must have made equal to `leader_pid`.
    pub leader_sid: u64,
    /// The one descendant's process id, which must differ from the leader's.
    pub descendant_pid: u64,
    /// The descendant's process-group id, which must be the **leader's** pid.
    pub descendant_pgid: u64,
    /// The descendant's session id, which must be the **leader's** pid.
    pub descendant_sid: u64,
}

/// The descendant's own readiness record, as the leader parses it.
///
/// Only the leader reads this, and only to prove that the process it spawned answered in its own
/// inherited group and session.
#[cfg(any(target_os = "linux", test))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GroupDescendantObservationV1 {
    /// The descendant's own process id.
    pub pid: u64,
    /// The descendant's process-group id, inherited from the leader.
    pub pgid: u64,
    /// The descendant's session id, inherited from the leader.
    pub sid: u64,
}

/// Parses the group leader's single readiness record, strictly.
///
/// Accepted form, exactly one line and nothing else:
///
/// ```text
/// SOLO-MEDIA-HOST-GROUP-V1 as_soft=<u64> as_hard=<u64> cpu_soft=<u64> cpu_hard=<u64> pid=<u64> pgid=<u64> sid=<u64> descendant_pid=<u64> descendant_pgid=<u64> descendant_sid=<u64>
/// ```
///
/// The marker, the ten field names and their order are fixed; every value is plain ASCII decimal
/// with no sign, no prefix, no separator and no whitespace; a single trailing newline is required
/// and a second line, a carriage return, a tab, a doubled space or any unknown, duplicated,
/// missing, extra or reordered field is refused.
///
/// # Errors
///
/// [`SmokeRefusalV1::HostOutputInvalid`] for any shape defect.
#[cfg(any(target_os = "linux", test))]
pub fn parse_group_readiness_v1(text: &str) -> Result<GroupChildObservationV1, SmokeRefusalV1> {
    let body = text
        .strip_suffix('\n')
        .ok_or(SmokeRefusalV1::HostOutputInvalid)?;
    if body.contains('\n') || body.contains('\r') || body.contains('\t') {
        return Err(SmokeRefusalV1::HostOutputInvalid);
    }

    let tokens: Vec<&str> = body.split(' ').collect();
    if tokens.len() != 11 || tokens[0] != GROUP_LEADER_READY_MARKER_V1 {
        return Err(SmokeRefusalV1::HostOutputInvalid);
    }

    Ok(GroupChildObservationV1 {
        address_space_soft: field_value(tokens[1], "as_soft=")?,
        address_space_hard: field_value(tokens[2], "as_hard=")?,
        cpu_soft: field_value(tokens[3], "cpu_soft=")?,
        cpu_hard: field_value(tokens[4], "cpu_hard=")?,
        leader_pid: field_value(tokens[5], "pid=")?,
        leader_pgid: field_value(tokens[6], "pgid=")?,
        leader_sid: field_value(tokens[7], "sid=")?,
        descendant_pid: field_value(tokens[8], "descendant_pid=")?,
        descendant_pgid: field_value(tokens[9], "descendant_pgid=")?,
        descendant_sid: field_value(tokens[10], "descendant_sid=")?,
    })
}

/// Parses the fixed descendant's single readiness record, strictly.
///
/// Accepted form, exactly one line and nothing else:
///
/// ```text
/// SOLO-MEDIA-HOST-GROUP-DESC-V1 pid=<u64> pgid=<u64> sid=<u64>
/// ```
///
/// # Errors
///
/// [`SmokeRefusalV1::HostOutputInvalid`] for any shape defect.
#[cfg(any(target_os = "linux", test))]
pub fn parse_group_descendant_readiness_v1(
    text: &str,
) -> Result<GroupDescendantObservationV1, SmokeRefusalV1> {
    let body = text
        .strip_suffix('\n')
        .ok_or(SmokeRefusalV1::HostOutputInvalid)?;
    if body.contains('\n') || body.contains('\r') || body.contains('\t') {
        return Err(SmokeRefusalV1::HostOutputInvalid);
    }

    let tokens: Vec<&str> = body.split(' ').collect();
    if tokens.len() != 4 || tokens[0] != GROUP_DESCENDANT_READY_MARKER_V1 {
        return Err(SmokeRefusalV1::HostOutputInvalid);
    }

    Ok(GroupDescendantObservationV1 {
        pid: field_value(tokens[1], "pid=")?,
        pgid: field_value(tokens[2], "pgid=")?,
        sid: field_value(tokens[3], "sid=")?,
    })
}

/// Validates the parsed leader record against the fixed profile and the parent's own handle.
///
/// The record must state the compile-time ceilings, a leader that is strictly its own group and
/// session leader, a descendant that is strictly **not** the leader, and a descendant that shares
/// exactly the leader's group and session. `expected_leader_pid` is the pid this parent's own
/// `Child` handle reported, so a record authored by any other process is refused.
///
/// # Errors
///
/// [`SmokeRefusalV1::HostOutputInvalid`] for a non-UTF-8 record;
/// [`SmokeRefusalV1::HostLimitUnavailable`] for any off-profile ceiling or disagreeing identity.
#[cfg(any(target_os = "linux", test))]
pub fn validate_group_readiness_v1(
    line: &[u8],
    expected_leader_pid: u64,
) -> Result<GroupChildObservationV1, SmokeRefusalV1> {
    let text = std::str::from_utf8(line).map_err(|_| SmokeRefusalV1::HostOutputInvalid)?;
    let observation = parse_group_readiness_v1(text)?;

    let limits_match = observation.address_space_soft == PROFILE_V1_AS_BYTES
        && observation.address_space_hard == PROFILE_V1_AS_BYTES
        && observation.cpu_soft == PROFILE_V1_CPU_SECONDS
        && observation.cpu_hard == PROFILE_V1_CPU_SECONDS;
    let leader_is_own_handle =
        expected_leader_pid > 1 && observation.leader_pid == expected_leader_pid;
    let leader_is_own_leader = observation.leader_pid > 1
        && observation.leader_pgid == observation.leader_pid
        && observation.leader_sid == observation.leader_pid;
    let descendant_is_distinct =
        observation.descendant_pid > 1 && observation.descendant_pid != observation.leader_pid;
    let descendant_is_in_group = observation.descendant_pgid == observation.leader_pid
        && observation.descendant_sid == observation.leader_pid;

    if !limits_match
        || !leader_is_own_handle
        || !leader_is_own_leader
        || !descendant_is_distinct
        || !descendant_is_in_group
    {
        return Err(SmokeRefusalV1::HostLimitUnavailable);
    }
    Ok(observation)
}

/// Validates the descendant record against the leader's own handle and group.
///
/// The record must name exactly the pid the leader's `Child` handle reported, that pid must not be
/// the leader itself, and both reported identities must be the leader's pid — the group and session
/// the descendant inherited, never a new one of its own.
///
/// # Errors
///
/// [`SmokeRefusalV1::HostOutputInvalid`] for a non-UTF-8 record;
/// [`SmokeRefusalV1::HostLimitUnavailable`] for a disagreeing identity.
#[cfg(any(target_os = "linux", test))]
pub fn validate_group_descendant_readiness_v1(
    line: &[u8],
    expected_descendant_pid: u64,
    leader_pid: u64,
) -> Result<GroupDescendantObservationV1, SmokeRefusalV1> {
    let text = std::str::from_utf8(line).map_err(|_| SmokeRefusalV1::HostOutputInvalid)?;
    let observation = parse_group_descendant_readiness_v1(text)?;

    let is_own_handle =
        expected_descendant_pid > 1 && leader_pid > 1 && observation.pid == expected_descendant_pid;
    let is_distinct = observation.pid != leader_pid;
    let inherited = observation.pgid == leader_pid && observation.sid == leader_pid;

    if !is_own_handle || !is_distinct || !inherited {
        return Err(SmokeRefusalV1::HostLimitUnavailable);
    }
    Ok(observation)
}

/// The parent's raw, unvalidated material for one group lifecycle attempt.
///
/// It carries only facts the **parent itself** established: its own handle's pids, its own kernel
/// queries of the leader and the descendant, the one signal it issued and the status it reaped, and
/// the bounded `/proc` observations of the same descendant identity. Nothing in it is evidence until
/// [`validate_group_attempt_v1`] accepts it, and no child text can set any field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GroupAttemptV1 {
    /// The exact positive pid this parent's own `Child` handle reported at spawn.
    pub leader_pid: u64,
    /// The exact positive pid this parent independently observed for the one descendant.
    pub descendant_pid: u64,
    /// The leader's kernel process-group id, observed by this parent while the leader was live.
    pub leader_pgid: u64,
    /// The leader's kernel session id, observed by this parent while the leader was live.
    pub leader_sid: u64,
    /// The descendant's kernel process-group id, observed by this parent before the signal.
    pub descendant_pgid: u64,
    /// The descendant's kernel session id, observed by this parent before the signal.
    pub descendant_sid: u64,
    /// Whether `try_wait()` reported the leader still running immediately **before** the signal.
    pub live_before: bool,
    /// Whether `try_wait()` reported the leader still running immediately **after** the queries.
    pub live_after: bool,
    /// Whether this parent's one group-directed signal was really accepted by the kernel.
    pub group_signal_issued: bool,
    /// The signal number the leader's own wait status reported, or `None` when it was unobservable.
    pub leader_signal: Option<i32>,
    /// Whether this parent actually reaped its own directly owned leader.
    pub leader_reaped: bool,
    /// The descendant's kernel start-time identity observed **before** the signal.
    pub descendant_start_before: u64,
    /// The same descendant's kernel start-time identity observed **after** the signal.
    pub descendant_start_after: u64,
    /// Whether the same descendant was observed non-running after the signal.
    pub descendant_nonrunning: bool,
    /// The parent-measured wall time from **before** leader spawn.
    pub wall_ms: u64,
}

/// The complete, validated group lifecycle result.
///
/// It only ever exists after the readiness record was accepted, both identities were proved by this
/// parent, exactly one group signal was accepted, the leader was reaped with the expected signal,
/// and the same descendant was observed non-running. It says nothing about **reaping** the
/// descendant: the parent holds no handle for it and must never claim one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GroupTerminationV1 {
    /// The leader pid this parent owned and reaped.
    pub leader_pid: u64,
    /// The descendant pid this parent observed start and then observe stop.
    pub descendant_pid: u64,
    /// The verified group identity, which is the leader's pid.
    pub group: u64,
    /// The verified session identity, which is the leader's pid.
    pub session: u64,
    /// The accepted termination signal.
    pub signal: i32,
    /// The parent-measured wall time.
    pub wall_ms: u64,
}

/// Validates one raw group attempt into accepted evidence, or refuses.
///
/// Every refusal path returns a typed refusal and therefore no evidence line at all. The rule keeps
/// the facts the order requires **distinct**: a signal **request**, a leader **reap**, a descendant
/// **non-running observation**, and a descendant **reap** — the last of which is unrepresentable
/// here, because neither this function nor its result type has a field for it.
///
/// # Errors
///
/// [`SmokeRefusalV1::HostLimitUnavailable`] for a missing group signal or a disagreeing identity;
/// [`SmokeRefusalV1::HostExitAbnormal`] for a leader not proved live or not reporting exactly the
/// accepted signal; [`SmokeRefusalV1::HostReapFailed`] for a missing leader reap;
/// [`SmokeRefusalV1::HostDescendantUnverified`] for a descendant not observed non-running under the
/// **same** start-time identity; [`SmokeRefusalV1::HostWallTimeout`] for a measurement at or beyond
/// the fixed deadline.
pub fn validate_group_attempt_v1(
    attempt: GroupAttemptV1,
) -> Result<GroupTerminationV1, SmokeRefusalV1> {
    if !attempt.group_signal_issued {
        return Err(SmokeRefusalV1::HostLimitUnavailable);
    }

    let leader_ok = attempt.leader_pid > 1
        && attempt.leader_pgid == attempt.leader_pid
        && attempt.leader_sid == attempt.leader_pid;
    let descendant_ok = attempt.descendant_pid > 1
        && attempt.descendant_pid != attempt.leader_pid
        && attempt.descendant_pgid == attempt.leader_pid
        && attempt.descendant_sid == attempt.leader_pid;
    if !leader_ok || !descendant_ok {
        return Err(SmokeRefusalV1::HostLimitUnavailable);
    }

    if !attempt.live_before || !attempt.live_after {
        return Err(SmokeRefusalV1::HostExitAbnormal);
    }
    if attempt.leader_signal != Some(GROUP_TERMINATION_SIGNAL_V1) {
        return Err(SmokeRefusalV1::HostExitAbnormal);
    }
    if !attempt.leader_reaped {
        return Err(SmokeRefusalV1::HostReapFailed);
    }

    if !attempt.descendant_nonrunning
        || attempt.descendant_start_before == 0
        || attempt.descendant_start_after != attempt.descendant_start_before
    {
        return Err(SmokeRefusalV1::HostDescendantUnverified);
    }

    if attempt.wall_ms == 0 || attempt.wall_ms >= GROUP_DEADLINE_MS {
        return Err(SmokeRefusalV1::HostWallTimeout);
    }

    Ok(GroupTerminationV1 {
        leader_pid: attempt.leader_pid,
        descendant_pid: attempt.descendant_pid,
        group: attempt.leader_pgid,
        session: attempt.leader_sid,
        signal: GROUP_TERMINATION_SIGNAL_V1,
        wall_ms: attempt.wall_ms,
    })
}

/// The one fixed group evidence line, built only from an already validated termination.
///
/// `leader_reaped` and `descendant_nonrunning` are written from the parent's own lifecycle and
/// observation values, never from any child text, and no field here can express a descendant reap.
fn group_evidence_line(termination: GroupTerminationV1) -> String {
    format!(
        "{GROUP_EVIDENCE_MARKER_V1} leader_pid={} descendant_pid={} pgid={} sid={} signal={} \
         leader_reaped=1 descendant_nonrunning=1 wall_ms={}\n",
        termination.leader_pid,
        termination.descendant_pid,
        termination.group,
        termination.session,
        termination.signal,
        termination.wall_ms
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
        Ok(InternalProbeModeV1::SessionParent) => Some(session_parent_report()),
        Ok(InternalProbeModeV1::SessionChild) => Some(session_child_report()),
        Ok(InternalProbeModeV1::GroupParent) => Some(group_parent_report()),
        Ok(InternalProbeModeV1::GroupLeader) => Some(group_leader_report()),
        Ok(InternalProbeModeV1::GroupDescendant) => Some(group_descendant_report()),
        Ok(InternalProbeModeV1::GroupInvalidParent) => Some(group_invalid_parent_report()),
        Ok(InternalProbeModeV1::GroupInvalidLeader) => Some(group_invalid_leader_report()),
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

/// Runs the one fixed Linux live-child session attestation and returns its raw material.
///
/// On Linux this starts exactly one `session-child` of this executable under the **unchanged**
/// pre-exec installer, observes that still-live, directly owned child's group and session through
/// read-only kernel queries, waits for its normal exit and reaps it inside the fixed 15 000 ms
/// deadline. On every other host it refuses before creating anything.
///
/// # Errors
///
/// Returns a [`SmokeRefusalV1`] for every launch, unobservable-status or failed-reap failure.
#[cfg(target_os = "linux")]
pub fn run_internal_session_attempt_v1() -> Result<SessionAttemptV1, SmokeRefusalV1> {
    linux::run_session_probe()
}

/// Runs the one fixed Linux live-child session attestation, which this host cannot provide.
///
/// # Errors
///
/// Always [`SmokeRefusalV1::HostUnsupported`]; no child is created and no session query is made.
#[cfg(not(target_os = "linux"))]
pub fn run_internal_session_attempt_v1() -> Result<SessionAttemptV1, SmokeRefusalV1> {
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

/// The session parent mode's whole result, as the composition root must print it.
///
/// As with the earlier proofs, the decision is made here in the platform-neutral layer, so a
/// refusal — including a watchdog kill, a child that was not proved live at the observation, or a
/// disagreement between the child's line and the parent's own kernel observation — is decided by
/// ordinary checked code rather than by the platform branch.
fn session_parent_report() -> InternalProbeReportV1 {
    match run_internal_session_attempt_v1().and_then(validate_session_attempt_v1) {
        Ok(attestation) => InternalProbeReportV1 {
            exit_code: 0,
            stdout: Some(session_evidence_line(attestation)),
            stderr: None,
        },
        Err(refusal) => refusal_report(refusal),
    }
}

/// The session child mode's whole result.
///
/// The child writes only its one readiness line to its own stdout, so this wrapper never carries a
/// payload: a successful child returns with no line of its own to print, and a verification
/// refusal is the child's own fixed status with no line either.
#[cfg(target_os = "linux")]
fn session_child_report() -> InternalProbeReportV1 {
    match linux::run_session_child() {
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

/// The session child mode is refused on a host with no approved pre-exec boundary: no child is
/// created, no session query is made and the fixed four-second sleep is never reached.
#[cfg(not(target_os = "linux"))]
fn session_child_report() -> InternalProbeReportV1 {
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

/// Runs the one fixed Linux group lifecycle proof.
///
/// On Linux this starts exactly one `group-leader` under the unchanged pre-exec installer, accepts
/// one bounded readiness record, re-proves both real identities through its own kernel queries,
/// issues exactly one group-directed signal to the group it owns, reaps its own leader with a
/// checked `Child::wait()`, and independently observes the same descendant non-running. Every other
/// host refuses before creating anything.
///
/// # Errors
///
/// Returns a [`SmokeRefusalV1`] for every launch, readiness, identity, signal, reap, delegation or
/// deadline failure.
#[cfg(target_os = "linux")]
pub fn run_internal_group_attempt_v1() -> Result<GroupAttemptV1, SmokeRefusalV1> {
    linux::run_group_probe()
}

/// Runs the one fixed Linux group lifecycle proof, which this host cannot provide.
///
/// # Errors
///
/// Always [`SmokeRefusalV1::HostUnsupported`]; no leader, descendant, query or signal occurs.
#[cfg(not(target_os = "linux"))]
pub fn run_internal_group_attempt_v1() -> Result<GroupAttemptV1, SmokeRefusalV1> {
    Err(SmokeRefusalV1::HostUnsupported)
}

/// Runs the fixed invalid-readiness group negative.
///
/// On Linux this starts exactly one `group-invalid-leader` under the same limits, receives its
/// deliberately malformed record, refuses with [`SmokeRefusalV1::HostOutputInvalid`] **before** any
/// group identity query, sends **zero** group signals, and reaps its own directly held leader with a
/// checked `Child::wait()`. A failed reap overrides the readiness refusal.
///
/// # Errors
///
/// [`SmokeRefusalV1::HostOutputInvalid`] on the intended path; any other [`SmokeRefusalV1`] for a
/// launch, deadline or unreaped-leader failure.
#[cfg(target_os = "linux")]
pub fn run_internal_group_invalid_attempt_v1() -> Result<(), SmokeRefusalV1> {
    linux::run_group_invalid_probe()
}

/// Runs the invalid-readiness group negative, which this host cannot provide.
///
/// # Errors
///
/// Always [`SmokeRefusalV1::HostUnsupported`]; no leader is created and no record is read.
#[cfg(not(target_os = "linux"))]
pub fn run_internal_group_invalid_attempt_v1() -> Result<(), SmokeRefusalV1> {
    Err(SmokeRefusalV1::HostUnsupported)
}

/// The group parent mode's whole result, as the composition root must print it.
///
/// As with the earlier proofs, the decision is made here in the platform-neutral layer, so a refusal
/// — including a rejected signal, an unreaped leader or an unverified descendant — is decided by
/// ordinary checked code rather than by the platform branch.
fn group_parent_report() -> InternalProbeReportV1 {
    match run_internal_group_attempt_v1().and_then(validate_group_attempt_v1) {
        Ok(termination) => InternalProbeReportV1 {
            exit_code: 0,
            stdout: Some(group_evidence_line(termination)),
            stderr: None,
        },
        Err(refusal) => refusal_report(refusal),
    }
}

/// The invalid-readiness group parent's whole result.
///
/// The intended path is a refusal, so this wrapper can never carry a success line: a returning
/// negative is itself treated as the closed output refusal it was supposed to produce.
fn group_invalid_parent_report() -> InternalProbeReportV1 {
    match run_internal_group_invalid_attempt_v1() {
        Ok(()) => refusal_report(SmokeRefusalV1::HostOutputInvalid),
        Err(refusal) => refusal_report(refusal),
    }
}

/// The group leader mode's whole result.
///
/// The leader writes only its own readiness record to stdout, so this wrapper never carries a
/// payload of its own: a leader that returns has refused, and reports its fixed status with no line.
#[cfg(target_os = "linux")]
fn group_leader_report() -> InternalProbeReportV1 {
    match linux::run_group_leader() {
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

/// The group leader mode is refused on a host with no approved pre-exec boundary.
#[cfg(not(target_os = "linux"))]
fn group_leader_report() -> InternalProbeReportV1 {
    refusal_report(SmokeRefusalV1::HostUnsupported)
}

/// The group descendant mode's whole result.
#[cfg(target_os = "linux")]
fn group_descendant_report() -> InternalProbeReportV1 {
    match linux::run_group_descendant() {
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

/// The group descendant mode is refused on a host with no approved pre-exec boundary.
#[cfg(not(target_os = "linux"))]
fn group_descendant_report() -> InternalProbeReportV1 {
    refusal_report(SmokeRefusalV1::HostUnsupported)
}

/// The invalid-readiness leader mode's whole result.
#[cfg(target_os = "linux")]
fn group_invalid_leader_report() -> InternalProbeReportV1 {
    match linux::run_group_invalid_leader() {
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

/// The invalid-readiness leader mode is refused on a host with no approved pre-exec boundary.
#[cfg(not(target_os = "linux"))]
fn group_invalid_leader_report() -> InternalProbeReportV1 {
    refusal_report(SmokeRefusalV1::HostUnsupported)
}

// ---------------------------------------------------------------------------------------------
// `R03.C3` — the confined image worker
// ---------------------------------------------------------------------------------------------

/// Which host's confinement the worker is running under.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageWorkerHostV1 {
    /// A Linux child: own session, own group, both ceilings installed before `exec`.
    Linux,
    /// A Windows child: created suspended, assigned to a job, then resumed.
    Windows,
}

/// The confinement facts one child attests about itself before it parses any byte.
///
/// Every value is read back from the child's own kernel/OS view, never passed in by a caller. The
/// parent nevertheless re-derives the same facts from its side; a child's claim is corroboration,
/// never authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImageWorkerConfinementV1 {
    host: ImageWorkerHostV1,
    address_space_soft: u64,
    address_space_hard: u64,
    cpu_soft: u64,
    cpu_hard: u64,
    pid: u32,
    pgid: u32,
    sid: u32,
}

impl ImageWorkerConfinementV1 {
    /// Whether this child really holds the fixed profile its parent claims to have installed.
    ///
    /// On Linux all four ceiling halves must equal the profile and the child must lead its own
    /// group and session. On Windows the job readback is the parent's fact, so the child's job
    /// membership check is what it can attest.
    #[must_use]
    pub const fn matches_profile_v1(&self) -> bool {
        match self.host {
            ImageWorkerHostV1::Linux => {
                self.address_space_soft == PROFILE_V1_AS_BYTES
                    && self.address_space_hard == PROFILE_V1_AS_BYTES
                    && self.cpu_soft == PROFILE_V1_CPU_SECONDS
                    && self.cpu_hard == PROFILE_V1_CPU_SECONDS
                    && self.pgid == self.pid
                    && self.sid == self.pid
                    && self.pid > 1
            }
            ImageWorkerHostV1::Windows => true,
        }
    }

    /// The exact host-specific frame fields this child must emit, in the agreed order.
    #[must_use]
    pub fn frame_fields_v1(&self) -> String {
        match self.host {
            ImageWorkerHostV1::Linux => format!(
                "host=linux as_soft={} as_hard={} cpu_soft={} cpu_hard={} pid={} pgid={} sid={}",
                self.address_space_soft,
                self.address_space_hard,
                self.cpu_soft,
                self.cpu_hard,
                self.pid,
                self.pgid,
                self.sid
            ),
            ImageWorkerHostV1::Windows => {
                format!("host=windows job=member pid={}", self.pid)
            }
        }
    }
}

/// Reads the calling child's own confinement facts, or `None` when they cannot be proved.
///
/// Linux: both ceilings and the child's group and session identity, all from the kernel, and the
/// group and session must be the child itself. Windows: the process id and a real `IsProcessInJob`
/// answer. Any unreadable value is `None`, so a child that cannot prove its own confinement never
/// decodes anything.
#[cfg(target_os = "linux")]
#[must_use]
pub fn child_image_worker_confinement_v1() -> Option<ImageWorkerConfinementV1> {
    let ceilings = linux::read_limit_ceilings_v1().ok()?;
    let identity = linux::observe_process_identity_v1(None).ok()?;
    Some(ImageWorkerConfinementV1 {
        host: ImageWorkerHostV1::Linux,
        address_space_soft: ceilings.address_space.soft,
        address_space_hard: ceilings.address_space.hard,
        cpu_soft: ceilings.cpu.soft,
        cpu_hard: ceilings.cpu.hard,
        pid: std::process::id(),
        pgid: identity.group,
        sid: identity.session,
    })
}

/// Reads the calling child's own confinement facts on Windows.
#[cfg(windows)]
#[must_use]
pub fn child_image_worker_confinement_v1() -> Option<ImageWorkerConfinementV1> {
    if !native_worker::in_job_v1() {
        return None;
    }
    Some(ImageWorkerConfinementV1 {
        host: ImageWorkerHostV1::Windows,
        address_space_soft: 0,
        address_space_hard: 0,
        cpu_soft: 0,
        cpu_hard: 0,
        pid: std::process::id(),
        pgid: 0,
        sid: 0,
    })
}

/// A host that implements neither confinement cannot run the worker at all.
#[cfg(not(any(target_os = "linux", windows)))]
#[must_use]
pub fn child_image_worker_confinement_v1() -> Option<ImageWorkerConfinementV1> {
    None
}

/// Why one supervised worker attempt produced no acceptable run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageWorkerRunRefusalV1 {
    /// The worker executable this adapter was bound to is not a file.
    ExecutableUnavailable,
    /// The confined spawn itself failed.
    SpawnFailed,
    /// The parent could not write the whole source to the child.
    WriteFailed,
    /// The wall deadline expired and the child was terminated.
    WallTimeout,
    /// The child wrote more than the fixed cap.
    OutputOversize,
    /// A reader thread failed or produced no bounded result.
    ReaderFailed,
    /// The child exited, but not with zero.
    ChildExited,
    /// The parent could not prove the confinement bounds were installed.
    LimitsUnverified,
    /// The parent could not confirm the child was reaped and its resources released.
    CleanupFailed,
    /// This host provides neither approved confinement.
    HostUnsupported,
}

impl ImageWorkerRunRefusalV1 {
    /// The inward refusal kind this host failure maps to.
    ///
    /// Only a failure to prove confinement or cleanup is reported as an unverified worker; every
    /// other host failure is an ordinary port refusal, so an OS error never becomes a media fact.
    #[must_use]
    pub const fn outcome_kind_v1(self) -> solo_application::GraphicImageInspectRefusalKindV1 {
        match self {
            Self::LimitsUnverified | Self::CleanupFailed => {
                solo_application::GraphicImageInspectRefusalKindV1::UnverifiedWorker
            }
            Self::ExecutableUnavailable
            | Self::SpawnFailed
            | Self::WriteFailed
            | Self::WallTimeout
            | Self::OutputOversize
            | Self::ReaderFailed
            | Self::ChildExited
            | Self::HostUnsupported => {
                solo_application::GraphicImageInspectRefusalKindV1::PortRefused
            }
        }
    }
}

/// One supervised worker attempt that ended with a reaped child.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageWorkerRunV1 {
    stdout: Vec<u8>,
    stderr_len: usize,
    exit_code: Option<i32>,
    wall_ms: u64,
    child_pid: u32,
    limits_verified: bool,
    cleanup_verified: bool,
}

impl ImageWorkerRunV1 {
    /// The exact bytes the child wrote to stdout, bounded by the fixed cap.
    #[must_use]
    pub fn stdout(&self) -> &[u8] {
        &self.stdout
    }

    /// How many bytes the child wrote to stderr, bounded by the fixed cap.
    #[must_use]
    pub const fn stderr_len(&self) -> usize {
        self.stderr_len
    }

    /// The child's exit code, when it exited normally.
    #[must_use]
    pub const fn exit_code(&self) -> Option<i32> {
        self.exit_code
    }

    /// The measured wall time of the attempt, in milliseconds.
    #[must_use]
    pub const fn wall_ms(&self) -> u64 {
        self.wall_ms
    }

    /// The child's process identifier, as the parent's own handle reported it.
    #[must_use]
    pub const fn child_pid(&self) -> u32 {
        self.child_pid
    }

    /// Whether the parent proved the confinement bounds were installed before the child ran.
    #[must_use]
    pub const fn limits_verified(&self) -> bool {
        self.limits_verified
    }

    /// Whether the parent confirmed the child was reaped and its resources released.
    #[must_use]
    pub const fn cleanup_verified(&self) -> bool {
        self.cleanup_verified
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

    pub(crate) use super::native_worker::{
        GroupSignalRefusalV1, ProcessStateV1, install_linux_child_limits_v1,
        observe_process_identity_v1, observe_process_state_v1, own_process_group_v1,
        read_limit_ceilings_v1, terminate_own_group_v1, terminate_owned_group_v1,
    };
    use super::{
        AS_CHILD_MARKER_V1, AS_DEADLINE_MS, AS_DENIED_TOKEN_V1, AS_REQUEST_BYTES_V1, AsAttemptV1,
        CHILD_EXIT_CPU_LOOP_RETURNED, CHILD_EXIT_GROUP_DESCENDANT_MISMATCH,
        CHILD_EXIT_GROUP_LEADER_SURVIVED, CHILD_EXIT_GROUP_MISMATCH, CHILD_EXIT_GROUP_SPAWN_FAILED,
        CHILD_EXIT_LIMITS_MISMATCH, CHILD_EXIT_READBACK_FAILED, CHILD_EXIT_RESERVE_GRANTED,
        CHILD_EXIT_SESSION_MISMATCH, CHILD_EXIT_SESSION_READBACK_FAILED,
        CHILD_OBSERVATION_MARKER_V1, CPU_CHILD_READY_MARKER_V1, CPU_DEADLINE_MS, CpuAttemptV1,
        GROUP_DEADLINE_MS, GROUP_DESCENDANT_LIFETIME_MS, GROUP_DESCENDANT_READY_DEADLINE_MS,
        GROUP_DESCENDANT_READY_MARKER_V1, GROUP_LEADER_HOLD_MS, GROUP_LEADER_READY_MARKER_V1,
        GroupAttemptV1, INTERNAL_PROBE_AS_CHILD_TOKEN_V1, INTERNAL_PROBE_CHILD_TOKEN_V1,
        INTERNAL_PROBE_CPU_CHILD_TOKEN_V1, INTERNAL_PROBE_FLAG_V1,
        INTERNAL_PROBE_GROUP_DESCENDANT_TOKEN_V1, INTERNAL_PROBE_GROUP_INVALID_LEADER_TOKEN_V1,
        INTERNAL_PROBE_GROUP_LEADER_TOKEN_V1, INTERNAL_PROBE_SESSION_CHILD_TOKEN_V1,
        INTERNAL_PROBE_WALL_CHILD_TOKEN_V1, MAX_CHILD_STDERR_BYTES, MAX_CHILD_STDOUT_BYTES,
        PROFILE_V1_AS_BYTES, PROFILE_V1_CPU_SECONDS, SESSION_CHILD_READY_MARKER_V1,
        SESSION_CHILD_SLEEP_MS, SESSION_DEADLINE_MS, SMOKE_WALL_DEADLINE_MS, SessionAttemptV1,
        SmokeObservationV1, SmokeRefusalV1, WALL_CHILD_READY_MARKER_V1, WALL_CHILD_SLEEP_MS,
        WALL_DEADLINE_MS, WallAttemptV1, classify_child_exit_v1, parse_child_observation_v1,
        validate_observation_v1,
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

    /// What the fixed 15 000 ms supervision decided about the one owned session child.
    ///
    /// The variants keep the possible outcomes apart so that only a normal, self-chosen exit can
    /// ever carry a live attestation; a status this parent produced itself is recorded as such and
    /// must be refused by the neutral validator.
    enum SessionSupervision {
        /// The child exited on its own before this parent's deadline; the value is its exit code.
        Observed(Option<i32>),
        /// The deadline expired: this parent signalled only its owned handle, and whether it reaped.
        ParentKilled(bool),
        /// Neither the status nor a clean reap could be observed, so nothing may be claimed.
        Unobservable,
    }

    /// Supervises the one fixed `session-child` and hands back its raw, unvalidated material.
    ///
    /// The timer starts **before** `Command::spawn`, the child is launched through the unchanged
    /// pre-exec installer, and the parent's own read-only kernel query is bracketed by two
    /// `try_wait` readbacks of the same `Child` handle, so the observation is provably of a child
    /// that was alive both immediately before and immediately after it. Only this parent's own
    /// `Child` handle is ever signalled, and only as bounded cleanup of that leader on a refusal or
    /// a deadline. No process group, descendant or foreign process is touched and no negative-pid
    /// signal is ever constructed.
    pub(super) fn run_session_probe() -> Result<SessionAttemptV1, SmokeRefusalV1> {
        use super::{SessionCleanupPlanV1, SessionReapV1};

        let executable =
            std::env::current_exe().map_err(|_| SmokeRefusalV1::HostExecutableUnavailable)?;
        if !executable.is_file() {
            return Err(SmokeRefusalV1::HostExecutableUnavailable);
        }

        let mut command = Command::new(&executable);
        command
            .arg(INTERNAL_PROBE_FLAG_V1)
            .arg(INTERNAL_PROBE_SESSION_CHILD_TOKEN_V1)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        // The same, unchanged pre-exec boundary: a new session and both ceilings before `exec`.
        install_linux_child_limits_v1(&mut command);

        let started = Instant::now();
        let mut child = command
            .spawn()
            .map_err(|_| SmokeRefusalV1::HostSpawnFailed)?;

        // The one exact positive pid this parent may ever query: its own handle's id.
        let handle_pid = child.id();
        if handle_pid == 0 {
            let _ = child.kill();
            let _ = child.wait();
            return Err(SmokeRefusalV1::HostSpawnFailed);
        }

        // One bounded, one-shot readiness handoff. The sender lives inside the capped stdout
        // reader and is dropped with it, so a closed stream, an early exit or a failed read
        // disconnects this receiver instead of leaving the parent waiting on a silent child.
        let (ready_tx, ready_rx) = std::sync::mpsc::channel::<Option<Vec<u8>>>();
        let out_reader = match child.stdout.take() {
            Some(stream) => Some(std::thread::spawn(move || {
                read_session_stdout(stream, MAX_CHILD_STDOUT_BYTES, ready_tx)
            })),
            None => {
                drop(ready_tx);
                None
            }
        };
        let err_reader = child
            .stderr
            .take()
            .map(|stream| std::thread::spawn(move || read_bounded(stream, MAX_CHILD_STDERR_BYTES)));

        // This bounded wait consumes only what is left of the one deadline that already started
        // before `spawn`. It never opens a second budget and never resets the measured window.
        let deadline = Duration::from_millis(SESSION_DEADLINE_MS);
        let remaining = deadline.saturating_sub(started.elapsed());
        let mut readiness_failure = None;
        let readiness_line = if remaining.is_zero() {
            readiness_failure = Some(SmokeRefusalV1::HostWallTimeout);
            None
        } else {
            match ready_rx.recv_timeout(remaining) {
                Ok(Some(line)) => {
                    let pid = u64::from(handle_pid);
                    match super::validate_session_readiness_v1(&line, pid) {
                        Ok(_) => Some(line),
                        Err(refusal) => {
                            readiness_failure = Some(refusal);
                            None
                        }
                    }
                }
                Ok(None) => {
                    readiness_failure = Some(SmokeRefusalV1::HostOutputInvalid);
                    None
                }
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                    readiness_failure = Some(SmokeRefusalV1::HostWallTimeout);
                    None
                }
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                    readiness_failure = Some(SmokeRefusalV1::HostOutputInvalid);
                    None
                }
            }
        };

        // The bracketed liveness readbacks and the single read-only kernel query exist only behind
        // a validated readiness line. Without that line the query is never reached at all, so no
        // spawn, reader, timer or `try_wait` success can substitute for the child's own readiness.
        let mut live_before = false;
        let mut live_after = false;
        let observed = if readiness_line.is_some() {
            // Immediately before the query, the child must still be running.
            live_before = matches!(child.try_wait(), Ok(None));
            // The read-only kernel observation of that still-live, directly owned child.
            let identity = if live_before {
                observe_process_identity_v1(Some(handle_pid)).ok()
            } else {
                None
            };
            // Immediately after the query, the same handle must still be running.
            live_after = matches!(child.try_wait(), Ok(None));
            identity
        } else {
            None
        };

        // A refusal before the query is bounded cleanup of this parent's own handle only. The kill
        // result is captured but is never evidence that the leader terminated, and the capped
        // readers are joined only after a checked `Child::wait()` has established the reap. If that
        // wait fails, the reap failure overrides the readiness refusal and no reader is joined, so
        // cleanup can never be reported as completed when it was not.
        if let Some(refusal) = readiness_failure {
            let kill_ok = child.kill().is_ok();
            let reap = SessionReapV1::from_wait_outcome_v1(child.wait());
            let plan = SessionCleanupPlanV1::classify_v1(kill_ok, reap, refusal);
            if plan.joins_readers_v1() {
                let _ = join_reader(out_reader);
                let _ = join_reader(err_reader);
            }
            return Err(plan.refusal_v1());
        }
        let outcome = loop {
            match child.try_wait() {
                // A normal, self-chosen exit before any kill by this parent.
                Ok(Some(status)) => break SessionSupervision::Observed(status.code()),
                Ok(None) => {}
                Err(_) => {
                    let _ = child.kill();
                    let _ = child.wait();
                    break SessionSupervision::Unobservable;
                }
            }
            if started.elapsed() >= deadline {
                // This parent's own watchdog expired. It signals only its owned handle and reaps
                // the leader: that status is the parent's own and can never become evidence.
                if child.kill().is_err() {
                    let _ = child.wait();
                    break SessionSupervision::Unobservable;
                }
                break match child.wait() {
                    Ok(_) => SessionSupervision::ParentKilled(true),
                    Err(_) => SessionSupervision::Unobservable,
                };
            }
            std::thread::sleep(POLL_INTERVAL);
        };
        let wall_ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX);

        // Joined only after the leader is gone, so a live child's open pipe ends cannot block.
        let stdout = join_reader(out_reader);
        let _stderr = join_reader(err_reader);

        let (observed_pgid, observed_sid) = match observed {
            Some(identity) => (u64::from(identity.group), u64::from(identity.session)),
            None => (0, 0),
        };

        match outcome {
            SessionSupervision::Observed(code) => Ok(SessionAttemptV1 {
                stdout: stdout.and_then(Result::ok),
                child_pid: u64::from(handle_pid),
                observed_pgid,
                observed_sid,
                live_before,
                live_after,
                exit_code: code,
                reaped: true,
                parent_killed: false,
                wall_ms,
                readiness_validated: true,
                readiness_line: readiness_line.clone(),
            }),
            SessionSupervision::ParentKilled(reaped) => Ok(SessionAttemptV1 {
                stdout: stdout.and_then(Result::ok),
                child_pid: u64::from(handle_pid),
                observed_pgid,
                observed_sid,
                live_before,
                live_after,
                exit_code: None,
                reaped,
                parent_killed: true,
                wall_ms,
                readiness_validated: true,
                readiness_line: readiness_line.clone(),
            }),
            SessionSupervision::Unobservable => Err(SmokeRefusalV1::HostReapFailed),
        }
    }

    /// The fixed session child: prove its own ceilings and its own session identity, then sleep.
    ///
    /// The four ceiling halves and the kernel's own `pid == pgid == sid` are verified **before** the
    /// readiness line and before any sleep, so a direct, unsupervised invocation refuses promptly,
    /// prints nothing and never reaches the four-second window. The child allocates no workload of
    /// its own, opens nothing, forks nothing, starts no thread and installs no signal handler: it
    /// writes one fixed line and then sleeps on its own thread.
    pub(super) fn run_session_child() -> Result<(), u8> {
        let readback = read_limit_ceilings_v1().map_err(|_| CHILD_EXIT_READBACK_FAILED)?;

        let limits_match = readback.address_space.soft == PROFILE_V1_AS_BYTES
            && readback.address_space.hard == PROFILE_V1_AS_BYTES
            && readback.cpu.soft == PROFILE_V1_CPU_SECONDS
            && readback.cpu.hard == PROFILE_V1_CPU_SECONDS;
        if !limits_match {
            return Err(CHILD_EXIT_LIMITS_MISMATCH);
        }

        let pid = u64::from(std::process::id());
        if pid == 0 {
            return Err(CHILD_EXIT_SESSION_READBACK_FAILED);
        }
        let identity =
            observe_process_identity_v1(None).map_err(|_| CHILD_EXIT_SESSION_READBACK_FAILED)?;
        let pgid = u64::from(identity.group);
        let sid = u64::from(identity.session);
        if pid != pgid || pid != sid {
            return Err(CHILD_EXIT_SESSION_MISMATCH);
        }

        // One fixed readiness line, written and flushed before the sleep, so the parent can
        // validate the child's own kernel answer while that child is still demonstrably alive.
        let line = format!(
            "{SESSION_CHILD_READY_MARKER_V1} as_soft={} as_hard={} cpu_soft={} cpu_hard={} \
             pid={pid} pgid={pgid} sid={sid}\n",
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
                .map_err(|_| CHILD_EXIT_SESSION_READBACK_FAILED)?;
        }

        // The one fixed four-second window that keeps a descendant-free child alive for the
        // parent's read-only observation: no CPU burn, no allocation stress and no descendants.
        std::thread::sleep(Duration::from_millis(SESSION_CHILD_SLEEP_MS));

        Ok(())
    }

    /// Reads the session child's whole capped stdout stream and hands its first complete line out.
    ///
    /// At the **first complete newline** this sends that exact prefix to the parent over the
    /// one-shot channel, without waiting for the parent and without waiting for EOF, so the parent
    /// can order its kernel query after the child's own flushed readiness. It keeps capturing under
    /// the same single [`MAX_CHILD_STDOUT_BYTES`] total cap until EOF, so the parent can reject
    /// trailing, duplicated or over-cap output after the leader is gone instead of trusting a
    /// partial early prefix. A read error, a closed stream without any newline, or one byte beyond
    /// the cap is a refusal, never a silent truncation.
    fn read_session_stdout(
        mut stream: impl Read,
        cap: usize,
        ready: std::sync::mpsc::Sender<Option<Vec<u8>>>,
    ) -> BoundedRead {
        let limit = u64::try_from(cap).map_err(|_| ())?.saturating_add(1);
        let mut buffer: Vec<u8> = Vec::new();
        let mut chunk = [0_u8; 128];
        let mut announced = false;
        let mut consumed: u64 = 0;
        loop {
            let remaining = limit.saturating_sub(consumed);
            if remaining == 0 {
                break;
            }
            let want = usize::try_from(remaining)
                .unwrap_or(usize::MAX)
                .min(chunk.len());
            match stream.read(&mut chunk[..want]) {
                Ok(0) => break,
                Ok(read) => {
                    consumed = consumed.saturating_add(u64::try_from(read).unwrap_or(u64::MAX));
                    buffer.extend_from_slice(&chunk[..read]);
                    if buffer.len() > cap {
                        if !announced {
                            let _ = ready.send(None);
                        }
                        return Err(());
                    }
                    if !announced {
                        let newline = buffer.iter().position(|byte| *byte == b'\n');
                        if let Some(end) = newline {
                            announced = true;
                            let _ = ready.send(Some(buffer[..=end].to_vec()));
                        }
                    }
                }
                Err(ref error) if error.kind() == std::io::ErrorKind::Interrupted => {}
                Err(_) => {
                    if !announced {
                        let _ = ready.send(None);
                    }
                    return Err(());
                }
            }
        }
        // A stream that closed before any newline never granted readiness.
        if !announced {
            let _ = ready.send(None);
        }
        Ok(buffer)
    }

    /// Carries one accepted readiness record through identity proof, one group signal, the checked
    /// leader reap and the bounded descendant observation.
    ///
    /// Order is the whole point, so it is fixed here and nowhere else: liveness bracketing, the
    /// leader's own identities, the descendant's identities, the descendant's live start-time
    /// identity, the **one** group signal, the checked leader reap, then the same descendant's
    /// non-running state. Every step that fails returns a typed refusal before the next one runs, so
    /// a signal can never be sent on unproved identities and no observation can be reported for a
    /// leader that was never reaped.
    fn probe_group_after_readiness(
        child: &mut std::process::Child,
        handle_pid: u32,
        observation: super::GroupChildObservationV1,
        started: Instant,
    ) -> Result<GroupAttemptV1, SmokeRefusalV1> {
        let deadline_at = started + Duration::from_millis(GROUP_DEADLINE_MS);

        let live_before = matches!(child.try_wait(), Ok(None));
        if !live_before {
            return Err(SmokeRefusalV1::HostExitAbnormal);
        }

        let owner = u64::from(handle_pid);
        let leader_identity = observe_process_identity_v1(Some(handle_pid))
            .map_err(|_| SmokeRefusalV1::HostLimitUnavailable)?;
        if u64::from(leader_identity.group) != owner || u64::from(leader_identity.session) != owner
        {
            return Err(SmokeRefusalV1::HostLimitUnavailable);
        }

        let descendant_pid = u32::try_from(observation.descendant_pid)
            .map_err(|_| SmokeRefusalV1::HostLimitUnavailable)?;
        let descendant_identity = observe_process_identity_v1(Some(descendant_pid))
            .map_err(|_| SmokeRefusalV1::HostLimitUnavailable)?;
        if u64::from(descendant_identity.group) != owner
            || u64::from(descendant_identity.session) != owner
        {
            return Err(SmokeRefusalV1::HostLimitUnavailable);
        }

        let descendant_start_before = match observe_process_state_v1(descendant_pid) {
            Ok(ProcessStateV1::Running { start_time }) if start_time > 0 => start_time,
            _ => return Err(SmokeRefusalV1::HostDescendantUnverified),
        };

        if !matches!(child.try_wait(), Ok(None)) {
            return Err(SmokeRefusalV1::HostExitAbnormal);
        }

        terminate_owned_group_v1(child, descendant_pid).map_err(|refusal| match refusal {
            GroupSignalRefusalV1::BadIdentity | GroupSignalRefusalV1::IdentityMismatch => {
                SmokeRefusalV1::HostLimitUnavailable
            }
            GroupSignalRefusalV1::LeaderNotLive | GroupSignalRefusalV1::SignalRejected => {
                SmokeRefusalV1::HostExitAbnormal
            }
        })?;

        // Linux-only group supervision: a failed reap cannot become an Ok attempt.
        let leader_signal = super::group_wait_signal_v1(child.wait())?;
        // Only the observed successful wait can establish this fact.
        let leader_reaped = true;

        // A disappearance is written with the earlier identity, because a recycled pid would instead
        // appear as a live or zombie record carrying a different start time, which is refused below.
        let mut descendant_nonrunning = false;
        let mut descendant_start_after = 0_u64;
        loop {
            match observe_process_state_v1(descendant_pid) {
                Ok(ProcessStateV1::Gone) => {
                    descendant_start_after = descendant_start_before;
                    descendant_nonrunning = true;
                    break;
                }
                Ok(ProcessStateV1::Zombie { start_time }) => {
                    descendant_start_after = start_time;
                    descendant_nonrunning = true;
                    break;
                }
                Ok(ProcessStateV1::Running { start_time }) => {
                    if start_time != descendant_start_before {
                        descendant_start_after = start_time;
                        break;
                    }
                }
                Err(_) => break,
            }
            if Instant::now() >= deadline_at {
                break;
            }
            std::thread::sleep(POLL_INTERVAL);
        }

        let wall_ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX);
        Ok(GroupAttemptV1 {
            leader_pid: owner,
            descendant_pid: observation.descendant_pid,
            leader_pgid: u64::from(leader_identity.group),
            leader_sid: u64::from(leader_identity.session),
            descendant_pgid: u64::from(descendant_identity.group),
            descendant_sid: u64::from(descendant_identity.session),
            live_before,
            live_after: true,
            group_signal_issued: true,
            leader_signal,
            leader_reaped,
            descendant_start_before,
            descendant_start_after,
            descendant_nonrunning,
            wall_ms,
        })
    }

    /// Supervises the one fixed `group-leader` and hands back its raw, unvalidated material.
    ///
    /// The single 15 000 ms budget starts **before** `Command::spawn`. The leader is launched through
    /// the unchanged pre-exec installer, so it is its own session and process-group leader with both
    /// fixed ceilings. The parent accepts at most one readiness record, then re-proves the leader and
    /// the descendant through its **own** kernel queries — bracketed by two `try_wait` readbacks of
    /// the same handle — before the one group-directed signal, its own checked leader reap, and the
    /// bounded `/proc` observation of the same descendant identity.
    ///
    /// When any pre-signal condition fails it sends **no** group signal at all: it reaps only its own
    /// handle through [`cleanup_owned_leader`] and reports a refusal. On the signal path it still
    /// establishes the leader reap with a checked `Child::wait()` before joining any capped reader.
    pub(super) fn run_group_probe() -> Result<GroupAttemptV1, SmokeRefusalV1> {
        let executable =
            std::env::current_exe().map_err(|_| SmokeRefusalV1::HostExecutableUnavailable)?;
        if !executable.is_file() {
            return Err(SmokeRefusalV1::HostExecutableUnavailable);
        }

        let mut command = Command::new(&executable);
        command
            .arg(INTERNAL_PROBE_FLAG_V1)
            .arg(INTERNAL_PROBE_GROUP_LEADER_TOKEN_V1)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        // The unchanged pre-exec boundary: a new session, then both ceilings, before `exec`.
        install_linux_child_limits_v1(&mut command);

        let started = Instant::now();
        let mut child = command
            .spawn()
            .map_err(|_| SmokeRefusalV1::HostSpawnFailed)?;

        let handle_pid = child.id();
        if handle_pid == 0 {
            return Err(cleanup_owned_leader(
                &mut child,
                None,
                None,
                SmokeRefusalV1::HostSpawnFailed,
            ));
        }

        let (ready_tx, ready_rx) = std::sync::mpsc::channel::<Option<Vec<u8>>>();
        let out_reader = match child.stdout.take() {
            Some(stream) => Some(std::thread::spawn(move || {
                read_session_stdout(stream, MAX_CHILD_STDOUT_BYTES, ready_tx)
            })),
            None => {
                drop(ready_tx);
                None
            }
        };
        let err_reader = child
            .stderr
            .take()
            .map(|stream| std::thread::spawn(move || read_bounded(stream, MAX_CHILD_STDERR_BYTES)));

        // One bounded, one-shot readiness handoff inside the single budget already started.
        let deadline = Duration::from_millis(GROUP_DEADLINE_MS);
        let remaining = deadline.saturating_sub(started.elapsed());
        let mut readiness_failure = None;
        let readiness = if remaining.is_zero() {
            readiness_failure = Some(SmokeRefusalV1::HostWallTimeout);
            None
        } else {
            match ready_rx.recv_timeout(remaining) {
                Ok(Some(line)) => {
                    match super::validate_group_readiness_v1(&line, u64::from(handle_pid)) {
                        Ok(observation) => Some(observation),
                        Err(refusal) => {
                            readiness_failure = Some(refusal);
                            None
                        }
                    }
                }
                Ok(None) => {
                    readiness_failure = Some(SmokeRefusalV1::HostOutputInvalid);
                    None
                }
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                    readiness_failure = Some(SmokeRefusalV1::HostWallTimeout);
                    None
                }
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                    readiness_failure = Some(SmokeRefusalV1::HostOutputInvalid);
                    None
                }
            }
        };

        // Without an accepted record the identity queries, the signal and the observation are never
        // reached at all: this path issues zero group signals and reaps only its own handle.
        let Some(observation) = readiness else {
            return Err(cleanup_owned_leader(
                &mut child,
                out_reader,
                err_reader,
                readiness_failure.unwrap_or(SmokeRefusalV1::HostOutputInvalid),
            ));
        };

        match probe_group_after_readiness(&mut child, handle_pid, observation, started) {
            Ok(attempt) => {
                // Joined only after the leader was reaped and the descendant was observed, so no
                // live writer can hold this parent on a pipe end.
                let _ = join_reader(out_reader);
                let _ = join_reader(err_reader);
                Ok(attempt)
            }
            Err(refusal) => Err(cleanup_owned_leader(
                &mut child, out_reader, err_reader, refusal,
            )),
        }
    }

    /// Bounded, checked cleanup of **this parent's own** directly held leader.
    ///
    /// It reuses the accepted `SOLO-WO-090 R01` rule unchanged: the kill result is recorded but never
    /// evidence, a confirmed reap is what allows the capped readers to be joined, and a failed reap
    /// overrides the original refusal with [`SmokeRefusalV1::HostReapFailed`]. It sends no group
    /// signal of any kind, so every refusal path through it issues exactly zero group `kill` calls.
    fn cleanup_owned_leader(
        child: &mut std::process::Child,
        out: Option<std::thread::JoinHandle<BoundedRead>>,
        err: Option<std::thread::JoinHandle<BoundedRead>>,
        refusal: SmokeRefusalV1,
    ) -> SmokeRefusalV1 {
        use super::{SessionCleanupPlanV1, SessionReapV1};

        let kill_accepted = child.kill().is_ok();
        let reap = SessionReapV1::from_wait_outcome_v1(child.wait());
        let plan = SessionCleanupPlanV1::classify_v1(kill_accepted, reap, refusal);
        if plan.joins_readers_v1() {
            let _ = join_reader(out);
            let _ = join_reader(err);
        }
        plan.refusal_v1()
    }

    /// The fixed group leader: prove its own ceilings and session, start exactly one descendant,
    /// verify that descendant, then emit one readiness record and hold.
    ///
    /// The descendant is started **without** the pre-exec installer, so it inherits this leader's
    /// group, session and both ceilings instead of creating a new session of its own. A direct,
    /// unsupervised `group-leader` invocation has no `setsid` and therefore refuses before spawning or
    /// printing anything.
    pub(super) fn run_group_leader() -> Result<(), u8> {
        let readback = read_limit_ceilings_v1().map_err(|_| CHILD_EXIT_READBACK_FAILED)?;
        let limits_match = readback.address_space.soft == PROFILE_V1_AS_BYTES
            && readback.address_space.hard == PROFILE_V1_AS_BYTES
            && readback.cpu.soft == PROFILE_V1_CPU_SECONDS
            && readback.cpu.hard == PROFILE_V1_CPU_SECONDS;
        if !limits_match {
            return Err(CHILD_EXIT_LIMITS_MISMATCH);
        }

        let pid = u64::from(std::process::id());
        if pid == 0 {
            return Err(CHILD_EXIT_SESSION_READBACK_FAILED);
        }
        let own =
            observe_process_identity_v1(None).map_err(|_| CHILD_EXIT_SESSION_READBACK_FAILED)?;
        if u64::from(own.group) != pid || u64::from(own.session) != pid {
            return Err(CHILD_EXIT_SESSION_MISMATCH);
        }

        let executable = std::env::current_exe().map_err(|_| CHILD_EXIT_GROUP_SPAWN_FAILED)?;
        if !executable.is_file() {
            return Err(CHILD_EXIT_GROUP_SPAWN_FAILED);
        }

        let mut command = Command::new(&executable);
        command
            .arg(INTERNAL_PROBE_FLAG_V1)
            .arg(INTERNAL_PROBE_GROUP_DESCENDANT_TOKEN_V1)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        // Deliberately **no** pre-exec installer: the descendant must inherit this group and these
        // ceilings, so it never calls `setsid` and never creates a session of its own.
        let mut descendant = command.spawn().map_err(|_| CHILD_EXIT_GROUP_SPAWN_FAILED)?;
        let descendant_pid = descendant.id();
        if descendant_pid == 0 || descendant_pid == std::process::id() {
            let _ = descendant.kill();
            let _ = descendant.wait();
            return Err(CHILD_EXIT_GROUP_DESCENDANT_MISMATCH);
        }

        let (ready_tx, ready_rx) = std::sync::mpsc::channel::<Option<Vec<u8>>>();
        let out_reader = match descendant.stdout.take() {
            Some(stream) => Some(std::thread::spawn(move || {
                read_session_stdout(stream, MAX_CHILD_STDOUT_BYTES, ready_tx)
            })),
            None => {
                drop(ready_tx);
                None
            }
        };
        let err_reader = descendant
            .stderr
            .take()
            .map(|stream| std::thread::spawn(move || read_bounded(stream, MAX_CHILD_STDERR_BYTES)));

        let ready_line = match ready_rx
            .recv_timeout(Duration::from_millis(GROUP_DESCENDANT_READY_DEADLINE_MS))
        {
            Ok(Some(line)) => Some(line),
            _ => None,
        };
        let verified = ready_line
            .as_deref()
            .map(|line| {
                super::validate_group_descendant_readiness_v1(line, u64::from(descendant_pid), pid)
            })
            .transpose();

        let observed = if verified.is_ok() {
            observe_process_identity_v1(Some(descendant_pid)).ok()
        } else {
            None
        };
        let live = matches!(descendant.try_wait(), Ok(None));
        // The descendant must be proved inside this leader's own group and session. Written as a
        // closure rather than a guarded `matches!` pattern so the source-boundary checker can parse
        // every macro argument as an ordinary expression.
        let observed_in_group = observed.is_some_and(|identity| {
            u64::from(identity.group) == pid && u64::from(identity.session) == pid
        });

        let ready = matches!(verified, Ok(Some(_))) && observed_in_group && live;
        if !ready {
            // A join is permitted only behind a **confirmed** reap of this leader's own descendant
            // handle. An unconfirmed wait must not be able to block this leader on an open pipe, so
            // the same rule the parent applies to its leader applies here to the descendant.
            let _ = descendant.kill();
            let reaped = descendant.wait().is_ok();
            if reaped {
                let _ = join_reader(out_reader);
                let _ = join_reader(err_reader);
            }
            return Err(CHILD_EXIT_GROUP_DESCENDANT_MISMATCH);
        }

        let line = format!(
            "{GROUP_LEADER_READY_MARKER_V1} as_soft={} as_hard={} cpu_soft={} cpu_hard={} \
             pid={pid} pgid={pid} sid={pid} descendant_pid={descendant_pid} \
             descendant_pgid={pid} descendant_sid={pid}\n",
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
                .map_err(|_| CHILD_EXIT_GROUP_DESCENDANT_MISMATCH)?;
        }

        // The parent is expected to signal the whole group well inside this hold. Reaching the end of
        // it means no group signal ever came, which is a refusal, never a termination: the descendant
        // still ends by itself inside its own fixed lifetime.
        std::thread::sleep(Duration::from_millis(GROUP_LEADER_HOLD_MS));
        Err(CHILD_EXIT_GROUP_LEADER_SURVIVED)
    }

    /// The fixed group descendant: prove inherited group and ceilings, emit readiness, then sleep a
    /// fixed finite life.
    ///
    /// It must be inside the leader's group and session but must **not** be a group leader itself, so
    /// `pgid == sid` is required and `pid == pgid` is refused. It creates no descendant of its own,
    /// forks nothing, opens nothing and installs no handler.
    pub(super) fn run_group_descendant() -> Result<(), u8> {
        let readback = read_limit_ceilings_v1().map_err(|_| CHILD_EXIT_READBACK_FAILED)?;
        let limits_match = readback.address_space.soft == PROFILE_V1_AS_BYTES
            && readback.address_space.hard == PROFILE_V1_AS_BYTES
            && readback.cpu.soft == PROFILE_V1_CPU_SECONDS
            && readback.cpu.hard == PROFILE_V1_CPU_SECONDS;
        if !limits_match {
            return Err(CHILD_EXIT_LIMITS_MISMATCH);
        }

        let pid = u64::from(std::process::id());
        if pid == 0 {
            return Err(CHILD_EXIT_SESSION_READBACK_FAILED);
        }
        let own =
            observe_process_identity_v1(None).map_err(|_| CHILD_EXIT_SESSION_READBACK_FAILED)?;
        let pgid = u64::from(own.group);
        let sid = u64::from(own.session);
        if pgid != sid || pid == pgid {
            return Err(CHILD_EXIT_GROUP_DESCENDANT_MISMATCH);
        }

        let line = format!("{GROUP_DESCENDANT_READY_MARKER_V1} pid={pid} pgid={pgid} sid={sid}\n");
        {
            let mut stdout = std::io::stdout();
            stdout
                .write_all(line.as_bytes())
                .and_then(|()| stdout.flush())
                .map_err(|_| CHILD_EXIT_GROUP_DESCENDANT_MISMATCH)?;
        }

        // The one fixed finite lifetime: the safety net that keeps a never-signalled descendant from
        // outliving a refused run.
        std::thread::sleep(Duration::from_millis(GROUP_DESCENDANT_LIFETIME_MS));
        Ok(())
    }

    /// Supervises the fixed invalid-readiness leader and refuses on its malformed record.
    ///
    /// This path exists to prove a real Linux **negative**: the record is shape-invalid, so
    /// [`super::validate_group_readiness_v1`] refuses with [`SmokeRefusalV1::HostOutputInvalid`]
    /// before any group identity authority exists. No group signal is reachable from here at all —
    /// the only signalling call in this function is the ordinary `Child::kill()` of this parent's own
    /// directly held leader inside [`cleanup_owned_leader`], and a failed reap still overrides the
    /// refusal.
    pub(super) fn run_group_invalid_probe() -> Result<(), SmokeRefusalV1> {
        let executable =
            std::env::current_exe().map_err(|_| SmokeRefusalV1::HostExecutableUnavailable)?;
        if !executable.is_file() {
            return Err(SmokeRefusalV1::HostExecutableUnavailable);
        }

        let mut command = Command::new(&executable);
        command
            .arg(INTERNAL_PROBE_FLAG_V1)
            .arg(INTERNAL_PROBE_GROUP_INVALID_LEADER_TOKEN_V1)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        install_linux_child_limits_v1(&mut command);

        let started = Instant::now();
        let mut child = command
            .spawn()
            .map_err(|_| SmokeRefusalV1::HostSpawnFailed)?;

        let (ready_tx, ready_rx) = std::sync::mpsc::channel::<Option<Vec<u8>>>();
        let out_reader = match child.stdout.take() {
            Some(stream) => Some(std::thread::spawn(move || {
                read_session_stdout(stream, MAX_CHILD_STDOUT_BYTES, ready_tx)
            })),
            None => {
                drop(ready_tx);
                None
            }
        };
        let err_reader = child
            .stderr
            .take()
            .map(|stream| std::thread::spawn(move || read_bounded(stream, MAX_CHILD_STDERR_BYTES)));

        let remaining = Duration::from_millis(GROUP_DEADLINE_MS).saturating_sub(started.elapsed());
        let refusal = if remaining.is_zero() {
            SmokeRefusalV1::HostWallTimeout
        } else {
            match ready_rx.recv_timeout(remaining) {
                Ok(Some(line)) => super::validate_group_readiness_v1(&line, u64::from(child.id()))
                    .err()
                    .unwrap_or(SmokeRefusalV1::HostOutputInvalid),
                Ok(None) => SmokeRefusalV1::HostOutputInvalid,
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => SmokeRefusalV1::HostWallTimeout,
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                    SmokeRefusalV1::HostOutputInvalid
                }
            }
        };

        // Zero group signals: this is the ordinary checked cleanup of this parent's own handle only.
        Err(cleanup_owned_leader(
            &mut child, out_reader, err_reader, refusal,
        ))
    }

    /// The fixed invalid-readiness leader: prove the same identity, create no descendant, write one
    /// deliberately malformed record and exit.
    ///
    /// Twelve tokens where the schema has eleven, so no identity field in it is ever authoritative.
    pub(super) fn run_group_invalid_leader() -> Result<(), u8> {
        let readback = read_limit_ceilings_v1().map_err(|_| CHILD_EXIT_READBACK_FAILED)?;
        let limits_match = readback.address_space.soft == PROFILE_V1_AS_BYTES
            && readback.address_space.hard == PROFILE_V1_AS_BYTES
            && readback.cpu.soft == PROFILE_V1_CPU_SECONDS
            && readback.cpu.hard == PROFILE_V1_CPU_SECONDS;
        if !limits_match {
            return Err(CHILD_EXIT_LIMITS_MISMATCH);
        }

        let pid = u64::from(std::process::id());
        let own =
            observe_process_identity_v1(None).map_err(|_| CHILD_EXIT_SESSION_READBACK_FAILED)?;
        if pid == 0 || u64::from(own.group) != pid || u64::from(own.session) != pid {
            return Err(CHILD_EXIT_SESSION_MISMATCH);
        }

        let line = format!(
            "{GROUP_LEADER_READY_MARKER_V1} as_soft={} as_hard={} cpu_soft={} cpu_hard={} \
             pid={pid} pgid={pid} sid={pid} descendant_pid=0 descendant_pgid=0 \
             descendant_sid=0 extra=1\n",
            readback.address_space.soft,
            readback.address_space.hard,
            readback.cpu.soft,
            readback.cpu.hard
        );
        let mut stdout = std::io::stdout();
        stdout
            .write_all(line.as_bytes())
            .and_then(|()| stdout.flush())
            .map_err(|_| CHILD_EXIT_GROUP_DESCENDANT_MISMATCH)?;
        Ok(())
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

/// One bounded reader that never grows past `cap` bytes.
///
/// The returned flag reports an overrun, so an oversized worker is refused instead of being
/// silently truncated into something that could look acceptable.
fn read_worker_stream_v1(stream: impl std::io::Read, cap: u64) -> (Vec<u8>, bool) {
    use std::io::Read as _;

    let mut collected: Vec<u8> = Vec::new();
    let mut handle = stream.take(cap.saturating_add(1));
    let mut overflow = false;
    let mut chunk = [0u8; 8192];
    loop {
        match handle.read(&mut chunk) {
            Ok(0) => break,
            Ok(read) => {
                collected.extend_from_slice(&chunk[..read]);
                if u64::try_from(collected.len()).unwrap_or(u64::MAX) > cap {
                    overflow = true;
                    break;
                }
            }
            Err(_) => return (collected, true),
        }
    }
    if u64::try_from(collected.len()).unwrap_or(u64::MAX) > cap {
        overflow = true;
    }
    (collected, overflow)
}

/// Runs one confined image worker on Linux and reaps it before ever joining a reader.
///
/// The unchanged accepted pre-exec boundary installs the new session and both ceilings before the
/// child is executed. The parent then proves the child leads its own group and session, enforces the
/// fixed wall deadline, and on expiry or any early refusal terminates exactly **its own** group and
/// confirms a real reap. Reader threads are joined only after that confirmation, so no live writer
/// can hold this parent on a pipe end.
#[cfg(target_os = "linux")]
fn run_confined_image_worker_platform_v1(
    executable: &std::path::Path,
    format_token: &str,
    source_sha_hex: &str,
    mode_token: Option<&str>,
    source: &[u8],
    _fault: &mut N102FaultRunV1,
) -> Result<ImageWorkerRunV1, ImageWorkerRunRefusalV1> {
    use std::io::Write as _;
    use std::process::{Command, Stdio};
    use std::time::{Duration, Instant};

    if !executable.is_file() {
        return Err(ImageWorkerRunRefusalV1::ExecutableUnavailable);
    }

    let mut command = Command::new(executable);
    command
        .arg(crate::graphic_image_inspection::INTERNAL_IMAGE_WORKER_FLAG_V1)
        .arg(format_token)
        .arg(source_sha_hex)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some(mode) = mode_token {
        command.arg(mode);
    }

    // The accepted installer: `setsid`, then both ceilings, before `exec`.
    linux::install_linux_child_limits_v1(&mut command);

    let started = Instant::now();
    let mut child = command
        .spawn()
        .map_err(|_| ImageWorkerRunRefusalV1::SpawnFailed)?;
    let child_pid = child.id();
    if child_pid <= 1 {
        let _ = child.kill();
        let _ = child.wait();
        return Err(ImageWorkerRunRefusalV1::SpawnFailed);
    }

    // The parent's own identity proof, taken while the child is alive.
    let limits_verified = match linux::observe_process_identity_v1(Some(child_pid)) {
        Ok(identity) => identity.group == child_pid && identity.session == child_pid,
        Err(_) => false,
    };

    let stdout_cap = worker_stdout_cap_v1(format_token);
    let out_reader = child
        .stdout
        .take()
        .map(|stream| std::thread::spawn(move || read_worker_stream_v1(stream, stdout_cap)));
    let err_reader = child.stderr.take().map(|stream| {
        std::thread::spawn(move || read_worker_stream_v1(stream, MAX_CHILD_STDERR_BYTES as u64))
    });
    let writer = child.stdin.take().map(|mut stream| {
        let payload = source.to_vec();
        std::thread::spawn(move || stream.write_all(&payload).is_ok())
    });

    let deadline = Duration::from_millis(solo_application::IMAGE_WORKER_WALL_MS_V1);
    let mut timed_out = false;
    let mut reaped = false;
    let mut exit_code: Option<i32> = None;
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                reaped = true;
                exit_code = status.code();
                break;
            }
            Ok(None) => {
                if started.elapsed() >= deadline {
                    timed_out = true;
                    break;
                }
                std::thread::sleep(Duration::from_millis(10));
            }
            Err(_) => break,
        }
    }

    if !reaped {
        // Bounded group termination of exactly this parent's owned group, then a real reap.
        let _ = linux::terminate_own_group_v1(&mut child);
        if child.wait().is_ok() {
            reaped = true;
        }
    }
    let cleanup_verified = reaped;
    let wall_ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX);

    let written = writer
        .map(|handle| handle.join().unwrap_or(false))
        .unwrap_or(false);
    let (stderr_bytes, stderr_over) = err_reader
        .map(|handle| handle.join().unwrap_or((Vec::new(), true)))
        .unwrap_or((Vec::new(), false));
    let (stdout_bytes, stdout_over) = out_reader
        .map(|handle| handle.join().unwrap_or((Vec::new(), true)))
        .unwrap_or((Vec::new(), false));

    if !cleanup_verified {
        return Err(ImageWorkerRunRefusalV1::CleanupFailed);
    }
    if timed_out {
        return Err(ImageWorkerRunRefusalV1::WallTimeout);
    }
    if exit_code != Some(0) {
        return Err(ImageWorkerRunRefusalV1::ChildExited);
    }
    if !written {
        return Err(ImageWorkerRunRefusalV1::WriteFailed);
    }
    if stderr_over || stdout_over {
        return Err(ImageWorkerRunRefusalV1::OutputOversize);
    }
    if !limits_verified {
        return Err(ImageWorkerRunRefusalV1::LimitsUnverified);
    }

    Ok(ImageWorkerRunV1 {
        stdout: stdout_bytes,
        stderr_len: stderr_bytes.len(),
        exit_code,
        wall_ms,
        child_pid,
        limits_verified,
        cleanup_verified,
    })
}

/// Runs one confined image worker on Windows and reaps it before ever joining a reader.
///
/// The worker is created **suspended**. The job object is created, limited, read back and assigned
/// while the worker still cannot execute a single instruction, and only then is its primary thread
/// resumed. That ordering is the whole point: an ordinary spawn-then-assign would let the decoder
/// start before any bound existed. On expiry or any early refusal the parent terminates exactly its
/// own job and requires a real process wait before reporting cleanup.
#[cfg(windows)]
fn run_confined_image_worker_platform_v1(
    executable: &std::path::Path,
    format_token: &str,
    source_sha_hex: &str,
    mode_token: Option<&str>,
    source: &[u8],
    fault: &mut N102FaultRunV1,
) -> Result<ImageWorkerRunV1, ImageWorkerRunRefusalV1> {
    use std::io::Write as _;
    use std::time::{Duration, Instant};

    if !executable.is_file() {
        return Err(ImageWorkerRunRefusalV1::ExecutableUnavailable);
    }

    let started = Instant::now();
    let arguments = [
        crate::graphic_image_inspection::INTERNAL_IMAGE_WORKER_FLAG_V1,
        format_token,
        source_sha_hex,
    ];
    // One per-run native fault context: production passes no request, so it can never be injected.
    let mut native_fault = match fault.requested() {
        Some(step) => native_worker::N102FaultContextV1::requesting(step),
        None => native_worker::N102FaultContextV1::production(),
    };
    let worker = match mode_token {
        Some(mode) => {
            let with_mode = [arguments[0], arguments[1], arguments[2], mode];
            native_worker::SuspendedWorkerV1::spawn(executable, &with_mode, &mut native_fault)
        }
        None => native_worker::SuspendedWorkerV1::spawn(executable, &arguments, &mut native_fault),
    };
    fault.absorb(native_fault.reached(), native_fault.cleanup_verified());
    let mut worker = worker.ok_or(ImageWorkerRunRefusalV1::SpawnFailed)?;

    let child_pid = worker.pid();
    let limits_verified = worker.limits_verified();
    let Some(mut stdin) = worker.take_stdin() else {
        let _ = worker.terminate_job();
        let _ = worker.wait_ms(5_000, &mut native_fault);
        return Err(ImageWorkerRunRefusalV1::SpawnFailed);
    };
    let Some(stdout) = worker.take_stdout() else {
        let _ = worker.terminate_job();
        let _ = worker.wait_ms(5_000, &mut native_fault);
        return Err(ImageWorkerRunRefusalV1::SpawnFailed);
    };

    let stdout_cap = worker_stdout_cap_v1(format_token);
    let out_reader = std::thread::spawn(move || read_worker_stream_v1(stdout, stdout_cap));
    let writer = {
        let payload = source.to_vec();
        std::thread::spawn(move || stdin.write_all(&payload).is_ok())
    };

    // Only now, with the job attached and its limits read back, may the worker execute.
    if !worker.resume(&mut native_fault) {
        // A refused resume still owns a live suspended child: it is terminated, its exit is
        // observed, and both the reached step and the real cleanup outcome are recorded before
        // the refusal is reported.
        let _ = worker.terminate_job();
        let _ = writer.join();
        let _ = out_reader.join();
        let exited = worker.wait_ms(10_000, &mut native_fault).is_some();
        fault.absorb(
            native_fault.reached(),
            native_fault.cleanup_verified() || exited,
        );
        return Err(ImageWorkerRunRefusalV1::SpawnFailed);
    }

    let deadline = Duration::from_millis(solo_application::IMAGE_WORKER_WALL_MS_V1);
    let mut timed_out = false;
    let mut exit_code: Option<u32> = None;
    loop {
        if let Some(code) = worker.wait_ms(10, &mut native_fault) {
            exit_code = Some(code);
            break;
        }
        if started.elapsed() >= deadline {
            timed_out = true;
            break;
        }
    }
    if exit_code.is_none() {
        let _ = worker.terminate_job();
        exit_code = worker.wait_ms(10_000, &mut native_fault);
    }
    fault.absorb(
        native_fault.reached(),
        native_fault.cleanup_verified() || exit_code.is_some(),
    );
    let cleanup_verified = exit_code.is_some();
    let wall_ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX);

    let written = writer.join().unwrap_or(false);
    let (stdout_bytes, stdout_over) = out_reader.join().unwrap_or((Vec::new(), true));

    if !cleanup_verified {
        return Err(ImageWorkerRunRefusalV1::CleanupFailed);
    }
    if timed_out {
        return Err(ImageWorkerRunRefusalV1::WallTimeout);
    }
    let normal_exit = exit_code.and_then(|code| i32::try_from(code).ok());
    if normal_exit != Some(0) {
        return Err(ImageWorkerRunRefusalV1::ChildExited);
    }
    if !written {
        return Err(ImageWorkerRunRefusalV1::WriteFailed);
    }
    if stdout_over {
        return Err(ImageWorkerRunRefusalV1::OutputOversize);
    }
    if !limits_verified {
        return Err(ImageWorkerRunRefusalV1::LimitsUnverified);
    }

    Ok(ImageWorkerRunV1 {
        stdout: stdout_bytes,
        stderr_len: 0,
        exit_code: normal_exit,
        wall_ms,
        child_pid,
        limits_verified,
        cleanup_verified,
    })
}

/// Runs one confined image worker attempt on a host that provides neither approved confinement.
#[cfg(not(any(target_os = "linux", windows)))]
fn run_confined_image_worker_platform_v1(
    _executable: &std::path::Path,
    _format_token: &str,
    _source_sha_hex: &str,
    _mode_token: Option<&str>,
    _source: &[u8],
    _fault: &mut N102FaultRunV1,
) -> Result<ImageWorkerRunV1, ImageWorkerRunRefusalV1> {
    Err(ImageWorkerRunRefusalV1::HostUnsupported)
}

/// Runs one confined image worker in exactly one optional closed mode.
///
/// The mode exists for the closed test seams only; production composition passes `None`. The
/// executable is always the trusted caller's own image and the arguments are always one closed flag,
/// one closed format token and one digest.
pub(crate) fn run_confined_image_worker_mode_v1(
    executable: &std::path::Path,
    format_token: &str,
    source_sha_hex: &str,
    mode_token: Option<&str>,
    source: &[u8],
    fault: &mut N102FaultRunV1,
) -> Result<ImageWorkerRunV1, ImageWorkerRunRefusalV1> {
    run_confined_image_worker_platform_v1(
        executable,
        format_token,
        source_sha_hex,
        mode_token,
        source,
        fault,
    )
}

/// Confines, runs and reaps exactly one image worker for one bounded request.
///
/// This is the only way the adapter can start a decoder. It never accepts a command line, an
/// environment value or an output destination: the executable is the trusted composition root's own
/// image, and the arguments are one closed flag, one closed format token and one digest.
///
/// # Errors
///
/// A closed [`ImageWorkerRunRefusalV1`]: the executable was unavailable, the confined spawn failed,
/// the source could not be written, the deadline expired, the child exceeded the output cap, a
/// reader failed, the child exited non-zero, the parent could not prove confinement, or it could not
/// confirm the reap.
pub fn run_confined_image_worker_v1(
    executable: &std::path::Path,
    format_token: &str,
    source_sha_hex: &str,
    source: &[u8],
) -> Result<ImageWorkerRunV1, ImageWorkerRunRefusalV1> {
    run_confined_image_worker_platform_v1(
        executable,
        format_token,
        source_sha_hex,
        None,
        source,
        &mut N102FaultRunV1::production(),
    )
}

/// The fixed per-format stdout cap.
///
/// Exactly the closed `gif` token gets the smaller GIF bound, applied by the reader while it
/// reads, so an oversize GIF response is capped (and therefore refused) during read rather than
/// after a larger still-image allowance was already collected. Every other token keeps the
/// unchanged still-image cap.
fn worker_stdout_cap_v1(format_token: &str) -> u64 {
    if format_token == crate::graphic_image_inspection::GIF_WORKER_TOKEN_V1 {
        crate::graphic_image_inspection::MAX_GIF_WORKER_STDOUT_BYTES_V1
    } else {
        crate::graphic_image_inspection::MAX_IMAGE_WORKER_STDOUT_BYTES_V1
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

    /// The exact session readiness line the fixed child is required to write.
    const WELL_FORMED_SESSION: &str = "SOLO-MEDIA-HOST-SESSION-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 pid=4242 pgid=4242 sid=4242\n";

    /// One raw attempt that a correct parent could have produced for that line.
    fn session_attempt() -> SessionAttemptV1 {
        SessionAttemptV1 {
            stdout: Some(WELL_FORMED_SESSION.as_bytes().to_vec()),
            child_pid: 4242,
            observed_pgid: 4242,
            observed_sid: 4242,
            live_before: true,
            live_after: true,
            exit_code: Some(0),
            reaped: true,
            parent_killed: false,
            wall_ms: 4_050,
            readiness_validated: true,
            readiness_line: Some(WELL_FORMED_SESSION.as_bytes().to_vec()),
        }
    }

    /// `N087-03`: both new tokens are closed and additive on either host, and every malformed,
    /// extra, repeated or misplaced token is the same typed refusal rather than a fall-through.
    /// Every earlier mode must still classify exactly as it did before this proof existed.
    #[test]
    fn n087_03_the_session_tokens_are_closed_and_additive() {
        for (token, mode) in [
            ("session-parent", InternalProbeModeV1::SessionParent),
            ("session-child", InternalProbeModeV1::SessionChild),
            ("parent", InternalProbeModeV1::Parent),
            ("child", InternalProbeModeV1::Child),
            ("wall-parent", InternalProbeModeV1::WallParent),
            ("wall-child", InternalProbeModeV1::WallChild),
            ("as-parent", InternalProbeModeV1::AsParent),
            ("as-child", InternalProbeModeV1::AsChild),
            ("cpu-parent", InternalProbeModeV1::CpuParent),
            ("cpu-child", InternalProbeModeV1::CpuChild),
        ] {
            assert_eq!(
                classify_internal_arguments_v1(&argv(&[INTERNAL_PROBE_FLAG_V1, token])),
                Ok(mode),
                "`{token}` must classify unchanged"
            );
        }

        for wrong in [
            vec![INTERNAL_PROBE_FLAG_V1, "session"],
            vec![INTERNAL_PROBE_FLAG_V1, "SESSION-PARENT"],
            vec![INTERNAL_PROBE_FLAG_V1, "session_parent"],
            vec![INTERNAL_PROBE_FLAG_V1, "session-parent", "session-parent"],
            vec![INTERNAL_PROBE_FLAG_V1, "session-child", "extra"],
            vec![INTERNAL_PROBE_FLAG_V1, "session-parent", "session-child"],
            vec![INTERNAL_PROBE_FLAG_V1, " session-parent"],
            vec![INTERNAL_PROBE_FLAG_V1, "session-child "],
            vec![INTERNAL_PROBE_FLAG_V1],
        ] {
            assert_eq!(
                classify_internal_arguments_v1(&argv(&wrong)),
                Err(SmokeRefusalV1::InternalModeInvalid),
                "must refuse {wrong:?}"
            );
        }

        // Neither mode is reachable behind a public subcommand.
        assert_eq!(
            classify_internal_arguments_v1(&argv(&[
                "bootstrap-preflight",
                INTERNAL_PROBE_FLAG_V1,
                "session-parent",
            ])),
            Ok(InternalProbeModeV1::NotRequested),
            "the reserved flag is honoured only in second position"
        );
    }

    /// `N087-03`: only the exact readiness line is accepted. Negative, zero, overflowing,
    /// reordered, duplicated, extra, tabbed, doubled-space, CRLF, unterminated and foreign-marker
    /// forms are all refused, and a well-formed but off-profile or non-identical line is refused by
    /// the validator rather than the reader.
    #[test]
    fn n087_03_only_the_exact_session_readiness_line_is_accepted() {
        let parsed =
            parse_session_child_readiness_v1(WELL_FORMED_SESSION).expect("the fixed line parses");
        assert_eq!(parsed.pid, 4242);
        assert_eq!(parsed.pgid, 4242);
        assert_eq!(parsed.sid, 4242);
        assert_eq!(parsed.address_space_soft, PROFILE_V1_AS_BYTES);
        assert_eq!(parsed.cpu_hard, PROFILE_V1_CPU_SECONDS);

        let malformed = [
            "SOLO-MEDIA-HOST-SESSION-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 pid=4242 pgid=4242\n",
            "SOLO-MEDIA-HOST-SESSION-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 pid=4242 pgid=4242 sid=4242 sid=4242\n",
            "SOLO-MEDIA-HOST-SESSION-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 pid=4242 pgid=4242 sid=4242 extra=1\n",
            "SOLO-MEDIA-HOST-SESSION-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 pgid=4242 pid=4242 sid=4242\n",
            "SOLO-MEDIA-HOST-SESSION-V1 as_soft=-536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 pid=4242 pgid=4242 sid=4242\n",
            "SOLO-MEDIA-HOST-SESSION-V1 as_soft=+536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 pid=4242 pgid=4242 sid=4242\n",
            "SOLO-MEDIA-HOST-SESSION-V1 as_soft=99999999999999999999999999 as_hard=536870912 cpu_soft=10 cpu_hard=10 pid=4242 pgid=4242 sid=4242\n",
            "SOLO-MEDIA-HOST-SESSION-V1  as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 pid=4242 pgid=4242 sid=4242\n",
            "SOLO-MEDIA-HOST-SESSION-V1 as_soft=536870912\tas_hard=536870912 cpu_soft=10 cpu_hard=10 pid=4242 pgid=4242 sid=4242\n",
            "SOLO-MEDIA-HOST-SESSION-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 pid=4242 pgid=4242 sid=4242\r\n",
            "SOLO-MEDIA-HOST-PROBE-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 pid=4242 pgid=4242 sid=4242\n",
            "SOLO-MEDIA-HOST-SESSION-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 pid=4242 pgid=4242 sid=4242",
        ];
        for text in malformed {
            assert_eq!(
                parse_session_child_readiness_v1(text),
                Err(SmokeRefusalV1::HostOutputInvalid),
                "must refuse a shape defect"
            );
        }

        let off_profile = [
            "SOLO-MEDIA-HOST-SESSION-V1 as_soft=536870913 as_hard=536870912 cpu_soft=10 cpu_hard=10 pid=4242 pgid=4242 sid=4242\n",
            "SOLO-MEDIA-HOST-SESSION-V1 as_soft=536870912 as_hard=536870912 cpu_soft=11 cpu_hard=10 pid=4242 pgid=4242 sid=4242\n",
            "SOLO-MEDIA-HOST-SESSION-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 pid=0 pgid=0 sid=0\n",
            "SOLO-MEDIA-HOST-SESSION-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 pid=4242 pgid=4243 sid=4242\n",
            "SOLO-MEDIA-HOST-SESSION-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 pid=4242 pgid=4242 sid=4243\n",
        ];
        for text in off_profile {
            let parsed = parse_session_child_readiness_v1(text).expect("well formed");
            assert_eq!(
                validate_session_child_v1(parsed),
                Err(SmokeRefusalV1::HostLimitUnavailable),
                "must refuse an off-profile or non-identical line"
            );
        }
    }

    /// `N087-03`: a forged identity line alone is never accepted. The child's own claim only
    /// becomes evidence when the parent's independent kernel observation of its still-live child
    /// agrees with it, so an attestation cannot be manufactured from child text.
    #[test]
    fn n087_03_a_forged_session_line_is_never_accepted_without_a_live_observation() {
        // The line parses and validates on its own ...
        let parsed = parse_session_child_readiness_v1(WELL_FORMED_SESSION).expect("shape");
        assert!(validate_session_child_v1(parsed).is_ok());

        // ... but with no parent observation it is not evidence.
        let mut unobserved = session_attempt();
        unobserved.observed_pgid = 0;
        unobserved.observed_sid = 0;
        assert_eq!(
            validate_session_attempt_v1(unobserved),
            Err(SmokeRefusalV1::HostLimitUnavailable),
            "a child's own claim cannot stand in for the parent's kernel observation"
        );

        // A disagreement between the two views is a refusal, in either identity.
        let mut wrong_group = session_attempt();
        wrong_group.observed_pgid = 4_243;
        assert_eq!(
            validate_session_attempt_v1(wrong_group),
            Err(SmokeRefusalV1::HostLimitUnavailable)
        );
        let mut wrong_session = session_attempt();
        wrong_session.observed_sid = 4_243;
        assert_eq!(
            validate_session_attempt_v1(wrong_session),
            Err(SmokeRefusalV1::HostLimitUnavailable)
        );

        // A child whose pid is not the one this parent's handle reported is refused.
        let mut wrong_handle = session_attempt();
        wrong_handle.child_pid = 4_243;
        assert_eq!(
            validate_session_attempt_v1(wrong_handle),
            Err(SmokeRefusalV1::HostLimitUnavailable),
            "the line's pid must be the exact pid of the parent's own handle"
        );

        // The child must be proved alive on both sides of the query.
        for (before, after) in [(false, true), (true, false), (false, false)] {
            let mut attempt = session_attempt();
            attempt.live_before = before;
            attempt.live_after = after;
            assert_eq!(
                validate_session_attempt_v1(attempt),
                Err(SmokeRefusalV1::HostExitAbnormal),
                "a child not proved live across the observation is never evidence"
            );
        }
    }

    /// `N087-03`: every incomplete or abnormal lifecycle is refused, so only a normal zero exit of
    /// a reaped leader, inside the fixed deadline, can produce the one evidence line.
    #[test]
    fn n087_03_every_incomplete_session_lifecycle_is_refused() {
        let mut killed = session_attempt();
        killed.parent_killed = true;
        killed.exit_code = None;
        assert_eq!(
            validate_session_attempt_v1(killed),
            Err(SmokeRefusalV1::HostExitAbnormal),
            "a status this parent produced is never evidence"
        );

        let mut unreaped = session_attempt();
        unreaped.reaped = false;
        assert_eq!(
            validate_session_attempt_v1(unreaped),
            Err(SmokeRefusalV1::HostReapFailed)
        );

        for code in [None, Some(1), Some(71), Some(77)] {
            let mut attempt = session_attempt();
            attempt.exit_code = code;
            assert_eq!(
                validate_session_attempt_v1(attempt),
                Err(SmokeRefusalV1::HostExitAbnormal),
                "only a normal zero exit is accepted: {code:?}"
            );
        }

        let mut at_deadline = session_attempt();
        at_deadline.wall_ms = SESSION_DEADLINE_MS;
        assert_eq!(
            validate_session_attempt_v1(at_deadline),
            Err(SmokeRefusalV1::HostWallTimeout),
            "the measurement must be strictly inside the deadline"
        );

        let mut missing = session_attempt();
        missing.stdout = None;
        assert_eq!(
            validate_session_attempt_v1(missing),
            Err(SmokeRefusalV1::HostOutputInvalid)
        );

        let mut non_utf8 = session_attempt();
        non_utf8.stdout = Some(vec![0xff, 0xfe, 0xfd]);
        assert_eq!(
            validate_session_attempt_v1(non_utf8),
            Err(SmokeRefusalV1::HostOutputInvalid)
        );

        let mut over_cap = session_attempt();
        over_cap.stdout = Some(vec![b'a'; 2_048]);
        assert_eq!(
            validate_session_attempt_v1(over_cap),
            Err(SmokeRefusalV1::HostOutputInvalid)
        );
    }

    /// `N087-03`: the one evidence line is fixed, bounded and never borrows an earlier proof's
    /// wording, so a live-child identity attestation can never be misread as a different result.
    #[test]
    fn n087_03_the_session_evidence_line_is_fixed_and_bounded() {
        let attestation =
            validate_session_attempt_v1(session_attempt()).expect("a complete attempt is evidence");
        let line = session_evidence_line(attestation);

        assert!(
            line.starts_with(SESSION_EVIDENCE_MARKER_V1),
            "the marker must state exactly what was proved: {line:?}"
        );
        assert!(line.ends_with('\n'));
        assert_eq!(
            line.matches('\n').count(),
            1,
            "exactly one evidence line, never a partial one first"
        );
        let expected = [
            "media_host_session=live_attested",
            "as_soft=536870912",
            "as_hard=536870912",
            "cpu_soft=10",
            "cpu_hard=10",
            "pid=4242",
            "pgid=4242",
            "sid=4242",
            "observed_pgid=4242",
            "observed_sid=4242",
            "reaped=1",
            "wall_ms=4050",
        ];
        assert_eq!(
            line.trim_end_matches('\n').split(' ').collect::<Vec<_>>(),
            expected.to_vec(),
            "the marker and the eleven fixed fields keep their exact order"
        );

        for foreign in [
            "media_host_probe=accepted",
            "media_host_wall=terminated",
            "media_host_as=limit_enforced",
            "media_host_cpu=kernel_terminated",
            "SOLO-MEDIA-HOST-PROBE-V1",
            "SOLO-MEDIA-HOST-WALL-V1",
            "SOLO-MEDIA-HOST-AS-V1",
            "SOLO-MEDIA-HOST-CPU-V1",
        ] {
            assert!(
                !line.contains(foreign),
                "the session record must not borrow {foreign}"
            );
        }
    }

    /// `N087-03`: the fixed 15 000 ms parent deadline and both fixed markers are the order's own
    /// values on **every** host, including the hosts that can only refuse.
    ///
    /// Read through a local so this stays a runtime comparison rather than a lintable constant
    /// assertion.
    #[test]
    fn n087_03_the_fixed_session_deadline_and_markers_are_the_ordered_values() {
        let deadline_ms = SESSION_DEADLINE_MS;
        assert_eq!(deadline_ms, 15_000, "the parent deadline is 15 000 ms");
        assert_eq!(SESSION_CHILD_READY_MARKER_V1, "SOLO-MEDIA-HOST-SESSION-V1");
        assert_eq!(
            SESSION_EVIDENCE_MARKER_V1,
            "media_host_session=live_attested"
        );
    }

    /// `N087-03`: on the only host that creates the child, the fixed window is exactly four
    /// seconds and closes strictly inside the parent's deadline.
    #[cfg(target_os = "linux")]
    #[test]
    fn n087_03_the_fixed_session_window_is_four_seconds_inside_the_deadline() {
        let window_ms = SESSION_CHILD_SLEEP_MS;
        assert_eq!(window_ms, 4_000, "the child sleeps exactly four seconds");
        assert!(
            window_ms < SESSION_DEADLINE_MS,
            "the child's window must close inside the parent's deadline"
        );
    }

    /// `N089-01`: the readiness gate is mandatory. An attempt whose parent never validated a
    /// complete readiness line is refused before its text is even parsed, so an attempt that lacks
    /// the ordering event can never be reported as evidence.
    #[test]
    fn n089_01_an_attempt_without_the_readiness_gate_is_refused() {
        // The flag alone authorizes the ordering claim: cleared, even a perfectly formed line and
        // an agreeing query cannot become evidence.
        let mut ungated = session_attempt();
        ungated.readiness_validated = false;
        assert_eq!(
            validate_session_attempt_v1(ungated),
            Err(SmokeRefusalV1::HostOutputInvalid)
        );

        // The gated line itself is likewise required, not merely implied by the flag.
        let mut lineless = session_attempt();
        lineless.readiness_line = None;
        assert_eq!(
            validate_session_attempt_v1(lineless),
            Err(SmokeRefusalV1::HostOutputInvalid)
        );
    }

    /// `N089-02`: the early prefix the parent gated on and the whole stream captured through EOF
    /// must be identical, so trailing or duplicated output discovered after the query still refuses.
    #[test]
    fn n089_02_the_gated_line_and_the_full_stream_must_agree_exactly() {
        let mut trailing = session_attempt();
        let mut with_trailer = WELL_FORMED_SESSION.as_bytes().to_vec();
        with_trailer.extend_from_slice(b"late\n");
        trailing.stdout = Some(with_trailer);
        assert_eq!(
            validate_session_attempt_v1(trailing),
            Err(SmokeRefusalV1::HostOutputInvalid)
        );

        let mut duplicated = session_attempt();
        let mut twice = WELL_FORMED_SESSION.as_bytes().to_vec();
        twice.extend_from_slice(WELL_FORMED_SESSION.as_bytes());
        duplicated.stdout = Some(twice);
        assert_eq!(
            validate_session_attempt_v1(duplicated),
            Err(SmokeRefusalV1::HostOutputInvalid)
        );

        // The exact single line stays accepted, so the two refusals above are meaningful.
        validate_session_attempt_v1(session_attempt()).expect("one exact line is evidence");
    }

    /// `N089-03`: readiness is accepted only for this parent's own positive handle pid, and every
    /// malformed, unterminated or duplicated line is the same closed refusal.
    #[test]
    fn n089_03_readiness_must_report_this_handles_own_pid() {
        let line = WELL_FORMED_SESSION.as_bytes();
        assert!(validate_session_readiness_v1(line, 4242).is_ok());
        assert_eq!(
            validate_session_readiness_v1(line, 4243),
            Err(SmokeRefusalV1::HostLimitUnavailable)
        );
        assert_eq!(
            validate_session_readiness_v1(line, 0),
            Err(SmokeRefusalV1::HostLimitUnavailable)
        );

        // A line without its newline was never complete, so it never granted readiness.
        let unterminated = WELL_FORMED_SESSION.trim_end_matches('\n');
        assert_eq!(
            validate_session_readiness_v1(unterminated.as_bytes(), 4242),
            Err(SmokeRefusalV1::HostOutputInvalid)
        );

        let doubled = format!("{WELL_FORMED_SESSION}{WELL_FORMED_SESSION}");
        assert_eq!(
            validate_session_readiness_v1(doubled.as_bytes(), 4242),
            Err(SmokeRefusalV1::HostOutputInvalid)
        );
        assert_eq!(
            validate_session_readiness_v1(&[0xFF_u8, b'\n'], 4242),
            Err(SmokeRefusalV1::HostOutputInvalid)
        );
    }

    /// `N089-04`: the readiness line is bounded by the same single total stdout cap as the whole
    /// stream, so an over-cap line is refused rather than truncated into a shorter success.
    #[test]
    fn n089_04_the_readiness_line_is_bounded_by_the_total_stdout_cap() {
        assert!(WELL_FORMED_SESSION.len() < MAX_CHILD_STDOUT_BYTES);
        let oversized = format!("{}\n", "a".repeat(MAX_CHILD_STDOUT_BYTES));
        assert!(oversized.len() > MAX_CHILD_STDOUT_BYTES);
        assert_eq!(
            validate_session_readiness_v1(oversized.as_bytes(), 4242),
            Err(SmokeRefusalV1::HostOutputInvalid)
        );
    }

    /// `N089-05`: the gate applies exactly the pair the final stream is judged by, so its refusal
    /// or acceptance can never diverge from the end-of-run validation.
    #[test]
    fn n089_05_the_gate_is_the_same_strict_pair_as_the_final_stream() {
        let parsed = parse_session_child_readiness_v1(WELL_FORMED_SESSION).expect("shape");
        assert_eq!(
            validate_session_readiness_v1(WELL_FORMED_SESSION.as_bytes(), 4242),
            validate_session_child_v1(parsed)
        );

        let off_profile = concat!(
            "SOLO-MEDIA-HOST-SESSION-V1 as_soft=1 as_hard=536870912 cpu_soft=10 ",
            "cpu_hard=10 pid=4242 pgid=4242 sid=4242\n"
        );
        let parsed_off = parse_session_child_readiness_v1(off_profile).expect("shape");
        assert_eq!(
            validate_session_readiness_v1(off_profile.as_bytes(), 4242),
            validate_session_child_v1(parsed_off)
        );
        assert_eq!(
            validate_session_readiness_v1(off_profile.as_bytes(), 4242),
            Err(SmokeRefusalV1::HostLimitUnavailable)
        );
    }

    /// `N090-01`: a confirmed reap keeps the original readiness refusal and is the only outcome that
    /// permits the reader-join phase, so cleanup success stays a refusal and never becomes evidence.
    #[test]
    fn n090_01_a_confirmed_reap_keeps_the_refusal_and_permits_joining() {
        let reap = SessionReapV1::Reaped;
        for refusal in [
            SmokeRefusalV1::HostOutputInvalid,
            SmokeRefusalV1::HostWallTimeout,
            SmokeRefusalV1::HostLimitUnavailable,
        ] {
            let plan = SessionCleanupPlanV1::classify_v1(true, reap, refusal);
            assert_eq!(plan.refusal_v1(), refusal);
            assert!(plan.joins_readers_v1(), "a confirmed reap permits joining");
        }
    }

    /// `N090-02`: a failed `Child::wait()` is a reap failure that overrides the readiness refusal
    /// which caused the cleanup, and it forbids the reader-join phase entirely.
    #[test]
    fn n090_02_a_failed_reap_overrides_the_readiness_refusal() {
        let error = std::io::Error::other("the leader could not be reaped");
        let reap = SessionReapV1::from_wait_outcome_v1(Err(error));
        assert_eq!(reap, SessionReapV1::Unreaped);
        for refusal in [
            SmokeRefusalV1::HostOutputInvalid,
            SmokeRefusalV1::HostWallTimeout,
            SmokeRefusalV1::HostLimitUnavailable,
        ] {
            let plan = SessionCleanupPlanV1::classify_v1(true, reap, refusal);
            assert_eq!(plan.refusal_v1(), SmokeRefusalV1::HostReapFailed);
            assert!(
                !plan.joins_readers_v1(),
                "an unverified reap forbids joining"
            );
        }
    }

    /// `N090-03`: an accepted or rejected `Child::kill()` cannot stand in for a reap. Only the
    /// checked wait outcome decides, so a kill result never upgrades a cleanup.
    #[test]
    fn n090_03_a_kill_result_cannot_stand_in_for_a_reap() {
        let refusal = SmokeRefusalV1::HostOutputInvalid;
        let unreaped = SessionReapV1::Unreaped;
        let reaped = SessionReapV1::Reaped;

        // A kill the kernel accepted is still not evidence that the leader exited.
        let lost = SessionCleanupPlanV1::classify_v1(true, unreaped, refusal);
        assert_eq!(lost.refusal_v1(), SmokeRefusalV1::HostReapFailed);
        assert!(!lost.joins_readers_v1());

        // A rejected kill is legitimate when the leader exited on its own: the reap still wins.
        let kept = SessionCleanupPlanV1::classify_v1(false, reaped, refusal);
        assert_eq!(kept.refusal_v1(), refusal);
        assert!(kept.joins_readers_v1());
    }

    /// `N090-04`: the production rule is a total truth table over every early refusal and both reap
    /// outcomes, so no combination can report a cleanup that was not established.
    #[test]
    fn n090_04_the_cleanup_rule_is_total_over_every_refusal() {
        for refusal in [
            SmokeRefusalV1::HostOutputInvalid,
            SmokeRefusalV1::HostWallTimeout,
            SmokeRefusalV1::HostLimitUnavailable,
        ] {
            for kill_ok in [true, false] {
                for reap in [SessionReapV1::Reaped, SessionReapV1::Unreaped] {
                    let plan = SessionCleanupPlanV1::classify_v1(kill_ok, reap, refusal);
                    let expected = match reap {
                        SessionReapV1::Reaped => refusal,
                        SessionReapV1::Unreaped => SmokeRefusalV1::HostReapFailed,
                    };
                    assert_eq!(plan.refusal_v1(), expected);
                    assert_eq!(plan.joins_readers_v1(), reap == SessionReapV1::Reaped);
                }
            }
        }
    }

    /// `N090-05`: a reader that failed to capture the stream still refuses on the accepted path, and
    /// no early-refusal plan can be mistaken for acceptance.
    #[test]
    fn n090_05_a_reader_failure_cannot_become_success() {
        let mut unreadable = session_attempt();
        unreadable.stdout = None;
        assert_eq!(
            validate_session_attempt_v1(unreadable),
            Err(SmokeRefusalV1::HostOutputInvalid)
        );

        let reap = SessionReapV1::Reaped;
        let refusal = SmokeRefusalV1::HostOutputInvalid;
        let plan = SessionCleanupPlanV1::classify_v1(true, reap, refusal);
        assert!(plan.joins_readers_v1());
        assert_eq!(plan.refusal_v1(), refusal);
    }

    /// A complete, internally consistent group attempt with every accepted property set.
    ///
    /// It is a **pure fixture**, never a kernel result: `N093-05` decides the acceptance rule from it
    /// and describes nothing as a real OS observation.
    fn group_attempt() -> GroupAttemptV1 {
        GroupAttemptV1 {
            leader_pid: 4712,
            descendant_pid: 4715,
            leader_pgid: 4712,
            leader_sid: 4712,
            descendant_pgid: 4712,
            descendant_sid: 4712,
            live_before: true,
            live_after: true,
            group_signal_issued: true,
            leader_signal: Some(GROUP_TERMINATION_SIGNAL_V1),
            leader_reaped: true,
            descendant_start_before: 987_654,
            descendant_start_after: 987_654,
            descendant_nonrunning: true,
            wall_ms: 731,
        }
    }

    /// `N093-05`: only the fully proved shape is accepted, and the accepted value names the group,
    /// the session and the one signal without ever offering a descendant-reap field.
    #[test]
    fn n093_05_the_fully_proved_group_attempt_is_the_only_accepted_shape() {
        let termination =
            validate_group_attempt_v1(group_attempt()).expect("the fixture is proved");
        assert_eq!(termination.leader_pid, 4712);
        assert_eq!(termination.descendant_pid, 4715);
        assert_eq!(termination.group, 4712);
        assert_eq!(termination.session, 4712);
        assert_eq!(termination.signal, GROUP_TERMINATION_SIGNAL_V1);
        assert_eq!(termination.wall_ms, 731);
    }

    /// `N093-05`: a leader that is not its own group/session leader, a descendant equal to the
    /// leader, a descendant outside the leader's group, and a zero identity all refuse.
    #[test]
    fn n093_05_every_wrong_group_or_descendant_identity_refuses() {
        let cases = [
            GroupAttemptV1 {
                leader_pgid: 4713,
                ..group_attempt()
            },
            GroupAttemptV1 {
                leader_sid: 4713,
                ..group_attempt()
            },
            GroupAttemptV1 {
                descendant_pid: 4712,
                ..group_attempt()
            },
            GroupAttemptV1 {
                descendant_pid: 0,
                ..group_attempt()
            },
            GroupAttemptV1 {
                descendant_pgid: 4716,
                ..group_attempt()
            },
            GroupAttemptV1 {
                descendant_sid: 4716,
                ..group_attempt()
            },
            GroupAttemptV1 {
                leader_pid: 1,
                ..group_attempt()
            },
        ];
        for case in cases {
            assert_eq!(
                validate_group_attempt_v1(case),
                Err(SmokeRefusalV1::HostLimitUnavailable),
                "an unproved identity must refuse: {case:?}"
            );
        }
    }

    /// `N093-05`: a missing signal request, a missing leader reap, a leader not proved live, an
    /// abnormal or absent signal status, an unobserved descendant, a recycled start-time identity and
    /// an over-deadline measurement each refuse with their own closed token.
    #[test]
    fn n093_05_signal_reap_observation_and_deadline_facts_stay_distinct() {
        let cases = [
            (
                GroupAttemptV1 {
                    group_signal_issued: false,
                    ..group_attempt()
                },
                SmokeRefusalV1::HostLimitUnavailable,
            ),
            (
                GroupAttemptV1 {
                    leader_reaped: false,
                    ..group_attempt()
                },
                SmokeRefusalV1::HostReapFailed,
            ),
            (
                GroupAttemptV1 {
                    live_before: false,
                    ..group_attempt()
                },
                SmokeRefusalV1::HostExitAbnormal,
            ),
            (
                GroupAttemptV1 {
                    live_after: false,
                    ..group_attempt()
                },
                SmokeRefusalV1::HostExitAbnormal,
            ),
            (
                GroupAttemptV1 {
                    leader_signal: None,
                    ..group_attempt()
                },
                SmokeRefusalV1::HostExitAbnormal,
            ),
            (
                GroupAttemptV1 {
                    leader_signal: Some(15),
                    ..group_attempt()
                },
                SmokeRefusalV1::HostExitAbnormal,
            ),
            (
                GroupAttemptV1 {
                    descendant_nonrunning: false,
                    ..group_attempt()
                },
                SmokeRefusalV1::HostDescendantUnverified,
            ),
            (
                GroupAttemptV1 {
                    descendant_start_before: 0,
                    ..group_attempt()
                },
                SmokeRefusalV1::HostDescendantUnverified,
            ),
            (
                GroupAttemptV1 {
                    descendant_start_after: 987_655,
                    ..group_attempt()
                },
                SmokeRefusalV1::HostDescendantUnverified,
            ),
            (
                GroupAttemptV1 {
                    wall_ms: 0,
                    ..group_attempt()
                },
                SmokeRefusalV1::HostWallTimeout,
            ),
            (
                GroupAttemptV1 {
                    wall_ms: GROUP_DEADLINE_MS,
                    ..group_attempt()
                },
                SmokeRefusalV1::HostWallTimeout,
            ),
        ];
        for (case, expected) in cases {
            assert_eq!(
                validate_group_attempt_v1(case),
                Err(expected),
                "this fact must refuse as {expected:?}: {case:?}"
            );
        }
    }

    /// `N093-05`: both group records are closed schemas — duplicated, extra, missing and reordered
    /// fields, a second line and a carriage return are all refused, and an identity disagreement is
    /// refused as an identity refusal rather than as a shape refusal.
    #[test]
    fn n093_05_both_group_records_are_closed_schemas() {
        let good = concat!(
            "SOLO-MEDIA-HOST-GROUP-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 ",
            "pid=4712 pgid=4712 sid=4712 descendant_pid=4715 descendant_pgid=4712 ",
            "descendant_sid=4712\n"
        );
        let parsed = parse_group_readiness_v1(good).expect("the fixed record parses");
        assert_eq!(parsed.leader_pid, 4712);
        assert_eq!(parsed.descendant_pid, 4715);

        let duplicated = good.replace("descendant_pid=4715", "descendant_pid=4715 pid=4712");
        let reordered = concat!(
            "SOLO-MEDIA-HOST-GROUP-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 ",
            "pgid=4712 pid=4712 sid=4712 descendant_pid=4715 descendant_pgid=4712 ",
            "descendant_sid=4712\n"
        );
        let extra = good.replace('\n', " extra=1\n");
        let missing = good.replace(" descendant_pid=4715", "");
        let two_lines = format!("{good}{good}");
        let carriage = good.replace('\n', "\r\n");
        for bad in [
            duplicated,
            reordered.to_owned(),
            extra,
            missing,
            two_lines,
            carriage,
        ] {
            assert_eq!(
                parse_group_readiness_v1(&bad),
                Err(SmokeRefusalV1::HostOutputInvalid),
                "an off-schema record must refuse: {bad:?}"
            );
        }

        let descendant = "SOLO-MEDIA-HOST-GROUP-DESC-V1 pid=4715 pgid=4712 sid=4712\n";
        let parsed =
            parse_group_descendant_readiness_v1(descendant).expect("the fixed record parses");
        assert_eq!(parsed.pid, 4715);
        assert_eq!(parsed.pgid, 4712);
        for bad in [
            "SOLO-MEDIA-HOST-GROUP-V1 pid=4715 pgid=4712 sid=4712\n",
            "SOLO-MEDIA-HOST-GROUP-DESC-V1 pid=4715 pgid=4712\n",
            "SOLO-MEDIA-HOST-GROUP-DESC-V1 pid=4715 pgid=4712 sid=4712",
        ] {
            assert_eq!(
                parse_group_descendant_readiness_v1(bad),
                Err(SmokeRefusalV1::HostOutputInvalid),
                "an off-schema descendant record must refuse: {bad:?}"
            );
        }

        assert_eq!(
            validate_group_readiness_v1(good.as_bytes(), 4713),
            Err(SmokeRefusalV1::HostLimitUnavailable)
        );
        assert_eq!(
            validate_group_descendant_readiness_v1(descendant.as_bytes(), 4715, 4713),
            Err(SmokeRefusalV1::HostLimitUnavailable)
        );
    }

    /// `N093-07`: the **same** classification helper production uses immediately after
    /// `child.wait()` turns a failed reap into `HOST_REAP_FAILED`.
    ///
    /// This is a pure classification test of the real production rule: it injects an `io::Error` into
    /// the helper rather than faking a kernel wait failure or altering `std::process::Child`, and it
    /// proves the negative only. The successful path is proved by the real Linux group case.
    #[cfg(target_os = "linux")]
    #[test]
    fn n093_07_a_failed_wait_classifies_as_a_reap_refusal() {
        let failed = group_wait_signal_v1(Err(std::io::Error::other("reap failed")));
        assert_eq!(
            failed,
            Err(SmokeRefusalV1::HostReapFailed),
            "a failed wait must never yield a signal status"
        );
        assert_eq!(
            SmokeRefusalV1::HostReapFailed.code(),
            "HOST_REAP_FAILED",
            "the classified refusal keeps its stable public token"
        );
    }

    /// `SOLO-WO-111 R01` — the closed `SOLO-N102-EVENT-V1` answer is exact, never permissive: a
    /// wrong protocol, a missing or reordered field and any trailing field are all refused, so a
    /// partially matching child line can never be read as a successful probe.
    #[test]
    fn n102_event_answer_refuses_wrong_protocol_and_missing_fields() {
        assert_eq!(
            super::parse_n102_event_answer_v1(
                "SOLO-N102-EVENT-V1 attempted=1 witness=1 pid=4242\n"
            ),
            Ok(super::N102EventProbeV1 {
                attempted: true,
                witness: true,
                pid: 4242,
            })
        );
        for bad in [
            "",
            "SOLO-N102-HANDLE-V1 attempted=1 witness=1 pid=4242\n",
            "SOLO-N102-ENV-V1 attempted=1 witness=1 pid=4242\n",
            "SOLO-N102-EVENT-V1 witness=1 pid=4242\n",
            "SOLO-N102-EVENT-V1 attempted=1 pid=4242\n",
            "SOLO-N102-EVENT-V1 attempted=1 witness=1\n",
            "SOLO-N102-EVENT-V1 witness=1 attempted=1 pid=4242\n",
            "SOLO-N102-EVENT-V1 attempted=1 witness=1 witness=1 pid=4242\n",
            "SOLO-N102-EVENT-V1 attempted=1 witness=1 pid=4242 extra=1\n",
            "SOLO-N102-EVENT-V1 attempted=1 witness=1 pid=4242\nunexpected-second-line\n",
            "SOLO-N102-EVENT-V1 attempted=1 witness=1 pid=4242\n\n",
            "SOLO-N102-EVENT-V1 attempted=1 witness=1 pid=4242\r\n\r\n",
            "SOLO-N102-EVENT-V1 attempted=1 witness=1 pid=4242\nEXTRA\n",
        ] {
            assert_eq!(
                super::parse_n102_event_answer_v1(bad),
                Err(super::N102ProbeRefusalV1::AnswerInvalid),
                "accepted an off-protocol event answer: {bad:?}"
            );
        }
    }

    /// The same answer refuses a non-canonical boolean, a signed, zero or over-wide pid, and a
    /// non-ASCII line: every one of them would otherwise let an unusable probe look answered.
    #[test]
    fn n102_event_answer_refuses_noncanonical_values() {
        for bad in [
            "SOLO-N102-EVENT-V1 attempted=2 witness=1 pid=4242\n",
            "SOLO-N102-EVENT-V1 attempted=1 witness=true pid=4242\n",
            "SOLO-N102-EVENT-V1 attempted=1 witness=1 pid=+4242\n",
            "SOLO-N102-EVENT-V1 attempted=1 witness=1 pid=0\n",
            "SOLO-N102-EVENT-V1 attempted=1 witness=1 pid=99999999999\n",
            "SOLO-N102-EVENT-V1 attempted=1 witness=1 pid=42 42\n",
            "SOLO-N102-EVENT-V1 attempted=1 witness=1 pid=4é42\n",
        ] {
            assert_eq!(
                super::parse_n102_event_answer_v1(bad),
                Err(super::N102ProbeRefusalV1::AnswerInvalid),
                "accepted a non-canonical event answer: {bad:?}"
            );
        }
    }

    /// One `ImageWorkerRunV1` fixture: the parent's own recorded facts are supplied independently of
    /// whatever the child wrote, exactly as the production supervisor reports them.
    fn n102_event_run_fixture(
        stdout: &str,
        child_pid: u32,
        exit_code: Option<i32>,
        limits_verified: bool,
        cleanup_verified: bool,
    ) -> super::ImageWorkerRunV1 {
        super::ImageWorkerRunV1 {
            stdout: stdout.as_bytes().to_vec(),
            stderr_len: 0,
            exit_code,
            wall_ms: 0,
            child_pid,
            limits_verified,
            cleanup_verified,
        }
    }

    /// `SOLO-WO-111 R01` REVISE — the run validator is the real function the production path calls,
    /// so these fixtures prove it accepts only mutually consistent evidence and refuses each defect
    /// independently: a child pid the parent never observed, an unproved confinement or reap, a
    /// non-zero or missing exit, a missing attempt/witness, and any non-single-record answer.
    #[test]
    fn n102_event_run_validation_refuses_every_inconsistent_observation() {
        const GOOD: &str = "SOLO-N102-EVENT-V1 attempted=1 witness=1 pid=4242\n";
        fn invalid() -> Result<super::N102EventProbeV1, super::N102ProbeRefusalV1> {
            Err(super::N102ProbeRefusalV1::AnswerInvalid)
        }
        assert_eq!(
            super::validate_n102_event_run_v1(&n102_event_run_fixture(
                GOOD,
                4242,
                Some(0),
                true,
                true
            )),
            Ok(super::N102EventProbeV1 {
                attempted: true,
                witness: true,
                pid: 4242,
            })
        );
        // The child claims a pid the parent's own handle never recorded.
        assert_eq!(
            super::validate_n102_event_run_v1(&n102_event_run_fixture(
                GOOD,
                4243,
                Some(0),
                true,
                true
            )),
            invalid()
        );
        // The parent recorded no child pid at all.
        assert_eq!(
            super::validate_n102_event_run_v1(&n102_event_run_fixture(
                GOOD,
                0,
                Some(0),
                true,
                true
            )),
            invalid()
        );
        // Confinement limits were never proved.
        assert_eq!(
            super::validate_n102_event_run_v1(&n102_event_run_fixture(
                GOOD,
                4242,
                Some(0),
                false,
                true
            )),
            invalid()
        );
        // The child was never proved reaped.
        assert_eq!(
            super::validate_n102_event_run_v1(&n102_event_run_fixture(
                GOOD,
                4242,
                Some(0),
                true,
                false
            )),
            invalid()
        );
        // A non-zero exit and a missing exit status are both refusals.
        assert_eq!(
            super::validate_n102_event_run_v1(&n102_event_run_fixture(
                GOOD,
                4242,
                Some(1),
                true,
                true
            )),
            invalid()
        );
        assert_eq!(
            super::validate_n102_event_run_v1(&n102_event_run_fixture(
                GOOD, 4242, None, true, true
            )),
            invalid()
        );
        // The child reported no real attempt or no working local witness.
        for stdout in [
            "SOLO-N102-EVENT-V1 attempted=0 witness=1 pid=4242\n",
            "SOLO-N102-EVENT-V1 attempted=1 witness=0 pid=4242\n",
            "SOLO-N102-EVENT-V1 attempted=0 witness=0 pid=4242\n",
        ] {
            assert_eq!(
                super::validate_n102_event_run_v1(&n102_event_run_fixture(
                    stdout,
                    4242,
                    Some(0),
                    true,
                    true
                )),
                invalid(),
                "accepted an answer with no real attempt or witness: {stdout:?}"
            );
        }
        // Repeated, appended, blank-second and unterminated records are all refused.
        for stdout in [
            "SOLO-N102-EVENT-V1 attempted=1 witness=1 pid=4242\nSOLO-N102-EVENT-V1 attempted=1 witness=1 pid=4242\n",
            "SOLO-N102-EVENT-V1 attempted=1 witness=1 pid=4242\nEXTRA\n",
            "SOLO-N102-EVENT-V1 attempted=1 witness=1 pid=4242\n\n",
            "SOLO-N102-EVENT-V1 attempted=1 witness=1 pid=4242",
        ] {
            assert_eq!(
                super::validate_n102_event_run_v1(&n102_event_run_fixture(
                    stdout,
                    4242,
                    Some(0),
                    true,
                    true
                )),
                invalid(),
                "accepted a non-single-record answer: {stdout:?}"
            );
        }
        // Bytes that are not UTF-8 can never be a bounded answer.
        assert_eq!(
            super::validate_n102_event_run_v1(&super::ImageWorkerRunV1 {
                stdout: vec![0xFF, 0xFE, b'\n'],
                stderr_len: 0,
                exit_code: Some(0),
                wall_ms: 0,
                child_pid: 4242,
                limits_verified: true,
                cleanup_verified: true,
            }),
            invalid()
        );
    }
}

#[cfg(windows)]
pub use crate::media_worker_host_native::{
    N102ExtraInheritableV1, n102_probe_environment_v1, n102_probe_event_v1, n102_probe_handle_v1,
};
// `SOLO-WO-104 R01` — the closed N102 probe seam.
//
// These helpers let the integration suite observe two real facts through the production confined
// spawn: which handles the child actually receives and which environment the child actually sees.
// They accept no command line, no arbitrary executable and no authored content, and they never
// print a handle value, a variable value or an environment block.

/// Why one closed N102 probe produced no observation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum N102ProbeRefusalV1 {
    /// The confined worker run itself refused, with its own exact kind preserved.
    Worker(ImageWorkerRunRefusalV1),
    /// The child's bounded answer was absent, over-long or off-schema.
    AnswerInvalid,
}

/// The bounded answer of one closed handle-isolation probe.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct N102HandleProbeV1 {
    /// Whether the child's own handle table resolved the parent's extra handle value.
    pub handle_live: bool,
    /// Whether the guaranteed-invalid control also resolved, which makes the answer unusable.
    pub control_live: bool,
    /// The child's own process identifier.
    pub pid: u32,
}

/// The bounded answer of one closed environment-isolation probe.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct N102EnvironmentProbeV1 {
    /// Whether the parent-only canary variable reached the child.
    pub canary_present: bool,
    /// Whether the child still has the system variable production installs.
    pub system_root_present: bool,
    /// The child's own process identifier.
    pub pid: u32,
}

/// One fixed probe byte, carried only so the supervisor takes its ordinary non-empty path: the
/// probe modes themselves never read source bytes.
const N102_PROBE_SOURCE_V1: [u8; 1] = [0x00];

/// The exact digest of that fixed probe byte.
const N102_PROBE_SOURCE_SHA256_V1: &str =
    "6e340b9cffb37a989ca544e6bb780a2c78901d3fb33738768511a30617afa01d";

/// The exact first token of the helper parent's own bounded answer line.
pub const N102_ENV_PARENT_PREFIX_V1: &str = "SOLO-N102-ENV-PARENT-V1";

/// The closed internal flag that makes one process the isolated parent of a real worker probe.
pub const INTERNAL_N102_ENV_PARENT_FLAG_V1: &str = "--internal-n102-env-parent";

fn n102_bounded_boolean_v1(line: &str, key: &str) -> Option<bool> {
    match line.split(' ').find_map(|part| part.strip_prefix(key)) {
        Some("0") => Some(false),
        Some("1") => Some(true),
        _ => None,
    }
}

/// Parses one bounded handle-isolation answer line.
pub fn parse_n102_handle_answer_v1(text: &str) -> Result<N102HandleProbeV1, N102ProbeRefusalV1> {
    let line = text.trim_end_matches(['\r', '\n']);
    if line.is_empty()
        || line.len() > crate::graphic_image_inspection::MAX_IMAGE_WORKER_FRAME_BYTES_V1
        || !line.is_ascii()
        || !line.starts_with(crate::graphic_image_inspection::N102_HANDLE_PROBE_PREFIX_V1)
    {
        return Err(N102ProbeRefusalV1::AnswerInvalid);
    }
    let handle_live =
        n102_bounded_boolean_v1(line, "live=").ok_or(N102ProbeRefusalV1::AnswerInvalid)?;
    let control_live =
        n102_bounded_boolean_v1(line, "control_live=").ok_or(N102ProbeRefusalV1::AnswerInvalid)?;
    let pid = line
        .split(' ')
        .find_map(|part| part.strip_prefix("pid="))
        .and_then(|value| value.parse::<u32>().ok())
        .ok_or(N102ProbeRefusalV1::AnswerInvalid)?;
    Ok(N102HandleProbeV1 {
        handle_live,
        control_live,
        pid,
    })
}

/// Parses one bounded environment-isolation answer line.
pub fn parse_n102_environment_answer_v1(
    text: &str,
) -> Result<N102EnvironmentProbeV1, N102ProbeRefusalV1> {
    let line = text.trim_end_matches(['\r', '\n']);
    if line.is_empty()
        || line.len() > crate::graphic_image_inspection::MAX_IMAGE_WORKER_FRAME_BYTES_V1
        || !line.is_ascii()
        || !line.starts_with(crate::graphic_image_inspection::N102_ENVIRONMENT_PROBE_PREFIX_V1)
    {
        return Err(N102ProbeRefusalV1::AnswerInvalid);
    }
    let canary_present =
        n102_bounded_boolean_v1(line, "canary=").ok_or(N102ProbeRefusalV1::AnswerInvalid)?;
    let system_root_present =
        n102_bounded_boolean_v1(line, "system_root=").ok_or(N102ProbeRefusalV1::AnswerInvalid)?;
    let pid = line
        .split(' ')
        .find_map(|part| part.strip_prefix("pid="))
        .and_then(|value| value.parse::<u32>().ok())
        .ok_or(N102ProbeRefusalV1::AnswerInvalid)?;
    Ok(N102EnvironmentProbeV1 {
        canary_present,
        system_root_present,
        pid,
    })
}

/// Runs one closed handle-isolation probe through the real production confined spawn.
pub fn run_n102_handle_probe_v1(
    executable: &std::path::Path,
    value: u32,
    control: u32,
) -> Result<N102HandleProbeV1, N102ProbeRefusalV1> {
    let token = format!(
        "{}:{value}:{control}",
        crate::graphic_image_inspection::IMAGE_WORKER_MODE_PROBE_HANDLE_V1
    );
    let run = run_confined_image_worker_mode_v1(
        executable,
        "png",
        N102_PROBE_SOURCE_SHA256_V1,
        Some(&token),
        &N102_PROBE_SOURCE_V1,
        &mut N102FaultRunV1::production(),
    )
    .map_err(N102ProbeRefusalV1::Worker)?;
    parse_n102_handle_answer_v1(&String::from_utf8_lossy(run.stdout()))
}

/// The bounded answer of one closed object-identity event probe.
///
/// It carries only booleans and a child pid. The child's attempt result is never decisive: the
/// caller decides inheritance by observing its own original event objects after the child is reaped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct N102EventProbeV1 {
    /// Whether the child really executed both `SetEvent` attempts.
    pub attempted: bool,
    /// Whether the child proved `SetEvent` plus a zero-timeout wait work on its own event.
    pub witness: bool,
    /// The child's own process identifier; never zero for a real run.
    pub pid: u32,
}

fn n102_probe_boolean_value_v1(text: &str) -> Option<bool> {
    match text {
        "0" => Some(false),
        "1" => Some(true),
        _ => None,
    }
}

fn n102_canonical_pid_v1(text: &str) -> Option<u32> {
    if text.is_empty() || text.len() > 10 || !text.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    match text.parse::<u32>().ok()? {
        0 => None,
        pid => Some(pid),
    }
}

/// Parses one bounded, exactly shaped event-probe answer line.
///
/// # Errors
///
/// [`N102ProbeRefusalV1::AnswerInvalid`] when the line is absent, off-protocol, over-long, carries a
/// repeated, reordered or unknown field, a non-canonical boolean or numeric value, or any trailing
/// data after the pid.
pub fn parse_n102_event_answer_v1(text: &str) -> Result<N102EventProbeV1, N102ProbeRefusalV1> {
    // Exactly one record: one trailing CRLF or LF is required and removed. A missing terminator,
    // two terminators or any second record is refused rather than trimmed away, because trimming
    // would let a duplicated or appended line look like the single bounded answer.
    let line = text
        .strip_suffix("\r\n")
        .or_else(|| text.strip_suffix('\n'))
        .ok_or(N102ProbeRefusalV1::AnswerInvalid)?;
    if line.is_empty()
        || line.len() > crate::graphic_image_inspection::MAX_IMAGE_WORKER_FRAME_BYTES_V1
        || !line.is_ascii()
        || line.contains(['\r', '\n'])
    {
        return Err(N102ProbeRefusalV1::AnswerInvalid);
    }
    let mut parts = line.split(' ');
    if parts.next() != Some(crate::graphic_image_inspection::N102_EVENT_PROBE_PREFIX_V1) {
        return Err(N102ProbeRefusalV1::AnswerInvalid);
    }
    let attempted = parts
        .next()
        .and_then(|part| part.strip_prefix("attempted="))
        .and_then(n102_probe_boolean_value_v1)
        .ok_or(N102ProbeRefusalV1::AnswerInvalid)?;
    let witness = parts
        .next()
        .and_then(|part| part.strip_prefix("witness="))
        .and_then(n102_probe_boolean_value_v1)
        .ok_or(N102ProbeRefusalV1::AnswerInvalid)?;
    let pid = parts
        .next()
        .and_then(|part| part.strip_prefix("pid="))
        .and_then(n102_canonical_pid_v1)
        .ok_or(N102ProbeRefusalV1::AnswerInvalid)?;
    if parts.next().is_some() {
        return Err(N102ProbeRefusalV1::AnswerInvalid);
    }
    Ok(N102EventProbeV1 {
        attempted,
        witness,
        pid,
    })
}

/// Validates one finished supervised event-probe run against the parent's own evidence.
///
/// The child's own line is never authoritative about identity: it is accepted only when the parent's
/// independently recorded child pid, normal exit, proved limits and proved reap all agree with it and
/// with each other. Every disagreement is a closed refusal, never a partial or best-effort answer.
fn validate_n102_event_run_v1(
    run: &ImageWorkerRunV1,
) -> Result<N102EventProbeV1, N102ProbeRefusalV1> {
    let text = std::str::from_utf8(run.stdout()).map_err(|_| N102ProbeRefusalV1::AnswerInvalid)?;

    let answer = parse_n102_event_answer_v1(text)?;

    if run.child_pid() == 0
        || answer.pid != run.child_pid()
        || run.exit_code() != Some(0)
        || !run.limits_verified()
        || !run.cleanup_verified()
        || !answer.attempted
        || !answer.witness
    {
        return Err(N102ProbeRefusalV1::AnswerInvalid);
    }

    Ok(answer)
}

/// Runs one closed object-identity event probe through the real production confined spawn.
///
/// # Errors
///
/// [`N102ProbeRefusalV1::Worker`] when the confined run itself refuses, or
/// [`N102ProbeRefusalV1::AnswerInvalid`] when the child's bounded answer is unusable.
pub fn run_n102_event_probe_v1(
    executable: &std::path::Path,
    candidate: u32,
    control: u32,
) -> Result<N102EventProbeV1, N102ProbeRefusalV1> {
    let token = format!(
        "{}:{candidate}:{control}",
        crate::graphic_image_inspection::IMAGE_WORKER_MODE_PROBE_EVENT_V1
    );
    let run = run_confined_image_worker_mode_v1(
        executable,
        "png",
        N102_PROBE_SOURCE_SHA256_V1,
        Some(&token),
        &N102_PROBE_SOURCE_V1,
        &mut N102FaultRunV1::production(),
    )
    .map_err(N102ProbeRefusalV1::Worker)?;
    validate_n102_event_run_v1(&run)
}

/// Runs one closed environment-isolation probe through the real production confined spawn.
pub fn run_n102_environment_probe_v1(
    executable: &std::path::Path,
    canary_name: &str,
) -> Result<N102EnvironmentProbeV1, N102ProbeRefusalV1> {
    let token = format!(
        "{}:{canary_name}",
        crate::graphic_image_inspection::IMAGE_WORKER_MODE_PROBE_ENVIRONMENT_V1
    );
    let run = run_confined_image_worker_mode_v1(
        executable,
        "png",
        N102_PROBE_SOURCE_SHA256_V1,
        Some(&token),
        &N102_PROBE_SOURCE_V1,
        &mut N102FaultRunV1::production(),
    )
    .map_err(N102ProbeRefusalV1::Worker)?;
    parse_n102_environment_answer_v1(&String::from_utf8_lossy(run.stdout()))
}

/// Runs the isolated helper parent when the reserved internal flag is present.
///
/// The helper is a separate process on purpose: the integration suite launches it with exactly one
/// parent-only canary variable through `Command::env`, and this process then starts the real worker
/// through the production minimal-environment spawn. Exit `0` reports that the canary was absent and
/// the system variable present, `61` that the canary reached the child, `62` that the system
/// variable was missing, and `63` that the helper or the worker run refused instead of observing.
pub fn run_internal_n102_environment_parent_v1(args: &[std::ffi::OsString]) -> Option<u8> {
    if args.get(1)? != std::ffi::OsStr::new(INTERNAL_N102_ENV_PARENT_FLAG_V1) {
        return None;
    }
    let Some(name) = args.get(2).and_then(|token| token.to_str()) else {
        return Some(63);
    };
    if args.len() > 3 || name.is_empty() || name.len() > 32 || !name.is_ascii() {
        return Some(63);
    }
    // The helper must itself carry the canary, otherwise its child observation proves nothing.
    if std::env::var_os(name).is_none() {
        return Some(63);
    }
    let Ok(own) = std::env::current_exe() else {
        return Some(63);
    };
    let Ok(answer) = run_n102_environment_probe_v1(&own, name) else {
        return Some(63);
    };
    let code = if answer.canary_present {
        61
    } else if !answer.system_root_present {
        62
    } else {
        0
    };
    // Only bounded booleans and the child's own identifier leave this process.
    println!(
        "{N102_ENV_PARENT_PREFIX_V1} canary_present={} system_root_present={} pid={} status={code}",
        u8::from(answer.canary_present),
        u8::from(answer.system_root_present),
        answer.pid
    );
    Some(code)
}

/// The highest closed N102 fault step the seam can request.
pub const N102_LAST_FAULT_STEP_V1: u8 = 11;

/// One per-run N102 fault observation: the boundary this run requested, the boundary it actually
/// reached and whether the cleanup that followed a fault was observed to complete.
///
/// Every production image run holds [`Self::production`], which can never inject. A request can only
/// come from the closed integration seam, and the reached boundary is recorded by the native code at
/// the boundary itself, so an injected refusal stays distinguishable from a real kernel error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct N102FaultRunV1 {
    requested: Option<u8>,
    reached: Option<u8>,
    cleanup_verified: bool,
}

impl N102FaultRunV1 {
    /// The only context a production image run may hold.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            requested: None,
            reached: None,
            cleanup_verified: false,
        }
    }

    /// A run that must make exactly one closed step refuse; `0` and values above 11 are rejected.
    #[must_use]
    pub const fn requesting(step: u8) -> Option<Self> {
        if step == 0 || step > N102_LAST_FAULT_STEP_V1 {
            return None;
        }
        Some(Self {
            requested: Some(step),
            reached: None,
            cleanup_verified: false,
        })
    }

    /// The step this run requested, if any.
    #[must_use]
    pub const fn requested(self) -> Option<u8> {
        self.requested
    }

    /// The boundary this run actually reached, recorded at the native boundary.
    #[must_use]
    pub const fn reached(self) -> Option<u8> {
        self.reached
    }

    /// Whether the cleanup that followed a fault was observed to complete.
    #[must_use]
    pub const fn cleanup_verified(self) -> bool {
        self.cleanup_verified
    }

    /// Records the native boundary reached and whether its cleanup was observed.
    #[cfg_attr(not(windows), allow(dead_code))]
    pub(crate) fn absorb(&mut self, reached: Option<u8>, cleanup_verified: bool) {
        if reached.is_some() {
            self.reached = reached;
        }
        self.cleanup_verified |= cleanup_verified;
    }
}

/// Runs one closed N102 fault attempt through the real production confined path.
///
/// Returns the per-run evidence and the refusal it produced. This is the only way an injected step
/// can be selected; it is reachable from the integration suite alone and never changes how a
/// production worker starts.
pub fn run_n102_fault_attempt_v1(
    executable: &std::path::Path,
    step: u8,
) -> (
    N102FaultRunV1,
    Result<N102HandleProbeV1, N102ProbeRefusalV1>,
) {
    let Some(mut fault) = N102FaultRunV1::requesting(step) else {
        return (
            N102FaultRunV1::production(),
            Err(N102ProbeRefusalV1::AnswerInvalid),
        );
    };
    let token = format!(
        "{}:0:4294967280",
        crate::graphic_image_inspection::IMAGE_WORKER_MODE_PROBE_HANDLE_V1
    );
    let outcome = run_confined_image_worker_mode_v1(
        executable,
        "png",
        N102_PROBE_SOURCE_SHA256_V1,
        Some(&token),
        &N102_PROBE_SOURCE_V1,
        &mut fault,
    )
    .map_err(N102ProbeRefusalV1::Worker)
    .and_then(|run| parse_n102_handle_answer_v1(&String::from_utf8_lossy(run.stdout())));
    (fault, outcome)
}
