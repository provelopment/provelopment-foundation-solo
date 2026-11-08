//! Validated single-Entity EFS ownership graph (`R02.A`).
//!
//! The accepted model has exactly one Entity that owns every Facet, and the
//! containment tree **is** the ownership tree: an Entity contains its Facets, a
//! Facet its Sites, a Site its Pages and a Page its localized representations
//! (`ARCHITECTURE.md` §1.1, §6.2B). There is no parent pointer, ancestor field,
//! type discriminator or second graph, so an orphan, a competing parent and a
//! cross-parent claim cannot even be expressed — they can only be refused.
//!
//! What this module proves is **identity, ownership structure, cardinality and
//! qualified lineage in pure Rust**. It is deliberately not the publication
//! manifest: it performs no JSON parsing, no source selection, no routing, no
//! visibility decision, no draft admission and no locale canonicalization. No
//! identifier is repaired, case-folded, slugified, truncated, generated or
//! inferred, and no value here is derived from a public address, hostname,
//! route, filename, locale or geographic fact.
//!
//! # Locale keys are membership keys, not validated tags
//!
//! [`LocaleKeyV1`] is the deliberate boundary between this structural model and
//! the future canonical BCP47/Unicode 18 validator. Its constructor
//! [`LocaleKeyV1::from_already_canonical`] accepts a key that the caller asserts
//! is already a canonical full region-qualified tag — `en-001`, `de-DE`,
//! `id-ID`, `zh-Hans-CN` — and checks structural completeness only. It is **not**
//! a locale validator and must never be described as one: it never case-folds,
//! reorders subtags or invents a region, and supplying a raw authored string to
//! this model confers no publication authority.

use core::fmt;
use std::collections::BTreeSet;

use crate::entity_id::{EntityIdErrorKind, EntityIdV1};

/// Maximum number of EFS nodes in one Installation graph, the Entity and every
/// representation included (`ARCHITECTURE.md` §6.2B.7).
pub const MAX_EFS_NODES: usize = 10_000;

/// Maximum number of Pages in one Installation graph.
pub const MAX_PAGES: usize = 500;

/// Maximum number of representations on one published Page.
pub const MAX_REPRESENTATIONS_PER_PAGE: usize = 100;

/// One of the five EFS levels, used to report where a refusal applies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EfsLevel {
    /// The single owning Entity.
    Entity,
    /// A context inside the Entity.
    Facet,
    /// A content context inside a Facet.
    Site,
    /// The language-neutral identity inside a Site.
    Page,
    /// Authored content for one Page and one locale.
    Representation,
}

impl EfsLevel {
    /// Stable, safe diagnostic token for this level.
    #[must_use]
    pub const fn detail(self) -> &'static str {
        match self {
            Self::Entity => "entity",
            Self::Facet => "facet",
            Self::Site => "site",
            Self::Page => "page",
            Self::Representation => "representation",
        }
    }
}

impl fmt::Display for EfsLevel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.detail())
    }
}

/// Refusal of a candidate Facet, Site or Page identifier.
///
/// The reason is delegated to the accepted Entity identity grammar, so there is
/// exactly one identifier rule in this crate. The refused input is never echoed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EfsNodeIdError {
    level: EfsLevel,
    kind: EntityIdErrorKind,
}

impl EfsNodeIdError {
    const fn new(level: EfsLevel, kind: EntityIdErrorKind) -> Self {
        Self { level, kind }
    }

    /// The level whose identifier was refused.
    #[must_use]
    pub const fn level(&self) -> EfsLevel {
        self.level
    }

    /// The refusal reason, taken from the accepted identity grammar.
    #[must_use]
    pub const fn kind(&self) -> EntityIdErrorKind {
        self.kind
    }

    /// Stable diagnostic code for this refusal.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        "PREFLIGHT_EFS_NODE_ID_INVALID"
    }

    /// Stable, safe diagnostic token for the refusal reason.
    #[must_use]
    pub const fn detail(&self) -> &'static str {
        self.kind.detail()
    }
}

impl fmt::Display for EfsNodeIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}: {} {}",
            self.code(),
            self.level,
            self.detail()
        )
    }
}

