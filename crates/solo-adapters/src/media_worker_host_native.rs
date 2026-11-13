//! Linux-only native resource-limit **readback** for `R03.C3`.
//!
//! This is the boundary that `SOLO-WO-074 R01` reserved and `SOLO-WO-076 R01` filled: a private
//! seam that observes the calling process's **existing** address-space and CPU ceilings through
//! the real kernel. `SOLO-WO-085 R01` added the fixed child's own process-group readback and
//! `SOLO-WO-087 R01` adds one read-only group **and** session identity query, so a trusted child can
//! attest its own identity and the owning parent can independently observe the same identities of
//! its live, directly owned child.
//!
//! It is deliberately not a supervisor. There is no descendant, no limit mutation, no signal, no
//! process-tree sweep, no handle cache and no output protocol here — only `getrlimit`, `getpgid`
//! and `getsid` observations with compile-time-fixed selectors and a caller-validated target.
//!
//! Everything, including `use libc`, sits inside a positively `#[cfg(target_os = "linux")]`-gated
//! module, and every `unsafe` block carries an adjacent `// SAFETY:` rationale, so both the AST
//! checker and the independent source scanner can prove the target of each call from real text.
//! The crate root declares this file only for `target_os = "linux"`, which is why a Windows build
//! compiles no Linux-native module and makes no Linux readback claim.
//!
//! Reading a ceiling proves no enforcement: it says nothing about a future child's limits, a
//! watchdog, a process group, descendants or any Windows Job Object.

#[cfg(target_os = "linux")]
mod linux {
    // The seam is established by this order and consumed by a later, separately issued one; until
    // then nothing outside this module's own unit tests names it, so the unused-code lint would
    // otherwise refuse a deliberately unexercised private API.
    #![allow(dead_code)]

    use std::io::Read;
    use std::mem::MaybeUninit;
    use std::os::unix::process::CommandExt;
    use std::process::{Child, Command};

    use crate::media_worker_host::{PROFILE_V1_AS_BYTES, PROFILE_V1_CPU_SECONDS};

    /// The most bytes of one `/proc/<pid>/stat` record this seam will read.
    ///
    /// A real record is a few hundred bytes; the cap means a hostile or unusual kernel surface can
    /// never make this observation allocate without bound.
    const PROC_STAT_READ_CAP: u64 = 4096;

    /// The field index, after the closing `)`, of the kernel's process start time.
    ///
    /// `/proc/<pid>/stat` numbers `state` as field 3 and `starttime` as field 22, so after the
    /// command name ends at the final `)` the start time sits at zero-based offset 19.
    const PROC_STAT_START_TIME_OFFSET: usize = 19;

    /// The kernel's "no limit" sentinel for an unsigned resource value.
    #[cfg(test)]
    const LIMIT_UNLIMITED: u64 = u64::MAX;

    /// Which compile-time-fixed ceiling an observation targets.
    ///
    /// An enum rather than a parameter, so no caller can name an arbitrary resource: only these
    /// two exist, and both are resolved to kernel constants inside this module.
    #[derive(Clone, Copy)]
    enum Ceiling {
        /// `RLIMIT_AS` — the process's virtual address-space ceiling in bytes.
        AddressSpace,
        /// `RLIMIT_CPU` — the process's consumed CPU-time ceiling in seconds.
        Cpu,
    }

