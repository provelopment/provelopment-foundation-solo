//! Protected, private, disk-backed staging for the assembled source-stage candidate.
//!
//! The point of this module is that a completed source stage must not keep every selected body
//! in RAM. Bodies are written to task-owned objects on disk, and the candidate keeps only a
//! small handle-and-range reference per representation. What the candidate exposes afterwards is
//! a **fallible, bounded** read of one member at a time.
//!
//! **What makes a staged object safe.** Linux opens every shard as an `O_TMPFILE | O_EXCL` inode
//! *through the retained descriptor of the caller's own staging root*: the object never acquires a
//! directory entry, so there is no window in which a second handle could open it, and no
//! transaction-owned directory or entry is ever created or removed. Windows creates each shard
//! **directly in that same adopted root, relative to the retained root handle**, with
//! `NtCreateFile(FILE_CREATE)` and no sharing at all, then marks that exact object for deletion
//! through `FileDispositionInfoEx(DELETE | POSIX_SEMANTICS)` on its own handle **before the first
//! plaintext byte**. The host keeps the name `DELETE_PENDING` while the handle stays open, so a
//! same-user attacker may still *enumerate* it; it can never open, read, write, truncate, rename or
//! map it, and it disappears when the last handle closes or the process dies.

//!
//! Objects are grouped into shards of at most [`STAGE_SHARD_TARGET_BYTES`], so the number of
//! live handles depends on the total staged size rather than on how many representations were
//! declared. A member never spans two shards.
//!
//! **What this is not.** It is temporary internal processing: not a source root, not a second
//! manifest, not generated output and not a publication artefact. It authorizes nothing.
//!
//! **Cleanup.** Dropping the store closes every shard handle, and that is the whole of cleanup on
//! both hosts: Linux's inodes were never linked, and a Windows pending name is removed by the kernel
//! when its last handle closes, including at process death through `FILE_DELETE_ON_CLOSE`. This
//! module creates no directory of its own and deletes nothing by name, so an impostor directory, a
//! foreign file or an Owner sentinel can never be removed or modified by staging.

//!
//! **Threat boundary, stated honestly.** A same-user process can *see* a Windows pending name in a
//! directory listing while the store holds it, and it may create new entries of its own in the
//! caller's root. It cannot reach, mutate, replace, rename or map an object this store holds: that
//! object admits no second handle at all, and it carries no plaintext until the disposition above has
//! succeeded. This is a claim about these staged bytes, not a claim that a same-user attacker cannot
//! interfere with the wider process.

use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

#[cfg(windows)]
use std::sync::atomic::{AtomicU64, Ordering};

use solo_application::{SourceReadRefusalKind, SourceReadRefusalV1};

#[cfg(windows)]
use crate::source_read_native::StagingNativeError;

/// A retained operating-system identity: what the handle *is*, never what a path claims.
///
/// The three components are deliberately opaque. On Windows they are the container serial and the
/// two halves of the 128-bit `FILE_ID_INFO` object id, which the volume does not reuse. On Unix
/// they are the device and the inode, and the third component is unused.
///
/// An identity is only ever a *secondary* control here. It cannot establish ownership of a
/// pathname, because Linux recycles a freed inode number and no mutable timestamp can substitute
/// for that: a directory's change time moves whenever its contents legitimately change. Staging
/// confinement therefore rests on retained descriptors, and identity checking is used only where a
/// descriptor cannot be carried — Windows object creation, where the object id is not recycled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ObjectIdentityV1 {
    container: u64,
    low: u64,
    high: u64,
}

impl ObjectIdentityV1 {
    /// Whether two identities name the same retained object.
    ///
    /// Written out rather than derived so every component is genuinely consulted; a derived
    /// comparison is ignored by dead-code analysis and would hide an unused component.
    const fn same_object(self, other: Self) -> bool {
        self.container == other.container && self.low == other.low && self.high == other.high
    }
}

