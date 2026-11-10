//! Strict, duplicate-aware parsing of already-supplied publication-manifest bytes.
//!
//! This is the **inert syntactic** half of the single-authority manifest. It accepts
//! bytes a caller already holds, refuses anything that is not strict RFC 8259 JSON in
//! the accepted closed shape, and returns a [`ParsedManifestCandidateV1`]. It performs
//! no I/O: it never walks a content tree, never opens a source, never resolves a
//! symlink and never selects a body.
//!
//! **A parsed candidate is never publication authority.** The output type is named
//! *candidate* deliberately and is neither `Published`, `PublicationReady` nor
//! `ValidatedPublication`. What this module proves is a closed **syntactic** shape only:
//!
//! * *Not proven* — canonical public URL, route and base conformance (§6.2J).
//! * *Not proven* — BCP47, full-region locale validity and Unicode/NFC normalization.
//! * *Not proven* — property-registry, module or `contentVersion` semantics; authored
//!   `properties` stay inert, bounded data.
//! * *Not proven* — source existence, confinement, snapshot atomicity, absence-versus-
//!   error semantics and effective JSON/Markdown selection (§6.2C.3–§6.2C.4).
//! * *Not proven* — visibility, fallback and entry-path policy outcomes.
//!
//! Those belong to later, separately authorized gates; no value produced here may be
//! reported as a validated or publishable manifest.

use core::fmt;

use serde::de::{Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};

use serde_json::Number;

use solo_domain::{
    EntityIdV1, FacetIdV1, MAX_EFS_NODES, MAX_PAGES, MAX_REPRESENTATIONS_PER_PAGE, PageIdV1,
    SiteIdV1, SourceCandidateDraftV1, SourceCandidateKind, SourceCandidatePathV1,
    SourceCandidateReservationsV1, SourceCandidateSetV1,
};

/// The sole logical manifest identity (§6.2A.1).
pub const MANIFEST_LOGICAL_IDENTITY: &str = "content/publication.json";

/// Maximum accepted manifest size in bytes (§6.2A.1).
pub const MAX_MANIFEST_BYTES: usize = 2 * 1024 * 1024;

/// The only accepted value of every declared contract version in this slice (§6.2A.6).
pub const SUPPORTED_VERSION: &str = "1.0.0";

/// Maximum accepted `base.aliases` entries (§6.2J.1).
pub const MAX_BASE_ALIASES: usize = 8;

/// Maximum accepted entries in one inert `properties` object (§6.2A.7).
pub const MAX_PROPERTY_ENTRIES: usize = 100;

/// Maximum accepted nesting depth of one inert `properties` value (§6.2A.7).
pub const MAX_PROPERTY_DEPTH: usize = 12;

/// The stable token used to carry a duplicate-member refusal out of the visitor.
///
/// Using a marker keeps the refusal unambiguous while still refusing **during** map
/// access, so no derived map is ever built that could let a later member win.
const DUPLICATE_KEY_TOKEN: &str = "solo-manifest-duplicate-key";

/// The five — and only five — root keys (§6.2B).
pub const ROOT_KEYS: [&str; 5] = [
    "schemaVersion",
    "propertyRegistryVersion",
    "platformAssetLibraryVersion",
    "base",
    "entity",
];

/// Why manifest bytes were refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ManifestErrorKind {
    /// The input was empty.
    Empty,
    /// The input exceeded [`MAX_MANIFEST_BYTES`].
    TooLarge,
    /// The input began with a UTF-8 byte-order mark.
    ByteOrderMark,
    /// The input was not valid UTF-8.
    InvalidUtf8,
    /// The same object member appeared twice at some depth.
    DuplicateKey,
    /// The bytes were not strict JSON, or carried trailing content.
    MalformedJson,
    /// The JSON root was not an object.
    NotAnObject,
    /// A mandatory key was absent.
    MissingKey,
    /// A key outside the closed shape was present.
    UnknownKey,
    /// A value had the wrong JSON type.
    WrongType,
    /// A declared version was not the supported exact value.
    UnsupportedVersion,
    /// A declared EFS identifier was invalid.
    InvalidIdentity,
    /// A scoped identifier was repeated under one parent.
    DuplicateScopedId,
    /// Two representations of one Page declared the same locale.
    DuplicateLocale,
    /// A collection that must be non-empty was empty.
    EmptyCollection,
    /// A declared resource budget was exceeded.
    BudgetExceeded,
    /// An enumerated value was not one of its accepted literals.
    InvalidEnumValue,
    /// A `sourceCandidates` member was declared `null`.
    NullCandidate,
    /// A source locator was refused by the portable-path rule.
    InvalidSourceLocator,
    /// A `sourceCandidates` object declared no member.
    EmptyCandidateSet,
    /// A duplicate member was declared inside `sourceCandidates`.
    DuplicateCandidateMember,
    /// An inert `properties` value exceeded an accepted bound.
    PropertiesOutOfBounds,
    /// A qualified target carried a partial lineage.
    IncompleteTarget,
    /// Two declared representations reserved the same logical candidate path.
    DuplicateSourceClaim,
}

impl ManifestErrorKind {
    /// Stable, safe diagnostic token for this refusal reason.
    ///
    /// Tokens are fixed ASCII words, so no authored value can leak through them.
    #[must_use]
    pub const fn detail(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::TooLarge => "too-large",
            Self::ByteOrderMark => "byte-order-mark",
            Self::InvalidUtf8 => "invalid-utf8",
            Self::DuplicateKey => "duplicate-key",
            Self::MalformedJson => "malformed-json",
            Self::NotAnObject => "not-an-object",
            Self::MissingKey => "missing-key",
            Self::UnknownKey => "unknown-key",
            Self::WrongType => "wrong-type",
            Self::UnsupportedVersion => "unsupported-version",
            Self::InvalidIdentity => "invalid-identity",
            Self::DuplicateScopedId => "duplicate-scoped-id",
            Self::DuplicateLocale => "duplicate-locale",
            Self::EmptyCollection => "empty-collection",
            Self::BudgetExceeded => "budget-exceeded",
            Self::InvalidEnumValue => "invalid-enum-value",
            Self::NullCandidate => "null-candidate",
            Self::InvalidSourceLocator => "invalid-source-locator",
            Self::EmptyCandidateSet => "empty-candidate-set",
            Self::DuplicateCandidateMember => "duplicate-candidate-member",
            Self::PropertiesOutOfBounds => "properties-out-of-bounds",
            Self::IncompleteTarget => "incomplete-target",
            Self::DuplicateSourceClaim => "duplicate-source-claim",
        }
    }
}