impl std::error::Error for EfsNodeIdError {}

/// Validates an identifier against the one accepted grammar and discards the
/// wrapper, so this module never restates the rule.
fn validate_node_grammar(level: EfsLevel, candidate: &str) -> Result<(), EfsNodeIdError> {
    EntityIdV1::parse(candidate)
        .map(|_| ())
        .map_err(|error| EfsNodeIdError::new(level, error.kind()))
}

/// A validated Facet identifier, unique within its Entity.
///
/// Construction is fallible and goes through [`FacetIdV1::parse`]; no public
/// constructor, builder or conversion can bypass the domain rule.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FacetIdV1(String);

impl FacetIdV1 {
    /// Validates an authored Facet identifier.
    ///
    /// # Errors
    ///
    /// Returns [`EfsNodeIdError`] when the candidate does not already match the
    /// accepted identifier grammar; the value is never repaired.
    pub fn parse(candidate: &str) -> Result<Self, EfsNodeIdError> {
        validate_node_grammar(EfsLevel::Facet, candidate)?;
        Ok(Self(candidate.to_owned()))
    }

    /// The validated identifier as authored.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for FacetIdV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// A validated Site identifier, unique within its Facet.
///
/// A Site ID is not a domain, geographic area or locale. Construction is
/// fallible and goes through [`SiteIdV1::parse`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SiteIdV1(String);

impl SiteIdV1 {
    /// Validates an authored Site identifier.
    ///
    /// # Errors
    ///
    /// Returns [`EfsNodeIdError`] when the candidate does not already match the
    /// accepted identifier grammar; the value is never repaired.
    pub fn parse(candidate: &str) -> Result<Self, EfsNodeIdError> {
        validate_node_grammar(EfsLevel::Site, candidate)?;
        Ok(Self(candidate.to_owned()))
    }

    /// The validated identifier as authored.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SiteIdV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// A validated Page identifier, unique within its Site.
///
/// A Page ID is the language-neutral content identity, never a URL, route
/// segment or translation. Construction is fallible and goes through
/// [`PageIdV1::parse`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PageIdV1(String);

impl PageIdV1 {
    /// Validates an authored Page identifier.
    ///
    /// # Errors
    ///
    /// Returns [`EfsNodeIdError`] when the candidate does not already match the
    /// accepted identifier grammar; the value is never repaired.
    pub fn parse(candidate: &str) -> Result<Self, EfsNodeIdError> {
        validate_node_grammar(EfsLevel::Page, candidate)?;
        Ok(Self(candidate.to_owned()))
    }

    /// The validated identifier as authored.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PageIdV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Why a candidate representation locale key was refused.
///
/// These are **structural** reasons only. They neither assert nor imply that a
/// key is a canonical BCP47 tag; the accepted canonicalizer is future work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LocaleKeyErrorKind {
    /// The key was empty.
    Empty,
    /// The key contained a non-ASCII byte.
    NonAscii,
    /// The key contained a byte outside `[A-Za-z0-9-]`.
    InvalidCharacter,
    /// The key was a bare primary subtag such as `en`, `de`, `id` or `zh`.
    NotFullLocale,
    /// The key had a leading, trailing or doubled hyphen.
    Malformed,
}

impl LocaleKeyErrorKind {
    /// Stable, safe diagnostic token for this refusal reason.
    #[must_use]
    pub const fn detail(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::NonAscii => "non-ascii",
            Self::InvalidCharacter => "invalid-character",
            Self::NotFullLocale => "not-full-locale",
            Self::Malformed => "malformed",
        }
    }
}

/// Refusal of a candidate representation locale key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocaleKeyError {
    kind: LocaleKeyErrorKind,
}

impl LocaleKeyError {
    const fn new(kind: LocaleKeyErrorKind) -> Self {
        Self { kind }
    }

    /// The refusal reason.
    #[must_use]
    pub const fn kind(&self) -> LocaleKeyErrorKind {
        self.kind
    }

