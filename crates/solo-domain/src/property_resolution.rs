//! Direct-ancestry property resolution, provenance and public projection (`R03.A`).
//!
//! This module owns the five-level cascade of `ARCHITECTURE.md` §6.2G.2 and §6.2G.6 over
//! **exactly one** Entity's `Entity → Facet → Site → Page → localized representation`
//! chain. It is pure Rust: no JSON, filesystem, transport or provider concern reaches it.
//!
//! # The three decisions this module keeps separate
//!
//! 1. **Authorization of authored input.** [`validate_declared_properties_v1`] accepts only
//!    owners that already exist in a validated [`EfsOwnershipGraph`], so a declaration can
//!    never attach itself to a sibling, a bare Page ID, a route, a label or a hostname.
//! 2. **Resolution.** [`ValidatedPropertyInputV1::resolve`] walks the target's exact direct
//!    chain: absent keys carry the parent's effective values, explicit `inherit` carries
//!    and then appends, explicit `suppress` clears and then appends, and the registry's
//!    effective maximum and uniqueness rules are enforced at every level **before** any
//!    public filtering.
//! 3. **Projection.** [`project_public_properties_v1`] is a separate entry point that emits
//!    only `public` values in effective order. It cannot resurrect an ancestor, run a
//!    fallback or carry internal provenance.
//!
//! A resolved record is a diagnostic model, never a manifest field and never publication
//! authority: the caller still needs the separate later gates (source snapshot, routing,
//! rendering and release) that this package does not provide.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

use crate::efs_ownership::EfsOwnershipGraph;
use crate::property_registry::{
    DeclaredOwnerPropertiesV1, Exposure, FactIdV1, Inheritance, PropertyKey, PropertyValueV1,
    QualifiedOwnerV1,
};

/// A refused property resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PropertyResolutionRefusalKind {
    /// The requested owner is not a node of the one validated Entity graph.
    UnknownOwner,
    /// A declaration was attached to an owner outside the graph.
    DeclarationOwnerUnknown,
    /// Two declaration sets were supplied for the same owner.
    DuplicateDeclarationOwner,
    /// An effective collection exceeded the key's registry maximum.
    CardinalityExceeded,
    /// One effective background collection repeated an exact source reference.
    DuplicateGraphicSource,
    /// One effective collection repeated an original `(owner, factId)` location identity.
    DuplicateLocationFact,
    /// A recognized `extend` operation was reached by a real `R03.A` resolution.
    LocationExtensionNotImplementedInR03A,
    /// A supplied owner inventory did not match the graph's complete membership.
    OwnerMembershipMismatch,
    /// The resolved record described a state the validated graph cannot contain.
    InconsistentGraph,
    /// A resolved property record did not describe the value it carried.
    InconsistentRecord,
}

impl PropertyResolutionRefusalKind {
    /// Stable, safe diagnostic token for this refusal reason.
    #[must_use]
    pub const fn detail(self) -> &'static str {
        match self {
            Self::UnknownOwner => "unknown-owner",
            Self::DeclarationOwnerUnknown => "declaration-owner-unknown",
            Self::DuplicateDeclarationOwner => "duplicate-declaration-owner",
            Self::CardinalityExceeded => "cardinality-exceeded",
            Self::DuplicateGraphicSource => "duplicate-graphic-source",
            Self::DuplicateLocationFact => "duplicate-location-fact",
            Self::LocationExtensionNotImplementedInR03A => {
                "location-extension-not-implemented-in-r03a"
            }
            Self::OwnerMembershipMismatch => "owner-membership-mismatch",
            Self::InconsistentGraph => "inconsistent-graph",
            Self::InconsistentRecord => "inconsistent-record",
        }
    }
}

