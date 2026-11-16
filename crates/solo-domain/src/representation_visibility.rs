//! Typed Page/representation visibility preflight (`R04.C1`).
//!
//! This is the closed, I/O-free half of the accepted four-case visibility contract
//! (`ARCHITECTURE.md` §3.2A): one Page declaration per Page of an already validated
//! `EfsOwnershipGraph`, an optional Page default, and zero or more exact representation
//! overrides inside that Page. The ledger it produces is a **typed policy fact**, not a
//! response header, not a route and not publication authority: it carries no URL, no path,
//! no origin, no hostname and no capability to serve anything.
//!
//! # The four accepted rows
//!
//! | Page `visibilityDefault` | Representation `visibility` | Effective |
//! |---|---|---|
//! | omitted (`public`) | omitted | `public` |
//! | `public` | explicit `unlisted` | `unlisted` |
//! | explicit `unlisted` | omitted | `unlisted` |
//! | explicit `unlisted` | explicit `public` | `public` |
//!
//! # What this module deliberately is not
//!
//! It is not a manifest parser, not a locale validator, not a router and not a
//! publication decision. [`RepresentationVisibilityV1`] has exactly two members, because
//! `internal` is a *property-value exposure* and never a third manifest representation
//! visibility. Membership is enumerated from the graph itself, so a sibling Site, a
//! same-named Page in another Site and a Page in another Entity can never share a
//! declaration: a foreign or incomplete declaration set is refused rather than repaired.

use core::fmt;

use crate::efs_ownership::{
    EfsLevel, EfsOwnershipGraph, LocaleKeyV1, MAX_PAGES, MAX_REPRESENTATIONS_PER_PAGE,
};
use crate::entity_id::EntityIdV1;
use crate::property_registry::QualifiedOwnerV1;

/// Maximum Page declarations one ledger build accepts (the accepted Page ceiling).
pub const MAX_VISIBILITY_PAGE_DECLARATIONS_V1: usize = MAX_PAGES;

/// Maximum representation overrides one Page declaration accepts.
pub const MAX_VISIBILITY_REPRESENTATION_OVERRIDES_V1: usize = MAX_REPRESENTATIONS_PER_PAGE;

/// One closed effective representation visibility.
///
/// The only values are `public` and `unlisted`. `internal` is an authored property-value
/// exposure, never a representation visibility, so it is refused as a token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RepresentationVisibilityV1 {
    /// Eligible to be considered for future discovery, once routes and publication exist.
    Public,
    /// Never automatically discoverable; a future exact authorized route may serve it with
    /// `noindex, nofollow`. It is **not** secret and not access-controlled.
    Unlisted,
}

impl RepresentationVisibilityV1 {
    /// The stable manifest token.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Public => "public",
            Self::Unlisted => "unlisted",
        }
    }

    /// The strict `public | unlisted` parser for eventual manifest use.
    ///
    /// Mixed case, an empty token, a property exposure such as `internal` and any unknown
    /// token are refused. Nothing is case-folded, defaulted or repaired.
    ///
    /// # Errors
    ///
    /// Returns [`RepresentationVisibilityTokenRefusalV1`] naming the closed cause.
    pub fn parse_token(candidate: &str) -> Result<Self, RepresentationVisibilityTokenRefusalV1> {
        match candidate {
            "public" => Ok(Self::Public),
            "unlisted" => Ok(Self::Unlisted),
            _ => Err(RepresentationVisibilityTokenRefusalV1::classify(candidate)),
        }
    }

    /// Whether this state is discoverable *intent*. Only `public` is.
    ///
    /// This is a policy fact, not a crawler instruction and not a discovery document.
    #[must_use]
    pub const fn is_discoverable_intent(self) -> bool {
        matches!(self, Self::Public)
    }

    /// The robot *intent* this state would require — never a response header.
    #[must_use]
    pub const fn robots_intent(self) -> &'static str {
        match self {
            Self::Public => "index,follow",
            Self::Unlisted => "noindex,nofollow",
        }
    }
}

impl fmt::Display for RepresentationVisibilityV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.token())
    }
}

/// Why a visibility token was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepresentationVisibilityTokenRefusalKindV1 {
    /// The token was empty.
    Empty,
    /// A property-value exposure (`internal`, `private`, `hidden`) is not a manifest
    /// representation visibility.
    NonManifestExposure,
    /// A correct token in mixed case.
    MixedCase,
    /// Any other unknown token.
    Unknown,
}

