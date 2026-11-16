//! Direct-ancestry property resolution, provenance and public projection (`R03.A`/`R03.B`).
//!
//! This module owns the five-level cascade of `ARCHITECTURE.md` §6.2G.2 and §6.2G.6 over
//! **exactly one** Entity's `Entity → Facet → Site → Page → localized representation`
//! chain, including the identified-record `extend` of §6.2H.4–§6.2H.5. It is pure Rust: no
//! JSON, filesystem, transport or provider concern reaches it.
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
    AssetNamespace, AssetReferenceV1, DeclaredOwnerPropertiesV1, Exposure, FactIdV1, Inheritance,
    LocationExtensionRequestV1, PropertyKey, PropertyValueV1, QualifiedOwnerV1,
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
    /// A `location` `extend` operation named a target it may not extend (§6.2H.4–§6.2H.5).
    InvalidLocationExtension,
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
            Self::InvalidLocationExtension => "invalid-location-extension",
            Self::OwnerMembershipMismatch => "owner-membership-mismatch",
            Self::InconsistentGraph => "inconsistent-graph",
            Self::InconsistentRecord => "inconsistent-record",
        }
    }
}

/// The typed cause of an invalid `location` `extend` (§6.2H.4–§6.2H.5).
///
/// Each token is stable, bounded and structural: it describes the relationship that failed and
/// never carries an authored field value, a place name, a coordinate or a locale.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocationExtensionCause {
    /// The named target is not a strict direct ancestor of the declaring node.
    TargetNotAncestor,
    /// No presently inherited fact carries that original owner and fact identifier.
    TargetNotInherited,
    /// This node's own `suppress` removed the fact the extension names.
    TargetSuppressed,
    /// This node already extended that original fact once.
    DuplicateExtensionTarget,
}

impl LocationExtensionCause {
    /// Stable, safe diagnostic token for this cause.
    #[must_use]
    pub const fn detail(self) -> &'static str {
        match self {
            Self::TargetNotAncestor => "target-not-ancestor",
            Self::TargetNotInherited => "target-not-inherited",
            Self::TargetSuppressed => "target-suppressed",
            Self::DuplicateExtensionTarget => "duplicate-extension-target",
        }
    }
}

/// Refusal of one property resolution.
///
/// An invalid `location` extension keeps its own engineering code, so a refused operation is
/// never confused with a malformed declaration. The refusal also records the offending
/// declaring owner and registered key when known, so a diagnostic names the node whose
/// declaration actually failed rather than an unrelated requested target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropertyResolutionRefusalV1 {
    kind: PropertyResolutionRefusalKind,
    cause: Option<LocationExtensionCause>,
    owner: Option<Box<QualifiedOwnerV1>>,
    property: Option<PropertyKey>,
}

impl PropertyResolutionRefusalV1 {
    /// Wraps one refusal category.
    #[must_use]
    pub const fn of(kind: PropertyResolutionRefusalKind) -> Self {
        Self {
            kind,
            cause: None,
            owner: None,
            property: None,
        }
    }

    /// Records the typed extension cause.
    #[must_use]
    pub const fn with_cause(mut self, cause: LocationExtensionCause) -> Self {
        self.cause = Some(cause);
        self
    }

    /// Records the offending declaring owner and registered key when they are not known yet.
    #[must_use]
    pub fn with_declaration(mut self, owner: &QualifiedOwnerV1, property: PropertyKey) -> Self {
        if self.owner.is_none() {
            self.owner = Some(Box::new(owner.clone()));
        }
        if self.property.is_none() {
            self.property = Some(property);
        }
        self
    }

    /// The typed extension cause, when this refusal is an invalid extension.
    #[must_use]
    pub const fn cause(&self) -> Option<LocationExtensionCause> {
        self.cause
    }