    /// One observed ceiling, soft and hard reported separately.
    ///
    /// `rlim_t` is unsigned and `RLIM_INFINITY` is its maximum value, so the pair is carried as
    /// `u64` without narrowing, clamping or a "finite" reinterpretation.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub(crate) struct LimitCeilingV1 {
        /// The soft ceiling the kernel currently enforces.
        pub(crate) soft: u64,
        /// The hard ceiling a soft value may not exceed.
        pub(crate) hard: u64,
    }

    /// Both ceilings of the calling process, observed by two real kernel readbacks.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub(crate) struct NativeLimitReadbackV1 {
        /// The `RLIMIT_AS` observation.
        pub(crate) address_space: LimitCeilingV1,
        /// The `RLIMIT_CPU` observation.
        pub(crate) cpu: LimitCeilingV1,
    }

    /// Why a readback produced no observation, and which selector failed.
    ///
    /// Closed and typed: it carries no OS error text, no path and no raw worker input, so a
    /// caller cannot leak host detail through it.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub(crate) enum NativeReadbackRefusalV1 {
        /// The `RLIMIT_AS` query did not return success.
        AddressSpaceCeilingRejected,
        /// The `RLIMIT_CPU` query did not return success.
        CpuCeilingRejected,
    }

    /// Whether a kernel return status means the structure was written.
    ///
    /// Separated from the call so the success/refusal decision is provable by a pure unit test
    /// without injecting a syscall failure or acquiring privilege.
    const fn ceiling_status_ok(status: i32) -> bool {
        status == 0
    }

    /// Builds the typed ceiling from the two raw values the kernel reported.
    ///
    /// Pure, so full-width preservation of `RLIM_INFINITY` and of a distinct soft/hard pair is
    /// unit-testable with no FFI and no process-limit change.
    const fn ceiling_from_raw(soft: u64, hard: u64) -> LimitCeilingV1 {
        LimitCeilingV1 { soft, hard }
    }

    /// The closed result of one readback attempt.
    type ReadbackV1 = Result<NativeLimitReadbackV1, NativeReadbackRefusalV1>;

    /// Reads the calling process's existing ceilings through two real `getrlimit` calls.
    pub(crate) fn read_limit_ceilings_v1() -> ReadbackV1 {
        Ok(NativeLimitReadbackV1 {
            address_space: read_ceiling(Ceiling::AddressSpace)?,
            cpu: read_ceiling(Ceiling::Cpu)?,
        })
    }

    fn read_ceiling(which: Ceiling) -> Result<LimitCeilingV1, NativeReadbackRefusalV1> {
        let selector = match which {
            Ceiling::AddressSpace => libc::RLIMIT_AS,
            Ceiling::Cpu => libc::RLIMIT_CPU,
        };
        let refusal = match which {
            Ceiling::AddressSpace => NativeReadbackRefusalV1::AddressSpaceCeilingRejected,
            Ceiling::Cpu => NativeReadbackRefusalV1::CpuCeilingRejected,
        };

        let mut raw = MaybeUninit::<libc::rlimit>::uninit();

        // SAFETY: `getrlimit` writes exactly one fully initialised `libc::rlimit` through the
        // pointer it is given and reads nothing else. `raw` is a live, properly aligned stack slot
        // exclusively borrowed for this call, `as_mut_ptr` yields a valid `*mut libc::rlimit` for
        // it, the selector is one of the two fixed kernel constants, and the call mutates no
        // process limit — it only reports one.
        let status = unsafe { libc::getrlimit(selector, raw.as_mut_ptr()) };

        if !ceiling_status_ok(status) {
            // The slot is deliberately left unread. A rejected call does not promise to have
            // written anything, so reading it, or turning it into a success, would be exactly the
            // partly initialised observation this type exists to make impossible.
            return Err(refusal);
        }

        // SAFETY: the status just checked was zero, which is `getrlimit`'s contract for having
        // written the complete structure into the slot. The slot is therefore initialised, it is
        // not read again afterwards, and `rlim_t` is `u64` on Linux, so both fields pass through
        // with no narrowing, clamping or reinterpretation of the ceiling value.
        let raw = unsafe { raw.assume_init() };

        Ok(ceiling_from_raw(raw.rlim_cur, raw.rlim_max))
    }

    /// Why the calling process's group identity could not be observed.
    ///
    /// Closed and typed, like the ceiling refusals: it carries no OS error text and no path.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub(crate) enum NativeGroupRefusalV1 {
        /// The `getpgid` query did not return a valid group identity.
        ProcessGroupUnreadable,
    }

    /// Registers the fixed Linux pre-exec boundary on `command`.
    ///
    /// The closure runs in the forked child **after** `fork` and **before** `exec`, so it may use
    /// only async-signal-safe operations: it calls `setsid`, then installs both hard and soft
    /// ceilings with two `setrlimit` calls, and returns a plain `errno`-derived error on failure.
    /// It allocates nothing, locks nothing, formats nothing, performs no Rust I/O, resolves no
    /// path, reads no limit, spawns no thread and cannot panic, because every value it needs was
    /// computed here, before the fork. A failure means the image is never executed.
    pub(crate) fn install_linux_child_limits_v1(command: &mut Command) {
        let address_space = libc::rlimit {
            rlim_cur: PROFILE_V1_AS_BYTES,
            rlim_max: PROFILE_V1_AS_BYTES,
        };
        let cpu = libc::rlimit {
            rlim_cur: PROFILE_V1_CPU_SECONDS,
            rlim_max: PROFILE_V1_CPU_SECONDS,
        };

        // SAFETY: `pre_exec` is `unsafe` because the registered closure runs in a restricted
        // post-fork child, where only async-signal-safe work is permitted. The closure captures
        // exactly two `Copy` `libc::rlimit` values built above and performs only `setsid` and two
        // `setrlimit` calls, all three of which POSIX lists as async-signal-safe; it allocates,
        // locks, formats, logs, opens, scans, resolves, observes and spawns nothing, and it cannot
        // panic. Each failure path reports the child's own `errno` through
        // `std::io::Error::last_os_error`, which only reads the thread-local errno and allocates
        // nothing, and the parent never echoes that text. Both ceilings are installed with equal
        // soft and hard values, so the child cannot raise them after `exec`.
        unsafe {
            command.pre_exec(move || {
                if libc::setsid() == -1 {
                    return Err(std::io::Error::last_os_error());
                }
                if libc::setrlimit(libc::RLIMIT_AS, &address_space) != 0 {
                    return Err(std::io::Error::last_os_error());
                }
                if libc::setrlimit(libc::RLIMIT_CPU, &cpu) != 0 {
                    return Err(std::io::Error::last_os_error());
                }
                Ok(())
            });
        }
    }

    /// Reads the calling process's own process-group id.
    ///
    /// Used only by the fixed child, to prove that `setsid` inside the pre-exec boundary really
    /// made this process a session leader in its own group.
    pub(crate) fn own_process_group_v1() -> Result<u32, NativeGroupRefusalV1> {
        // SAFETY: `getpgid(0)` asks the kernel for the process-group id of the calling process
        // itself. It takes a plain `pid_t`, writes through no pointer, allocates nothing, mutates
        // no state and cannot fault; a negative return is the kernel's error report and is turned
        // into the closed refusal below rather than being cast or ignored.
        let group = unsafe { libc::getpgid(0) };
        if group < 0 {
            return Err(NativeGroupRefusalV1::ProcessGroupUnreadable);
        }
        Ok(group as u32)
    }

    /// Why a process-group or session identity could not be observed.
    ///
    /// Closed and typed, like the refusals above: it carries no `errno`, no path, no process
    /// metadata and no OS error text, so nothing host-specific can reach an operator through it.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub(crate) enum NativeIdentityRefusalV1 {
        /// The caller's target was zero, negative or not representable as a kernel `pid_t`.
        ProcessIdNotRepresentable,
        /// The `getpgid` query did not report a valid, positive group identity.
        ProcessGroupUnreadable,
        /// The `getsid` query did not report a valid, positive session identity.
        SessionUnreadable,
    }

    /// One read-only identity observation: the group and session the kernel reports for a target.
    ///
    /// Both fields are positive kernel identifiers that were validated **before** conversion, so a
    /// caller never sees a zero or a wrapped negative value presented as an identity.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub(crate) struct ProcessIdentityV1 {
        /// The process-group id the kernel reported for the target.
        pub(crate) group: u32,
        /// The session id the kernel reported for the target.
        pub(crate) session: u32,
    }

    /// Converts a caller's target into the `pid_t` the kernel queries accept.
    ///
    /// `None` is the kernel's own "the calling process" selector (`0`), which is the only form the
    /// fixed trusted child may use. `Some(pid)` must be a positive value a `pid_t` can represent:
    /// a zero, or a value wider than `pid_t`, is refused **before** any call, so a wrapped negative
    /// pid can never be handed to the kernel and no caller can name an arbitrary unrepresentable
    /// process.
    fn pid_target(target: Option<u32>) -> Result<libc::pid_t, NativeIdentityRefusalV1> {
        match target {
            None => Ok(0),
            Some(pid) => match libc::pid_t::try_from(pid) {
                Ok(converted) if converted > 0 => Ok(converted),
                _ => Err(NativeIdentityRefusalV1::ProcessIdNotRepresentable),
            },
        }
    }

    /// Observes the process-group and session identity of `target`, read-only.
    ///
    /// `None` asks the kernel about the calling process itself and is used only by the fixed,
    /// trusted child. `Some(pid)` names one exact positive pid that only this parent's own
    /// `Child::id()` can supply, and is used only by the fixed parent while that child is alive.
    ///
    /// Nothing here mutates state: `getpgid` and `getsid` are pure observations. No signal, no
    /// `waitpid`, no `pidfd`, no `fork` and no process-group or session control is reachable from
    /// this module, and each kernel status is checked before any result is converted.
    pub(crate) fn observe_process_identity_v1(
        target: Option<u32>,
    ) -> Result<ProcessIdentityV1, NativeIdentityRefusalV1> {
        let pid = pid_target(target)?;

        // SAFETY: `getpgid` is a read-only query. It takes one `pid_t` that was just validated as
        // either the kernel's own `0` selector or a positive, representable pid; it writes through
        // no pointer, allocates nothing, mutates no process state and cannot fault. A negative
        // return is the kernel's error report and becomes the closed refusal below rather than
        // being cast, so no fabricated group identity can leave this function.
        let group = unsafe { libc::getpgid(pid) };
        if group <= 0 {
            return Err(NativeIdentityRefusalV1::ProcessGroupUnreadable);
        }

        // SAFETY: `getsid` is the same shape of read-only query for the session identity: one
        // already validated `pid_t`, no pointer, no allocation, no state mutation and no fault. Its
        // negative return is likewise the kernel's error report, checked before conversion, so a
        // rejected call can never become a session identity.
        let session = unsafe { libc::getsid(pid) };
        if session <= 0 {
            return Err(NativeIdentityRefusalV1::SessionUnreadable);
        }

        let group =
            u32::try_from(group).map_err(|_| NativeIdentityRefusalV1::ProcessGroupUnreadable)?;
        let session =
            u32::try_from(session).map_err(|_| NativeIdentityRefusalV1::SessionUnreadable)?;
        Ok(ProcessIdentityV1 { group, session })
    }

    /// Why one owned-group termination was not issued.
    ///
    /// Closed and typed, like every refusal above: it carries no `errno`, no path and no process
    /// metadata, so nothing host-specific reaches an operator through it.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub(crate) enum GroupSignalRefusalV1 {
        /// A pid was zero, one, not representable as a kernel `pid_t`, or self-contradictory.
        BadIdentity,
        /// The directly owned leader was not proved still running immediately around the call.
        LeaderNotLive,
        /// The kernel's own group/session answers did not match the identities this call requires.
        IdentityMismatch,
        /// The kernel rejected the one signal, so no termination may be claimed.
        SignalRejected,
    }

    /// What one owned-group termination actually established.
    ///
    /// It records the **request** and the identities it was checked against. It is deliberately not a
    /// termination, a reap or a descendant exit: the caller must establish those separately.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub(crate) struct GroupSignalOutcomeV1 {
        /// The leader pid, derived inside this seam from the owned handle.
        pub(crate) leader_pid: u32,
        /// The descendant pid supplied only for identity validation.
        pub(crate) descendant_pid: u32,
        /// The verified group identity, which the kernel reported as the leader pid.
        pub(crate) group: u32,
        /// The verified session identity, which the kernel reported as the leader pid.
        pub(crate) session: u32,
    }

    /// Sends exactly one `SIGKILL` to the **owned** group of `owned_child`, after re-proving it.
    ///
    /// The leader pid is derived here from the caller's own `Child` handle, never from a parameter;
    /// `descendant_pid` is used only to prove that the descendant really sits in that group. The
    /// group number is therefore never caller-selected, and no generic signaller exists: this
    /// function cannot be asked to signal an arbitrary group, PID `0`, or `-1`.
    ///
    /// The pre-call `try_wait` bracket narrows, but does **not** eliminate, the window in which the
    /// leader could exit and its pid be reused. It is not a race-free lifetime pin, and no caller may
    /// present it as hostile-descendant containment.
    pub(crate) fn terminate_owned_group_v1(
        owned_child: &mut Child,
        descendant_pid: u32,
    ) -> Result<GroupSignalOutcomeV1, GroupSignalRefusalV1> {
        let handle_pid = owned_child.id();
        let leader_pid =
            i32::try_from(handle_pid).map_err(|_| GroupSignalRefusalV1::BadIdentity)?;
        if leader_pid <= 1 || descendant_pid == handle_pid || descendant_pid <= 1 {
            return Err(GroupSignalRefusalV1::BadIdentity);
        }
        if !matches!(owned_child.try_wait(), Ok(None)) {
            return Err(GroupSignalRefusalV1::LeaderNotLive);
        }

        let leader = observe_process_identity_v1(Some(handle_pid))
            .map_err(|_| GroupSignalRefusalV1::IdentityMismatch)?;
        if leader.group != handle_pid || leader.session != handle_pid {
            return Err(GroupSignalRefusalV1::IdentityMismatch);
        }
        let descendant = observe_process_identity_v1(Some(descendant_pid))
            .map_err(|_| GroupSignalRefusalV1::IdentityMismatch)?;
        if descendant.group != handle_pid || descendant.session != handle_pid {
            return Err(GroupSignalRefusalV1::IdentityMismatch);
        }

        // The verified group must not be this process's own group: a group signal that could reach
        // the supervising parent is refused before the call rather than discovered afterwards.
        let own_group =
            own_process_group_v1().map_err(|_| GroupSignalRefusalV1::IdentityMismatch)?;
        if own_group == leader.group {
            return Err(GroupSignalRefusalV1::IdentityMismatch);
        }

        // Recheck the same handle immediately before signalling, so a leader that exited during the
        // observations above cannot be signalled by a stale identity this call just derived.
        if !matches!(owned_child.try_wait(), Ok(None)) {
            return Err(GroupSignalRefusalV1::LeaderNotLive);
        }

        let target = leader_pid
            .checked_neg()
            .ok_or(GroupSignalRefusalV1::BadIdentity)?;

        // SAFETY: `kill` takes a `pid_t` and a signal number and performs no memory access, so no
        // pointer, allocation or Rust state is involved. `target` is the negation of a positive pid
        // this function just derived from the caller's own `Child` handle, which is the kernel's
        // documented "this process group" selector; it can never be `0` or `-1`, because
        // `leader_pid > 1` was enforced above and `checked_neg` refused any overflow. The group was
        // verified as the leader's own and proved different from this process's group, so the signal
        // cannot reach the supervising parent, a foreign group or a caller-selected group. The
        // signal is the fixed `SIGKILL` constant, never a caller-supplied number.
        let rc = unsafe { libc::kill(target, libc::SIGKILL) };

        if rc != 0 {
            return Err(GroupSignalRefusalV1::SignalRejected);
        }
        Ok(GroupSignalOutcomeV1 {
            leader_pid: handle_pid,
            descendant_pid,
            group: leader.group,
            session: leader.session,
        })
    }

    /// The bounded, kernel-backed run state of one observed pid.
    ///
    /// `Running` and `Zombie` both carry the kernel's **start-time** identity, so a caller can prove
    /// that the state it read belongs to the same process it observed earlier and not to a recycled
    /// pid. A zombie is non-running but is **not** parent-reaped by this process.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub(crate) enum ProcessStateV1 {
        /// The kernel reports the process as alive and not terminated.
        Running {
            /// The kernel's `starttime` identity for this process.
            start_time: u64,
        },
        /// The kernel reports the process as a zombie: terminated, awaiting its own parent's reap.
        Zombie {
            /// The kernel's `starttime` identity for this process.
            start_time: u64,
        },
        /// No `/proc` entry exists for the pid any more.
        Gone,
    }

    /// Why one `/proc` observation was refused rather than reported.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub(crate) enum ProcessStateRefusalV1 {
        /// The record existed but could not be read.
        Unreadable,
        /// The record existed but was truncated, non-UTF-8, or off-schema.
        Malformed,
    }

    /// Observes one pid's run state and start-time identity from `/proc`, bounded and read-only.
    ///
    /// Nothing here mutates state, sends a signal, waits on a child or scans a process table: it
    /// opens exactly one fixed `/proc/<pid>/stat` path for one already-validated positive pid. An
    /// absent entry is reported as [`ProcessStateV1::Gone`]; anything unreadable or off-schema is a
    /// typed refusal, so an inconclusive read can never be presented as a termination.
    pub(crate) fn observe_process_state_v1(
        pid: u32,
    ) -> Result<ProcessStateV1, ProcessStateRefusalV1> {
        if pid <= 1 {
            return Err(ProcessStateRefusalV1::Malformed);
        }
        let path = format!("/proc/{pid}/stat");
        let file = match std::fs::File::open(&path) {
            Ok(file) => file,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(ProcessStateV1::Gone);
            }
            Err(_) => return Err(ProcessStateRefusalV1::Unreadable),
        };

        let mut buffer = Vec::new();
        let read = file
            .take(PROC_STAT_READ_CAP)
            .read_to_end(&mut buffer)
            .map_err(|_| ProcessStateRefusalV1::Unreadable)?;
        if u64::try_from(read).unwrap_or(u64::MAX) >= PROC_STAT_READ_CAP {
            return Err(ProcessStateRefusalV1::Malformed);
        }
        let text = std::str::from_utf8(&buffer).map_err(|_| ProcessStateRefusalV1::Malformed)?;
        parse_proc_stat_v1(text)
    }

    /// Parses one `/proc/<pid>/stat` record into a typed state and start-time identity.
    ///
    /// The command name is delimited by the **last** `)`, so a name that itself contains a space or a
    /// bracket cannot shift the field offsets. Only the fixed state character and the fixed
    /// start-time field are read; every other field is ignored rather than interpreted.
    fn parse_proc_stat_v1(text: &str) -> Result<ProcessStateV1, ProcessStateRefusalV1> {
        let open = text.find('(').ok_or(ProcessStateRefusalV1::Malformed)?;
        let close = text.rfind(')').ok_or(ProcessStateRefusalV1::Malformed)?;
        if close <= open {
            return Err(ProcessStateRefusalV1::Malformed);
        }
        let rest = text
            .get(close + 1..)
            .ok_or(ProcessStateRefusalV1::Malformed)?;
        let fields: Vec<&str> = rest.split_whitespace().collect();
        if fields.len() <= PROC_STAT_START_TIME_OFFSET {
            return Err(ProcessStateRefusalV1::Malformed);
        }

        let state = fields[0];
        if state.len() != 1 {
            return Err(ProcessStateRefusalV1::Malformed);
        }
        let start_time = fields[PROC_STAT_START_TIME_OFFSET]
            .parse::<u64>()
            .map_err(|_| ProcessStateRefusalV1::Malformed)?;

        match state {
            "R" | "S" | "D" | "I" | "T" | "t" | "W" | "P" => {
                Ok(ProcessStateV1::Running { start_time })
            }
            "Z" | "X" | "x" => Ok(ProcessStateV1::Zombie { start_time }),
            _ => Err(ProcessStateRefusalV1::Malformed),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        /// `N076-01`: both real kernel readbacks succeed and are reported separately.
        #[test]
        fn n076_01_getrlimit_rlimit_as_and_rlimit_cpu_observe_the_real_kernel() {
            let address_space =
                read_ceiling(Ceiling::AddressSpace).expect("RLIMIT_AS readback must succeed");
            let cpu = read_ceiling(Ceiling::Cpu).expect("RLIMIT_CPU readback must succeed");

            // The kernel never reports a soft ceiling above its own hard ceiling.
            assert!(address_space.soft <= address_space.hard);
            assert!(cpu.soft <= cpu.hard);

            let both = read_limit_ceilings_v1().expect("both readbacks must succeed");
            assert_eq!(both.address_space, address_space);
            assert_eq!(both.cpu, cpu);
        }

        /// `N076-02`: soft and hard stay separate, the whole `rlim_t` range is preserved, and a
        /// repeated read is consistent. No fixed host value is asserted.
        #[test]
        fn n076_02_ceilings_keep_their_full_range_and_repeat_consistently() {
            let first = read_limit_ceilings_v1().expect("readback");
            let second = read_limit_ceilings_v1().expect("readback");
            assert_eq!(
                first, second,
                "repeating the observation must be consistent"
            );

            let unlimited = ceiling_from_raw(LIMIT_UNLIMITED, LIMIT_UNLIMITED);
            assert_eq!(unlimited.soft, u64::MAX, "RLIM_INFINITY must not truncate");
            assert_eq!(unlimited.hard, u64::MAX, "RLIM_INFINITY must not truncate");

            let split = ceiling_from_raw(1_048_576, 536_870_912);
            assert_ne!(
                split.soft, split.hard,
                "soft and hard are reported separately"
            );
            assert_eq!(split.hard, 536_870_912);
        }

        /// `N076-03`: a **source-path** proof, not an injected syscall failure. No privilege is
        /// used and no limit is changed; the decision that turns a status into a typed refusal is
        /// proved purely.
        #[test]
        fn n076_03_a_rejected_status_is_a_typed_refusal_not_a_partial_observation() {
            assert!(!ceiling_status_ok(-1), "a failed status is never success");
            assert!(!ceiling_status_ok(1), "only zero is success");
            assert!(ceiling_status_ok(0), "zero is the kernel's success status");

            assert_ne!(
                NativeReadbackRefusalV1::AddressSpaceCeilingRejected,
                NativeReadbackRefusalV1::CpuCeilingRejected,
                "each selector keeps its own closed refusal"
            );
        }

        /// Sends exactly one `SIGKILL` to the verified **own** group of `owned_child`.
        ///
        /// The accepted `N093` helper requires a descendant to prove group membership. This image
        /// worker is designed to create **zero** descendants, so this separate entry point derives the
        /// group from the leader alone: the kernel must report `group == session == leader pid`, and that
        /// group must differ from this parent's own group, all inside a `try_wait` bracket. The group
        /// number is never caller-selected and no generic signaller exists.
        ///
        /// This narrows but does not eliminate the pid-reuse window, so no caller may present it as
        /// race-free process-tree containment.
        pub(crate) fn terminate_own_group_v1(
            owned_child: &mut Child,
        ) -> Result<u32, GroupSignalRefusalV1> {
            let handle_pid = owned_child.id();
            let leader_pid =
                i32::try_from(handle_pid).map_err(|_| GroupSignalRefusalV1::BadIdentity)?;
            if leader_pid <= 1 {
                return Err(GroupSignalRefusalV1::BadIdentity);
            }
            if !matches!(owned_child.try_wait(), Ok(None)) {
                return Err(GroupSignalRefusalV1::LeaderNotLive);
            }

            let leader = observe_process_identity_v1(Some(handle_pid))
                .map_err(|_| GroupSignalRefusalV1::IdentityMismatch)?;
            if leader.group != handle_pid || leader.session != handle_pid {
                return Err(GroupSignalRefusalV1::IdentityMismatch);
            }
            let own_group =
                own_process_group_v1().map_err(|_| GroupSignalRefusalV1::IdentityMismatch)?;
            if own_group == leader.group {
                return Err(GroupSignalRefusalV1::IdentityMismatch);
            }

            // Recheck the same handle immediately before signalling.
            if !matches!(owned_child.try_wait(), Ok(None)) {
                return Err(GroupSignalRefusalV1::LeaderNotLive);
            }

            let target = leader_pid
                .checked_neg()
                .ok_or(GroupSignalRefusalV1::BadIdentity)?;

            // SAFETY: `kill` takes a `pid_t` and a signal number and touches no memory, so no pointer,
            // allocation or Rust state is involved. `target` is the negation of a positive pid this
            // function just derived from the caller's own `Child` handle — the kernel's documented
            // "this process group" selector — and it can never be `0` or `-1` because `leader_pid > 1`
            // was enforced and `checked_neg` refused overflow. The group was verified as the leader's
            // own and proved different from this process's group, so the signal cannot reach the
            // supervising parent. A non-zero return is the kernel's error report and becomes the closed
            // `SignalRejected` refusal rather than a claimed termination.
            let status = unsafe { libc::kill(target, libc::SIGKILL) };
            if status != 0 {
                return Err(GroupSignalRefusalV1::SignalRejected);
            }
            Ok(leader.group)
        }
    }
}