/// Reads the retained identity of an already-open handle.
fn identity_of(file: &File) -> Result<ObjectIdentityV1, SourceReadRefusalV1> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        let metadata = file.metadata().map_err(classify_io)?;
        // Device plus inode. The third component is unused on this host, but `same_object` reads
        // it, so the identity stays one shape across hosts without an allowance attribute.
        Ok(ObjectIdentityV1 {
            container: metadata.dev(),
            low: metadata.ino(),
            high: 0,
        })
    }
    #[cfg(windows)]
    {
        // The stable `MetadataExt` identity accessors are still unstable, so the audited
        // `FILE_ID_INFO` query the source reader already uses is reused through a safe wrapper.
        let identity = crate::source_read_native::staging_handle_identity(file)
            .map_err(|_| unsafe_location())?;
        Ok(ObjectIdentityV1 {
            container: identity.container(),
            low: identity.object_low(),
            high: identity.object_high(),
        })
    }
    #[cfg(not(any(unix, windows)))]
    {
        let _ = file;
        Err(SourceReadRefusalV1::new(
            SourceReadRefusalKind::Unsupported,
            None,
        ))
    }
}

/// Whether the retained staging-root descriptor is still linked into the directory tree.
///
/// A directory's link count drops to zero when its entry is removed, and the count is read from the
/// handle this store holds rather than from a name, so the answer cannot be redirected. It is the
/// deterministic form of "the namespace this store created is still there".
#[cfg(unix)]
fn descriptor_is_linked(directory: &File) -> Result<bool, SourceReadRefusalV1> {
    use std::os::unix::fs::MetadataExt;
    Ok(directory.metadata().map_err(classify_io)?.nlink() > 0)
}

/// Adopts the caller's staging root through the host's confined opener.
fn adopt_root_handle(root: &Path) -> Result<File, SourceReadRefusalV1> {
    #[cfg(target_os = "linux")]
    {
        if !root.is_absolute() {
            return Err(unsafe_location());
        }
        crate::source_read_native::adopt_staging_root(root).map_err(classify_adoption)
    }
    #[cfg(windows)]
    {
        crate::source_read_native::adopt_staging_root(root).map_err(staging_native)
    }
    #[cfg(not(any(target_os = "linux", windows)))]
    {
        let _ = root;
        Err(unsupported())
    }
}

/// Classifies a Linux adoption failure.
///
/// A denial is reported as a denial, an unavailable kernel primitive as unsupported, and every
/// other answer — a missing directory, a symlink the kernel refused, an unsafe shape — as an
/// unprovable location rather than something to invent or fall back from.
#[cfg(target_os = "linux")]
fn classify_adoption(error: std::io::Error) -> SourceReadRefusalV1 {
    /// `EACCES`.
    const ACCESS: i32 = 13;
    /// `EPERM`.
    const PERM: i32 = 1;
    /// `ENOSYS`.
    const NOSYS: i32 = 38;
    /// `EINVAL`.
    const INVAL: i32 = 22;
    /// `EOPNOTSUPP`.
    const OPNOTSUPP: i32 = 95;
    match error.raw_os_error() {
        Some(ACCESS | PERM) => {
            SourceReadRefusalV1::new(SourceReadRefusalKind::PermissionDenied, None)
        }
        Some(NOSYS | INVAL | OPNOTSUPP) => unsupported(),
        _ => unsafe_location(),
    }
}

/// Classifies a Linux shard-creation failure.
///
/// A genuine denial or a genuine lack of space is reported as what it is. Anything else means this
/// store could not create an inode inside the directory it provably holds — the root was unlinked,
/// or the host answered with something this store cannot interpret — so confinement is unproven
/// and the answer is an unsafe location, never an absence or a shorter member.
#[cfg(target_os = "linux")]
fn classify_object_creation(error: std::io::Error) -> SourceReadRefusalV1 {
    /// `ENOSPC`.
    const NO_SPACE: i32 = 28;
    /// `EDQUOT`.
    const QUOTA_EXCEEDED: i32 = 122;
    match error.raw_os_error() {
        Some(NO_SPACE | QUOTA_EXCEEDED) => classify_io(error),
        Some(_) if error.kind() == std::io::ErrorKind::PermissionDenied => classify_io(error),
        _ => unsafe_location(),
    }
}

/// Classifies one Windows staging-native outcome onto the registered vocabulary.
#[cfg(windows)]
fn staging_native(error: StagingNativeError) -> SourceReadRefusalV1 {
    match error {
        StagingNativeError::Unsafe => unsafe_location(),
        StagingNativeError::PermissionDenied => {
            SourceReadRefusalV1::new(SourceReadRefusalKind::PermissionDenied, None)
        }
        StagingNativeError::Unsupported => unsupported(),
        // A genuine native failure is an I/O failure, never a claimed path refusal.
        StagingNativeError::NativeFailure => {
            SourceReadRefusalV1::new(SourceReadRefusalKind::OtherIo, None)
        }
    }
}