    /// Stable diagnostic code for this refusal.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        "PREFLIGHT_EFS_LOCALE_KEY_INVALID"
    }

    /// Stable, safe diagnostic token for the refusal reason.
    #[must_use]
    pub const fn detail(&self) -> &'static str {
        self.kind.detail()
    }
}

impl fmt::Display for LocaleKeyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code(), self.detail())
    }
}

impl std::error::Error for LocaleKeyError {}

/// A representation membership key for one locale within one Page.
///
/// This type is a **membership key**, not a validated locale tag and not a
/// publication decision. [`LocaleKeyV1::from_already_canonical`] checks only
/// that the supplied key is structurally complete — non-empty, ASCII, restricted
/// to `[A-Za-z0-9-]`, hyphen-separated with no leading, trailing or doubled
/// hyphen, and carrying at least one subtag separator so that a bare language
/// such as `en` cannot become an authored key. It never case-folds, canonicalizes,
/// reorders subtags, substitutes a region or invents a fallback.
///
/// Canonicity is asserted by the caller and is the contract the future BCP47 and
/// Unicode 18 validator will have to enforce before this type may be produced
/// from authored input.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LocaleKeyV1(String);

impl LocaleKeyV1 {
    /// Wraps a key the caller asserts is already a canonical full locale tag.
    ///
    /// # Errors
    ///
    /// Returns [`LocaleKeyError`] when the key is structurally incomplete or
    /// contains a character outside the permitted ASCII subset. A refused key is
    /// never repaired, case-folded or completed with an inferred region.
    pub fn from_already_canonical(candidate: &str) -> Result<Self, LocaleKeyError> {
        if candidate.is_empty() {
            return Err(LocaleKeyError::new(LocaleKeyErrorKind::Empty));
        }
        if !candidate.is_ascii() {
            return Err(LocaleKeyError::new(LocaleKeyErrorKind::NonAscii));
        }

        let mut saw_separator = false;
        let mut previous_hyphen = false;
        for (index, byte) in candidate.bytes().enumerate() {
            match byte {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' => previous_hyphen = false,
                b'-' => {
                    // A separating hyphen may never open the key, and two in a row
                    // are refused rather than collapsed.
                    if previous_hyphen || index == 0 {
                        return Err(LocaleKeyError::new(LocaleKeyErrorKind::Malformed));
                    }
                    saw_separator = true;
                    previous_hyphen = true;
                }
                _ => {
                    return Err(LocaleKeyError::new(LocaleKeyErrorKind::InvalidCharacter));
                }
            }
        }
        if previous_hyphen {
            return Err(LocaleKeyError::new(LocaleKeyErrorKind::Malformed));
        }
        if !saw_separator {
            // A bare primary subtag is never a full region-qualified locale.
            return Err(LocaleKeyError::new(LocaleKeyErrorKind::NotFullLocale));
        }

        Ok(Self(candidate.to_owned()))
    }

    /// The key exactly as supplied by the caller.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for LocaleKeyV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Why a candidate ownership graph was refused.
///
/// Cardinality refusals and uniqueness refusals are distinct categories, and the
/// whole graph is refused: no partially accepted structure is ever returned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EfsGraphErrorKind {
    /// The Entity contained no Facet, although one or more are required.
    NoFacet,
    /// A Facet contained no Site, although one or more are required.
    FacetWithoutSite,
    /// A Site contained no Page, although one or more are required.
    SiteWithoutPage,
    /// A published Page contained no representation, although one is required.
    PageWithoutRepresentation,
    /// Two Facets shared an identifier inside the one Entity.
    DuplicateFacetId,
    /// Two Sites shared an identifier inside one Facet.
    DuplicateSiteId,
    /// Two Pages shared an identifier inside one Site.
    DuplicatePageId,
    /// Two representations shared a locale key inside one Page.
    DuplicateLocaleKey,
    /// The graph exceeded [`MAX_EFS_NODES`].
    TooManyNodes,
    /// The graph exceeded [`MAX_PAGES`].
    TooManyPages,
    /// A Page exceeded [`MAX_REPRESENTATIONS_PER_PAGE`].
    TooManyRepresentations,
}