/// Refusal of one property resolution.
///
/// The dedicated [`PropertyResolutionRefusalKind::LocationExtensionNotImplementedInR03A`]
/// outcome is the only refusal with its own engineering code, so an unsupported `extend`
/// is never confused with a malformed declaration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PropertyResolutionRefusalV1 {
    kind: PropertyResolutionRefusalKind,
}

impl PropertyResolutionRefusalV1 {
    /// Wraps one refusal category.
    #[must_use]
    pub const fn of(kind: PropertyResolutionRefusalKind) -> Self {
        Self { kind }
    }

    /// The refusal category.
    #[must_use]
    pub const fn kind(&self) -> PropertyResolutionRefusalKind {
        self.kind
    }

    /// Stable engineering code for this refusal.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self.kind {
            PropertyResolutionRefusalKind::LocationExtensionNotImplementedInR03A => {
                "LOCATION_EXTENSION_NOT_IMPLEMENTED_IN_R03A"
            }
            _ => "PROPERTY_RESOLUTION_REFUSED",
        }
    }

    /// Stable, safe diagnostic token for the refusal reason.
    #[must_use]
    pub const fn detail(&self) -> &'static str {
        self.kind.detail()
    }
}

impl fmt::Display for PropertyResolutionRefusalV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code(), self.detail())
    }
}

impl std::error::Error for PropertyResolutionRefusalV1 {}

/// The internal field provenance of one currently set location field (§6.2H.6).
///
/// This is the single accepted field-origin shape: the owning node and the precise local
/// array position that last explicitly authored the field. It carries no operation member
/// and is never publicly projected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldOriginV1 {
    owner: QualifiedOwnerV1,
    local_index: usize,
}

impl FieldOriginV1 {
    /// Pairs one owner with one zero-based local array index.
    #[must_use]
    pub const fn new(owner: QualifiedOwnerV1, local_index: usize) -> Self {
        Self { owner, local_index }
    }

    /// The owner that authored the field.
    #[must_use]
    pub const fn owner(&self) -> &QualifiedOwnerV1 {
        &self.owner
    }

    /// The zero-based local array position that authored the field.
    #[must_use]
    pub const fn local_index(&self) -> usize {
        self.local_index
    }
}

/// One effective value with its complete internal provenance (§6.2G.6).
#[derive(Debug, Clone, PartialEq)]
pub struct ProvenanceValueV1 {
    data: PropertyValueV1,
    exposure: Exposure,
    origin_owner: QualifiedOwnerV1,
    origin_local_index: usize,
    field_origins: Option<Vec<(String, FieldOriginV1)>>,
}

impl ProvenanceValueV1 {
    fn new(
        data: PropertyValueV1,
        exposure: Exposure,
        origin_owner: QualifiedOwnerV1,
        origin_local_index: usize,
        field_origins: Option<Vec<(String, FieldOriginV1)>>,
    ) -> Self {
        Self {
            data,
            exposure,
            origin_owner,
            origin_local_index,
            field_origins,
        }
    }

    /// The fully resolved typed value.
    #[must_use]
    pub const fn data(&self) -> &PropertyValueV1 {
        &self.data
    }

    /// The authored exposure.
    #[must_use]
    pub const fn exposure(&self) -> Exposure {
        self.exposure
    }

    /// The **original** qualified owner of this value.
    #[must_use]
    pub const fn origin_owner(&self) -> &QualifiedOwnerV1 {
        &self.origin_owner
    }

    /// The original zero-based local array index of this value.
    #[must_use]
    pub const fn origin_local_index(&self) -> usize {
        self.origin_local_index
    }

    /// The per-field origins of a location fact, when this value is one.
    #[must_use]
    pub fn field_origins(&self) -> Option<&[(String, FieldOriginV1)]> {
        self.field_origins.as_deref()
    }

    fn identity_matches(&self, owner: &QualifiedOwnerV1, fact_id: &FactIdV1) -> bool {
        if &self.origin_owner != owner {
            return false;
        }
        match &self.data {
            PropertyValueV1::Location(fact) => fact.fact_id() == fact_id,
            _ => false,
        }
    }

