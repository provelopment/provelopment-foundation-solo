//! Typed `propertyRegistryVersion: "1.0.0"` property registry (`R03.A`).
//!
//! This module is the pure, closed description of the nine accepted property
//! identities of `ARCHITECTURE.md` §6.2G.4 and the typed values they may carry. It
//! performs no resolution: [`crate::property_resolution`] owns the five-level
//! cascade, the effective cardinality gate and the derived provenance record, and a
//! separate public projection decides what may ever be rendered.
//!
//! Everything here is a refusal-first value object. There is no generic property bag,
//! no custom key, no coercion, no trimming, no case folding, no inferred geography and
//! no default value: an authored entry that does not already match the closed shape is
//! refused, and the caller must author a correct one. The registry is pure Rust over
//! the standard library, so it imports no JSON, transport, filesystem or provider
//! concern.
//!
//! # What this module deliberately does not do
//!
//! * It never decides publication. A validated declaration set is authored input, not
//!   manifest, route, locale or snapshot authority.
//! * It never resolves inheritance. See [`crate::property_resolution`].
//! * It never inspects an asset file, media container or platform catalogue. A graphic
//!   `source` is a *declared reference* only (§6.2I.1); existence, decoding, fallback
//!   and delivery remain later packages.
//! * A `location` `extend` operation is **recognized** as the accepted §6.2H.4 syntax and
//!   carried as an explicit unsupported request. Resolution refuses it by name; it is
//!   never merged, never treated as `add` and never reported as invalid syntax.

use core::fmt;

use crate::efs_ownership::{EfsLevel, FacetIdV1, LocaleKeyV1, PageIdV1, SiteIdV1};
use crate::entity_id::EntityIdV1;

/// The one accepted property-registry version (§6.2G.1).
pub const PROPERTY_REGISTRY_VERSION: &str = "1.0.0";

/// Effective maximum number of `location` facts (§6.2G.4).
pub const MAX_LOCATION_FACTS: usize = 16;

/// Effective maximum number of `graphic.background` layers (§6.2G.4).
pub const MAX_BACKGROUND_LAYERS: usize = 4;

/// Maximum `seo.title` length in Unicode scalar values (§6.2G.4).
pub const MAX_SEO_TITLE_SCALARS: usize = 140;

/// Maximum `seo.description` length in Unicode scalar values (§6.2G.4).
pub const MAX_SEO_DESCRIPTION_SCALARS: usize = 300;

/// Maximum `worldRegion` length in Unicode scalar values (§6.2H.3).
pub const MAX_WORLD_REGION_SCALARS: usize = 80;

/// Maximum length of `administrativeArea`, `locality` or `district` (§6.2H.3).
pub const MAX_AREA_SCALARS: usize = 120;

/// Maximum `namedPlace` length in Unicode scalar values (§6.2H.3).
pub const MAX_NAMED_PLACE_SCALARS: usize = 160;

/// Maximum length of one address line (§6.2H.3).
pub const MAX_ADDRESS_LINE_SCALARS: usize = 200;

/// Maximum `postalCode` length in Unicode scalar values (§6.2H.3).
pub const MAX_POSTAL_CODE_SCALARS: usize = 32;

/// Maximum length of one portable asset reference, in ASCII bytes.
pub const MAX_ASSET_REFERENCE_BYTES: usize = 200;

/// Maximum length of one safe mailbox, in ASCII bytes.
pub const MAX_MAILBOX_BYTES: usize = 254;

/// Maximum number of local entries one property declaration may carry.
///
/// The bound is deliberate and generous: the effective registry maximum still refuses
/// anything shorter that exceeds its own cardinality.
pub const MAX_LOCAL_ENTRIES: usize = 16;

/// The nine accepted property identities of the initial registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PropertyKey {
    /// A §6.2H location fact cascade.
    Location,
    /// A logo graphic selection.
    GraphicLogo,
    /// A favicon graphic selection.
    GraphicFavicon,
    /// A banner graphic selection.
    GraphicBanner,
    /// A background graphic selection.
    GraphicBackground,
    /// A social-preview graphic selection.
    GraphicSocialPreview,
    /// A plain-text SEO title.
    SeoTitle,
    /// A plain-text SEO description.
    SeoDescription,
    /// A single safe contact mailbox.
    ContactEmail,
}

impl PropertyKey {
    /// Every registered key, in registry order.
    pub const ALL: [Self; 9] = [
        Self::Location,
        Self::GraphicLogo,
        Self::GraphicFavicon,
        Self::GraphicBanner,
        Self::GraphicBackground,
        Self::GraphicSocialPreview,
        Self::SeoTitle,
        Self::SeoDescription,
        Self::ContactEmail,
    ];

    /// The exact registered manifest name of this key.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Location => "location",
            Self::GraphicLogo => "graphic.logo",
            Self::GraphicFavicon => "graphic.favicon",
            Self::GraphicBanner => "graphic.banner",
            Self::GraphicBackground => "graphic.background",
            Self::GraphicSocialPreview => "graphic.social-preview",
            Self::SeoTitle => "seo.title",
            Self::SeoDescription => "seo.description",
            Self::ContactEmail => "contact.email",
        }
    }

    /// Looks up a registered key by its exact authored name.
    ///
    /// The match is exact and case-sensitive: `SEO.Title`, `seo_title` and every other
    /// spelling are unregistered names rather than aliases.
    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|key| key.name() == name)
    }

    /// The effective maximum of this key, counted **before** any public projection.
    #[must_use]
    pub const fn max_effective(self) -> usize {
        match self {
            Self::Location => MAX_LOCATION_FACTS,
            Self::GraphicBackground => MAX_BACKGROUND_LAYERS,
            _ => 1,
        }
    }

    /// Whether this key may be declared at `level` (§6.2G.4).
    ///
    /// Only `seo.*` is restricted: it is unavailable at Entity and Facet level.
    #[must_use]
    pub const fn allows_level(self, level: EfsLevel) -> bool {
        match self {
            Self::SeoTitle | Self::SeoDescription => {
                !matches!(level, EfsLevel::Entity | EfsLevel::Facet)
            }
            _ => true,
        }
    }

    /// Whether this key is the location fact collection.
    #[must_use]
    pub const fn is_location(self) -> bool {
        matches!(self, Self::Location)
    }

    /// Whether this key is one of the graphic role selections.
    #[must_use]
    pub const fn is_graphic(self) -> bool {
        matches!(
            self,
            Self::GraphicLogo
                | Self::GraphicFavicon
                | Self::GraphicBanner
                | Self::GraphicBackground
                | Self::GraphicSocialPreview
        )
    }
}

impl fmt::Display for PropertyKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