/// Refusal of manifest bytes.
///
/// A refusal carries a stable kind and the safe dotted location of the refused member.
/// The location is built only from fixed key words and decimal indices this module chose
/// itself, so refused authored text is never echoed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestError {
    kind: ManifestErrorKind,
    location: String,
}

impl ManifestError {
    fn new(kind: ManifestErrorKind, location: impl Into<String>) -> Self {
        Self {
            kind,
            location: location.into(),
        }
    }

    /// The refusal reason.
    #[must_use]
    pub const fn kind(&self) -> ManifestErrorKind {
        self.kind
    }

    /// The safe dotted location of the refused member, e.g. `entity.facets[2].id`.
    #[must_use]
    pub fn location(&self) -> &str {
        &self.location
    }

    /// Stable engineering code for this refusal.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        "PREFLIGHT_MANIFEST_INVALID"
    }

    /// Stable, safe diagnostic token for the refusal reason.
    #[must_use]
    pub const fn detail(&self) -> &'static str {
        self.kind.detail()
    }
}

impl fmt::Display for ManifestError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}: {} at {}",
            self.code(),
            self.detail(),
            self.location
        )
    }
}

impl std::error::Error for ManifestError {}

/// A JSON value as authored, retaining object members as **ordered pairs**.
///
/// Ordering is preserved and every member is kept, so authored order survives and a
/// duplicate can never be silently collapsed into a single winner.
#[derive(Debug, Clone, PartialEq)]
pub enum DeclaredNode {
    /// JSON `null`.
    Null,
    /// A boolean.
    Bool(bool),
    /// A finite number.
    Number(Number),
    /// A string.
    String(String),
    /// An array in authored order.
    Array(Vec<DeclaredNode>),
    /// An object as authored member pairs.
    Object(Vec<(String, DeclaredNode)>),
}

impl<'de> Deserialize<'de> for DeclaredNode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(DeclaredNodeVisitor)
    }
}

struct DeclaredNodeVisitor;

impl<'de> Visitor<'de> for DeclaredNodeVisitor {
    type Value = DeclaredNode;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a JSON value")
    }

    fn visit_bool<E: serde::de::Error>(self, value: bool) -> Result<Self::Value, E> {
        Ok(DeclaredNode::Bool(value))
    }

    fn visit_i64<E: serde::de::Error>(self, value: i64) -> Result<Self::Value, E> {
        Ok(DeclaredNode::Number(Number::from(value)))
    }

    fn visit_u64<E: serde::de::Error>(self, value: u64) -> Result<Self::Value, E> {
        Ok(DeclaredNode::Number(Number::from(value)))
    }

    fn visit_f64<E: serde::de::Error>(self, value: f64) -> Result<Self::Value, E> {
        // Non-finite forms are refused rather than repaired into a number.
        Number::from_f64(value)
            .map(DeclaredNode::Number)
            .ok_or_else(|| E::custom("solo-manifest-nonfinite-number"))
    }

    fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
        Ok(DeclaredNode::String(value.to_owned()))
    }

    fn visit_string<E: serde::de::Error>(self, value: String) -> Result<Self::Value, E> {
        Ok(DeclaredNode::String(value))
    }

    fn visit_none<E: serde::de::Error>(self) -> Result<Self::Value, E> {
        Ok(DeclaredNode::Null)
    }

    fn visit_unit<E: serde::de::Error>(self) -> Result<Self::Value, E> {
        Ok(DeclaredNode::Null)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        DeclaredNode::deserialize(deserializer)
    }

    fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut items: Vec<DeclaredNode> = Vec::new();
        while let Some(item) = sequence.next_element::<DeclaredNode>()? {
            items.push(item);
        }
        Ok(DeclaredNode::Array(items))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut members: Vec<(String, DeclaredNode)> = Vec::new();
        while let Some(key) = map.next_key::<String>()? {
            // Refusal happens here, during map access, before any map that could let a
            // second member win can exist.
            if members.iter().any(|(seen, _)| *seen == key) {
                return Err(serde::de::Error::custom(DUPLICATE_KEY_TOKEN));
            }
            let value = map.next_value::<DeclaredNode>()?;
            members.push((key, value));
        }
        Ok(DeclaredNode::Object(members))
    }
}

/// Parses already-supplied manifest bytes into an inert syntactic candidate.
///
/// # Errors
///
/// Returns a [`ManifestError`] for empty input, a byte-order mark, invalid UTF-8,
/// non-strict JSON, trailing content, a duplicate object member at any depth, a value
/// outside the closed shape, an unsupported version, or a duplicate/unsupported source
/// declaration. Nothing is normalised into validity.
pub fn parse_manifest_bytes(bytes: &[u8]) -> Result<ParsedManifestCandidateV1, ManifestError> {
    if bytes.is_empty() {
        return Err(ManifestError::new(ManifestErrorKind::Empty, "manifest"));
    }
    if bytes.len() > MAX_MANIFEST_BYTES {
        return Err(ManifestError::new(ManifestErrorKind::TooLarge, "manifest"));
    }
    if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        return Err(ManifestError::new(
            ManifestErrorKind::ByteOrderMark,
            "manifest",
        ));
    }
    let text = core::str::from_utf8(bytes)
        .map_err(|_| ManifestError::new(ManifestErrorKind::InvalidUtf8, "manifest"))?;

    // `from_str` refuses trailing content, comments, NaN/Infinity literals and every
    // non-JSON syntax, so no separate lenient path exists.
    let node: DeclaredNode = serde_json::from_str(text).map_err(|error| {
        let message = error.to_string();
        if message.contains(DUPLICATE_KEY_TOKEN) {
            ManifestError::new(ManifestErrorKind::DuplicateKey, "manifest")
        } else {
            ManifestError::new(ManifestErrorKind::MalformedJson, "manifest")
        }
    })?;

    build_candidate(&node)
}

fn build_candidate(node: &DeclaredNode) -> Result<ParsedManifestCandidateV1, ManifestError> {
    let root = as_object(node, "manifest")?;
    // Exactly the five accepted root keys and nothing else. Without this the root would
    // silently tolerate an extra member such as `draft`.
    closed_keys(root, &ROOT_KEYS, "manifest")?;

    let schema_version = version(root, "schemaVersion")?;
    let property_registry_version = version(root, "propertyRegistryVersion")?;
    let platform_asset_library_version = version(root, "platformAssetLibraryVersion")?;

    let base = build_base(member(root, "base", "manifest")?)?;
    let (entity, counts) = build_entity(member(root, "entity", "manifest")?)?;

    if counts.nodes > MAX_EFS_NODES {
        return Err(ManifestError::new(
            ManifestErrorKind::BudgetExceeded,
            "entity",
        ));
    }

    let candidate = ParsedManifestCandidateV1 {
        schema_version,
        property_registry_version,
        platform_asset_library_version,
        base,
        entity,
        node_count: counts.nodes,
        page_count: counts.pages,
        representation_count: counts.representations,
    };
    Ok(candidate)
}

