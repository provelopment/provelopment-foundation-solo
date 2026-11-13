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

    use std::mem::MaybeUninit;
    use std::os::unix::process::CommandExt;
    use std::process::Command;

    use crate::media_worker_host::{PROFILE_V1_AS_BYTES, PROFILE_V1_CPU_SECONDS};

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

// The three fixed entry points the safe façade consumes.
//
// The implementation lives inside the positively Linux-gated `linux` module above, which keeps
// every `unsafe` occurrence inside a provable gate; these re-exports only make the same three
// fixed functions nameable from the rest of the crate. There is no general-purpose native API, no
// caller-selected limit, no executable parameter and no Windows counterpart here.
#[cfg(target_os = "linux")]
pub(crate) use linux::{
    install_linux_child_limits_v1, observe_process_identity_v1, own_process_group_v1,
    read_limit_ceilings_v1,
};
