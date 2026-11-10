//! Strict authored-manifest → typed property translation (`R03.A`).
//!
//! This adapter reads only the **already parsed** inert candidate produced by
//! [`crate::parse_manifest_bytes`] and turns each node's `properties` object into the
//! closed, typed declarations the domain registry accepts. It never re-parses JSON, never
//! opens a source file, never scans a content directory and never decides which body is a
//! Page: property ownership is exactly the EFS containment of the one Entity.
//!
//! [`validate_manifest_property_candidate_v1`] is deliberately named as a *candidate*
//! operation. It returns a validated property candidate, not publication authority, not a
//! verified source snapshot, not routing and not a rendered page — a distinction the
//! parser's own inert return type keeps intact, because `parse_manifest_bytes` is neither
//! renamed nor promoted here.
//!
//! # Diagnostic discipline
//!
//! Every refusal is typed and owner/key specific, and its text contains only the refusal
//! category, the structural qualified owner and the registered key name. No authored
//! property content, no internal value, no filesystem path and no secret is ever echoed.

use core::fmt;

use solo_application::{
    PropertyResolutionCandidateV1, PropertyResolutionRequestV1, resolve_declared_properties_v1,
};
use solo_domain::{
    AssetNamespace, AssetReferenceV1, AuthoredPropertyValueV1, CoordinatesV1, CurrentContextV1,
    DeclaredOwnerPropertiesV1, EfsOwnershipGraph, EntityIdV1, Exposure, FactIdV1, Inheritance,
    LOCATION_FIELD_NAMES, LocaleKeyV1, LocationExtensionRequestV1, LocationFactV1,
    LocationFieldsV1, LocationKind, MAX_SEO_DESCRIPTION_SCALARS, MAX_SEO_TITLE_SCALARS, MailboxV1,
    PROPERTY_REGISTRY_VERSION, PlainTextV1, PropertyDeclarationRefusalV1, PropertyDeclarationV1,
    PropertyKey, PropertyRefusalKind, PropertyResolutionRefusalKind, PropertyValueV1,
    PublicPropertyProjectionV1, QualifiedOwnerV1, ResolvedPropertiesV1,
};

use crate::publication_manifest::{DeclaredNode, InertPropertiesV1, ParsedManifestCandidateV1};

/// Why a manifest property candidate was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManifestPropertyRefusalKind {
    /// The authored property name is not one of the nine registered keys.
    UnknownProperty,
    /// The declared `propertyRegistryVersion` is not the accepted `1.0.0`.
    UnsupportedRegistryVersion,
    /// One closed declaration envelope or entry was refused by the typed registry.
    DeclarationRefused(PropertyRefusalKind),
    /// The typed resolution refused the requested candidate.
    ResolutionRefused(PropertyResolutionRefusalKind),
}

impl ManifestPropertyRefusalKind {
    /// Stable, safe diagnostic token for this refusal reason.
    #[must_use]
    pub const fn detail(self) -> &'static str {
        match self {
            Self::UnknownProperty => "unknown-property",
            Self::UnsupportedRegistryVersion => "unsupported-registry-version",
            Self::DeclarationRefused(kind) => kind.detail(),
            Self::ResolutionRefused(kind) => kind.detail(),
        }
    }
}

/// One refused manifest property candidate.
///
/// The refusal names the qualified structural owner and the registered key involved. It
/// never serialises the authored property value that was refused.
#[derive(Debug, Clone, PartialEq)]
pub struct ManifestPropertyRefusalV1 {
    kind: ManifestPropertyRefusalKind,
    owner: Option<Box<QualifiedOwnerV1>>,
    property: Option<PropertyKey>,
}

impl ManifestPropertyRefusalV1 {
    fn new(
        kind: ManifestPropertyRefusalKind,
        owner: Option<QualifiedOwnerV1>,
        property: Option<PropertyKey>,
    ) -> Self {
        Self {
            kind,
            owner: owner.map(Box::new),
            property,
        }
    }