/// A version string that must equal [`SUPPORTED_VERSION`] exactly (§6.2A.6).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclaredVersionV1(String);

impl DeclaredVersionV1 {
    /// The accepted exact version as authored.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DeclaredVersionV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

fn version(root: &[(String, DeclaredNode)], key: &str) -> Result<DeclaredVersionV1, ManifestError> {
    let value = member(root, key, "manifest")?;
    let text = as_string(value, key)?;
    if text != SUPPORTED_VERSION {
        return Err(ManifestError::new(
            ManifestErrorKind::UnsupportedVersion,
            key,
        ));
    }
    Ok(DeclaredVersionV1(text.to_owned()))
}

// ---------------------------------------------------------------------------
// Shape helpers. Every lookup is safe because duplicate members were already
// refused during deserialization, so a linear scan has exactly one match.
// ---------------------------------------------------------------------------

fn as_object<'a>(
    node: &'a DeclaredNode,
    location: &str,
) -> Result<&'a [(String, DeclaredNode)], ManifestError> {
    match node {
        DeclaredNode::Object(members) => Ok(members),
        _ => Err(ManifestError::new(ManifestErrorKind::NotAnObject, location)),
    }
}

fn as_string<'a>(node: &'a DeclaredNode, location: &str) -> Result<&'a str, ManifestError> {
    match node {
        DeclaredNode::String(text) => Ok(text),
        _ => Err(ManifestError::new(ManifestErrorKind::WrongType, location)),
    }
}

fn as_array<'a>(
    node: &'a DeclaredNode,
    location: &str,
) -> Result<&'a [DeclaredNode], ManifestError> {
    match node {
        DeclaredNode::Array(items) => Ok(items),
        _ => Err(ManifestError::new(ManifestErrorKind::WrongType, location)),
    }
}

fn member<'a>(
    members: &'a [(String, DeclaredNode)],
    key: &str,
    location: &str,
) -> Result<&'a DeclaredNode, ManifestError> {
    members
        .iter()
        .find(|(name, _)| name == key)
        .map(|(_, value)| value)
        .ok_or_else(|| {
            ManifestError::new(ManifestErrorKind::MissingKey, format!("{location}.{key}"))
        })
}

fn optional<'a>(members: &'a [(String, DeclaredNode)], key: &str) -> Option<&'a DeclaredNode> {
    members
        .iter()
        .find(|(name, _)| name == key)
        .map(|(_, value)| value)
}

fn closed_keys(
    members: &[(String, DeclaredNode)],
    allowed: &[&str],
    location: &str,
) -> Result<(), ManifestError> {
    for (name, _) in members {
        if !allowed.contains(&name.as_str()) {
            // `parentId`, ancestor pointers and type discriminators are all refused here
            // because they are not enumerated in the accepted shape.
            return Err(ManifestError::new(
                ManifestErrorKind::UnknownKey,
                format!("{location}.{name}"),
            ));
        }
    }
    Ok(())
}

/// The declared `base` entry, checked as a **closed syntactic shape**.
///
/// The `http:`/`https:` prefix test here is a shape check only. Full §6.2J.1 canonical
/// URL conformance — IDNA host folding, default-port normalisation, prefix segment rules
/// and alias collision resolution — is **not proven** by this type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaseDeclarationV1 {
    canonical: String,
    aliases: Vec<String>,
    landing: Option<QualifiedTargetV1>,
}

impl BaseDeclarationV1 {
    /// The declared canonical base as authored.
    #[must_use]
    pub fn canonical(&self) -> &str {
        &self.canonical
    }

    /// The declared alias bases in authored order.
    #[must_use]
    pub fn aliases(&self) -> &[String] {
        &self.aliases
    }

    /// The declared landing target, if any.
    #[must_use]
    pub fn landing(&self) -> Option<&QualifiedTargetV1> {
        self.landing.as_ref()
    }
}

/// A fully qualified EFS target (§6.2B).
///
/// All four lineage components are required; a shorthand or partial target is refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualifiedTargetV1 {
    entity: String,
    facet: String,
    site: String,
    page: String,
    locale: Option<String>,
}

impl QualifiedTargetV1 {
    /// The declared Entity component.
    #[must_use]
    pub fn entity(&self) -> &str {
        &self.entity
    }

    /// The declared Facet component.
    #[must_use]
    pub fn facet(&self) -> &str {
        &self.facet
    }

    /// The declared Site component.
    #[must_use]
    pub fn site(&self) -> &str {
        &self.site
    }

    /// The declared Page component.
    #[must_use]
    pub fn page(&self) -> &str {
        &self.page
    }

    /// The declared full locale, when this target names a representation.
    ///
    /// The value is recorded as authored and is **not** proven to be a canonical, full
    /// region-qualified BCP47 tag.
    #[must_use]
    pub fn locale(&self) -> Option<&str> {
        self.locale.as_deref()
    }
}

/// Declared visibility of a Page or representation (§6.2B).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclaredVisibility {
    /// Publicly discoverable.
    Public,
    /// Reachable only by its exact authorized URL, and never discovered.
    Unlisted,
}

impl DeclaredVisibility {
    /// The authored literal.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Public => "public",
            Self::Unlisted => "unlisted",
        }
    }
}

/// Declared route-claim case policy (§6.2J).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclaredCaseMatching {
    /// Matches case-insensitively with a lowercase canonical spelling.
    Insensitive,
    /// Matches exactly and preserves the authored capitalisation.
    Sensitive,
}

impl DeclaredCaseMatching {
    /// The authored literal.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Insensitive => "insensitive",
            Self::Sensitive => "sensitive",
        }
    }
}

/// Declared missing-translation policy (§6.2B).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeclaredMissingTranslation {
    /// Fall back to the Site default representation of the same Page.
    Default,
    /// Report not-found.
    Strict,
    /// Fall back to one declared full locale of the same Page.
    Locale(String),
}