    /// Stable, safe diagnostic token for the typed extension cause, when present.
    #[must_use]
    pub fn cause_detail(&self) -> Option<&'static str> {
        self.cause.map(LocationExtensionCause::detail)
    }

    /// The offending declaring owner, when known.
    #[must_use]
    pub fn owner(&self) -> Option<&QualifiedOwnerV1> {
        self.owner.as_deref()
    }

    /// The registered key this refusal applies to, when known.
    #[must_use]
    pub const fn property(&self) -> Option<PropertyKey> {
        self.property
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
            PropertyResolutionRefusalKind::InvalidLocationExtension => "LOCATION_INVALID_EXTEND",
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

    /// Whether any validated declaration carries a `location` `extend` operation.
    ///
    /// This is an inventory of authored operations, not a verdict: every extension is still
    /// resolved against its own target and refused when that target is not extendable.
    #[must_use]
    pub fn has_recognized_location_extension(&self) -> bool {
        self.declarations.iter().any(|declaration| {
            declaration.entries().iter().any(|(_, envelope)| {
                envelope
                    .current_context()
                    .values()
                    .iter()
                    .any(|value| value.value().is_location_extension())
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
            // A refusal is attributed to the node whose declaration produced it, so a
            // diagnostic never blames the requested target for a distant failure.
            let record = Self::resolve_key_from(parent, declaration, key, owner)
                .map_err(|refusal| refusal.with_declaration(owner, key))?;
            properties.push(record);
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
            // The identities this node's own `suppress` removed. They are kept only long enough
            // to refuse an `extend` naming one of them with the accurate cause instead of a
            // vague "missing" one; nothing here can restore a suppressed value.
            let mut suppressed_here: Vec<ProvenanceValueV1> = Vec::new();
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
                    suppressed_here = core::mem::take(&mut values);
                    resolution_events.push(ResolutionEventV1::new(
                        owner.clone(),
                        ResolutionOperation::Suppressed,
                        None,
                        None,
                    ));
                }
            }
            // Facts carried from the immediate parent occupy the prefix `[0, inherited_len)`;
            // anything appended below is authored at this node and is therefore never eligible
            // as an extension target at the same node.
            let inherited_len = values.len();
            let mut extended_targets: Vec<(QualifiedOwnerV1, FactIdV1)> = Vec::new();
            for (local_index, authored) in envelope.current_context().values().iter().enumerate() {
                if key.is_location()
                    && let PropertyValueV1::LocationExtension(request) = authored.value()
                {
                    apply_location_extension(
                        request,
                        owner,
                        local_index,
                        &mut values,
                        inherited_len,
                        &suppressed_here,
                        &mut extended_targets,
                    )?;
                    resolution_events.push(ResolutionEventV1::new(
                        owner.clone(),
                        ResolutionOperation::Extended,
                        Some(local_index),
                        Some(request.target_fact_id().clone()),
                    ));
                    continue;
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

/// Applies one `extend` operation to the identified inherited fact, in place (§6.2H.4–§6.2H.5).
///
/// The named target must be a strict direct ancestor of the declaring node, the fact must sit
/// in the immediate parent's effective set and must still be inherited here, and one node may
/// extend one original fact only once. On success only the members `set` explicitly names are
/// written; the fact keeps its identifier, kind, original owner/index, exposure and position,
/// and only the written members' field origins move to this node and this local index.
fn apply_location_extension(
    request: &LocationExtensionRequestV1,
    owner: &QualifiedOwnerV1,
    local_index: usize,
    values: &mut [ProvenanceValueV1],
    inherited_len: usize,
    suppressed_here: &[ProvenanceValueV1],
    extended_targets: &mut Vec<(QualifiedOwnerV1, FactIdV1)>,
) -> Result<(), PropertyResolutionRefusalV1> {
    let target_owner = request.target_owner();
    let target_fact_id = request.target_fact_id();
    if !target_owner.is_ancestor_of(owner) {
        return Err(extension_refusal(LocationExtensionCause::TargetNotAncestor));
    }
    let position = values
        .iter()
        .enumerate()
        .find(|(index, value)| {
            *index < inherited_len && value.identity_matches(target_owner, target_fact_id)
        })
        .map(|(index, _)| index);
    let Some(position) = position else {
        let cause = if suppressed_here
            .iter()
            .any(|value| value.identity_matches(target_owner, target_fact_id))
        {
            LocationExtensionCause::TargetSuppressed
        } else {
            LocationExtensionCause::TargetNotInherited
        };
        return Err(extension_refusal(cause));
    };
    let identity = (
        values[position].origin_owner().clone(),
        target_fact_id.clone(),
    );
    if extended_targets.contains(&identity) {
        return Err(extension_refusal(
            LocationExtensionCause::DuplicateExtensionTarget,
        ));
    }
    let Some(existing) = values[position].data().location_fact() else {
        return Err(extension_refusal(
            LocationExtensionCause::TargetNotInherited,
        ));
    };
    let set = request.set();
    let set_names = set.set_field_names();
    let updated = existing.with_overlay(set);
    let field_origins: Vec<(String, FieldOriginV1)> = updated
        .fields()
        .set_field_names()
        .into_iter()
        .map(|name| {
            let origin = if set_names.contains(&name) {
                FieldOriginV1::new(owner.clone(), local_index)
            } else {
                values[position]
                    .field_origins()
                    .and_then(|origins| {
                        origins
                            .iter()
                            .find(|(field, _)| field == name)
                            .map(|(_, origin)| origin.clone())
                    })
                    .unwrap_or_else(|| FieldOriginV1::new(owner.clone(), local_index))
            };
            (name.to_owned(), origin)
        })
        .collect();
    extended_targets.push(identity);
    values[position] = ProvenanceValueV1::new(
        PropertyValueV1::location(updated),
        values[position].exposure(),
        values[position].origin_owner().clone(),
        values[position].origin_local_index(),
        Some(field_origins),
    );
    Ok(())
}

/// One invalid-extension refusal with its typed cause.
fn extension_refusal(cause: LocationExtensionCause) -> PropertyResolutionRefusalV1 {
    PropertyResolutionRefusalV1::of(PropertyResolutionRefusalKind::InvalidLocationExtension)
        .with_cause(cause)
}

/// The field origins of one locally authored location fact (§6.2H.6).
///
/// On an `add`, every populated field carries the adding node's owner and that operation's
/// array index. An `extend` never calls this: it updates only the origins of the members it
/// explicitly sets and leaves every untouched field origin exactly as inherited.
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

// ---------------------------------------------------------------------------
// `SOLO-WO-118 R17` — §6.2I.2 governed Platform-first graphic cascade
// ---------------------------------------------------------------------------
//
// This is an **additive, opt-in** slice. Nothing above this line changes: the legacy
// additive resolver, its refusals, its provenance and its public projection keep their
// exact semantics, so a project that adopts nothing resolves byte-identically. Only a
// caller that supplies a non-empty registration table or an explicit slot selection
// enters slot-aware resolution.
//
// The three decisions this slice keeps separate:
//
// 1. **Registration is configuration, never inference.** A role/slot default exists only
//    because an exact record in [`PlatformGraphicBaselineV1`] names it, and that record is
//    eligible only when a verified approved-catalogue binding proves the exact identifier,
//    digest and media class. No filename, count, sort order, route, role name or asset
//    presence can create a default.
// 2. **A closer explicit supersession wins; nothing else does.** A slot changes only
//    through an authored [`GraphicSlotOperationV1::Supersede`] (replace) or
//    [`GraphicSlotOperationV1::Append`] (fill an empty slot). An invalid closer selection
//    refuses the whole candidate rather than reviving the root default.
// 3. **Suppression is final within its scope.** `suppress` discards inherited and
//    root-layer contributions for its key, then permits valid explicitly authored local
//    values; it never re-establishes a default by itself.

/// The only slot a singular graphic role may occupy.
pub const GRAPHIC_PRIMARY_SLOT_V1: &str = "primary";

/// Maximum length of one graphic slot identifier, in ASCII bytes.
pub const MAX_GRAPHIC_SLOT_BYTES_V1: usize = 63;

/// Maximum number of registered Platform graphic baseline records.
pub const MAX_PLATFORM_BASELINE_RECORDS_V1: usize = 64;

/// Maximum number of approved Platform assets one verified binding may carry.
pub const MAX_APPROVED_PLATFORM_ASSETS_V1: usize = 128;

/// Maximum number of slot selections one owner may author.
pub const MAX_OWNER_GRAPHIC_SELECTIONS_V1: usize = 16;

/// Maximum number of supersession lineage steps: the root layer plus five EFS levels.
pub const MAX_GRAPHIC_LINEAGE_STEPS_V1: usize = 6;

/// A successful graphic cascade grants **no** publication authority.
pub const GRAPHIC_CASCADE_GRANTS_PUBLICATION_AUTHORITY_V1: bool = false;

/// A refused graphic-cascade operation or input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GraphicCascadeRefusalKind {
    /// A slot identifier violated the closed grammar.
    InvalidSlotIdentifier,
    /// A non-`primary` slot was named for a singular graphic role.
    SlotNotPermittedForRole,
    /// The registration table repeated one role/slot pair.
    DuplicateBaselineRecord,
    /// A baseline record named a non-graphic property key.
    BaselineRoleNotGraphic,
    /// A baseline reference was not in the Platform namespace.
    BaselineNamespaceAlias,
    /// A baseline reference named an asset the verified binding does not approve.
    BaselineAssetUnapproved,
    /// A baseline record's declared digest disagreed with the approved catalogue digest.
    BaselineDigestMismatch,
    /// The registration's library version or catalogue pin disagreed with the binding.
    BaselineBindingMismatch,
    /// A baseline asset's media class is not permitted for that graphic role.
    BaselineRoleFormatMismatch,
    /// The registration table exceeded its bound.
    BaselineTableOversize,
    /// One owner named the same role/slot twice.
    DuplicateGraphicSelection,
    /// A legacy unqualified graphic value was authored in slot-aware mode.
    UnqualifiedGraphicEntry,
    /// A role carried more slots than its registered capacity.
    SlotCapacityExceeded,
    /// An `append` targeted a slot that is already occupied.
    SlotAlreadyFilled,
    /// A graphic selection named an owner outside the validated graph.
    SelectionOwnerUnknown,
    /// The requested owner is not a node of the one validated Entity graph.
    UnknownOwner,
    /// The resolved state described something the validated graph cannot contain.
    InconsistentGraph,
}

impl GraphicCascadeRefusalKind {
    /// Stable, safe diagnostic token for this refusal reason.
    #[must_use]
    pub const fn detail(self) -> &'static str {
        match self {
            Self::InvalidSlotIdentifier => "invalid-slot-identifier",
            Self::SlotNotPermittedForRole => "slot-not-permitted-for-role",
            Self::DuplicateBaselineRecord => "duplicate-baseline-record",
            Self::BaselineRoleNotGraphic => "baseline-role-not-graphic",
            Self::BaselineNamespaceAlias => "baseline-namespace-alias",
            Self::BaselineAssetUnapproved => "baseline-asset-unapproved",
            Self::BaselineDigestMismatch => "baseline-digest-mismatch",
            Self::BaselineBindingMismatch => "baseline-binding-mismatch",
            Self::BaselineRoleFormatMismatch => "baseline-role-format-mismatch",
            Self::BaselineTableOversize => "baseline-table-oversize",
            Self::DuplicateGraphicSelection => "duplicate-graphic-selection",
            Self::UnqualifiedGraphicEntry => "unqualified-graphic-entry",
            Self::SlotCapacityExceeded => "slot-capacity-exceeded",
            Self::SlotAlreadyFilled => "slot-already-filled",
            Self::SelectionOwnerUnknown => "selection-owner-unknown",
            Self::UnknownOwner => "unknown-owner",
            Self::InconsistentGraph => "inconsistent-graph",
        }
    }
}

/// One refused graphic-cascade operation or input.
///
/// The diagnostic carries only the closed reason, the offending registered key and (when the
/// failure belongs to an authored selection) the qualified owner. It never carries an authored
/// reference, a filesystem path, a media byte or a private provenance record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphicCascadeRefusalV1 {
    kind: GraphicCascadeRefusalKind,
    key: Option<PropertyKey>,
    owner: Option<Box<QualifiedOwnerV1>>,
}

impl GraphicCascadeRefusalV1 {
    /// Builds one refusal from its closed reason.
    #[must_use]
    pub const fn of(kind: GraphicCascadeRefusalKind) -> Self {
        Self {
            kind,
            key: None,
            owner: None,
        }
    }

    /// Records the role whose registration or selection failed.
    #[must_use]
    pub const fn with_key(mut self, key: PropertyKey) -> Self {
        self.key = Some(key);
        self
    }

    /// Records the owner whose selection failed.
    #[must_use]
    pub fn with_owner(mut self, owner: &QualifiedOwnerV1) -> Self {
        self.owner = Some(Box::new(owner.clone()));
        self
    }

    /// The closed refusal reason.
    #[must_use]
    pub const fn kind(&self) -> GraphicCascadeRefusalKind {
        self.kind
    }

    /// The offending registered key, when the failure belongs to one.
    #[must_use]
    pub const fn key(&self) -> Option<PropertyKey> {
        self.key
    }

    /// The stable, safe detail token.
    #[must_use]
    pub const fn detail(&self) -> &'static str {
        self.kind.detail()
    }
}

impl fmt::Display for GraphicCascadeRefusalV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "graphic-cascade-refused: {}", self.detail())
    }
}

impl std::error::Error for GraphicCascadeRefusalV1 {}

/// One validated graphic slot identifier.
///
/// The grammar is closed and lowercase only: 1–63 ASCII bytes, first byte `a`–`z`, remaining
/// bytes `a`–`z`, `0`–`9` or `-`, with no leading, trailing or adjacent hyphen. Because
/// uppercase never parses, two spellings can never differ only by case, so a case collision is
/// impossible rather than merely detected.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GraphicSlotIdV1(String);