impl RepresentationVisibilityTokenRefusalKindV1 {
    /// The stable diagnostic token.
    #[must_use]
    pub const fn detail(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::NonManifestExposure => "non-manifest-exposure",
            Self::MixedCase => "mixed-case",
            Self::Unknown => "unknown",
        }
    }
}

/// One refused visibility token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RepresentationVisibilityTokenRefusalV1 {
    kind: RepresentationVisibilityTokenRefusalKindV1,
}

impl RepresentationVisibilityTokenRefusalV1 {
    fn classify(candidate: &str) -> Self {
        let kind = if candidate.is_empty() {
            RepresentationVisibilityTokenRefusalKindV1::Empty
        } else if matches!(candidate, "internal" | "private" | "hidden") {
            RepresentationVisibilityTokenRefusalKindV1::NonManifestExposure
        } else if candidate.eq_ignore_ascii_case("public")
            || candidate.eq_ignore_ascii_case("unlisted")
        {
            RepresentationVisibilityTokenRefusalKindV1::MixedCase
        } else {
            RepresentationVisibilityTokenRefusalKindV1::Unknown
        };
        Self { kind }
    }

    /// The closed cause.
    #[must_use]
    pub const fn kind(self) -> RepresentationVisibilityTokenRefusalKindV1 {
        self.kind
    }

    /// The stable diagnostic token.
    #[must_use]
    pub const fn detail(self) -> &'static str {
        self.kind.detail()
    }
}

/// Where one effective state came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RepresentationVisibilityOriginV1 {
    /// The Page default (which is `public` when the Page default is omitted).
    PageDefault,
    /// An exact representation override inside the Page.
    RepresentationOverride,
}

impl RepresentationVisibilityOriginV1 {
    /// The stable provenance token.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::PageDefault => "page-default",
            Self::RepresentationOverride => "representation-override",
        }
    }
}

/// Why one ledger build or declaration was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepresentationVisibilityRefusalKindV1 {
    /// The declaration owner is not exactly a Page.
    PageOwnerNotAPage,
    /// The declared Entity is not the graph's Entity.
    UnknownEntity,
    /// The declared Facet is not directly owned by that Entity.
    WrongFacet,
    /// The declared Site is not directly owned by that Facet.
    WrongSite,
    /// The declared Page is not directly owned by that Site.
    UnknownPage,
    /// Two declarations name the same Page.
    DuplicatePageDeclaration,
    /// A Page of the graph carries no declaration.
    MissingPageDeclaration,
    /// A Page carries no representation, so no effective state can exist for it.
    PageWithoutRepresentations,
    /// An override names a locale that is not a representation of that exact Page.
    ForeignRepresentation,
    /// One representation is overridden twice inside the same Page.
    DuplicateRepresentationOverride,
    /// More Page declarations than the accepted ceiling.
    TooManyPageDeclarations,
    /// More overrides on one Page than the accepted ceiling.
    TooManyRepresentationOverrides,
}

impl RepresentationVisibilityRefusalKindV1 {
    /// The stable diagnostic token.
    #[must_use]
    pub const fn detail(self) -> &'static str {
        match self {
            Self::PageOwnerNotAPage => "page-owner-not-a-page",
            Self::UnknownEntity => "unknown-entity",
            Self::WrongFacet => "wrong-facet",
            Self::WrongSite => "wrong-site",
            Self::UnknownPage => "unknown-page",
            Self::DuplicatePageDeclaration => "duplicate-page-declaration",
            Self::MissingPageDeclaration => "missing-page-declaration",
            Self::PageWithoutRepresentations => "page-without-representations",
            Self::ForeignRepresentation => "foreign-representation",
            Self::DuplicateRepresentationOverride => "duplicate-representation-override",
            Self::TooManyPageDeclarations => "too-many-page-declarations",
            Self::TooManyRepresentationOverrides => "too-many-representation-overrides",
        }
    }
}

/// One refused ledger build, attributable to the owner it refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepresentationVisibilityRefusalV1 {
    kind: RepresentationVisibilityRefusalKindV1,
    owner: Option<Box<QualifiedOwnerV1>>,
}

impl RepresentationVisibilityRefusalV1 {
    /// A refusal of one closed kind, with no owner attributed yet.
    #[must_use]
    pub const fn of(kind: RepresentationVisibilityRefusalKindV1) -> Self {
        Self { kind, owner: None }
    }

    /// Attributes this refusal to the owner it refused.
    #[must_use]
    pub fn with_owner(mut self, owner: &QualifiedOwnerV1) -> Self {
        self.owner = Some(Box::new(owner.clone()));
        self
    }

