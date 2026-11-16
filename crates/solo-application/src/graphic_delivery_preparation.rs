//! `SOLO-WO-118 R19` — the unpublished, immutable graphic **delivery-preparation** package.
//!
//! This module is the bounded step *after* the accepted R18 mixed inspection. It turns one
//! **already accepted** whole-candidate transaction into a reproducible, privacy-screened,
//! content-addressed preparation artifact. It is deliberately **not** a publication mechanism:
//!
//! * it creates no route, URL, host origin, sitemap entry or discovery document;
//! * it writes no file at all — the payload store is in memory, so nothing can be served, mounted
//!   or accidentally committed;
//! * it exposes no writer, so no caller can overwrite a source asset, the catalogue, a licence
//!   record, a donor fixture or a previously verified immutable output;
//! * it carries no publication, route, derived-output or release authority
//!   ([`GRAPHIC_PACKAGE_GRANTS_PUBLICATION_AUTHORITY_V1`] and [`GRAPHIC_PACKAGE_ROUTES_ANY_URL_V1`]
//!   are both `false`).
//!
//! **What it does prove.** That for one explicitly scoped representation owner, the *winning
//! public* graphic slots of the accepted cascade can be materialised into exact immutable payloads:
//! an accepted Platform SVG becomes its sanitized **canonical vector bytes**, an accepted Entity
//! raster becomes a **byte-for-byte pass-through** of the very bytes that passed inspection, each
//! payload gets a deterministic content-addressed key derived from its own digest, and the whole
//! package is all-or-nothing.
//!
//! **What it does not prove.** Media correctness beyond the accepted inspector, colour or
//! orientation handling, licensing, real Page/representation visibility, routing, a public
//! response, a derivative publication or a release. Stage 04/05/07 still own those.

use solo_domain::{
    AssetNamespace, GraphicCascadeInputV1, GraphicSlotResolutionV1, GraphicSourceKindV1,
    GraphicSourceV1, PropertyKey, QualifiedOwnerV1, ResolvedGraphicOwnerV1,
    project_public_graphic_slots_v1,
};

use crate::graphic_asset_inventory::{
    MixedGraphicOriginV1, MixedGraphicPlanV1, collect_mixed_graphic_obligations_v1,
};
use crate::graphic_asset_read::GraphicAssetReadPort;
use crate::graphic_image_inspection::{
    GraphicImageFormatV3, GraphicImageInspectPortV3, GraphicImagePayloadV3, GraphicImagePayloadV4,
    GraphicMixedInspectionSetV1, GraphicMixedInspectionV1, MixedGraphicCandidateOutcomeV1,
    PlatformSourceRequestV1, inspect_mixed_graphic_candidate_staged_v1,
};
use crate::media_worker::{MediaWorkerDigestV1, MediaWorkerPortV1, sha256_media_worker_bytes_v1};
use crate::platform_asset_read::{
    APPROVED_PLATFORM_CATALOGUE_SHA256_V1, APPROVED_PLATFORM_LIBRARY_VERSION_V1,
    PlatformAssetLibraryReadPortV1,
};

/// Whether holding or exporting a package authorizes publication of any kind. Always `false`.
pub const GRAPHIC_PACKAGE_GRANTS_PUBLICATION_AUTHORITY_V1: bool = false;

/// Whether a package identifier is a routable address. Always `false`: a content key is an
/// identity, never a URL, and this module derives no host, scheme or path.
pub const GRAPHIC_PACKAGE_ROUTES_ANY_URL_V1: bool = false;

/// The hard ceiling on the number of selected entries one package may carry.
pub const MAX_PACKAGE_ENTRIES_V1: usize = 16;

/// The hard ceiling on the total staged payload bytes of one package.
pub const MAX_PACKAGE_TOTAL_BYTES_V1: u64 = 8 * 1024 * 1024;

/// The hard ceiling on one content-addressed key, in bytes.
pub const MAX_PACKAGE_CONTENT_KEY_BYTES_V1: usize = 96;

/// The one accepted prefix of a content-addressed key.
pub const PACKAGE_KEY_PREFIX_V1: &str = "sha256/";

/// The prepublication display eligibility of the scoped representation (**untrusted input**).
///
/// Authoritative Page/representation visibility and routing do not exist yet, so this value is an
/// explicitly named **test-only** stand-in: it may demonstrate the privacy boundary, it has no
/// publication capability, and it must never drive production serving.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrepublicationDisplayEligibilityV1 {
    /// The scoped representation is treated as display-eligible for this dry run.
    Public,
    /// The scoped representation is unlisted: no discoverable general-purpose entry is produced.
    Unlisted,
    /// The scoped representation is internal: no discoverable general-purpose entry is produced.
    Internal,
}

