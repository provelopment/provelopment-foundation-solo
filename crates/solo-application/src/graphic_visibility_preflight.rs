//! Graph-bound visibility preflight over the accepted unpublished graphic package (`R04.C1`).
//!
//! This wrapper binds one already validated `GraphicCascadeInputV1`, the mixed plan of **that**
//! candidate and one fully qualified representation target to the immutable
//! [`RepresentationVisibilityLedgerV1`] of the same EFS graph, derives the R19 prepublication
//! display intent **only** from that ledger, and then calls the unchanged
//! [`prepare_mixed_graphic_package_v1`] exactly once.
//!
//! It copies no staging algorithm, opens no second source handle, re-reads no byte, re-runs no
//! sanitizer, re-encodes no raster, mints no default and changes no content key. A caller's
//! claim that a representation is `public` is insufficient: the effective state comes from the
//! ledger, and the ledger is checked against the candidate's whole declared owner membership
//! **before** any root is opened or any worker is launched.
//!
//! # What this is not
//!
//! `public` is discoverable *intent*, not a discovery document; `unlisted` is *not* secret and
//! not permanently non-serveable, but it is never automatically discoverable and this wrapper
//! can produce no route for it; `internal` remains a property-value exposure and is never a
//! third representation state. The result view carries no source path, no private property
//! value, no hidden qualified owner, no hostname, no alias and no token that could be passed as
//! a public URL, and every authority boolean on it is `false`.

use solo_domain::{
    AssetNamespace, EfsLevel, GraphicCascadeInputV1, PropertyKey, QualifiedOwnerV1,
    RepresentationVisibilityLedgerV1, RepresentationVisibilityOriginV1, RepresentationVisibilityV1,
};

use crate::graphic_asset_inventory::MixedGraphicPlanV1;
use crate::graphic_asset_read::GraphicAssetReadPort;
use crate::graphic_delivery_preparation::{
    GRAPHIC_PACKAGE_GRANTS_PUBLICATION_AUTHORITY_V1, GraphicDeliveryPackageOutcomeV1,
    GraphicPackageLimitsV1, GraphicPackageRefusalV1, GraphicPackageRequestV1,
    PrepublicationDisplayEligibilityV1, prepare_mixed_graphic_package_v1,
};
use crate::graphic_image_inspection::{
    GraphicImageInspectPortV3, MixedGraphicCandidateOutcomeV1, PlatformSourceRequestV1,
};
use crate::media_worker::MediaWorkerPortV1;
use crate::platform_asset_read::PlatformAssetLibraryReadPortV1;

/// This preflight grants no publication authority. Always `false`.
pub const GRAPHIC_VISIBILITY_PREFLIGHT_GRANTS_PUBLICATION_AUTHORITY_V1: bool = false;

/// This preflight routes no URL. Always `false`.
pub const GRAPHIC_VISIBILITY_PREFLIGHT_ROUTES_ANY_URL_V1: bool = false;

// The accepted package underneath must never carry publication authority either; that is the
// premise this wrapper would be unsafe to build on, so it is asserted at compile time.
const _: () = assert!(!GRAPHIC_PACKAGE_GRANTS_PUBLICATION_AUTHORITY_V1);

/// Why a preflight was refused before the accepted package path was reached.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphicVisibilityPreflightRefusalKindV1 {
    /// The requested owner is not exactly one localized representation.
    TargetNotRepresentation,
    /// The ledger does not govern the requested representation.
    TargetNotInLedger,
    /// The ledger belongs to another Entity than the candidate's validated owners.
    LedgerForeignEntity,
    /// The ledger does not cover every Page/representation the candidate declares.
    LedgerPartial,
}

impl GraphicVisibilityPreflightRefusalKindV1 {
    /// The stable diagnostic token.
    #[must_use]
    pub const fn detail(self) -> &'static str {
        match self {
            Self::TargetNotRepresentation => "target-not-representation",
            Self::TargetNotInLedger => "target-not-in-ledger",
            Self::LedgerForeignEntity => "ledger-foreign-entity",
            Self::LedgerPartial => "ledger-partial",
        }
    }
}

/// One refused preflight, attributable to the owner it refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphicVisibilityPreflightRefusalV1 {
    kind: GraphicVisibilityPreflightRefusalKindV1,
    owner: Option<Box<QualifiedOwnerV1>>,
}

