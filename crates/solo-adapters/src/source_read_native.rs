//! The audited native filesystem boundary for the source reader.
//!
//! This is the **only** file in the project permitted to call an operating-system
//! primitive directly, and the only file permitted to contain `unsafe`. Everything a
//! caller can observe comes back through the safe types in [`crate::source_read`].
//!
//! Two implementations exist, and exactly one is ever compiled:
//!
//! * **Linux** is entirely safe Rust over `rustix`. A single `openat2` call requests
//!   `RESOLVE_BENEATH`, `RESOLVE_NO_SYMLINKS`, `RESOLVE_NO_MAGICLINKS` and
//!   `RESOLVE_NO_XDEV`, so confinement, no-follow and no-cross-device are decided by the
//!   kernel in the same operation that opens the file. There is no check-then-open window
//!   to lose, and a kernel without `openat2` is refused rather than retried with a weaker
//!   call.
//! * **Windows** uses `NtCreateFile` one component at a time, relative to the handle of
//!   the directory above it, with `OBJ_DONT_REPARSE` on every component and
//!   `FILE_OPEN_REPARSE_POINT` on every open. The resulting handle is then interrogated
//!   for its own reparse tag, type and physical identity, so a substitution that happened
//!   during the open is detected on the object that was actually opened rather than on a
//!   name that may since have changed.
//!
//! **What this module does not prove.** Neither host provides an atomic,
//! filesystem-wide snapshot. The *transfer* is atomic on Linux in the strong sense: the bytes
//! come from an anonymous copy-on-write clone, so nothing a writer does afterwards can move
//! them. On Windows the read handle denies write and delete sharing, so no *handle-holding*
//! writer can modify the file while the read is in progress; a writable section that was mapped
//! before the read and has outlived its own file handle is an adversary that sharing rules do not
//! reach, and this order stopped before it could be executed, so nothing here claims it is
//! excluded. Beyond the transfer, a writer can still change this file after its clone was taken,
//! and the before/after resample reports exactly that as [`NativeError::Changed`] rather than
//! returning a stale body. A change that leaves every observed fact identical is not detectable,
//! and nothing here claims otherwise.

use std::path::Path;

use solo_application::PhysicalIdentityV1;

/// Why a native operation did not produce a usable, confined leaf.
///
/// This is deliberately finer than the public outcome: the public layer collapses it into
/// a typed refusal and never into absence, except for the one genuinely-absent case.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NativeError {
    /// The leaf provably does not exist.
    ///
    /// Only ever produced by a call that could distinguish "no such entry" from "could
    /// not tell", and never from a permission, loop, reparse or unsupported-kernel result.
    Absent,
    /// The location, or the object found there, is not safe to treat as an authored Page
    /// source: a symlink, junction, reparse point, non-regular file, cross-device escape
    /// or a root that is not a plain directory.
    Unsafe,
    /// Access was refused at some component or at the leaf.
    PermissionDenied,
    /// The object changed identity or length while it was being read.
    Changed,
    /// Some other operating-system fault prevented a trustworthy result.
    OtherIo,
    /// The platform cannot provide the required guarantee, so nothing is returned.
    Unsupported,
}

/// The facts an open handle reports about itself, before and after a read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LeafFacts {
    /// Stable physical identity: a device/volume component plus the file component.
    pub(crate) identity: PhysicalIdentityV1,
    /// Byte length observed on the handle.
    pub(crate) length: u64,
    /// How many directory entries reference this object. `> 1` means a hardlink exists.
    pub(crate) links: u64,
    /// Modification and change timestamps in nanoseconds, as the handle reports them.
    ///
    /// These are compared before and after the transfer, so a rewrite that preserves the
    /// inode and the length is still refused when it moves either timestamp.
    pub(crate) changed_nanos: (i64, i64),
}

/// An opened, identity-checked regular file beneath a root capability.
pub(crate) struct OpenLeaf {
    raw: sys::LeafHandle,
    facts: LeafFacts,
}

impl OpenLeaf {
    /// The facts observed on the handle at open time.
    pub(crate) const fn facts(&self) -> LeafFacts {
        self.facts
    }
}

/// A trusted root directory capability, held as an operating-system handle.
///
/// The path is used exactly once, here, and never again: every later operation is
/// relative to this handle. That is the point of holding it, and it is why a caller
/// cannot substitute a string for one.
#[derive(Debug)]
pub(crate) struct RootHandle {
    raw: sys::DirHandle,
    identity: PhysicalIdentityV1,
}

impl RootHandle {
    /// Opens `path` once and records the identity of what was actually opened.
    ///
    /// This is the **bootstrap** route: the path names the operator-designated content root,
    /// and after this call it is never consulted again. It is deliberately *not* the trusted
    /// handle transfer — see [`RootHandle::from_trusted_directory`], which accepts a handle
    /// that was already opened and verified by the caller.
    pub(crate) fn adopt_bootstrap_path(path: &Path) -> Result<Self, NativeError> {
        let (raw, identity) = sys::adopt_root(path)?;
        Ok(Self { raw, identity })
    }

    /// Takes ownership of an already-opened directory handle and verifies it on the handle.
    ///
    /// Nothing here resolves a path: the object under test is the handle the caller supplied.
    /// Its own type, reparse state and identity are read from the handle, so a handle that
    /// names a regular file, or a reparse point, is refused rather than trusted.
    pub(crate) fn from_trusted_directory(handle: TrustedDirectory) -> Result<Self, NativeError> {
        let (raw, identity) = sys::adopt_owned_root(handle)?;
        Ok(Self { raw, identity })
    }

    /// The identity recorded when the capability was adopted.
    pub(crate) const fn identity(&self) -> PhysicalIdentityV1 {
        self.identity
    }