    /// The closed cause.
    #[must_use]
    pub const fn kind(&self) -> RepresentationVisibilityRefusalKindV1 {
        self.kind
    }

    /// The exact owner this refusal concerns, when known.
    #[must_use]
    pub fn owner(&self) -> Option<&QualifiedOwnerV1> {
        self.owner.as_deref()
    }

    /// The stable diagnostic token.
    #[must_use]
    pub const fn detail(&self) -> &'static str {
        self.kind.detail()
    }
}

/// One Page's visibility declaration: an optional Page default plus exact overrides.
///
/// The owner is required to be exactly a Page, so an Entity, Facet, Site or representation
/// can never carry a declaration. An omitted default means `public`; an omitted override
/// inherits the Page default.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageVisibilityDeclarationV1 {
    owner: QualifiedOwnerV1,
    default: Option<RepresentationVisibilityV1>,
    overrides: Vec<(LocaleKeyV1, RepresentationVisibilityV1)>,
}

impl PageVisibilityDeclarationV1 {
    /// Declares one Page and its optional default.
    ///
    /// # Errors
    ///
    /// Returns [`RepresentationVisibilityRefusalKindV1::PageOwnerNotAPage`] when the owner is
    /// not exactly one Page of one Site of one Facet of one Entity.
    pub fn new(
        owner: QualifiedOwnerV1,
        default: Option<RepresentationVisibilityV1>,
    ) -> Result<Self, RepresentationVisibilityRefusalV1> {
        if owner.level() != EfsLevel::Page {
            return Err(RepresentationVisibilityRefusalV1::of(
                RepresentationVisibilityRefusalKindV1::PageOwnerNotAPage,
            )
            .with_owner(&owner));
        }
        Ok(Self {
            owner,
            default,
            overrides: Vec::new(),
        })
    }

    /// Adds one exact representation override inside this Page.
    ///
    /// # Errors
    ///
    /// Returns [`RepresentationVisibilityRefusalKindV1::DuplicateRepresentationOverride`] for a
    /// second override of the same representation, and
    /// [`RepresentationVisibilityRefusalKindV1::TooManyRepresentationOverrides`] beyond the
    /// accepted ceiling. Whether the locale really belongs to this Page is decided by
    /// [`build_representation_visibility_ledger_v1`], which holds the graph.
    pub fn with_override(
        mut self,
        locale: LocaleKeyV1,
        visibility: RepresentationVisibilityV1,
    ) -> Result<Self, RepresentationVisibilityRefusalV1> {
        if self.overrides.len() >= MAX_VISIBILITY_REPRESENTATION_OVERRIDES_V1 {
            return Err(RepresentationVisibilityRefusalV1::of(
                RepresentationVisibilityRefusalKindV1::TooManyRepresentationOverrides,
            )
            .with_owner(&self.owner));
        }
        if self.overrides.iter().any(|(seen, _)| *seen == locale) {
            return Err(RepresentationVisibilityRefusalV1::of(
                RepresentationVisibilityRefusalKindV1::DuplicateRepresentationOverride,
            )
            .with_owner(&self.owner));
        }
        self.overrides.push((locale, visibility));
        Ok(self)
    }

    /// The exact Page this declaration belongs to.
    #[must_use]
    pub const fn owner(&self) -> &QualifiedOwnerV1 {
        &self.owner
    }

    /// The declared Page default, when one is authored.
    #[must_use]
    pub const fn default(&self) -> Option<RepresentationVisibilityV1> {
        self.default
    }

    /// The exact representation overrides inside this Page, in declaration order.
    #[must_use]
    pub fn overrides(&self) -> &[(LocaleKeyV1, RepresentationVisibilityV1)] {
        &self.overrides
    }
}

/// One effective representation state, bound to its exact qualified owner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepresentationVisibilityEntryV1 {
    owner: QualifiedOwnerV1,
    effective: RepresentationVisibilityV1,
    origin: RepresentationVisibilityOriginV1,
}

impl RepresentationVisibilityEntryV1 {
    /// The exact representation this entry governs.
    #[must_use]
    pub const fn owner(&self) -> &QualifiedOwnerV1 {
        &self.owner
    }

    /// The closed effective visibility.
    #[must_use]
    pub const fn effective(&self) -> RepresentationVisibilityV1 {
        self.effective
    }

    /// Where that effective state came from.
    #[must_use]
    pub const fn origin(&self) -> RepresentationVisibilityOriginV1 {
        self.origin
    }