/// The two exposure values of §6.2G.3.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Exposure {
    /// Eligible for the validated public projection.
    Public,
    /// Never eligible for the public projection.
    Internal,
}

impl Exposure {
    /// Parses the exact authored spelling.
    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "public" => Some(Self::Public),
            "internal" => Some(Self::Internal),
            _ => None,
        }
    }

    /// The exact authored spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Public => "public",
            Self::Internal => "internal",
        }
    }

    /// Whether this exposure may appear in the public projection.
    #[must_use]
    pub const fn is_public(self) -> bool {
        matches!(self, Self::Public)
    }
}

/// The two accepted declaration inheritance modes of §6.2G.2.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Inheritance {
    /// Keep the inherited effective values, then append local contributions.
    Inherit,
    /// Clear the inherited effective values, then append local contributions.
    Suppress,
}

impl Inheritance {
    /// Parses the exact authored spelling.
    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "inherit" => Some(Self::Inherit),
            "suppress" => Some(Self::Suppress),
            _ => None,
        }
    }

    /// The exact authored spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Inherit => "inherit",
            Self::Suppress => "suppress",
        }
    }
}

/// Why one authored property declaration was refused.
///
/// Every variant maps to a stable token, and no variant ever carries authored content,
/// a filesystem path or a secret. The categories are the closed refusals of §6.2G,
/// §6.2H and §6.2I.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PropertyRefusalKind {
    /// The property name is not one of the nine registered keys.
    UnknownProperty,
    /// The declared key is not allowed at this EFS level.
    WrongLevel,
    /// A required declaration-envelope field was absent.
    MissingEnvelopeField,
    /// The declaration envelope carried a member outside its closed shape.
    ExtraEnvelopeField,
    /// `inheritance` was not exactly `inherit` or `suppress`.
    InvalidInheritance,
    /// `currentContext` was neither the literal `"none"` nor an array.
    InvalidCurrentContext,
    /// `currentContext` was an empty array.
    EmptyContextArray,
    /// The same property was declared twice at one owner.
    DuplicatePropertyEntry,
    /// A local declaration array exceeded the local bound.
    TooManyLocalEntries,
    /// A required entry member was absent.
    MissingEntryField,
    /// An entry carried a member outside its closed shape.
    ExtraEntryField,
    /// `exposure` was not exactly `public` or `internal`.
    InvalidExposure,
    /// The typed value shape does not belong to the declared key.
    WrongEntryShape,
    /// A plain-text value was empty.
    EmptyPlainText,
    /// A plain-text value exceeded its key's scalar bound.
    PlainTextTooLong,
    /// A plain-text value carried a control character.
    ControlCharacter,
    /// A plain-text value carried markup delimiters.
    ActiveMarkup,
    /// A mailbox was not one safe addr-spec.
    InvalidMailbox,
    /// An asset namespace was neither `entity` nor `platform`.
    InvalidAssetNamespace,
    /// An asset reference was not a portable relative image file.
    InvalidAssetReference,
    /// An asset reference used an image extension outside the accepted set.
    UnsupportedAssetExtension,
    /// A favicon selection was not a PNG reference.
    FaviconNotPng,
    /// A location fact identifier did not match the accepted technical-ID grammar.
    InvalidFactId,
    /// A location kind was neither `association` nor `premises`.
    InvalidLocationKind,
    /// A location fact carried no field.
    EmptyLocationFields,
    /// A location field name is not part of the closed `LocationFields` shape.
    UnknownLocationField,
    /// A location field value was outside its accepted type or bound.
    InvalidLocationField,
    /// A country code was not an explicit uppercase ISO 3166-1 alpha-2 code.
    InvalidCountryCode,
    /// Coordinates were absent, non-finite or outside the WGS84 ranges.
    InvalidCoordinates,
    /// A location operation was neither `add` nor `extend`.
    InvalidOperation,
    /// An authored `extend` wrapper claimed a public exposure, which section 6.2H.6 forbids.
    ExtensionExposureForbidden,
    /// The declared property-registry version is not the accepted `1.0.0`.
    UnsupportedRegistryVersion,
    /// A qualified owner was not internally consistent.
    InconsistentOwner,
}

impl PropertyRefusalKind {
    /// Stable, safe diagnostic token for this refusal reason.
    #[must_use]
    pub const fn detail(self) -> &'static str {
        match self {
            Self::UnknownProperty => "unknown-property",
            Self::WrongLevel => "wrong-level",
            Self::MissingEnvelopeField => "missing-envelope-field",
            Self::ExtraEnvelopeField => "extra-envelope-field",
            Self::InvalidInheritance => "invalid-inheritance",
            Self::InvalidCurrentContext => "invalid-current-context",
            Self::EmptyContextArray => "empty-context-array",
            Self::DuplicatePropertyEntry => "duplicate-property-entry",
            Self::TooManyLocalEntries => "too-many-local-entries",
            Self::MissingEntryField => "missing-entry-field",
            Self::ExtraEntryField => "extra-entry-field",
            Self::InvalidExposure => "invalid-exposure",
            Self::WrongEntryShape => "wrong-entry-shape",
            Self::EmptyPlainText => "empty-plain-text",
            Self::PlainTextTooLong => "plain-text-too-long",
            Self::ControlCharacter => "control-character",
            Self::ActiveMarkup => "active-markup",
            Self::InvalidMailbox => "invalid-mailbox",
            Self::InvalidAssetNamespace => "invalid-asset-namespace",
            Self::InvalidAssetReference => "invalid-asset-reference",
            Self::UnsupportedAssetExtension => "unsupported-asset-extension",
            Self::FaviconNotPng => "favicon-not-png",
            Self::InvalidFactId => "invalid-fact-id",
            Self::InvalidLocationKind => "invalid-location-kind",
            Self::EmptyLocationFields => "empty-location-fields",
            Self::UnknownLocationField => "unknown-location-field",
            Self::InvalidLocationField => "invalid-location-field",
            Self::InvalidCountryCode => "invalid-country-code",
            Self::InvalidCoordinates => "invalid-coordinates",
            Self::InvalidOperation => "invalid-operation",
            Self::ExtensionExposureForbidden => "extension-exposure-forbidden",
            Self::UnsupportedRegistryVersion => "unsupported-registry-version",
            Self::InconsistentOwner => "inconsistent-owner",
        }
    }
}

/// Refusal of one authored property declaration.
///
/// The value carries only the refusal category and, when the refusal is specific to one
/// registered key, that key. It never carries authored content, so a diagnostic can never
/// echo a refused property value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PropertyDeclarationRefusalV1 {
    kind: PropertyRefusalKind,
    key: Option<PropertyKey>,
}

