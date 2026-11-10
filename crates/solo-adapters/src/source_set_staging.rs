//! Protected, private, disk-backed staging for the assembled source-stage candidate.
//!
//! The point of this module is that a completed source stage must not keep every selected body
//! in RAM. Bodies are written to task-owned objects on disk, and the candidate keeps only a
//! small handle-and-range reference per representation. What the candidate exposes afterwards is
//! a **fallible, bounded** read of one member at a time.
//!
//! **What makes a staged object safe without `unsafe`.** Each object is created with exclusive
//! creation inside a namespace this run created itself, and its directory entry is then
//! *removed while the handle stays open*: on Linux through ordinary unlink-while-open semantics,
//! on Windows because `std` opens files with `FILE_SHARE_DELETE`, so a delete-pending name
//! disappears immediately. From that moment the bytes are reachable only through the handle this
//! module owns. There is no path to replace, truncate, redirect or re-open, no memory mapping and
//! no second writable alias — which is exactly why no `unsafe`, no FFI and no new dependency is
//! needed to hold immutability across the candidate's lifetime.
//!
//! Objects are grouped into shards of at most [`STAGE_SHARD_TARGET_BYTES`], so the number of
//! live handles depends on the total staged size rather than on how many representations were
//! declared. A member never spans two shards.
//!
//! **What this is not.** It is temporary internal processing: not a source root, not a second
//! manifest, not generated output and not a publication artefact. It authorizes nothing.
//!
//! **Cleanup.** Dropping the store closes every handle and removes only the namespace directory
//! this run created, and only when it is empty. It never pre-cleans, never removes a directory
//! it does not own, never adopts a pre-existing one and never touches a shared OS temp root.
//! Because every object is nameless while it exists, an abruptly terminated process leaves
//! either nothing or an empty directory behind: there are no plaintext directory entries to
//! recover.
//!
//! **Threat boundary, stated honestly.** A same-user process that can reach this namespace may
//! create *new* entries inside it, but it cannot reach, mutate or replace an object this store
//! already holds, because that object has no name; it cannot make a sealed member read
//! differently. This is a claim about these staged bytes, not a claim that a same-user attacker
//! cannot interfere with the wider process.

use std::fs::{DirBuilder, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use solo_application::{SourceReadRefusalKind, SourceReadRefusalV1};

/// `FILE_SHARE_DELETE`. A literal because naming the native crate outside the approved module is
/// forbidden; the value is the documented Windows constant and nothing else is shared.
#[cfg(windows)]
const FILE_SHARE_DELETE_ONLY: u32 = 0x0000_0004;

/// `FILE_FLAG_BACKUP_SEMANTICS`: what lets a directory be opened as a handle at all.
#[cfg(windows)]
const FILE_FLAG_BACKUP_SEMANTICS: u32 = 0x0200_0000;

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

/// Opens a directory as a retained handle.
///
/// The handle, not the string, is what later operations depend on, and its identity is what
/// confinement comparisons use. Windows refuses an ordinary `File::open` on a directory, so the
/// safe backup-semantics flag is required there.
fn open_directory(path: &Path) -> Result<File, SourceReadRefusalV1> {
    let mut options = OpenOptions::new();
    options.read(true);
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        options.custom_flags(FILE_FLAG_BACKUP_SEMANTICS);
    }
    let file = options.open(path).map_err(classify_io)?;
    if !file.metadata().map_err(classify_io)?.is_dir() {
        return Err(unsafe_location());
    }
    Ok(file)
}

/// Creates one staging object that no other handle can read from or write to.
///
/// Linux creates an `O_TMPFILE | O_EXCL` inode: it never has a directory entry, so there is no
/// window in which an alias could be opened. Windows establishes the guarantee atomically in the
/// create itself by sharing only delete, so no other handle can obtain read or write access; the
/// unlink that follows then leaves no path to reach it either.
#[cfg(target_os = "linux")]
fn open_staging_object(
    _namespace: &Path,
    directory: &File,
    _index: usize,
) -> Result<File, SourceReadRefusalV1> {
    match crate::source_read_native::open_unnamed_staging_object(directory) {
        Ok(file) => Ok(file),
        // A genuine denial or a genuine lack of space is reported as what it is.
        Err(error)
            if error.kind() == std::io::ErrorKind::PermissionDenied
                || is_capacity_error(&error) =>
        {
            Err(classify_io(error))
        }
        // Anything else means this store could not create an object inside the directory it
        // provably holds — the namespace has been unlinked from the tree, or the host answered
        // with something this store cannot interpret. Either way confinement is unproven, so it is
        // refused as an unsafe location and never as an absence or a shorter member.
        Err(_) => Err(unsafe_location()),
    }
}