    fn same_location_identity(&self, other: &Self) -> bool {
        if self.origin_owner != other.origin_owner {
            return false;
        }
        match (self.data.location_fact(), other.data.location_fact()) {
            (Some(left), Some(right)) => left.fact_id() == right.fact_id(),
            _ => false,
        }
    }
}

/// The operation a §6.2G.6 resolution event records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResolutionOperation {
    /// An explicit `inherit` declaration carried the inherited effective values.
    Inherited,
    /// One local value was appended.
    Added,
    /// An explicit `suppress` declaration cleared the inherited effective values.
    Suppressed,
    /// An identified inherited fact was extended in place (`R03.B` only).
    Extended,
}

impl ResolutionOperation {
    /// Stable, safe token for this operation.
    #[must_use]
    pub const fn detail(self) -> &'static str {
        match self {
            Self::Inherited => "inherited",
            Self::Added => "added",
            Self::Suppressed => "suppressed",
            Self::Extended => "extended",
        }
    }
}

/// One §6.2G.6 resolution event, in direct-ancestry then local order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolutionEventV1 {
    owner: QualifiedOwnerV1,
    operation: ResolutionOperation,
    local_index: Option<usize>,
    target_fact_id: Option<FactIdV1>,
}

impl ResolutionEventV1 {
    fn new(
        owner: QualifiedOwnerV1,
        operation: ResolutionOperation,
        local_index: Option<usize>,
        target_fact_id: Option<FactIdV1>,
    ) -> Self {
        Self {
            owner,
            operation,
            local_index,
            target_fact_id,
        }
    }

    /// The node whose declaration produced this event.
    #[must_use]
    pub const fn owner(&self) -> &QualifiedOwnerV1 {
        &self.owner
    }

    /// The recorded operation.
    #[must_use]
    pub const fn operation(&self) -> ResolutionOperation {
        self.operation
    }

    /// The zero-based local index, or `None` for an implicit mode change.
    #[must_use]
    pub const fn local_index(&self) -> Option<usize> {
        self.local_index
    }

    /// The extension target, when this event records an extension.
    #[must_use]
    pub const fn target_fact_id(&self) -> Option<&FactIdV1> {
        self.target_fact_id.as_ref()
    }
}

/// One resolved registered property, with its internal provenance record.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedPropertyV1 {
    property: PropertyKey,
    values: Vec<ProvenanceValueV1>,
    resolution_events: Vec<ResolutionEventV1>,
}

impl ResolvedPropertyV1 {
    fn new(
        property: PropertyKey,
        values: Vec<ProvenanceValueV1>,
        resolution_events: Vec<ResolutionEventV1>,
    ) -> Self {
        Self {
            property,
            values,
            resolution_events,
        }
    }

    /// The registered key of this record.
    #[must_use]
    pub const fn property(&self) -> PropertyKey {
        self.property
    }

    /// The effective values in inherited-first, then authored local order.
    #[must_use]
    pub fn values(&self) -> &[ProvenanceValueV1] {
        &self.values
    }

    /// The resolution events in direct-ancestry, then local order.
    #[must_use]
    pub fn resolution_events(&self) -> &[ResolutionEventV1] {
        &self.resolution_events
    }

    /// Whether the effective collection is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// How many effective values are `public`.
    ///
    /// The count is diagnostic only: the public projection decides what may be emitted.
    #[must_use]
    pub fn public_value_count(&self) -> usize {
        self.values
            .iter()
            .filter(|value| value.exposure().is_public())
            .count()
    }
}

/// The complete resolved property record of one qualified owner.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedPropertiesV1 {
    owner: QualifiedOwnerV1,
    properties: Vec<ResolvedPropertyV1>,
}

impl ResolvedPropertiesV1 {
    /// The owner this record resolves for.
    #[must_use]
    pub const fn owner(&self) -> &QualifiedOwnerV1 {
        &self.owner
    }