impl PropertyDeclarationRefusalV1 {
    /// Wraps one refusal category.
    ///
    /// This constructor is available to the inward-facing translation layers so that a
    /// closed-envelope refusal discovered while reading authored input uses the same
    /// typed category as a domain-level refusal.
    #[must_use]
    pub const fn of(kind: PropertyRefusalKind) -> Self {
        Self { kind, key: None }
    }

    /// Attaches the registered key this refusal is specific to.
    #[must_use]
    pub const fn with_key(self, key: PropertyKey) -> Self {
        Self {
            kind: self.kind,
            key: Some(key),
        }
    }

    /// The refusal category.
    #[must_use]
    pub const fn kind(&self) -> PropertyRefusalKind {
        self.kind
    }

    /// The registered key this refusal is specific to, when known.
    #[must_use]
    pub const fn key(&self) -> Option<PropertyKey> {
        self.key
    }

    /// Stable engineering code for this refusal.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        "PROPERTY_DECLARATION_REFUSED"
    }

    /// Stable, safe diagnostic token for the refusal reason.
    #[must_use]
    pub const fn detail(&self) -> &'static str {
        self.kind.detail()
    }
}

impl fmt::Display for PropertyDeclarationRefusalV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code(), self.detail())
    }
}

impl std::error::Error for PropertyDeclarationRefusalV1 {}

/// A validated single-line plain-text value.
///
/// The value is stored exactly as authored: it is never trimmed, case-folded or
/// truncated. Empty text, control characters, markup delimiters and values beyond the
/// requested scalar bound are refused instead.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlainTextV1(String);

impl PlainTextV1 {
    /// Validates plain text against a scalar bound and the plain-text safety rules.
    ///
    /// # Errors
    ///
    /// Returns a [`PropertyDeclarationRefusalV1`] of kind
    /// [`PropertyRefusalKind::EmptyPlainText`],
    /// [`PropertyRefusalKind::ControlCharacter`],
    /// [`PropertyRefusalKind::ActiveMarkup`] or
    /// [`PropertyRefusalKind::PlainTextTooLong`].
    pub fn parse(
        candidate: &str,
        max_scalars: usize,
    ) -> Result<Self, PropertyDeclarationRefusalV1> {
        if candidate.is_empty() {
            return Err(PropertyDeclarationRefusalV1::of(
                PropertyRefusalKind::EmptyPlainText,
            ));
        }
        for character in candidate.chars() {
            if character.is_control() {
                return Err(PropertyDeclarationRefusalV1::of(
                    PropertyRefusalKind::ControlCharacter,
                ));
            }
            if character == '<' || character == '>' {
                return Err(PropertyDeclarationRefusalV1::of(
                    PropertyRefusalKind::ActiveMarkup,
                ));
            }
        }
        if candidate.chars().count() > max_scalars {
            return Err(PropertyDeclarationRefusalV1::of(
                PropertyRefusalKind::PlainTextTooLong,
            ));
        }
        Ok(Self(candidate.to_owned()))
    }

    /// The validated text as authored.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// The text length in Unicode scalar values.
    #[must_use]
    pub fn scalar_len(&self) -> usize {
        self.0.chars().count()
    }
}

impl fmt::Display for PlainTextV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// A validated single contact mailbox.
///
/// This is one plain addr-spec and nothing else: no display name, no angle brackets, no
/// comment, no list, no header, no line break and no `mailto:` endpoint. It implies no
/// contact submission surface and no deliverability claim.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MailboxV1(String);

impl MailboxV1 {
    /// Validates one authored mailbox.
    ///
    /// # Errors
    ///
    /// Returns a [`PropertyDeclarationRefusalV1`] of kind
    /// [`PropertyRefusalKind::InvalidMailbox`] for anything that is not exactly one safe
    /// ASCII addr-spec.
    pub fn parse(candidate: &str) -> Result<Self, PropertyDeclarationRefusalV1> {
        let refused = || PropertyDeclarationRefusalV1::of(PropertyRefusalKind::InvalidMailbox);
        if candidate.is_empty() || candidate.len() > MAX_MAILBOX_BYTES {
            return Err(refused());
        }
        if !candidate.is_ascii() {
            return Err(refused());
        }
        let mut parts = candidate.split('@');
        let local = match parts.next() {
            Some(part) => part,
            None => return Err(refused()),
        };
        let domain = match parts.next() {
            Some(part) => part,
            None => return Err(refused()),
        };
        if parts.next().is_some() {
            return Err(refused());
        }
        if !is_safe_mailbox_local(local) || !is_safe_mailbox_domain(domain) {
            return Err(refused());
        }
        Ok(Self(candidate.to_owned()))
    }

    /// The validated mailbox as authored.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

const MAILBOX_LOCAL_SPECIALS: [char; 20] = [
    '!', '#', '$', '%', '&', '\'', '*', '+', '-', '/', '=', '?', '^', '_', '`', '{', '|', '}', '~',
    '.',
];

fn is_safe_mailbox_local(local: &str) -> bool {
    if local.is_empty() || local.len() > 64 {
        return false;
    }
    if local.starts_with('.') || local.ends_with('.') || local.contains("..") {
        return false;
    }
    local.chars().all(|character| {
        character.is_ascii_alphanumeric() || MAILBOX_LOCAL_SPECIALS.contains(&character)
    })
}

fn is_safe_mailbox_domain(domain: &str) -> bool {
    if domain.is_empty() || domain.len() > 190 || !domain.contains('.') {
        return false;
    }
    let mut labels = domain.split('.');
    let mut last = "";
    for label in &mut labels {
        if label.is_empty() || label.len() > 63 {
            return false;
        }
        if label.starts_with('-') || label.ends_with('-') {
            return false;
        }
        if !label
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || character == '-')
        {
            return false;
        }
        last = label;
    }
    last.len() >= 2
        && last
            .chars()
            .all(|character| character.is_ascii_alphabetic())
}

/// The two asset namespaces of §6.2I.1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AssetNamespace {
    /// An asset owned by the one Entity's own source tree.
    Entity,
    /// An asset of the pinned shared platform library.
    Platform,
}

impl AssetNamespace {
    /// Parses the exact authored spelling.
    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "entity" => Some(Self::Entity),
            "platform" => Some(Self::Platform),
            _ => None,
        }
    }

    /// The exact authored spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Entity => "entity",
            Self::Platform => "platform",
        }
    }
}

/// The image extensions the initial profile admits (§6.2I.3).
pub const ACCEPTED_IMAGE_EXTENSIONS: [&str; 5] = ["png", "jpg", "jpeg", "webp", "avif"];