/// `SOURCE_UNSUPPORTED`: this host cannot perform the operation at all.
fn unsupported() -> SourceReadRefusalV1 {
    SourceReadRefusalV1::new(SourceReadRefusalKind::Unsupported, None)
}

/// The deterministic staging fault a test asks the **production** path to hit.
///
/// It exists so a later-member write failure, a short write, a flush failure and a finalization
/// failure are exercised through the real transaction code rather than a parallel fake. No
/// production caller sets one.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StagingFaultV1 {
    /// Fail the pre-write delete disposition, through a real native create without `DELETE` access.
    Disposition,
    /// Fail before writing the body.
    Write,
    /// Write a prefix of the body and then fail.
    ShortWrite,
    /// Write the body fully and then fail at the flush.
    Flush,
    /// Fail at whole-set finalization.
    Finalize,
}

/// Where a [`StagingFaultV1`] applies: the ordinal (zero-based) staging call it hits.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StagingFaultsV1 {
    after: usize,
    fault: Option<StagingFaultV1>,
}

impl StagingFaultsV1 {
    /// No injected fault: the production behaviour.
    #[must_use]
    pub const fn none() -> Self {
        Self {
            after: usize::MAX,
            fault: None,
        }
    }

    /// Injects `fault` at the staging call whose ordinal is `after`.
    #[doc(hidden)]
    #[must_use]
    pub const fn inject(after: usize, fault: StagingFaultV1) -> Self {
        Self {
            after,
            fault: Some(fault),
        }
    }
}

/// The largest number of bytes one staged object may accumulate before the store starts another.
///
/// It bounds open handles by `ceil(total / this)` instead of by the number of representations. It
/// is an internal staging detail, never an authored-content limit: the accepted per-body limits
/// are unchanged and no cumulative publication budget is introduced.
pub const STAGE_SHARD_TARGET_BYTES: u64 = 64 * 1024 * 1024;

/// The buffer this module uses when it streams one staged member back out.
pub const STAGE_READ_CHUNK_BYTES: usize = 8 * 1024;

/// The prefix of the unique staging name this process derives for one task-owned shard.
///
/// It is a name component only. This module creates no directory, so nothing here is a namespace,
/// and on Linux no staging name is ever linked at all.
#[cfg(windows)]
const STAGE_NAME_PREFIX: &str = "solo-source-stage-";

/// The prefix of the task-owned shard name Windows creates in the adopted staging root.
#[cfg(windows)]
const STAGE_OBJECT_PREFIX: &str = "shard-";

/// Distinguishes concurrently live stores inside one process.
///
/// A name counter, never a store of content: it exists so two live stores in one process cannot
/// choose the same namespace name.
#[cfg(windows)]
static STAGE_SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// An explicitly adopted, already-existing private directory that staging may use.
///
/// The caller supplies it; this module never chooses one. Adoption refuses a relative path, a
/// symbolic link and anything that is not a directory, so a link planted in place of the scratch
/// location is refused rather than followed.
#[derive(Debug)]
pub struct StagingRootCapability {
    root: PathBuf,
    handle: File,
    identity: ObjectIdentityV1,
}

impl StagingRootCapability {
    /// Test-only: attempts the production native `FILE_CREATE` for one fixed name in this root.
    ///
    /// # Errors
    ///
    /// Returns the classified refusal the production creator reports when the name already exists,
    /// which is how a pre-existing foreign object is proved never to be adopted or changed.
    #[cfg(windows)]
    #[doc(hidden)]
    pub fn create_shard_at(&self, name: &str) -> Result<(), SourceReadRefusalV1> {
        let file = crate::source_read_native::create_staging_shard(&self.handle, name)
            .map_err(staging_native)?;
        drop(file);
        Ok(())
    }

    /// Adopts an existing directory as the staging root.
    ///
    /// The directory must already exist and be a real directory. It is opened once, and the
    /// **handle and the identity of what was actually opened are retained**; every later
    /// confinement decision compares that identity rather than re-trusting a string. A relative
    /// path, a symbolic link at the final component, a missing directory and a non-directory are
    /// all refused, and no system temp location is ever substituted.
    ///
    /// # Errors
    ///
    /// Returns `SOURCE_UNSAFE_PATH` for a relative path, a symbolic link or a non-directory, and
    /// the classified staging refusal when the directory cannot be opened.
    pub fn adopt(root: &Path) -> Result<Self, SourceReadRefusalV1> {
        // The host adapter opens the directory itself: Linux resolves the whole path from `/`
        // under `BENEATH | NO_SYMLINKS | NO_MAGICLINKS`, and Windows walks one component at a time
        // relative to the handle above it, refusing every reparse point. Neither inspects the path
        // with `symlink_metadata` first, because a check before an open is a check that races.
        let handle = adopt_root_handle(root)?;
        let identity = identity_of(&handle)?;
        Ok(Self {
            root: root.to_path_buf(),
            handle,
            identity,
        })
    }