    /// All nine registered records, in registry order.
    #[must_use]
    pub fn properties(&self) -> &[ResolvedPropertyV1] {
        &self.properties
    }

    /// One registered record.
    #[must_use]
    pub fn property(&self, key: PropertyKey) -> Option<&ResolvedPropertyV1> {
        self.properties.iter().find(|record| record.property == key)
    }

    /// The effective values of one key, or an empty slice.
    #[must_use]
    pub fn values_of(&self, key: PropertyKey) -> &[ProvenanceValueV1] {
        self.property(key).map_or(&[], ResolvedPropertyV1::values)
    }
}

/// One publicly projected property.
///
/// Only the registered key and the ordered `public` values exist here: no owner, exposure,
/// origin, local index, field origin or resolution event can be read from it.
#[derive(Debug, Clone, PartialEq)]
pub struct PublicPropertyV1 {
    property: PropertyKey,
    values: Vec<PropertyValueV1>,
}

impl PublicPropertyV1 {
    /// The registered key of this record.
    #[must_use]
    pub const fn property(&self) -> PropertyKey {
        self.property
    }

    /// The `public` values in effective order.
    #[must_use]
    pub fn values(&self) -> &[PropertyValueV1] {
        &self.values
    }

    /// Whether no public value survived projection.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

/// The validated safe public projection of one resolved owner.
#[derive(Debug, Clone, PartialEq)]
pub struct PublicPropertyProjectionV1 {
    owner: QualifiedOwnerV1,
    properties: Vec<PublicPropertyV1>,
}

impl PublicPropertyProjectionV1 {
    /// The owner this projection was produced for.
    #[must_use]
    pub const fn owner(&self) -> &QualifiedOwnerV1 {
        &self.owner
    }

    /// All nine projected records, in registry order.
    #[must_use]
    pub fn properties(&self) -> &[PublicPropertyV1] {
        &self.properties
    }

    /// One projected record.
    #[must_use]
    pub fn property(&self, key: PropertyKey) -> Option<&PublicPropertyV1> {
        self.properties.iter().find(|record| record.property == key)
    }

    /// The projected public values of one key, or an empty slice.
    #[must_use]
    pub fn values_of(&self, key: PropertyKey) -> &[PropertyValueV1] {
        self.property(key).map_or(&[], PublicPropertyV1::values)
    }
}

/// Authored declarations that have been proven to belong to one validated Entity graph.
///
/// The value is built only by [`validate_declared_properties_v1`]. Because it records the
/// graph's own node set, a later [`Self::resolve`] cannot be tricked into resolving a
/// sibling, a bare Page ID or a different Entity.
#[derive(Debug, Clone, PartialEq)]
pub struct ValidatedPropertyInputV1 {
    owners: Vec<QualifiedOwnerV1>,
    declarations: Vec<DeclaredOwnerPropertiesV1>,
}

impl ValidatedPropertyInputV1 {
    /// Every node of the validated one-Entity graph, in authored traversal order.
    #[must_use]
    pub fn owners(&self) -> &[QualifiedOwnerV1] {
        &self.owners
    }

    /// The validated declaration sets.
    #[must_use]
    pub fn declarations(&self) -> &[DeclaredOwnerPropertiesV1] {
        &self.declarations
    }

    /// The declaration set of one owner, when it declares anything.
    #[must_use]
    pub fn declaration_for(&self, owner: &QualifiedOwnerV1) -> Option<&DeclaredOwnerPropertiesV1> {
        self.declarations
            .iter()
            .find(|declaration| declaration.owner() == owner)
    }