impl GraphicSlotIdV1 {
    /// Validates one authored slot identifier.
    ///
    /// # Errors
    ///
    /// Returns [`GraphicCascadeRefusalKind::InvalidSlotIdentifier`] for an empty, overlong,
    /// non-ASCII, uppercase or badly hyphenated identifier.
    pub fn parse(text: &str) -> Result<Self, GraphicCascadeRefusalV1> {
        let refused =
            || GraphicCascadeRefusalV1::of(GraphicCascadeRefusalKind::InvalidSlotIdentifier);
        if text.is_empty() || text.len() > MAX_GRAPHIC_SLOT_BYTES_V1 || !text.is_ascii() {
            return Err(refused());
        }
        let bytes = text.as_bytes();
        if !bytes[0].is_ascii_lowercase() {
            return Err(refused());
        }
        for (position, byte) in bytes.iter().enumerate() {
            if !(byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'-') {
                return Err(refused());
            }
            if *byte == b'-' {
                let at_edge = position == 0 || position + 1 == bytes.len();
                if at_edge || bytes[position - 1] == b'-' {
                    return Err(refused());
                }
            }
        }
        Ok(Self(text.to_owned()))
    }

    /// The canonical primary slot.
    #[must_use]
    pub fn primary() -> Self {
        Self(GRAPHIC_PRIMARY_SLOT_V1.to_owned())
    }

    /// The validated identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Whether this is the primary slot.
    #[must_use]
    pub fn is_primary(&self) -> bool {
        self.0 == GRAPHIC_PRIMARY_SLOT_V1
    }
}

impl fmt::Display for GraphicSlotIdV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// The closed media classes an approved Platform asset may carry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PlatformAssetMediaClassV1 {
    /// A static SVG document.
    Svg,
    /// A PNG raster image.
    Png,
    /// A JPEG raster image.
    Jpeg,
    /// A WebP raster image.
    Webp,
    /// An AVIF raster image.
    Avif,
    /// A GIF image.
    Gif,
}

impl PlatformAssetMediaClassV1 {
    /// The class of one lowercase extension, when the extension is a registered class.
    #[must_use]
    pub fn from_extension(extension: &str) -> Option<Self> {
        match extension {
            "svg" => Some(Self::Svg),
            "png" => Some(Self::Png),
            "jpg" | "jpeg" => Some(Self::Jpeg),
            "webp" => Some(Self::Webp),
            "avif" => Some(Self::Avif),
            "gif" => Some(Self::Gif),
            _ => None,
        }
    }

    /// The stable class token.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Svg => "svg",
            Self::Png => "png",
            Self::Jpeg => "jpeg",
            Self::Webp => "webp",
            Self::Avif => "avif",
            Self::Gif => "gif",
        }
    }

    /// Whether this class is permitted for one graphic role.
    ///
    /// This mirrors the registry's own value/key contract: `graphic.favicon` is PNG only, and
    /// every other graphic role accepts the admitted image classes.
    #[must_use]
    pub const fn permitted_for(self, key: PropertyKey) -> bool {
        match key {
            PropertyKey::GraphicFavicon => matches!(self, Self::Png),
            _ => true,
        }
    }
}

/// One asset the verified approved Platform catalogue proves.
///
/// Holding one proves only that these three facts were established inward from the transferred
/// catalogue bytes: the exact identifier, its declared SHA-256 and its media class. It proves no
/// media validity, no read, no route and no publication authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApprovedPlatformAssetV1 {
    id: String,
    sha256: String,
    media_class: PlatformAssetMediaClassV1,
}

impl ApprovedPlatformAssetV1 {
    /// Builds one verified asset record from already established facts.
    ///
    /// The identifier must be a valid Platform-namespace asset reference, its own extension must
    /// agree with `media_class`, and the digest must be 64 lowercase hexadecimal characters.
    ///
    /// # Errors
    ///
    /// Returns [`GraphicCascadeRefusalKind::BaselineNamespaceAlias`],
    /// [`GraphicCascadeRefusalKind::BaselineAssetUnapproved`] or
    /// [`GraphicCascadeRefusalKind::BaselineRoleFormatMismatch`].
    pub fn new(
        id: &str,
        sha256: &str,
        media_class: PlatformAssetMediaClassV1,
    ) -> Result<Self, GraphicCascadeRefusalV1> {
        let reference = AssetReferenceV1::parse(AssetNamespace::Platform, id).map_err(|_| {
            GraphicCascadeRefusalV1::of(GraphicCascadeRefusalKind::BaselineNamespaceAlias)
        })?;
        if !is_lowercase_hex_digest(sha256) {
            return Err(GraphicCascadeRefusalV1::of(
                GraphicCascadeRefusalKind::BaselineAssetUnapproved,
            ));
        }
        let extension = reference_extension(&reference).ok_or_else(|| {
            GraphicCascadeRefusalV1::of(GraphicCascadeRefusalKind::BaselineAssetUnapproved)
        })?;
        if PlatformAssetMediaClassV1::from_extension(extension) != Some(media_class) {
            return Err(GraphicCascadeRefusalV1::of(
                GraphicCascadeRefusalKind::BaselineRoleFormatMismatch,
            ));
        }
        Ok(Self {
            id: id.to_owned(),
            sha256: sha256.to_owned(),
            media_class,
        })
    }

    /// The exact catalogue identifier.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// The declared SHA-256 of the asset's own bytes.
    #[must_use]
    pub fn sha256(&self) -> &str {
        &self.sha256
    }

    /// The asset's media class.
    #[must_use]
    pub const fn media_class(&self) -> PlatformAssetMediaClassV1 {
        self.media_class
    }
}

/// The verified approved Platform catalogue a registration may bind to.
///
/// This value is built only by an application boundary that independently verified the
/// transferred catalogue bytes against the accepted pin and that observed at least one accepted
/// confined source read. It is an inward-bound *fact set*, never a lookup registry: nothing here
/// discovers files, consults a route or grants publication.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApprovedPlatformGraphicBindingV1 {
    library_version: String,
    catalogue_sha256: String,
    assets: Vec<ApprovedPlatformAssetV1>,
}

impl ApprovedPlatformGraphicBindingV1 {
    /// Binds one verified library version, catalogue digest and approved asset set.
    ///
    /// # Errors
    ///
    /// Returns [`GraphicCascadeRefusalKind::BaselineBindingMismatch`] for an empty or malformed
    /// version or digest, [`GraphicCascadeRefusalKind::BaselineAssetUnapproved`] for an empty or
    /// duplicate asset set, and [`GraphicCascadeRefusalKind::BaselineTableOversize`] when the set
    /// exceeds [`MAX_APPROVED_PLATFORM_ASSETS_V1`].
    pub fn new(
        library_version: &str,
        catalogue_sha256: &str,
        assets: Vec<ApprovedPlatformAssetV1>,
    ) -> Result<Self, GraphicCascadeRefusalV1> {
        let refused =
            || GraphicCascadeRefusalV1::of(GraphicCascadeRefusalKind::BaselineBindingMismatch);
        if library_version.is_empty()
            || library_version.len() > 16
            || !library_version
                .bytes()
                .all(|byte| byte.is_ascii_digit() || byte == b'.')
            || !is_lowercase_hex_digest(catalogue_sha256)
        {
            return Err(refused());
        }
        if assets.is_empty() {
            return Err(GraphicCascadeRefusalV1::of(
                GraphicCascadeRefusalKind::BaselineAssetUnapproved,
            ));
        }
        if assets.len() > MAX_APPROVED_PLATFORM_ASSETS_V1 {
            return Err(GraphicCascadeRefusalV1::of(
                GraphicCascadeRefusalKind::BaselineTableOversize,
            ));
        }
        for (position, asset) in assets.iter().enumerate() {
            if assets[..position]
                .iter()
                .any(|seen| seen.id() == asset.id())
            {
                return Err(GraphicCascadeRefusalV1::of(
                    GraphicCascadeRefusalKind::BaselineAssetUnapproved,
                ));
            }
        }
        Ok(Self {
            library_version: library_version.to_owned(),
            catalogue_sha256: catalogue_sha256.to_owned(),
            assets,
        })
    }

    /// The verified library version.
    #[must_use]
    pub fn library_version(&self) -> &str {
        &self.library_version
    }

    /// The verified catalogue SHA-256.
    #[must_use]
    pub fn catalogue_sha256(&self) -> &str {
        &self.catalogue_sha256
    }

    /// Every verified approved asset, in catalogue order.
    #[must_use]
    pub fn assets(&self) -> &[ApprovedPlatformAssetV1] {
        &self.assets
    }

    /// The one approved asset with this exact identifier, never a near match.
    #[must_use]
    pub fn exact(&self, id: &str) -> Option<&ApprovedPlatformAssetV1> {
        self.assets.iter().find(|asset| asset.id() == id)
    }
}