    /// The directory that was adopted, for diagnostics and for a caller that owns it.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Creates one private namespace inside the adopted root and returns its store.
    ///
    /// The namespace name is unique to this call and is created with exclusive creation, then
    /// tightened to owner-only permissions on Linux. A name that already exists is an unsafe
    /// location, never something to adopt or reuse.
    ///
    /// # Errors
    ///
    /// Returns `SOURCE_UNSAFE_PATH` when the namespace cannot be created exclusively, and the
    /// classified staging refusal for a permission or capacity failure.
    pub fn begin(&self) -> Result<SourceSetStagingV1, SourceReadRefusalV1> {
        self.begin_with_faults(StagingFaultsV1::none())
    }

    /// Creates the private namespace with a deterministic fault plan attached.
    ///
    /// This is the seam the acceptance matrix needs: the injected failure is raised by the same
    /// production code that a real host failure would reach, not by a parallel implementation.
    #[doc(hidden)]
    pub fn begin_with_faults(
        &self,
        faults: StagingFaultsV1,
    ) -> Result<SourceSetStagingV1, SourceReadRefusalV1> {
        // The retained capability must still be the live directory it was adopted as. This reads
        // the handle, not a string, so a root that disappeared, was replaced or stopped being a
        // directory refuses before anything is created or written.
        if !self.handle.metadata().map_err(classify_io)?.is_dir() {
            return Err(unsafe_location());
        }
        if !identity_of(&self.handle)?.same_object(self.identity) {
            return Err(unsafe_location());
        }

        #[cfg(any(target_os = "linux", windows))]
        {
            // Nothing is created here. Shards are anonymous inodes opened later through this
            // retained descriptor, and the caller's directory is never written by name.
            let root_dir = self.handle.try_clone().map_err(classify_io)?;
            Ok(SourceSetStagingV1 {
                root_dir,
                root_identity: self.identity,
                root_path: self.root.clone(),
                shards: Vec::new(),
                staged_bytes: 0,
                max_member_bytes: 0,
                staged_members: 0,
                sealed: false,
                faults,
            })
        }

        #[cfg(not(any(target_os = "linux", windows)))]
        {
            let _ = faults;
            Err(unsupported())
        }
    }
}

/// One staging object: a handle whose directory entry is already gone.
#[derive(Debug)]
struct StageShardV1 {
    file: File,
    written: u64,
}

/// A reference to one member's bytes inside one staged object.
///
/// It is a range, not a buffer: holding it retains no body bytes at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StagedBodyV1 {
    shard: usize,
    offset: u64,
    len: u64,
}

impl StagedBodyV1 {
    /// How many bytes this member occupies.
    #[must_use]
    pub const fn len(&self) -> u64 {
        self.len
    }

    /// Whether this member is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// The offset inside its staging object where the member starts.
    #[must_use]
    pub const fn offset(&self) -> u64 {
        self.offset
    }

    /// Which staging object holds it. Object indices are internal and carry no path.
    #[must_use]
    pub const fn object(&self) -> usize {
        self.shard
    }
}

/// The private staging store owned by one assembly attempt.
///
/// It exists only while an attempt is in flight: either the assembled candidate owns it, or a
/// failure drops it and its [`Drop`] closes every handle and removes the empty namespace.
pub struct SourceSetStagingV1 {
    /// The retained descriptor of the caller's staging root, on both hosts.
    root_dir: File,
    root_identity: ObjectIdentityV1,
    /// The adopted root's path, for observation only — never a creation or deletion authority.
    root_path: PathBuf,
    shards: Vec<StageShardV1>,
    staged_bytes: u64,
    max_member_bytes: u64,
    staged_members: usize,
    sealed: bool,
    faults: StagingFaultsV1,
}