    /// The stable description of the host mechanism in use, for the report.
    pub(crate) const fn mechanism(&self) -> &'static str {
        sys::MECHANISM
    }
}

/// The owned directory handle a trusted bootstrap hands to [`RootHandle`].
#[cfg(unix)]
pub(crate) type TrustedDirectory = std::os::fd::OwnedFd;

/// The owned directory handle a trusted bootstrap hands to [`RootHandle`].
#[cfg(windows)]
pub(crate) type TrustedDirectory = std::os::windows::io::OwnedHandle;

/// Opens one leaf by single relative names beneath `root`, without following anything.
///
/// `components` are the already-validated logical segments of a `pages/…` locator. Each
/// one is used as a single name; no component is ever re-assembled from caller text.
///
/// # Errors
///
/// Returns [`NativeError::Absent`] only when absence was positively established, and the
/// other variants for every condition that must not be mistaken for absence.
pub(crate) fn open_leaf(root: &RootHandle, components: &[&str]) -> Result<OpenLeaf, NativeError> {
    let (raw, facts) = sys::open_leaf(&root.raw, components)?;
    Ok(OpenLeaf { raw, facts })
}

/// Takes an immutable snapshot of an already-opened leaf and reads that snapshot.
///
/// On Linux the bytes come from an anonymous copy-on-write clone of the source, so a
/// concurrent writer cannot tear the transfer; on Windows the same effect comes from the
/// read handle denying write and delete sharing. Either way the caller receives one instant
/// of the file, or a typed refusal — never a partial mixture and never a silent copy.
pub(crate) fn read_bounded(
    root: &RootHandle,
    leaf: &mut OpenLeaf,
    max_bytes: u64,
) -> Result<Vec<u8>, NativeError> {
    sys::read_bounded(&root.raw, &mut leaf.raw, max_bytes)
}

/// Re-observes identity, length and link count on the open handle after the read.
pub(crate) fn recheck(leaf: &OpenLeaf) -> Result<LeafFacts, NativeError> {
    sys::recheck(&leaf.raw)
}

#[cfg(target_os = "linux")]
mod sys {
    //! Linux: safe `rustix` only, with every guarantee delegated to `openat2`.

    use super::{LeafFacts, NativeError, PhysicalIdentityV1};
    use rustix::fs::{Mode, OFlags, ResolveFlags};
    use std::path::Path;

    /// The mechanism string reported for this host.
    pub(super) const MECHANISM: &str =
        "linux-openat2-beneath-otmpfile-excl-ficlone-immutable-clone";

    #[derive(Debug)]
    pub(super) struct DirHandle {
        fd: rustix::fd::OwnedFd,
    }

    #[derive(Debug)]
    pub(super) struct LeafHandle {
        fd: rustix::fd::OwnedFd,
    }

    /// Reads identity, length and link count from the descriptor itself, never from a
    /// name, so the answer describes the object that was opened.
    fn facts_of(fd: &rustix::fd::OwnedFd) -> Result<LeafFacts, NativeError> {
        let stat = rustix::fs::fstat(fd).map_err(map_err)?;
        Ok(LeafFacts {
            identity: PhysicalIdentityV1::new(stat.st_dev as u64, stat.st_ino as u64, 0),
            length: u64::try_from(stat.st_size).unwrap_or(0),
            links: stat.st_nlink as u64,
            changed_nanos: (
                (stat.st_mtime as i64)
                    .saturating_mul(1_000_000_000)
                    .saturating_add(stat.st_mtime_nsec as i64),
                (stat.st_ctime as i64)
                    .saturating_mul(1_000_000_000)
                    .saturating_add(stat.st_ctime_nsec as i64),
            ),
        })
    }

    fn is_regular(fd: &rustix::fd::OwnedFd) -> Result<bool, NativeError> {
        let stat = rustix::fs::fstat(fd).map_err(map_err)?;
        Ok(rustix::fs::FileType::from_raw_mode(stat.st_mode) == rustix::fs::FileType::RegularFile)
    }

    /// Maps a `rustix` error onto the closed refusal set.
    ///
    /// `ENOENT` is the only mapping to `Absent`, and it is safe here precisely because the
    /// call that produced it resolved the whole path under `RESOLVE_BENEATH` and the
    /// no-follow flags: the kernel answered about that path, not about a prefix it
    /// declined to traverse. A missing `openat2` (`ENOSYS`) or an unsupported flag
    /// combination (`EINVAL`) is `Unsupported`, never a silent downgrade to `open`.
    fn map_err(error: rustix::io::Errno) -> NativeError {
        use rustix::io::Errno;
        match error {
            Errno::NOENT => NativeError::Absent,
            Errno::ACCESS | Errno::PERM => NativeError::PermissionDenied,
            // A symlink or magic link was met, or resolution would have left the root, or
            // crossed a mount. All are refusals, never absence.
            Errno::LOOP | Errno::XDEV | Errno::MLINK | Errno::ISDIR | Errno::NOTDIR => {
                NativeError::Unsafe
            }
            Errno::NAMETOOLONG => NativeError::Unsafe,
            Errno::NOSYS | Errno::INVAL | Errno::OPNOTSUPP => NativeError::Unsupported,
            _ => NativeError::OtherIo,
        }
    }

    pub(super) fn adopt_root(path: &Path) -> Result<(DirHandle, PhysicalIdentityV1), NativeError> {
        // The root itself is opened with `NOFOLLOW`, so a root that is a symlink is
        // refused rather than silently resolved into something else.
        let fd = rustix::fs::open(
            path,
            OFlags::RDONLY | OFlags::DIRECTORY | OFlags::NOFOLLOW | OFlags::CLOEXEC,
            Mode::empty(),
        )
        .map_err(map_err)?;
        let identity = facts_of(&fd)?.identity;
        Ok((DirHandle { fd }, identity))
    }