fn build_base(node: &DeclaredNode) -> Result<BaseDeclarationV1, ManifestError> {
    let members = as_object(node, "base")?;
    closed_keys(members, &["canonical", "aliases", "landing"], "base")?;

    let canonical = as_string(member(members, "canonical", "base")?, "base.canonical")?;
    require_http_prefix(canonical, "base.canonical")?;

    let mut aliases: Vec<String> = Vec::new();
    if let Some(value) = optional(members, "aliases") {
        let items = as_array(value, "base.aliases")?;
        if items.len() > MAX_BASE_ALIASES {
            return Err(ManifestError::new(
                ManifestErrorKind::BudgetExceeded,
                "base.aliases",
            ));
        }
        for (index, item) in items.iter().enumerate() {
            let location = format!("base.aliases[{index}]");
            let text = as_string(item, &location)?;
            require_http_prefix(text, &location)?;
            if text == canonical || aliases.iter().any(|seen| seen == text) {
                return Err(ManifestError::new(
                    ManifestErrorKind::DuplicateScopedId,
                    location,
                ));
            }
            aliases.push(text.to_owned());
        }
    }

    let landing = match optional(members, "landing") {
        Some(value) => Some(build_target(value, "base.landing")?),
        None => None,
    };

    Ok(BaseDeclarationV1 {
        canonical: canonical.to_owned(),
        aliases,
        landing,
    })
}

/// A shape check only: the value must begin with an explicit `http://` or `https://`.
fn require_http_prefix(value: &str, location: &str) -> Result<(), ManifestError> {
    let rest = value
        .strip_prefix("https://")
        .or_else(|| value.strip_prefix("http://"));
    match rest {
        Some(after) if !after.is_empty() => Ok(()),
        _ => Err(ManifestError::new(ManifestErrorKind::WrongType, location)),
    }
}

fn build_target(node: &DeclaredNode, location: &str) -> Result<QualifiedTargetV1, ManifestError> {
    let members = as_object(node, location)?;
    closed_keys(
        members,
        &["entity", "facet", "site", "page", "locale"],
        location,
    )?;
    let read = |key: &str| -> Result<String, ManifestError> {
        let value = member(members, key, location)?;
        Ok(as_string(value, &format!("{location}.{key}"))?.to_owned())
    };
    let entity = read("entity")?;
    let facet = read("facet")?;
    let site = read("site")?;
    let page = read("page")?;
    if entity.is_empty() || facet.is_empty() || site.is_empty() || page.is_empty() {
        return Err(ManifestError::new(
            ManifestErrorKind::IncompleteTarget,
            location,
        ));
    }
    let locale = match optional(members, "locale") {
        Some(value) => Some(as_string(value, &format!("{location}.locale"))?.to_owned()),
        None => None,
    };
    Ok(QualifiedTargetV1 {
        entity,
        facet,
        site,
        page,
        locale,
    })
}

/// A validated, inert `properties` object.
///
/// The entries are kept only so a future property-registry gate can inspect them. No
/// property value is validated, promoted or made public here (§6.2B, §6.2A.7).
#[derive(Debug, Clone, PartialEq)]
pub struct InertPropertiesV1 {
    entries: Vec<(String, DeclaredNode)>,
}

impl InertPropertiesV1 {
    /// How many inert entries were declared.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether nothing was declared.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// The declared entry names, in authored order.
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.entries.iter().map(|(name, _)| name.as_str())
    }

    /// The declared entries as `(name, authored value)` pairs, in authored order.
    ///
    /// The accessor is read-only: it exposes neither a mutable entry list nor a way to
    /// change this inert candidate, so a translator can inspect the authored property input
    /// without gaining any publication authority. Each value is still the raw
    /// [`DeclaredNode`] tree; only the property-registry translation validates it.
    pub fn entries(&self) -> impl Iterator<Item = (&str, &DeclaredNode)> {
        self.entries
            .iter()
            .map(|(name, value)| (name.as_str(), value))
    }
}

fn build_properties(
    node: &DeclaredNode,
    location: &str,
) -> Result<InertPropertiesV1, ManifestError> {
    let members = as_object(node, location)?;
    if members.len() > MAX_PROPERTY_ENTRIES {
        return Err(ManifestError::new(
            ManifestErrorKind::PropertiesOutOfBounds,
            location,
        ));
    }
    for (name, value) in members {
        if depth_of(value) > MAX_PROPERTY_DEPTH {
            return Err(ManifestError::new(
                ManifestErrorKind::PropertiesOutOfBounds,
                format!("{location}.{name}"),
            ));
        }
    }
    Ok(InertPropertiesV1 {
        entries: members.to_vec(),
    })
}

fn depth_of(node: &DeclaredNode) -> usize {
    match node {
        DeclaredNode::Array(items) => 1 + items.iter().map(depth_of).max().unwrap_or(0),
        DeclaredNode::Object(members) => {
            1 + members
                .iter()
                .map(|(_, value)| depth_of(value))
                .max()
                .unwrap_or(0)
        }
        _ => 1,
    }
}

fn optional_properties(
    members: &[(String, DeclaredNode)],
    location: &str,
) -> Result<Option<InertPropertiesV1>, ManifestError> {
    match optional(members, "properties") {
        Some(value) => Ok(Some(build_properties(
            value,
            &format!("{location}.properties"),
        )?)),
        None => Ok(None),
    }
}

/// The declared Entity node of the nested EFS shape (§6.2B).
#[derive(Debug, Clone, PartialEq)]
pub struct EntityDeclarationV1 {
    id: EntityIdV1,
    label: String,
    facets: Vec<FacetDeclarationV1>,
    properties: Option<InertPropertiesV1>,
}

impl EntityDeclarationV1 {
    /// The validated Entity identifier.
    #[must_use]
    pub fn id(&self) -> &EntityIdV1 {
        &self.id
    }

    /// The declared display label.
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// The declared Facets in authored order.
    #[must_use]
    pub fn facets(&self) -> &[FacetDeclarationV1] {
        &self.facets
    }

    /// The inert declared properties, if any.
    #[must_use]
    pub fn properties(&self) -> Option<&InertPropertiesV1> {
        self.properties.as_ref()
    }
}

/// The declared Facet node.
#[derive(Debug, Clone, PartialEq)]
pub struct FacetDeclarationV1 {
    id: FacetIdV1,
    label: String,
    sites: Vec<SiteDeclarationV1>,
    properties: Option<InertPropertiesV1>,
}

impl FacetDeclarationV1 {
    /// The validated Facet identifier, unique within its Entity.
    #[must_use]
    pub fn id(&self) -> &FacetIdV1 {
        &self.id
    }

    /// The declared display label.
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// The declared Sites in authored order.
    #[must_use]
    pub fn sites(&self) -> &[SiteDeclarationV1] {
        &self.sites
    }

    /// The inert declared properties, if any.
    #[must_use]
    pub fn properties(&self) -> Option<&InertPropertiesV1> {
        self.properties.as_ref()
    }
}

