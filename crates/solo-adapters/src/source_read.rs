//! The real, capability-confined source reader.
//!
//! This module is safe Rust only. It holds a [`ContentRootCapability`], an operating-system
//! handle to an explicitly chosen content root, and resolves declared `pages/…` locators
//! relative to that handle. It performs no confinement check of its own:
//! [`crate::source_read_native`] does that, and this module never replaces it with a string
//! comparison, a pre-open `metadata` check or a `canonicalize`.
//!
//! **What a successful read proves here.** The returned bytes are one instant of the file,
//! not a mutable read a writer can tear: they come from an anonymous copy-on-write clone on
//! Linux and from a write-denying handle on Windows. They also belong to the physical object
//! whose identity is reported, because identity, length and link count are sampled on the
//! handle before and after the transfer and any difference is refused. What it does not prove
//! is a filesystem-wide or multi-file snapshot, or that the source will not change immediately
//! afterwards. A read never authorizes publication, selects a route, decides visibility or
//! parses a body.

use std::path::Path;

use solo_application::{
    CandidateBodyV1, CandidateProbeV1, PhysicalIdentityV1, SourceReadPort, SourceReadRefusalKind,
    SourceReadRefusalV1, bound_for, fingerprint_of,
};
use solo_domain::{SourceCandidateKind, SourceCandidatePathV1};

use crate::source_read_native::{self as native, NativeError};

/// The one namespace a source locator may address inside the content root.
const PAGES_NAMESPACE: &str = "pages";

/// Why the caller asserts that a directory is the installation content root.
///
/// The reader cannot attest provenance; it can only refuse to *assume* it. Requiring this
/// value makes the trust input explicit at every call site, and the recorded value travels
/// with the capability, so a capability can never be mistaken for something the reader
/// discovered by itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RootProvenanceV1 {
    /// The operator designated this directory as the installation content root.
    OperatorDesignatedContentRoot,
}

impl RootProvenanceV1 {
    /// A stable code for the execution report.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::OperatorDesignatedContentRoot => "operator-designated-content-root",
        }
    }
}

/// The owned directory handle a trusted bootstrap hands to the reader.
#[cfg(unix)]
pub type TrustedDirectoryHandle = std::os::fd::OwnedFd;

/// The owned directory handle a trusted bootstrap hands to the reader.
#[cfg(windows)]
pub type TrustedDirectoryHandle = std::os::windows::io::OwnedHandle;

/// An explicitly trusted, already-opened content-root capability.
///
/// The caller supplies the root *once*. Everything afterwards is relative to this handle, so
/// a later change to a path string cannot redirect a read: there is no later path string.
#[derive(Debug)]
pub struct ContentRootCapability {
    root: native::RootHandle,
    provenance: RootProvenanceV1,
}

impl ContentRootCapability {
    /// Opens `path` **once** as the operator-designated content root and records what was
    /// opened.
    ///
    /// This is the bootstrap route, not the trusted-handle transfer. The path is the operator's
    /// designation; it is resolved exactly once, and every later operation is relative to the
    /// resulting handle, so no later path text can redirect a read. `provenance` records which
    /// assertion the caller made; it is not evidence about the handle, which is why the handle
    /// route [`Self::from_trusted_directory`] exists for a caller that already holds one.
    ///
    /// # Errors
    ///
    /// Returns a refusal when the root cannot be opened, is not a directory, or is itself a
    /// symlink or reparse point.
    pub fn adopt_bootstrap_path(
        path: &Path,
        provenance: RootProvenanceV1,
    ) -> Result<Self, SourceReadRefusalV1> {
        native::RootHandle::adopt_bootstrap_path(path)
            .map(|root| Self { root, provenance })
            .map_err(|error| map_native(error, None))
    }

    /// Takes ownership of a directory handle a trusted bootstrap already opened.
    ///
    /// This is the trusted-handle route R02 §1 requires. The object under test is the handle
    /// itself: its own type, reparse state and identity are read from it, so a handle naming a
    /// regular file or a reparse point is refused, and the path that opened it is never
    /// consulted again — there is no later path.
    ///
    /// # Errors
    ///
    /// Returns a refusal when the supplied handle is not a directory, names a symlink or
    /// reparse point, or cannot be interrogated.
    pub fn from_trusted_directory(
        handle: TrustedDirectoryHandle,
        provenance: RootProvenanceV1,
    ) -> Result<Self, SourceReadRefusalV1> {
        native::RootHandle::from_trusted_directory(handle)
            .map(|root| Self { root, provenance })
            .map_err(|error| map_native(error, None))
    }

    /// The trust assertion the caller made when adopting this capability.
    #[must_use]
    pub const fn provenance(&self) -> RootProvenanceV1 {
        self.provenance
    }