/// The lowercase extension of one validated reference.
fn reference_extension(reference: &AssetReferenceV1) -> Option<&str> {
    reference.reference().rsplit_once('.').map(|(_, tail)| tail)
}

/// Whether one digest is exactly 64 lowercase hexadecimal characters.
fn is_lowercase_hex_digest(text: &str) -> bool {
    text.len() == 64
        && text
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

/// One registered Platform graphic baseline record.
///
/// A record is trusted installation configuration: it states that this exact role and slot takes
/// this exact approved Platform asset as its root-layer default. It is never inferred from a
/// filename, a count, a sort order, a route, a role name or an asset's presence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformGraphicBaselineRecordV1 {
    key: PropertyKey,
    slot: GraphicSlotIdV1,
    reference: AssetReferenceV1,
    asset_sha256: String,
}

impl PlatformGraphicBaselineRecordV1 {
    /// Builds one record without yet binding it to a verified catalogue.
    ///
    /// # Errors
    ///
    /// Returns [`GraphicCascadeRefusalKind::BaselineRoleNotGraphic`] for a non-graphic key,
    /// [`GraphicCascadeRefusalKind::BaselineNamespaceAlias`] for a non-Platform reference,
    /// [`GraphicCascadeRefusalKind::SlotNotPermittedForRole`] for a non-`primary` slot on a
    /// singular role, and [`GraphicCascadeRefusalKind::BaselineAssetUnapproved`] for a malformed
    /// digest.
    pub fn new(
        key: PropertyKey,
        slot: &str,
        reference: AssetReferenceV1,
        asset_sha256: &str,
    ) -> Result<Self, GraphicCascadeRefusalV1> {
        if !key.is_graphic() {
            return Err(GraphicCascadeRefusalV1::of(
                GraphicCascadeRefusalKind::BaselineRoleNotGraphic,
            )
            .with_key(key));
        }
        if reference.namespace() != AssetNamespace::Platform {
            return Err(GraphicCascadeRefusalV1::of(
                GraphicCascadeRefusalKind::BaselineNamespaceAlias,
            )
            .with_key(key));
        }
        let slot = GraphicSlotIdV1::parse(slot).map_err(|refusal| refusal.with_key(key))?;
        if key.max_effective() == 1 && !slot.is_primary() {
            return Err(GraphicCascadeRefusalV1::of(
                GraphicCascadeRefusalKind::SlotNotPermittedForRole,
            )
            .with_key(key));
        }
        if !is_lowercase_hex_digest(asset_sha256) {
            return Err(GraphicCascadeRefusalV1::of(
                GraphicCascadeRefusalKind::BaselineAssetUnapproved,
            )
            .with_key(key));
        }
        Ok(Self {
            key,
            slot,
            reference,
            asset_sha256: asset_sha256.to_owned(),
        })
    }

    /// The registered graphic role.
    #[must_use]
    pub const fn key(&self) -> PropertyKey {
        self.key
    }

    /// The registered slot.
    #[must_use]
    pub const fn slot(&self) -> &GraphicSlotIdV1 {
        &self.slot
    }

    /// The exact approved Platform reference.
    #[must_use]
    pub const fn reference(&self) -> &AssetReferenceV1 {
        &self.reference
    }

    /// The declared digest of the referenced asset's own bytes.
    #[must_use]
    pub fn asset_sha256(&self) -> &str {
        &self.asset_sha256
    }
}

/// The closed registration table of Platform graphic role/slot defaults.
///
/// The table is **separate** from the immutable `1.0.0` catalogue document: it adds no catalogue
/// field, changes no catalogue entry and cannot be derived from the installed product tree. It may
/// legitimately be **empty**, in which case no product default exists for any role and the cascade
/// carries only authored selections.
///
/// A record becomes eligible only when the verified binding proves the same library version, the
/// same catalogue digest and the exact approved identifier with the declared digest and a media
/// class the role permits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformGraphicBaselineV1 {
    binding: ApprovedPlatformGraphicBindingV1,
    records: Vec<PlatformGraphicBaselineRecordV1>,
}

impl PlatformGraphicBaselineV1 {
    /// Binds a registration table to the verified approved catalogue.
    ///
    /// # Errors
    ///
    /// Returns [`GraphicCascadeRefusalKind::BaselineTableOversize`] beyond
    /// [`MAX_PLATFORM_BASELINE_RECORDS_V1`], [`GraphicCascadeRefusalKind::DuplicateBaselineRecord`]
    /// for a repeated role/slot pair, [`GraphicCascadeRefusalKind::SlotCapacityExceeded`] for more
    /// slots than the role permits, [`GraphicCascadeRefusalKind::BaselineAssetUnapproved`] for an
    /// unapproved identifier, [`GraphicCascadeRefusalKind::BaselineDigestMismatch`] for a digest
    /// that is not the catalogue's own, and
    /// [`GraphicCascadeRefusalKind::BaselineRoleFormatMismatch`] for a media class the role does
    /// not permit.
    pub fn new(
        binding: ApprovedPlatformGraphicBindingV1,
        records: Vec<PlatformGraphicBaselineRecordV1>,
    ) -> Result<Self, GraphicCascadeRefusalV1> {
        if records.len() > MAX_PLATFORM_BASELINE_RECORDS_V1 {
            return Err(GraphicCascadeRefusalV1::of(
                GraphicCascadeRefusalKind::BaselineTableOversize,
            ));
        }
        for (position, record) in records.iter().enumerate() {
            if records[..position]
                .iter()
                .any(|seen| seen.key == record.key && seen.slot == record.slot)
            {
                return Err(GraphicCascadeRefusalV1::of(
                    GraphicCascadeRefusalKind::DuplicateBaselineRecord,
                )
                .with_key(record.key));
            }
            let permitted = if record.key.max_effective() == 1 {
                1
            } else {
                record.key.max_effective()
            };
            if records[..=position]
                .iter()
                .filter(|seen| seen.key == record.key)
                .count()
                > permitted
            {
                return Err(GraphicCascadeRefusalV1::of(
                    GraphicCascadeRefusalKind::SlotCapacityExceeded,
                )
                .with_key(record.key));
            }
            let asset = binding.exact(record.reference.reference()).ok_or_else(|| {
                GraphicCascadeRefusalV1::of(GraphicCascadeRefusalKind::BaselineAssetUnapproved)
                    .with_key(record.key)
            })?;
            if asset.sha256() != record.asset_sha256 {
                return Err(GraphicCascadeRefusalV1::of(
                    GraphicCascadeRefusalKind::BaselineDigestMismatch,
                )
                .with_key(record.key));
            }
            if !asset.media_class().permitted_for(record.key) {
                return Err(GraphicCascadeRefusalV1::of(
                    GraphicCascadeRefusalKind::BaselineRoleFormatMismatch,
                )
                .with_key(record.key));
            }
        }
        Ok(Self { binding, records })
    }

    /// An empty table bound to the verified catalogue: no product default exists.
    #[must_use]
    pub fn empty(binding: ApprovedPlatformGraphicBindingV1) -> Self {
        Self {
            binding,
            records: Vec::new(),
        }
    }

    /// The verified binding this table was validated against.
    #[must_use]
    pub const fn binding(&self) -> &ApprovedPlatformGraphicBindingV1 {
        &self.binding
    }

    /// Every registered record, in authored registration order.
    #[must_use]
    pub fn records(&self) -> &[PlatformGraphicBaselineRecordV1] {
        &self.records
    }

    /// Whether no default is registered at all.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// The record registered for one exact role/slot pair.
    #[must_use]
    pub fn registration(
        &self,
        key: PropertyKey,
        slot: &GraphicSlotIdV1,
    ) -> Option<&PlatformGraphicBaselineRecordV1> {
        self.records
            .iter()
            .find(|record| record.key == key && record.slot == *slot)
    }
}

/// The two closed slot operations an author may state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GraphicSlotOperationV1 {
    /// Replace whatever occupies the slot, including a root-layer default.
    Supersede,
    /// Fill the slot only when it is empty; the legacy additive conflict still refuses.
    Append,
}

impl GraphicSlotOperationV1 {
    /// The stable operation token.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Supersede => "supersede",
            Self::Append => "append",
        }
    }
}

/// One explicitly authored graphic slot selection.
///
/// A selection is *slot-qualified by construction*, so nothing ever has to guess which slot a
/// value belongs to. `Supersede` is the only operation that displaces an inherited or root-layer
/// value; `Append` keeps the accepted additive meaning and therefore refuses when the slot is
/// already occupied.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphicSlotSelectionV1 {
    key: PropertyKey,
    slot: GraphicSlotIdV1,
    operation: GraphicSlotOperationV1,
    reference: AssetReferenceV1,
    exposure: Exposure,
}

