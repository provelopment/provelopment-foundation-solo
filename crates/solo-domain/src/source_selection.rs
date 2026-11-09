//! Provisional JSON/Markdown source selection over explicitly supplied observations.
//!
//! This module is the **pure policy** half of the accepted portable-source contract
//! (§1.6, §6.2C.4). It consumes the accepted [`SourceCandidateSetV1`] declaration and a
//! set of observations that a *future* trustworthy acquisition adapter would supply, and
//! it decides which single declared member is provisionally chosen. It opens no file,
//! stats no path, resolves no symlink, confirms no absence and parses no body.
//!
//! **Read order is fixed, never declaration order.** A declared `json` member is always
//! considered first: supplied present JSON is chosen without the Markdown body ever being
//! requested, whether or not those JSON bytes would later pass body preflight. Only a
//! genuinely supplied `Absent` JSON permits considering a declared `markdown` member.
//!
//! **What is explicitly not proven here.** An [`SourceObservationV1`] is a *caller
//! supplied claim*, not filesystem attestation: `Absent` is a reported absence, not a
//! verified one, and it is never inferred from a permission or I/O result. Selection is
//! *provisional*, not final effective-source selection, and it confers no publication
//! authority. Confinement, no-follow identity, alias/hardlink/case equivalence and
//! immutable filesystem snapshots remain the future adapter's obligations
//! (§6.2C.3–§6.2C.4) and are represented by [`OBSERVATION_IS_FILESYSTEM_PROOF`] and
//! [`PROVISIONAL_CHOICE_IS_FINAL_SELECTION`], both `false`.

use core::fmt;

use crate::source_locator::{SourceCandidateKind, SourceCandidatePathV1, SourceCandidateSetV1};

/// Whether a supplied observation is proof obtained from the filesystem.
///
/// It never is. The policy sees only what a caller states, so a consumer that needs
/// confirmed absence, no-follow confinement or an immutable snapshot must not read this
/// module's decision as if it provided them.
pub const OBSERVATION_IS_FILESYSTEM_PROOF: bool = false;

/// Whether a provisional choice is final effective-source selection.
///
/// It never is. A chosen candidate still has to be acquired under confinement and its
/// bytes validated before anything may be published.
pub const PROVISIONAL_CHOICE_IS_FINAL_SELECTION: bool = false;

/// Why a supplied observation carries no usable bytes, in a form that never collapses
/// into absence.
///
/// `Absent` is a result a caller may only report when absence was safely established.
/// These variants exist so a permission problem, an unrelated I/O fault, an unsafe
/// location and a body that changed while it was being read can never be converted into
/// an empty or absent source (§6.2C.4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceFailureKind {
    /// Access was denied at some component or at the leaf.
    PermissionDenied,
    /// Some other I/O fault prevented a trustworthy observation.
    OtherIo,
    /// The location or its identity could not be shown to be safe and confined.
    UnsafeLocation,
    /// The content changed while it was being read, so no consistent body exists.
    ChangedDuringRead,
}

impl SourceFailureKind {
    /// Stable, safe diagnostic token for this failure.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::PermissionDenied => "SOURCE_OBSERVATION_PERMISSION_DENIED",
            Self::OtherIo => "SOURCE_OBSERVATION_IO_ERROR",
            Self::UnsafeLocation => "SOURCE_OBSERVATION_UNSAFE_LOCATION",
            Self::ChangedDuringRead => "SOURCE_OBSERVATION_CHANGED_DURING_READ",
        }
    }

    /// Human-readable description carrying no path, body or secret.
    #[must_use]
    pub const fn detail(self) -> &'static str {
        match self {
            Self::PermissionDenied => "the supplied observation was refused access",
            Self::OtherIo => "the supplied observation reported an I/O fault",
            Self::UnsafeLocation => "the supplied observation could not be shown to be safe",
            Self::ChangedDuringRead => "the supplied observation changed while being read",
        }
    }
}

impl fmt::Display for SourceFailureKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

/// One already-obtained observation for a declared candidate member.
///
/// The policy borrows the caller's bytes and never copies or owns them. `Present` is not
/// a validity claim: bytes that are empty, oversized or syntactically invalid still
/// arrive here as `Present`, and refusing them is the separate body-preflight concern.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceObservationV1<'a> {
    /// Absence that the caller safely established, not a stand-in for an error.
    Absent,
    /// Bytes the caller obtained, with no claim yet about their validity.
    Present(&'a [u8]),
    /// A named failure that must be reported rather than treated as absence.
    Failure(SourceFailureKind),
}

impl<'a> SourceObservationV1<'a> {
    /// Whether this observation reports a named failure.
    #[must_use]
    pub const fn is_failure(self) -> bool {
        matches!(self, Self::Failure(_))
    }