    /// Whether this state is discoverable *intent* (typed policy fact, not a header).
    #[must_use]
    pub const fn is_discoverable_intent(&self) -> bool {
        self.effective.is_discoverable_intent()
    }

    /// The robot *intent* this state would require (never a response header).
    #[must_use]
    pub const fn robots_intent(&self) -> &'static str {
        self.effective.robots_intent()
    }
}

/// One immutable, graph-bound effective visibility ledger.
///
/// The value exists only through [`build_representation_visibility_ledger_v1`], which
/// enumerates the graph itself, so it can neither omit a representation nor contain one the
/// graph does not own. There is no setter and no interior mutability, so an effective state
/// cannot be overwritten after validation. It carries no URL, route, path, origin, hostname
/// or publication authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepresentationVisibilityLedgerV1 {
    entity: EntityIdV1,
    entries: Vec<RepresentationVisibilityEntryV1>,
}

impl RepresentationVisibilityLedgerV1 {
    /// The one Entity every entry belongs to.
    #[must_use]
    pub const fn entity_id(&self) -> &EntityIdV1 {
        &self.entity
    }

    /// Every representation of the graph, in stable graph order.
    #[must_use]
    pub fn entries(&self) -> &[RepresentationVisibilityEntryV1] {
        &self.entries
    }

    /// The entry for one exact representation owner, when the ledger covers it.
    #[must_use]
    pub fn for_owner(&self, owner: &QualifiedOwnerV1) -> Option<&RepresentationVisibilityEntryV1> {
        self.entries.iter().find(|entry| &entry.owner == owner)
    }

    /// Whether this ledger governs one exact Page (some entry names that Page).
    ///
    /// A sibling Site, a same-named Page in another Site and a Page in another Entity are
    /// never covered by mistake: the comparison is the whole qualified owner.
    #[must_use]
    pub fn covers_page(&self, page_owner: &QualifiedOwnerV1) -> bool {
        page_owner.level() == EfsLevel::Page
            && self.entries.iter().any(|entry| {
                entry.owner.entity_id() == page_owner.entity_id()
                    && entry.owner.facet_id() == page_owner.facet_id()
                    && entry.owner.site_id() == page_owner.site_id()
                    && entry.owner.page_id() == page_owner.page_id()
            })
    }

    /// How many representations this ledger governs.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the ledger governs no representation at all.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// This ledger grants no publication authority. Always `false`.
    #[must_use]
    pub const fn grants_publication_authority(&self) -> bool {
        false
    }

    /// This ledger routes no URL. Always `false`.
    #[must_use]
    pub const fn routes_any_url(&self) -> bool {
        false
    }
}