impl PrepublicationDisplayEligibilityV1 {
    /// Parses the exact case token used by the host tests.
    #[must_use]
    pub fn from_token(token: &str) -> Option<Self> {
        match token {
            "public" => Some(Self::Public),
            "unlisted" => Some(Self::Unlisted),
            "internal" => Some(Self::Internal),
            _ => None,
        }
    }

    /// The exact case token.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Public => "public",
            Self::Unlisted => "unlisted",
            Self::Internal => "internal",
        }
    }

    /// Whether this case admits any entry into the public-intent view.
    #[must_use]
    pub const fn admits_public_entries(self) -> bool {
        matches!(self, Self::Public)
    }
}

/// The bounded limits one package request may set.
///
/// The values are configuration with hard maxima: a request may tighten them, never exceed them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GraphicPackageLimitsV1 {
    max_entries: usize,
    max_total_bytes: u64,
}

impl GraphicPackageLimitsV1 {
    /// The documented hard maxima.
    #[must_use]
    pub const fn hard() -> Self {
        Self {
            max_entries: MAX_PACKAGE_ENTRIES_V1,
            max_total_bytes: MAX_PACKAGE_TOTAL_BYTES_V1,
        }
    }

    /// Validates one bounded limit pair.
    ///
    /// # Errors
    ///
    /// Returns [`GraphicPackageRefusalKindV1::InvalidLimits`] for a zero entry count or a zero or
    /// above-maximum total bound.
    pub fn new(max_entries: usize, max_total_bytes: u64) -> Result<Self, GraphicPackageRefusalV1> {
        if max_entries == 0
            || max_entries > MAX_PACKAGE_ENTRIES_V1
            || max_total_bytes == 0
            || max_total_bytes > MAX_PACKAGE_TOTAL_BYTES_V1
        {
            return Err(GraphicPackageRefusalV1::of(
                GraphicPackageRefusalKindV1::InvalidLimits,
            ));
        }
        Ok(Self {
            max_entries,
            max_total_bytes,
        })
    }

    /// The entry ceiling of this request.
    #[must_use]
    pub const fn max_entries(self) -> usize {
        self.max_entries
    }

    /// The total payload-byte ceiling of this request.
    #[must_use]
    pub const fn max_total_bytes(self) -> u64 {
        self.max_total_bytes
    }
}

/// Every way one preparation attempt can be refused. A refusal always discards **all** entries and
/// payloads: there is no partial package.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GraphicPackageRefusalKindV1 {
    /// The supplied plan is not the plan of the supplied candidate.
    PlanDoesNotMatchInput,
    /// The supplied inspection set does not correspond to the supplied plan or candidate.
    InspectionFromAnotherCandidate,
    /// The candidate does not resolve, or the scoped owner is not in the graph.
    InspectionIncomplete,
    /// A winning slot's source digest does not match the inspected source.
    SourceDigestMismatch,
    /// More than one obligation could own the winning slot, or none could be identified.
    AmbiguousOwnership,
    /// The winning slot's format cannot be materialised as output.
    UnsupportedOutputFormat,
    /// A derived content key violates the closed key grammar.
    UnsafeContentKey,
    /// The package exceeds an entry or total-byte bound.
    QuotaExceeded,
    /// The requested limits are zero or above the documented hard maxima.
    InvalidLimits,
    /// Two different payloads would share one content-addressed key.
    ContentCollision,
    /// A selected entry was not public, so exporting it would leak a private value.
    PrivateLeakage,
    /// A root-layer Platform registration is not the approved asset it claims to be.
    UnapprovedPlatformSource,
    /// A staged payload failed its own digest or length recheck.
    StageRecheckFailed,
}

impl GraphicPackageRefusalKindV1 {
    /// The stable, safe detail token.
    #[must_use]
    pub const fn detail(self) -> &'static str {
        match self {
            Self::PlanDoesNotMatchInput => "plan-does-not-match-input",
            Self::InspectionFromAnotherCandidate => "inspection-from-another-candidate",
            Self::InspectionIncomplete => "inspection-incomplete",
            Self::SourceDigestMismatch => "source-digest-mismatch",
            Self::AmbiguousOwnership => "ambiguous-ownership",
            Self::UnsupportedOutputFormat => "unsupported-output-format",
            Self::UnsafeContentKey => "unsafe-content-key",
            Self::QuotaExceeded => "quota-exceeded",
            Self::InvalidLimits => "invalid-limits",
            Self::ContentCollision => "content-collision",
            Self::PrivateLeakage => "private-leakage",
            Self::UnapprovedPlatformSource => "unapproved-platform-source",
            Self::StageRecheckFailed => "stage-recheck-failed",
        }
    }
}