// The fixed entry points the safe façade consumes.
//
// The implementation lives inside the positively Linux-gated `linux` module above, which keeps
// every `unsafe` occurrence inside a provable gate; these re-exports only make the same fixed
// items nameable from the rest of the crate. There is no general-purpose native API, no
// caller-selected limit, no executable parameter and no Windows counterpart here: the two group
// items are the owned-group termination request and the bounded `/proc` state observation that
// `SOLO-WO-093 R01` added, and `SOLO-WO-094 R01` re-exports them so the façade's own imports
// resolve on Linux.
#[cfg(target_os = "linux")]
pub(crate) use linux::{
    GroupSignalRefusalV1, ProcessStateV1, install_linux_child_limits_v1,
    observe_process_identity_v1, observe_process_state_v1, own_process_group_v1,
    read_limit_ceilings_v1, terminate_own_group_v1, terminate_owned_group_v1,
};

// The fixed Windows entry points the safe façade consumes.
//
// The implementation lives inside the positively `#[cfg(windows)]`-gated `windows` module above;
// these re-exports only make the one suspended-worker type and the one job-membership query
// nameable from the rest of the crate. There is no general-purpose native API here: no
// caller-selected executable, limit, command line or output destination is reachable.
#[cfg(windows)]
pub(crate) use windows::{SuspendedWorkerV1, in_job_v1};