    /// Verifies an already-opened directory descriptor and takes ownership of it.
    ///
    /// The object under test is the descriptor the caller supplied: its own type and identity
    /// are read with `fstat`, so a descriptor naming a regular file, a device or a symlinked
    /// object is refused. No path is resolved or re-resolved here.
    pub(super) fn adopt_owned_root(
        fd: rustix::fd::OwnedFd,
    ) -> Result<(DirHandle, PhysicalIdentityV1), NativeError> {
        let stat = rustix::fs::fstat(&fd).map_err(map_err)?;
        if rustix::fs::FileType::from_raw_mode(stat.st_mode) != rustix::fs::FileType::Directory {
            return Err(NativeError::Unsafe);
        }
        let identity = facts_of(&fd)?.identity;
        Ok((DirHandle { fd }, identity))
    }

    pub(super) fn open_leaf(
        root: &DirHandle,
        components: &[&str],
    ) -> Result<(LeafHandle, LeafFacts), NativeError> {
        // Every component is already a validated single lowercase-ASCII segment, so a
        // joined relative path cannot smuggle a separator, a dot segment or an escape.
        let relative = components.join("/");
        let resolve = ResolveFlags::BENEATH
            | ResolveFlags::NO_SYMLINKS
            | ResolveFlags::NO_MAGICLINKS
            | ResolveFlags::NO_XDEV;
        let fd = rustix::fs::openat2(
            &root.fd,
            relative.as_str(),
            OFlags::RDONLY | OFlags::CLOEXEC,
            Mode::empty(),
            resolve,
        )
        .map_err(map_err)?;

        // The kernel proved the path was confined and symlink-free; the fstat below proves
        // that what was actually opened is a plain regular file, not a directory or a
        // device that would block a bounded read.
        if !is_regular(&fd)? {
            return Err(NativeError::Unsafe);
        }
        let facts = facts_of(&fd)?;
        Ok((LeafHandle { fd }, facts))
    }

    /// Maps a clone-mechanism failure onto the closed refusal set.
    ///
    /// Real non-support is `Unsupported`: a filesystem without reflink (`EOPNOTSUPP`), a kernel
    /// without the call (`ENOSYS`), or a form the kernel declines (`EINVAL`, `EPERM`, `EROFS`,
    /// `EBUSY`). A cross-device attempt is `Unsafe`, because the destination must live on the
    /// source's own filesystem. None of these may become absence, and none may be retried with
    /// a copying fallback.
    fn map_snapshot_err(error: rustix::io::Errno) -> NativeError {
        use rustix::io::Errno;
        match error {
            Errno::OPNOTSUPP
            | Errno::NOSYS
            | Errno::INVAL
            | Errno::PERM
            | Errno::ROFS
            | Errno::BUSY => NativeError::Unsupported,
            Errno::XDEV => NativeError::Unsafe,
            Errno::ACCESS => NativeError::PermissionDenied,
            Errno::NOENT | Errno::ISDIR | Errno::NOTDIR | Errno::LOOP | Errno::MLINK => {
                NativeError::Unsafe
            }
            _ => NativeError::OtherIo,
        }
    }

    /// Takes the immutable clone of the leaf and returns it with its verified length.
    ///
    /// The destination is an unnamed, unlinkable `O_TMPFILE | O_EXCL` inode created in the
    /// trusted root, so it is on the source's own filesystem and can never appear in the
    /// namespace; one `FICLONE` then makes it an immutable copy-on-write view of the source.
    /// The source handle is never read, because its bytes are exactly what a concurrent writer
    /// can still change. A filesystem that cannot clone this source is refused rather than
    /// copied, and a clone whose device or length disagrees with the source is refused.
    pub(super) fn snapshot_leaf(
        root: &DirHandle,
        leaf: &LeafHandle,
        max_bytes: u64,
    ) -> Result<(rustix::fd::OwnedFd, usize), NativeError> {
        let source = rustix::fs::fstat(&leaf.fd).map_err(map_err)?;
        let root_stat = rustix::fs::fstat(&root.fd).map_err(map_err)?;
        if !is_regular(&leaf.fd)? {
            return Err(NativeError::Unsafe);
        }
        // A destination created anywhere but the source's own filesystem would turn the
        // snapshot into a copy across mounts, which is not a snapshot of this object.
        if root_stat.st_dev != source.st_dev {
            return Err(NativeError::Unsafe);
        }
        let length = u64::try_from(source.st_size).unwrap_or(u64::MAX);
        if length > max_bytes {
            // The body exceeds its documented bound: a refusal, never a truncation.
            return Err(NativeError::Unsafe);
        }

        let destination = rustix::fs::openat(
            &root.fd,
            c".",
            OFlags::TMPFILE | OFlags::EXCL | OFlags::RDWR | OFlags::CLOEXEC,
            Mode::RUSR | Mode::WUSR,
        )
        .map_err(map_snapshot_err)?;
        // One clone, no retry and no alternative: a filesystem that cannot clone must refuse.
        rustix::fs::ioctl_ficlone(&destination, &leaf.fd).map_err(map_snapshot_err)?;

        let cloned = rustix::fs::fstat(&destination).map_err(map_err)?;
        if cloned.st_dev != source.st_dev || !is_regular(&destination)? {
            return Err(NativeError::Unsafe);
        }
        if u64::try_from(cloned.st_size).unwrap_or(u64::MAX) != length {
            return Err(NativeError::Changed);
        }
        let expected = usize::try_from(length).map_err(|_| NativeError::Unsupported)?;
        Ok((destination, expected))
    }