impl EfsGraphErrorKind {
    /// Stable, safe diagnostic token for this refusal reason.
    #[must_use]
    pub const fn detail(self) -> &'static str {
        match self {
            Self::NoFacet => "no-facet",
            Self::FacetWithoutSite => "facet-without-site",
            Self::SiteWithoutPage => "site-without-page",
            Self::PageWithoutRepresentation => "page-without-representation",
            Self::DuplicateFacetId => "duplicate-facet-id",
            Self::DuplicateSiteId => "duplicate-site-id",
            Self::DuplicatePageId => "duplicate-page-id",
            Self::DuplicateLocaleKey => "duplicate-locale-key",
            Self::TooManyNodes => "too-many-nodes",
            Self::TooManyPages => "too-many-pages",
            Self::TooManyRepresentations => "too-many-representations",
        }
    }

    /// The EFS level the refusal applies to.
    #[must_use]
    pub const fn level(self) -> EfsLevel {
        match self {
            Self::NoFacet | Self::TooManyNodes | Self::TooManyPages => EfsLevel::Entity,
            Self::FacetWithoutSite | Self::DuplicateFacetId => EfsLevel::Facet,
            Self::SiteWithoutPage | Self::DuplicateSiteId => EfsLevel::Site,
            Self::PageWithoutRepresentation
            | Self::DuplicatePageId
            | Self::DuplicateLocaleKey
            | Self::TooManyRepresentations => EfsLevel::Page,
        }
    }
}

/// Refusal of a candidate ownership graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EfsGraphError {
    kind: EfsGraphErrorKind,
}

impl EfsGraphError {
    const fn new(kind: EfsGraphErrorKind) -> Self {
        Self { kind }
    }

    /// The refusal reason.
    #[must_use]
    pub const fn kind(&self) -> EfsGraphErrorKind {
        self.kind
    }

    /// The EFS level the refusal applies to.
    #[must_use]
    pub const fn level(&self) -> EfsLevel {
        self.kind.level()
    }

    /// Stable diagnostic code for this refusal.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        "PREFLIGHT_EFS_GRAPH_INVALID"
    }

    /// Stable, safe diagnostic token for the refusal reason.
    #[must_use]
    pub const fn detail(&self) -> &'static str {
        self.kind.detail()
    }
}

impl fmt::Display for EfsGraphError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code(), self.detail())
    }
}

impl std::error::Error for EfsGraphError {}

/// Why a qualified EFS lineage could not be resolved.
///
/// A refusal deliberately does not distinguish "absent" from "belongs to another
/// parent", so a lookup can never become an existence oracle across scopes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EfsLineageErrorKind {
    /// The asserted Entity identifier was not this graph's single Entity.
    UnknownEntity,
    /// No Facet with that identifier exists in the Entity.
    UnknownFacet,
    /// No Site with that identifier exists in the named Facet.
    UnknownSite,
    /// No Page with that identifier exists in the named Site.
    UnknownPage,
    /// No representation with that locale key exists on the named Page.
    UnknownRepresentation,
}

impl EfsLineageErrorKind {
    /// Stable, safe diagnostic token for this refusal reason.
    #[must_use]
    pub const fn detail(self) -> &'static str {
        match self {
            Self::UnknownEntity => "unknown-entity",
            Self::UnknownFacet => "unknown-facet",
            Self::UnknownSite => "unknown-site",
            Self::UnknownPage => "unknown-page",
            Self::UnknownRepresentation => "unknown-representation",
        }
    }

    /// The EFS level that failed to resolve.
    #[must_use]
    pub const fn level(self) -> EfsLevel {
        match self {
            Self::UnknownEntity => EfsLevel::Entity,
            Self::UnknownFacet => EfsLevel::Facet,
            Self::UnknownSite => EfsLevel::Site,
            Self::UnknownPage => EfsLevel::Page,
            Self::UnknownRepresentation => EfsLevel::Representation,
        }
    }
}

/// Refusal of a qualified EFS lineage lookup.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EfsLineageError {
    kind: EfsLineageErrorKind,
}

impl EfsLineageError {
    const fn new(kind: EfsLineageErrorKind) -> Self {
        Self { kind }
    }