    /// The refusal category.
    #[must_use]
    pub const fn kind(&self) -> ManifestPropertyRefusalKind {
        self.kind
    }

    /// The qualified structural owner the refusal applies to, when known.
    #[must_use]
    pub fn owner(&self) -> Option<&QualifiedOwnerV1> {
        self.owner.as_deref()
    }

    /// The registered key the refusal applies to, when known.
    #[must_use]
    pub const fn property(&self) -> Option<PropertyKey> {
        self.property
    }

    /// Stable engineering code for this refusal.
    ///
    /// A recognized-but-unimplemented `location` extension keeps its own dedicated code so
    /// it is never confused with a malformed declaration.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self.kind {
            ManifestPropertyRefusalKind::UnknownProperty => "PROPERTY_UNKNOWN_KEY",
            ManifestPropertyRefusalKind::UnsupportedRegistryVersion => {
                "PROPERTY_REGISTRY_VERSION_UNSUPPORTED"
            }
            ManifestPropertyRefusalKind::DeclarationRefused(_) => "PROPERTY_DECLARATION_REFUSED",
            ManifestPropertyRefusalKind::ResolutionRefused(
                PropertyResolutionRefusalKind::LocationExtensionNotImplementedInR03A,
            ) => "LOCATION_EXTENSION_NOT_IMPLEMENTED_IN_R03A",
            ManifestPropertyRefusalKind::ResolutionRefused(_) => "PROPERTY_RESOLUTION_REFUSED",
        }
    }

    /// Stable, safe diagnostic token for the refusal reason.
    #[must_use]
    pub const fn detail(&self) -> &'static str {
        self.kind.detail()
    }
}

impl fmt::Display for ManifestPropertyRefusalV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code(), self.detail())?;
        if let Some(owner) = &self.owner {
            write!(formatter, " at {}", owner.describe())?;
        }
        if let Some(property) = self.property {
            write!(formatter, " [{}]", property.name())?;
        }
        Ok(())
    }
}

impl std::error::Error for ManifestPropertyRefusalV1 {}

/// One validated whole-graph property candidate for a single requested owner.
///
/// The value holds the inbound use-case candidate, the qualified target it resolved and how
/// many owners actually declared properties. It authorizes nothing: it is not a manifest
/// decision, a source snapshot, a route, a rendered page or a release.
#[derive(Debug, Clone)]
pub struct ManifestPropertyCandidateV1 {
    candidate: PropertyResolutionCandidateV1,
    target: QualifiedOwnerV1,
    declared_owner_count: usize,
}

impl ManifestPropertyCandidateV1 {
    /// The inbound use-case candidate.
    #[must_use]
    pub const fn candidate(&self) -> &PropertyResolutionCandidateV1 {
        &self.candidate
    }

    /// The fully qualified owner that was resolved.
    #[must_use]
    pub const fn target(&self) -> &QualifiedOwnerV1 {
        &self.target
    }

    /// The complete internal resolved record of the target.
    #[must_use]
    pub const fn resolved(&self) -> &ResolvedPropertiesV1 {
        self.candidate.resolved()
    }

    /// The validated safe public projection of the target.
    #[must_use]
    pub const fn public(&self) -> &PublicPropertyProjectionV1 {
        self.candidate.public()
    }

    /// How many owners in the candidate declared at least one property.
    #[must_use]
    pub const fn declared_owner_count(&self) -> usize {
        self.declared_owner_count
    }

    /// Whether this candidate authorizes publication. Always `false`.
    #[must_use]
    pub const fn is_publication_authorized(&self) -> bool {
        self.candidate.is_publication_authorized()
    }

    /// Whether any authored declaration carried a recognized `extend` request.
    #[must_use]
    pub fn has_recognized_location_extension(&self) -> bool {
        self.candidate.has_recognized_location_extension()
    }
}