impl GraphicVisibilityPreflightRefusalV1 {
    /// A refusal of one closed kind.
    #[must_use]
    pub const fn of(kind: GraphicVisibilityPreflightRefusalKindV1) -> Self {
        Self { kind, owner: None }
    }

    /// Attributes this refusal to the owner it refused.
    #[must_use]
    pub fn with_owner(mut self, owner: &QualifiedOwnerV1) -> Self {
        self.owner = Some(Box::new(owner.clone()));
        self
    }

    /// The closed cause.
    #[must_use]
    pub const fn kind(&self) -> GraphicVisibilityPreflightRefusalKindV1 {
        self.kind
    }

    /// The exact owner this refusal concerns, when known.
    #[must_use]
    pub fn owner(&self) -> Option<&QualifiedOwnerV1> {
        self.owner.as_deref()
    }

    /// The stable diagnostic token.
    #[must_use]
    pub const fn detail(&self) -> &'static str {
        self.kind.detail()
    }
}

/// One closed preflight request.
pub struct GraphicVisibilityPreflightRequestV1<'a> {
    /// The validated R17 candidate the plan and the acquisition came from.
    pub input: &'a GraphicCascadeInputV1,
    /// The whole-graph mixed plan built from exactly that candidate.
    pub plan: &'a MixedGraphicPlanV1,
    /// The one fully qualified localized representation this preflight is scoped to.
    pub target: &'a QualifiedOwnerV1,
    /// The immutable, graph-bound visibility ledger of the same EFS graph.
    pub ledger: &'a RepresentationVisibilityLedgerV1,
    /// The bounded package limits.
    pub limits: GraphicPackageLimitsV1,
    /// The closed Platform source request the accepted driver acquires.
    pub platform: PlatformSourceRequestV1<'a>,
}

/// One staged payload, described only by safe identity facts.
///
/// It carries the role, slot, namespace, canonical MIME, bounded dimensions, byte length and the
/// exact source and output digests. It carries **no** filesystem path, no rights-holder data, no
/// hidden qualified owner and no address: a content key is an identity, never a URL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphicVisibilityPreflightEntryV1 {
    role: PropertyKey,
    slot: String,
    namespace: AssetNamespace,
    mime: &'static str,
    content_key: String,
    source_sha256: String,
    output_sha256: String,
    width: u32,
    height: u32,
    output_bytes: u64,
}

impl GraphicVisibilityPreflightEntryV1 {
    /// The property role this payload serves.
    #[must_use]
    pub const fn role(&self) -> PropertyKey {
        self.role
    }

    /// The exact slot inside that role.
    #[must_use]
    pub fn slot(&self) -> &str {
        &self.slot
    }

    /// The authored namespace the winning source came from.
    #[must_use]
    pub const fn namespace(&self) -> AssetNamespace {
        self.namespace
    }

    /// The canonical MIME of the staged bytes.
    #[must_use]
    pub const fn mime(&self) -> &'static str {
        self.mime
    }

    /// The derived immutable content identity.
    #[must_use]
    pub fn content_key(&self) -> &str {
        &self.content_key
    }

    /// The accepted source digest that was inspected.
    #[must_use]
    pub fn source_sha256(&self) -> &str {
        &self.source_sha256
    }

    /// The exact digest of the staged output bytes.
    #[must_use]
    pub fn output_sha256(&self) -> &str {
        &self.output_sha256
    }

    /// The staged pixel width.
    #[must_use]
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// The staged pixel height.
    #[must_use]
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// The staged byte length.
    #[must_use]
    pub const fn output_bytes(&self) -> u64 {
        self.output_bytes
    }
}

/// One closed, result-only preflight view.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphicVisibilityPreflightV1 {
    owner: QualifiedOwnerV1,
    visibility: RepresentationVisibilityV1,
    origin: RepresentationVisibilityOriginV1,
    entries: Vec<GraphicVisibilityPreflightEntryV1>,
    total_bytes: u64,
}

impl GraphicVisibilityPreflightV1 {
    /// The exact representation this result is bound to.
    #[must_use]
    pub const fn owner(&self) -> &QualifiedOwnerV1 {
        &self.owner
    }

    /// The ledger's closed effective visibility for that representation.
    #[must_use]
    pub const fn visibility(&self) -> RepresentationVisibilityV1 {
        self.visibility
    }