    /// The refusal reason.
    #[must_use]
    pub const fn kind(&self) -> EfsLineageErrorKind {
        self.kind
    }

    /// The EFS level that failed to resolve.
    #[must_use]
    pub const fn level(&self) -> EfsLevel {
        self.kind.level()
    }

    /// Stable diagnostic code for this refusal.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        "PREFLIGHT_EFS_LINEAGE_UNRESOLVED"
    }

    /// Stable, safe diagnostic token for the refusal reason.
    #[must_use]
    pub const fn detail(&self) -> &'static str {
        self.kind.detail()
    }
}

impl fmt::Display for EfsLineageError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code(), self.detail())
    }
}

impl std::error::Error for EfsLineageError {}

/// A Page under construction.
///
/// A draft is never a published graph and is deliberately permissive: it records
/// what the author supplied so that [`EfsOwnershipDraft::build`] can refuse it
/// with a precise reason instead of silently repairing it.
#[derive(Debug, Clone)]
pub struct PageDraft {
    id: PageIdV1,
    representations: Vec<LocaleKeyV1>,
}

impl PageDraft {
    /// Starts a Page draft with no representation yet.
    #[must_use]
    pub fn new(id: PageIdV1) -> Self {
        Self {
            id,
            representations: Vec::new(),
        }
    }

    /// Appends one representation membership key, preserving authored order.
    pub fn add_representation(&mut self, locale: LocaleKeyV1) -> &mut Self {
        self.representations.push(locale);
        self
    }
}

/// A Site under construction.
#[derive(Debug, Clone)]
pub struct SiteDraft {
    id: SiteIdV1,
    pages: Vec<PageDraft>,
}

impl SiteDraft {
    /// Starts a Site draft with no Page yet.
    #[must_use]
    pub fn new(id: SiteIdV1) -> Self {
        Self {
            id,
            pages: Vec::new(),
        }
    }

    /// Appends one Page draft, preserving authored order.
    pub fn add_page(&mut self, page: PageDraft) -> &mut Self {
        self.pages.push(page);
        self
    }
}

/// A Facet under construction.
#[derive(Debug, Clone)]
pub struct FacetDraft {
    id: FacetIdV1,
    sites: Vec<SiteDraft>,
}

impl FacetDraft {
    /// Starts a Facet draft with no Site yet.
    #[must_use]
    pub fn new(id: FacetIdV1) -> Self {
        Self {
            id,
            sites: Vec::new(),
        }
    }

    /// Appends one Site draft, preserving authored order.
    pub fn add_site(&mut self, site: SiteDraft) -> &mut Self {
        self.sites.push(site);
        self
    }
}

/// The one Entity under construction, owning its Facets.
///
/// There is exactly one draft root and no way to add a second Entity, so the
/// one-Entity invariant holds before validation even runs. Nothing is inferred:
/// [`EfsOwnershipDraft::build`] never invents a starter `main` / `main` / `home`
/// structure, and an empty draft is refused rather than seeded.
#[derive(Debug, Clone)]
pub struct EfsOwnershipDraft {
    entity: EntityIdV1,
    facets: Vec<FacetDraft>,
}

impl EfsOwnershipDraft {
    /// Starts the graph for the one Entity that owns all content.
    #[must_use]
    pub fn new(entity: EntityIdV1) -> Self {
        Self {
            entity,
            facets: Vec::new(),
        }
    }

    /// Appends one Facet draft, preserving authored order.
    pub fn add_facet(&mut self, facet: FacetDraft) -> &mut Self {
        self.facets.push(facet);
        self
    }

    /// Validates the whole draft and returns a complete immutable graph.
    ///
    /// Validation is deterministic and order-independent: the same authored tree
    /// is refused for the same reason regardless of array order, every element is
    /// checked instead of stopping at a dictionary-order first match, and a
    /// refusal returns no partial graph. Uniqueness and required-child refusals are
    /// reported before the aggregate limits.
    ///
    /// # Errors
    ///
    /// Returns [`EfsGraphError`] when a required child is absent, an identifier is
    /// duplicated inside its own scope, or an accepted EFS bound is exceeded.
    pub fn build(self) -> Result<EfsOwnershipGraph, EfsGraphError> {
        Self::validate(&self)?;

        let Self { entity, facets } = self;
        let facets = facets
            .into_iter()
            .map(|facet| FacetNode {
                id: facet.id,
                sites: facet
                    .sites
                    .into_iter()
                    .map(|site| SiteNode {
                        id: site.id,
                        pages: site
                            .pages
                            .into_iter()
                            .map(|page| PageNode {
                                id: page.id,
                                representations: page.representations,
                            })
                            .collect(),
                    })
                    .collect(),
            })
            .collect();

        Ok(EfsOwnershipGraph { entity, facets })
    }