/// A validated typed asset **reference**.
///
/// This value names a portable relative image file. It opens no file, decodes no media,
/// resolves no platform-library entry and performs no fallback: those are later packages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetReferenceV1 {
    namespace: AssetNamespace,
    reference: String,
}

impl AssetReferenceV1 {
    /// Validates one authored typed asset reference.
    ///
    /// # Errors
    ///
    /// Returns a [`PropertyDeclarationRefusalV1`] of kind
    /// [`PropertyRefusalKind::InvalidAssetReference`] or
    /// [`PropertyRefusalKind::UnsupportedAssetExtension`].
    pub fn parse(
        namespace: AssetNamespace,
        reference: &str,
    ) -> Result<Self, PropertyDeclarationRefusalV1> {
        let refused =
            || PropertyDeclarationRefusalV1::of(PropertyRefusalKind::InvalidAssetReference);
        if reference.is_empty()
            || reference.len() > MAX_ASSET_REFERENCE_BYTES
            || !reference.is_ascii()
        {
            return Err(refused());
        }
        if reference.starts_with('/') || reference.ends_with('/') {
            return Err(refused());
        }
        for character in reference.chars() {
            if character.is_ascii_control()
                || matches!(
                    character,
                    '\\' | ':' | '?' | '#' | '%' | '*' | '"' | '<' | '>' | '|'
                )
            {
                return Err(refused());
            }
        }
        let mut segments = reference.split('/');
        let mut last = "";
        for segment in &mut segments {
            if segment.is_empty() || segment == "." || segment == ".." {
                return Err(refused());
            }
            if segment.starts_with('.') || segment.ends_with('.') {
                return Err(refused());
            }
            if !segment.chars().all(|character| {
                character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | '-')
            }) {
                return Err(refused());
            }
            last = segment;
        }
        let extension = match extension_of(last) {
            Some(value) => value,
            None => return Err(refused()),
        };
        if !ACCEPTED_IMAGE_EXTENSIONS.contains(&extension) {
            return Err(PropertyDeclarationRefusalV1::of(
                PropertyRefusalKind::UnsupportedAssetExtension,
            ));
        }
        Ok(Self {
            namespace,
            reference: reference.to_owned(),
        })
    }

    /// The typed namespace.
    #[must_use]
    pub const fn namespace(&self) -> AssetNamespace {
        self.namespace
    }

    /// The validated portable relative reference as authored.
    #[must_use]
    pub fn reference(&self) -> &str {
        &self.reference
    }

    /// Whether this reference names a PNG file.
    #[must_use]
    pub fn is_png(&self) -> bool {
        self.reference
            .rsplit_once('.')
            .is_some_and(|(_, extension)| extension == "png")
    }
}

/// Validated WGS84 coordinates (§6.2H.3, §6.2H.7).
///
/// Only finiteness and the accepted ranges are checked. No reprojection, reverse
/// geocoding, distance, elevation or accuracy fact is inferred.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CoordinatesV1 {
    latitude: f64,
    longitude: f64,
}

impl CoordinatesV1 {
    /// Validates one latitude/longitude pair.
    ///
    /// # Errors
    ///
    /// Returns [`PropertyRefusalKind::InvalidCoordinates`] for a non-finite or
    /// out-of-range pair.
    pub fn new(latitude: f64, longitude: f64) -> Result<Self, PropertyDeclarationRefusalV1> {
        let refused = || PropertyDeclarationRefusalV1::of(PropertyRefusalKind::InvalidCoordinates);
        if !latitude.is_finite() || !longitude.is_finite() {
            return Err(refused());
        }
        if !(-90.0..=90.0).contains(&latitude) || !(-180.0..=180.0).contains(&longitude) {
            return Err(refused());
        }
        Ok(Self {
            latitude,
            longitude,
        })
    }

    /// The validated latitude.
    #[must_use]
    pub const fn latitude(&self) -> f64 {
        self.latitude
    }

    /// The validated longitude.
    #[must_use]
    pub const fn longitude(&self) -> f64 {
        self.longitude
    }
}

/// The exact field names of the closed `LocationFields` object, in §6.2H.3 order.
pub const LOCATION_FIELD_NAMES: [&str; 10] = [
    "worldRegion",
    "countryCode",
    "administrativeArea",
    "locality",
    "district",
    "namedPlace",
    "addressLine1",
    "addressLine2",
    "postalCode",
    "coordinates",
];

/// A validated, closed `LocationFields` object (§6.2H.3).
///
/// Every member is optional in the shape, but at least one must be present before a fact
/// may exist, omitted geographic levels are never auto-filled, and a partial address is
/// lawful for both kinds.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LocationFieldsV1 {
    world_region: Option<String>,
    country_code: Option<String>,
    administrative_area: Option<String>,
    locality: Option<String>,
    district: Option<String>,
    named_place: Option<String>,
    address_line1: Option<String>,
    address_line2: Option<String>,
    postal_code: Option<String>,
    coordinates: Option<CoordinatesV1>,
}

/// The extension of one file name, when it has a non-empty stem and extension.
fn extension_of(file_name: &str) -> Option<&str> {
    let (stem, extension) = file_name.rsplit_once('.')?;
    if stem.is_empty() || extension.is_empty() {
        return None;
    }
    Some(extension)
}

impl LocationFieldsV1 {
    /// An empty field set. It is not yet a valid location fact field set.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets `worldRegion`.
    ///
    /// # Errors
    ///
    /// Returns [`PropertyRefusalKind::InvalidLocationField`] for empty text, a control
    /// character, a markup delimiter or a value beyond its scalar bound.
    pub fn set_world_region(&mut self, value: &str) -> Result<(), PropertyDeclarationRefusalV1> {
        self.world_region = Some(location_text(value, MAX_WORLD_REGION_SCALARS)?);
        Ok(())
    }

    /// Sets an explicit uppercase ISO 3166-1 alpha-2 `countryCode`.
    ///
    /// # Errors
    ///
    /// Returns [`PropertyRefusalKind::InvalidCountryCode`] for anything that is not
    /// exactly two uppercase ASCII letters. The value is never inferred or case-folded.
    pub fn set_country_code(&mut self, value: &str) -> Result<(), PropertyDeclarationRefusalV1> {
        let bytes = value.as_bytes();
        if bytes.len() != 2 || !bytes.iter().all(u8::is_ascii_uppercase) {
            return Err(PropertyDeclarationRefusalV1::of(
                PropertyRefusalKind::InvalidCountryCode,
            ));
        }
        self.country_code = Some(value.to_owned());
        Ok(())
    }