/// Translates and resolves the authored properties of one already parsed manifest.
///
/// Every authored declaration is validated, including those of unlisted representations, so
/// one invalid declaration anywhere in the one-Entity candidate fails the whole operation
/// closed.
///
/// # Errors
///
/// Returns a [`ManifestPropertyRefusalV1`] for an unsupported registry version, an
/// unregistered property name, a refused closed envelope or typed entry, an owner outside
/// the validated graph, an exceeded effective maximum, a repeated background source or
/// location identity, or a recognized `extend` request reached by the resolution.
pub fn validate_manifest_property_candidate_v1(
    manifest: &ParsedManifestCandidateV1,
    graph: &EfsOwnershipGraph,
    target: &QualifiedOwnerV1,
) -> Result<ManifestPropertyCandidateV1, ManifestPropertyRefusalV1> {
    if manifest.property_registry_version().as_str() != PROPERTY_REGISTRY_VERSION {
        return Err(refuse(
            ManifestPropertyRefusalKind::UnsupportedRegistryVersion,
            Some(target.clone()),
            None,
        ));
    }
    let (declared, declared_owner_count) = translate_candidate(manifest)?;
    let request = PropertyResolutionRequestV1::new(graph, declared, target.clone());
    let candidate = resolve_declared_properties_v1(request).map_err(|resolution| {
        refuse(
            ManifestPropertyRefusalKind::ResolutionRefused(resolution.kind()),
            Some(target.clone()),
            None,
        )
    })?;
    Ok(ManifestPropertyCandidateV1 {
        candidate,
        target: target.clone(),
        declared_owner_count,
    })
}

fn refuse(
    kind: ManifestPropertyRefusalKind,
    owner: Option<QualifiedOwnerV1>,
    property: Option<PropertyKey>,
) -> ManifestPropertyRefusalV1 {
    ManifestPropertyRefusalV1::new(kind, owner, property)
}

/// Walks the parsed Entity containment and translates every node's authored properties.
///
/// Ownership comes from the containment itself: the Entity, its Facets, their Sites, their
/// Pages and each Page's declared representations. No directory, filename, route or label is
/// consulted, so a property can never attach to a node the manifest does not contain.
fn translate_candidate(
    manifest: &ParsedManifestCandidateV1,
) -> Result<(Vec<DeclaredOwnerPropertiesV1>, usize), ManifestPropertyRefusalV1> {
    let mut declared: Vec<DeclaredOwnerPropertiesV1> = Vec::new();
    let entity = manifest.entity();
    let entity_id: EntityIdV1 = entity.id().clone();
    push_owner(
        &mut declared,
        QualifiedOwnerV1::entity(entity_id.clone()),
        entity.properties(),
    )?;
    for facet in entity.facets() {
        let facet_id = facet.id().clone();
        let facet_owner = QualifiedOwnerV1::facet(entity_id.clone(), facet_id.clone());
        for site in facet.sites() {
            let site_id = site.id().clone();
            let site_owner =
                QualifiedOwnerV1::site(entity_id.clone(), facet_id.clone(), site_id.clone());
            for page in site.pages() {
                let page_id = page.id().clone();
                let page_owner = QualifiedOwnerV1::page(
                    entity_id.clone(),
                    facet_id.clone(),
                    site_id.clone(),
                    page_id.clone(),
                );
                push_owner(&mut declared, page_owner.clone(), page.properties())?;
                for representation in page.representations() {
                    let locale = LocaleKeyV1::from_already_canonical(representation.locale())
                        .map_err(|_| {
                            refuse(
                                ManifestPropertyRefusalKind::DeclarationRefused(
                                    PropertyRefusalKind::InconsistentOwner,
                                ),
                                Some(page_owner.clone()),
                                None,
                            )
                        })?;
                    let representation_owner = QualifiedOwnerV1::representation(
                        entity_id.clone(),
                        facet_id.clone(),
                        site_id.clone(),
                        page_id.clone(),
                        locale,
                    );
                    push_owner(
                        &mut declared,
                        representation_owner,
                        representation.properties(),
                    )?;
                }
            }
            push_owner(&mut declared, site_owner, site.properties())?;
        }
        push_owner(&mut declared, facet_owner, facet.properties())?;
    }
    let declared_owner_count = declared.len();
    Ok((declared, declared_owner_count))
}