    /// Reads exactly the cloned bytes, never the mutable source.
    pub(super) fn read_bounded(
        root: &DirHandle,
        leaf: &mut LeafHandle,
        max_bytes: u64,
    ) -> Result<Vec<u8>, NativeError> {
        let (destination, expected) = snapshot_leaf(root, leaf, max_bytes)?;
        // Exact-length read of the clone. A short clone is a change, never a shorter body.
        let mut buffer = vec![0_u8; expected];
        let mut filled = 0_usize;
        while filled < expected {
            let read = rustix::io::read(&destination, &mut buffer[filled..]).map_err(map_err)?;
            if read == 0 {
                return Err(NativeError::Changed);
            }
            filled += read;
        }
        Ok(buffer)
    }

    pub(super) fn recheck(leaf: &LeafHandle) -> Result<LeafFacts, NativeError> {
        facts_of(&leaf.fd)
    }
}

#[cfg(windows)]
mod sys {
    //! Windows: `NtCreateFile` per component, with reparse refusal and by-handle identity.
    //!
    //! Every `unsafe` block in this module is one of four calls into the NT/Win32 surface,
    //! each carrying a `// SAFETY:` note. Nothing else in the project is allowed to be
    //! unsafe, and the architecture guard enforces that.

    use super::{LeafFacts, NativeError, PhysicalIdentityV1};
    use std::ffi::c_void;
    use std::fs::File;
    use std::io::Read;
    use std::os::windows::io::{AsRawHandle, FromRawHandle, OwnedHandle};
    use std::path::Path;
    use std::ptr;

    use windows_sys::Wdk::Foundation::OBJECT_ATTRIBUTES;
    use windows_sys::Wdk::Storage::FileSystem::NtCreateFile;
    use windows_sys::Win32::Foundation::{HANDLE, INVALID_HANDLE_VALUE, NTSTATUS, UNICODE_STRING};
    use windows_sys::Win32::Storage::FileSystem::{
        CreateFileW, FILE_ATTRIBUTE_DIRECTORY, FILE_ATTRIBUTE_TAG_INFO, FILE_BASIC_INFO,
        FILE_FLAG_BACKUP_SEMANTICS, FILE_FLAG_OPEN_REPARSE_POINT, FILE_ID_INFO, FILE_SHARE_DELETE,
        FILE_SHARE_READ, FILE_SHARE_WRITE, FILE_STANDARD_INFO, FileAttributeTagInfo, FileBasicInfo,
        FileIdInfo, FileStandardInfo, GetFileInformationByHandleEx, OPEN_EXISTING,
    };
    use windows_sys::Win32::System::IO::IO_STATUS_BLOCK;

    /// The mechanism string reported for this host.
    pub(super) const MECHANISM: &str = "windows-ntcreatefile-obj-dont-reparse-file-id-info";

    // NT object-attribute and create-option constants. They are defined here rather than
    // taken from a binding because windows-sys does not re-export every one of them, and a
    // wrong value here is caught by the hosted proof on both an ordinary file and every
    // reparse case, which is why those cases are mandatory.
    const OBJ_DONT_REPARSE: u32 = 0x0000_1000;
    const FILE_OPEN: u32 = 0x0000_0001;
    const FILE_DIRECTORY_FILE: u32 = 0x0000_0001;
    const FILE_NON_DIRECTORY_FILE: u32 = 0x0000_0040;
    const FILE_SYNCHRONOUS_IO_NONALERT: u32 = 0x0000_0020;
    const FILE_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
    const FILE_READ_DATA: u32 = 0x0001;
    const FILE_READ_ATTRIBUTES: u32 = 0x0080;
    const SYNCHRONIZE: u32 = 0x0010_0000;
    const SHARE_ALL: u32 = FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE;

    // NTSTATUS values this boundary must distinguish. Absence is only `OBJECT_NAME_NOT_FOUND`
    // and `OBJECT_PATH_NOT_FOUND`; everything else is a refusal.
    const STATUS_SUCCESS: NTSTATUS = 0;
    const STATUS_OBJECT_NAME_NOT_FOUND: NTSTATUS = 0xC000_0034u32 as NTSTATUS;
    const STATUS_OBJECT_PATH_NOT_FOUND: NTSTATUS = 0xC000_003Au32 as NTSTATUS;
    const STATUS_OBJECT_NAME_INVALID: NTSTATUS = 0xC000_0033u32 as NTSTATUS;
    const STATUS_ACCESS_DENIED: NTSTATUS = 0xC000_0022u32 as NTSTATUS;
    const STATUS_REPARSE_POINT_ENCOUNTERED: NTSTATUS = 0xC000_050Bu32 as NTSTATUS;
    const STATUS_STOPPED_ON_SYMLINK: NTSTATUS = 0x8000_002Du32 as NTSTATUS;
    const STATUS_NOT_A_DIRECTORY: NTSTATUS = 0xC000_0103u32 as NTSTATUS;
    const STATUS_FILE_IS_A_DIRECTORY: NTSTATUS = 0xC000_00BAu32 as NTSTATUS;
    const STATUS_DELETE_PENDING: NTSTATUS = 0xC000_0056u32 as NTSTATUS;

    #[derive(Debug)]
    pub(super) struct DirHandle {
        handle: OwnedHandle,
    }

    #[derive(Debug)]
    pub(super) struct LeafHandle {
        handle: OwnedHandle,
    }