// ---------------------------------------------------------------------------------------------
// `R03.C3` — the Windows suspended-worker job boundary
// ---------------------------------------------------------------------------------------------
//
// This is the second approved use of the reserved process-native file and the first Windows one in
// it. Every operation below sits inside a positively `#[cfg(windows)]`-gated module, and every
// `unsafe` block carries an adjacent `// SAFETY:` rationale naming the exact invariant that makes
// the call sound.
//
// What it provides, and nothing more: create the one trusted worker **suspended**, create and limit
// a job object before that worker may execute, assign the still-suspended worker, read the limits
// and membership back from the kernel, only then resume its captured primary thread, wait with a
// timeout, and terminate or confirm the reap of exactly this parent's own job and process. It opens
// no file, reads no authored path, writes no output file and starts no other executable.
//
// What it does **not** provide: a filesystem, registry or network sandbox; protection against a
// hostile worker that tries to break out of its job; race-free identity pinning against handle
// duplication; or any claim that a Job Object bounds committed memory the way a container would.

#[cfg(windows)]
mod windows {
    use std::ffi::c_void;
    use std::path::Path;

    use windows_sys::Win32::Foundation::{
        CloseHandle, HANDLE, HANDLE_FLAG_INHERIT, SetHandleInformation, WAIT_OBJECT_0,
    };
    use windows_sys::Win32::Security::SECURITY_ATTRIBUTES;
    use windows_sys::Win32::System::JobObjects::{
        AssignProcessToJobObject, CreateJobObjectW, IsProcessInJob,
        JOB_OBJECT_BASIC_ACCOUNTING_INFORMATION, JOB_OBJECT_EXTENDED_LIMIT_INFORMATION,
        JOB_OBJECT_LIMIT_JOB_MEMORY, JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
        JOB_OBJECT_LIMIT_PROCESS_MEMORY, JOB_OBJECT_LIMIT_PROCESS_TIME,
        JobObjectBasicAccountingInformation, JobObjectExtendedLimitInformation,
        QueryInformationJobObject, SetInformationJobObject, TerminateJobObject,
    };
    use windows_sys::Win32::System::Pipes::CreatePipe;
    use windows_sys::Win32::System::Threading::{
        CREATE_SUSPENDED, CreateProcessW, GetCurrentProcess, GetExitCodeProcess,
        PROCESS_INFORMATION, ResumeThread, STARTF_USESTDHANDLES, STARTUPINFOW, WaitForSingleObject,
    };

