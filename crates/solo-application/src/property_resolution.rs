//! Inbound property-resolution use case (`R03.A`).
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
    DeclaredOwnerPropertiesV1, EfsOwnershipGraph, PropertyResolutionRefusalV1,
    PublicPropertyProjectionV1, QualifiedOwnerV1, ResolvedPropertiesV1, ValidatedPropertyInputV1,
    project_public_properties_v1, validate_declared_properties_v1,
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
/// accepts a bare Page ID, a route, a filename or a locale-as-geography selector.
#[derive(Debug, Clone)]
pub struct PropertyResolutionRequestV1<'a> {
    graph: &'a EfsOwnershipGraph,
    declared: Vec<DeclaredOwnerPropertiesV1>,
    target: QualifiedOwnerV1,
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
        }
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
}

/// The validated property candidate of one owner.
///
/// It carries the bound input (so the caller can see whether any recognized-but-unsupported
/// operation was authored), the complete internal resolved record and the separate validated
/// public projection.
#[derive(Debug, Clone, PartialEq)]
pub struct PropertyResolutionCandidateV1 {
    input: ValidatedPropertyInputV1,
    resolved: ResolvedPropertiesV1,
    public: PublicPropertyProjectionV1,
}

impl PropertyResolutionCandidateV1 {
    /// The bound, graph-validated authored input.
    #[must_use]
    pub const fn input(&self) -> &ValidatedPropertyInputV1 {
        &self.input
    }

    /// The complete internal resolved record, provenance included.
    #[must_use]
    pub const fn resolved(&self) -> &ResolvedPropertiesV1 {
        &self.resolved
    }

    /// The validated safe public projection.
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
/// maximum, when a background source or location identity repeats, or when a recognized
/// `extend` request is reached (the dedicated
/// `LOCATION_EXTENSION_NOT_IMPLEMENTED_IN_R03A` outcome).
pub fn resolve_declared_properties_v1(
    request: PropertyResolutionRequestV1<'_>,
) -> Result<PropertyResolutionCandidateV1, PropertyResolutionRefusalV1> {
    let PropertyResolutionRequestV1 {
        graph,
        declared,
        target,
    } = request;
    let input = validate_declared_properties_v1(graph, declared)?;
    let resolved = input.resolve(&target)?;
    let public = project_public_properties_v1(&resolved);
    Ok(PropertyResolutionCandidateV1 {
        input,
        resolved,
        public,
    })
}