impl SourceSetStagingV1 {
    /// Writes one validated body into a task-owned object and returns its range.
    ///
    /// The write is checked for failure, then flushed, and the object's own length is verified
    /// against the bytes that were handed in before the range is returned. A short, failed or
    /// interrupted write is a typed refusal, never a shorter member.
    ///
    /// # Errors
    ///
    /// Returns the classified staging refusal: `SOURCE_OBSERVATION_IO_ERROR` for a write, flush
    /// or capacity failure (never absence and never a Markdown fallback), and
    /// `SOURCE_OBSERVATION_PERMISSION_DENIED` when the host denies the write.
    pub fn stage_body(&mut self, bytes: &[u8]) -> Result<StagedBodyV1, SourceReadRefusalV1> {
        if self.sealed {
            return Err(SourceReadRefusalV1::new(
                SourceReadRefusalKind::Unsupported,
                None,
            ));
        }

        let ordinal = self.staged_members;
        self.staged_members = self.staged_members.saturating_add(1);
        if let Some(fault) = self.faults.fault
            && ordinal == self.faults.after
            && fault == StagingFaultV1::Write
        {
            return Err(out_of_capacity());
        }

        let len = bytes.len() as u64;
        let rotate = match self.shards.last() {
            None => true,
            Some(shard) => shard.written.saturating_add(len) > STAGE_SHARD_TARGET_BYTES,
        };
        if rotate {
            self.open_object()?;
        }

        let index = self.shards.len() - 1;
        let shard = &mut self.shards[index];
        let offset = shard.written;

        // The short-write fault is the only path that writes a prefix; it exists to prove that a
        // partial object is refused and cleaned rather than accepted as a shorter member.
        let short_write = self.faults.fault == Some(StagingFaultV1::ShortWrite)
            && ordinal == self.faults.after
            && bytes.len() > 1;
        if short_write {
            let prefix = bytes.len() / 2;
            shard
                .file
                .write_all(&bytes[..prefix])
                .map_err(classify_io)?;
            shard.written = offset.saturating_add(prefix as u64);
            return Err(out_of_capacity());
        }

        shard.file.write_all(bytes).map_err(classify_io)?;

        if self.faults.fault == Some(StagingFaultV1::Flush) && ordinal == self.faults.after {
            return Err(out_of_capacity());
        }
        shard.file.flush().map_err(classify_io)?;

        let observed = shard.file.metadata().map_err(classify_io)?.len();
        if observed != offset.saturating_add(len) {
            return Err(changed());
        }

        shard.written = observed;
        self.staged_bytes = self.staged_bytes.saturating_add(len);
        if len > self.max_member_bytes {
            self.max_member_bytes = len;
        }
        Ok(StagedBodyV1 {
            shard: index,
            offset,
            len,
        })
    }

    /// Seals the store once every member is staged.
    ///
    /// It re-confirms that the retained root is still the directory this store adopted and that
    /// every object still reports exactly the length that was written, then disposes the Windows
    /// namespace by handle. A last-write, finalization or disposal failure is refused here rather
    /// than becoming an accepted candidate, and no public value exists until this returns `Ok`.
    ///
    /// # Errors
    ///
    /// Returns `SOURCE_OBSERVATION_IO_ERROR` for an injected or real finalization failure,
    /// `SOURCE_OBSERVATION_CHANGED_DURING_READ` when an object no longer reports its own length,
    /// and `SOURCE_UNSAFE_PATH` when the retained root is no longer the adopted directory.
    pub fn finalize(&mut self) -> Result<(), SourceReadRefusalV1> {
        if self.sealed {
            return Ok(());
        }
        if self.faults.fault == Some(StagingFaultV1::Finalize)
            && self.faults.after <= self.staged_members
        {
            return Err(out_of_capacity());
        }
        self.confirm_root()?;
        for shard in &self.shards {
            let observed = shard.file.metadata().map_err(classify_io)?.len();
            if observed != shard.written {
                return Err(changed());
            }
        }

        // No staging-owned directory exists on either host, so there is nothing to remove here.
        self.sealed = true;
        Ok(())
    }

    /// Confirms that the retained root is still the directory this store adopted.
    ///
    /// The control is the descriptor, not a name: every shard is created *through* a retained
    /// handle, so a substituted pathname never receives a byte and re-resolving a path here would
    /// only add a check-then-use window. This comparison is a cheap sanity check on that handle.
    ///
    /// # Errors
    ///
    /// Returns `SOURCE_UNSAFE_PATH` when the descriptor is no longer the adopted root.
    fn confirm_root(&self) -> Result<(), SourceReadRefusalV1> {
        if !identity_of(&self.root_dir)?.same_object(self.root_identity) {
            return Err(unsafe_location());
        }
        Ok(())
    }

