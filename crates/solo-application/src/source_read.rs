//! The outbound `SourceRead` port and the non-publishing read use case.
//!
//! This module is the inward half of the source reader. It declares what a read must be
//! able to report, drives the accepted selection policy over those reports, and holds no
//! operating-system handle, no filesystem call and no external crate. The one thing it can
//! never do is turn a successful read into permission to publish.
//!
//! The order of operations is the accepted JSON-first contract: a declared `json` member
//! is probed and, only if it exists, read; a declared `markdown` member is probed for its
//! identity but its bytes are never read while a JSON body is available. That is why the
//! port has two operations instead of one — identity must be checkable without pulling an
//! unselected body into memory.

use core::fmt;

use solo_domain::{
    SourceCandidateKind, SourceCandidatePathV1, SourceCandidateSetV1, SourceFailureKind,
    SourceObservationV1, SourceSelectionRefusal, SourceSelectionRefusalKind,
    SuppliedObservationsV1, select_provisional_source,
};

/// The accepted upper bound for one JSON body: 2 MiB.
pub const MAX_JSON_SOURCE_BYTES: u64 = 2 * 1024 * 1024;

/// The accepted upper bound for one Markdown body: 1 MiB.
pub const MAX_MARKDOWN_SOURCE_BYTES: u64 = 1024 * 1024;

/// Whether a completed read authorizes publication.
///
/// It never does. The value is a constant so no code path can report otherwise, and a
/// caller that wants to publish must obtain authority from a publication pipeline that
/// does not exist yet.
pub const SOURCE_READ_GRANTS_PUBLICATION_AUTHORITY: bool = false;

/// Whether a completed read proves an atomic, filesystem-wide snapshot.
///
/// It never does. The reader proves that the returned bytes belong to the object whose
/// identity it reports, and that an observed change is refused. A concurrent writer can
/// still change the file after the last check, so no snapshot claim is made.
pub const SOURCE_READ_PROVES_GLOBAL_SNAPSHOT: bool = false;

/// The size bound that applies to a declared member.
#[must_use]
pub const fn bound_for(member: SourceCandidateKind) -> u64 {
    match member {
        SourceCandidateKind::Json => MAX_JSON_SOURCE_BYTES,
        SourceCandidateKind::Markdown => MAX_MARKDOWN_SOURCE_BYTES,
    }
}

/// Stable physical identity of one filesystem object, as reported by its own handle.
///
/// It is an opaque value: a container (device or volume) component plus a 128-bit object
/// component, which on Linux is the inode in the low half and zero in the high half. Two
/// identities compare equal only when both components are equal, so a case-folded or
/// string-normalised comparison can never stand in for it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PhysicalIdentityV1 {
    container: u64,
    object_low: u64,
    object_high: u64,
}

impl PhysicalIdentityV1 {
    /// Builds an identity from the three raw components an adapter observed.
    #[must_use]
    pub const fn new(container: u64, object_low: u64, object_high: u64) -> Self {
        Self {
            container,
            object_low,
            object_high,
        }
    }

    /// The device or volume component.
    #[must_use]
    pub const fn container(&self) -> u64 {
        self.container
    }

    /// The low half of the object component.
    #[must_use]
    pub const fn object_low(&self) -> u64 {
        self.object_low
    }

    /// The high half of the object component; always zero where the host has no such half.
    #[must_use]
    pub const fn object_high(&self) -> u64 {
        self.object_high
    }
}

/// Why a candidate could not be reported as present or confirmed absent.
///
/// Every variant is a refusal. None of them is absence, and none may be converted into
/// absence by a caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceReadRefusalKind {
    /// A symlink, junction, reparse point, non-regular file, cross-device escape or
    /// unsafe root was found, so the location is not a usable authored source.
    UnsafeLocation,
    /// Access was refused.
    PermissionDenied,
    /// The object changed identity or length while it was being read.
    ChangedDuringRead,
    /// A physical identity collision, alias or hardlink made ownership ambiguous.
    IdentityCollision,
    /// Every declared candidate was positively and safely shown not to exist.
    NoneEffective,
    /// The host cannot provide the required guarantee, so nothing is reported.
    Unsupported,
    /// Some other I/O fault prevented a trustworthy result.
    OtherIo,
}