/// One refused preparation attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GraphicPackageRefusalV1 {
    kind: GraphicPackageRefusalKindV1,
}

impl GraphicPackageRefusalV1 {
    /// Builds one refusal.
    #[must_use]
    pub const fn of(kind: GraphicPackageRefusalKindV1) -> Self {
        Self { kind }
    }

    /// The closed refusal reason.
    #[must_use]
    pub const fn kind(self) -> GraphicPackageRefusalKindV1 {
        self.kind
    }

    /// The stable, safe detail token.
    #[must_use]
    pub const fn detail(self) -> &'static str {
        self.kind.detail()
    }
}

impl core::fmt::Display for GraphicPackageRefusalV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "graphic-package-refused: {}", self.detail())
    }
}

impl std::error::Error for GraphicPackageRefusalV1 {}

/// One immutable staged payload. The bytes are owned here and nowhere else.
#[derive(Clone, PartialEq, Eq)]
pub struct GraphicPackagePayloadV1 {
    content_key: String,
    sha256: MediaWorkerDigestV1,
    bytes: Vec<u8>,
}

impl GraphicPackagePayloadV1 {
    /// The deterministic content-addressed key.
    #[must_use]
    pub fn content_key(&self) -> &str {
        &self.content_key
    }

    /// The independently recomputed digest of exactly these bytes.
    #[must_use]
    pub const fn sha256(&self) -> MediaWorkerDigestV1 {
        self.sha256
    }

    /// Exactly the staged bytes.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// The exact staged byte length.
    #[must_use]
    pub fn len(&self) -> u64 {
        self.bytes.len() as u64
    }

    /// Whether the payload is empty, which no accepted image is.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
}

/// The privacy-safe outward view of one prepared entry.
///
/// It carries an identity, a MIME type, intrinsic geometry and the exact output digest — and nothing
/// else. No owner, no namespace, no private ancestry, no source path, no rights-holder string and no
/// filesystem handle can reach it, so it is safe to compare in a test or inspect by hand.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicGraphicPackageEntryV1 {
    content_key: String,
    role: PropertyKey,
    slot: String,
    mime: &'static str,
    width: u32,
    height: u32,
    output_bytes: u64,
    output_sha256: String,
}

impl PublicGraphicPackageEntryV1 {
    /// The deterministic content-addressed key. An identity, **not** a URL or a path.
    #[must_use]
    pub fn content_key(&self) -> &str {
        &self.content_key
    }

    /// The graphic role.
    #[must_use]
    pub const fn role(&self) -> PropertyKey {
        self.role
    }

    /// The named slot.
    #[must_use]
    pub fn slot(&self) -> &str {
        &self.slot
    }

    /// The confirmed MIME type.
    #[must_use]
    pub const fn mime(&self) -> &'static str {
        self.mime
    }

    /// The accepted intrinsic width.
    #[must_use]
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// The accepted intrinsic height.
    #[must_use]
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// The exact output byte length.
    #[must_use]
    pub const fn output_bytes(&self) -> u64 {
        self.output_bytes
    }

    /// The exact output digest, lowercase hex.
    #[must_use]
    pub fn output_sha256(&self) -> &str {
        &self.output_sha256
    }
}

/// The privacy-safe outward view of a whole package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicGraphicPackageV1 {
    entries: Vec<PublicGraphicPackageEntryV1>,
    total_bytes: u64,
    discoverable: bool,
    publication_authority: bool,
}

impl PublicGraphicPackageV1 {
    /// Every entry, in deterministic selection order.
    #[must_use]
    pub fn entries(&self) -> &[PublicGraphicPackageEntryV1] {
        &self.entries
    }

    /// The exact aggregate payload bytes.
    #[must_use]
    pub const fn total_bytes(&self) -> u64 {
        self.total_bytes
    }

    /// Whether this view may advertise itself as a discovery source.
    ///
    /// Only a display-eligible case with at least one entry can report `true`, and even then it is a
    /// **preparation** fact, not a routing or publication grant.
    #[must_use]
    pub const fn discoverable(&self) -> bool {
        self.discoverable
    }

    /// Whether holding this view authorizes publication. Always `false`.
    #[must_use]
    pub const fn publication_authority(&self) -> bool {
        self.publication_authority
    }