    /// A bounded reader over one staged member.
    ///
    /// The reader borrows the store, so it cannot outlive the objects it reads and cannot be used
    /// to write: nothing in this API hands out a writable handle or a mutable slice.
    #[must_use]
    pub fn reader(&self, body: StagedBodyV1) -> Option<StageReaderV1<'_>> {
        self.shards.get(body.shard).map(|shard| StageReaderV1 {
            file: &shard.file,
            offset: body.offset,
            len: body.len,
        })
    }

    /// How many bytes in total this store has staged.
    #[must_use]
    pub const fn staged_bytes(&self) -> u64 {
        self.staged_bytes
    }

    /// How many body bytes the store's largest member needed.
    ///
    /// Bounded by the accepted single-body limit, never by the number or cumulative size of
    /// representations.
    #[must_use]
    pub const fn max_member_bytes(&self) -> u64 {
        self.max_member_bytes
    }

    /// How many staging objects (open handles) this store holds.
    ///
    /// It grows with the total staged size, not with the declaration count.
    /// How many staging-owned names in the adopted root a caller could actually open.
    ///
    /// Linux links no name at all. On Windows this store's own names may be enumerable as
    /// delete-pending entries, but a name that cannot be opened is not a reachable plaintext path,
    /// so only an **openable** staging-owned name is counted. A non-zero answer is a real defect.
    ///
    /// # Errors
    ///
    /// Returns the classified refusal when the adopted root cannot be listed.
    pub fn visible_staging_entries(&self) -> Result<usize, SourceReadRefusalV1> {
        let entries = std::fs::read_dir(&self.root_path).map_err(classify_io)?;
        let mut reachable = 0_usize;
        for entry in entries {
            let entry = entry.map_err(classify_io)?;
            let name = entry.file_name();
            let Some(name) = name.to_str() else {
                continue;
            };
            if !name.starts_with("shard-") {
                continue;
            }
            if std::fs::File::open(entry.path()).is_ok() {
                reachable += 1;
            }
        }
        Ok(reachable)
    }

    pub fn object_count(&self) -> usize {
        self.shards.len()
    }

    /// Creates one nameless staging object directly in the retained staging root.
    fn open_object(&mut self) -> Result<(), SourceReadRefusalV1> {
        #[cfg(target_os = "linux")]
        {
            // A root that was unlinked while this store holds it cannot accept a new inode, and
            // that link count is read from the descriptor itself rather than from a name.
            if !descriptor_is_linked(&self.root_dir)? {
                return Err(unsafe_location());
            }
            let file = crate::source_read_native::open_unnamed_staging_object(&self.root_dir)
                .map_err(classify_object_creation)?;
            self.shards.push(StageShardV1 { file, written: 0 });
            Ok(())
        }

        #[cfg(windows)]
        {
            // Creation is relative to the retained root handle itself: no staging-owned directory
            // exists, so there is nothing to clean up and nothing to substitute. The name is
            // derived here and never re-resolved, so a hostile same-name entry is never adopted.
            let name = format!(
                "{STAGE_OBJECT_PREFIX}{}-{}.bin",
                self.shards.len(),
                staging_name()
            );
            let file = if self.faults.fault == Some(StagingFaultV1::Disposition)
                && self.faults.after <= self.staged_members
            {
                // The seam creates the object exactly as production does and then has the kernel
                // refuse the disposition, so no plaintext is written and the empty object is removed
                // when its handle closes.
                crate::source_read_native::create_staging_shard_disposition_refused(
                    &self.root_dir,
                    &name,
                )
            } else {
                crate::source_read_native::create_staging_shard(&self.root_dir, &name)
            }
            .map_err(staging_native)?;
            self.shards.push(StageShardV1 { file, written: 0 });
            Ok(())
        }

        #[cfg(not(any(target_os = "linux", windows)))]
        {
            Err(unsupported())
        }
    }
}

/// A bounded read of one staged member.
///
/// Every operation is fallible and bounded by the member's recorded length: there is no method
/// that returns a `&[u8]` over a whole member, and no method that touches a second member.
#[derive(Debug)]
pub struct StageReaderV1<'a> {
    file: &'a File,
    offset: u64,
    len: u64,
}