    /// Whether any validated declaration carries a recognized `extend` request.
    ///
    /// This reports the accepted-but-unimplemented operation without claiming it works and
    /// without failing an unrelated target: a real resolution that reaches the request
    /// refuses by name.
    #[must_use]
    pub fn has_recognized_location_extension(&self) -> bool {
        self.declarations.iter().any(|declaration| {
            declaration.entries().iter().any(|(_, envelope)| {
                envelope
                    .current_context()
                    .values()
                    .iter()
                    .any(|value| !value.value().is_supported_in_r03a())
            })
        })
    }

    /// Resolves all nine registered keys for one owner of this graph.
    ///
    /// The walk starts at the Entity and reuses each ancestor's resolved record, so the
    /// target's own direct chain is the only ancestry consulted. Whole-graph validation is a
    /// separate entry point ([`Self::resolve_all`]).
    ///
    /// # Errors
    ///
    /// Returns a [`PropertyResolutionRefusalV1`] for an owner that is not in the graph, an
    /// exceeded effective maximum, a duplicated background source, a duplicated location
    /// identity, or a recognized `extend` request reached by this resolution.
    pub fn resolve(
        &self,
        target: &QualifiedOwnerV1,
    ) -> Result<ResolvedPropertiesV1, PropertyResolutionRefusalV1> {
        if !self.owners.contains(target) {
            return Err(PropertyResolutionRefusalV1::of(
                PropertyResolutionRefusalKind::UnknownOwner,
            ));
        }
        let index = self.declaration_index();
        let mut resolved: Option<ResolvedPropertiesV1> = None;
        for owner in target.chain() {
            resolved = Some(self.resolve_owner(&index, &owner, resolved.as_ref())?);
        }
        resolved.ok_or_else(|| {
            PropertyResolutionRefusalV1::of(PropertyResolutionRefusalKind::InconsistentGraph)
        })
    }

    /// Resolves and validates **every** owner of the one Entity, parent first.
    ///
    /// The traversal is deterministic - the graph's own authored order, which is already
    /// parent-before-child - and each owner consumes only its validated immediate parent's
    /// effective state. Declarations are indexed once and owners are located through that
    /// index, so no owner triggers a rescan of the declaration list and nothing is quadratic.
    ///
    /// # Errors
    ///
    /// Returns a [`PropertyResolutionRefusalV1`] for **any** failure anywhere in the graph:
    /// an exceeded effective maximum, a duplicated background source, a duplicated location
    /// identity or a recognized `extend` request. A single-owner success is impossible while
    /// an unrelated Site, Page or representation is invalid.
    pub fn resolve_all(&self) -> Result<ResolvedPropertyGraphV1, PropertyResolutionRefusalV1> {
        let index = self.declaration_index();
        let positions: BTreeMap<&QualifiedOwnerV1, usize> = self
            .owners
            .iter()
            .enumerate()
            .map(|(position, owner)| (owner, position))
            .collect();
        let mut resolved: Vec<ResolvedPropertiesV1> = Vec::with_capacity(self.owners.len());
        for owner in &self.owners {
            let parent = if owner.level_index() == 0 {
                None
            } else {
                let mut chain = owner.chain();
                chain.pop();
                let parent_owner = chain.pop().ok_or_else(|| {
                    PropertyResolutionRefusalV1::of(
                        PropertyResolutionRefusalKind::InconsistentGraph,
                    )
                })?;
                let position = positions.get(&parent_owner).copied().ok_or_else(|| {
                    PropertyResolutionRefusalV1::of(
                        PropertyResolutionRefusalKind::InconsistentGraph,
                    )
                })?;
                Some(resolved.get(position).ok_or_else(|| {
                    PropertyResolutionRefusalV1::of(
                        PropertyResolutionRefusalKind::InconsistentGraph,
                    )
                })?)
            };
            let record = self.resolve_owner(&index, owner, parent)?;
            resolved.push(record);
        }
        Ok(ResolvedPropertyGraphV1 { resolved })
    }

