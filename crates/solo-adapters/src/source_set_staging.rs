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
}

impl StagingRootCapability {
    /// Adopts an existing directory as the staging root.
    ///
    /// The directory must already exist and be a real directory reached without a symbolic link:
    /// this function never creates the root, never adopts a parent chain and never treats a
    /// missing root as a reason to fall back to an OS temp directory.
    ///
    /// # Errors
    ///
    /// Returns `SOURCE_UNSAFE_PATH` for a relative path, a symbolic link or a non-directory, and
    /// the classified staging refusal when the directory cannot be inspected.
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
        Ok(Self {
            root: root.to_path_buf(),
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
        let namespace = self.root.join(namespace_name());
        create_private_directory(&namespace)?;
        Ok(SourceSetStagingV1 {
            namespace,
            shards: Vec::new(),
            staged_bytes: 0,
            max_member_bytes: 0,
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
    shards: Vec<StageShardV1>,
    staged_bytes: u64,
    max_member_bytes: u64,
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
        shard.file.write_all(bytes).map_err(classify_io)?;
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

    /// Creates one object and immediately detaches its name.
    fn open_object(&mut self) -> Result<(), SourceReadRefusalV1> {
        let path = self
            .namespace
            .join(format!("{STAGE_OBJECT_PREFIX}{}.bin", self.shards.len()));
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(classify_io)?;

        // Detach the directory entry before a single body byte is written. If the host refuses,
        // the object is closed and removed and the attempt refuses: a named staging object is
        // never accepted as a substitute for a nameless one.
        if let Err(error) = std::fs::remove_file(&path) {
            drop(file);
            let _ = std::fs::remove_file(&path);
            return Err(classify_io(error));
        }

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
        // Close every handle first, so a host that needs the handle gone can remove the name.
        self.shards.clear();
        // Remove only this run's namespace, and only if it is empty. Never recursive, never a
        // directory this store did not create, never a shared temp root.
        let _ = std::fs::remove_dir(&self.namespace);
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