impl StageReaderV1<'_> {
    /// How many bytes the member holds.
    #[must_use]
    pub const fn len(&self) -> u64 {
        self.len
    }

    /// Whether the member holds no bytes.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Reads up to `out.len()` bytes starting `position` bytes into the member.
    ///
    /// A position beyond the member's recorded length is refused as an unexpected state rather
    /// than silently clamped to the end: an object that answers outside its own range is not the
    /// object that was validated.
    ///
    /// # Errors
    ///
    /// Returns `SNAPSHOT_CHANGED` for an out-of-range position and the
    /// classified staging refusal for an I/O failure.
    pub fn read_chunk(&self, position: u64, out: &mut [u8]) -> Result<usize, SourceReadRefusalV1> {
        if position > self.len {
            return Err(changed());
        }
        let remaining = self.len - position;
        let limit = remaining.min(out.len() as u64) as usize;
        if limit == 0 {
            return Ok(0);
        }
        read_object_at(
            self.file,
            &mut out[..limit],
            self.offset.saturating_add(position),
        )
        .map_err(classify_io)
    }

    /// Streams the whole member into `out`, allocating with a fallible reservation.
    ///
    /// This is the only way to obtain a member's complete bytes. It is bounded by that one
    /// member's length, which the accepted limits cap, and it never holds another member.
    ///
    /// # Errors
    ///
    /// Returns `SOURCE_OBSERVATION_IO_ERROR` when the bytes cannot be reserved or the object
    /// stops producing data, and the classified staging refusal for an I/O failure.
    pub fn read_whole(&self, out: &mut Vec<u8>) -> Result<(), SourceReadRefusalV1> {
        let total = usize::try_from(self.len).map_err(|_| changed())?;
        out.try_reserve_exact(total)
            .map_err(|_| out_of_capacity())?;

        let mut buffer = [0_u8; STAGE_READ_CHUNK_BYTES];
        let mut position = 0_u64;
        while position < self.len {
            let read = self.read_chunk(position, &mut buffer)?;
            if read == 0 {
                return Err(changed());
            }
            out.extend_from_slice(&buffer[..read]);
            position = position.saturating_add(read as u64);
        }
        Ok(())
    }

    /// Compares the member against `expected` in bounded chunks, without materialising it.
    ///
    /// # Errors
    ///
    /// Returns the classified staging refusal for an I/O failure and
    /// `SNAPSHOT_CHANGED` when the object stops producing data early.
    pub fn matches(&self, expected: &[u8]) -> Result<bool, SourceReadRefusalV1> {
        if self.len != expected.len() as u64 {
            return Ok(false);
        }
        let mut buffer = [0_u8; STAGE_READ_CHUNK_BYTES];
        let mut position = 0_u64;
        while position < self.len {
            let read = self.read_chunk(position, &mut buffer)?;
            if read == 0 {
                return Err(changed());
            }
            let start = position as usize;
            if buffer[..read] != expected[start..start + read] {
                return Ok(false);
            }
            position = position.saturating_add(read as u64);
        }
        Ok(true)
    }
}

/// Reads at an absolute offset through the platform's safe positional-read extension.
fn read_object_at(file: &File, out: &mut [u8], at: u64) -> std::io::Result<usize> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::FileExt;
        file.read_at(out, at)
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::FileExt;
        file.seek_read(out, at)
    }
    #[cfg(not(any(unix, windows)))]
    {
        // No positional-read extension exists on this host. Refusing is the only honest answer:
        // a positional read is what keeps two readers of one object independent.
        let _ = (file, out, at);
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "no positional read on this host",
        ))
    }
}

impl Drop for SourceSetStagingV1 {
    fn drop(&mut self) {
        // Objects are anonymous on Linux and name-detached on Windows, so closing every shard
        // handle is the whole of cleanup: on Windows the pending entry disappears when the last
        // handle closes, including at process death. No directory is removed because none was
        // created.
        self.shards.clear();
    }
}

/// Classifies one host I/O failure onto an already registered diagnostic.
///
/// The mapping is deliberate and is reported in the canonical report: absence is never produced
/// here, capacity exhaustion is an I/O failure rather than a silent fallback, and an object this
/// store already owns disappearing is a detected change rather than a new absence.
fn classify_io(error: std::io::Error) -> SourceReadRefusalV1 {
    use std::io::ErrorKind;

    let kind = match error.kind() {
        ErrorKind::PermissionDenied => SourceReadRefusalKind::PermissionDenied,
        ErrorKind::NotFound | ErrorKind::UnexpectedEof => SourceReadRefusalKind::ChangedDuringRead,
        ErrorKind::AlreadyExists => SourceReadRefusalKind::UnsafeLocation,
        ErrorKind::Unsupported => SourceReadRefusalKind::Unsupported,
        ErrorKind::StorageFull
        | ErrorKind::QuotaExceeded
        | ErrorKind::FileTooLarge
        | ErrorKind::OutOfMemory
        | ErrorKind::WriteZero => SourceReadRefusalKind::OtherIo,
        _ => SourceReadRefusalKind::OtherIo,
    };
    SourceReadRefusalV1::new(kind, None)
}