    /// One index from owner to its validated declaration set.
    fn declaration_index(&self) -> BTreeMap<&QualifiedOwnerV1, &DeclaredOwnerPropertiesV1> {
        self.declarations
            .iter()
            .map(|declaration| (declaration.owner(), declaration))
            .collect()
    }

    /// Resolves one owner from its validated immediate parent's effective state.
    fn resolve_owner(
        &self,
        index: &BTreeMap<&QualifiedOwnerV1, &DeclaredOwnerPropertiesV1>,
        owner: &QualifiedOwnerV1,
        parent: Option<&ResolvedPropertiesV1>,
    ) -> Result<ResolvedPropertiesV1, PropertyResolutionRefusalV1> {
        let declaration = index.get(owner).copied();
        let mut properties = Vec::with_capacity(PropertyKey::ALL.len());
        for key in PropertyKey::ALL {
            properties.push(Self::resolve_key_from(parent, declaration, key, owner)?);
        }
        Ok(ResolvedPropertiesV1 {
            owner: owner.clone(),
            properties,
        })
    }

    /// Applies one owner's local declaration to its parent's effective record.
    fn resolve_key_from(
        parent: Option<&ResolvedPropertiesV1>,
        declaration: Option<&DeclaredOwnerPropertiesV1>,
        key: PropertyKey,
        owner: &QualifiedOwnerV1,
    ) -> Result<ResolvedPropertyV1, PropertyResolutionRefusalV1> {
        let inherited = parent.and_then(|record| record.property(key));
        let mut values: Vec<ProvenanceValueV1> =
            inherited.map_or_else(Vec::new, |record| record.values().to_vec());
        let mut resolution_events: Vec<ResolutionEventV1> =
            inherited.map_or_else(Vec::new, |record| record.resolution_events().to_vec());
        if let Some(envelope) = declaration.and_then(|declared| declared.declaration(key)) {
            match envelope.inheritance() {
                // `inherited` records an explicit declaration that actually carried
                // inherited values; a root declaration has nothing to inherit and records
                // only its own `added` events.
                Inheritance::Inherit => {
                    if !values.is_empty() {
                        resolution_events.push(ResolutionEventV1::new(
                            owner.clone(),
                            ResolutionOperation::Inherited,
                            None,
                            None,
                        ));
                    }
                }
                Inheritance::Suppress => {
                    values.clear();
                    resolution_events.push(ResolutionEventV1::new(
                        owner.clone(),
                        ResolutionOperation::Suppressed,
                        None,
                        None,
                    ));
                }
            }
            for (local_index, authored) in envelope.current_context().values().iter().enumerate() {
                if !authored.value().is_supported_in_r03a() {
                    return Err(PropertyResolutionRefusalV1::of(
                        PropertyResolutionRefusalKind::LocationExtensionNotImplementedInR03A,
                    ));
                }
                if let Some(fact) = authored.value().location_fact()
                    && values
                        .iter()
                        .any(|existing| existing.identity_matches(owner, fact.fact_id()))
                {
                    return Err(PropertyResolutionRefusalV1::of(
                        PropertyResolutionRefusalKind::DuplicateLocationFact,
                    ));
                }
                values.push(ProvenanceValueV1::new(
                    authored.value().clone(),
                    authored.exposure(),
                    owner.clone(),
                    local_index,
                    field_origins_for(key, owner, local_index, authored.value()),
                ));
                resolution_events.push(ResolutionEventV1::new(
                    owner.clone(),
                    ResolutionOperation::Added,
                    Some(local_index),
                    None,
                ));
            }
        }
        validate_effective(key, &values)?;
        Ok(ResolvedPropertyV1::new(key, values, resolution_events))
    }
}

/// The whole-graph resolved record of one validated one-Entity input.
///
/// Every owner of the graph appears exactly once, in the graph's deterministic parent-first
/// order, and this record exists only when **every** owner resolved without a refusal.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedPropertyGraphV1 {
    resolved: Vec<ResolvedPropertiesV1>,
}

