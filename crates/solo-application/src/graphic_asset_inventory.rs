//! Authored graphic asset-obligation inventory (`R03.C1`).
//!
//! This module is the *nonpublishing* inventory of every independently authored graphic
//! source reference in one already validated property candidate (`R03.A`/`R03.B`). It answers
//! exactly one question — "which graphic sources did the author actually declare, and where?" —
//! and nothing else:
//!
//! * it derives from the whole-graph validated [`PropertyResolutionCandidateV1`], so an invalid
//!   declaration anywhere in the one Entity produces no inventory at all;
//! * it keeps the full qualified declaring owner, the registered graphic role, the zero-based
//!   local array index, the authored `public`/`internal` exposure and the original typed asset
//!   reference, with its explicit `entity`/`platform` namespace and unchanged reference text;
//! * it includes declarations that are invisible to the public projection: `internal` values,
//!   values on unlisted representations and values a descendant later `suppress`es;
//! * it never counts a repeated inherited copy as new authoring, never merges two distinct
//!   declarations, never invents a default, never claims an asset exists and never infers a
//!   media type from an extension.
//!
//! What is deliberately absent: any file, network, decoder, catalogue, provider or transport
//! access. A validated reference is *not* proof that a file exists, is confined, is safe media,
//! carries the declared MIME/magic match, has bounded dimensions, is licensed or belongs to the
//! pinned platform library. Those verification gates are not implemented here, and nothing in
//! this module authorizes publication.

use crate::property_resolution::PropertyResolutionCandidateV1;
use solo_domain::{
    AssetNamespace, AssetReferenceV1, Exposure, GraphicCascadeInputV1, GraphicSlotOperationV1,
    PropertyKey, PropertyResolutionRefusalKind, PropertyResolutionRefusalV1, QualifiedOwnerV1,
};

/// Whether this inventory authorizes publication of any kind.
///
/// Always `false`. The inventory is the authoritative *input* to a future separately authorized
/// confined asset-byte and pinned-platform-catalogue gate; it is not that gate, not a property
/// candidate, not a rendered value and never publication authority.
pub const GRAPHIC_ASSET_INVENTORY_GRANTS_PUBLICATION_AUTHORITY: bool = false;

/// One independently authored graphic source obligation.
///
/// The value is immutable once built: it exposes the declaring owner, the registered role, the
/// authored local index, the authored exposure and the original typed reference, but no setter
/// and no mutable access to the inherited or resolved data of the candidate it came from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphicAssetObligationV1 {
    declaring_owner: QualifiedOwnerV1,
    role: PropertyKey,
    local_index: usize,
    exposure: Exposure,
    reference: AssetReferenceV1,
}

impl GraphicAssetObligationV1 {
    /// Pairs one authored declaration with its own typed reference.
    ///
    /// Both typed halves were already validated upstream, so this constructor re-validates and
    /// repairs nothing; the invariant that the role is a registered graphic role is enforced by
    /// [`collect_graphic_asset_obligations_v1`] before any obligation is built.
    fn new(
        declaring_owner: QualifiedOwnerV1,
        role: PropertyKey,
        local_index: usize,
        exposure: Exposure,
        reference: AssetReferenceV1,
    ) -> Self {
        Self {
            declaring_owner,
            role,
            local_index,
            exposure,
            reference,
        }
    }

    /// The qualified EFS owner that originally authored this source.
    ///
    /// This is the declaring node itself, never the requested target and never a node that
    /// merely inherited the value.
    #[must_use]
    pub const fn declaring_owner(&self) -> &QualifiedOwnerV1 {
        &self.declaring_owner
    }

    /// The registered graphic role this source was authored for.
    #[must_use]
    pub const fn role(&self) -> PropertyKey {
        self.role
    }

    /// The zero-based position inside the declaring owner's authored `currentContext` array.
    #[must_use]
    pub const fn local_index(&self) -> usize {
        self.local_index
    }

    /// The exposure the author declared for this source.
    #[must_use]
    pub const fn exposure(&self) -> Exposure {
        self.exposure
    }

    /// The original typed asset reference, namespace and reference text included.
    #[must_use]
    pub const fn reference(&self) -> &AssetReferenceV1 {
        &self.reference
    }

    /// The explicit authored namespace of this source.
    ///
    /// An `entity` source and a `platform` source are never the same asset, even when their
    /// reference text is identical.
    #[must_use]
    pub const fn namespace(&self) -> AssetNamespace {
        self.reference.namespace()
    }

    /// The validated portable relative reference exactly as authored.
    #[must_use]
    pub fn reference_text(&self) -> &str {
        self.reference.reference()
    }