fn push_owner(
    declared: &mut Vec<DeclaredOwnerPropertiesV1>,
    owner: QualifiedOwnerV1,
    properties: Option<&InertPropertiesV1>,
) -> Result<(), ManifestPropertyRefusalV1> {
    if let Some(entry) = translate_owner(owner, properties)? {
        declared.push(entry);
    }
    Ok(())
}

fn refusal(kind: PropertyRefusalKind) -> PropertyDeclarationRefusalV1 {
    PropertyDeclarationRefusalV1::of(kind)
}

/// Translates one node's authored `properties` object, if it declares anything.
///
/// An absent `properties` object and an empty one both mean "no declaration at this node",
/// which is different from declaring a key with `currentContext: "none"`.
fn translate_owner(
    owner: QualifiedOwnerV1,
    properties: Option<&InertPropertiesV1>,
) -> Result<Option<DeclaredOwnerPropertiesV1>, ManifestPropertyRefusalV1> {
    let Some(properties) = properties else {
        return Ok(None);
    };
    if properties.is_empty() {
        return Ok(None);
    }
    let mut entries: Vec<(PropertyKey, PropertyDeclarationV1)> = Vec::new();
    for (name, node) in properties.entries() {
        let key = match PropertyKey::from_name(name) {
            Some(key) => key,
            None => {
                return Err(refuse(
                    ManifestPropertyRefusalKind::UnknownProperty,
                    Some(owner),
                    None,
                ));
            }
        };
        let declaration = translate_declaration(key, node).map_err(|declined| {
            refuse(
                ManifestPropertyRefusalKind::DeclarationRefused(declined.kind()),
                Some(owner.clone()),
                Some(key),
            )
        })?;
        entries.push((key, declaration));
    }
    DeclaredOwnerPropertiesV1::new(owner.clone(), entries)
        .map(Some)
        .map_err(|declined| {
            refuse(
                ManifestPropertyRefusalKind::DeclarationRefused(declined.kind()),
                Some(owner),
                declined.key(),
            )
        })
}

/// Translates the closed `{ inheritance, currentContext }` declaration envelope.
fn translate_declaration(
    key: PropertyKey,
    node: &DeclaredNode,
) -> Result<PropertyDeclarationV1, PropertyDeclarationRefusalV1> {
    let members = match node {
        DeclaredNode::Object(members) => members.as_slice(),
        _ => return Err(refusal(PropertyRefusalKind::InvalidCurrentContext)),
    };
    let mut inheritance_node: Option<&DeclaredNode> = None;
    let mut context_node: Option<&DeclaredNode> = None;
    for (name, value) in members {
        match name.as_str() {
            "inheritance" => {
                if inheritance_node.is_some() {
                    return Err(refusal(PropertyRefusalKind::ExtraEnvelopeField));
                }
                inheritance_node = Some(value);
            }
            "currentContext" => {
                if context_node.is_some() {
                    return Err(refusal(PropertyRefusalKind::ExtraEnvelopeField));
                }
                context_node = Some(value);
            }
            _ => return Err(refusal(PropertyRefusalKind::ExtraEnvelopeField)),
        }
    }
    let inheritance = match inheritance_node {
        Some(DeclaredNode::String(text)) => match Inheritance::from_name(text) {
            Some(mode) => mode,
            None => return Err(refusal(PropertyRefusalKind::InvalidInheritance)),
        },
        Some(_) => return Err(refusal(PropertyRefusalKind::InvalidInheritance)),
        None => return Err(refusal(PropertyRefusalKind::MissingEnvelopeField)),
    };
    let current_context = match context_node {
        Some(node) => translate_context(key, node)?,
        None => return Err(refusal(PropertyRefusalKind::MissingEnvelopeField)),
    };
    Ok(PropertyDeclarationV1::new(inheritance, current_context))
}