    /// Where that effective state came from.
    #[must_use]
    pub const fn origin(&self) -> RepresentationVisibilityOriginV1 {
        self.origin
    }

    /// Discoverable *intent* only — never a discovery document.
    #[must_use]
    pub const fn is_discoverable_intent(&self) -> bool {
        self.visibility.is_discoverable_intent()
    }

    /// The robot *intent* this state would require — never a response header.
    #[must_use]
    pub const fn robots_intent(&self) -> &'static str {
        self.visibility.robots_intent()
    }

    /// The staged payloads of the prepublication package.
    #[must_use]
    pub fn entries(&self) -> &[GraphicVisibilityPreflightEntryV1] {
        &self.entries
    }

    /// The total staged byte length.
    #[must_use]
    pub const fn total_bytes(&self) -> u64 {
        self.total_bytes
    }

    /// How many payloads were staged.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether nothing was staged (always the case for `unlisted`).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// This result grants no publication authority. Always `false`.
    #[must_use]
    pub const fn grants_publication_authority(&self) -> bool {
        false
    }

    /// This result routes no URL. Always `false`.
    #[must_use]
    pub const fn routes_any_url(&self) -> bool {
        false
    }

    /// This result authorizes no release. Always `false`.
    #[must_use]
    pub const fn grants_release_authority(&self) -> bool {
        false
    }

    /// This result publishes no derived output. Always `false`.
    #[must_use]
    pub const fn publishes_derived_output(&self) -> bool {
        false
    }
}

/// The closed outcome of one visibility preflight.
#[derive(Debug, Clone)]
pub enum GraphicVisibilityPreflightOutcomeV1 {
    /// The bound preflight completed and the accepted package path ran.
    Prepared(GraphicVisibilityPreflightV1),
    /// Binding was refused before any root was opened or worker launched.
    Refused(GraphicVisibilityPreflightRefusalV1),
    /// The accepted preparation refused the package.
    PackageRefused(GraphicPackageRefusalV1),
    /// The accepted composition refused the candidate's sources.
    SourceRefused(Box<MixedGraphicCandidateOutcomeV1>),
}

impl GraphicVisibilityPreflightOutcomeV1 {
    /// The prepared result, when one exists.
    #[must_use]
    pub fn prepared(&self) -> Option<&GraphicVisibilityPreflightV1> {
        match self {
            Self::Prepared(preflight) => Some(preflight),
            _ => None,
        }
    }

    /// The binding refusal, when one exists.
    #[must_use]
    pub fn refusal(&self) -> Option<GraphicVisibilityPreflightRefusalV1> {
        match self {
            Self::Refused(refusal) => Some(refusal.clone()),
            _ => None,
        }
    }

    /// The accepted package refusal, when one exists.
    #[must_use]
    pub const fn package_refusal(&self) -> Option<GraphicPackageRefusalV1> {
        match self {
            Self::PackageRefused(refusal) => Some(*refusal),
            _ => None,
        }
    }

    /// The accepted source refusal, when one exists.
    #[must_use]
    pub fn source_refusal(&self) -> Option<&MixedGraphicCandidateOutcomeV1> {
        match self {
            Self::SourceRefused(refusal) => Some(refusal.as_ref()),
            _ => None,
        }
    }

    /// Whether nothing was prepared.
    #[must_use]
    pub const fn is_refused(&self) -> bool {
        !matches!(self, Self::Prepared(_))
    }

    /// This outcome grants no publication authority. Always `false`.
    #[must_use]
    pub const fn grants_publication_authority(&self) -> bool {
        false
    }

    /// This outcome routes no URL. Always `false`.
    #[must_use]
    pub const fn routes_any_url(&self) -> bool {
        false
    }
}