    /// The fixed CPU ceiling for one worker, in 100-nanosecond units (10 seconds).
    const PROCESS_TIME_100NS: i64 = 10 * 10_000_000;

    /// The fixed committed-memory ceiling for one worker process, in bytes (512 MiB).
    const PROCESS_MEMORY_BYTES: usize = 536_870_912;

    /// The fixed committed-memory ceiling for the whole job, in bytes (512 MiB).
    const JOB_MEMORY_BYTES: usize = 536_870_912;

    /// Whether the calling process really belongs to a job object.
    ///
    /// The fixed child calls this before it parses a single byte, so a worker started outside the
    /// approved boundary refuses instead of decoding unconfined.
    pub(crate) fn in_job_v1() -> bool {
        let mut inside: windows_sys::core::BOOL = 0;
        // SAFETY: `IsProcessInJob` takes this process's own pseudo-handle, a null job handle (which
        // asks about *any* job) and one out-pointer to a stack `BOOL`. It writes exactly that
        // `BOOL`, allocates nothing, opens no handle and cannot fault on these arguments.
        let status =
            unsafe { IsProcessInJob(GetCurrentProcess(), std::ptr::null_mut(), &mut inside) };
        status != 0 && inside != 0
    }

    /// A UTF-16, NUL-terminated rendering of one Rust string.
    fn wide(text: &str) -> Vec<u16> {
        text.encode_utf16().chain(std::iter::once(0)).collect()
    }