/// Translates the closed `currentContext`: the literal `"none"` or a non-empty array.
fn translate_context(
    key: PropertyKey,
    node: &DeclaredNode,
) -> Result<CurrentContextV1, PropertyDeclarationRefusalV1> {
    match node {
        DeclaredNode::String(text) if text == "none" => Ok(CurrentContextV1::None),
        DeclaredNode::String(_) => Err(refusal(PropertyRefusalKind::InvalidCurrentContext)),
        DeclaredNode::Array(items) => {
            if items.is_empty() {
                return Err(refusal(PropertyRefusalKind::EmptyContextArray));
            }
            let mut values = Vec::with_capacity(items.len());
            for item in items {
                values.push(translate_value(key, item)?);
            }
            Ok(CurrentContextV1::Values(values))
        }
        _ => Err(refusal(PropertyRefusalKind::InvalidCurrentContext)),
    }
}

/// Translates one local typed value for a registered key.
fn translate_value(
    key: PropertyKey,
    node: &DeclaredNode,
) -> Result<AuthoredPropertyValueV1, PropertyDeclarationRefusalV1> {
    if key.is_location() {
        let (value, exposure) = translate_location(node)?;
        return Ok(AuthoredPropertyValueV1::new(value, exposure));
    }
    if key.is_graphic() {
        let (reference, exposure) = translate_graphic(node)?;
        return Ok(AuthoredPropertyValueV1::new(
            PropertyValueV1::graphic(reference),
            exposure,
        ));
    }
    match key {
        PropertyKey::SeoTitle => translate_text(node, MAX_SEO_TITLE_SCALARS, "text"),
        PropertyKey::SeoDescription => translate_text(node, MAX_SEO_DESCRIPTION_SCALARS, "text"),
        PropertyKey::ContactEmail => translate_mailbox(node),
        _ => Err(refusal(PropertyRefusalKind::WrongEntryShape)),
    }
}

/// Translates one closed `{ <field>, exposure }` plain-text entry.
fn translate_text(
    node: &DeclaredNode,
    max_scalars: usize,
    field: &str,
) -> Result<AuthoredPropertyValueV1, PropertyDeclarationRefusalV1> {
    let members = closed_members(node, &[field, "exposure"])?;
    let exposure = exposure_of(required(&members, "exposure")?)?;
    let text = PlainTextV1::parse(as_string(required(&members, field)?)?, max_scalars)?;
    Ok(AuthoredPropertyValueV1::new(
        PropertyValueV1::text(text),
        exposure,
    ))
}

/// Translates one closed `{ address, exposure }` mailbox entry.
fn translate_mailbox(
    node: &DeclaredNode,
) -> Result<AuthoredPropertyValueV1, PropertyDeclarationRefusalV1> {
    let members = closed_members(node, &["address", "exposure"])?;
    let exposure = exposure_of(required(&members, "exposure")?)?;
    let mailbox = MailboxV1::parse(as_string(required(&members, "address")?)?)?;
    Ok(AuthoredPropertyValueV1::new(
        PropertyValueV1::mailbox(mailbox),
        exposure,
    ))
}

/// Translates one closed `{ source: { namespace, reference }, exposure }` graphic entry.
///
/// Only the declared reference is validated here. File existence, media decoding, the
/// platform catalogue and any fallback remain later packages and are never assumed.
fn translate_graphic(
    node: &DeclaredNode,
) -> Result<(AssetReferenceV1, Exposure), PropertyDeclarationRefusalV1> {
    let members = closed_members(node, &["source", "exposure"])?;
    let exposure = exposure_of(required(&members, "exposure")?)?;
    let source = closed_members(required(&members, "source")?, &["namespace", "reference"])?;
    let namespace = match AssetNamespace::from_name(as_string(required(&source, "namespace")?)?) {
        Some(namespace) => namespace,
        None => return Err(refusal(PropertyRefusalKind::InvalidAssetNamespace)),
    };
    let reference =
        AssetReferenceV1::parse(namespace, as_string(required(&source, "reference")?)?)?;
    Ok((reference, exposure))
}