    /// The whole-graph check, run before any owned conversion.
    fn validate(draft: &Self) -> Result<(), EfsGraphError> {
        if draft.facets.is_empty() {
            return Err(EfsGraphError::new(EfsGraphErrorKind::NoFacet));
        }

        // The Entity itself is an EFS node.
        let mut node_count = 1usize;
        let mut page_count = 0usize;
        let mut facet_ids: BTreeSet<&str> = BTreeSet::new();

        for facet in &draft.facets {
            if !facet_ids.insert(facet.id.as_str()) {
                return Err(EfsGraphError::new(EfsGraphErrorKind::DuplicateFacetId));
            }
            if facet.sites.is_empty() {
                return Err(EfsGraphError::new(EfsGraphErrorKind::FacetWithoutSite));
            }
            node_count += 1;

            let mut site_ids: BTreeSet<&str> = BTreeSet::new();
            for site in &facet.sites {
                if !site_ids.insert(site.id.as_str()) {
                    return Err(EfsGraphError::new(EfsGraphErrorKind::DuplicateSiteId));
                }
                if site.pages.is_empty() {
                    return Err(EfsGraphError::new(EfsGraphErrorKind::SiteWithoutPage));
                }
                node_count += 1;

                let mut page_ids: BTreeSet<&str> = BTreeSet::new();
                for page in &site.pages {
                    if !page_ids.insert(page.id.as_str()) {
                        return Err(EfsGraphError::new(EfsGraphErrorKind::DuplicatePageId));
                    }
                    if page.representations.is_empty() {
                        return Err(EfsGraphError::new(
                            EfsGraphErrorKind::PageWithoutRepresentation,
                        ));
                    }
                    if page.representations.len() > MAX_REPRESENTATIONS_PER_PAGE {
                        return Err(EfsGraphError::new(
                            EfsGraphErrorKind::TooManyRepresentations,
                        ));
                    }

                    let mut locale_keys: BTreeSet<&str> = BTreeSet::new();
                    for locale in &page.representations {
                        if !locale_keys.insert(locale.as_str()) {
                            return Err(EfsGraphError::new(EfsGraphErrorKind::DuplicateLocaleKey));
                        }
                        node_count += 1;
                    }

                    node_count += 1;
                    page_count += 1;
                }
            }
        }

        if page_count > MAX_PAGES {
            return Err(EfsGraphError::new(EfsGraphErrorKind::TooManyPages));
        }
        if node_count > MAX_EFS_NODES {
            return Err(EfsGraphError::new(EfsGraphErrorKind::TooManyNodes));
        }

        Ok(())
    }
}

/// A validated Facet and the Sites it directly owns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FacetNode {
    id: FacetIdV1,
    sites: Vec<SiteNode>,
}

impl FacetNode {
    /// The validated Facet identifier.
    #[must_use]
    pub fn id(&self) -> &FacetIdV1 {
        &self.id
    }

    /// The Sites this Facet directly owns, in authored order.
    #[must_use]
    pub fn sites(&self) -> &[SiteNode] {
        &self.sites
    }
}

/// A validated Site and the Pages it directly owns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiteNode {
    id: SiteIdV1,
    pages: Vec<PageNode>,
}

impl SiteNode {
    /// The validated Site identifier.
    #[must_use]
    pub fn id(&self) -> &SiteIdV1 {
        &self.id
    }

    /// The Pages this Site directly owns, in authored order.
    #[must_use]
    pub fn pages(&self) -> &[PageNode] {
        &self.pages
    }
}