    /// The entry count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the view is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl core::fmt::Debug for GraphicPackagePayloadV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Never print payload bytes: a diagnostic must not become a data channel.
        formatter
            .debug_struct("GraphicPackagePayloadV1")
            .field("content_key", &self.content_key)
            .field("sha256", &self.sha256.to_hex())
            .field("len", &self.len())
            .finish()
    }
}

/// One complete, unpublished preparation package.
///
/// Every field is private and every accessor is read-only: there is no writer, no setter, no
/// filesystem path and no serving surface. The payload store is in memory, so a package cannot be
/// mounted, served, committed or published by any code path in this module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphicDeliveryPackageV1 {
    owner: QualifiedOwnerV1,
    eligibility: PrepublicationDisplayEligibilityV1,
    entries: Vec<GraphicPackageEntryV1>,
    payloads: Vec<GraphicPackagePayloadV1>,
    total_bytes: u64,
}

impl GraphicDeliveryPackageV1 {
    /// The single scoped representation owner this package was prepared for (internal evidence).
    #[must_use]
    pub const fn owner(&self) -> &QualifiedOwnerV1 {
        &self.owner
    }

    /// The prepublication display eligibility case that was applied (**untrusted input**).
    #[must_use]
    pub const fn eligibility(&self) -> PrepublicationDisplayEligibilityV1 {
        self.eligibility
    }

    /// Every selected entry, in registry-then-slot order.
    #[must_use]
    pub fn entries(&self) -> &[GraphicPackageEntryV1] {
        &self.entries
    }

    /// Every deduplicated payload, in first-appearance order.
    #[must_use]
    pub fn payloads(&self) -> &[GraphicPackagePayloadV1] {
        &self.payloads
    }

    /// The exact aggregate payload bytes.
    #[must_use]
    pub const fn total_bytes(&self) -> u64 {
        self.total_bytes
    }

    /// The entry count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the package carries no entry.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// One payload by exact content key.
    #[must_use]
    pub fn payload(&self, content_key: &str) -> Option<&GraphicPackagePayloadV1> {
        self.payloads
            .iter()
            .find(|payload| payload.content_key == content_key)
    }

    /// The privacy-safe outward view of the whole package.
    #[must_use]
    pub fn public_intent(&self) -> PublicGraphicPackageV1 {
        let entries: Vec<PublicGraphicPackageEntryV1> = self
            .entries
            .iter()
            .map(GraphicPackageEntryV1::public_intent)
            .collect();
        PublicGraphicPackageV1 {
            discoverable: self.eligibility.admits_public_entries() && !entries.is_empty(),
            entries,
            total_bytes: self.total_bytes,
            publication_authority: GRAPHIC_PACKAGE_GRANTS_PUBLICATION_AUTHORITY_V1,
        }
    }
}

/// The closed result of one preparation attempt.
///
/// There is deliberately no partial arm: a candidate-level acquisition or inspection refusal keeps
/// its **original** typed outcome ([`MixedGraphicCandidateOutcomeV1`]) so the reason is never
/// flattened, and it carries no entry and no payload.
#[derive(Debug, Clone, PartialEq)]
pub enum GraphicDeliveryPackageOutcomeV1 {
    /// The whole candidate was accepted and every selected payload was staged.
    Prepared(Box<GraphicDeliveryPackageV1>),
    /// This module refused the preparation attempt.
    Refused(GraphicPackageRefusalV1),
    /// The accepted mixed driver refused the whole candidate, unchanged.
    SourceRefused(Box<MixedGraphicCandidateOutcomeV1>),
}

impl GraphicDeliveryPackageOutcomeV1 {
    /// The prepared package, when one exists.
    #[must_use]
    pub fn prepared(&self) -> Option<&GraphicDeliveryPackageV1> {
        match self {
            Self::Prepared(package) => Some(package),
            Self::Refused(_) | Self::SourceRefused(_) => None,
        }
    }

    /// The closed refusal, when this module refused.
    #[must_use]
    pub const fn refusal(&self) -> Option<GraphicPackageRefusalV1> {
        match self {
            Self::Refused(refusal) => Some(*refusal),
            Self::Prepared(_) | Self::SourceRefused(_) => None,
        }
    }

    /// The unchanged mixed-driver outcome, when the candidate itself was refused.
    #[must_use]
    pub fn source_refusal(&self) -> Option<&MixedGraphicCandidateOutcomeV1> {
        match self {
            Self::SourceRefused(outcome) => Some(outcome),
            Self::Prepared(_) | Self::Refused(_) => None,
        }
    }

