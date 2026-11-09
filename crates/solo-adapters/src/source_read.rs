//! The real, capability-confined source reader.
//!
//! This module is safe Rust only. It holds a [`ContentRootCapability`], an operating-system
//! handle to an explicitly chosen content root, and resolves declared `pages/…` locators
//! relative to that handle. It performs no confinement check of its own:
//! [`crate::source_read_native`] does that, and this module never replaces it with a string
//! comparison, a pre-open `metadata` check or a `canonicalize`.
//!
//! **What a successful read proves here.** The bytes returned belong to the physical object
//! whose identity is reported, because identity, length and link count are sampled on the
//! handle before and after the transfer and any difference is refused. What it does not
//! prove is a filesystem-wide snapshot: a writer can still change the file after the final
//! sample. A read never authorizes publication, selects a route, decides visibility or parses
//! a body.

use std::path::Path;

use solo_application::{
    CandidateBodyV1, CandidateProbeV1, PhysicalIdentityV1, SourceReadPort, SourceReadRefusalKind,
    SourceReadRefusalV1, bound_for, fingerprint_of,
};
use solo_domain::{SourceCandidateKind, SourceCandidatePathV1};

use crate::source_read_native::{self as native, NativeError};

/// The one namespace a source locator may address inside the content root.
const PAGES_NAMESPACE: &str = "pages";

/// An explicitly trusted, already-opened content-root capability.
///
/// The caller supplies the root *once*. Everything afterwards is relative to this handle, so
/// a later change to a path string cannot redirect a read: there is no later path string.
#[derive(Debug)]
pub struct ContentRootCapability {
    root: native::RootHandle,
}

impl ContentRootCapability {
    /// Adopts `path` as the trusted content root.
    ///
    /// This is the single trust decision in the reader: the caller states which directory is
    /// the content root, and this call opens it once and records what was actually opened.
    ///
    /// # Errors
    ///
    /// Returns a refusal when the root cannot be opened, is not a directory, or is itself a
    /// symlink or reparse point.
    pub fn adopt(path: &Path) -> Result<Self, SourceReadRefusalV1> {
        native::RootHandle::adopt(path)
            .map(|root| Self { root })
            .map_err(|error| map_native(error, None))
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
        let bytes = match native::read_bounded(&mut leaf, bound_for(member)) {
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