    /// Sets `administrativeArea`.
    ///
    /// # Errors
    ///
    /// Returns [`PropertyRefusalKind::InvalidLocationField`] for invalid text.
    pub fn set_administrative_area(
        &mut self,
        value: &str,
    ) -> Result<(), PropertyDeclarationRefusalV1> {
        self.administrative_area = Some(location_text(value, MAX_AREA_SCALARS)?);
        Ok(())
    }

    /// Sets `locality`.
    ///
    /// # Errors
    ///
    /// Returns [`PropertyRefusalKind::InvalidLocationField`] for invalid text.
    pub fn set_locality(&mut self, value: &str) -> Result<(), PropertyDeclarationRefusalV1> {
        self.locality = Some(location_text(value, MAX_AREA_SCALARS)?);
        Ok(())
    }
}

impl LocationFieldsV1 {
    /// Sets `district`.
    ///
    /// # Errors
    ///
    /// Returns [`PropertyRefusalKind::InvalidLocationField`] for invalid text.
    pub fn set_district(&mut self, value: &str) -> Result<(), PropertyDeclarationRefusalV1> {
        self.district = Some(location_text(value, MAX_AREA_SCALARS)?);
        Ok(())
    }

    /// Sets `namedPlace`.
    ///
    /// # Errors
    ///
    /// Returns [`PropertyRefusalKind::InvalidLocationField`] for invalid text.
    pub fn set_named_place(&mut self, value: &str) -> Result<(), PropertyDeclarationRefusalV1> {
        self.named_place = Some(location_text(value, MAX_NAMED_PLACE_SCALARS)?);
        Ok(())
    }

    /// Sets `addressLine1`.
    ///
    /// # Errors
    ///
    /// Returns [`PropertyRefusalKind::InvalidLocationField`] for invalid text.
    pub fn set_address_line1(&mut self, value: &str) -> Result<(), PropertyDeclarationRefusalV1> {
        self.address_line1 = Some(location_text(value, MAX_ADDRESS_LINE_SCALARS)?);
        Ok(())
    }

    /// Sets `addressLine2`.
    ///
    /// # Errors
    ///
    /// Returns [`PropertyRefusalKind::InvalidLocationField`] for invalid text.
    pub fn set_address_line2(&mut self, value: &str) -> Result<(), PropertyDeclarationRefusalV1> {
        self.address_line2 = Some(location_text(value, MAX_ADDRESS_LINE_SCALARS)?);
        Ok(())
    }

    /// Sets `postalCode`.
    ///
    /// # Errors
    ///
    /// Returns [`PropertyRefusalKind::InvalidLocationField`] for invalid text.
    pub fn set_postal_code(&mut self, value: &str) -> Result<(), PropertyDeclarationRefusalV1> {
        self.postal_code = Some(location_text(value, MAX_POSTAL_CODE_SCALARS)?);
        Ok(())
    }

    /// Sets validated `coordinates`.
    pub fn set_coordinates(&mut self, coordinates: CoordinatesV1) {
        self.coordinates = Some(coordinates);
    }

    /// Whether no field is set.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.set_field_names().is_empty()
    }

    /// Overlays the members that `set` explicitly carries onto this field set (§6.2H.4).
    ///
    /// Only members present in `set` are written: an existing member is overwritten, an absent
    /// member is added, and every member `set` does not mention is preserved exactly as it was.
    /// `coordinates` is one atomic field, so it is either replaced whole or left untouched.
    /// This is the single bounded typed overlay the resolver uses; no JSON, string map or
    /// parallel location type is involved, and no member is ever removed or defaulted.
    pub fn overlay(&mut self, set: &Self) {
        if let Some(value) = &set.world_region {
            self.world_region = Some(value.clone());
        }
        if let Some(value) = &set.country_code {
            self.country_code = Some(value.clone());
        }
        if let Some(value) = &set.administrative_area {
            self.administrative_area = Some(value.clone());
        }
        if let Some(value) = &set.locality {
            self.locality = Some(value.clone());
        }
        if let Some(value) = &set.district {
            self.district = Some(value.clone());
        }
        if let Some(value) = &set.named_place {
            self.named_place = Some(value.clone());
        }
        if let Some(value) = &set.address_line1 {
            self.address_line1 = Some(value.clone());
        }
        if let Some(value) = &set.address_line2 {
            self.address_line2 = Some(value.clone());
        }
        if let Some(value) = &set.postal_code {
            self.postal_code = Some(value.clone());
        }
        if let Some(value) = &set.coordinates {
            self.coordinates = Some(*value);
        }
    }

    /// The set field names in the closed §6.2H.3 order.
    #[must_use]
    pub fn set_field_names(&self) -> Vec<&'static str> {
        let mut names = Vec::new();
        if self.world_region.is_some() {
            names.push("worldRegion");
        }
        if self.country_code.is_some() {
            names.push("countryCode");
        }
        if self.administrative_area.is_some() {
            names.push("administrativeArea");
        }
        if self.locality.is_some() {
            names.push("locality");
        }
        if self.district.is_some() {
            names.push("district");
        }
        if self.named_place.is_some() {
            names.push("namedPlace");
        }
        if self.address_line1.is_some() {
            names.push("addressLine1");
        }
        if self.address_line2.is_some() {
            names.push("addressLine2");
        }
        if self.postal_code.is_some() {
            names.push("postalCode");
        }
        if self.coordinates.is_some() {
            names.push("coordinates");
        }
        names
    }
}

fn location_text(value: &str, max_scalars: usize) -> Result<String, PropertyDeclarationRefusalV1> {
    PlainTextV1::parse(value, max_scalars)
        .map(|text| text.as_str().to_owned())
        .map_err(|_| PropertyDeclarationRefusalV1::of(PropertyRefusalKind::InvalidLocationField))
}

/// The two accepted location fact kinds (§6.2H.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocationKind {
    /// A conceptual association.
    Association,
    /// Premises.
    Premises,
}

impl LocationKind {
    /// Parses the exact authored spelling.
    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "association" => Some(Self::Association),
            "premises" => Some(Self::Premises),
            _ => None,
        }
    }

    /// The exact authored spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Association => "association",
            Self::Premises => "premises",
        }
    }
}

/// A validated location fact identifier.
///
/// `factId` uses the accepted technical-ID grammar: 1 to 63 characters matching
/// `^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$`. The value is never generated, folded or repaired.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FactIdV1(String);

impl FactIdV1 {
    /// Validates one authored fact identifier.
    ///
    /// # Errors
    ///
    /// Returns [`PropertyRefusalKind::InvalidFactId`] for any value outside the accepted
    /// technical-ID grammar.
    pub fn parse(candidate: &str) -> Result<Self, PropertyDeclarationRefusalV1> {
        EntityIdV1::parse(candidate)
            .map(|_| Self(candidate.to_owned()))
            .map_err(|_| PropertyDeclarationRefusalV1::of(PropertyRefusalKind::InvalidFactId))
    }