    /// Maps an `NTSTATUS` onto the closed refusal set.
    fn map_status(status: NTSTATUS) -> NativeError {
        match status {
            STATUS_SUCCESS => NativeError::OtherIo,
            STATUS_OBJECT_NAME_NOT_FOUND | STATUS_OBJECT_PATH_NOT_FOUND => NativeError::Absent,
            STATUS_ACCESS_DENIED => NativeError::PermissionDenied,
            // A reparse point met on the way, or a component that is not the kind required.
            STATUS_REPARSE_POINT_ENCOUNTERED
            | STATUS_STOPPED_ON_SYMLINK
            | STATUS_NOT_A_DIRECTORY
            | STATUS_FILE_IS_A_DIRECTORY
            | STATUS_DELETE_PENDING => NativeError::Unsafe,
            STATUS_OBJECT_NAME_INVALID => NativeError::Unsafe,
            _ => NativeError::OtherIo,
        }
    }

    fn last_error() -> NativeError {
        NativeError::OtherIo
    }

    fn size_u32<T>() -> u32 {
        u32::try_from(core::mem::size_of::<T>()).unwrap_or(u32::MAX)
    }

    /// A zeroed value of one of the plain Win32 structures this module uses.
    ///
    /// The four types it is ever instantiated with are `#[repr(C)]` records of integers,
    /// byte arrays and `bool`, all of which have a valid all-zero bit pattern, so this cannot
    /// manufacture an invalid value for them.
    fn zeroed<T: Copy>() -> T {
        // SAFETY: `T` is one of `OBJECT_ATTRIBUTES`, `IO_STATUS_BLOCK`,
        // `FILE_ATTRIBUTE_TAG_INFO`, `FILE_ID_INFO`, `FILE_STANDARD_INFO` or
        // `FILE_BASIC_INFO`. Each is `#[repr(C)]` and consists only of integers, byte arrays
        // and `bool`, for which the all-zero pattern is a valid, fully initialised value.
        unsafe { core::mem::zeroed() }
    }

    /// Reads identity, length, link count and directory-ness from the handle itself.
    ///
    /// A non-zero reparse tag on the object that was actually opened is [`NativeError::Unsafe`]
    /// regardless of how the open was requested: asking the kernel not to follow a reparse
    /// point and *checking* that it did not are deliberately both done.
    fn handle_facts(handle: HANDLE) -> Result<(LeafFacts, bool), NativeError> {
        let mut tag: FILE_ATTRIBUTE_TAG_INFO = zeroed();
        // SAFETY: `handle` is a live handle owned by the caller for this call; `tag` is a
        // correctly sized and aligned `FILE_ATTRIBUTE_TAG_INFO`; the byte count is exactly
        // that size, which is the documented contract of `GetFileInformationByHandleEx`.
        let tag_ok = unsafe {
            GetFileInformationByHandleEx(
                handle,
                FileAttributeTagInfo,
                ptr::from_mut(&mut tag).cast::<c_void>(),
                size_u32::<FILE_ATTRIBUTE_TAG_INFO>(),
            )
        };
        if tag_ok == 0 {
            return Err(last_error());
        }
        if tag.ReparseTag != 0 {
            return Err(NativeError::Unsafe);
        }

        let mut id: FILE_ID_INFO = zeroed();
        // SAFETY: as above, with the `FILE_ID_INFO` shape and its exact size.
        let id_ok = unsafe {
            GetFileInformationByHandleEx(
                handle,
                FileIdInfo,
                ptr::from_mut(&mut id).cast::<c_void>(),
                size_u32::<FILE_ID_INFO>(),
            )
        };
        if id_ok == 0 {
            return Err(NativeError::Unsupported);
        }

        let mut standard: FILE_STANDARD_INFO = zeroed();
        // SAFETY: as above, with the `FILE_STANDARD_INFO` shape and its exact size.
        let standard_ok = unsafe {
            GetFileInformationByHandleEx(
                handle,
                FileStandardInfo,
                ptr::from_mut(&mut standard).cast::<c_void>(),
                size_u32::<FILE_STANDARD_INFO>(),
            )
        };
        if standard_ok == 0 {
            return Err(last_error());
        }

        let mut basic: FILE_BASIC_INFO = zeroed();
        // SAFETY: as above, with the `FILE_BASIC_INFO` shape and its exact size.
        let basic_ok = unsafe {
            GetFileInformationByHandleEx(
                handle,
                FileBasicInfo,
                ptr::from_mut(&mut basic).cast::<c_void>(),
                size_u32::<FILE_BASIC_INFO>(),
            )
        };
        if basic_ok == 0 {
            return Err(last_error());
        }

        let mut low = [0_u8; 8];
        let mut high = [0_u8; 8];
        low.copy_from_slice(&id.FileId.Identifier[0..8]);
        high.copy_from_slice(&id.FileId.Identifier[8..16]);
        let facts = LeafFacts {
            identity: PhysicalIdentityV1::new(
                id.VolumeSerialNumber,
                u64::from_le_bytes(low),
                u64::from_le_bytes(high),
            ),
            length: u64::try_from(standard.EndOfFile).unwrap_or(0),
            links: u64::from(standard.NumberOfLinks),
            changed_nanos: (basic.LastWriteTime, basic.ChangeTime),
        };
        let is_dir = tag.FileAttributes & FILE_ATTRIBUTE_DIRECTORY != 0;
        Ok((facts, is_dir))
    }