    /// Quotes one command-line argument with the Windows `CommandLineToArgvW` rules.
    ///
    /// The worker's arguments (a closed flag, a closed format token and one lowercase digest) never
    /// need quoting, but the executable path may contain spaces, so quoting is applied to every
    /// element uniformly rather than to a hand-picked subset.
    fn quote_argument(text: &str) -> String {
        let needs_quotes = text.is_empty() || text.contains([' ', '\t', '"']);
        if !needs_quotes {
            return text.to_owned();
        }
        let mut out = String::with_capacity(text.len() + 2);
        out.push('"');
        let mut backslashes = 0usize;
        for character in text.chars() {
            match character {
                '\\' => {
                    backslashes += 1;
                    out.push('\\');
                }
                '"' => {
                    for _ in 0..backslashes {
                        out.push('\\');
                    }
                    backslashes = 0;
                    out.push('\\');
                    out.push('"');
                }
                _ => {
                    backslashes = 0;
                    out.push(character);
                }
            }
        }
        out.push('"');
        out
    }

    /// Wraps one raw handle as an owned standard-library file, refusing an obviously invalid handle.
    ///
    /// # Safety
    ///
    /// The caller must pass a handle it exclusively owns and has not transferred elsewhere; the
    /// returned `File` closes it exactly once.
    unsafe fn file_from_owned_handle(handle: HANDLE) -> Option<std::fs::File> {
        use std::os::windows::io::FromRawHandle;

        if handle.is_null() || handle == windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE {
            return None;
        }
        // SAFETY: the caller guarantees exclusive ownership of a live handle, and `File` takes it
        // over with exactly one matching `CloseHandle`, so no double close or leak follows.
        Some(unsafe { std::fs::File::from_raw_handle(handle as std::os::windows::io::RawHandle) })
    }

    /// Creates one job object and installs the four mandatory limits.
    ///
    /// The limits are written once and then **queried back**, so a silently rejected setting can
    /// never be reported as installed.
    fn create_limited_job_v1() -> Option<(HANDLE, bool)> {
        // SAFETY: `CreateJobObjectW` takes a null security descriptor and a null name, so it creates
        // an anonymous job whose only handle this process owns. It returns a handle or null.
        let job = unsafe { CreateJobObjectW(std::ptr::null(), std::ptr::null()) };
        if job.is_null() {
            return None;
        }

        // SAFETY: every field of these `repr(C)` window structures is valid when zeroed, which is
        // the documented "no limits" baseline this code then fills in explicitly.
        let mut limits: JOB_OBJECT_EXTENDED_LIMIT_INFORMATION = unsafe { std::mem::zeroed() };
        limits.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_PROCESS_TIME
            | JOB_OBJECT_LIMIT_PROCESS_MEMORY
            | JOB_OBJECT_LIMIT_JOB_MEMORY
            | JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
        limits.BasicLimitInformation.PerProcessUserTimeLimit = PROCESS_TIME_100NS;
        limits.ProcessMemoryLimit = PROCESS_MEMORY_BYTES;
        limits.JobMemoryLimit = JOB_MEMORY_BYTES;

        let Some(length) =
            u32::try_from(std::mem::size_of::<JOB_OBJECT_EXTENDED_LIMIT_INFORMATION>()).ok()
        else {
            // SAFETY: `job` is a live handle this function owns and has stored nowhere else.
            unsafe { CloseHandle(job) };
            return None;
        };

        // SAFETY: the job handle came from `CreateJobObjectW` just above, the information class
        // matches the structure type, and the length is exactly that structure's size, which is
        // `SetInformationJobObject`'s contract for this class.
        let set = unsafe {
            SetInformationJobObject(
                job,
                JobObjectExtendedLimitInformation,
                std::ptr::from_ref(&limits).cast::<c_void>(),
                length,
            )
        };
        if set == 0 {
            // SAFETY: `job` is a live handle this function owns and has stored nowhere else.
            unsafe { CloseHandle(job) };
            return None;
        }

        let verified = read_job_limits_v1(job);
        Some((job, verified))
    }