/// Whether the host reported that the file system is out of space rather than a confinement
/// fault. Read from the raw code so no unstable `ErrorKind` variant is required.
#[cfg(target_os = "linux")]
fn is_capacity_error(error: &std::io::Error) -> bool {
    /// `ENOSPC`.
    const NO_SPACE: i32 = 28;
    /// `EDQUOT`.
    const QUOTA_EXCEEDED: i32 = 122;
    matches!(error.raw_os_error(), Some(NO_SPACE | QUOTA_EXCEEDED))
}

/// The share mode is the guarantee here; the unlink removes the path as well.
///
/// Windows cannot create a file relative to a retained directory handle through the standard
/// library, so the object is created by name inside the confirmed namespace. The object id this
/// host reports is not recycled, so the confirmation either side of the create is sound here.
#[cfg(windows)]
fn open_staging_object(
    namespace: &Path,
    _directory: &File,
    index: usize,
) -> Result<File, SourceReadRefusalV1> {
    use std::os::windows::fs::OpenOptionsExt;
    let path = namespace.join(format!("{STAGE_OBJECT_PREFIX}{index}.bin"));
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .share_mode(FILE_SHARE_DELETE_ONLY)
        .open(&path)
        .map_err(classify_io)?;
    if let Err(error) = std::fs::remove_file(&path) {
        drop(file);
        let _ = std::fs::remove_file(&path);
        return Err(classify_io(error));
    }
    Ok(file)
}

/// Other Unix hosts have no proven unnamed primitive here, so staging refuses rather than guess.
#[cfg(all(unix, not(target_os = "linux")))]
fn open_staging_object(
    _namespace: &Path,
    _directory: &File,
    _index: usize,
) -> Result<File, SourceReadRefusalV1> {
    Err(SourceReadRefusalV1::new(
        SourceReadRefusalKind::Unsupported,
        None,
    ))
}

/// Hosts with neither primitive refuse for the same reason.
#[cfg(not(any(unix, windows)))]
fn open_staging_object(
    _namespace: &Path,
    _directory: &File,
    _index: usize,
) -> Result<File, SourceReadRefusalV1> {
    Err(SourceReadRefusalV1::new(
        SourceReadRefusalKind::Unsupported,
        None,
    ))
}

/// The deterministic staging fault a test asks the **production** path to hit.
///
/// It exists so a later-member write failure, a short write, a flush failure and a finalization
/// failure are exercised through the real transaction code rather than a parallel fake. No
/// production caller sets one.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StagingFaultV1 {
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

/// The prefix of the namespace directory this module creates inside an adopted staging root.
const STAGE_NAMESPACE_PREFIX: &str = "solo-source-stage-";

/// The prefix of every staging object inside that namespace.
///
/// Only the Windows implementation creates an object *by name* and then detaches it; Linux
/// creates an `O_TMPFILE` inode that never has one, so the constant is compiled there and not
/// merely unused.
#[cfg(windows)]
const STAGE_OBJECT_PREFIX: &str = "shard-";