    /// The validated identifier as authored.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for FactIdV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// A validated §6.2H.2 location `add` fact.
///
/// The fact keeps its own `factId`, kind and whole-record fields. Its identity is the
/// pair `(original qualified owner, factId)` and that pair is supplied by the resolution
/// layer, never authored twice.
#[derive(Debug, Clone, PartialEq)]
pub struct LocationFactV1 {
    fact_id: FactIdV1,
    kind: LocationKind,
    fields: LocationFieldsV1,
}

impl LocationFactV1 {
    /// Validates one authored `add` fact.
    ///
    /// # Errors
    ///
    /// Returns [`PropertyRefusalKind::EmptyLocationFields`] when no field is present.
    pub fn add(
        fact_id: FactIdV1,
        kind: LocationKind,
        fields: LocationFieldsV1,
    ) -> Result<Self, PropertyDeclarationRefusalV1> {
        if fields.is_empty() {
            return Err(PropertyDeclarationRefusalV1::of(
                PropertyRefusalKind::EmptyLocationFields,
            ));
        }
        Ok(Self {
            fact_id,
            kind,
            fields,
        })
    }

    /// The stable fact identifier.
    #[must_use]
    pub fn fact_id(&self) -> &FactIdV1 {
        &self.fact_id
    }

    /// The fact kind.
    #[must_use]
    pub const fn kind(&self) -> LocationKind {
        self.kind
    }

    /// The validated closed fields.
    #[must_use]
    pub fn fields(&self) -> &LocationFieldsV1 {
        &self.fields
    }

    /// The same identified fact with `set` overlaid in place (§6.2H.4).
    ///
    /// The fact identifier, kind and every member `set` does not name are preserved exactly;
    /// nothing is removed, renumbered, re-exposed or re-created. The returned value is the
    /// same fact, so an extension can never mint a new identity or a new effective position.
    #[must_use]
    pub fn with_overlay(&self, set: &LocationFieldsV1) -> Self {
        let mut fields = self.fields.clone();
        fields.overlay(set);
        Self {
            fact_id: self.fact_id.clone(),
            kind: self.kind,
            fields,
        }
    }
}

/// A validated §6.2H.4 `extend` request naming one inherited fact and the members to set.
///
/// The request carries only the fully qualified **original** owner, the stable fact identifier
/// and the typed `set` members. It never carries an exposure: the accepted contract forbids an
/// authored extension exposure, so a request can never change, promote or replace the exposed
/// state of the fact it names. Construction refuses an empty `set`, so this type can never
/// express a no-op extension, and identity resolution stays the resolver's decision.
#[derive(Debug, Clone, PartialEq)]
pub struct LocationExtensionRequestV1 {
    target_fact_id: FactIdV1,
    target_owner: QualifiedOwnerV1,
    set: LocationFieldsV1,
}

impl LocationExtensionRequestV1 {
    /// Validates one extension request.
    ///
    /// # Errors
    ///
    /// Returns [`PropertyRefusalKind::EmptyLocationFields`] when `set` carries no member: an
    /// extension must explicitly set at least one field.
    pub fn new(
        target_fact_id: FactIdV1,
        target_owner: QualifiedOwnerV1,
        set: LocationFieldsV1,
    ) -> Result<Self, PropertyDeclarationRefusalV1> {
        if set.is_empty() {
            return Err(PropertyDeclarationRefusalV1::of(
                PropertyRefusalKind::EmptyLocationFields,
            ));
        }
        Ok(Self {
            target_fact_id,
            target_owner,
            set,
        })
    }

    /// The fact the extension names.
    #[must_use]
    pub const fn target_fact_id(&self) -> &FactIdV1 {
        &self.target_fact_id
    }

    /// The qualified owner the extension names.
    #[must_use]
    pub const fn target_owner(&self) -> &QualifiedOwnerV1 {
        &self.target_owner
    }

    /// The validated partial fields the extension would set.
    #[must_use]
    pub const fn set(&self) -> &LocationFieldsV1 {
        &self.set
    }
}

/// One typed authored property value.
#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValueV1 {
    /// A §6.2H.2 location `add` fact.
    Location(Box<LocationFactV1>),
    /// A §6.2H.4 `extend` operation targeting one inherited fact.
    LocationExtension(Box<LocationExtensionRequestV1>),
    /// A graphic role selection reference.
    Graphic(AssetReferenceV1),
    /// A plain-text value.
    Text(PlainTextV1),
    /// A contact mailbox.
    Mailbox(MailboxV1),
}

impl PropertyValueV1 {
    /// Wraps one validated location fact.
    #[must_use]
    pub fn location(fact: LocationFactV1) -> Self {
        Self::Location(Box::new(fact))
    }

    /// Wraps one recognized extension request.
    #[must_use]
    pub fn location_extension(request: LocationExtensionRequestV1) -> Self {
        Self::LocationExtension(Box::new(request))
    }

    /// Wraps one validated graphic reference.
    #[must_use]
    pub fn graphic(reference: AssetReferenceV1) -> Self {
        Self::Graphic(reference)
    }

    /// Wraps one validated plain text.
    #[must_use]
    pub fn text(text: PlainTextV1) -> Self {
        Self::Text(text)
    }

    /// Wraps one validated mailbox.
    #[must_use]
    pub fn mailbox(mailbox: MailboxV1) -> Self {
        Self::Mailbox(mailbox)
    }

    /// Whether this value is a §6.2H.4 `extend` operation rather than an `add` fact.
    ///
    /// An extension is an operation, never an independently effective value: it updates one
    /// inherited fact in place, so it never appears as its own fact, its own exposure or its
    /// own position in any projection.
    #[must_use]
    pub fn is_location_extension(&self) -> bool {
        matches!(self, Self::LocationExtension(_))
    }

    /// The exact asset reference, when this value is a graphic selection.
    #[must_use]
    pub fn asset_reference(&self) -> Option<&AssetReferenceV1> {
        match self {
            Self::Graphic(reference) => Some(reference),
            _ => None,
        }
    }

    /// The location fact, when this value is one.
    #[must_use]
    pub fn location_fact(&self) -> Option<&LocationFactV1> {
        match self {
            Self::Location(fact) => Some(fact),
            _ => None,
        }
    }
}

/// One typed value together with its explicit exposure.
#[derive(Debug, Clone, PartialEq)]
pub struct AuthoredPropertyValueV1 {
    value: PropertyValueV1,
    exposure: Exposure,
}