    /// Whether this obligation authorizes publication. Always `false`.
    #[must_use]
    pub const fn grants_publication_authority(&self) -> bool {
        GRAPHIC_ASSET_INVENTORY_GRANTS_PUBLICATION_AUTHORITY
    }
}

/// The complete authored graphic source inventory of one validated candidate.
///
/// Ordering is deterministic and independent of the requested target: declaring owners in graph
/// traversal (parent-first) order, then registry key order, then the zero-based authored local
/// index. Two collections over the same candidate compare equal.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GraphicAssetObligationInventoryV1 {
    obligations: Vec<GraphicAssetObligationV1>,
}

impl GraphicAssetObligationInventoryV1 {
    /// Every authored obligation, in deterministic inventory order.
    #[must_use]
    pub fn obligations(&self) -> &[GraphicAssetObligationV1] {
        &self.obligations
    }

    /// How many independently authored graphic sources exist.
    #[must_use]
    pub fn len(&self) -> usize {
        self.obligations.len()
    }

    /// Whether the candidate authored no graphic source at all.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.obligations.is_empty()
    }

    /// How many obligations one registered role carries.
    #[must_use]
    pub fn role_count(&self, role: PropertyKey) -> usize {
        self.obligations
            .iter()
            .filter(|obligation| obligation.role == role)
            .count()
    }

    /// Every obligation declared by one exact owner, in inventory order.
    pub fn obligations_of(
        &self,
        owner: &QualifiedOwnerV1,
    ) -> impl Iterator<Item = &GraphicAssetObligationV1> {
        self.obligations
            .iter()
            .filter(move |obligation| &obligation.declaring_owner == owner)
    }

    /// Whether this inventory authorizes publication. Always `false`.
    #[must_use]
    pub const fn grants_publication_authority(&self) -> bool {
        GRAPHIC_ASSET_INVENTORY_GRANTS_PUBLICATION_AUTHORITY
    }
}

/// Collects every independently authored graphic source of one validated candidate.
///
/// The candidate is the sole input: it has already proven complete EFS membership, closed typed
/// declarations and whole-graph resolution, including every unlisted owner. Nothing here accepts
/// raw JSON, a partial declaration list, a caller-supplied owner list, a route, a locale, a
/// directory or a catalogue, and nothing here reads a file or a network.
///
/// # Errors
///
/// Returns a [`PropertyResolutionRefusalV1`] of kind
/// [`PropertyResolutionRefusalKind::InconsistentRecord`] when a registered graphic role carries
/// a non-graphic typed value. That state is unrepresentable through the validated constructors,
/// so the refusal is a defensive fail-closed guard: an impossible typed mismatch is refused with
/// a safe structured family error instead of a silently dropped obligation. The refusal names the
/// declaring owner and key only, never an authored value.
pub fn collect_graphic_asset_obligations_v1(
    candidate: &PropertyResolutionCandidateV1,
) -> Result<GraphicAssetObligationInventoryV1, PropertyResolutionRefusalV1> {
    let input = candidate.input();
    let mut obligations: Vec<GraphicAssetObligationV1> = Vec::new();
    for owner in input.owners() {
        let Some(declared) = input.declaration_for(owner) else {
            continue;
        };
        for role in PropertyKey::ALL {
            if !role.is_graphic() {
                continue;
            }
            let Some(declaration) = declared.declaration(role) else {
                continue;
            };
            // A local `"currentContext": "none"` yields an empty slice, so it contributes no
            // obligation; an absent key never reaches this point at all.
            for (local_index, authored) in declaration.current_context().values().iter().enumerate()
            {
                let Some(reference) = authored.value().asset_reference() else {
                    return Err(PropertyResolutionRefusalV1::of(
                        PropertyResolutionRefusalKind::InconsistentRecord,
                    )
                    .with_declaration(owner, role));
                };
                obligations.push(GraphicAssetObligationV1::new(
                    owner.clone(),
                    role,
                    local_index,
                    authored.exposure(),
                    reference.clone(),
                ));
            }
        }
    }
    Ok(GraphicAssetObligationInventoryV1 { obligations })
}

// ---------------------------------------------------------------------------
// `SOLO-WO-118 R18` — the mixed Entity/Platform whole-candidate obligation plan
// ---------------------------------------------------------------------------
//
// R17's opt-in cascade decides *which* graphic source governs each role/slot. R18 answers the
// separate question a source gate must ask: **which graphic sources did this whole candidate
// declare, across both namespaces, so that every one of them can be acquired and inspected?**
//
// The plan is built from the authored inputs only — the registration table and the authored
// selections/legacy values of the whole validated graph — never from a resolved winner, a public
// projection or a visibility filter. A source that a closer supersession replaced, a source on a
// `suppress`ing owner, a source exposed only as `internal` and a source declared on an unlisted
// representation are all obligations, so no authored declaration can escape verification simply
// because something nearer replaced it.