    /// Whether the attempt produced no package at all.
    #[must_use]
    pub const fn is_refused(&self) -> bool {
        !matches!(self, Self::Prepared(_))
    }

    /// Whether holding this outcome grants publication authority. Always `false`.
    #[must_use]
    pub const fn grants_publication_authority(&self) -> bool {
        false
    }

    /// Whether this outcome assigns any routable address. Always `false`.
    #[must_use]
    pub const fn routes_any_url(&self) -> bool {
        false
    }
}

/// One complete preparation request.
///
/// The caller supplies the **already validated** candidate, the plan built from exactly that
/// candidate and the explicitly scoped representation owner; nothing here is derived from a
/// filename, a route, a locale or a guess.
pub struct GraphicPackageRequestV1<'a> {
    /// The validated R17 candidate the plan and the inspection transaction came from.
    pub input: &'a GraphicCascadeInputV1,
    /// The whole-graph mixed plan built from exactly that candidate.
    pub plan: &'a MixedGraphicPlanV1,
    /// The one fully-qualified representation owner this package is scoped to.
    pub target: &'a QualifiedOwnerV1,
    /// The test-only prepublication display eligibility case.
    pub eligibility: PrepublicationDisplayEligibilityV1,
    /// The bounded package limits.
    pub limits: GraphicPackageLimitsV1,
    /// The closed Platform source request the driver acquires.
    pub platform: PlatformSourceRequestV1<'a>,
}

/// Prepares one unpublished, immutable package from a complete accepted mixed candidate.
///
/// The whole candidate is acquired and inspected **first**, through the accepted R18 composition, so
/// every authored declaration — public, `internal`, suppressed, superseded and unrelated — must be
/// valid before a single package record exists. Only then are the winning *public* slots of the one
/// scoped representation materialised.
///
/// A caller can therefore never obtain a package from a plan that belongs to another candidate, and
/// never a package that skipped an invalid declaration.
#[must_use]
pub fn prepare_mixed_graphic_package_v1<E, P, L, A>(
    request: &GraphicPackageRequestV1<'_>,
    entity: &E,
    platform: &P,
    legacy: &L,
    avif: &A,
) -> GraphicDeliveryPackageOutcomeV1
where
    E: GraphicAssetReadPort + ?Sized,
    P: PlatformAssetLibraryReadPortV1 + ?Sized,
    L: GraphicImageInspectPortV3,
    A: MediaWorkerPortV1,
{
    // 1. The plan must be exactly the plan of this candidate: a foreign plan is refused before any
    //    source is touched, so no caller can pair one candidate's obligations with another's bytes.
    let Ok(expected) = collect_mixed_graphic_obligations_v1(request.input) else {
        return GraphicDeliveryPackageOutcomeV1::Refused(GraphicPackageRefusalV1::of(
            GraphicPackageRefusalKindV1::PlanDoesNotMatchInput,
        ));
    };
    if &expected != request.plan {
        return GraphicDeliveryPackageOutcomeV1::Refused(GraphicPackageRefusalV1::of(
            GraphicPackageRefusalKindV1::PlanDoesNotMatchInput,
        ));
    }

    // 2. The whole candidate must resolve, and the scoped owner must be part of it.
    let Ok(graph) = request.input.resolve_graphic_cascade_all_v1() else {
        return GraphicDeliveryPackageOutcomeV1::Refused(GraphicPackageRefusalV1::of(
            GraphicPackageRefusalKindV1::InspectionIncomplete,
        ));
    };
    let Some(resolved) = graph.for_owner(request.target) else {
        return GraphicDeliveryPackageOutcomeV1::Refused(GraphicPackageRefusalV1::of(
            GraphicPackageRefusalKindV1::InspectionIncomplete,
        ));
    };

    // 3. Acquire and inspect every obligation of the whole candidate, keeping the exact acquired
    //    bytes of the same in-flight transfer so no source is ever re-opened by path.
    let (set, staged) = match inspect_mixed_graphic_candidate_staged_v1(
        request.plan,
        entity,
        platform,
        request.platform,
        legacy,
        avif,
    ) {
        Ok(pair) => pair,
        Err(outcome) => {
            return GraphicDeliveryPackageOutcomeV1::SourceRefused(Box::new(outcome));
        }
    };

    // 4. Materialise the scoped public-intent selection.
    match select_package_records(request, resolved, &set, &staged) {
        Ok(package) => GraphicDeliveryPackageOutcomeV1::Prepared(Box::new(package)),
        Err(refusal) => GraphicDeliveryPackageOutcomeV1::Refused(refusal),
    }
}