impl GraphicSlotSelectionV1 {
    /// Builds one validated selection.
    ///
    /// # Errors
    ///
    /// Returns [`GraphicCascadeRefusalKind::BaselineRoleNotGraphic`] for a non-graphic key,
    /// [`GraphicCascadeRefusalKind::InvalidSlotIdentifier`] for a bad slot and
    /// [`GraphicCascadeRefusalKind::SlotNotPermittedForRole`] for a non-`primary` slot on a
    /// singular role.
    pub fn new(
        key: PropertyKey,
        slot: &str,
        operation: GraphicSlotOperationV1,
        reference: AssetReferenceV1,
        exposure: Exposure,
    ) -> Result<Self, GraphicCascadeRefusalV1> {
        if !key.is_graphic() {
            return Err(GraphicCascadeRefusalV1::of(
                GraphicCascadeRefusalKind::BaselineRoleNotGraphic,
            )
            .with_key(key));
        }
        let slot = GraphicSlotIdV1::parse(slot).map_err(|refusal| refusal.with_key(key))?;
        if key.max_effective() == 1 && !slot.is_primary() {
            return Err(GraphicCascadeRefusalV1::of(
                GraphicCascadeRefusalKind::SlotNotPermittedForRole,
            )
            .with_key(key));
        }
        Ok(Self {
            key,
            slot,
            operation,
            reference,
            exposure,
        })
    }

    /// An explicitly superseding selection for one named role/slot.
    ///
    /// # Errors
    ///
    /// As [`Self::new`].
    pub fn supersede(
        key: PropertyKey,
        slot: &str,
        reference: AssetReferenceV1,
        exposure: Exposure,
    ) -> Result<Self, GraphicCascadeRefusalV1> {
        Self::new(
            key,
            slot,
            GraphicSlotOperationV1::Supersede,
            reference,
            exposure,
        )
    }

    /// An additive selection for one named role/slot.
    ///
    /// # Errors
    ///
    /// As [`Self::new`].
    pub fn append(
        key: PropertyKey,
        slot: &str,
        reference: AssetReferenceV1,
        exposure: Exposure,
    ) -> Result<Self, GraphicCascadeRefusalV1> {
        Self::new(
            key,
            slot,
            GraphicSlotOperationV1::Append,
            reference,
            exposure,
        )
    }

    /// The selected role.
    #[must_use]
    pub const fn key(&self) -> PropertyKey {
        self.key
    }

    /// The named slot.
    #[must_use]
    pub const fn slot(&self) -> &GraphicSlotIdV1 {
        &self.slot
    }

    /// The authored operation.
    #[must_use]
    pub const fn operation(&self) -> GraphicSlotOperationV1 {
        self.operation
    }

    /// The selected authored reference.
    #[must_use]
    pub const fn reference(&self) -> &AssetReferenceV1 {
        &self.reference
    }

    /// The authored exposure of this selection.
    #[must_use]
    pub const fn exposure(&self) -> Exposure {
        self.exposure
    }
}

/// One owner's validated graphic slot selections.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnerGraphicSelectionsV1 {
    owner: QualifiedOwnerV1,
    selections: Vec<GraphicSlotSelectionV1>,
}

impl OwnerGraphicSelectionsV1 {
    /// Validates one owner's selection block.
    ///
    /// # Errors
    ///
    /// Returns [`GraphicCascadeRefusalKind::DuplicateGraphicSelection`] for a repeated role/slot
    /// pair, [`GraphicCascadeRefusalKind::SlotCapacityExceeded`] past the role's slot capacity and
    /// [`GraphicCascadeRefusalKind::BaselineTableOversize`] beyond
    /// [`MAX_OWNER_GRAPHIC_SELECTIONS_V1`].
    pub fn new(
        owner: QualifiedOwnerV1,
        selections: Vec<GraphicSlotSelectionV1>,
    ) -> Result<Self, GraphicCascadeRefusalV1> {
        if selections.len() > MAX_OWNER_GRAPHIC_SELECTIONS_V1 {
            return Err(GraphicCascadeRefusalV1::of(
                GraphicCascadeRefusalKind::BaselineTableOversize,
            )
            .with_owner(&owner));
        }
        for (position, selection) in selections.iter().enumerate() {
            if selections[..position]
                .iter()
                .any(|seen| seen.key == selection.key && seen.slot == selection.slot)
            {
                return Err(GraphicCascadeRefusalV1::of(
                    GraphicCascadeRefusalKind::DuplicateGraphicSelection,
                )
                .with_key(selection.key)
                .with_owner(&owner));
            }
            let permitted = if selection.key.max_effective() == 1 {
                1
            } else {
                selection.key.max_effective()
            };
            if selections[..=position]
                .iter()
                .filter(|seen| seen.key == selection.key)
                .count()
                > permitted
            {
                return Err(GraphicCascadeRefusalV1::of(
                    GraphicCascadeRefusalKind::SlotCapacityExceeded,
                )
                .with_key(selection.key)
                .with_owner(&owner));
            }
        }
        Ok(Self { owner, selections })
    }

    /// The owner these selections belong to.
    #[must_use]
    pub const fn owner(&self) -> &QualifiedOwnerV1 {
        &self.owner
    }

    /// The selections in authored order.
    #[must_use]
    pub fn selections(&self) -> &[GraphicSlotSelectionV1] {
        &self.selections
    }

    /// The selections authored for one role, in authored order.
    fn for_key(&self, key: PropertyKey) -> impl Iterator<Item = &GraphicSlotSelectionV1> {
        self.selections
            .iter()
            .filter(move |selection| selection.key == key)
    }
}

/// The closed kind of one effective graphic source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GraphicSourceKindV1 {
    /// A root-layer registered default from the approved Platform library.
    PlatformRootDefault,
    /// An authored selection at one EFS owner.
    AuthoredSelection,
}

impl GraphicSourceKindV1 {
    /// The stable source-kind token.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PlatformRootDefault => "platform-root-default",
            Self::AuthoredSelection => "authored-selection",
        }
    }
}

/// The catalogue binding of one root-layer default.
///
/// This is internal provenance only: it records which approved library version, catalogue digest,
/// identifier and asset digest supplied the default. It is never publicly projected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphicPlatformBindingV1 {
    library_version: String,
    catalogue_sha256: String,
    asset_id: String,
    asset_sha256: String,
}

impl GraphicPlatformBindingV1 {
    /// The approved library version the default came from.
    #[must_use]
    pub fn library_version(&self) -> &str {
        &self.library_version
    }

    /// The verified catalogue digest.
    #[must_use]
    pub fn catalogue_sha256(&self) -> &str {
        &self.catalogue_sha256
    }

    /// The exact catalogue identifier.
    #[must_use]
    pub fn asset_id(&self) -> &str {
        &self.asset_id
    }

    /// The asset's own declared digest.
    #[must_use]
    pub fn asset_sha256(&self) -> &str {
        &self.asset_sha256
    }
}

/// One effective graphic source with its complete internal provenance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphicSourceV1 {
    kind: GraphicSourceKindV1,
    namespace: AssetNamespace,
    reference: String,
    exposure: Exposure,
    binding: Option<GraphicPlatformBindingV1>,
}

impl GraphicSourceV1 {
    /// The closed source kind.
    #[must_use]
    pub const fn kind(&self) -> GraphicSourceKindV1 {
        self.kind
    }

    /// The namespace the winning source lives in.
    #[must_use]
    pub const fn namespace(&self) -> AssetNamespace {
        self.namespace
    }

    /// The winning portable relative reference.
    #[must_use]
    pub fn reference(&self) -> &str {
        &self.reference
    }

    /// The winning source's authored exposure.
    #[must_use]
    pub const fn exposure(&self) -> Exposure {
        self.exposure
    }

    /// The Platform catalogue binding, for a root-layer default only.
    #[must_use]
    pub const fn binding(&self) -> Option<&GraphicPlatformBindingV1> {
        self.binding.as_ref()
    }
}

/// The closed operation one lineage step records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GraphicLineageOperationV1 {
    /// The root layer supplied the slot's initial default.
    Selected,
    /// An authored selection explicitly superseded the current value.
    Superseded,
    /// An authored selection filled an empty slot additively.
    Appended,
    /// This owner's `suppress` discarded the inherited and root-layer values for the key.
    Suppressed,
}

impl GraphicLineageOperationV1 {
    /// The stable operation token.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Selected => "selected",
            Self::Superseded => "superseded",
            Self::Appended => "appended",
            Self::Suppressed => "suppressed",
        }
    }
}

/// One ordered step of a slot's supersession lineage.
///
/// `owner` is `None` for the Platform root layer, which is a library layer and **not** an EFS node;
/// a later public projection never receives this record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphicLineageStepV1 {
    owner: Option<QualifiedOwnerV1>,
    operation: GraphicLineageOperationV1,
}

impl GraphicLineageStepV1 {
    /// The owner that authored the step, or `None` for the root layer.
    #[must_use]
    pub const fn owner(&self) -> Option<&QualifiedOwnerV1> {
        self.owner.as_ref()
    }