/// Maximum independently authored graphic obligations one mixed plan may carry.
///
/// The bound is deliberately finite and independent of any file, directory or catalogue scan.
pub const MAX_MIXED_GRAPHIC_OBLIGATIONS_V1: usize = 64;

/// Where one mixed obligation came from in the authored candidate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MixedGraphicOriginV1 {
    /// A root-layer registration in the accepted R17 table: a `platform` source with no EFS owner.
    PlatformRootRegistration,
    /// An authored R17 slot selection at one qualified EFS owner.
    AuthoredSelection,
    /// An accepted legacy graphic value of a parity (non-opt-in) candidate.
    LegacyEnvelopeValue,
}

impl MixedGraphicOriginV1 {
    /// The stable, safe origin token.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PlatformRootRegistration => "platform-root-registration",
            Self::AuthoredSelection => "authored-selection",
            Self::LegacyEnvelopeValue => "legacy-envelope-value",
        }
    }
}

/// The closed authored operation one obligation carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MixedGraphicOperationV1 {
    /// The root-layer registration itself.
    Registration,
    /// An explicit closer supersession of the named role/slot.
    Supersede,
    /// An additive selection of the named role/slot.
    Append,
    /// An accepted legacy additive graphic value, which carries no slot identity.
    LegacyAdd,
}

impl MixedGraphicOperationV1 {
    /// The stable, safe operation token.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Registration => "registration",
            Self::Supersede => "supersede",
            Self::Append => "append",
            Self::LegacyAdd => "legacy-add",
        }
    }
}

/// One independently authored graphic source the whole candidate depends on.
///
/// The value keeps the exact namespace, the registered role, the explicit slot (absent only for a
/// legacy value, which has no slot identity), the qualified declaring owner (absent only for a
/// root-layer registration, which is not an EFS node), the authored local ordinal, the authored
/// exposure and the original typed reference. It carries no bytes, no path and no authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MixedGraphicObligationV1 {
    origin: MixedGraphicOriginV1,
    operation: MixedGraphicOperationV1,
    role: PropertyKey,
    slot: Option<String>,
    declaring_owner: Option<QualifiedOwnerV1>,
    local_index: usize,
    exposure: Exposure,
    reference: AssetReferenceV1,
}

impl MixedGraphicObligationV1 {
    /// The authored origin of this obligation.
    #[must_use]
    pub const fn origin(&self) -> MixedGraphicOriginV1 {
        self.origin
    }

    /// The authored operation of this obligation.
    #[must_use]
    pub const fn operation(&self) -> MixedGraphicOperationV1 {
        self.operation
    }

    /// The registered graphic role.
    #[must_use]
    pub const fn role(&self) -> PropertyKey {
        self.role
    }

    /// The explicit slot, when the authored format names one.
    #[must_use]
    pub fn slot(&self) -> Option<&str> {
        self.slot.as_deref()
    }

    /// The qualified EFS owner that authored this source, absent for the root layer.
    #[must_use]
    pub const fn declaring_owner(&self) -> Option<&QualifiedOwnerV1> {
        self.declaring_owner.as_ref()
    }

    /// The authored local ordinal inside that owner's own declaration list.
    #[must_use]
    pub const fn local_index(&self) -> usize {
        self.local_index
    }

    /// The authored exposure.
    #[must_use]
    pub const fn exposure(&self) -> Exposure {
        self.exposure
    }

    /// Whether this source was authored as `internal`.
    #[must_use]
    pub const fn is_internal(&self) -> bool {
        matches!(self.exposure, Exposure::Internal)
    }

    /// The original typed reference, with its explicit namespace.
    #[must_use]
    pub const fn reference(&self) -> &AssetReferenceV1 {
        &self.reference
    }

    /// The explicit namespace of this obligation, taken from the typed reference alone.
    #[must_use]
    pub const fn namespace(&self) -> AssetNamespace {
        self.reference.namespace()
    }