/// Reads a closed entry object, refusing any member outside `allowed`.
fn closed_members<'a>(
    node: &'a DeclaredNode,
    allowed: &[&str],
) -> Result<Vec<(&'a str, &'a DeclaredNode)>, PropertyDeclarationRefusalV1> {
    let members = match node {
        DeclaredNode::Object(members) => members.as_slice(),
        _ => return Err(refusal(PropertyRefusalKind::WrongEntryShape)),
    };
    let mut result: Vec<(&'a str, &'a DeclaredNode)> = Vec::with_capacity(members.len());
    for (name, value) in members {
        if !allowed.contains(&name.as_str()) {
            return Err(refusal(PropertyRefusalKind::ExtraEntryField));
        }
        if result.iter().any(|(seen, _)| *seen == name.as_str()) {
            return Err(refusal(PropertyRefusalKind::ExtraEntryField));
        }
        result.push((name.as_str(), value));
    }
    Ok(result)
}

fn required<'a>(
    members: &[(&'a str, &'a DeclaredNode)],
    name: &str,
) -> Result<&'a DeclaredNode, PropertyDeclarationRefusalV1> {
    members
        .iter()
        .find(|(key, _)| *key == name)
        .map(|(_, value)| *value)
        .ok_or_else(|| refusal(PropertyRefusalKind::MissingEntryField))
}

fn as_string(node: &DeclaredNode) -> Result<&str, PropertyDeclarationRefusalV1> {
    match node {
        DeclaredNode::String(text) => Ok(text.as_str()),
        _ => Err(refusal(PropertyRefusalKind::WrongEntryShape)),
    }
}

fn as_number(node: &DeclaredNode) -> Result<f64, PropertyDeclarationRefusalV1> {
    match node {
        DeclaredNode::Number(number) => number
            .as_f64()
            .ok_or_else(|| refusal(PropertyRefusalKind::InvalidCoordinates)),
        _ => Err(refusal(PropertyRefusalKind::InvalidCoordinates)),
    }
}

fn exposure_of(node: &DeclaredNode) -> Result<Exposure, PropertyDeclarationRefusalV1> {
    match Exposure::from_name(as_string(node)?) {
        Some(exposure) => Ok(exposure),
        None => Err(refusal(PropertyRefusalKind::InvalidExposure)),
    }
}

/// Translates one location operation: a validated `add` fact or a recognized `extend`.
///
/// The `extend` branch validates only that the accepted §6.2H.4 envelope is present and
/// closed. It deliberately does **not** implement identified-record mutation: it returns the
/// typed unsupported request so a real resolution refuses by name instead of merging in
/// place, publishing a partial record or claiming the accepted syntax is invalid.
fn translate_location(
    node: &DeclaredNode,
) -> Result<(PropertyValueV1, Exposure), PropertyDeclarationRefusalV1> {
    let members = match node {
        DeclaredNode::Object(members) => members.as_slice(),
        _ => return Err(refusal(PropertyRefusalKind::WrongEntryShape)),
    };
    let operation = match members.iter().find(|(name, _)| name == "operation") {
        Some((_, DeclaredNode::String(text))) => text.as_str(),
        Some(_) => return Err(refusal(PropertyRefusalKind::InvalidOperation)),
        None => return Err(refusal(PropertyRefusalKind::MissingEntryField)),
    };
    match operation {
        "add" => translate_location_add(node),
        "extend" => translate_location_extend(node),
        _ => Err(refusal(PropertyRefusalKind::InvalidOperation)),
    }
}

fn translate_location_add(
    node: &DeclaredNode,
) -> Result<(PropertyValueV1, Exposure), PropertyDeclarationRefusalV1> {
    let members = closed_members(node, &["operation", "factId", "kind", "fields", "exposure"])?;
    let exposure = exposure_of(required(&members, "exposure")?)?;
    let fact_id = FactIdV1::parse(as_string(required(&members, "factId")?)?)?;
    let kind = match LocationKind::from_name(as_string(required(&members, "kind")?)?) {
        Some(kind) => kind,
        None => return Err(refusal(PropertyRefusalKind::InvalidLocationKind)),
    };
    let fields = translate_location_fields(required(&members, "fields")?)?;
    let fact = LocationFactV1::add(fact_id, kind, fields)?;
    Ok((PropertyValueV1::location(fact), exposure))
}