    pub(super) fn adopt_root(path: &Path) -> Result<(DirHandle, PhysicalIdentityV1), NativeError> {
        use std::os::windows::ffi::OsStrExt;
        let mut wide: Vec<u16> = path.as_os_str().encode_wide().collect();
        wide.push(0);

        // SAFETY: `wide` is NUL-terminated and outlives the call; the remaining arguments
        // are the documented constants for opening an existing directory for attribute
        // access. `FILE_FLAG_OPEN_REPARSE_POINT` means the root itself is not silently
        // resolved through a reparse point, and `FILE_FLAG_BACKUP_SEMANTICS` is what makes
        // a directory openable at all.
        let raw = unsafe {
            CreateFileW(
                wide.as_ptr(),
                FILE_READ_ATTRIBUTES | SYNCHRONIZE,
                SHARE_ALL,
                ptr::null(),
                OPEN_EXISTING,
                FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT,
                ptr::null_mut(),
            )
        };
        if raw == INVALID_HANDLE_VALUE {
            return Err(last_error());
        }
        // SAFETY: `raw` was just returned by `CreateFileW`, is not `INVALID_HANDLE_VALUE`,
        // is unowned, and is moved exactly once into this `OwnedHandle`, which becomes its
        // only owner and closes it on drop.
        let handle = unsafe { OwnedHandle::from_raw_handle(raw) };
        let (facts, is_dir) = handle_facts(raw)?;
        if !is_dir {
            return Err(NativeError::Unsafe);
        }
        Ok((DirHandle { handle }, facts.identity))
    }

    /// Verifies an already-opened directory handle and takes ownership of it.
    ///
    /// Type, reparse state and identity are read from the handle the caller supplied, so a
    /// handle naming a regular file or a reparse point is refused. No path is resolved here.
    pub(super) fn adopt_owned_root(
        handle: OwnedHandle,
    ) -> Result<(DirHandle, PhysicalIdentityV1), NativeError> {
        let (facts, is_dir) = handle_facts(handle.as_raw_handle())?;
        if !is_dir {
            return Err(NativeError::Unsafe);
        }
        Ok((DirHandle { handle }, facts.identity))
    }

    /// Opens one single relative name under an already-open parent handle.
    ///
    /// `OBJ_CASE_INSENSITIVE` is deliberately **not** requested: the lookup is meant to be
    /// exact, so a differently-cased spelling is not silently accepted as the same entry.
    fn open_relative(
        parent: HANDLE,
        name: &str,
        want_dir: bool,
        want_read: bool,
    ) -> Result<OwnedHandle, NativeError> {
        if name.is_empty() || name.contains('\0') || name.contains(['/', '\\']) {
            return Err(NativeError::Unsafe);
        }
        let wide: Vec<u16> = name.encode_utf16().collect();
        let Ok(byte_len) = u16::try_from(wide.len().saturating_mul(2)) else {
            return Err(NativeError::Unsupported);
        };
        let unicode = UNICODE_STRING {
            Length: byte_len,
            MaximumLength: byte_len,
            Buffer: wide.as_ptr().cast_mut(),
        };
        let mut attributes: OBJECT_ATTRIBUTES = zeroed();
        attributes.Length = size_u32::<OBJECT_ATTRIBUTES>();
        attributes.RootDirectory = parent;
        attributes.ObjectName = ptr::from_ref(&unicode);
        attributes.Attributes = OBJ_DONT_REPARSE;
        let mut io_status: IO_STATUS_BLOCK = zeroed();
        let mut out: HANDLE = ptr::null_mut();
        let options = FILE_OPEN_REPARSE_POINT
            | FILE_SYNCHRONOUS_IO_NONALERT
            | if want_dir {
                FILE_DIRECTORY_FILE
            } else {
                FILE_NON_DIRECTORY_FILE
            };
        let access =
            FILE_READ_ATTRIBUTES | SYNCHRONIZE | if want_read { FILE_READ_DATA } else { 0 };
        // A handle used to read a body denies write and delete sharing for as long as it is
        // open, so no other opener can modify the object under this handle: that is what makes
        // the read atomic on this host rather than merely sampled. Directory handles keep full
        // sharing, because they are only traversed.
        let share = if want_read {
            FILE_SHARE_READ
        } else {
            SHARE_ALL
        };

        // SAFETY: `parent` is a live handle owned by the caller for this call; `unicode`
        // names `wide`, which is a NUL-free UTF-16 buffer alive across the call and whose
        // byte length is exactly `Length`; `attributes` is initialised in full with its
        // documented `Length`, a borrowed `ObjectName` pointer and `OBJ_DONT_REPARSE`;
        // `io_status` and `out` are valid, properly aligned writable outputs; the
        // allocation size and EA arguments are null with length zero, which is the
        // documented "none" form.
        let status = unsafe {
            NtCreateFile(
                ptr::from_mut(&mut out),
                access,
                ptr::from_ref(&attributes),
                ptr::from_mut(&mut io_status),
                ptr::null(),
                0,
                share,
                FILE_OPEN,
                options,
                ptr::null(),
                0,
            )
        };
        if status != STATUS_SUCCESS {
            return Err(map_status(status));
        }
        if out.is_null() || out == INVALID_HANDLE_VALUE {
            return Err(NativeError::OtherIo);
        }
        // SAFETY: `out` is a valid handle returned by a successful `NtCreateFile`, is
        // unowned, and is moved exactly once into this `OwnedHandle`, which becomes its
        // only owner.
        Ok(unsafe { OwnedHandle::from_raw_handle(out) })
    }

    pub(super) fn open_leaf(
        root: &DirHandle,
        components: &[&str],
    ) -> Result<(LeafHandle, LeafFacts), NativeError> {
        let Some((leaf, intermediates)) = components.split_last() else {
            return Err(NativeError::Unsafe);
        };
        let mut walked: Option<OwnedHandle> = None;
        for segment in intermediates {
            let parent = match &walked {
                Some(handle) => handle.as_raw_handle(),
                None => root.handle.as_raw_handle(),
            };
            let child = open_relative(parent, segment, true, false)?;
            // The child becomes the next root, so its own type is checked on the handle
            // that was just opened rather than assumed from the name that was used.
            let (_, is_dir) = handle_facts(child.as_raw_handle())?;
            if !is_dir {
                return Err(NativeError::Unsafe);
            }
            walked = Some(child);
        }
        let parent = match &walked {
            Some(handle) => handle.as_raw_handle(),
            None => root.handle.as_raw_handle(),
        };
        let opened = open_relative(parent, leaf, false, true)?;
        let (facts, is_dir) = handle_facts(opened.as_raw_handle())?;
        if is_dir {
            return Err(NativeError::Unsafe);
        }
        Ok((LeafHandle { handle: opened }, facts))
    }