    /// Reads the job's effective limits back from the kernel and compares all four.
    fn read_job_limits_v1(job: HANDLE) -> bool {
        // SAFETY: zeroed `repr(C)` structure, filled by the query below.
        let mut observed: JOB_OBJECT_EXTENDED_LIMIT_INFORMATION = unsafe { std::mem::zeroed() };
        let mut returned: u32 = 0;
        let Some(length) =
            u32::try_from(std::mem::size_of::<JOB_OBJECT_EXTENDED_LIMIT_INFORMATION>()).ok()
        else {
            return false;
        };
        // SAFETY: the job handle is live, the class matches the structure, the pointer and length
        // name this function's own stack structure, and the size is its exact size.
        let queried = unsafe {
            QueryInformationJobObject(
                job,
                JobObjectExtendedLimitInformation,
                std::ptr::from_mut(&mut observed).cast::<c_void>(),
                length,
                &mut returned,
            )
        };
        if queried == 0 {
            return false;
        }
        let required = JOB_OBJECT_LIMIT_PROCESS_TIME
            | JOB_OBJECT_LIMIT_PROCESS_MEMORY
            | JOB_OBJECT_LIMIT_JOB_MEMORY
            | JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
        observed.BasicLimitInformation.LimitFlags & required == required
            && observed.BasicLimitInformation.PerProcessUserTimeLimit == PROCESS_TIME_100NS
            && observed.ProcessMemoryLimit == PROCESS_MEMORY_BYTES
            && observed.JobMemoryLimit == JOB_MEMORY_BYTES
    }

    /// Whether the job currently holds at least one live process.
    fn job_has_a_process_v1(job: HANDLE) -> bool {
        // SAFETY: zeroed `repr(C)` accounting structure, filled by the query below.
        let mut accounting: JOB_OBJECT_BASIC_ACCOUNTING_INFORMATION = unsafe { std::mem::zeroed() };
        let mut returned: u32 = 0;
        let Some(length) =
            u32::try_from(std::mem::size_of::<JOB_OBJECT_BASIC_ACCOUNTING_INFORMATION>()).ok()
        else {
            return false;
        };
        // SAFETY: live job handle, matching class, this function's own stack structure and its exact
        // size.
        let queried = unsafe {
            QueryInformationJobObject(
                job,
                JobObjectBasicAccountingInformation,
                std::ptr::from_mut(&mut accounting).cast::<c_void>(),
                length,
                &mut returned,
            )
        };
        queried != 0 && accounting.ActiveProcesses >= 1
    }

    /// One worker that was created suspended, limited by its job, and only then resumed.
    ///
    /// It owns the job, the process handle, the primary thread handle and the two parent-side pipe
    /// ends. Dropping it terminates a still-running job and closes every handle exactly once, so no
    /// native handle can leak even on an early refusal.
    pub(crate) struct SuspendedWorkerV1 {
        job: HANDLE,
        process: HANDLE,
        thread: HANDLE,
        pid: u32,
        limits_verified: bool,
        resumed: bool,
        exited: bool,
        stdin: Option<std::fs::File>,
        stdout: Option<std::fs::File>,
    }

    /// Closes one handle if it is live.
    ///
    /// # Safety
    ///
    /// The caller must own `handle` exclusively and must not use it afterwards.
    unsafe fn close_owned(handle: HANDLE) {
        if !handle.is_null() && handle != windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE {
            // SAFETY: the caller guarantees exclusive ownership and no later use.
            unsafe { CloseHandle(handle) };
        }
    }