/// The declared Site node.
#[derive(Debug, Clone, PartialEq)]
pub struct SiteDeclarationV1 {
    id: SiteIdV1,
    label: String,
    default_locale: String,
    language_preferences: Vec<(String, String)>,
    pages: Vec<PageDeclarationV1>,
    properties: Option<InertPropertiesV1>,
}

impl SiteDeclarationV1 {
    /// The validated Site identifier, unique within its Facet.
    #[must_use]
    pub fn id(&self) -> &SiteIdV1 {
        &self.id
    }

    /// The declared display label.
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// The declared default locale, recorded as authored.
    ///
    /// **Not proven** to be a canonical full region-qualified BCP47 tag.
    #[must_use]
    pub fn default_locale(&self) -> &str {
        &self.default_locale
    }

    /// The declared request-preference to authored-locale mappings, in authored order.
    #[must_use]
    pub fn language_preferences(&self) -> &[(String, String)] {
        &self.language_preferences
    }

    /// The declared Pages in authored order.
    #[must_use]
    pub fn pages(&self) -> &[PageDeclarationV1] {
        &self.pages
    }

    /// The inert declared properties, if any.
    #[must_use]
    pub fn properties(&self) -> Option<&InertPropertiesV1> {
        self.properties.as_ref()
    }
}

/// The declared Page node.
#[derive(Debug, Clone, PartialEq)]
pub struct PageDeclarationV1 {
    id: PageIdV1,
    label: String,
    visibility_default: DeclaredVisibility,
    entry_paths: Vec<String>,
    missing_translation: DeclaredMissingTranslation,
    representations: Vec<RepresentationDeclarationV1>,
    properties: Option<InertPropertiesV1>,
}

impl PageDeclarationV1 {
    /// The validated Page identifier, unique within its Site.
    #[must_use]
    pub fn id(&self) -> &PageIdV1 {
        &self.id
    }

    /// The declared display label.
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// The declared default visibility.
    #[must_use]
    pub const fn visibility_default(&self) -> DeclaredVisibility {
        self.visibility_default
    }

    /// The declared locale-less entry paths, in authored order.
    #[must_use]
    pub fn entry_paths(&self) -> &[String] {
        &self.entry_paths
    }

    /// The declared missing-translation policy.
    #[must_use]
    pub fn missing_translation(&self) -> &DeclaredMissingTranslation {
        &self.missing_translation
    }

    /// The declared representations in authored order.
    #[must_use]
    pub fn representations(&self) -> &[RepresentationDeclarationV1] {
        &self.representations
    }

    /// The inert declared properties, if any.
    #[must_use]
    pub fn properties(&self) -> Option<&InertPropertiesV1> {
        self.properties.as_ref()
    }
}

/// The declared representation leaf.
#[derive(Debug, Clone, PartialEq)]
pub struct RepresentationDeclarationV1 {
    locale: String,
    route: RouteDeclarationV1,
    source_candidates: SourceCandidateSetV1,
    visibility: Option<DeclaredVisibility>,
    properties: Option<InertPropertiesV1>,
}

impl RepresentationDeclarationV1 {
    /// The declared representation locale, unique within its Page.
    ///
    /// Recorded as authored and deliberately **not** converted into a canonical locale
    /// key: it is unvalidated input, never a verified locale.
    #[must_use]
    pub fn locale(&self) -> &str {
        &self.locale
    }

    /// The declared route claim.
    #[must_use]
    pub fn route(&self) -> &RouteDeclarationV1 {
        &self.route
    }

    /// The validated `sourceCandidates` declaration.
    #[must_use]
    pub fn source_candidates(&self) -> &SourceCandidateSetV1 {
        &self.source_candidates
    }

    /// The declared override of the Page visibility default.
    #[must_use]
    pub const fn visibility(&self) -> Option<DeclaredVisibility> {
        self.visibility
    }

    /// The inert declared properties, if any.
    #[must_use]
    pub fn properties(&self) -> Option<&InertPropertiesV1> {
        self.properties.as_ref()
    }
}

/// The declared route claim of one representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouteDeclarationV1 {
    path: String,
    case_matching: Option<DeclaredCaseMatching>,
    aliases: Vec<RouteAliasV1>,
}

impl RouteDeclarationV1 {
    /// The declared canonical path, recorded as authored.
    ///
    /// **Not proven** against the §6.2J route profile, locale prefixes or case policy.
    #[must_use]
    pub fn path(&self) -> &str {
        &self.path
    }

    /// The declared case policy, when authored.
    #[must_use]
    pub const fn case_matching(&self) -> Option<DeclaredCaseMatching> {
        self.case_matching
    }

    /// The declared aliases in authored order.
    #[must_use]
    pub fn aliases(&self) -> &[RouteAliasV1] {
        &self.aliases
    }
}

/// One declared route alias.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouteAliasV1 {
    path: String,
    case_matching: Option<DeclaredCaseMatching>,
}

impl RouteAliasV1 {
    /// The declared alias path, recorded as authored.
    #[must_use]
    pub fn path(&self) -> &str {
        &self.path
    }

    /// The declared case policy of the alias.
    #[must_use]
    pub const fn case_matching(&self) -> Option<DeclaredCaseMatching> {
        self.case_matching
    }
}

/// The result of a successful syntactic parse: an **inert candidate**.
///
/// This type is not a publication decision. It carries declared authored values that
/// have passed the closed syntactic shape and the accepted budgets. It carries no
/// resolved source, no validated content, no public address and no publication
/// authority, and no method on it can produce one.
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedManifestCandidateV1 {
    schema_version: DeclaredVersionV1,
    property_registry_version: DeclaredVersionV1,
    platform_asset_library_version: DeclaredVersionV1,
    base: BaseDeclarationV1,
    entity: EntityDeclarationV1,
    node_count: usize,
    page_count: usize,
    representation_count: usize,
}