/// Chooses exactly the winning public slots of the scoped owner and stages their payloads.
fn select_package_records(
    request: &GraphicPackageRequestV1<'_>,
    resolved: &ResolvedGraphicOwnerV1,
    set: &GraphicMixedInspectionSetV1,
    staged: &[Vec<u8>],
) -> Result<GraphicDeliveryPackageV1, GraphicPackageRefusalV1> {
    let empty = || GraphicDeliveryPackageV1 {
        owner: resolved.owner().clone(),
        eligibility: request.eligibility,
        entries: Vec::new(),
        payloads: Vec::new(),
        total_bytes: 0,
    };

    // A non-eligible display case produces **no** discoverable entry at all, even though the whole
    // candidate was inspected above and therefore still had to be entirely valid.
    if !request.eligibility.admits_public_entries() {
        return Ok(empty());
    }

    let projection = project_public_graphic_slots_v1(resolved);
    let mut entries: Vec<GraphicPackageEntryV1> = Vec::new();
    let mut payloads: Vec<GraphicPackagePayloadV1> = Vec::new();
    let mut total_bytes: u64 = 0;

    for role in resolved.roles() {
        for resolution in role.slots() {
            let source = resolution.source();
            // `internal` contributes nothing and never resurrects an ancestor or Platform default.
            if !source.exposure().is_public() {
                continue;
            }
            // The exported set must be exactly the accepted public projection, not a wider guess.
            let projected = projection.slots_of(role.key()).iter().any(|slot| {
                slot.slot() == resolution.slot().as_str() && slot.reference() == source.reference()
            });
            if !projected {
                return Err(GraphicPackageRefusalV1::of(
                    GraphicPackageRefusalKindV1::PrivateLeakage,
                ));
            }

            let index = locate_inspection(request.plan, set, role.key(), resolution, source)?;
            let inspection = &set.inspections()[index];
            let bytes = staged.get(index).ok_or_else(|| {
                GraphicPackageRefusalV1::of(
                    GraphicPackageRefusalKindV1::InspectionFromAnotherCandidate,
                )
            })?;

            // A root-layer Platform default must be exactly the approved asset it claims: same
            // identifier, same approved digest, same pinned catalogue and library version. A
            // registration that names an unapproved or modified asset never becomes a payload.
            if matches!(source.kind(), GraphicSourceKindV1::PlatformRootDefault) {
                let approved = source.binding().is_some_and(|binding| {
                    binding.asset_id() == source.reference()
                        && binding.asset_sha256() == inspection.source_digest().to_hex()
                        && binding.catalogue_sha256() == APPROVED_PLATFORM_CATALOGUE_SHA256_V1
                        && binding.library_version() == APPROVED_PLATFORM_LIBRARY_VERSION_V1
                });
                if !approved {
                    return Err(GraphicPackageRefusalV1::of(
                        GraphicPackageRefusalKindV1::UnapprovedPlatformSource,
                    ));
                }
            }

            let staged_digest = sha256_media_worker_bytes_v1(bytes);
            if &staged_digest != inspection.source_digest() {
                return Err(GraphicPackageRefusalV1::of(
                    GraphicPackageRefusalKindV1::SourceDigestMismatch,
                ));
            }

            let (output, output_digest) = materialise_output(inspection, bytes, staged_digest)?;
            let content_key = content_key_of(output_digest, inspection.format())?;

            // Identical final bytes and MIME may share one payload; anything else must not coalesce.
            if let Some(existing) = payloads
                .iter()
                .find(|payload| payload.content_key == content_key)
            {
                if existing.bytes != output || existing.sha256 != output_digest {
                    return Err(GraphicPackageRefusalV1::of(
                        GraphicPackageRefusalKindV1::ContentCollision,
                    ));
                }
            } else {
                total_bytes = total_bytes.saturating_add(output.len() as u64);
                if total_bytes > request.limits.max_total_bytes {
                    return Err(GraphicPackageRefusalV1::of(
                        GraphicPackageRefusalKindV1::QuotaExceeded,
                    ));
                }
                payloads.push(GraphicPackagePayloadV1 {
                    content_key: content_key.clone(),
                    sha256: output_digest,
                    bytes: output,
                });
            }

            if entries.len() >= request.limits.max_entries {
                return Err(GraphicPackageRefusalV1::of(
                    GraphicPackageRefusalKindV1::QuotaExceeded,
                ));
            }
            entries.push(GraphicPackageEntryV1 {
                content_key,
                role: role.key(),
                slot: resolution.slot().as_str().to_owned(),
                format: inspection.format(),
                mime: inspection.mime(),
                width: inspection.width(),
                height: inspection.height(),
                output_bytes: payloads
                    .iter()
                    .find(|payload| payload.sha256 == output_digest)
                    .map_or(0, GraphicPackagePayloadV1::len),
                output_sha256: output_digest,
                source_sha256: staged_digest,
                namespace: inspection.namespace(),
                winning_owner: resolution.winning_owner().cloned(),
            });
        }
    }

    // Final recheck: every staged payload must still verify against its own key.
    for payload in &payloads {
        if sha256_media_worker_bytes_v1(payload.bytes()) != payload.sha256
            || !payload.content_key.starts_with(PACKAGE_KEY_PREFIX_V1)
        {
            return Err(GraphicPackageRefusalV1::of(
                GraphicPackageRefusalKindV1::StageRecheckFailed,
            ));
        }
    }

    Ok(GraphicDeliveryPackageV1 {
        owner: resolved.owner().clone(),
        eligibility: request.eligibility,
        entries,
        payloads,
        total_bytes,
    })
}

