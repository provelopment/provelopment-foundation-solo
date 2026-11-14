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
#[cfg(windows)]
pub use windows::{
    N102ExtraInheritableV1, N102FaultContextV1, n102_probe_environment_v1, n102_probe_handle_v1,
};
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
        CloseHandle, GetHandleInformation, HANDLE, HANDLE_FLAG_INHERIT, SetHandleInformation,
        WAIT_OBJECT_0,
    };
    use windows_sys::Win32::Security::SECURITY_ATTRIBUTES;
    use windows_sys::Win32::System::JobObjects::{
        AssignProcessToJobObject, CreateJobObjectW, IsProcessInJob, JOB_OBJECT_LIMIT_JOB_MEMORY,
        JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE, JOB_OBJECT_LIMIT_PROCESS_MEMORY,
        JOB_OBJECT_LIMIT_PROCESS_TIME, JOBOBJECT_BASIC_ACCOUNTING_INFORMATION,
        JOBOBJECT_EXTENDED_LIMIT_INFORMATION, JobObjectBasicAccountingInformation,
        JobObjectExtendedLimitInformation, QueryInformationJobObject, SetInformationJobObject,
        TerminateJobObject,
    };
    use windows_sys::Win32::System::Pipes::CreatePipe;
    use windows_sys::Win32::System::Threading::{
        CREATE_SUSPENDED, CREATE_UNICODE_ENVIRONMENT, CreateEventW, CreateProcessW,
        DeleteProcThreadAttributeList, EXTENDED_STARTUPINFO_PRESENT, GetCurrentProcess,
        GetExitCodeProcess, InitializeProcThreadAttributeList, LPPROC_THREAD_ATTRIBUTE_LIST,
        PROC_THREAD_ATTRIBUTE_HANDLE_LIST, PROCESS_INFORMATION, ResumeThread, STARTF_USESTDHANDLES,
        STARTUPINFOEXW, TerminateProcess, UpdateProcThreadAttribute, WaitForSingleObject,
    };

    /// The largest attribute-list size this code will allocate for a one-entry list.
    ///
    /// The documented size for a two-handle inheritance list is a few hundred bytes; anything
    /// outside this bound is treated as a refused host rather than a number to trust.
    // ---------------------------------------------------------------------------------
    // `SOLO-WO-103 R01` closed N102 test seam.
    //
    // Two fixed, bounded facts are exposed to the integration suite and to nothing else: an
    // injection selector that makes one production native boundary refuse, and the child-side
    // probes for the two isolation canaries. Neither is reachable from authored content, an
    // ordinary public argument or any product configuration: the requested step is an immutable
    // per-run value owned by one invocation, so it cannot leak between runs. The probes run only in
    // the reserved internal worker modes, and no handle value, canary value or path is ever logged.
    /// One closed per-run N102 fault context: the boundary this invocation must make refuse, the
    /// boundary it actually reached, and whether the cleanup that followed a fault was observed.
    ///
    /// The value is constructed per invocation and passed by reference, so no global or thread-local
    /// state exists: a concurrent host test can never observe another test's fault, and no normal
    /// production run can be injected because it constructs [`Self::production`].
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct N102FaultContextV1 {
        requested: Option<u8>,
        reached: Option<u8>,
        cleanup_verified: bool,
    }

    impl N102FaultContextV1 {
        /// A non-injected context: every normal production image run constructs this.
        pub const fn production() -> Self {
            Self {
                requested: None,
                reached: None,
                cleanup_verified: false,
            }
        }

        /// A context that must make exactly `step` refuse, for the closed integration seam only.
        pub const fn requesting(step: u8) -> Self {
            Self {
                requested: Some(step),
                reached: None,
                cleanup_verified: false,
            }
        }

        /// Whether this boundary is the one this run requested, recording that it was reached.
        pub fn inject_here(&mut self, boundary: u8) -> bool {
            if self.reached.is_none() && self.requested == Some(boundary) {
                self.reached = Some(boundary);
                true
            } else {
                false
            }
        }

        /// The boundary this run actually reached, recorded at the native boundary itself.
        pub fn reached(&self) -> Option<u8> {
            self.reached
        }

        /// Whether the cleanup that followed a fault was observed to complete.
        pub fn cleanup_verified(&self) -> bool {
            self.cleanup_verified
        }

        /// Records that the owned child was terminated and its exit actually observed.
        pub fn mark_cleanup_verified(&mut self) {
            self.cleanup_verified = true;
        }
    }

    const ATTRIBUTE_LIST_MAX_BYTES: usize = 4_096;

    /// One initialised `PROC_THREAD_ATTRIBUTE_LIST` plus the aligned storage it points into.
    ///
    /// The owner keeps `storage` alive and never reallocates it while Windows holds `list`, and
    /// `DeleteProcThreadAttributeList` runs exactly once, only after a successful initialisation.
    struct AttributeListV1 {
        storage: Vec<usize>,
        list: LPPROC_THREAD_ATTRIBUTE_LIST,
        initialised: bool,
    }

    impl AttributeListV1 {
        /// Probes the required size, allocates, initialises, and installs the two-handle allowlist.
        ///
        /// Every step is fail-closed: a probe that unexpectedly succeeds, a size of zero or one
        /// beyond the bound, an initialisation failure and a rejected update all return `None`
        /// without ever having installed a partial or unrestricted inheritance arrangement.
        fn new(handles: &[HANDLE; 2]) -> Option<Self> {
            if handles[0].is_null()
                || handles[1].is_null()
                || handles[0] == handles[1]
                || handles[0] == windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE
                || handles[1] == windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE
            {
                return None;
            }

            let mut bytes_needed: usize = 0;
            // SAFETY: this is the documented size-probe form — a null list, one entry and this
            // function's own `usize` for the size — and it is expected to fail without writing
            // anywhere except that `usize`.
            let probe = unsafe {
                InitializeProcThreadAttributeList(std::ptr::null_mut(), 1, 0, &mut bytes_needed)
            };
            if probe != 0 || bytes_needed == 0 || bytes_needed > ATTRIBUTE_LIST_MAX_BYTES {
                return None;
            }

            let word = std::mem::size_of::<usize>();
            let words = bytes_needed.checked_add(word - 1)? / word;
            // `Vec<usize>` provides `usize` alignment, which satisfies the list's alignment
            // requirement, and the buffer is created at its final size so it never moves.
            let mut storage: Vec<usize> = vec![0; words];
            let list: LPPROC_THREAD_ATTRIBUTE_LIST = storage.as_mut_ptr().cast();
            // SAFETY: `list` names `words * size_of::<usize>() >= bytes_needed` writable, aligned
            // bytes owned by `storage`, the entry count matches the single attribute installed
            // below, and the size pointer is this function's own.
            let initialised =
                unsafe { InitializeProcThreadAttributeList(list, 1, 0, &mut bytes_needed) };
            if initialised == 0 {
                // The list was never initialised, so it must not be deleted.
                return None;
            }
            let owner = Self {
                storage,
                list,
                initialised: true,
            };
            // SAFETY: `list` is initialised with capacity for one attribute; the value pointer names
            // the caller's live two-handle array and the size is exactly that array's size, which is
            // the documented shape of an inheritance allowlist.
            let updated = unsafe {
                UpdateProcThreadAttribute(
                    list,
                    0,
                    PROC_THREAD_ATTRIBUTE_HANDLE_LIST as usize,
                    handles.as_ptr().cast(),
                    std::mem::size_of_val(handles),
                    std::ptr::null_mut(),
                    std::ptr::null(),
                )
            };
            if updated == 0 {
                return None;
            }
            Some(owner)
        }

        fn list_ptr(&self) -> LPPROC_THREAD_ATTRIBUTE_LIST {
            self.list
        }
    }

    impl Drop for AttributeListV1 {
        fn drop(&mut self) {
            if self.initialised {
                // SAFETY: the list was initialised exactly once by `new` and is deleted exactly once
                // here, while `storage` — the memory it points into — is still alive.
                unsafe { DeleteProcThreadAttributeList(self.list) };
                self.initialised = false;
            }
            let _ = self.storage.capacity();
        }
    }

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
    /// One native handle this module owns, closed exactly once however the path leaves.
    ///
    /// Guards exist because `spawn` has many fallible steps between acquiring a handle and handing
    /// it to the constructed worker: with explicit closes only, one added `?` is one new leak.
    struct OwnedHandleV1(HANDLE);

    impl OwnedHandleV1 {
        fn new(handle: HANDLE) -> Option<Self> {
            if handle.is_null() || handle == windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE {
                None
            } else {
                Some(Self(handle))
            }
        }

        fn raw(&self) -> HANDLE {
            self.0
        }

        /// Transfers ownership to the struct that will close it.
        fn into_raw(self) -> HANDLE {
            let handle = self.0;
            std::mem::forget(self);
            handle
        }
    }

    impl Drop for OwnedHandleV1 {
        fn drop(&mut self) {
            // SAFETY: this guard is the sole owner of the handle and closes it exactly once, after
            // which no copy of it is used anywhere.
            unsafe { close_owned(self.0) }
        }
    }

    /// Creates a job, installs the four limits and **requires** a successful readback.
    ///
    /// A job whose readback fails, or whose limits cannot be written, is closed here and reported as
    /// a failure. No caller can therefore hold a job it has not verified, and the process launch
    /// that follows is impossible without confirmed limits.
    /// One additional explicitly inheritable parent handle that the allowlist never contains.
    ///
    /// The integration suite holds one of these across a real spawn, so a child that could reach it
    /// would prove broad inheritance. It is created with `bInheritHandle` set on purpose and is
    /// never added to the two-entry `PROC_THREAD_ATTRIBUTE_HANDLE_LIST`.
    pub struct N102ExtraInheritableV1(OwnedHandleV1);

    impl N102ExtraInheritableV1 {
        /// Creates the extra inheritable handle, or `None` when the kernel refuses.
        pub fn create() -> Option<Self> {
            Self::create_with_inherit_v1(1)
        }

        /// Creates one comparable **non**-inheritable handle: a numeric value the child can never
        /// receive by inheritance, used as the ambiguity control for one handle probe.
        pub fn create_control() -> Option<Self> {
            Self::create_with_inherit_v1(0)
        }

        fn create_with_inherit_v1(inherit: i32) -> Option<Self> {
            // SAFETY: `SECURITY_ATTRIBUTES` is a plain `repr(C)` structure valid when zeroed; only
            // the three fields set below are read by `CreateEventW`, and a null name with manual
            // reset asks for one fresh anonymous event owned by this process.
            let mut attributes: SECURITY_ATTRIBUTES = unsafe { std::mem::zeroed() };
            attributes.nLength = u32::try_from(std::mem::size_of::<SECURITY_ATTRIBUTES>()).ok()?;
            attributes.lpSecurityDescriptor = std::ptr::null_mut();
            attributes.bInheritHandle = inherit;
            // SAFETY: the attributes pointer names this function's own live structure, the name is a
            // null pointer (anonymous object) and both flags are documented constants.
            let event =
                unsafe { CreateEventW(std::ptr::from_ref(&attributes), 1, 0, std::ptr::null()) };
            if event.is_null() {
                return None;
            }
            Some(Self(OwnedHandleV1::new(event)?))
        }

        /// The kernel's own numeric value for this handle, as a child would see it if inherited.
        pub fn value(&self) -> u32 {
            self.0.raw() as usize as u32
        }
    }

    /// Whether one numeric handle value is live in this process, plus whether a value the parent
    /// guarantees can never be a handle is also reported live.
    ///
    /// Used only in the reserved internal probe mode. A value that is not a handle is the expected
    /// answer for a correctly excluded handle; a probe whose guaranteed-invalid control also reports
    /// live means the child's handle table cannot be trusted for this question, so the caller refuses
    /// the run instead of reading it as either inheritance or exclusion.
    pub fn n102_probe_handle_v1(value: u32, control: u32) -> (bool, bool) {
        fn live(value: u32) -> bool {
            let mut flags: u32 = 0;
            // SAFETY: `GetHandleInformation` only reads this process's own handle table for the
            // supplied value and writes the out-parameter owned here; an absent handle makes it fail,
            // which is exactly the fact being measured.
            (unsafe { GetHandleInformation(value as usize as HANDLE, &mut flags) }) != 0
        }
        let ambiguous = live(control);
        (live(value), ambiguous)
    }

    /// Whether the named variable is visible to this process, and whether `SystemRoot` is present.
    ///
    /// `std::env` reads the real process environment block, so this observes what the production
    /// minimal-environment `CreateProcessW` actually installed rather than any local model of it.
    pub fn n102_probe_environment_v1(name: &std::ffi::OsStr) -> (bool, bool) {
        (
            std::env::var_os(name).is_some(),
            std::env::var_os("SystemRoot").is_some(),
        )
    }

    fn create_limited_job_v1() -> Option<OwnedHandleV1> {
        // SAFETY: `CreateJobObjectW` takes a null security descriptor and a null name, so it creates
        // an anonymous job whose only handle this process owns. It returns a handle or null.
        let job =
            OwnedHandleV1::new(unsafe { CreateJobObjectW(std::ptr::null(), std::ptr::null()) })?;

        // SAFETY: every field of these `repr(C)` window structures is valid when zeroed, which is
        // the documented "no limits" baseline this code then fills in explicitly.
        let mut limits: JOBOBJECT_EXTENDED_LIMIT_INFORMATION = unsafe { std::mem::zeroed() };
        limits.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_PROCESS_TIME
            | JOB_OBJECT_LIMIT_PROCESS_MEMORY
            | JOB_OBJECT_LIMIT_JOB_MEMORY
            | JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
        limits.BasicLimitInformation.PerProcessUserTimeLimit = PROCESS_TIME_100NS;
        limits.ProcessMemoryLimit = PROCESS_MEMORY_BYTES;
        limits.JobMemoryLimit = JOB_MEMORY_BYTES;

        let length =
            u32::try_from(std::mem::size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>()).ok()?;

        // SAFETY: the job handle came from `CreateJobObjectW` just above and is owned by the guard,
        // the information class matches the structure type, and the length is exactly that
        // structure's size, which is `SetInformationJobObject`'s contract for this class.
        let set = unsafe {
            SetInformationJobObject(
                job.raw(),
                JobObjectExtendedLimitInformation,
                std::ptr::from_ref(&limits).cast::<c_void>(),
                length,
            )
        };
        if set == 0 {
            return None;
        }

        // The readback is the contract: a job that cannot prove its four limits is closed by the
        // guard and never reaches a process launch.
        if !read_job_limits_v1(job.raw()) {
            return None;
        }
        Some(job)
    }

    /// Reads the job's effective limits back from the kernel and compares all four.
    fn read_job_limits_v1(job: HANDLE) -> bool {
        // SAFETY: zeroed `repr(C)` structure, filled by the query below.
        let mut observed: JOBOBJECT_EXTENDED_LIMIT_INFORMATION = unsafe { std::mem::zeroed() };
        let mut returned: u32 = 0;
        let Some(length) =
            u32::try_from(std::mem::size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>()).ok()
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
        let mut accounting: JOBOBJECT_BASIC_ACCOUNTING_INFORMATION = unsafe { std::mem::zeroed() };
        let mut returned: u32 = 0;
        let Some(length) =
            u32::try_from(std::mem::size_of::<JOBOBJECT_BASIC_ACCOUNTING_INFORMATION>()).ok()
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

    /// The fixed, minimal environment the worker is given.
    ///
    /// Only `SystemRoot` and — when it differs — `WINDIR` are admitted, each read from its single
    /// named OS variable, re-validated as a non-empty absolute ASCII Windows path with no interior
    /// NUL and a bounded length, and emitted as an explicitly sorted, double-NUL-terminated UTF-16
    /// block. Nothing else is forwarded: no `PATH`, profile, temp, token, cloud, repository or
    /// arbitrary inherited value, and never the ambient block itself.
    fn minimal_environment_v1() -> Option<Vec<u16>> {
        let mut admitted: Vec<(String, String)> = Vec::new();
        for name in ["SystemRoot", "WINDIR"] {
            let value = std::env::var(name).ok()?;
            if value.is_empty()
                || value.len() > 260
                || value.contains('\0')
                || !value.is_ascii()
                || !std::path::Path::new(&value).is_absolute()
            {
                return None;
            }
            if admitted
                .iter()
                .any(|(_, existing)| existing.eq_ignore_ascii_case(&value))
            {
                continue;
            }
            admitted.push((name.to_owned(), value));
        }
        // Windows expects the block's names in case-insensitive sorted order.
        admitted.sort_by_key(|(name, _)| name.to_uppercase());

        let mut block: Vec<u16> = Vec::new();
        for (name, value) in &admitted {
            block.extend(format!("{name}={value}").encode_utf16());
            block.push(0);
        }
        // The block itself must be terminated by one further NUL.
        block.push(0);
        Some(block)
    }
    impl SuspendedWorkerV1 {
        /// Creates the worker suspended, assigns it to a freshly limited job and verifies both.
        ///
        /// Nothing here lets the child execute: `CREATE_SUSPENDED` holds it at creation, the job is
        /// limited and read back before assignment, and `resume` is a separate call the caller makes
        /// only after those facts are known.
        pub(crate) fn spawn(
            executable: &Path,
            arguments: &[&str],
            fault: &mut N102FaultContextV1,
        ) -> Option<Self> {
            if fault.inject_here(1) {
                return None;
            }
            let job = create_limited_job_v1()?;

            // SAFETY: `SECURITY_ATTRIBUTES` is a plain `repr(C)` structure valid when zeroed; the
            // three fields set below are the only ones `CreatePipe` reads.
            let mut attributes: SECURITY_ATTRIBUTES = unsafe { std::mem::zeroed() };
            attributes.nLength = u32::try_from(std::mem::size_of::<SECURITY_ATTRIBUTES>()).ok()?;
            attributes.lpSecurityDescriptor = std::ptr::null_mut();
            attributes.bInheritHandle = 1;

            if fault.inject_here(2) {
                return None;
            }
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
                return None;
            }
            // Every acquired handle is owned by a guard from here on, so each early return below
            // closes it exactly once without a hand-written close list.
            let stdout_read = OwnedHandleV1::new(stdout_read)?;
            let stdout_write = OwnedHandleV1::new(stdout_write)?;

            if fault.inject_here(3) {
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
                return None;
            }
            let stdin_read = OwnedHandleV1::new(stdin_read)?;
            let stdin_write = OwnedHandleV1::new(stdin_write)?;

            // The parent-side ends must never be inherited by any other process.
            // SAFETY: both handles are live parent ends owned by the guards above.
            if fault.inject_here(4) {
                return None;
            }
            let cleared = unsafe {
                SetHandleInformation(stdout_read.raw(), HANDLE_FLAG_INHERIT, 0) != 0
                    && SetHandleInformation(stdin_write.raw(), HANDLE_FLAG_INHERIT, 0) != 0
            };
            if !cleared {
                return None;
            }

            // The child may inherit exactly two handles: its stdin read end and its stdout write
            // end. Nothing else — no parent end, no job or process handle, no ambient std handle and
            // no provider handle — is admitted, and `bInheritHandles=TRUE` is only ever passed
            // together with this validated list.
            let child_handles: [HANDLE; 2] = [stdin_read.raw(), stdout_write.raw()];
            if fault.inject_here(5) {
                return None;
            }
            let allowlist = AttributeListV1::new(&child_handles)?;

            // SAFETY: `STARTUPINFOEXW` is a plain `repr(C)` structure valid when zeroed; the fields
            // set here are exactly those `STARTF_USESTDHANDLES` and the attribute list document.
            let mut startup: STARTUPINFOEXW = unsafe { std::mem::zeroed() };
            startup.StartupInfo.cb = u32::try_from(std::mem::size_of::<STARTUPINFOEXW>()).ok()?;
            startup.StartupInfo.dwFlags = STARTF_USESTDHANDLES;
            startup.StartupInfo.hStdInput = child_handles[0];
            startup.StartupInfo.hStdOutput = child_handles[1];
            startup.StartupInfo.hStdError = child_handles[1];
            startup.lpAttributeList = allowlist.list_ptr();

            let image = executable.to_string_lossy();
            let mut parts: Vec<String> = Vec::with_capacity(arguments.len() + 1);
            parts.push(quote_argument(&image));
            for argument in arguments {
                parts.push(quote_argument(argument));
            }
            let application = wide(&image);
            let mut command_line = wide(&parts.join(" "));

            // The child gets a freshly built minimal block, never the ambient one, and a working
            // directory bound to this one trusted executable's own parent directory.
            if fault.inject_here(6) {
                return None;
            }
            let environment = minimal_environment_v1()?;
            if fault.inject_here(7) {
                return None;
            }
            let working_directory = wide(
                &executable
                    .parent()
                    .filter(|parent| parent.is_dir())?
                    .to_string_lossy(),
            );

            // SAFETY: `PROCESS_INFORMATION` is a plain `repr(C)` structure valid when zeroed.
            let mut info: PROCESS_INFORMATION = unsafe { std::mem::zeroed() };

            // SAFETY: the application name and mutable command line are NUL-terminated UTF-16
            // buffers that outlive this call; the environment block and working directory are
            // NUL-terminated buffers owned by this function; `bInheritHandles` is `TRUE` only with
            // the initialised handle allowlist above, so exactly the two child ends are inherited;
            // `CREATE_SUSPENDED` holds the child before it can execute, `EXTENDED_STARTUPINFO_PRESENT`
            // is what makes the attribute list authoritative and `CREATE_UNICODE_ENVIRONMENT`
            // declares the block's encoding; the startup pointer is the base structure as the API
            // requires; and `info` is this function's own out-structure.
            if fault.inject_here(8) {
                return None;
            }
            let created = unsafe {
                CreateProcessW(
                    application.as_ptr(),
                    command_line.as_mut_ptr(),
                    std::ptr::null(),
                    std::ptr::null(),
                    1,
                    CREATE_SUSPENDED | EXTENDED_STARTUPINFO_PRESENT | CREATE_UNICODE_ENVIRONMENT,
                    environment.as_ptr().cast(),
                    working_directory.as_ptr(),
                    &startup.StartupInfo,
                    &mut info,
                )
            };

            // The child's own ends are closed in the parent whatever happened.
            // SAFETY: both are live and owned by their guards, which close each exactly once now.
            drop(stdout_write);
            drop(stdin_read);
            if created == 0 {
                return None;
            }
            let process = OwnedHandleV1::new(info.hProcess)?;
            let thread = OwnedHandleV1::new(info.hThread)?;

            // Assign the still-suspended worker and confirm real membership.
            // SAFETY: all three handles came from the calls above and are live; assigning a
            // suspended process changes no memory and cannot start it.
            if fault.inject_here(9) {
                // The suspended child exists but has never joined the job, so closing or terminating
                // the job cannot end it. The owned process itself is terminated and its exit is
                // observed; only that observation may be recorded as verified cleanup.
                // SAFETY: `process` is this function's own live process handle returned by the
                // `CreateProcessW` above; terminating it touches no Rust memory, and the bounded wait
                // observes the owned process itself.
                let ended = unsafe {
                    TerminateProcess(process.raw(), 1) != 0
                        && WaitForSingleObject(process.raw(), 5_000) == WAIT_OBJECT_0
                };
                if ended {
                    fault.mark_cleanup_verified();
                }
                return None;
            }
            let assigned = unsafe { AssignProcessToJobObject(job.raw(), process.raw()) };
            let member = assigned != 0 && job_has_a_process_v1(job.raw());
            if !member {
                // A process that never joined this job cannot be ended by terminating the job, so the
                // owned process handle is terminated directly and its exit observed; the job is also
                // terminated for any process that did join. Only an observed process exit is recorded
                // as verified cleanup, and every handle is then closed exactly once by its guard.
                // SAFETY: `job` and `process` are this function's own live handles from the calls
                // above; neither call touches Rust memory and both are bounded.
                let ended = unsafe {
                    TerminateJobObject(job.raw(), 1);
                    TerminateProcess(process.raw(), 1) != 0
                        && WaitForSingleObject(process.raw(), 5_000) == WAIT_OBJECT_0
                };
                if ended {
                    fault.mark_cleanup_verified();
                }
                return None;
            }

            // SAFETY: both parent ends are live, exclusively owned and transferred to a `File` that
            // closes each exactly once; no other copy of either handle exists in this process.
            let stdin = unsafe { file_from_owned_handle(stdin_write.into_raw()) };
            // SAFETY: identical guarantee for the stdout read end.
            let stdout = unsafe { file_from_owned_handle(stdout_read.into_raw()) };

            Some(Self {
                job: job.into_raw(),
                process: process.into_raw(),
                thread: thread.into_raw(),
                pid: info.dwProcessId,
                limits_verified: true,
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
        /// Terminates this worker's job and observes the owned child's actual exit, bounded.
        ///
        /// Returns `true` only when the termination call succeeded **and** the owned process handle
        /// signalled, so a caller may record verified cleanup only from an observed exit.
        pub(crate) fn terminate_and_observe_v1(&mut self) -> bool {
            // SAFETY: `self.job` is this worker's own live job handle, and terminating a job
            // touches no Rust memory.
            let job_ended = unsafe { TerminateJobObject(self.job, 1) != 0 };
            // SAFETY: `self.process` is this worker's own live process handle; the bounded wait
            // observes the owned process and cannot fault.
            let exited = unsafe { WaitForSingleObject(self.process, 5_000) == WAIT_OBJECT_0 };
            if exited {
                self.exited = true;
            }
            job_ended && exited
        }

        pub(crate) fn resume(&mut self, fault: &mut N102FaultContextV1) -> bool {
            if fault.inject_here(10) {
                return false;
            }
            if self.resumed || self.exited {
                return false;
            }
            // A worker whose four job limits were not read back as installed, or whose assignment
            // and membership were not confirmed, must never execute: the refusal is enforced here as
            // well as at construction, so a future caller cannot regress it by constructing a worker
            // some other way.
            if !self.limits_verified {
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
        pub(crate) fn wait_ms(
            &mut self,
            milliseconds: u32,
            fault: &mut N102FaultContextV1,
        ) -> Option<u32> {
            if fault.inject_here(11) {
                // The injected wait failure must not suppress cleanup: the job is terminated and the
                // owned child's exit observed before the refusal is returned.
                if self.terminate_and_observe_v1() {
                    fault.mark_cleanup_verified();
                }
                return None;
            }
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