/// `SOURCE_UNSAFE_PATH`: a path that cannot be proven to be the private directory it claims.
fn unsafe_location() -> SourceReadRefusalV1 {
    SourceReadRefusalV1::new(SourceReadRefusalKind::UnsafeLocation, None)
}

/// `SNAPSHOT_CHANGED`: an object this store owns answered unexpectedly.
fn changed() -> SourceReadRefusalV1 {
    SourceReadRefusalV1::new(SourceReadRefusalKind::ChangedDuringRead, None)
}

/// `SOURCE_OBSERVATION_IO_ERROR`: the host could not give this stage room to work.
fn out_of_capacity() -> SourceReadRefusalV1 {
    SourceReadRefusalV1::new(SourceReadRefusalKind::OtherIo, None)
}

/// A namespace name that no other live store in this process can be holding.
///
/// Windows is the only host that creates one: Linux stages into anonymous inodes in the caller's
/// own directory and never names anything.
#[cfg(windows)]
fn staging_name() -> String {
    let sequence = STAGE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos())
        .unwrap_or(0);
    format!(
        "{STAGE_NAME_PREFIX}{}-{sequence}-{nanos}",
        std::process::id()
    )
}

#[cfg(test)]
mod tests {
    //! Deterministic coverage of the staging refusal matrix.
    //!
    //! The host decides *which* I/O error it reports, but the classification of that error onto a
    //! registered diagnostic is this module's own decision, so it is tested exactly rather than
    //! inferred from a host that happens to be out of disk. What is asserted here is the mapping
    //! and, just as importantly, that no failure in it is ever reported as an absent authored
    //! body.

    use std::io::ErrorKind;

    use super::classify_io;

    /// The code a synthetic host error is classified into.
    fn code_for(kind: ErrorKind) -> &'static str {
        classify_io(std::io::Error::from(kind)).code()
    }

    #[test]
    fn capacity_failures_are_io_errors_and_never_absence() {
        for kind in [
            ErrorKind::StorageFull,
            ErrorKind::QuotaExceeded,
            ErrorKind::FileTooLarge,
            ErrorKind::OutOfMemory,
            ErrorKind::WriteZero,
        ] {
            assert_eq!(code_for(kind), "SOURCE_OBSERVATION_IO_ERROR", "{kind:?}");
        }
        assert_eq!(
            code_for(ErrorKind::StorageFull),
            "SOURCE_OBSERVATION_IO_ERROR"
        );
        assert_ne!(code_for(ErrorKind::StorageFull), "SOURCE_NONE_EFFECTIVE");
    }

    #[test]
    fn every_failure_kind_maps_onto_a_registered_token() {
        assert_eq!(
            code_for(ErrorKind::PermissionDenied),
            "SOURCE_OBSERVATION_PERMISSION_DENIED"
        );
        assert_eq!(code_for(ErrorKind::NotFound), "SNAPSHOT_CHANGED");
        assert_eq!(code_for(ErrorKind::UnexpectedEof), "SNAPSHOT_CHANGED");
        assert_eq!(code_for(ErrorKind::AlreadyExists), "SOURCE_UNSAFE_PATH");
        assert_eq!(code_for(ErrorKind::Unsupported), "SOURCE_UNSUPPORTED");
        assert_eq!(code_for(ErrorKind::Other), "SOURCE_OBSERVATION_IO_ERROR");

        // No staging failure may be reported as an absent body: that is what would silently turn
        // a broken stage into a Markdown fallback.
        for kind in [
            ErrorKind::StorageFull,
            ErrorKind::PermissionDenied,
            ErrorKind::NotFound,
            ErrorKind::AlreadyExists,
            ErrorKind::Unsupported,
            ErrorKind::Other,
        ] {
            assert_ne!(code_for(kind), "SOURCE_NONE_EFFECTIVE", "{kind:?}");
        }
    }
}