/// Finds the one inspected obligation that actually produced this winning slot value.
///
/// A role/slot pair can be authored at several owners, so the reference, the namespace **and** the
/// authoring layer must all agree. Exactly one obligation may match; zero or several is ambiguous.
fn locate_inspection(
    plan: &MixedGraphicPlanV1,
    set: &GraphicMixedInspectionSetV1,
    role: PropertyKey,
    resolution: &GraphicSlotResolutionV1,
    source: &GraphicSourceV1,
) -> Result<usize, GraphicPackageRefusalV1> {
    let ambiguous = || GraphicPackageRefusalV1::of(GraphicPackageRefusalKindV1::AmbiguousOwnership);
    if set.len() != plan.len() {
        return Err(GraphicPackageRefusalV1::of(
            GraphicPackageRefusalKindV1::InspectionFromAnotherCandidate,
        ));
    }
    let winner = resolution.winning_owner();
    let mut found: Option<usize> = None;
    for (index, obligation) in plan.obligations().iter().enumerate() {
        if obligation.role() != role
            || obligation.slot() != Some(resolution.slot().as_str())
            || obligation.reference().reference() != source.reference()
            || obligation.namespace() != source.namespace()
        {
            continue;
        }
        let layer_matches = match source.kind() {
            GraphicSourceKindV1::PlatformRootDefault => {
                obligation.origin() == MixedGraphicOriginV1::PlatformRootRegistration
                    && obligation.declaring_owner().is_none()
            }
            GraphicSourceKindV1::AuthoredSelection => obligation.declaring_owner() == winner,
        };
        if !layer_matches {
            continue;
        }
        if found.replace(index).is_some() {
            return Err(ambiguous());
        }
    }
    found.ok_or_else(ambiguous)
}

/// Produces the exact output bytes for one inspected slot and their own digest.
///
/// A vector source becomes the accepted **canonical SVG text**, whose digest must equal the accepted
/// vector output digest; every raster source is a byte-for-byte pass-through of the very bytes that
/// passed inspection. Nothing is rasterised, re-encoded, resized or reinterpreted.
fn materialise_output(
    inspection: &GraphicMixedInspectionV1,
    source_bytes: &[u8],
    source_digest: MediaWorkerDigestV1,
) -> Result<(Vec<u8>, MediaWorkerDigestV1), GraphicPackageRefusalV1> {
    if inspection.format() == GraphicImageFormatV3::Svg {
        let GraphicImagePayloadV4::Legacy(GraphicImagePayloadV3::Vector(vector)) =
            inspection.payload()
        else {
            return Err(GraphicPackageRefusalV1::of(
                GraphicPackageRefusalKindV1::UnsupportedOutputFormat,
            ));
        };
        let canonical = vector.canonical().to_vec();
        let output_digest = sha256_media_worker_bytes_v1(&canonical);
        if output_digest != vector.output_digest || vector.source_digest != source_digest {
            return Err(GraphicPackageRefusalV1::of(
                GraphicPackageRefusalKindV1::StageRecheckFailed,
            ));
        }
        return Ok((canonical, output_digest));
    }
    if matches!(
        inspection.payload(),
        GraphicImagePayloadV4::Legacy(GraphicImagePayloadV3::Vector(_))
    ) {
        return Err(GraphicPackageRefusalV1::of(
            GraphicPackageRefusalKindV1::UnsupportedOutputFormat,
        ));
    }
    Ok((source_bytes.to_vec(), source_digest))
}