impl AuthoredPropertyValueV1 {
    /// Pairs a validated value with its authored exposure.
    ///
    /// # Errors
    ///
    /// Returns [`PropertyRefusalKind::ExtensionExposureForbidden`] when an `extend` operation is
    /// paired with `public`. Section 6.2H.6 defines no authored extension exposure, so this
    /// validated boundary makes a public extension wrapper unrepresentable instead of letting a
    /// placeholder decide exposure later.
    pub fn new(
        value: PropertyValueV1,
        exposure: Exposure,
    ) -> Result<Self, PropertyDeclarationRefusalV1> {
        if value.is_location_extension() && exposure.is_public() {
            return Err(PropertyDeclarationRefusalV1::of(
                PropertyRefusalKind::ExtensionExposureForbidden,
            ));
        }
        Ok(Self { value, exposure })
    }

    /// The validated typed value.
    #[must_use]
    pub const fn value(&self) -> &PropertyValueV1 {
        &self.value
    }

    /// The authored exposure.
    #[must_use]
    pub const fn exposure(&self) -> Exposure {
        self.exposure
    }
}

/// The closed `currentContext` of one declaration (§6.2G.2).
///
/// The literal string `"none"` and an absent contribution are distinct authored states
/// and stay distinct here: [`CurrentContextV1::None`] means "explicitly nothing", while a
/// whole absent property key means "no declaration at all".
#[derive(Debug, Clone, PartialEq)]
pub enum CurrentContextV1 {
    /// The literal string `"none"`: no local contribution.
    None,
    /// One or more local contributions in authored order.
    Values(Vec<AuthoredPropertyValueV1>),
}

impl CurrentContextV1 {
    /// The local contributions, or an empty slice for `"none"`.
    #[must_use]
    pub fn values(&self) -> &[AuthoredPropertyValueV1] {
        match self {
            Self::None => &[],
            Self::Values(values) => values,
        }
    }

    /// Whether this context is the literal `"none"`.
    #[must_use]
    pub fn is_none_literal(&self) -> bool {
        matches!(self, Self::None)
    }
}

/// One closed property declaration envelope (§6.2G.2).
#[derive(Debug, Clone, PartialEq)]
pub struct PropertyDeclarationV1 {
    inheritance: Inheritance,
    current_context: CurrentContextV1,
}

impl PropertyDeclarationV1 {
    /// Builds one declaration from an already validated mode and context.
    #[must_use]
    pub const fn new(inheritance: Inheritance, current_context: CurrentContextV1) -> Self {
        Self {
            inheritance,
            current_context,
        }
    }

    /// The inheritance mode.
    #[must_use]
    pub const fn inheritance(&self) -> Inheritance {
        self.inheritance
    }

    /// The closed current context.
    #[must_use]
    pub const fn current_context(&self) -> &CurrentContextV1 {
        &self.current_context
    }
}

/// Checks that a typed value belongs to the declared key's closed shape.
fn validate_value_for_key(
    key: PropertyKey,
    value: &PropertyValueV1,
) -> Result<(), PropertyDeclarationRefusalV1> {
    match (key, value) {
        (
            PropertyKey::Location,
            PropertyValueV1::Location(_) | PropertyValueV1::LocationExtension(_),
        ) => Ok(()),
        (PropertyKey::SeoTitle, PropertyValueV1::Text(text)) => {
            if text.scalar_len() <= MAX_SEO_TITLE_SCALARS {
                Ok(())
            } else {
                Err(PropertyDeclarationRefusalV1::of(
                    PropertyRefusalKind::PlainTextTooLong,
                ))
            }
        }
        (PropertyKey::SeoDescription, PropertyValueV1::Text(text)) => {
            if text.scalar_len() <= MAX_SEO_DESCRIPTION_SCALARS {
                Ok(())
            } else {
                Err(PropertyDeclarationRefusalV1::of(
                    PropertyRefusalKind::PlainTextTooLong,
                ))
            }
        }
        (PropertyKey::ContactEmail, PropertyValueV1::Mailbox(_)) => Ok(()),
        (declared, PropertyValueV1::Graphic(reference)) if declared.is_graphic() => {
            if declared == PropertyKey::GraphicFavicon && !reference.is_png() {
                Err(PropertyDeclarationRefusalV1::of(
                    PropertyRefusalKind::FaviconNotPng,
                ))
            } else {
                Ok(())
            }
        }
        _ => Err(PropertyDeclarationRefusalV1::of(
            PropertyRefusalKind::WrongEntryShape,
        )),
    }
}

/// A complete qualified EFS owner: the identity that owns a declaration.
///
/// The value can only be built through the five constructors, so a Facet without an
/// Entity, a Site without a Facet, a Page without a Site or a locale without a Page
/// cannot be expressed. A locale is a representation membership key, never geography.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QualifiedOwnerV1 {
    entity: EntityIdV1,
    facet: Option<FacetIdV1>,
    site: Option<SiteIdV1>,
    page: Option<PageIdV1>,
    locale: Option<LocaleKeyV1>,
}

impl QualifiedOwnerV1 {
    /// The Entity owner.
    #[must_use]
    pub fn entity(entity: EntityIdV1) -> Self {
        Self {
            entity,
            facet: None,
            site: None,
            page: None,
            locale: None,
        }
    }

    /// One Facet inside the Entity.
    #[must_use]
    pub fn facet(entity: EntityIdV1, facet: FacetIdV1) -> Self {
        Self {
            entity,
            facet: Some(facet),
            site: None,
            page: None,
            locale: None,
        }
    }

    /// One Site inside that Facet.
    #[must_use]
    pub fn site(entity: EntityIdV1, facet: FacetIdV1, site: SiteIdV1) -> Self {
        Self {
            entity,
            facet: Some(facet),
            site: Some(site),
            page: None,
            locale: None,
        }
    }

    /// One Page inside that Site.
    #[must_use]
    pub fn page(entity: EntityIdV1, facet: FacetIdV1, site: SiteIdV1, page: PageIdV1) -> Self {
        Self {
            entity,
            facet: Some(facet),
            site: Some(site),
            page: Some(page),
            locale: None,
        }
    }

    /// One localized representation of that Page.
    #[must_use]
    pub fn representation(
        entity: EntityIdV1,
        facet: FacetIdV1,
        site: SiteIdV1,
        page: PageIdV1,
        locale: LocaleKeyV1,
    ) -> Self {
        Self {
            entity,
            facet: Some(facet),
            site: Some(site),
            page: Some(page),
            locale: Some(locale),
        }
    }
}

impl QualifiedOwnerV1 {
    /// The owning Entity identifier.
    #[must_use]
    pub const fn entity_id(&self) -> &EntityIdV1 {
        &self.entity
    }

    /// The Facet identifier, when this owner is below the Entity.
    #[must_use]
    pub const fn facet_id(&self) -> Option<&FacetIdV1> {
        self.facet.as_ref()
    }