impl ParsedManifestCandidateV1 {
    /// The sole logical identity these bytes claim: always `content/publication.json`.
    #[must_use]
    pub const fn logical_identity(&self) -> &'static str {
        MANIFEST_LOGICAL_IDENTITY
    }

    /// The declared manifest schema version.
    #[must_use]
    pub fn schema_version(&self) -> &DeclaredVersionV1 {
        &self.schema_version
    }

    /// The declared property-registry version.
    #[must_use]
    pub fn property_registry_version(&self) -> &DeclaredVersionV1 {
        &self.property_registry_version
    }

    /// The declared platform asset library version.
    #[must_use]
    pub fn platform_asset_library_version(&self) -> &DeclaredVersionV1 {
        &self.platform_asset_library_version
    }

    /// The declared base entry.
    #[must_use]
    pub fn base(&self) -> &BaseDeclarationV1 {
        &self.base
    }

    /// The declared Entity.
    #[must_use]
    pub fn entity(&self) -> &EntityDeclarationV1 {
        &self.entity
    }

    /// How many EFS nodes were declared, including representations.
    #[must_use]
    pub const fn node_count(&self) -> usize {
        self.node_count
    }

    /// How many Pages were declared.
    #[must_use]
    pub const fn page_count(&self) -> usize {
        self.page_count
    }

    /// How many representations were declared.
    #[must_use]
    pub const fn representation_count(&self) -> usize {
        self.representation_count
    }

    /// Whether anything about these bytes is publication-authorized.
    ///
    /// Always `false`; a syntactic parse never authorizes publication.
    #[must_use]
    pub const fn is_publication_authorized(&self) -> bool {
        false
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Counts {
    nodes: usize,
    pages: usize,
    representations: usize,
}

fn label_of(members: &[(String, DeclaredNode)], location: &str) -> Result<String, ManifestError> {
    let text = as_string(
        member(members, "label", location)?,
        &format!("{location}.label"),
    )?;
    let scalars = text.chars().count();
    // NFC normalisation itself is a later Unicode gate; only the accepted scalar bound is
    // asserted here, and the docs say so rather than implying normalisation happened.
    if scalars == 0 || scalars > 120 {
        return Err(ManifestError::new(
            ManifestErrorKind::InvalidEnumValue,
            format!("{location}.label"),
        ));
    }
    Ok(text.to_owned())
}

fn build_entity(node: &DeclaredNode) -> Result<(EntityDeclarationV1, Counts), ManifestError> {
    let members = as_object(node, "entity")?;
    closed_keys(members, &["id", "label", "facets", "properties"], "entity")?;

    let id = EntityIdV1::parse(as_string(member(members, "id", "entity")?, "entity.id")?)
        .map_err(|_| ManifestError::new(ManifestErrorKind::InvalidIdentity, "entity.id"))?;
    let label = label_of(members, "entity")?;
    let properties = optional_properties(members, "entity")?;

    let raw_facets = as_array(member(members, "facets", "entity")?, "entity.facets")?;
    if raw_facets.is_empty() {
        return Err(ManifestError::new(
            ManifestErrorKind::EmptyCollection,
            "entity.facets",
        ));
    }

    let mut counts = Counts {
        nodes: 1,
        pages: 0,
        representations: 0,
    };
    let mut facets: Vec<FacetDeclarationV1> = Vec::new();
    let mut seen: Vec<String> = Vec::new();
    for (index, raw) in raw_facets.iter().enumerate() {
        let location = format!("entity.facets[{index}]");
        let facet = build_facet(raw, &location, &mut counts)?;
        let key = facet.id().as_str().to_owned();
        if seen.contains(&key) {
            return Err(ManifestError::new(
                ManifestErrorKind::DuplicateScopedId,
                format!("{location}.id"),
            ));
        }
        seen.push(key);
        facets.push(facet);
    }

    verify_unique_source_claims(&facets)?;

    Ok((
        EntityDeclarationV1 {
            id,
            label,
            facets,
            properties,
        },
        counts,
    ))
}

/// Refuses two declared representations reserving one logical candidate path.
///
/// This is a **logical** comparison only, exactly as [`solo_domain`] documents: alias,
/// hardlink and case-equivalence identity need the future confined snapshot adapter and
/// are not claimed here.
fn verify_unique_source_claims(facets: &[FacetDeclarationV1]) -> Result<(), ManifestError> {
    let mut reservations = SourceCandidateReservationsV1::new();
    let mut owner: u64 = 0;
    for (facet_index, facet) in facets.iter().enumerate() {
        for (site_index, site) in facet.sites().iter().enumerate() {
            for (page_index, page) in site.pages().iter().enumerate() {
                for (representation_index, representation) in
                    page.representations().iter().enumerate()
                {
                    owner += 1;
                    reservations
                        .reserve(owner, representation.source_candidates())
                        .map_err(|_| {
                            ManifestError::new(
                                ManifestErrorKind::DuplicateSourceClaim,
                                format!(
                                    "entity.facets[{facet_index}].sites[{site_index}]\
                                     .pages[{page_index}].representations[{representation_index}]\
                                     .sourceCandidates"
                                ),
                            )
                        })?;
                }
            }
        }
    }
    Ok(())
}

fn build_facet(
    node: &DeclaredNode,
    location: &str,
    counts: &mut Counts,
) -> Result<FacetDeclarationV1, ManifestError> {
    let members = as_object(node, location)?;
    closed_keys(members, &["id", "label", "sites", "properties"], location)?;

    let id = FacetIdV1::parse(as_string(
        member(members, "id", location)?,
        &format!("{location}.id"),
    )?)
    .map_err(|_| {
        ManifestError::new(ManifestErrorKind::InvalidIdentity, format!("{location}.id"))
    })?;
    let label = label_of(members, location)?;
    let properties = optional_properties(members, location)?;
    counts.nodes += 1;

    let raw_sites = as_array(
        member(members, "sites", location)?,
        &format!("{location}.sites"),
    )?;
    if raw_sites.is_empty() {
        return Err(ManifestError::new(
            ManifestErrorKind::EmptyCollection,
            format!("{location}.sites"),
        ));
    }

    let mut sites: Vec<SiteDeclarationV1> = Vec::new();
    let mut seen: Vec<String> = Vec::new();
    for (index, raw) in raw_sites.iter().enumerate() {
        let site_location = format!("{location}.sites[{index}]");
        let site = build_site(raw, &site_location, counts)?;
        let key = site.id().as_str().to_owned();
        if seen.contains(&key) {
            return Err(ManifestError::new(
                ManifestErrorKind::DuplicateScopedId,
                format!("{site_location}.id"),
            ));
        }
        seen.push(key);
        sites.push(site);
    }

    Ok(FacetDeclarationV1 {
        id,
        label,
        sites,
        properties,
    })
}

fn build_site(
    node: &DeclaredNode,
    location: &str,
    counts: &mut Counts,
) -> Result<SiteDeclarationV1, ManifestError> {
    let members = as_object(node, location)?;
    closed_keys(
        members,
        &[
            "id",
            "label",
            "defaultLocale",
            "languagePreferences",
            "pages",
            "properties",
        ],
        location,
    )?;

    let id = SiteIdV1::parse(as_string(
        member(members, "id", location)?,
        &format!("{location}.id"),
    )?)
    .map_err(|_| {
        ManifestError::new(ManifestErrorKind::InvalidIdentity, format!("{location}.id"))
    })?;
    let label = label_of(members, location)?;

    let default_locale = as_string(
        member(members, "defaultLocale", location)?,
        &format!("{location}.defaultLocale"),
    )?;
    if default_locale.is_empty() {
        return Err(ManifestError::new(
            ManifestErrorKind::InvalidEnumValue,
            format!("{location}.defaultLocale"),
        ));
    }
    let default_locale = default_locale.to_owned();

    let mut language_preferences: Vec<(String, String)> = Vec::new();
    if let Some(value) = optional(members, "languagePreferences") {
        let preference_location = format!("{location}.languagePreferences");
        let entries = as_object(value, &preference_location)?;
        for (request, target) in entries {
            // The request key is authored data whose BCP47 meaning is not proven here;
            // only the closed value type is asserted.
            let target = as_string(target, &format!("{preference_location}.{request}"))?;
            language_preferences.push((request.clone(), target.to_owned()));
        }
    }

    let properties = optional_properties(members, location)?;
    counts.nodes += 1;

    let raw_pages = as_array(
        member(members, "pages", location)?,
        &format!("{location}.pages"),
    )?;
    if raw_pages.is_empty() {
        return Err(ManifestError::new(
            ManifestErrorKind::EmptyCollection,
            format!("{location}.pages"),
        ));
    }

    let mut pages: Vec<PageDeclarationV1> = Vec::new();
    let mut seen: Vec<String> = Vec::new();
    for (index, raw) in raw_pages.iter().enumerate() {
        let page_location = format!("{location}.pages[{index}]");
        let page = build_page(raw, &page_location, counts)?;
        let key = page.id().as_str().to_owned();
        if seen.contains(&key) {
            return Err(ManifestError::new(
                ManifestErrorKind::DuplicateScopedId,
                format!("{page_location}.id"),
            ));
        }
        seen.push(key);
        pages.push(page);
    }

    Ok(SiteDeclarationV1 {
        id,
        label,
        default_locale,
        language_preferences,
        pages,
        properties,
    })
}

fn build_page(
    node: &DeclaredNode,
    location: &str,
    counts: &mut Counts,
) -> Result<PageDeclarationV1, ManifestError> {
    let members = as_object(node, location)?;
    closed_keys(
        members,
        &[
            "id",
            "label",
            "representations",
            "visibilityDefault",
            "entryPaths",
            "missingTranslation",
            "properties",
        ],
        location,
    )?;

    let id = PageIdV1::parse(as_string(
        member(members, "id", location)?,
        &format!("{location}.id"),
    )?)
    .map_err(|_| {
        ManifestError::new(ManifestErrorKind::InvalidIdentity, format!("{location}.id"))
    })?;
    let label = label_of(members, location)?;

    counts.nodes += 1;
    counts.pages += 1;
    if counts.pages > MAX_PAGES {
        return Err(ManifestError::new(
            ManifestErrorKind::BudgetExceeded,
            format!("{location}.pages"),
        ));
    }

    let visibility_default = match optional(members, "visibilityDefault") {
        Some(value) => parse_visibility(value, &format!("{location}.visibilityDefault"))?,
        None => DeclaredVisibility::Public,
    };

    let mut entry_paths: Vec<String> = Vec::new();
    if let Some(value) = optional(members, "entryPaths") {
        let path_location = format!("{location}.entryPaths");
        for (index, item) in as_array(value, &path_location)?.iter().enumerate() {
            entry_paths.push(as_string(item, &format!("{path_location}[{index}]"))?.to_owned());
        }
    }

    let missing_translation = match optional(members, "missingTranslation") {
        Some(value) => parse_missing_translation(value, &format!("{location}.missingTranslation"))?,
        None => DeclaredMissingTranslation::Default,
    };

    let properties = optional_properties(members, location)?;

    let raw_representations = as_array(
        member(members, "representations", location)?,
        &format!("{location}.representations"),
    )?;
    if raw_representations.is_empty() {
        return Err(ManifestError::new(
            ManifestErrorKind::EmptyCollection,
            format!("{location}.representations"),
        ));
    }
    if raw_representations.len() > MAX_REPRESENTATIONS_PER_PAGE {
        return Err(ManifestError::new(
            ManifestErrorKind::BudgetExceeded,
            format!("{location}.representations"),
        ));
    }

    let mut representations: Vec<RepresentationDeclarationV1> = Vec::new();
    let mut seen_locales: Vec<String> = Vec::new();
    for (index, raw) in raw_representations.iter().enumerate() {
        let representation_location = format!("{location}.representations[{index}]");
        let representation = build_representation(raw, &representation_location, counts)?;
        let key = representation.locale().to_owned();
        if seen_locales.contains(&key) {
            return Err(ManifestError::new(
                ManifestErrorKind::DuplicateLocale,
                format!("{representation_location}.locale"),
            ));
        }
        seen_locales.push(key);
        representations.push(representation);
    }

    Ok(PageDeclarationV1 {
        id,
        label,
        visibility_default,
        entry_paths,
        missing_translation,
        representations,
        properties,
    })
}

fn build_representation(
    node: &DeclaredNode,
    location: &str,
    counts: &mut Counts,
) -> Result<RepresentationDeclarationV1, ManifestError> {
    let members = as_object(node, location)?;
    closed_keys(
        members,
        &[
            "locale",
            "route",
            "sourceCandidates",
            "visibility",
            "properties",
        ],
        location,
    )?;

    let locale = as_string(
        member(members, "locale", location)?,
        &format!("{location}.locale"),
    )?;
    if locale.is_empty() {
        return Err(ManifestError::new(
            ManifestErrorKind::InvalidEnumValue,
            format!("{location}.locale"),
        ));
    }
    let locale = locale.to_owned();

    let route = build_route(
        member(members, "route", location)?,
        &format!("{location}.route"),
    )?;
    let source_candidates = build_source_candidates(
        member(members, "sourceCandidates", location)?,
        &format!("{location}.sourceCandidates"),
    )?;
    let visibility = match optional(members, "visibility") {
        Some(value) => Some(parse_visibility(value, &format!("{location}.visibility"))?),
        None => None,
    };
    let properties = optional_properties(members, location)?;

    counts.nodes += 1;
    counts.representations += 1;

    Ok(RepresentationDeclarationV1 {
        locale,
        route,
        source_candidates,
        visibility,
        properties,
    })
}

fn build_route(node: &DeclaredNode, location: &str) -> Result<RouteDeclarationV1, ManifestError> {
    let members = as_object(node, location)?;
    closed_keys(members, &["path", "caseMatching", "aliases"], location)?;

    let path = as_string(
        member(members, "path", location)?,
        &format!("{location}.path"),
    )?;
    if path.is_empty() {
        return Err(ManifestError::new(
            ManifestErrorKind::InvalidEnumValue,
            format!("{location}.path"),
        ));
    }
    let case_matching = match optional(members, "caseMatching") {
        Some(value) => Some(parse_case_matching(
            value,
            &format!("{location}.caseMatching"),
        )?),
        None => None,
    };

    let mut aliases: Vec<RouteAliasV1> = Vec::new();
    if let Some(value) = optional(members, "aliases") {
        let alias_location = format!("{location}.aliases");
        for (index, item) in as_array(value, &alias_location)?.iter().enumerate() {
            let at = format!("{alias_location}[{index}]");
            let alias_members = as_object(item, &at)?;
            closed_keys(alias_members, &["path", "caseMatching"], &at)?;
            let alias_path = as_string(member(alias_members, "path", &at)?, &format!("{at}.path"))?;
            if alias_path.is_empty() {
                return Err(ManifestError::new(
                    ManifestErrorKind::InvalidEnumValue,
                    format!("{at}.path"),
                ));
            }
            let alias_case = match optional(alias_members, "caseMatching") {
                Some(raw) => Some(parse_case_matching(raw, &format!("{at}.caseMatching"))?),
                None => None,
            };
            aliases.push(RouteAliasV1 {
                path: alias_path.to_owned(),
                case_matching: alias_case,
            });
        }
    }

    Ok(RouteDeclarationV1 {
        path: path.to_owned(),
        case_matching,
        aliases,
    })
}

fn build_source_candidates(
    node: &DeclaredNode,
    location: &str,
) -> Result<SourceCandidateSetV1, ManifestError> {
    let members = as_object(node, location)?;
    if members.is_empty() {
        return Err(ManifestError::new(
            ManifestErrorKind::EmptyCandidateSet,
            location,
        ));
    }
    closed_keys(members, &["json", "markdown"], location)?;

    let mut draft = SourceCandidateDraftV1::new();
    for (name, value) in members {
        let kind = match name.as_str() {
            "json" => SourceCandidateKind::Json,
            "markdown" => SourceCandidateKind::Markdown,
            _ => {
                return Err(ManifestError::new(
                    ManifestErrorKind::UnknownKey,
                    format!("{location}.{name}"),
                ));
            }
        };
        // `null` is never an absent candidate: absence is expressed by omission only.
        if matches!(value, DeclaredNode::Null) {
            return Err(ManifestError::new(
                ManifestErrorKind::NullCandidate,
                format!("{location}.{name}"),
            ));
        }
        let text = as_string(value, &format!("{location}.{name}"))?;
        let path = SourceCandidatePathV1::parse(text, kind).map_err(|error| {
            let kind = match error.kind() {
                solo_domain::SourceLocatorErrorKind::EmptyCandidateSet => {
                    ManifestErrorKind::EmptyCandidateSet
                }
                solo_domain::SourceLocatorErrorKind::DuplicateMember => {
                    ManifestErrorKind::DuplicateCandidateMember
                }
                solo_domain::SourceLocatorErrorKind::NullMember => ManifestErrorKind::NullCandidate,
                solo_domain::SourceLocatorErrorKind::UnknownMember => ManifestErrorKind::UnknownKey,
                _ => ManifestErrorKind::InvalidSourceLocator,
            };
            ManifestError::new(kind, format!("{location}.{name}"))
        })?;
        match kind {
            SourceCandidateKind::Json => {
                draft.json(path).map_err(|_| {
                    ManifestError::new(
                        ManifestErrorKind::DuplicateCandidateMember,
                        format!("{location}.{name}"),
                    )
                })?;
            }
            SourceCandidateKind::Markdown => {
                draft.markdown(path).map_err(|_| {
                    ManifestError::new(
                        ManifestErrorKind::DuplicateCandidateMember,
                        format!("{location}.{name}"),
                    )
                })?;
            }
        }
    }

    draft
        .build()
        .map_err(|_| ManifestError::new(ManifestErrorKind::EmptyCandidateSet, location))
}

fn parse_visibility(
    node: &DeclaredNode,
    location: &str,
) -> Result<DeclaredVisibility, ManifestError> {
    match as_string(node, location)? {
        "public" => Ok(DeclaredVisibility::Public),
        "unlisted" => Ok(DeclaredVisibility::Unlisted),
        _ => Err(ManifestError::new(
            ManifestErrorKind::InvalidEnumValue,
            location,
        )),
    }
}

fn parse_case_matching(
    node: &DeclaredNode,
    location: &str,
) -> Result<DeclaredCaseMatching, ManifestError> {
    match as_string(node, location)? {
        "insensitive" => Ok(DeclaredCaseMatching::Insensitive),
        "sensitive" => Ok(DeclaredCaseMatching::Sensitive),
        _ => Err(ManifestError::new(
            ManifestErrorKind::InvalidEnumValue,
            location,
        )),
    }
}

fn parse_missing_translation(
    node: &DeclaredNode,
    location: &str,
) -> Result<DeclaredMissingTranslation, ManifestError> {
    let members = as_object(node, location)?;
    closed_keys(members, &["mode", "fallbackLocale"], location)?;

    let mode = as_string(
        member(members, "mode", location)?,
        &format!("{location}.mode"),
    )?;
    let fallback = optional(members, "fallbackLocale");
    match mode {
        "default" => {
            if fallback.is_some() {
                return Err(ManifestError::new(
                    ManifestErrorKind::InvalidEnumValue,
                    format!("{location}.fallbackLocale"),
                ));
            }
            Ok(DeclaredMissingTranslation::Default)
        }
        "strict" => {
            if fallback.is_some() {
                return Err(ManifestError::new(
                    ManifestErrorKind::InvalidEnumValue,
                    format!("{location}.fallbackLocale"),
                ));
            }
            Ok(DeclaredMissingTranslation::Strict)
        }
        "locale" => {
            let Some(value) = fallback else {
                return Err(ManifestError::new(
                    ManifestErrorKind::MissingKey,
                    format!("{location}.fallbackLocale"),
                ));
            };
            let text = as_string(value, &format!("{location}.fallbackLocale"))?;
            if text.is_empty() {
                return Err(ManifestError::new(
                    ManifestErrorKind::InvalidEnumValue,
                    format!("{location}.fallbackLocale"),
                ));
            }
            Ok(DeclaredMissingTranslation::Locale(text.to_owned()))
        }
        _ => Err(ManifestError::new(
            ManifestErrorKind::InvalidEnumValue,
            format!("{location}.mode"),
        )),
    }
}