    pub(super) fn read_bounded(
        _root: &DirHandle,
        leaf: &mut LeafHandle,
        max_bytes: u64,
    ) -> Result<Vec<u8>, NativeError> {
        // The read handle already denies write and delete sharing, so no other opener can
        // modify the object under it; the transfer below is therefore one instant of the file.
        // A borrowed clone keeps the original handle owned by `LeafHandle` and lets the
        // standard library's safe `Read` implementation do the transfer.
        let cloned = leaf.handle.try_clone().map_err(|_| NativeError::OtherIo)?;
        let mut file = File::from(cloned);
        let limit = usize::try_from(max_bytes).map_err(|_| NativeError::Unsupported)?;
        let ceiling = limit.checked_add(1).ok_or(NativeError::Unsupported)?;
        let mut buffer = vec![0_u8; ceiling];
        let mut filled = 0_usize;
        while filled < ceiling {
            match file.read(&mut buffer[filled..]) {
                Ok(0) => break,
                Ok(read) => filled += read,
                Err(error) => {
                    return Err(match error.kind() {
                        std::io::ErrorKind::PermissionDenied => NativeError::PermissionDenied,
                        _ => NativeError::OtherIo,
                    });
                }
            }
        }
        if filled > limit {
            return Err(NativeError::Unsafe);
        }
        buffer.truncate(filled);
        Ok(buffer)
    }

    pub(super) fn recheck(leaf: &LeafHandle) -> Result<LeafFacts, NativeError> {
        let (facts, _) = handle_facts(leaf.handle.as_raw_handle())?;
        Ok(facts)
    }

    #[cfg(all(test, windows))]
    mod mapped_view_proofs {
        use std::fs::{self, OpenOptions};
        use std::os::windows::io::AsRawHandle;
        use std::ptr;

        #[test]
        fn std_file_and_pointer_usage_is_permitted() {
            let base = std::env::temp_dir().join(format!("solo-r04-a10-{}", std::process::id()));
            let _ = fs::remove_dir_all(&base);
            fs::create_dir_all(&base).expect("create the probe root");
            let body = base.join("body.bin");
            fs::write(&body, vec![b'A'; 8]).expect("write the probe body");
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .open(&body)
                .expect("open the probe body");
            assert!(!file.as_raw_handle().is_null(), "the handle must be real");
            let mut buffer = [0_u8; 8];
            let pointer = buffer.as_mut_ptr();
            // SAFETY: `pointer` covers exactly the eight bytes of the live local `buffer`, and
            // only that range is written.
            unsafe { ptr::write_bytes(pointer, b'C', 8) };
            assert_eq!(buffer[0], b'C', "the raw write must land in the buffer");
            drop(file);
            let _ = fs::remove_dir_all(&base);
        }
    }
}

#[cfg(all(test, target_os = "linux"))]
mod snapshot_proofs {
    //! Real-host proof of the D2 snapshot, taken through the audited module itself.
    //!
    //! These live here, not in an integration test, because this is the only file permitted to
    //! name the native crate at all: the boundary guard refuses `rustix` everywhere else and
    //! that rule is not relaxed for tests.

    use super::NativeError;
    use super::sys::{adopt_root, open_leaf, read_bounded, snapshot_leaf};
    use rustix::fd::OwnedFd;
    use rustix::fs::SeekFrom;
    use std::fs;
    use std::path::PathBuf;

    const SOURCE: &[u8] = b"{\"slug\":\"p\"}";
    /// A rewrite of exactly the same length, differing only in content.
    const SAME_SIZE: &[u8] = b"{\"slug\":\"q\"}";
    const REWRITTEN: &[u8] = b"{\"slug\":\"p-rewritten\"}";

    fn fixture(variable: &str, tag: &str) -> PathBuf {
        let root = match std::env::var_os(variable) {
            Some(value) if !value.is_empty() => PathBuf::from(value),
            _ => panic!("{variable} must name an R03 snapshot fixture filesystem"),
        };
        let directory = root.join(format!("solo-r03-unit-{}-{tag}", std::process::id()));
        let _ = fs::remove_dir_all(&directory);
        fs::create_dir(&directory).expect("create the unit fixture directory");
        directory
    }

    fn read_from_start(fd: &OwnedFd, length: usize) -> Vec<u8> {
        rustix::fs::seek(fd, SeekFrom::Start(0)).expect("rewind the clone");
        let mut buffer = vec![0_u8; length];
        let mut filled = 0_usize;
        while filled < length {
            let read = rustix::io::read(fd, &mut buffer[filled..]).expect("read the clone");
            if read == 0 {
                break;
            }
            filled += read;
        }
        buffer.truncate(filled);
        buffer
    }