/// Distinguishes concurrently live stores inside one process.
///
/// A name counter, never a store of content: it exists so two live stores in one process cannot
/// choose the same namespace name.
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
        if !root.is_absolute() {
            return Err(unsafe_location());
        }
        let metadata = match std::fs::symlink_metadata(root) {
            Ok(metadata) => metadata,
            // A staging root that is not there cannot be proven to be the private directory the
            // caller intended, so it is an unsafe location rather than an absence to invent.
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Err(unsafe_location());
            }
            Err(error) => return Err(classify_io(error)),
        };
        if metadata.file_type().is_symlink() || !metadata.is_dir() {
            return Err(unsafe_location());
        }
        let handle = open_directory(root)?;
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
        let namespace = self.root.join(namespace_name());
        // The retained capability must still be the live directory it was adopted as. This reads
        // the handle, not the string, so a root that disappeared or stopped being a directory
        // refuses before anything is created.
        if !self.handle.metadata().map_err(classify_io)?.is_dir() {
            return Err(unsafe_location());
        }
        if !identity_of(&self.handle)?.same_object(self.identity) {
            return Err(unsafe_location());
        }
        create_private_directory(&namespace)?;

        let namespace_handle = match open_directory(&namespace) {
            Ok(handle) => handle,
            Err(refusal) => {
                // Only the empty directory this call just created, and only on this failure path.
                let _ = std::fs::remove_dir(&namespace);
                return Err(refusal);
            }
        };
        let namespace_identity = identity_of(&namespace_handle)?;

        // Confinement is decided by identity, not by the string: the parent of the namespace we
        // just created must be the very directory this capability adopted. A redirected path
        // component or a root replaced between adoption and now fails here, before any byte is
        // written or any object created.
        let parent = match open_directory(&namespace.join("..")) {
            Ok(handle) => handle,
            Err(refusal) => {
                let _ = std::fs::remove_dir(&namespace);
                return Err(refusal);
            }
        };
        if !identity_of(&parent)?.same_object(self.identity) {
            let _ = std::fs::remove_dir(&namespace);
            return Err(unsafe_location());
        }

        #[cfg(target_os = "linux")]
        let root_dir = self.handle.try_clone().map_err(classify_io)?;
        Ok(SourceSetStagingV1 {
            namespace,
            namespace_dir: namespace_handle,
            #[cfg(target_os = "linux")]
            root_dir,
            namespace_identity,
            shards: Vec::new(),
            staged_bytes: 0,
            max_member_bytes: 0,
            staged_members: 0,
            sealed: false,
            faults,
        })
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
    namespace: PathBuf,
    /// The retained namespace descriptor every staging object is created through.
    namespace_dir: File,
    /// The adopted root descriptor, used to remove the namespace entry without re-walking a path.
    #[cfg(target_os = "linux")]
    root_dir: File,
    namespace_identity: ObjectIdentityV1,
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
    /// It re-confirms that the namespace is still the directory this store created and that every
    /// object still reports exactly the length that was written. A last-write, finalization or
    /// substitution failure is refused here rather than becoming an accepted candidate, and no
    /// public value exists until this returns `Ok`.
    ///
    /// # Errors
    ///
    /// Returns `SOURCE_OBSERVATION_IO_ERROR` for an injected or real finalization failure,
    /// `SOURCE_OBSERVATION_CHANGED_DURING_READ` when an object no longer reports its own length,
    /// and `SOURCE_UNSAFE_PATH` when the namespace is no longer the one this store created.
    pub fn finalize(&mut self) -> Result<(), SourceReadRefusalV1> {
        if self.sealed {
            return Ok(());
        }
        if self.faults.fault == Some(StagingFaultV1::Finalize)
            && self.faults.after <= self.staged_members
        {
            return Err(out_of_capacity());
        }
        self.confirm_namespace()?;
        for shard in &self.shards {
            let observed = shard.file.metadata().map_err(classify_io)?.len();
            if observed != shard.written {
                return Err(changed());
            }
        }
        self.sealed = true;
        Ok(())
    }

    /// Confirms that the directory staging writes into is still this store's own.
    ///
    /// On Linux the control is the descriptor, not a name: objects are created *through* the
    /// retained namespace handle, so a substituted pathname never receives a byte and re-resolving
    /// the path here would only add a check-then-use window. Comparing the descriptor with the
    /// identity recorded when it was created is a cheap sanity check, not the guarantee.
    ///
    /// Windows creates objects by name inside the namespace, so the entry must additionally still
    /// resolve to this store's directory. Its object id is not recycled, which is what makes that
    /// comparison sound there.
    ///
    /// # Errors
    ///
    /// Returns `SOURCE_UNSAFE_PATH` when the descriptor is no longer the recorded namespace, and on
    /// Windows also when the namespace entry no longer resolves to it.
    fn confirm_namespace(&self) -> Result<(), SourceReadRefusalV1> {
        if !identity_of(&self.namespace_dir)?.same_object(self.namespace_identity) {
            return Err(unsafe_location());
        }
        #[cfg(not(target_os = "linux"))]
        {
            let handle = open_directory(&self.namespace)?;
            if !identity_of(&handle)?.same_object(self.namespace_identity) {
                return Err(unsafe_location());
            }
        }
        Ok(())
    }

    /// Removes this store's own, now-empty namespace — and nothing else.
    ///
    /// Ownership is decided before anything is removed, and the removal is deliberately not
    /// recursive. A swap confined to the window between that decision and the removal is not
    /// excluded by the standard library, so the consequence is bounded instead: only a directory
    /// inside the adopted root is ever considered, and only an **empty** one can be removed. No
    /// staged content, no Owner material and no populated adversary directory can be deleted, and
    /// a directory that is not provably this store's is left in place rather than forced.
    fn remove_namespace(&self) {
        let is_ours = match open_directory(&self.namespace) {
            Ok(handle) => identity_of(&handle)
                .map(|found| found.same_object(self.namespace_identity))
                .unwrap_or(false),
            Err(_) => false,
        };
        if !is_ours {
            return;
        }
        #[cfg(target_os = "linux")]
        {
            // Resolved through the retained root descriptor, so no path component is re-walked and
            // a replaced ancestor cannot redirect the removal.
            if let Some(name) = self.namespace.file_name() {
                let _ =
                    crate::source_read_native::remove_staging_namespace_entry(&self.root_dir, name);
            }
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = std::fs::remove_dir(&self.namespace);
        }
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
    #[must_use]
    pub fn object_count(&self) -> usize {
        self.shards.len()
    }

    /// How many directory entries are visible inside the namespace.
    ///
    /// A sealed store has **none**: every object is nameless while it lives. This is observed
    /// state, not an assertion about intent, and it is what proves on a real host that no
    /// plaintext path exists to read, replace or recover.
    ///
    /// # Errors
    ///
    /// Returns the classified staging refusal when the namespace cannot be listed.
    pub fn visible_object_entries(&self) -> Result<usize, SourceReadRefusalV1> {
        let entries = std::fs::read_dir(&self.namespace).map_err(classify_io)?;
        let mut count = 0_usize;
        for entry in entries {
            entry.map_err(classify_io)?;
            count += 1;
        }
        Ok(count)
    }

    /// Creates one nameless staging object inside this store's own namespace.
    fn open_object(&mut self) -> Result<(), SourceReadRefusalV1> {
        // Linux creates the object through the retained namespace descriptor, so a substituted
        // pathname cannot receive a byte and there is no check-then-use window to lose. Windows
        // creates it by name, so the entry is confirmed immediately before and after the create:
        // a namespace substituted during the create is detected before a byte is written into the
        // object it produced.
        #[cfg(not(target_os = "linux"))]
        self.confirm_namespace()?;
        let index = self.shards.len();
        let file = open_staging_object(&self.namespace, &self.namespace_dir, index)?;
        #[cfg(not(target_os = "linux"))]
        self.confirm_namespace()?;
        self.shards.push(StageShardV1 { file, written: 0 });
        Ok(())
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
        // Objects are nameless on Linux and already detached on Windows, so closing every handle is
        // the whole of object cleanup.
        self.shards.clear();
        // Then the entry itself, and only when it is provably still this store's own empty
        // namespace. A namespace an adversary substituted is left where it is.
        self.remove_namespace();
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
fn namespace_name() -> String {
    let sequence = STAGE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos())
        .unwrap_or(0);
    format!(
        "{STAGE_NAMESPACE_PREFIX}{}-{sequence}-{nanos}",
        std::process::id()
    )
}

/// Creates one namespace with exclusive creation and owner-only permissions where the host has
/// them.
fn create_private_directory(path: &Path) -> Result<(), SourceReadRefusalV1> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        DirBuilder::new()
            .mode(0o700)
            .create(path)
            .map_err(classify_io)
    }
    #[cfg(not(unix))]
    {
        // Other hosts have no per-directory mode to set here; the private namespace is still
        // created exclusively, and the ACL of the adopted root is the caller's decision.
        DirBuilder::new().create(path).map_err(classify_io)
    }
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
