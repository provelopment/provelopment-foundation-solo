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
    AssetNamespace, AssetReferenceV1, Exposure, PropertyKey, PropertyResolutionRefusalKind,
    PropertyResolutionRefusalV1, QualifiedOwnerV1,
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