    /// The accepted Entity-side obligation value, when this source has an EFS owner.
    ///
    /// This exists so the accepted AVIF parent-check can record the **real** authored obligation as
    /// its own provenance. A root-layer registration has no EFS owner and therefore no carrier, so
    /// it can never be given a fabricated one.
    #[must_use]
    pub(crate) fn authored_obligation_v1(&self) -> Option<GraphicAssetObligationV1> {
        self.declaring_owner.as_ref().map(|owner| {
            GraphicAssetObligationV1::new(
                owner.clone(),
                self.role,
                self.local_index,
                self.exposure,
                self.reference.clone(),
            )
        })
    }
}

/// Why a whole-candidate mixed obligation plan was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MixedGraphicPlanRefusalKindV1 {
    /// A slot-aware candidate also authored a legacy unqualified graphic value, so the two formats
    /// would be double-counted or silently reinterpreted.
    AmbiguousGraphicFormat,
    /// The candidate declares more obligations than the accepted bound.
    PlanOversize,
}

impl MixedGraphicPlanRefusalKindV1 {
    /// The stable, safe detail token.
    #[must_use]
    pub const fn detail(self) -> &'static str {
        match self {
            Self::AmbiguousGraphicFormat => "ambiguous-graphic-format",
            Self::PlanOversize => "plan-oversize",
        }
    }
}

/// One refused mixed obligation plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MixedGraphicPlanRefusalV1 {
    kind: MixedGraphicPlanRefusalKindV1,
}

impl MixedGraphicPlanRefusalV1 {
    /// Builds one refusal.
    #[must_use]
    pub const fn of(kind: MixedGraphicPlanRefusalKindV1) -> Self {
        Self { kind }
    }

    /// The closed refusal reason.
    #[must_use]
    pub const fn kind(self) -> MixedGraphicPlanRefusalKindV1 {
        self.kind
    }

    /// The stable, safe detail token.
    #[must_use]
    pub const fn detail(self) -> &'static str {
        self.kind.detail()
    }
}

impl core::fmt::Display for MixedGraphicPlanRefusalV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "mixed-graphic-plan-refused: {}", self.detail())
    }
}

impl std::error::Error for MixedGraphicPlanRefusalV1 {}

/// The deterministic, closed source-obligation plan of one whole candidate.
///
/// Order is fixed and reproducible: the root-layer registrations in registration order, then the
/// validated graph's own parent-first owner order, and inside each owner its legacy values (parity
/// candidates only) and then its authored selections in authored order. Holding a plan proves only
/// which sources were declared; it proves nothing about their existence, media, licence or use.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MixedGraphicPlanV1 {
    obligations: Vec<MixedGraphicObligationV1>,
}

impl MixedGraphicPlanV1 {
    /// An empty plan: the parity candidate that declares no graphic source at all.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            obligations: Vec::new(),
        }
    }

    /// Every obligation, in the plan's deterministic order.
    #[must_use]
    pub fn obligations(&self) -> &[MixedGraphicObligationV1] {
        &self.obligations
    }

    /// How many independently authored sources the candidate declared.
    #[must_use]
    pub fn len(&self) -> usize {
        self.obligations.len()
    }

    /// Whether the candidate declared no graphic source at all.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.obligations.is_empty()
    }

    /// How many obligations live in the Entity namespace.
    #[must_use]
    pub fn entity_count(&self) -> usize {
        self.obligations
            .iter()
            .filter(|obligation| obligation.namespace() == AssetNamespace::Entity)
            .count()
    }

    /// How many obligations live in the Platform namespace.
    #[must_use]
    pub fn platform_count(&self) -> usize {
        self.obligations
            .iter()
            .filter(|obligation| obligation.namespace() == AssetNamespace::Platform)
            .count()
    }

    /// Whether this plan authorizes publication. Always `false`.
    #[must_use]
    pub const fn grants_publication_authority(&self) -> bool {
        GRAPHIC_ASSET_INVENTORY_GRANTS_PUBLICATION_AUTHORITY
    }

    /// The Entity-namespace obligations as the accepted Entity read gate's own inventory.
    ///
    /// The accepted gate refuses any inventory that contains a `platform` obligation, so the mixed
    /// composition hands it exactly the Entity subset and keeps the Platform subset for the
    /// Platform reader. A root-layer registration can never appear here: it is a `platform` source
    /// by construction and has no EFS owner.
    #[must_use]
    pub(crate) fn entity_inventory_v1(&self) -> GraphicAssetObligationInventoryV1 {
        let obligations = self
            .obligations
            .iter()
            .filter(|obligation| obligation.namespace() == AssetNamespace::Entity)
            .filter_map(MixedGraphicObligationV1::authored_obligation_v1)
            .collect();
        GraphicAssetObligationInventoryV1 { obligations }
    }
}