/// The canonical extension of one accepted format. A closed set, never authored text.
const fn canonical_extension(format: GraphicImageFormatV3) -> &'static str {
    match format {
        GraphicImageFormatV3::Png => "png",
        GraphicImageFormatV3::Jpeg => "jpg",
        GraphicImageFormatV3::WebP => "webp",
        GraphicImageFormatV3::Gif => "gif",
        GraphicImageFormatV3::Svg => "svg",
        GraphicImageFormatV3::Avif => "avif",
    }
}

/// Derives and validates the deterministic content-addressed key.
///
/// The key is `sha256/<64 lowercase hex>.<safe extension>`: an identity derived from the exact output
/// bytes, never a URL, never a caller-chosen path, and never a case-insensitive alias.
fn content_key_of(
    digest: MediaWorkerDigestV1,
    format: GraphicImageFormatV3,
) -> Result<String, GraphicPackageRefusalV1> {
    let key = format!(
        "{}{}.{}",
        PACKAGE_KEY_PREFIX_V1,
        digest.to_hex(),
        canonical_extension(format)
    );
    let hex = digest.to_hex();
    let safe_hex = !hex.is_empty()
        && hex.len() == 64
        && hex
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte));
    if key.len() > MAX_PACKAGE_CONTENT_KEY_BYTES_V1
        || !key.is_ascii()
        || !safe_hex
        || key.contains("..")
        || !key.starts_with(PACKAGE_KEY_PREFIX_V1)
    {
        return Err(GraphicPackageRefusalV1::of(
            GraphicPackageRefusalKindV1::UnsafeContentKey,
        ));
    }
    Ok(key)
}

///
/// It keeps the exact internal provenance (namespace, source digest, declaring owner) so a reviewer
/// can audit the transition, and it deliberately exposes a separate, narrower
/// [`PublicGraphicPackageEntryV1`] for any outward view.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphicPackageEntryV1 {
    content_key: String,
    role: PropertyKey,
    slot: String,
    format: GraphicImageFormatV3,
    mime: &'static str,
    width: u32,
    height: u32,
    output_bytes: u64,
    output_sha256: MediaWorkerDigestV1,
    source_sha256: MediaWorkerDigestV1,
    namespace: AssetNamespace,
    winning_owner: Option<QualifiedOwnerV1>,
}

impl GraphicPackageEntryV1 {
    /// The deterministic content-addressed key.
    #[must_use]
    pub fn content_key(&self) -> &str {
        &self.content_key
    }

    /// The graphic role this entry satisfies.
    #[must_use]
    pub const fn role(&self) -> PropertyKey {
        self.role
    }

    /// The named slot this entry satisfies.
    #[must_use]
    pub fn slot(&self) -> &str {
        &self.slot
    }

    /// The confirmed output format.
    #[must_use]
    pub const fn format(&self) -> GraphicImageFormatV3 {
        self.format
    }

    /// The confirmed output MIME type.
    #[must_use]
    pub const fn mime(&self) -> &'static str {
        self.mime
    }

    /// The accepted intrinsic width.
    #[must_use]
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// The accepted intrinsic height.
    #[must_use]
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// The exact output byte length.
    #[must_use]
    pub const fn output_bytes(&self) -> u64 {
        self.output_bytes
    }

    /// The digest of exactly the output bytes.
    #[must_use]
    pub const fn output_sha256(&self) -> MediaWorkerDigestV1 {
        self.output_sha256
    }

    /// The digest of exactly the source bytes that passed inspection (internal evidence).
    #[must_use]
    pub const fn source_sha256(&self) -> MediaWorkerDigestV1 {
        self.source_sha256
    }

    /// The source namespace (internal evidence).
    #[must_use]
    pub const fn namespace(&self) -> AssetNamespace {
        self.namespace
    }

    /// The owner that authored the winning value, or `None` for a root-layer default (internal).
    #[must_use]
    pub const fn winning_owner(&self) -> Option<&QualifiedOwnerV1> {
        self.winning_owner.as_ref()
    }

    /// The privacy-safe outward projection of this entry.
    #[must_use]
    pub fn public_intent(&self) -> PublicGraphicPackageEntryV1 {
        PublicGraphicPackageEntryV1 {
            content_key: self.content_key.clone(),
            role: self.role,
            slot: self.slot.clone(),
            mime: self.mime,
            width: self.width,
            height: self.height,
            output_bytes: self.output_bytes,
            output_sha256: self.output_sha256.to_hex(),
        }
    }
}