/// A validated published Page and the locale keys it carries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageNode {
    id: PageIdV1,
    representations: Vec<LocaleKeyV1>,
}

impl PageNode {
    /// The validated Page identifier, which is language-neutral.
    #[must_use]
    pub fn id(&self) -> &PageIdV1 {
        &self.id
    }

    /// The representation membership keys of this Page, in authored order.
    #[must_use]
    pub fn representations(&self) -> &[LocaleKeyV1] {
        &self.representations
    }
}

/// A complete qualified EFS lineage for one Page.
///
/// Every level is required, mirroring the accepted qualified-target shape
/// (`ARCHITECTURE.md` §6.2B): there is no bare Page-ID lookup, no guessed Site
/// selection and no shorthand.
#[derive(Debug, Clone, Copy)]
pub struct EfsPageTarget<'a> {
    /// The Entity that must be this graph's single Entity.
    pub entity: &'a EntityIdV1,
    /// The Facet that must directly own the Site.
    pub facet: &'a FacetIdV1,
    /// The Site that must directly own the Page.
    pub site: &'a SiteIdV1,
    /// The Page inside that Site.
    pub page: &'a PageIdV1,
}

/// A complete, validated one-Entity ownership graph.
///
/// The value is built only by [`EfsOwnershipDraft::build`], which validates the
/// whole tree first, and it exposes no mutator, no interior-mutability escape and
/// no draft accessor, so the checked cardinality cannot be invalidated afterwards.
/// Traversal preserves the authored order of every child array.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EfsOwnershipGraph {
    entity: EntityIdV1,
    facets: Vec<FacetNode>,
}

impl EfsOwnershipGraph {
    /// The one Entity identifier that owns every Facet in this graph.
    #[must_use]
    pub fn entity(&self) -> &EntityIdV1 {
        &self.entity
    }

    /// The Facets of the Entity, in authored order.
    #[must_use]
    pub fn facets(&self) -> &[FacetNode] {
        &self.facets
    }

    /// Resolves one Page by its complete Entity → Facet → Site → Page lineage.
    ///
    /// The Entity must be this graph's Entity, and each ancestor must be the
    /// **direct** owner of the next level. Because identifiers are unique inside
    /// their own scope, at most one candidate can match at each level, so the
    /// result never depends on a first-match choice.
    ///
    /// # Errors
    ///
    /// Returns [`EfsLineageError`] naming the first level that does not resolve.
    /// A wrong Site or a wrong Facet is reported as unresolved rather than
    /// distinguished from absence, and the message never echoes the asserted IDs.
    pub fn resolve_page(&self, target: EfsPageTarget<'_>) -> Result<&PageNode, EfsLineageError> {
        if target.entity != &self.entity {
            return Err(EfsLineageError::new(EfsLineageErrorKind::UnknownEntity));
        }

        let facet = self
            .facets
            .iter()
            .find(|facet| facet.id == *target.facet)
            .ok_or_else(|| EfsLineageError::new(EfsLineageErrorKind::UnknownFacet))?;

        let site = facet
            .sites
            .iter()
            .find(|site| site.id == *target.site)
            .ok_or_else(|| EfsLineageError::new(EfsLineageErrorKind::UnknownSite))?;

        site.pages
            .iter()
            .find(|page| page.id == *target.page)
            .ok_or_else(|| EfsLineageError::new(EfsLineageErrorKind::UnknownPage))
    }

    /// Resolves one representation by its complete Page lineage and locale key.
    ///
    /// # Errors
    ///
    /// Returns [`EfsLineageError`], including
    /// [`EfsLineageErrorKind::UnknownRepresentation`] when the Page resolves but
    /// carries no such canonical locale key.
    pub fn resolve_representation(
        &self,
        target: EfsPageTarget<'_>,
        locale: &LocaleKeyV1,
    ) -> Result<&LocaleKeyV1, EfsLineageError> {
        let page = self.resolve_page(target)?;
        page.representations
            .iter()
            .find(|key| *key == locale)
            .ok_or_else(|| EfsLineageError::new(EfsLineageErrorKind::UnknownRepresentation))
    }
}