    /// The Site identifier, when this owner is below the Facet.
    #[must_use]
    pub const fn site_id(&self) -> Option<&SiteIdV1> {
        self.site.as_ref()
    }

    /// The Page identifier, when this owner is below the Site.
    #[must_use]
    pub const fn page_id(&self) -> Option<&PageIdV1> {
        self.page.as_ref()
    }

    /// The representation locale key, when this owner is a representation.
    #[must_use]
    pub const fn locale(&self) -> Option<&LocaleKeyV1> {
        self.locale.as_ref()
    }

    /// The EFS level of this owner.
    #[must_use]
    pub fn level(&self) -> EfsLevel {
        if self.locale.is_some() {
            EfsLevel::Representation
        } else if self.page.is_some() {
            EfsLevel::Page
        } else if self.site.is_some() {
            EfsLevel::Site
        } else if self.facet.is_some() {
            EfsLevel::Facet
        } else {
            EfsLevel::Entity
        }
    }

    /// The zero-based depth of this owner in the EFS level order.
    #[must_use]
    pub const fn level_index(&self) -> usize {
        if self.locale.is_some() {
            4
        } else if self.page.is_some() {
            3
        } else if self.site.is_some() {
            2
        } else if self.facet.is_some() {
            1
        } else {
            0
        }
    }

    /// Every owner on this direct chain, root first, ending with this owner.
    #[must_use]
    pub fn chain(&self) -> Vec<Self> {
        let mut chain = vec![Self::entity(self.entity.clone())];
        if let Some(facet) = &self.facet {
            chain.push(Self::facet(self.entity.clone(), facet.clone()));
            if let Some(site) = &self.site {
                chain.push(Self::site(self.entity.clone(), facet.clone(), site.clone()));
                if let Some(page) = &self.page {
                    chain.push(Self::page(
                        self.entity.clone(),
                        facet.clone(),
                        site.clone(),
                        page.clone(),
                    ));
                    if let Some(locale) = &self.locale {
                        chain.push(Self::representation(
                            self.entity.clone(),
                            facet.clone(),
                            site.clone(),
                            page.clone(),
                            locale.clone(),
                        ));
                    }
                }
            }
        }
        chain
    }

    /// Whether this owner is a strict ancestor of `other`.
    ///
    /// Only direct containment counts: another branch of the same Entity is never an
    /// ancestor, and an owner is never its own ancestor.
    #[must_use]
    pub fn is_ancestor_of(&self, other: &Self) -> bool {
        if self == other {
            return false;
        }
        other
            .chain()
            .get(self.level_index())
            .is_some_and(|candidate| candidate == self)
    }

    /// A stable structural description of this owner.
    ///
    /// Only validated EFS identifiers and a canonical locale key appear here, so the text
    /// is safe for a diagnostic and carries no property content.
    #[must_use]
    pub fn describe(&self) -> String {
        let mut text = format!("entity:{}", self.entity.as_str());
        if let Some(facet) = &self.facet {
            text.push_str("/facet:");
            text.push_str(facet.as_str());
        }
        if let Some(site) = &self.site {
            text.push_str("/site:");
            text.push_str(site.as_str());
        }
        if let Some(page) = &self.page {
            text.push_str("/page:");
            text.push_str(page.as_str());
        }
        if let Some(locale) = &self.locale {
            text.push_str("/representation:");
            text.push_str(locale.as_str());
        }
        text
    }
}

/// The validated property declarations of one qualified EFS owner.
///
/// Validation is complete at construction: every key must be registered, allowed at this
/// owner's level, unique within the entry list, typed for its own key and within the local
/// entry bound. A wrong-level `seo.title` on the Entity is refused here, not silently
/// ignored later.
#[derive(Debug, Clone, PartialEq)]
pub struct DeclaredOwnerPropertiesV1 {
    owner: QualifiedOwnerV1,
    entries: Vec<(PropertyKey, PropertyDeclarationV1)>,
}

impl DeclaredOwnerPropertiesV1 {
    /// Validates one owner's declarations.
    ///
    /// # Errors
    ///
    /// Returns a [`PropertyDeclarationRefusalV1`] naming the first refused entry:
    /// [`PropertyRefusalKind::WrongLevel`],
    /// [`PropertyRefusalKind::DuplicatePropertyEntry`],
    /// [`PropertyRefusalKind::TooManyLocalEntries`] or the value's own refusal.
    pub fn new(
        owner: QualifiedOwnerV1,
        entries: Vec<(PropertyKey, PropertyDeclarationV1)>,
    ) -> Result<Self, PropertyDeclarationRefusalV1> {
        for (index, (key, declaration)) in entries.iter().enumerate() {
            if !key.allows_level(owner.level()) {
                return Err(
                    PropertyDeclarationRefusalV1::of(PropertyRefusalKind::WrongLevel)
                        .with_key(*key),
                );
            }
            if entries[..index].iter().any(|(seen, _)| seen == key) {
                return Err(PropertyDeclarationRefusalV1::of(
                    PropertyRefusalKind::DuplicatePropertyEntry,
                )
                .with_key(*key));
            }
            // An explicitly empty array is invalid authored input, never an implicit "none":
            // the typed model is reachable outside the JSON adapter, so the refusal belongs
            // here and not only in the translation layer.
            if let CurrentContextV1::Values(values) = declaration.current_context()
                && values.is_empty()
            {
                return Err(PropertyDeclarationRefusalV1::of(
                    PropertyRefusalKind::EmptyContextArray,
                )
                .with_key(*key));
            }
            let values = declaration.current_context().values();
            if values.len() > MAX_LOCAL_ENTRIES {
                return Err(PropertyDeclarationRefusalV1::of(
                    PropertyRefusalKind::TooManyLocalEntries,
                )
                .with_key(*key));
            }
            for value in values {
                validate_value_for_key(*key, value.value())
                    .map_err(|refusal| refusal.with_key(*key))?;
            }
        }
        Ok(Self { owner, entries })
    }

    /// The qualified owner these declarations belong to.
    #[must_use]
    pub const fn owner(&self) -> &QualifiedOwnerV1 {
        &self.owner
    }

    /// The declared entries in authored order.
    #[must_use]
    pub fn entries(&self) -> &[(PropertyKey, PropertyDeclarationV1)] {
        &self.entries
    }

    /// One declared key, when this owner declares it.
    #[must_use]
    pub fn declaration(&self, key: PropertyKey) -> Option<&PropertyDeclarationV1> {
        self.entries
            .iter()
            .find(|(declared, _)| *declared == key)
            .map(|(_, declaration)| declaration)
    }
}