/// Builds the one effective visibility ledger of an already validated EFS graph.
///
/// # Errors
///
/// Returns [`RepresentationVisibilityRefusalV1`] for a declaration that does not name exactly
/// one Page of this graph, a duplicate declaration, a Page without a declaration, a Page
/// without representations, an override outside its own Page, a duplicate override, or an
/// exceeded ceiling. Nothing is defaulted, repaired or inferred, and no sibling Site shares a
/// declaration.
pub fn build_representation_visibility_ledger_v1(
    graph: &EfsOwnershipGraph,
    declarations: &[PageVisibilityDeclarationV1],
) -> Result<RepresentationVisibilityLedgerV1, RepresentationVisibilityRefusalV1> {
    use RepresentationVisibilityRefusalKindV1 as Kind;

    if declarations.len() > MAX_VISIBILITY_PAGE_DECLARATIONS_V1 {
        return Err(RepresentationVisibilityRefusalV1::of(
            Kind::TooManyPageDeclarations,
        ));
    }
    for (position, declaration) in declarations.iter().enumerate() {
        if declaration.owner.level() != EfsLevel::Page {
            return Err(
                RepresentationVisibilityRefusalV1::of(Kind::PageOwnerNotAPage)
                    .with_owner(&declaration.owner),
            );
        }
        if declaration.overrides.len() > MAX_VISIBILITY_REPRESENTATION_OVERRIDES_V1 {
            return Err(RepresentationVisibilityRefusalV1::of(
                Kind::TooManyRepresentationOverrides,
            )
            .with_owner(&declaration.owner));
        }
        for (index, (locale, _)) in declaration.overrides.iter().enumerate() {
            if declaration.overrides[..index]
                .iter()
                .any(|(seen, _)| seen == locale)
            {
                return Err(RepresentationVisibilityRefusalV1::of(
                    Kind::DuplicateRepresentationOverride,
                )
                .with_owner(&declaration.owner));
            }
        }
        if declarations[..position]
            .iter()
            .any(|seen| seen.owner == declaration.owner)
        {
            return Err(
                RepresentationVisibilityRefusalV1::of(Kind::DuplicatePageDeclaration)
                    .with_owner(&declaration.owner),
            );
        }
    }

    let mut entries: Vec<RepresentationVisibilityEntryV1> = Vec::new();
    // A declaration that names no Page of this graph is foreign, and is named as such *before*
    // whole-graph coverage is demanded: the first level that does not resolve is the reported
    // cause, so a ledger from another Entity can never be misreported as merely incomplete.
    let mut page_owners: Vec<QualifiedOwnerV1> = Vec::new();
    for facet in graph.facets() {
        for site in facet.sites() {
            for page in site.pages() {
                page_owners.push(QualifiedOwnerV1::page(
                    graph.entity().clone(),
                    facet.id().clone(),
                    site.id().clone(),
                    page.id().clone(),
                ));
            }
        }
    }
    for declaration in declarations {
        if !page_owners.contains(&declaration.owner) {
            return Err(classify_unmatched_page(graph, declaration));
        }
    }
    for facet in graph.facets() {
        for site in facet.sites() {
            for page in site.pages() {
                let page_owner = QualifiedOwnerV1::page(
                    graph.entity().clone(),
                    facet.id().clone(),
                    site.id().clone(),
                    page.id().clone(),
                );
                if page.representations().is_empty() {
                    return Err(RepresentationVisibilityRefusalV1::of(
                        Kind::PageWithoutRepresentations,
                    )
                    .with_owner(&page_owner));
                }
                let Some(position) = declarations
                    .iter()
                    .position(|declaration| declaration.owner == page_owner)
                else {
                    return Err(RepresentationVisibilityRefusalV1::of(
                        Kind::MissingPageDeclaration,
                    )
                    .with_owner(&page_owner));
                };
                let declaration = &declarations[position];
                for (locale, _) in &declaration.overrides {
                    if !page.representations().iter().any(|member| member == locale) {
                        return Err(RepresentationVisibilityRefusalV1::of(
                            Kind::ForeignRepresentation,
                        )
                        .with_owner(&page_owner));
                    }
                }
                for locale in page.representations() {
                    let overridden = declaration
                        .overrides
                        .iter()
                        .find(|(candidate, _)| candidate == locale)
                        .map(|(_, visibility)| *visibility);
                    let (effective, origin) = match overridden {
                        Some(visibility) => (
                            visibility,
                            RepresentationVisibilityOriginV1::RepresentationOverride,
                        ),
                        None => (
                            declaration
                                .default
                                .unwrap_or(RepresentationVisibilityV1::Public),
                            RepresentationVisibilityOriginV1::PageDefault,
                        ),
                    };
                    entries.push(RepresentationVisibilityEntryV1 {
                        owner: QualifiedOwnerV1::representation(
                            graph.entity().clone(),
                            facet.id().clone(),
                            site.id().clone(),
                            page.id().clone(),
                            locale.clone(),
                        ),
                        effective,
                        origin,
                    });
                }
            }
        }
    }

    Ok(RepresentationVisibilityLedgerV1 {
        entity: graph.entity().clone(),
        entries,
    })
}

/// Names the first level at which a declaration failed to resolve inside the graph.
fn classify_unmatched_page(
    graph: &EfsOwnershipGraph,
    declaration: &PageVisibilityDeclarationV1,
) -> RepresentationVisibilityRefusalV1 {
    use RepresentationVisibilityRefusalKindV1 as Kind;

    let owner = &declaration.owner;
    if owner.entity_id() != graph.entity() {
        return RepresentationVisibilityRefusalV1::of(Kind::UnknownEntity).with_owner(owner);
    }
    let Some(facet_id) = owner.facet_id() else {
        return RepresentationVisibilityRefusalV1::of(Kind::WrongFacet).with_owner(owner);
    };
    let Some(facet) = graph.facets().iter().find(|facet| facet.id() == facet_id) else {
        return RepresentationVisibilityRefusalV1::of(Kind::WrongFacet).with_owner(owner);
    };
    let Some(site_id) = owner.site_id() else {
        return RepresentationVisibilityRefusalV1::of(Kind::WrongSite).with_owner(owner);
    };
    if !facet.sites().iter().any(|site| site.id() == site_id) {
        return RepresentationVisibilityRefusalV1::of(Kind::WrongSite).with_owner(owner);
    }
    RepresentationVisibilityRefusalV1::of(Kind::UnknownPage).with_owner(owner)
}