    /// The observed bytes, when the observation is present.
    #[must_use]
    pub const fn present_bytes(self) -> Option<&'a [u8]> {
        match self {
            Self::Present(bytes) => Some(bytes),
            Self::Absent | Self::Failure(_) => None,
        }
    }
}

/// Typed observations supplied for one representation's declared candidate members.
///
/// A member that was not declared may not carry an observation, and this type keeps that
/// check explicit at the policy boundary rather than trusting a caller's convention.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SuppliedObservationsV1<'a> {
    json: Option<SourceObservationV1<'a>>,
    markdown: Option<SourceObservationV1<'a>>,
}

impl<'a> SuppliedObservationsV1<'a> {
    /// No observations supplied yet.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            json: None,
            markdown: None,
        }
    }

    /// Supplies the observation for the declared `json` member.
    #[must_use]
    pub const fn with_json(self, observation: SourceObservationV1<'a>) -> Self {
        Self {
            json: Some(observation),
            markdown: self.markdown,
        }
    }

    /// Supplies the observation for the declared `markdown` member.
    #[must_use]
    pub const fn with_markdown(self, observation: SourceObservationV1<'a>) -> Self {
        Self {
            json: self.json,
            markdown: Some(observation),
        }
    }

    /// The supplied `json` observation, if one was supplied at all.
    #[must_use]
    pub const fn json(self) -> Option<SourceObservationV1<'a>> {
        self.json
    }

    /// The supplied `markdown` observation, if one was supplied at all.
    #[must_use]
    pub const fn markdown(self) -> Option<SourceObservationV1<'a>> {
        self.markdown
    }
}

/// The single declared member chosen provisionally, with its logical locator.
///
/// The name is deliberately *provisional*: this is a selection outcome over supplied
/// observations, not `EffectivePublishedSource`, not a `ValidatedBody` and not a
/// `VerifiedSnapshot`. Both byte and identity guarantees are absent by design.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProvisionalSourceChoiceV1<'a> {
    kind: SourceCandidateKind,
    locator: &'a SourceCandidatePathV1,
    bytes: &'a [u8],
}

impl<'a> ProvisionalSourceChoiceV1<'a> {
    /// Which declared member was chosen.
    #[must_use]
    pub const fn kind(&self) -> SourceCandidateKind {
        self.kind
    }

    /// The validated logical locator of the chosen member.
    #[must_use]
    pub fn locator(&self) -> &'a SourceCandidatePathV1 {
        self.locator
    }

    /// The chosen member's logical path, which is never a physical location.
    #[must_use]
    pub fn logical_path(&self) -> &'a str {
        self.locator.as_str()
    }

    /// The caller's supplied bytes for the chosen member, borrowed unmodified.
    #[must_use]
    pub fn supplied_bytes(&self) -> &'a [u8] {
        self.bytes
    }

    /// Whether this choice may be published. It never may.
    #[must_use]
    pub const fn is_publication_authorized(&self) -> bool {
        false
    }

    /// Whether this choice is a verified immutable filesystem snapshot. It never is.
    #[must_use]
    pub const fn is_verified_filesystem_snapshot(&self) -> bool {
        false
    }
}

/// Why a provisional selection was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceSelectionRefusalKind {
    /// A declared member's observation was not supplied at all.
    MissingObservation,
    /// An observation was supplied for a member that was never declared.
    UndeclaredCandidateObservation,
    /// Present JSON was supplied together with Markdown body bytes, which are unwanted.
    RedundantMarkdownObservation,
    /// Every declared member was safely reported absent.
    NeitherPresent,
    /// A declared member reported a named failure instead of bytes or absence.
    ObservationFailure,
}

impl SourceSelectionRefusalKind {
    /// Stable, safe diagnostic token for this refusal reason.
    #[must_use]
    pub const fn detail(self) -> &'static str {
        match self {
            Self::MissingObservation => "a declared candidate has no supplied observation",
            Self::UndeclaredCandidateObservation => {
                "an observation was supplied for an undeclared candidate"
            }
            Self::RedundantMarkdownObservation => {
                "present JSON was supplied together with unwanted Markdown body bytes"
            }
            Self::NeitherPresent => "every declared candidate was reported absent",
            Self::ObservationFailure => "a declared candidate reported a named failure",
        }
    }
}

/// A refused provisional selection.
///
/// The diagnostic names the refusal reason, the affected declared member and any
/// underlying failure kind, and it never carries a body, a physical path or a secret.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceSelectionRefusal {
    kind: SourceSelectionRefusalKind,
    member: Option<SourceCandidateKind>,
    failure: Option<SourceFailureKind>,
}

impl SourceSelectionRefusal {
    const fn new(kind: SourceSelectionRefusalKind, member: Option<SourceCandidateKind>) -> Self {
        Self {
            kind,
            member,
            failure: None,
        }
    }