    /// The recorded operation.
    #[must_use]
    pub const fn operation(&self) -> GraphicLineageOperationV1 {
        self.operation
    }

    /// Whether this step is the Platform root layer rather than an EFS owner.
    #[must_use]
    pub const fn is_platform_root(&self) -> bool {
        self.owner.is_none()
    }
}

/// One effective graphic slot with its winning source and lineage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphicSlotResolutionV1 {
    slot: GraphicSlotIdV1,
    source: GraphicSourceV1,
    lineage: Vec<GraphicLineageStepV1>,
}

impl GraphicSlotResolutionV1 {
    /// The slot this value occupies.
    #[must_use]
    pub const fn slot(&self) -> &GraphicSlotIdV1 {
        &self.slot
    }

    /// The winning source.
    #[must_use]
    pub const fn source(&self) -> &GraphicSourceV1 {
        &self.source
    }

    /// The ordered lineage from the root layer to the winner.
    #[must_use]
    pub fn lineage(&self) -> &[GraphicLineageStepV1] {
        &self.lineage
    }

    /// The owner that authored the winning value, or `None` for a root-layer default.
    #[must_use]
    pub fn winning_owner(&self) -> Option<&QualifiedOwnerV1> {
        self.lineage.last().and_then(|step| step.owner())
    }
}

/// One resolved graphic role.
///
/// Exactly one carrier is populated. In **slot-aware** mode ([`GraphicCascadeInputV1::is_slot_aware`])
/// `slots` carries the effective per-slot values and `legacy` is empty. In **parity** mode the role
/// carries the untouched legacy effective values in `legacy` and never invents a slot identity,
/// which is what makes an adopting project's previous semantics byte-identical.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedGraphicRoleV1 {
    key: PropertyKey,
    slots: Vec<GraphicSlotResolutionV1>,
    legacy: Vec<ProvenanceValueV1>,
    suppressed_at: Option<QualifiedOwnerV1>,
}

impl ResolvedGraphicRoleV1 {
    /// The resolved role.
    #[must_use]
    pub const fn key(&self) -> PropertyKey {
        self.key
    }

    /// The effective slot values, in registration-then-authored order.
    #[must_use]
    pub fn slots(&self) -> &[GraphicSlotResolutionV1] {
        &self.slots
    }

    /// The untouched legacy effective values, in parity mode only.
    #[must_use]
    pub fn legacy(&self) -> &[ProvenanceValueV1] {
        &self.legacy
    }

    /// The nearest owner whose explicit `suppress` cleared this role's inherited values.
    #[must_use]
    pub const fn suppressed_at(&self) -> Option<&QualifiedOwnerV1> {
        self.suppressed_at.as_ref()
    }

    /// Whether this role resolved to nothing at all.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.slots.is_empty() && self.legacy.is_empty()
    }

    /// One effective slot, by exact identifier.
    #[must_use]
    pub fn slot(&self, slot: &str) -> Option<&GraphicSlotResolutionV1> {
        self.slots
            .iter()
            .find(|resolution| resolution.slot.as_str() == slot)
    }
}

/// The two kinds of refusal a graphic cascade can produce.
///
/// A slot-aware candidate still resolves its non-graphic keys through the accepted legacy
/// algorithm, so a legacy refusal must reach the caller **unchanged** rather than being flattened
/// into a generic graphic error. Either variant fails the whole candidate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphicCascadeFailureV1 {
    /// A graphic registration, slot, suppression or resolution failure.
    Graphic(GraphicCascadeRefusalV1),
    /// The unchanged legacy resolution refused a key of this candidate.
    Legacy(PropertyResolutionRefusalV1),
}

impl GraphicCascadeFailureV1 {
    /// The graphic refusal, when this failure is one.
    #[must_use]
    pub const fn graphic(&self) -> Option<&GraphicCascadeRefusalV1> {
        match self {
            Self::Graphic(refusal) => Some(refusal),
            Self::Legacy(_) => None,
        }
    }

    /// The graphic refusal kind, when this failure is one.
    #[must_use]
    pub fn graphic_kind(&self) -> Option<GraphicCascadeRefusalKind> {
        self.graphic().map(GraphicCascadeRefusalV1::kind)
    }

    /// The stable, safe detail token of either variant.
    #[must_use]
    pub const fn detail(&self) -> &'static str {
        match self {
            Self::Graphic(refusal) => refusal.detail(),
            Self::Legacy(refusal) => refusal.detail(),
        }
    }
}

impl From<GraphicCascadeRefusalV1> for GraphicCascadeFailureV1 {
    fn from(refusal: GraphicCascadeRefusalV1) -> Self {
        Self::Graphic(refusal)
    }
}

impl From<PropertyResolutionRefusalV1> for GraphicCascadeFailureV1 {
    fn from(refusal: PropertyResolutionRefusalV1) -> Self {
        Self::Legacy(refusal)
    }
}

impl fmt::Display for GraphicCascadeFailureV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "graphic-cascade-failed: {}", self.detail())
    }
}

impl std::error::Error for GraphicCascadeFailureV1 {}

/// One owner's resolved graphic roles and unchanged non-graphic values.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedGraphicOwnerV1 {
    owner: QualifiedOwnerV1,
    roles: Vec<ResolvedGraphicRoleV1>,
    non_graphic: ResolvedPropertiesV1,
}

impl ResolvedGraphicOwnerV1 {
    /// The resolved owner.
    #[must_use]
    pub const fn owner(&self) -> &QualifiedOwnerV1 {
        &self.owner
    }

    /// Every graphic role, in registry order.
    #[must_use]
    pub fn roles(&self) -> &[ResolvedGraphicRoleV1] {
        &self.roles
    }

    /// One graphic role's resolution.
    #[must_use]
    pub fn role(&self, key: PropertyKey) -> Option<&ResolvedGraphicRoleV1> {
        self.roles.iter().find(|role| role.key == key)
    }

    /// The unchanged effective non-graphic values (`location`, `seo.*`, `contact.email`).
    ///
    /// These are produced by the legacy algorithm itself, so a graphic change can never alter
    /// location, SEO or contact semantics.
    #[must_use]
    pub const fn non_graphic(&self) -> &ResolvedPropertiesV1 {
        &self.non_graphic
    }
}

/// The whole-candidate resolved graphic cascade.
///
/// The value exists only when **every** owner of the one Entity resolved without a refusal, so a
/// single malicious or invalid late declaration leaves the caller with no partial result.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedGraphicCascadeGraphV1 {
    owners: Vec<ResolvedGraphicOwnerV1>,
}

impl ResolvedGraphicCascadeGraphV1 {
    /// Every resolved owner, parent first.
    #[must_use]
    pub fn owners(&self) -> &[ResolvedGraphicOwnerV1] {
        &self.owners
    }

    /// One owner's record.
    #[must_use]
    pub fn for_owner(&self, owner: &QualifiedOwnerV1) -> Option<&ResolvedGraphicOwnerV1> {
        self.owners.iter().find(|record| record.owner == *owner)
    }

    /// How many owners resolved.
    #[must_use]
    pub fn len(&self) -> usize {
        self.owners.len()
    }

    /// Whether no owner resolved at all.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.owners.is_empty()
    }
}

/// One publicly projectable graphic slot value.
///
/// It carries the role, the slot and the portable reference only: never a lineage, a Platform
/// catalogue binding, an authored owner, a filesystem path, source bytes or an internal record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicGraphicSlotV1 {
    key: PropertyKey,
    slot: String,
    reference: String,
}

impl PublicGraphicSlotV1 {
    /// The graphic role.
    #[must_use]
    pub const fn key(&self) -> PropertyKey {
        self.key
    }

    /// The named slot.
    #[must_use]
    pub fn slot(&self) -> &str {
        &self.slot
    }

    /// The portable relative asset reference.
    #[must_use]
    pub fn reference(&self) -> &str {
        &self.reference
    }
}

/// The public projection of one owner's slot-aware graphic roles.
///
/// This is a **filtered view, not delivery**: it creates no route, URL, derived file or release, it
/// cannot resurrect a suppressed or superseded value, and it never exposes internal provenance. A
/// role whose winner is `internal` contributes nothing, so a private closer value can never be
/// replaced by a public root default at projection time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicGraphicProjectionV1 {
    owner: QualifiedOwnerV1,
    slots: Vec<PublicGraphicSlotV1>,
}

impl PublicGraphicProjectionV1 {
    /// The projected owner.
    #[must_use]
    pub const fn owner(&self) -> &QualifiedOwnerV1 {
        &self.owner
    }

    /// Every public slot value, in registry then slot order.
    #[must_use]
    pub fn slots(&self) -> &[PublicGraphicSlotV1] {
        &self.slots
    }