impl SourceReadRefusalKind {
    /// Stable, safe diagnostic token, aligned with Architecture §6.2K.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::UnsafeLocation => "SOURCE_UNSAFE_PATH",
            Self::PermissionDenied => "SOURCE_OBSERVATION_PERMISSION_DENIED",
            Self::ChangedDuringRead => "SNAPSHOT_CHANGED",
            Self::IdentityCollision => "SOURCE_COLLISION",
            Self::NoneEffective => "SOURCE_NONE_EFFECTIVE",
            Self::Unsupported => "SOURCE_UNSUPPORTED",
            Self::OtherIo => "SOURCE_OBSERVATION_IO_ERROR",
        }
    }

    /// Fixed, bounded description carrying no path, body, handle or secret.
    #[must_use]
    pub const fn detail(self) -> &'static str {
        match self {
            Self::UnsafeLocation => "the candidate location is not safe to read",
            Self::PermissionDenied => "the candidate could not be opened for reading",
            Self::ChangedDuringRead => "the candidate changed while it was being read",
            Self::IdentityCollision => {
                "the candidate shares a physical object with another candidate"
            }
            Self::Unsupported => "the host cannot prove the required guarantee",
            Self::NoneEffective => "no declared candidate exists",
            Self::OtherIo => "the candidate could not be read reliably",
        }
    }

    /// The matching accepted policy failure, when the policy can express one.
    ///
    /// Absence is deliberately not representable here: this type never describes absence.
    #[must_use]
    pub const fn failure_kind(self) -> SourceFailureKind {
        match self {
            Self::PermissionDenied => SourceFailureKind::PermissionDenied,
            Self::ChangedDuringRead => SourceFailureKind::ChangedDuringRead,
            Self::UnsafeLocation | Self::IdentityCollision | Self::Unsupported => {
                SourceFailureKind::UnsafeLocation
            }
            Self::OtherIo => SourceFailureKind::OtherIo,
            Self::NoneEffective => SourceFailureKind::OtherIo,
        }
    }
}

impl fmt::Display for SourceReadRefusalKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

/// A refusal, naming the declared member it concerns where one is meaningful.
///
/// The diagnostic never carries a physical path, a body byte, a raw handle or native
/// error text: it names only the declared logical member.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceReadRefusalV1 {
    kind: SourceReadRefusalKind,
    member: Option<SourceCandidateKind>,
}

impl SourceReadRefusalV1 {
    /// Builds a refusal for one member, or for a set-level condition when `member` is
    /// `None`.
    #[must_use]
    pub const fn new(kind: SourceReadRefusalKind, member: Option<SourceCandidateKind>) -> Self {
        Self { kind, member }
    }

    /// The refusal reason.
    #[must_use]
    pub const fn kind(&self) -> SourceReadRefusalKind {
        self.kind
    }

    /// The declared member the refusal concerns, where one is meaningful.
    #[must_use]
    pub const fn member(&self) -> Option<SourceCandidateKind> {
        self.member
    }

    /// Stable, safe diagnostic token for the reason.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        self.kind.code()
    }

    /// Fixed, bounded description for the reason.
    #[must_use]
    pub const fn detail(&self) -> &'static str {
        self.kind.detail()
    }
}

impl fmt::Display for SourceReadRefusalV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())?;
        formatter.write_str(": ")?;
        formatter.write_str(self.detail())?;
        if let Some(member) = self.member {
            write!(formatter, " (candidate `{member}`)")?;
        }
        Ok(())
    }
}

impl std::error::Error for SourceReadRefusalV1 {}