    const fn failed(member: SourceCandidateKind, failure: SourceFailureKind) -> Self {
        Self {
            kind: SourceSelectionRefusalKind::ObservationFailure,
            member: Some(member),
            failure: Some(failure),
        }
    }

    /// The refusal reason.
    #[must_use]
    pub const fn kind(&self) -> SourceSelectionRefusalKind {
        self.kind
    }

    /// The declared member the refusal concerns, where one is meaningful.
    #[must_use]
    pub const fn member(&self) -> Option<SourceCandidateKind> {
        self.member
    }

    /// The underlying observation failure, when the refusal was caused by one.
    #[must_use]
    pub const fn failure(&self) -> Option<SourceFailureKind> {
        self.failure
    }

    /// Stable engineering code for this refusal.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        "PREFLIGHT_SOURCE_SELECTION_REFUSED"
    }

    /// Stable, safe diagnostic token for the refusal reason.
    #[must_use]
    pub const fn detail(&self) -> &'static str {
        self.kind.detail()
    }
}

impl fmt::Display for SourceSelectionRefusal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())?;
        formatter.write_str(": ")?;
        formatter.write_str(self.detail())?;
        if let Some(member) = self.member {
            write!(formatter, " (candidate `{member}`)")?;
        }
        if let Some(failure) = self.failure {
            write!(formatter, " [{}]", failure.code())?;
        }
        Ok(())
    }
}

impl std::error::Error for SourceSelectionRefusal {}

/// Chooses one declared candidate **provisionally** from supplied observations.
///
/// The decision depends only on the declaration and the observations, never on
/// declaration order, enumeration order, a directory listing, an inferred suffix, the
/// clock, the locale or the environment.
///
/// # Errors
///
/// Returns a [`SourceSelectionRefusal`] when a declared member has no supplied
/// observation, when an undeclared member carries one, when present JSON arrives with
/// unwanted Markdown body bytes, when a member reports a named failure, or when every
/// declared member is reported absent. A failure is never converted into absence and no
/// fallback is attempted after a JSON `Failure`.
pub fn select_provisional_source<'a>(
    candidates: &'a SourceCandidateSetV1,
    observations: SuppliedObservationsV1<'a>,
) -> Result<ProvisionalSourceChoiceV1<'a>, SourceSelectionRefusal> {
    use SourceCandidateKind::{Json, Markdown};

    // An observation for a member that was never declared is a caller fault, refused
    // before any read order could make it look meaningful.
    if observations.json().is_some() && candidates.json().is_none() {
        return Err(SourceSelectionRefusal::new(
            SourceSelectionRefusalKind::UndeclaredCandidateObservation,
            Some(Json),
        ));
    }
    if observations.markdown().is_some() && candidates.markdown().is_none() {
        return Err(SourceSelectionRefusal::new(
            SourceSelectionRefusalKind::UndeclaredCandidateObservation,
            Some(Markdown),
        ));
    }

    if let Some(locator) = candidates.json() {
        let observation = observations.json().ok_or(SourceSelectionRefusal::new(
            SourceSelectionRefusalKind::MissingObservation,
            Some(Json),
        ))?;

        match observation {
            SourceObservationV1::Present(bytes) => {
                // Present JSON ends the read order. Markdown body bytes supplied anyway
                // were never needed, so they are refused rather than ignored or parsed.
                if observations.markdown().is_some() {
                    return Err(SourceSelectionRefusal::new(
                        SourceSelectionRefusalKind::RedundantMarkdownObservation,
                        Some(Markdown),
                    ));
                }
                return Ok(ProvisionalSourceChoiceV1 {
                    kind: Json,
                    locator,
                    bytes,
                });
            }
            SourceObservationV1::Failure(failure) => {
                return Err(SourceSelectionRefusal::failed(Json, failure));
            }
            SourceObservationV1::Absent => {}
        }
    }

    // Reached only when no JSON was declared, or when the declared JSON was reported
    // absent. Only a reported absence may permit considering Markdown.
    if let Some(locator) = candidates.markdown() {
        let observation = observations.markdown().ok_or(SourceSelectionRefusal::new(
            SourceSelectionRefusalKind::MissingObservation,
            Some(Markdown),
        ))?;

        match observation {
            SourceObservationV1::Present(bytes) => {
                return Ok(ProvisionalSourceChoiceV1 {
                    kind: Markdown,
                    locator,
                    bytes,
                });
            }
            SourceObservationV1::Failure(failure) => {
                return Err(SourceSelectionRefusal::failed(Markdown, failure));
            }
            SourceObservationV1::Absent => {}
        }
    }

    Err(SourceSelectionRefusal::new(
        SourceSelectionRefusalKind::NeitherPresent,
        None,
    ))
}