impl ResolvedPropertyGraphV1 {
    /// Every resolved owner, in the graph's parent-first authored order.
    #[must_use]
    pub fn owners(&self) -> &[ResolvedPropertiesV1] {
        &self.resolved
    }

    /// The resolved record of one owner.
    #[must_use]
    pub fn for_owner(&self, owner: &QualifiedOwnerV1) -> Option<&ResolvedPropertiesV1> {
        self.resolved.iter().find(|record| record.owner() == owner)
    }

    /// How many owners the graph resolved.
    #[must_use]
    pub fn len(&self) -> usize {
        self.resolved.len()
    }

    /// Whether the graph resolved no owner at all.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.resolved.is_empty()
    }
}

/// Resolves and validates every owner of one validated input.
///
/// # Errors
///
/// Returns a [`PropertyResolutionRefusalV1`] for any failure anywhere in the graph, so a
/// caller can never obtain a partial success while an unrelated Site, Page or
/// representation is invalid.
pub fn resolve_all_owners_v1(
    input: &ValidatedPropertyInputV1,
) -> Result<ResolvedPropertyGraphV1, PropertyResolutionRefusalV1> {
    input.resolve_all()
}

/// Requires a caller-supplied owner inventory to be exactly the graph's membership.
///
/// This binds a parsed manifest's complete EFS membership - including every owner that
/// declares no properties - to the supplied validated graph, so a graph that is missing an
/// owner, or that carries an extra one, can never be used to resolve a property candidate.
///
/// # Errors
///
/// Returns [`PropertyResolutionRefusalKind::OwnerMembershipMismatch`] when the inventories
/// differ in size or in any member.
pub fn require_matching_owner_membership_v1(
    input: &ValidatedPropertyInputV1,
    inventory: &[QualifiedOwnerV1],
) -> Result<(), PropertyResolutionRefusalV1> {
    let mismatch =
        || PropertyResolutionRefusalV1::of(PropertyResolutionRefusalKind::OwnerMembershipMismatch);
    if inventory.len() != input.owners().len() {
        return Err(mismatch());
    }
    let expected: BTreeSet<&QualifiedOwnerV1> = input.owners().iter().collect();
    let actual: BTreeSet<&QualifiedOwnerV1> = inventory.iter().collect();
    if expected != actual {
        return Err(mismatch());
    }
    Ok(())
}

/// Enforces the registry's effective maximum and uniqueness rules on one effective set.
///
/// The check runs **before** any public projection, so a hidden `internal` value still
/// counts for a singular conflict and filtering can never mask a cardinality error.
fn validate_effective(
    key: PropertyKey,
    values: &[ProvenanceValueV1],
) -> Result<(), PropertyResolutionRefusalV1> {
    if values.len() > key.max_effective() {
        return Err(PropertyResolutionRefusalV1::of(
            PropertyResolutionRefusalKind::CardinalityExceeded,
        ));
    }
    for (index, value) in values.iter().enumerate() {
        if key == PropertyKey::GraphicBackground
            && let Some(reference) = value.data().asset_reference()
            && values[..index]
                .iter()
                .any(|earlier| earlier.data().asset_reference() == Some(reference))
        {
            return Err(PropertyResolutionRefusalV1::of(
                PropertyResolutionRefusalKind::DuplicateGraphicSource,
            ));
        }
        if values[..index]
            .iter()
            .any(|earlier| earlier.same_location_identity(value))
        {
            return Err(PropertyResolutionRefusalV1::of(
                PropertyResolutionRefusalKind::DuplicateLocationFact,
            ));
        }
    }
    Ok(())
}