    /// One role's public slot values.
    #[must_use]
    pub fn slots_of(&self, key: PropertyKey) -> Vec<&PublicGraphicSlotV1> {
        self.slots.iter().filter(|slot| slot.key == key).collect()
    }
}

/// Projects the public subset of one owner's slot-aware graphic resolution.
///
/// Parity-mode records have no slots, so this returns an empty projection for them; a caller in
/// parity mode keeps using [`project_public_properties_v1`] and obtains byte-identical behaviour.
#[must_use]
pub fn project_public_graphic_slots_v1(
    resolved: &ResolvedGraphicOwnerV1,
) -> PublicGraphicProjectionV1 {
    let mut slots = Vec::new();
    for role in &resolved.roles {
        for resolution in &role.slots {
            if !resolution.source.exposure.is_public() {
                continue;
            }
            slots.push(PublicGraphicSlotV1 {
                key: role.key,
                slot: resolution.slot.as_str().to_owned(),
                reference: resolution.source.reference.clone(),
            });
        }
    }
    PublicGraphicProjectionV1 {
        owner: resolved.owner.clone(),
        slots,
    }
}

/// One effective slot while the walk is in progress.
#[derive(Debug, Clone, PartialEq, Eq)]
struct SlotStateV1 {
    slot: GraphicSlotIdV1,
    source: GraphicSourceV1,
    lineage: Vec<GraphicLineageStepV1>,
}

impl SlotStateV1 {
    /// Copies one finished slot back into a mutable working state.
    fn from_resolution(resolution: &GraphicSlotResolutionV1) -> Self {
        Self {
            slot: resolution.slot.clone(),
            source: resolution.source.clone(),
            lineage: resolution.lineage.clone(),
        }
    }

    /// Finishes this working state.
    fn into_resolution(self) -> GraphicSlotResolutionV1 {
        GraphicSlotResolutionV1 {
            slot: self.slot,
            source: self.source,
            lineage: self.lineage,
        }
    }
}

/// The opt-in graphic-cascade candidate: validated declarations plus registration and selections.
///
/// Constructing one validates membership and the registration/selections model. Resolving it
/// validates the **whole** candidate, so a caller can never obtain a partial success.
#[derive(Debug, Clone, PartialEq)]
pub struct GraphicCascadeInputV1 {
    validated: ValidatedPropertyInputV1,
    baseline: PlatformGraphicBaselineV1,
    selections: Vec<OwnerGraphicSelectionsV1>,
}

impl GraphicCascadeInputV1 {
    /// Builds one candidate over an already validated declaration set.
    ///
    /// # Errors
    ///
    /// Returns [`GraphicCascadeRefusalKind::SelectionOwnerUnknown`] for a selection block attached
    /// to an owner outside the graph and [`GraphicCascadeRefusalKind::DuplicateGraphicSelection`]
    /// for two blocks for the same owner.
    pub fn new(
        validated: ValidatedPropertyInputV1,
        baseline: PlatformGraphicBaselineV1,
        selections: Vec<OwnerGraphicSelectionsV1>,
    ) -> Result<Self, GraphicCascadeRefusalV1> {
        for (position, block) in selections.iter().enumerate() {
            if !validated.owners().contains(block.owner()) {
                return Err(GraphicCascadeRefusalV1::of(
                    GraphicCascadeRefusalKind::SelectionOwnerUnknown,
                )
                .with_owner(block.owner()));
            }
            if selections[..position]
                .iter()
                .any(|seen| seen.owner() == block.owner())
            {
                return Err(GraphicCascadeRefusalV1::of(
                    GraphicCascadeRefusalKind::DuplicateGraphicSelection,
                )
                .with_owner(block.owner()));
            }
        }
        Ok(Self {
            validated,
            baseline,
            selections,
        })
    }

    /// The validated declarations this candidate resolves over.
    #[must_use]
    pub const fn validated(&self) -> &ValidatedPropertyInputV1 {
        &self.validated
    }

    /// The closed registration table.
    #[must_use]
    pub const fn baseline(&self) -> &PlatformGraphicBaselineV1 {
        &self.baseline
    }

    /// Every authored selection block, in authored order.
    #[must_use]
    pub fn selections(&self) -> &[OwnerGraphicSelectionsV1] {
        &self.selections
    }

    /// Whether this candidate opts into slot-aware resolution.
    ///
    /// A candidate that registers no default and authors no selection is in **parity** mode: it
    /// reproduces the accepted legacy graphic semantics exactly, which is why installing
    /// `platform/assets/` can never change an existing project on its own.
    #[must_use]
    pub fn is_slot_aware(&self) -> bool {
        !self.baseline.is_empty() || !self.selections.is_empty()
    }
}

impl GraphicCascadeInputV1 {
    /// Resolves the whole candidate and returns one owner's record.
    ///
    /// The whole graph is resolved even when one owner is requested, so a refusal anywhere in the
    /// candidate fails this call. There is deliberately no partial result.
    ///
    /// # Errors
    ///
    /// Returns [`GraphicCascadeFailureV1`] for an unknown target, a graphic refusal, or an
    /// unchanged legacy refusal of any key.
    pub fn resolve_graphic_cascade_v1(
        &self,
        target: &QualifiedOwnerV1,
    ) -> Result<ResolvedGraphicOwnerV1, GraphicCascadeFailureV1> {
        if !self.validated.owners().contains(target) {
            return Err(
                GraphicCascadeRefusalV1::of(GraphicCascadeRefusalKind::UnknownOwner).into(),
            );
        }
        let graph = self.resolve_graphic_cascade_all_v1()?;
        graph.for_owner(target).cloned().ok_or_else(|| {
            GraphicCascadeRefusalV1::of(GraphicCascadeRefusalKind::InconsistentGraph).into()
        })
    }

    /// Resolves and validates **every** owner of the one Entity, parent first.
    ///
    /// # Errors
    ///
    /// Returns [`GraphicCascadeFailureV1`] for any failure anywhere in the candidate: an invalid
    /// registration, an unauthorised or unqualified selection, a filled slot, an exceeded
    /// capacity, or the refusal of an unchanged legacy key.
    pub fn resolve_graphic_cascade_all_v1(
        &self,
    ) -> Result<ResolvedGraphicCascadeGraphV1, GraphicCascadeFailureV1> {
        if !self.is_slot_aware() {
            return self.resolve_parity_graph();
        }
        let positions: BTreeMap<&QualifiedOwnerV1, usize> = self
            .validated
            .owners()
            .iter()
            .enumerate()
            .map(|(position, owner)| (owner, position))
            .collect();
        let mut resolved: Vec<ResolvedGraphicOwnerV1> =
            Vec::with_capacity(self.validated.owners().len());
        for owner in self.validated.owners() {
            let parent = if owner.level_index() == 0 {
                None
            } else {
                let mut chain = owner.chain();
                chain.pop();
                let parent_owner = chain.pop().ok_or_else(|| {
                    GraphicCascadeRefusalV1::of(GraphicCascadeRefusalKind::InconsistentGraph)
                })?;
                let position = positions.get(&parent_owner).copied().ok_or_else(|| {
                    GraphicCascadeRefusalV1::of(GraphicCascadeRefusalKind::InconsistentGraph)
                })?;
                Some(resolved.get(position).ok_or_else(|| {
                    GraphicCascadeRefusalV1::of(GraphicCascadeRefusalKind::InconsistentGraph)
                })?)
            };
            resolved.push(self.resolve_slot_owner(owner, parent)?);
        }
        Ok(ResolvedGraphicCascadeGraphV1 { owners: resolved })
    }

    /// Resolves the candidate in parity mode: exactly the accepted legacy semantics.
    ///
    /// The legacy resolver does all the work, so parity is achieved by construction rather than by
    /// a parallel re-implementation that could drift.
    fn resolve_parity_graph(
        &self,
    ) -> Result<ResolvedGraphicCascadeGraphV1, GraphicCascadeFailureV1> {
        let legacy = self.validated.resolve_all()?;
        let mut owners = Vec::with_capacity(legacy.len());
        for record in legacy.owners() {
            let roles = PropertyKey::ALL
                .into_iter()
                .filter(|key| key.is_graphic())
                .map(|key| {
                    let property = record.property(key);
                    ResolvedGraphicRoleV1 {
                        key,
                        slots: Vec::new(),
                        legacy: property.map_or_else(Vec::new, |held| held.values().to_vec()),
                        suppressed_at: property.and_then(|held| {
                            held.resolution_events()
                                .iter()
                                .rev()
                                .find(|event| event.operation() == ResolutionOperation::Suppressed)
                                .map(|event| event.owner().clone())
                        }),
                    }
                })
                .collect();
            owners.push(ResolvedGraphicOwnerV1 {
                owner: record.owner().clone(),
                roles,
                non_graphic: non_graphic_record(record),
            });
        }
        Ok(ResolvedGraphicCascadeGraphV1 { owners })
    }
}