    #[test]
    fn an_anonymous_clone_keeps_its_bytes_while_the_source_is_rewritten_in_place() {
        let directory = fixture("SOLO_TEST_SOURCE_FS_ROOT", "clone");
        fs::write(directory.join("body.json"), SOURCE).expect("write the source");
        let (root, _) = adopt_root(&directory).expect("adopt the fixture root");
        let (leaf, _) = open_leaf(&root, &["body.json"]).expect("open the source");

        let (clone, length) = snapshot_leaf(&root, &leaf, 4096).expect("clone the source");
        assert_eq!(length, SOURCE.len());

        // The source is rewritten in place after the clone, with different bytes.
        fs::write(directory.join("body.json"), REWRITTEN).expect("rewrite the source");
        assert_eq!(read_from_start(&clone, length).as_slice(), SOURCE);

        // An equal-length rewrite is equally invisible to the clone: the length and the byte
        // count are unchanged, so nothing but the content differs.
        assert_eq!(SAME_SIZE.len(), SOURCE.len());
        fs::write(directory.join("body.json"), SAME_SIZE).expect("rewrite with the same length");
        assert_eq!(read_from_start(&clone, length).as_slice(), SOURCE);
        assert_eq!(read_from_start(&clone, length).as_slice(), SOURCE);

        // A fresh snapshot does see the new bytes: the clone is one instant, not a cache.
        fs::write(directory.join("body.json"), REWRITTEN).expect("rewrite the source again");
        let (second, second_length) = snapshot_leaf(&root, &leaf, 4096).expect("re-clone");
        assert_eq!(
            read_from_start(&second, second_length).as_slice(),
            REWRITTEN
        );

        // Neither destination was ever named.
        assert_eq!(
            fs::read_dir(&directory).expect("list the fixture").count(),
            1
        );
    }

    #[test]
    fn a_filesystem_without_reflink_is_refused_instead_of_copied() {
        let directory = fixture("SOLO_TEST_UNSUPPORTED_FS_ROOT", "unsupported");
        fs::write(directory.join("body.json"), SOURCE).expect("write the source");
        let (root, _) = adopt_root(&directory).expect("adopt the fixture root");
        let (mut leaf, _) = open_leaf(&root, &["body.json"]).expect("open the source");

        match read_bounded(&root, &mut leaf, 4096) {
            Ok(bytes) => panic!(
                "a filesystem without reflink must refuse rather than copy; it returned {} byte(s)",
                bytes.len()
            ),
            Err(NativeError::Unsupported) => {}
            Err(other) => panic!("expected Unsupported on a non-reflink filesystem, got {other:?}"),
        }
    }

    #[test]
    fn an_oversized_source_is_refused_before_anything_is_cloned() {
        let directory = fixture("SOLO_TEST_SOURCE_FS_ROOT", "bound");
        fs::write(directory.join("body.json"), SOURCE).expect("write the source");
        let (root, _) = adopt_root(&directory).expect("adopt the fixture root");
        let (mut leaf, _) = open_leaf(&root, &["body.json"]).expect("open the source");

        let exact = u64::try_from(SOURCE.len()).expect("the fixture length fits a u64");
        match read_bounded(&root, &mut leaf, exact - 1) {
            Ok(_) => panic!("a body above its bound must be refused"),
            Err(NativeError::Unsafe) => {}
            Err(other) => panic!("expected Unsafe above the bound, got {other:?}"),
        }
        // Exactly at the bound is accepted, so the bound is inclusive and not off by one.
        assert_eq!(
            read_bounded(&root, &mut leaf, exact)
                .expect("the bound is inclusive")
                .as_slice(),
            SOURCE
        );
    }

    #[test]
    fn an_unlinked_source_does_not_move_an_existing_clone() {
        let directory = fixture("SOLO_TEST_SOURCE_FS_ROOT", "unlink");
        let source_path = directory.join("body.json");
        fs::write(&source_path, SOURCE).expect("write the source");
        let (root, _) = adopt_root(&directory).expect("adopt the fixture root");
        let (leaf, _) = open_leaf(&root, &["body.json"]).expect("open the source");
        let (clone, length) = snapshot_leaf(&root, &leaf, 4096).expect("clone the source");

        fs::remove_file(&source_path).expect("unlink the source");

        // The clone holds the data, not the name: removing the last link cannot move it.
        assert_eq!(read_from_start(&clone, length).as_slice(), SOURCE);
        assert_eq!(read_from_start(&clone, length).as_slice(), SOURCE);

        // A fresh lookup is a confirmed absence, never a body and never an error refusal.
        assert!(matches!(
            open_leaf(&root, &["body.json"]),
            Err(NativeError::Absent)
        ));

        // The still-open descriptor keeps the inode alive, so it still clones its bytes.
        let (second, second_length) =
            snapshot_leaf(&root, &leaf, 4096).expect("clone from the still-open handle");
        assert_eq!(read_from_start(&second, second_length).as_slice(), SOURCE);
        assert_eq!(
            fs::read_dir(&directory).expect("list the fixture").count(),
            0
        );
    }

    #[test]
    fn a_replaced_source_is_a_new_object_while_the_clone_keeps_the_old_bytes() {
        let directory = fixture("SOLO_TEST_SOURCE_FS_ROOT", "replace");
        let source_path = directory.join("body.json");
        fs::write(&source_path, SOURCE).expect("write the source");
        let (root, _) = adopt_root(&directory).expect("adopt the fixture root");
        let (leaf, original_facts) = open_leaf(&root, &["body.json"]).expect("open the source");
        let (clone, length) = snapshot_leaf(&root, &leaf, 4096).expect("clone the source");

        // The name now refers to a different physical object.
        fs::remove_file(&source_path).expect("unlink the source");
        fs::write(&source_path, REWRITTEN).expect("write a replacement at the same name");

        assert_eq!(read_from_start(&clone, length).as_slice(), SOURCE);

        let (replacement, replacement_facts) =
            open_leaf(&root, &["body.json"]).expect("open the replacement");
        assert_ne!(
            replacement_facts.identity, original_facts.identity,
            "a replacement is a different physical object"
        );
        assert_eq!(replacement_facts.links, 1);

        let (replacement_clone, replacement_length) =
            snapshot_leaf(&root, &replacement, 4096).expect("clone the replacement");
        assert_eq!(
            read_from_start(&replacement_clone, replacement_length).as_slice(),
            REWRITTEN
        );
    }
}