    /// The physical identity recorded when the capability was adopted.
    #[must_use]
    pub fn identity(&self) -> PhysicalIdentityV1 {
        self.root.identity()
    }

    /// The host mechanism this reader uses, for the execution report.
    #[must_use]
    pub const fn mechanism(&self) -> &'static str {
        self.root.mechanism()
    }

    /// The validated single relative names of a locator, checked again at the boundary.
    ///
    /// The locator type has already proven the shape, so this is defence in depth: it is what
    /// makes the guarantee independent of any future caller constructing a locator another
    /// way.
    fn components(locator: &SourceCandidatePathV1) -> Result<Vec<&str>, SourceReadRefusalV1> {
        let path = locator.as_str();
        let mut segments = Vec::new();
        for segment in path.split('/') {
            if segment.is_empty() || segment.contains('\\') || segment.contains('\0') {
                return Err(SourceReadRefusalV1::new(
                    SourceReadRefusalKind::UnsafeLocation,
                    Some(locator.kind()),
                ));
            }
            segments.push(segment);
        }
        if segments.first() != Some(&PAGES_NAMESPACE) || segments.len() < 2 {
            return Err(SourceReadRefusalV1::new(
                SourceReadRefusalKind::UnsafeLocation,
                Some(locator.kind()),
            ));
        }
        Ok(segments)
    }
}

impl SourceReadPort for ContentRootCapability {
    fn probe_candidate(
        &self,
        member: SourceCandidateKind,
        locator: &SourceCandidatePathV1,
    ) -> CandidateProbeV1 {
        let refusal_member = Some(member);
        let components = match Self::components(locator) {
            Ok(components) => components,
            Err(refusal) => return CandidateProbeV1::Refused(refusal),
        };
        match native::open_leaf(&self.root, &components) {
            Ok(leaf) => {
                let facts = leaf.facts();
                CandidateProbeV1::Located {
                    identity: facts.identity,
                    byte_length: facts.length,
                    links: facts.links,
                    changed_nanos: facts.changed_nanos,
                }
            }
            Err(NativeError::Absent) => CandidateProbeV1::ConfirmedAbsent,
            Err(error) => CandidateProbeV1::Refused(map_native(error, refusal_member)),
        }
    }

    fn read_candidate(
        &self,
        member: SourceCandidateKind,
        locator: &SourceCandidatePathV1,
    ) -> CandidateBodyV1 {
        let refusal_member = Some(member);
        let components = match Self::components(locator) {
            Ok(components) => components,
            Err(refusal) => return CandidateBodyV1::Refused(refusal),
        };
        let mut leaf = match native::open_leaf(&self.root, &components) {
            Ok(leaf) => leaf,
            Err(NativeError::Absent) => return CandidateBodyV1::ConfirmedAbsent,
            Err(error) => return CandidateBodyV1::Refused(map_native(error, refusal_member)),
        };
        let before = leaf.facts();
        let bytes = match native::read_bounded(&self.root, &mut leaf, bound_for(member)) {
            Ok(bytes) => bytes,
            Err(NativeError::Absent) => return CandidateBodyV1::ConfirmedAbsent,
            Err(error) => return CandidateBodyV1::Refused(map_native(error, refusal_member)),
        };
        let after = match native::recheck(&leaf) {
            Ok(after) => after,
            Err(error) => return CandidateBodyV1::Refused(map_native(error, refusal_member)),
        };

        // Identity, length and link count are compared on the handle, and the transferred
        // length is compared with the length that handle reported, so a rewrite that keeps the
        // size but changes the bytes is refused as a change rather than returned as a body.
        let transferred = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
        if after != before || transferred != before.length {
            return CandidateBodyV1::Refused(map_native(NativeError::Changed, refusal_member));
        }

        CandidateBodyV1::Present {
            fingerprint: fingerprint_of(&bytes),
            bytes,
            identity: before.identity,
            links: before.links,
            changed_nanos: before.changed_nanos,
        }
    }
}

/// Maps a native refusal onto the public one, naming the member it concerns.
fn map_native(error: NativeError, member: Option<SourceCandidateKind>) -> SourceReadRefusalV1 {
    let kind = match error {
        // `Absent` is handled by callers before this function is reached; if it ever arrives
        // here it becomes a refusal, so that absence can never be produced by accident.
        NativeError::Absent => SourceReadRefusalKind::OtherIo,
        NativeError::Unsafe => SourceReadRefusalKind::UnsafeLocation,
        NativeError::PermissionDenied => SourceReadRefusalKind::PermissionDenied,
        NativeError::Changed => SourceReadRefusalKind::ChangedDuringRead,
        NativeError::OtherIo => SourceReadRefusalKind::OtherIo,
        NativeError::Unsupported => SourceReadRefusalKind::Unsupported,
    };
    SourceReadRefusalV1::new(kind, member)
}