/// Prepares one visibility-bound, unpublished graphic preflight.
///
/// The binding is proved **first**, over pure values only: the target must be exactly one
/// localized representation, the ledger must govern it, the ledger's Entity must be the
/// candidate's Entity, and every Page and representation the candidate declares must be
/// governed by that ledger. Only then is the accepted
/// [`prepare_mixed_graphic_package_v1`] invoked, exactly once, with the display intent derived
/// from the ledger. A caller therefore cannot borrow another candidate's ledger, promote an
/// arbitrary untrusted enum, or reach a reader/worker with a misbound target.
#[must_use]
pub fn prepare_graphic_visibility_preflight_v1<E, P, L, A>(
    request: &GraphicVisibilityPreflightRequestV1<'_>,
    entity: &E,
    platform: &P,
    legacy: &L,
    avif: &A,
) -> GraphicVisibilityPreflightOutcomeV1
where
    E: GraphicAssetReadPort + ?Sized,
    P: PlatformAssetLibraryReadPortV1 + ?Sized,
    L: GraphicImageInspectPortV3,
    A: MediaWorkerPortV1,
{
    use GraphicVisibilityPreflightRefusalKindV1 as Kind;

    // 1. The selected owner must be one localized representation of one Page.
    if request.target.level() != EfsLevel::Representation {
        return GraphicVisibilityPreflightOutcomeV1::Refused(
            GraphicVisibilityPreflightRefusalV1::of(Kind::TargetNotRepresentation)
                .with_owner(request.target),
        );
    }

    // 2. The ledger must belong to the candidate's Entity and cover its whole declared
    //    Page/representation membership. A foreign or partial ledger is refused here, before any
    //    root is opened or any worker is launched.
    let owners = request.input.validated().owners();
    let Some(first) = owners.first() else {
        return GraphicVisibilityPreflightOutcomeV1::Refused(
            GraphicVisibilityPreflightRefusalV1::of(Kind::LedgerPartial).with_owner(request.target),
        );
    };
    if request.ledger.entity_id() != first.entity_id() {
        return GraphicVisibilityPreflightOutcomeV1::Refused(
            GraphicVisibilityPreflightRefusalV1::of(Kind::LedgerForeignEntity)
                .with_owner(request.target),
        );
    }
    for owner in owners {
        let covered = match owner.level() {
            EfsLevel::Representation => request.ledger.for_owner(owner).is_some(),
            EfsLevel::Page => request.ledger.covers_page(owner),
            _ => true,
        };
        if !covered {
            return GraphicVisibilityPreflightOutcomeV1::Refused(
                GraphicVisibilityPreflightRefusalV1::of(Kind::LedgerPartial).with_owner(owner),
            );
        }
    }

    // 3. The selected representation must itself be governed by that ledger.
    let Some(entry) = request.ledger.for_owner(request.target) else {
        return GraphicVisibilityPreflightOutcomeV1::Refused(
            GraphicVisibilityPreflightRefusalV1::of(Kind::TargetNotInLedger)
                .with_owner(request.target),
        );
    };

    // 4. The display intent is derived from the ledger alone. `internal` is not a manifest
    //    representation visibility, so it can never arrive here.
    let eligibility = match entry.effective() {
        RepresentationVisibilityV1::Public => PrepublicationDisplayEligibilityV1::Public,
        RepresentationVisibilityV1::Unlisted => PrepublicationDisplayEligibilityV1::Unlisted,
    };
    let visibility = entry.effective();
    let origin = entry.origin();

    // 5. The accepted, unchanged preparation runs exactly once.
    let package_request = GraphicPackageRequestV1 {
        input: request.input,
        plan: request.plan,
        target: request.target,
        eligibility,
        limits: request.limits,
        platform: request.platform,
    };
    match prepare_mixed_graphic_package_v1(&package_request, entity, platform, legacy, avif) {
        GraphicDeliveryPackageOutcomeV1::Prepared(package) => {
            let entries = package
                .entries()
                .iter()
                .map(|entry| GraphicVisibilityPreflightEntryV1 {
                    role: entry.role(),
                    slot: entry.slot().to_owned(),
                    namespace: entry.namespace(),
                    mime: entry.mime(),
                    content_key: entry.content_key().to_owned(),
                    source_sha256: entry.source_sha256().to_hex(),
                    output_sha256: entry.output_sha256().to_hex(),
                    width: entry.width(),
                    height: entry.height(),
                    output_bytes: entry.output_bytes(),
                })
                .collect();
            GraphicVisibilityPreflightOutcomeV1::Prepared(GraphicVisibilityPreflightV1 {
                owner: request.target.clone(),
                visibility,
                origin,
                entries,
                total_bytes: package.total_bytes(),
            })
        }
        GraphicDeliveryPackageOutcomeV1::Refused(refusal) => {
            GraphicVisibilityPreflightOutcomeV1::PackageRefused(refusal)
        }
        GraphicDeliveryPackageOutcomeV1::SourceRefused(refusal) => {
            GraphicVisibilityPreflightOutcomeV1::SourceRefused(refusal)
        }
    }
}