/// The four non-graphic keys of the accepted registry, in registry order.
const NON_GRAPHIC_KEYS_V1: [PropertyKey; 4] = [
    PropertyKey::Location,
    PropertyKey::SeoTitle,
    PropertyKey::SeoDescription,
    PropertyKey::ContactEmail,
];

/// Rebuilds one owner's non-graphic record from its complete legacy resolution.
fn non_graphic_record(record: &ResolvedPropertiesV1) -> ResolvedPropertiesV1 {
    let properties = NON_GRAPHIC_KEYS_V1
        .into_iter()
        .filter_map(|key| record.property(key).cloned())
        .collect();
    ResolvedPropertiesV1 {
        owner: record.owner().clone(),
        properties,
    }
}

impl GraphicCascadeInputV1 {
    /// Resolves one owner in slot-aware mode.
    fn resolve_slot_owner(
        &self,
        owner: &QualifiedOwnerV1,
        parent: Option<&ResolvedGraphicOwnerV1>,
    ) -> Result<ResolvedGraphicOwnerV1, GraphicCascadeFailureV1> {
        let declaration = self.validated.declaration_for(owner);
        let block = self
            .selections
            .iter()
            .find(|candidate| candidate.owner() == owner);
        let mut roles = Vec::with_capacity(PropertyKey::ALL.len());
        for key in PropertyKey::ALL.into_iter().filter(|key| key.is_graphic()) {
            let inherited = parent.and_then(|record| record.role(key));
            let mut slots: Vec<SlotStateV1> = inherited.map_or_else(Vec::new, |role| {
                role.slots
                    .iter()
                    .map(SlotStateV1::from_resolution)
                    .collect()
            });
            let mut suppressed_at = inherited.and_then(|role| role.suppressed_at.clone());

            // The root layer applies once, at the Entity, before any authored declaration.
            if owner.level_index() == 0 {
                for record in self
                    .baseline
                    .records()
                    .iter()
                    .filter(|record| record.key() == key)
                {
                    slots.push(SlotStateV1 {
                        slot: record.slot().clone(),
                        source: self.baseline_source(record)?,
                        lineage: vec![GraphicLineageStepV1 {
                            owner: None,
                            operation: GraphicLineageOperationV1::Selected,
                        }],
                    });
                }
            }

            if let Some(envelope) = declaration.and_then(|declared| declared.declaration(key)) {
                // Suppression is final within its scope: it clears root-layer and inherited
                // values and never re-establishes a default by itself.
                if envelope.inheritance() == Inheritance::Suppress {
                    slots.clear();
                    suppressed_at = Some(owner.clone());
                }
                // A legacy unqualified graphic value cannot name a slot, so slot-aware mode
                // refuses it instead of guessing where it belongs.
                let unqualified = envelope
                    .current_context()
                    .values()
                    .iter()
                    .any(|authored| authored.value().asset_reference().is_some());
                if unqualified {
                    return Err(GraphicCascadeRefusalV1::of(
                        GraphicCascadeRefusalKind::UnqualifiedGraphicEntry,
                    )
                    .with_key(key)
                    .with_owner(owner)
                    .into());
                }
            }

            if let Some(block) = block {
                for selection in block.for_key(key) {
                    self.apply_selection(&mut slots, selection, owner)?;
                }
            }

            roles.push(ResolvedGraphicRoleV1 {
                key,
                slots: slots
                    .into_iter()
                    .map(SlotStateV1::into_resolution)
                    .collect(),
                legacy: Vec::new(),
                suppressed_at,
            });
        }
        let non_graphic = self.resolve_non_graphic(owner, declaration, parent)?;
        Ok(ResolvedGraphicOwnerV1 {
            owner: owner.clone(),
            roles,
            non_graphic,
        })
    }
}

impl GraphicCascadeInputV1 {
    /// Applies one authored slot selection to the working slot set.
    fn apply_selection(
        &self,
        slots: &mut Vec<SlotStateV1>,
        selection: &GraphicSlotSelectionV1,
        owner: &QualifiedOwnerV1,
    ) -> Result<(), GraphicCascadeFailureV1> {
        let capacity = selection.key().max_effective();
        let existing = slots
            .iter()
            .position(|state| state.slot == *selection.slot());
        let refused = |kind: GraphicCascadeRefusalKind| {
            GraphicCascadeFailureV1::Graphic(
                GraphicCascadeRefusalV1::of(kind)
                    .with_key(selection.key())
                    .with_owner(owner),
            )
        };
        match selection.operation() {
            GraphicSlotOperationV1::Supersede => match existing {
                Some(index) => {
                    let state = &mut slots[index];
                    state.source = authored_source(selection);
                    state.lineage.push(GraphicLineageStepV1 {
                        owner: Some(owner.clone()),
                        operation: GraphicLineageOperationV1::Superseded,
                    });
                }
                None => {
                    if slots.len() >= capacity {
                        return Err(refused(GraphicCascadeRefusalKind::SlotCapacityExceeded));
                    }
                    slots.push(SlotStateV1 {
                        slot: selection.slot().clone(),
                        source: authored_source(selection),
                        lineage: vec![GraphicLineageStepV1 {
                            owner: Some(owner.clone()),
                            operation: GraphicLineageOperationV1::Superseded,
                        }],
                    });
                }
            },
            GraphicSlotOperationV1::Append => {
                if existing.is_some() {
                    return Err(refused(GraphicCascadeRefusalKind::SlotAlreadyFilled));
                }
                if slots.len() >= capacity {
                    return Err(refused(GraphicCascadeRefusalKind::SlotCapacityExceeded));
                }
                slots.push(SlotStateV1 {
                    slot: selection.slot().clone(),
                    source: authored_source(selection),
                    lineage: vec![GraphicLineageStepV1 {
                        owner: Some(owner.clone()),
                        operation: GraphicLineageOperationV1::Appended,
                    }],
                });
            }
        }
        Ok(())
    }
}

impl GraphicCascadeInputV1 {
    /// Builds the winning source of one registered root-layer default.
    ///
    /// Every clause is re-proved here, so even a hand-built record cannot smuggle an unapproved
    /// identifier, a different digest or a media class the role does not permit.
    fn baseline_source(
        &self,
        record: &PlatformGraphicBaselineRecordV1,
    ) -> Result<GraphicSourceV1, GraphicCascadeFailureV1> {
        let binding = self.baseline.binding();
        let asset = binding
            .exact(record.reference().reference())
            .ok_or_else(|| {
                GraphicCascadeRefusalV1::of(GraphicCascadeRefusalKind::BaselineAssetUnapproved)
                    .with_key(record.key())
            })?;
        if asset.sha256() != record.asset_sha256() {
            return Err(GraphicCascadeRefusalV1::of(
                GraphicCascadeRefusalKind::BaselineDigestMismatch,
            )
            .with_key(record.key())
            .into());
        }
        if !asset.media_class().permitted_for(record.key()) {
            return Err(GraphicCascadeRefusalV1::of(
                GraphicCascadeRefusalKind::BaselineRoleFormatMismatch,
            )
            .with_key(record.key())
            .into());
        }
        Ok(GraphicSourceV1 {
            kind: GraphicSourceKindV1::PlatformRootDefault,
            namespace: AssetNamespace::Platform,
            reference: record.reference().reference().to_owned(),
            exposure: Exposure::Public,
            binding: Some(GraphicPlatformBindingV1 {
                library_version: binding.library_version().to_owned(),
                catalogue_sha256: binding.catalogue_sha256().to_owned(),
                asset_id: asset.id().to_owned(),
                asset_sha256: asset.sha256().to_owned(),
            }),
        })
    }

    /// Resolves the unchanged non-graphic keys through the accepted legacy algorithm.
    fn resolve_non_graphic(
        &self,
        owner: &QualifiedOwnerV1,
        declaration: Option<&DeclaredOwnerPropertiesV1>,
        parent: Option<&ResolvedGraphicOwnerV1>,
    ) -> Result<ResolvedPropertiesV1, GraphicCascadeFailureV1> {
        let parent_record = parent.map(ResolvedGraphicOwnerV1::non_graphic);
        let mut properties = Vec::with_capacity(NON_GRAPHIC_KEYS_V1.len());
        for key in NON_GRAPHIC_KEYS_V1 {
            let record =
                ValidatedPropertyInputV1::resolve_key_from(parent_record, declaration, key, owner)?;
            properties.push(record);
        }
        Ok(ResolvedPropertiesV1 {
            owner: owner.clone(),
            properties,
        })
    }
}

/// Builds the winning source of one authored slot selection.
fn authored_source(selection: &GraphicSlotSelectionV1) -> GraphicSourceV1 {
    GraphicSourceV1 {
        kind: GraphicSourceKindV1::AuthoredSelection,
        namespace: selection.reference().namespace(),
        reference: selection.reference().reference().to_owned(),
        exposure: selection.exposure(),
        binding: None,
    }
}