fn translate_location_extend(
    node: &DeclaredNode,
) -> Result<(PropertyValueV1, Exposure), PropertyDeclarationRefusalV1> {
    let members = closed_members(node, &["operation", "target", "set"])?;
    let target = closed_members(required(&members, "target")?, &["owner", "factId"])?;
    let fact_id = FactIdV1::parse(as_string(required(&target, "factId")?)?)?;
    let owner_target = closed_members(
        required(&target, "owner")?,
        &["entity", "facet", "site", "page", "locale"],
    )?;
    as_string(required(&owner_target, "entity")?)?;
    let set = location_field_members(required(&members, "set")?)?;
    if set.is_empty() {
        return Err(refusal(PropertyRefusalKind::EmptyLocationFields));
    }
    // An extension may never supply or change exposure (§6.2H.6), and it never becomes an
    // effective value in R03.A because resolution refuses it by name. `Internal` is the inert
    // placeholder and cannot elevate anything.
    Ok((
        PropertyValueV1::location_extension(LocationExtensionRequestV1::new(fact_id)),
        Exposure::Internal,
    ))
}

/// Translates one closed `LocationFields` object with at least one set field.
fn translate_location_fields(
    node: &DeclaredNode,
) -> Result<LocationFieldsV1, PropertyDeclarationRefusalV1> {
    let members = location_field_members(node)?;
    if members.is_empty() {
        return Err(refusal(PropertyRefusalKind::EmptyLocationFields));
    }
    let mut fields = LocationFieldsV1::new();
    for (name, value) in members {
        match name {
            "worldRegion" => fields.set_world_region(as_string(value)?)?,
            "countryCode" => fields.set_country_code(as_string(value)?)?,
            "administrativeArea" => fields.set_administrative_area(as_string(value)?)?,
            "locality" => fields.set_locality(as_string(value)?)?,
            "district" => fields.set_district(as_string(value)?)?,
            "namedPlace" => fields.set_named_place(as_string(value)?)?,
            "addressLine1" => fields.set_address_line1(as_string(value)?)?,
            "addressLine2" => fields.set_address_line2(as_string(value)?)?,
            "postalCode" => fields.set_postal_code(as_string(value)?)?,
            "coordinates" => {
                let coordinates = closed_members(value, &["latitude", "longitude"])?;
                let latitude = as_number(required(&coordinates, "latitude")?)?;
                let longitude = as_number(required(&coordinates, "longitude")?)?;
                fields.set_coordinates(CoordinatesV1::new(latitude, longitude)?);
            }
            _ => return Err(refusal(PropertyRefusalKind::UnknownLocationField)),
        }
    }
    Ok(fields)
}

/// Reads a location field object, refusing any name outside the closed §6.2H.3 shape.
fn location_field_members(
    node: &DeclaredNode,
) -> Result<Vec<(&str, &DeclaredNode)>, PropertyDeclarationRefusalV1> {
    let members = match node {
        DeclaredNode::Object(members) => members.as_slice(),
        _ => return Err(refusal(PropertyRefusalKind::WrongEntryShape)),
    };
    let mut result: Vec<(&str, &DeclaredNode)> = Vec::with_capacity(members.len());
    for (name, value) in members {
        if !LOCATION_FIELD_NAMES.contains(&name.as_str()) {
            return Err(refusal(PropertyRefusalKind::UnknownLocationField));
        }
        if result.iter().any(|(seen, _)| *seen == name.as_str()) {
            return Err(refusal(PropertyRefusalKind::InvalidLocationField));
        }
        result.push((name.as_str(), value));
    }
    Ok(result)
}