/// What a probe established about one declared candidate, without reading its body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CandidateProbeV1 {
    /// The candidate exists and this is what its own handle reported.
    Located {
        /// Physical identity observed on the handle.
        identity: PhysicalIdentityV1,
        /// Byte length observed on the same handle.
        byte_length: u64,
        /// How many directory entries reference the object.
        links: u64,
        /// Modification and change timestamps the handle reported.
        changed_nanos: (i64, i64),
    },
    /// The candidate positively does not exist.
    ConfirmedAbsent,
    /// The candidate could not be reported at all.
    Refused(SourceReadRefusalV1),
}

impl CandidateProbeV1 {
    /// The evidence a located probe recorded, if it located anything.
    #[must_use]
    pub const fn located(&self) -> Option<ProbeFactsV1> {
        match self {
            Self::Located {
                identity,
                byte_length,
                links,
                changed_nanos,
            } => Some(ProbeFactsV1 {
                identity: *identity,
                byte_length: *byte_length,
                links: *links,
                changed_nanos: *changed_nanos,
            }),
            Self::ConfirmedAbsent | Self::Refused(_) => None,
        }
    }
}

/// The identity evidence a probe recorded for one candidate.
///
/// This is what a later read has to agree with. Comparing a read only against its own
/// before/after samples would answer "did this handle stay the same", which is not the same
/// question as "is this still the object the caller was told about".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProbeFactsV1 {
    /// Physical identity the probe recorded.
    pub identity: PhysicalIdentityV1,
    /// Byte length the probe recorded.
    pub byte_length: u64,
    /// Link count the probe recorded.
    pub links: u64,
    /// Modification and change timestamps the probe recorded.
    pub changed_nanos: (i64, i64),
}

/// The body of one declared candidate, or a typed refusal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CandidateBodyV1 {
    /// The candidate was read under its bound and its bytes are owned here.
    Present {
        /// The owned, immutable bytes.
        bytes: Vec<u8>,
        /// Physical identity observed on the handle before and after the read.
        identity: PhysicalIdentityV1,
        /// How many directory entries reference the object.
        links: u64,
        /// Modification and change timestamps the handle reported.
        changed_nanos: (i64, i64),
        /// Non-cryptographic fingerprint of exactly these bytes, for later linkage.
        fingerprint: u64,
    },
    /// The candidate positively does not exist.
    ConfirmedAbsent,
    /// The candidate could not be read.
    Refused(SourceReadRefusalV1),
}

/// The outbound port the application drives and an adapter implements.
///
/// `probe_candidate` must not read a body, and `read_candidate` must not be called by an
/// implementation for a candidate the caller did not ask about. Together they are what
/// lets the use case check identity on a candidate whose bytes it must never load.
pub trait SourceReadPort {
    /// Establishes whether one declared candidate exists, and its identity, without
    /// reading its body.
    fn probe_candidate(
        &self,
        member: SourceCandidateKind,
        locator: &SourceCandidatePathV1,
    ) -> CandidateProbeV1;

    /// Reads one declared candidate's body, within the bound for its member kind.
    fn read_candidate(
        &self,
        member: SourceCandidateKind,
        locator: &SourceCandidatePathV1,
    ) -> CandidateBodyV1;
}

/// A source chosen by the read use case, with its owned bytes and evidence.
///
/// Holding one proves that the bytes belong to the reported physical object at the moment
/// it was read. It proves nothing about publication, and the two constants on this type
/// say so in a form no caller can mistake.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadSourceCandidateV1 {
    member: SourceCandidateKind,
    logical_path: String,
    bytes: Vec<u8>,
    identity: PhysicalIdentityV1,
    fingerprint: u64,
}

impl ReadSourceCandidateV1 {
    /// Which declared member was read.
    #[must_use]
    pub const fn member(&self) -> SourceCandidateKind {
        self.member
    }

    /// The validated logical locator, never a physical path.
    #[must_use]
    pub fn logical_path(&self) -> &str {
        &self.logical_path
    }

    /// The owned bytes.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// The physical identity established from the handle.
    #[must_use]
    pub const fn identity(&self) -> PhysicalIdentityV1 {
        self.identity
    }