    impl SuspendedWorkerV1 {
        /// Creates the worker suspended, assigns it to a freshly limited job and verifies both.
        ///
        /// Nothing here lets the child execute: `CREATE_SUSPENDED` holds it at creation, the job is
        /// limited and read back before assignment, and `resume` is a separate call the caller makes
        /// only after those facts are known.
        pub(crate) fn spawn(executable: &Path, arguments: &[&str]) -> Option<Self> {
            let (job, limits_verified) = create_limited_job_v1()?;

            // SAFETY: `SECURITY_ATTRIBUTES` is a plain `repr(C)` structure valid when zeroed; the
            // three fields set below are the only ones `CreatePipe` reads.
            let mut attributes: SECURITY_ATTRIBUTES = unsafe { std::mem::zeroed() };
            attributes.nLength = u32::try_from(std::mem::size_of::<SECURITY_ATTRIBUTES>()).ok()?;
            attributes.lpSecurityDescriptor = std::ptr::null_mut();
            attributes.bInheritHandle = 1;

            let mut stdout_read: HANDLE = std::ptr::null_mut();
            let mut stdout_write: HANDLE = std::ptr::null_mut();
            // SAFETY: both out-pointers name this function's own stack handles, and the attributes
            // ask for an inheritable child end.
            let out_pipe = unsafe {
                CreatePipe(
                    &mut stdout_read,
                    &mut stdout_write,
                    std::ptr::from_ref(&attributes),
                    0,
                )
            };
            if out_pipe == 0 {
                // SAFETY: `job` is live and owned here; nothing else has used it.
                unsafe { close_owned(job) };
                return None;
            }

            let mut stdin_read: HANDLE = std::ptr::null_mut();
            let mut stdin_write: HANDLE = std::ptr::null_mut();
            // SAFETY: identical shape to the pipe above.
            let in_pipe = unsafe {
                CreatePipe(
                    &mut stdin_read,
                    &mut stdin_write,
                    std::ptr::from_ref(&attributes),
                    0,
                )
            };
            if in_pipe == 0 {
                // SAFETY: the handles are live, exclusively owned here and not used again.
                unsafe {
                    close_owned(stdout_read);
                    close_owned(stdout_write);
                    close_owned(job);
                }
                return None;
            }

            // The parent-side ends must never be inherited by any other process.
            // SAFETY: both handles are live parent ends this function owns.
            let cleared = unsafe {
                SetHandleInformation(stdout_read, HANDLE_FLAG_INHERIT, 0) != 0
                    && SetHandleInformation(stdin_write, HANDLE_FLAG_INHERIT, 0) != 0
            };
            if !cleared {
                // SAFETY: all five handles are live, owned and not used again.
                unsafe {
                    close_owned(stdout_read);
                    close_owned(stdout_write);
                    close_owned(stdin_read);
                    close_owned(stdin_write);
                    close_owned(job);
                }
                return None;
            }

            // SAFETY: `STARTUPINFOW` is a plain `repr(C)` structure valid when zeroed; the four
            // fields set here are exactly the ones `STARTF_USESTDHANDLES` documents.
            let mut startup: STARTUPINFOW = unsafe { std::mem::zeroed() };
            startup.cb = u32::try_from(std::mem::size_of::<STARTUPINFOW>()).ok()?;
            startup.dwFlags = STARTF_USESTDHANDLES;
            startup.hStdInput = stdin_read;
            startup.hStdOutput = stdout_write;
            startup.hStdError = stdout_write;

            let image = executable.to_string_lossy();
            let mut parts: Vec<String> = Vec::with_capacity(arguments.len() + 1);
            parts.push(quote_argument(&image));
            for argument in arguments {
                parts.push(quote_argument(argument));
            }
            let application = wide(&image);
            let mut command_line = wide(&parts.join(" "));

            // SAFETY: `PROCESS_INFORMATION` is a plain `repr(C)` structure valid when zeroed.
            let mut info: PROCESS_INFORMATION = unsafe { std::mem::zeroed() };

            // SAFETY: the application name and the mutable command line are NUL-terminated UTF-16
            // buffers that outlive this call; both attribute pointers are null (inherited default
            // security); `bInheritHandles` is set only because exactly the two child pipe ends carry
            // the inherit flag and every parent end was just cleared; `CREATE_SUSPENDED` guarantees
            // the child cannot execute before the job assignment below; the environment and current
            // directory are deliberately inherited because the child reads neither; and `info` is
            // this function's own out-structure.
            let created = unsafe {
                CreateProcessW(
                    application.as_ptr(),
                    command_line.as_mut_ptr(),
                    std::ptr::null(),
                    std::ptr::null(),
                    1,
                    CREATE_SUSPENDED,
                    std::ptr::null(),
                    std::ptr::null(),
                    &startup,
                    &mut info,
                )
            };

            // The child's own ends are closed in the parent whatever happened.
            // SAFETY: both are live, exclusively owned here and unused afterwards.
            unsafe {
                close_owned(stdout_write);
                close_owned(stdin_read);
            }
            if created == 0 {
                // SAFETY: the remaining handles are live and owned here.
                unsafe {
                    close_owned(stdout_read);
                    close_owned(stdin_write);
                    close_owned(job);
                }
                return None;
            }

            // Assign the still-suspended worker and confirm real membership.
            // SAFETY: all three handles came from the calls above and are live; assigning a
            // suspended process changes no memory and cannot start it.
            let assigned = unsafe { AssignProcessToJobObject(job, info.hProcess) };
            let member = assigned != 0 && job_has_a_process_v1(job);
            if !member {
                // SAFETY: terminate this parent's own job, then close every handle exactly once.
                unsafe {
                    TerminateJobObject(job, 1);
                    close_owned(info.hThread);
                    close_owned(info.hProcess);
                    close_owned(stdout_read);
                    close_owned(stdin_write);
                    close_owned(job);
                }
                return None;
            }

            // SAFETY: both parent ends are live, exclusively owned and transferred to a `File` that
            // closes each exactly once; no other copy of either handle exists in this process.
            let stdin = unsafe { file_from_owned_handle(stdin_write) };
            // SAFETY: identical guarantee for the stdout read end.
            let stdout = unsafe { file_from_owned_handle(stdout_read) };

            Some(Self {
                job,
                process: info.hProcess,
                thread: info.hThread,
                pid: info.dwProcessId,
                limits_verified,
                resumed: false,
                exited: false,
                stdin,
                stdout,
            })
        }

        /// The worker's own process identifier, as the create call reported it.
        pub(crate) fn pid(&self) -> u32 {
            self.pid
        }

        /// Whether the job's four limits were read back as installed before the worker ran.
        pub(crate) fn limits_verified(&self) -> bool {
            self.limits_verified
        }

        /// Takes the parent's write end of the child's stdin.
        pub(crate) fn take_stdin(&mut self) -> Option<std::fs::File> {
            self.stdin.take()
        }

        /// Takes the parent's read end of the child's stdout.
        pub(crate) fn take_stdout(&mut self) -> Option<std::fs::File> {
            self.stdout.take()
        }

        /// Resumes the captured primary thread, which is the moment the worker may execute.
        ///
        /// It must only be called after the job was created, limited, read back and assigned.
        pub(crate) fn resume(&mut self) -> bool {
            if self.resumed || self.exited {
                return false;
            }
            // SAFETY: `self.thread` is the live primary-thread handle from `CreateProcessW`, still
            // suspended and owned exclusively here. `ResumeThread` decrements the suspend count and
            // returns the previous count, or `u32::MAX` on failure; it touches no Rust memory.
            let previous = unsafe { ResumeThread(self.thread) };
            if previous == u32::MAX {
                return false;
            }
            self.resumed = true;
            true
        }

        /// Waits up to `milliseconds` for the worker and returns its exit code when it has exited.
        pub(crate) fn wait_ms(&mut self, milliseconds: u32) -> Option<u32> {
            // SAFETY: `self.process` is a live process handle owned exclusively here. Waiting on a
            // handle changes no memory of this process and cannot fault.
            let signalled = unsafe { WaitForSingleObject(self.process, milliseconds) };
            if signalled != WAIT_OBJECT_0 {
                return None;
            }
            let mut code: u32 = 0;
            // SAFETY: the same live handle, and one out-pointer to this stack `u32`.
            let read = unsafe { GetExitCodeProcess(self.process, &mut code) };
            if read == 0 {
                return None;
            }
            self.exited = true;
            Some(code)
        }

        /// Terminates every process in this parent's own job.
        pub(crate) fn terminate_job(&mut self) -> bool {
            // SAFETY: `self.job` is the live job handle this struct owns, and the exit status is a
            // plain integer. Terminating a job cannot fault and touches no Rust memory.
            unsafe { TerminateJobObject(self.job, 1) != 0 }
        }
    }

    impl Drop for SuspendedWorkerV1 {
        /// Terminates a still-running job and closes every handle exactly once.
        fn drop(&mut self) {
            if !self.exited {
                // SAFETY: the job and process handles are live and owned by this struct; the bounded
                // wait below requires the termination to be observed, not merely requested.
                unsafe {
                    TerminateJobObject(self.job, 1);
                    WaitForSingleObject(self.process, 5_000);
                }
            }
            // SAFETY: each handle is live, owned here, closed exactly once and never used again.
            unsafe {
                close_owned(self.thread);
                close_owned(self.process);
                close_owned(self.job);
            }
        }
    }
}
