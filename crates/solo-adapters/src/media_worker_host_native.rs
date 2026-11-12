//! Linux-only native resource-limit **readback** for `R03.C3`.
//!
//! This is the boundary that `SOLO-WO-074 R01` reserved and `SOLO-WO-076 R01` filled: a private
//! seam that observes the calling process's **existing** address-space and CPU ceilings through
//! the real kernel. It is deliberately not a supervisor. There is no child process, no limit
//! mutation, no signal, no session, no handle and no output protocol here — only two `getrlimit`
//! observations with compile-time-fixed selectors.
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