    /// Non-cryptographic fingerprint of exactly these bytes.
    #[must_use]
    pub const fn fingerprint(&self) -> u64 {
        self.fingerprint
    }

    /// Whether this read may be published. It never may.
    #[must_use]
    pub const fn is_publication_authorized(&self) -> bool {
        SOURCE_READ_GRANTS_PUBLICATION_AUTHORITY
    }

    /// Whether this read is a verified filesystem-wide snapshot. It never is.
    #[must_use]
    pub const fn is_verified_filesystem_snapshot(&self) -> bool {
        SOURCE_READ_PROVES_GLOBAL_SNAPSHOT
    }
}

/// The outcome of reading one declaration's declared candidates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceReadOutcomeV1 {
    /// One candidate was read and chosen under the accepted JSON-first policy.
    Chosen(ReadSourceCandidateV1),
    /// Nothing was chosen, for the stated safe reason.
    Refused(SourceReadRefusalV1),
}

/// FNV-1a over the measured bytes.
///
/// A linkage fingerprint, not a cryptographic digest: it is here so a later stage can tell
/// whether the same bytes were seen twice without introducing a hashing dependency. It is
/// never used to decide safety.
#[must_use]
pub fn fingerprint_of(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

/// Maps an accepted policy refusal onto a read refusal.
///
/// The two refusals mean different things and are deliberately kept distinct: the policy
/// refuses a *decision* over supplied observations, while a read refusal refuses an
/// *observation*. Only the observation-failure case carries a failure to translate.
fn policy_refusal(refusal: &SourceSelectionRefusal) -> SourceReadRefusalV1 {
    let member = refusal.member();
    match refusal.kind() {
        SourceSelectionRefusalKind::NeitherPresent => {
            SourceReadRefusalV1::new(SourceReadRefusalKind::NoneEffective, member)
        }
        SourceSelectionRefusalKind::ObservationFailure => {
            let kind = match refusal.failure() {
                Some(SourceFailureKind::PermissionDenied) => {
                    SourceReadRefusalKind::PermissionDenied
                }
                Some(SourceFailureKind::ChangedDuringRead) => {
                    SourceReadRefusalKind::ChangedDuringRead
                }
                Some(SourceFailureKind::UnsafeLocation) => SourceReadRefusalKind::UnsafeLocation,
                _ => SourceReadRefusalKind::OtherIo,
            };
            SourceReadRefusalV1::new(kind, member)
        }
        // A missing or undeclared observation is a caller fault: the use case below always
        // supplies exactly what the declaration asks for, so reaching one of these means an
        // invariant broke and the only safe answer is a refusal.
        SourceSelectionRefusalKind::MissingObservation
        | SourceSelectionRefusalKind::Undeclared
        | SourceSelectionRefusalKind::RedundantMarkdown => {
            SourceReadRefusalV1::new(SourceReadRefusalKind::Unsupported, member)
        }
    }
}

/// Reads one located candidate and hands it to the accepted JSON-first policy.
///
/// `json_confirmed_absent` records that a declared JSON member was positively shown not to
/// exist, which is the only condition that permits considering a Markdown body.
fn read_and_choose<P: SourceReadPort + ?Sized>(
    port: &P,
    candidates: &SourceCandidateSetV1,
    member: SourceCandidateKind,
    locator: &SourceCandidatePathV1,
    probe: ProbeFactsV1,
    json_confirmed_absent: bool,
) -> SourceReadOutcomeV1 {
    let (bytes, identity, links, changed_nanos, fingerprint) =
        match port.read_candidate(member, locator) {
            CandidateBodyV1::Present {
                bytes,
                identity,
                links,
                changed_nanos,
                fingerprint,
            } => (bytes, identity, links, changed_nanos, fingerprint),
            // The entry existed at probe time and does not now. That is a change, never an
            // absence, and it must not silently promote the other member.
            CandidateBodyV1::ConfirmedAbsent => {
                return SourceReadOutcomeV1::Refused(SourceReadRefusalV1::new(
                    SourceReadRefusalKind::ChangedDuringRead,
                    Some(member),
                ));
            }
            CandidateBodyV1::Refused(refusal) => return SourceReadOutcomeV1::Refused(refusal),
        };

    // A second name for the same physical object is refused rather than shared: exactly one
    // directory entry must own this object for it to count as this candidate's source. This
    // is decided before any change comparison, because sharing is a collision, not a change.
    if links != 1 {
        return SourceReadOutcomeV1::Refused(SourceReadRefusalV1::new(
            SourceReadRefusalKind::IdentityCollision,
            Some(member),
        ));
    }

    // The read must agree with the evidence the probe recorded, not merely with its own
    // before/after samples. A same-size replacement, or a rewrite that keeps the inode,
    // therefore shows up here instead of being returned as a trustworthy body.
    let transferred = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
    let agrees = identity == probe.identity
        && changed_nanos == probe.changed_nanos
        && transferred == probe.byte_length;
    if !agrees {
        return SourceReadOutcomeV1::Refused(SourceReadRefusalV1::new(
            SourceReadRefusalKind::ChangedDuringRead,
            Some(member),
        ));
    }

    let mut observations = SuppliedObservationsV1::new();
    match member {
        SourceCandidateKind::Json => {
            observations = observations.with_json(SourceObservationV1::Present(&bytes));
        }
        SourceCandidateKind::Markdown => {
            if json_confirmed_absent {
                observations = observations.with_json(SourceObservationV1::Absent);
            }
            observations = observations.with_markdown(SourceObservationV1::Present(&bytes));
        }
    }

    let (chosen, logical_path) = match select_provisional_source(candidates, observations) {
        Ok(choice) => (choice.kind(), choice.logical_path().to_owned()),
        Err(refusal) => return SourceReadOutcomeV1::Refused(policy_refusal(&refusal)),
    };
    if chosen != member {
        return SourceReadOutcomeV1::Refused(SourceReadRefusalV1::new(
            SourceReadRefusalKind::Unsupported,
            Some(member),
        ));
    }

    SourceReadOutcomeV1::Chosen(ReadSourceCandidateV1 {
        member,
        logical_path,
        bytes,
        identity,
        fingerprint,
    })
}

/// Reads one representation's declared candidates under the accepted JSON-first contract.
///
/// Both declared members are probed first, because identity must be comparable without
/// loading a body. Then:
///
/// 1. a probe refusal is final — nothing is read and nothing falls back;
/// 2. two declared candidates that resolve to one physical object are refused before any
///    body is read, so the unselected Markdown body is never pulled in;
/// 3. an existing JSON body is read and chosen, and Markdown is never read;
/// 4. only a positively confirmed JSON absence permits reading the declared Markdown body;
/// 5. if every declared member is positively absent, the accepted policy's own refusal is
///    reported as `SOURCE_NONE_EFFECTIVE`.
///
/// A reported absence is only ever produced by a probe that could distinguish absence from
/// every refusal, which is the adapter's obligation and is tested on both hosts.
pub fn read_declared_source_v1<P: SourceReadPort + ?Sized>(
    port: &P,
    candidates: &SourceCandidateSetV1,
) -> SourceReadOutcomeV1 {
    use SourceCandidateKind::{Json, Markdown};

    let json_probe = candidates
        .json()
        .map(|locator| (locator, port.probe_candidate(Json, locator)));
    let markdown_probe = candidates
        .markdown()
        .map(|locator| (locator, port.probe_candidate(Markdown, locator)));

    // A refusal is final: nothing is read and nothing falls back to the other member.
    if let Some((_, CandidateProbeV1::Refused(refusal))) = json_probe {
        return SourceReadOutcomeV1::Refused(refusal);
    }
    if let Some((_, CandidateProbeV1::Refused(refusal))) = markdown_probe {
        return SourceReadOutcomeV1::Refused(refusal);
    }
    if let (
        Some((
            _,
            CandidateProbeV1::Located {
                identity: json_identity,
                ..
            },
        )),
        Some((
            _,
            CandidateProbeV1::Located {
                identity: markdown_identity,
                ..
            },
        )),
    ) = (&json_probe, &markdown_probe)
        && json_identity == markdown_identity
    {
        return SourceReadOutcomeV1::Refused(SourceReadRefusalV1::new(
            SourceReadRefusalKind::IdentityCollision,
            Some(Markdown),
        ));
    }

    if let Some((locator, probe)) = &json_probe
        && let Some(evidence) = probe.located()
    {
        return read_and_choose(port, candidates, Json, locator, evidence, false);
    }
    if let Some((locator, probe)) = &markdown_probe
        && let Some(evidence) = probe.located()
    {
        let json_confirmed_absent = candidates.json().is_some();
        return read_and_choose(
            port,
            candidates,
            Markdown,
            locator,
            evidence,
            json_confirmed_absent,
        );
    }

    // Every declared member was positively absent, so the accepted policy decides, and its
    // `NeitherPresent` refusal becomes `SOURCE_NONE_EFFECTIVE` here.
    let mut observations = SuppliedObservationsV1::new();
    if candidates.json().is_some() {
        observations = observations.with_json(SourceObservationV1::Absent);
    }
    if candidates.markdown().is_some() {
        observations = observations.with_markdown(SourceObservationV1::Absent);
    }
    match select_provisional_source(candidates, observations) {
        Ok(_) => SourceReadOutcomeV1::Refused(SourceReadRefusalV1::new(
            SourceReadRefusalKind::Unsupported,
            None,
        )),
        Err(refusal) => SourceReadOutcomeV1::Refused(policy_refusal(&refusal)),
    }
}

/// One physical identity reserved by one declared candidate of one representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalReservationV1 {
    /// Index of the declaration in the submitted list that owns this reservation.
    pub declaration: usize,
    /// Which member of that declaration the identity came from.
    pub member: SourceCandidateKind,
    /// The locator that was probed.
    pub locator: SourceCandidatePathV1,
    /// The physical identity that locator resolves to.
    pub identity: PhysicalIdentityV1,
}

