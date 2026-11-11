//! Inbound property-resolution use case (`R03.A`/`R03.B`).
//!
//! This module is the application boundary for the accepted typed property contract. It
//! composes the pure domain steps in one place — bind authored declarations to the one
//! validated Entity graph, resolve the target's exact direct chain, then produce the
//! separate validated public projection — and returns a typed **candidate**.
//!
//! The candidate is deliberately not publication authority: it proves nothing about the
//! source snapshot, routing, locale canonicalization, rendering or release, and the
//! constant [`PROPERTY_CANDIDATE_GRANTS_PUBLICATION_AUTHORITY`] is `false` so no caller can
//! mistake it for the publication decision. No disk, network, file or HTTP port is
//! declared, because pure property calculation needs none; the adapter crate performs the
//! outward translation.

use solo_domain::{
    DeclaredOwnerPropertiesV1, EfsOwnershipGraph, PropertyResolutionRefusalKind,
    PropertyResolutionRefusalV1, PublicPropertyProjectionV1, QualifiedOwnerV1,
    ResolvedPropertiesV1, ResolvedPropertyGraphV1, ValidatedPropertyInputV1,
    project_public_properties_v1, require_matching_owner_membership_v1, resolve_all_owners_v1,
    validate_declared_properties_v1,
};

/// Whether a validated property candidate authorizes publication.
///
/// Always `false`. A property candidate is one validated gate, never the publication
/// decision, the manifest authority or a rendered page.
pub const PROPERTY_CANDIDATE_GRANTS_PUBLICATION_AUTHORITY: bool = false;

/// One inbound property-resolution request.
///
/// The graph is the already validated one-Entity ownership structure, the declarations are
/// already typed and closed, and the target is one fully qualified owner. Nothing here
/// accepts a bare Page ID, a route, a filename or a locale-as-geography selector. A caller
/// that also holds the source manifest may supply its complete owner inventory, which is
/// then required to match the graph's membership exactly.
#[derive(Debug, Clone)]
pub struct PropertyResolutionRequestV1<'a> {
    graph: &'a EfsOwnershipGraph,
    declared: Vec<DeclaredOwnerPropertiesV1>,
    target: QualifiedOwnerV1,
    required_membership: Option<Vec<QualifiedOwnerV1>>,
}

impl<'a> PropertyResolutionRequestV1<'a> {
    /// Builds one request over an already validated graph and typed declarations.
    #[must_use]
    pub const fn new(
        graph: &'a EfsOwnershipGraph,
        declared: Vec<DeclaredOwnerPropertiesV1>,
        target: QualifiedOwnerV1,
    ) -> Self {
        Self {
            graph,
            declared,
            target,
            required_membership: None,
        }
    }

    /// Binds the caller's complete owner inventory to this request.
    ///
    /// The use case then refuses any graph whose membership differs, even when the differing
    /// owners declare no properties at all.
    #[must_use]
    pub fn with_required_membership(mut self, inventory: Vec<QualifiedOwnerV1>) -> Self {
        self.required_membership = Some(inventory);
        self
    }

    /// The validated graph this request resolves against.
    #[must_use]
    pub const fn graph(&self) -> &'a EfsOwnershipGraph {
        self.graph
    }

    /// The typed declarations supplied with this request.
    #[must_use]
    pub fn declared(&self) -> &[DeclaredOwnerPropertiesV1] {
        &self.declared
    }

    /// The fully qualified owner this request resolves.
    #[must_use]
    pub const fn target(&self) -> &QualifiedOwnerV1 {
        &self.target
    }

    /// The required complete owner inventory, when the caller supplied one.
    #[must_use]
    pub fn required_membership(&self) -> Option<&[QualifiedOwnerV1]> {
        self.required_membership.as_deref()
    }
}

/// The validated property candidate of one owner.
///
/// It carries the bound input (so the caller can see whether any recognized-but-unsupported
/// operation was authored), the **whole-graph** resolved record that had to pass first, the
/// selected owner's internal record and the separate validated public projection.
#[derive(Debug, Clone, PartialEq)]
pub struct PropertyResolutionCandidateV1 {
    input: ValidatedPropertyInputV1,
    whole: ResolvedPropertyGraphV1,
    resolved: ResolvedPropertiesV1,
    public: PublicPropertyProjectionV1,
}

impl PropertyResolutionCandidateV1 {
    /// The bound, graph-validated authored input.
    #[must_use]
    pub const fn input(&self) -> &ValidatedPropertyInputV1 {
        &self.input
    }

    /// The whole-graph resolved record that had to validate before this candidate existed.
    #[must_use]
    pub const fn resolved_graph(&self) -> &ResolvedPropertyGraphV1 {
        &self.whole
    }

    /// How many owners of the single Entity were resolved and validated.
    #[must_use]
    pub fn resolved_owner_count(&self) -> usize {
        self.whole.len()
    }

    /// The complete internal resolved record of the selected owner, provenance included.
    #[must_use]
    pub const fn resolved(&self) -> &ResolvedPropertiesV1 {
        &self.resolved
    }

    /// The validated safe public projection of the selected owner.
    #[must_use]
    pub const fn public(&self) -> &PublicPropertyProjectionV1 {
        &self.public
    }

    /// Whether this candidate authorizes publication. Always `false`.
    #[must_use]
    pub const fn is_publication_authorized(&self) -> bool {
        PROPERTY_CANDIDATE_GRANTS_PUBLICATION_AUTHORITY
    }

    /// Whether any bound declaration carried a recognized `extend` request.
    #[must_use]
    pub fn has_recognized_location_extension(&self) -> bool {
        self.input.has_recognized_location_extension()
    }
}

/// Resolves the declared properties of one fully qualified owner.
///
/// # Errors
///
/// Returns a [`PropertyResolutionRefusalV1`] when a declaration owner is outside the graph,
/// when one owner is declared twice, when an effective collection exceeds its registry
/// maximum, when a background source or location identity repeats, when a supplied owner
/// inventory differs from the graph's membership, or when an identified `location` `extend`
/// names a target it may not extend (the dedicated `LOCATION_INVALID_EXTEND` outcome).
pub fn resolve_declared_properties_v1(
    request: PropertyResolutionRequestV1<'_>,
) -> Result<PropertyResolutionCandidateV1, PropertyResolutionRefusalV1> {
    let PropertyResolutionRequestV1 {
        graph,
        declared,
        target,
        required_membership,
    } = request;
    let input = validate_declared_properties_v1(graph, declared)?;
    if let Some(inventory) = &required_membership {
        require_matching_owner_membership_v1(&input, inventory)?;
    }
    // Whole-graph, fail-closed: every owner of the single Entity is resolved and validated
    // before any successful candidate can exist. A conflict in an unrelated Site, Page or
    // representation therefore refuses the request even when the target itself is valid.
    let whole = resolve_all_owners_v1(&input)?;
    let resolved = whole
        .for_owner(&target)
        .ok_or_else(|| {
            PropertyResolutionRefusalV1::of(PropertyResolutionRefusalKind::UnknownOwner)
        })?
        .clone();
    let public = project_public_properties_v1(&resolved);
    Ok(PropertyResolutionCandidateV1 {
        input,
        whole,
        resolved,
        public,
    })
}