/// Builds the deterministic source-obligation plan of one whole validated candidate.
///
/// The R17 input is the sole source of declarations: the registration table and the authored
/// selections/legacy values of every owner of the one validated graph. No resolution winner, public
/// projection, exposure filter or visibility state narrows the set, so a superseded, suppressed or
/// `internal` declaration is planned exactly like a governing one.
///
/// # Errors
///
/// Returns [`MixedGraphicPlanRefusalKindV1::AmbiguousGraphicFormat`] when a slot-aware candidate
/// also authors a legacy unqualified graphic value — the two formats would otherwise be
/// double-counted or silently reinterpreted — and
/// [`MixedGraphicPlanRefusalKindV1::PlanOversize`] past
/// [`MAX_MIXED_GRAPHIC_OBLIGATIONS_V1`].
pub fn collect_mixed_graphic_obligations_v1(
    input: &GraphicCascadeInputV1,
) -> Result<MixedGraphicPlanV1, MixedGraphicPlanRefusalV1> {
    let mut obligations: Vec<MixedGraphicObligationV1> = Vec::new();
    let oversize = || MixedGraphicPlanRefusalV1::of(MixedGraphicPlanRefusalKindV1::PlanOversize);
    let ambiguous =
        || MixedGraphicPlanRefusalV1::of(MixedGraphicPlanRefusalKindV1::AmbiguousGraphicFormat);

    // 1. The root layer, in registration order. A registered default is an authored Platform
    //    declaration even when a closer suppress or supersede means it does not govern, so it is
    //    always planned and always verified.
    for record in input.baseline().records() {
        if obligations.len() >= MAX_MIXED_GRAPHIC_OBLIGATIONS_V1 {
            return Err(oversize());
        }
        obligations.push(MixedGraphicObligationV1 {
            origin: MixedGraphicOriginV1::PlatformRootRegistration,
            operation: MixedGraphicOperationV1::Registration,
            role: record.key(),
            slot: Some(record.slot().as_str().to_owned()),
            declaring_owner: None,
            local_index: 0,
            // A registered root default is a public generic approved asset, exactly as R17
            // resolves it; this states the authored exposure, not a visibility decision.
            exposure: Exposure::Public,
            reference: record.reference().clone(),
        });
    }

    // 2. The whole validated graph, parent first, in each owner's own authored order.
    let slot_aware = input.is_slot_aware();
    for owner in input.validated().owners() {
        if let Some(declared) = input.validated().declaration_for(owner) {
            for role in PropertyKey::ALL {
                if !role.is_graphic() {
                    continue;
                }
                let Some(declaration) = declared.declaration(role) else {
                    continue;
                };
                for (local_index, authored) in
                    declaration.current_context().values().iter().enumerate()
                {
                    let Some(reference) = authored.value().asset_reference() else {
                        continue;
                    };
                    // A slot-aware candidate must express every graphic source as a slot
                    // selection. An unqualified legacy value here is ambiguous, never merged.
                    if slot_aware {
                        return Err(ambiguous());
                    }
                    if obligations.len() >= MAX_MIXED_GRAPHIC_OBLIGATIONS_V1 {
                        return Err(oversize());
                    }
                    obligations.push(MixedGraphicObligationV1 {
                        origin: MixedGraphicOriginV1::LegacyEnvelopeValue,
                        operation: MixedGraphicOperationV1::LegacyAdd,
                        role,
                        slot: None,
                        declaring_owner: Some(owner.clone()),
                        local_index,
                        exposure: authored.exposure(),
                        reference: reference.clone(),
                    });
                }
            }
        }
        if let Some(block) = input
            .selections()
            .iter()
            .find(|candidate| candidate.owner() == owner)
        {
            for (local_index, selection) in block.selections().iter().enumerate() {
                if obligations.len() >= MAX_MIXED_GRAPHIC_OBLIGATIONS_V1 {
                    return Err(oversize());
                }
                obligations.push(MixedGraphicObligationV1 {
                    origin: MixedGraphicOriginV1::AuthoredSelection,
                    operation: match selection.operation() {
                        GraphicSlotOperationV1::Supersede => MixedGraphicOperationV1::Supersede,
                        GraphicSlotOperationV1::Append => MixedGraphicOperationV1::Append,
                    },
                    role: selection.key(),
                    slot: Some(selection.slot().as_str().to_owned()),
                    declaring_owner: Some(owner.clone()),
                    local_index,
                    exposure: selection.exposure(),
                    reference: selection.reference().clone(),
                });
            }
        }
    }

    Ok(MixedGraphicPlanV1 { obligations })
}