/// Reserves the physical identities of every declared candidate across all representations.
///
/// Comparing one representation's two members with each other is not sufficient: two
/// *different* representations can declare locators that resolve to one physical object, and
/// that would let a single authored body serve two published Pages. Every declared candidate
/// is probed once, and every identity that is found must be distinct from the others.
///
/// # Errors
///
/// Returns the first probe refusal unchanged, or `IdentityCollision` when two distinct
/// candidates resolve to the same physical object.
pub fn check_physical_reservations_v1<P: SourceReadPort + ?Sized>(
    port: &P,
    declarations: &[SourceCandidateSetV1],
) -> Result<Vec<PhysicalReservationV1>, SourceReadRefusalV1> {
    let mut reservations: Vec<PhysicalReservationV1> = Vec::new();
    for (index, candidates) in declarations.iter().enumerate() {
        for member in [SourceCandidateKind::Json, SourceCandidateKind::Markdown] {
            let locator = match member {
                SourceCandidateKind::Json => candidates.json(),
                SourceCandidateKind::Markdown => candidates.markdown(),
            };
            let Some(locator) = locator else {
                continue;
            };
            match port.probe_candidate(member, locator) {
                CandidateProbeV1::Located { identity, .. } => {
                    if reservations
                        .iter()
                        .any(|reserved| reserved.identity == identity)
                    {
                        return Err(SourceReadRefusalV1::new(
                            SourceReadRefusalKind::IdentityCollision,
                            Some(member),
                        ));
                    }
                    reservations.push(PhysicalReservationV1 {
                        declaration: index,
                        member,
                        locator: locator.clone(),
                        identity,
                    });
                }
                CandidateProbeV1::ConfirmedAbsent => {}
                CandidateProbeV1::Refused(refusal) => return Err(refusal),
            }
        }
    }
    Ok(reservations)
}