/// The field origins of one locally authored location fact (§6.2H.6).
///
/// On an `add`, every populated field carries the adding node's owner and that operation's
/// array index. `R03.A` performs no extension, so no field origin can be inherited-through
/// or partially updated here.
fn field_origins_for(
    key: PropertyKey,
    owner: &QualifiedOwnerV1,
    local_index: usize,
    value: &PropertyValueV1,
) -> Option<Vec<(String, FieldOriginV1)>> {
    if !key.is_location() {
        return None;
    }
    let fact = value.location_fact()?;
    Some(
        fact.fields()
            .set_field_names()
            .into_iter()
            .map(|name| {
                (
                    name.to_owned(),
                    FieldOriginV1::new(owner.clone(), local_index),
                )
            })
            .collect(),
    )
}

/// Every node of one validated Entity graph as a qualified owner, in authored order.
fn graph_owners(graph: &EfsOwnershipGraph) -> Vec<QualifiedOwnerV1> {
    let entity = graph.entity().clone();
    let mut owners = vec![QualifiedOwnerV1::entity(entity.clone())];
    for facet in graph.facets() {
        let facet_id = facet.id().clone();
        owners.push(QualifiedOwnerV1::facet(entity.clone(), facet_id.clone()));
        for site in facet.sites() {
            let site_id = site.id().clone();
            owners.push(QualifiedOwnerV1::site(
                entity.clone(),
                facet_id.clone(),
                site_id.clone(),
            ));
            for page in site.pages() {
                let page_id = page.id().clone();
                owners.push(QualifiedOwnerV1::page(
                    entity.clone(),
                    facet_id.clone(),
                    site_id.clone(),
                    page_id.clone(),
                ));
                for locale in page.representations() {
                    owners.push(QualifiedOwnerV1::representation(
                        entity.clone(),
                        facet_id.clone(),
                        site_id.clone(),
                        page_id.clone(),
                        locale.clone(),
                    ));
                }
            }
        }
    }
    owners
}

/// Binds authored declarations to the one validated Entity graph.
///
/// # Errors
///
/// Returns [`PropertyResolutionRefusalKind::DeclarationOwnerUnknown`] when a declaration
/// names an owner the graph does not contain (a wrong Entity, a wrong Facet/Site/Page, a
/// sibling branch or a locale the Page does not declare), and
/// [`PropertyResolutionRefusalKind::DuplicateDeclarationOwner`] when one owner is supplied
/// twice.
pub fn validate_declared_properties_v1(
    graph: &EfsOwnershipGraph,
    declared: Vec<DeclaredOwnerPropertiesV1>,
) -> Result<ValidatedPropertyInputV1, PropertyResolutionRefusalV1> {
    let owners = graph_owners(graph);
    for (index, entry) in declared.iter().enumerate() {
        if !owners.contains(entry.owner()) {
            return Err(PropertyResolutionRefusalV1::of(
                PropertyResolutionRefusalKind::DeclarationOwnerUnknown,
            ));
        }
        if declared[..index]
            .iter()
            .any(|seen| seen.owner() == entry.owner())
        {
            return Err(PropertyResolutionRefusalV1::of(
                PropertyResolutionRefusalKind::DuplicateDeclarationOwner,
            ));
        }
    }
    Ok(ValidatedPropertyInputV1 {
        owners,
        declarations: declared,
    })
}

/// Projects only the genuinely `public` effective values, in effective order.
///
/// The projection is a separate decision from resolution: it drops every `internal` value
/// and all internal provenance, keeps suppression final (an empty result stays empty and no
/// ancestor is retried) and exposes neither owner, local index nor resolution event.
#[must_use]
pub fn project_public_properties_v1(resolved: &ResolvedPropertiesV1) -> PublicPropertyProjectionV1 {
    let properties = resolved
        .properties()
        .iter()
        .map(|record| PublicPropertyV1 {
            property: record.property(),
            values: record
                .values()
                .iter()
                .filter(|value| value.exposure().is_public())
                .map(|value| value.data().clone())
                .collect(),
        })
        .collect();
    PublicPropertyProjectionV1 {
        owner: resolved.owner().clone(),
        properties,
    }
}
