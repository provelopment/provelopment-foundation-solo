//! `SOLO-WO-118 R22` — the manifest-bound, **nonpublishing** route/visibility policy preflight.
//!
//! This module consumes exactly one accepted R21 [`ManifestVisibilitySnapshotV1`] and derives a
//! closed, deterministic, pre-route *policy* for every localized representation and every related
//! Page/base entry. It enforces the visibility-dependent authoring rules whose inputs already exist
//! in the strict parsed candidate and the immutable R21 visibility ledger, and it refuses an unsafe
//! authored claim **before** any later route resolver, source reader or publisher could use it.
//!
//! # What this is not
//!
//! This is **not** a route ledger, a canonical URL, a collision-free route set, a validated Unicode
//! path, an accepted locale preference, a content decision or a public response. No path here is
//! normalized, percent-decoded, NFC-checked, lowercased, matched, indexed or servable; every
//! `route.path`, alias string, `entryPaths` entry and `base` value remains an **inert authored
//! string**. Full Unicode 18 / ICU4X / BCP47 / IDNA conformance, route collision detection, routing
//! and HTTP delivery remain `R04.A`/`R04.B`/`R04.D`/Stage 04 work and are deliberately **not**
//! simulated here with ASCII case folding, OS locale tables or ad-hoc decoding.
//!
//! # Binding rules enforced
//!
//! * **Unlisted is exact-case only and never discoverable.** An effectively unlisted representation
//!   must author **no** `route.caseMatching` (neither `sensitive` nor `insensitive`) and **zero**
//!   `route.aliases`; its only eligible future match policy is `exact-unlisted`, with
//!   `noindex,nofollow` intent. Unlisted is **not** private or authenticated, and this module neither
//!   hides nor serves it.
//! * **Public claims stay authored.** A public representation may omit `caseMatching` (future default
//!   `insensitive`) or author `insensitive`/`sensitive`; whether it was authored is preserved. Aliases
//!   remain declared and are never promoted into routes or used as conflict resolution.
//! * **Page entries need a public representation.** A Page with zero public representations may not
//!   author `entryPaths`; a mixed Page keeps them only as **pending inert** claims. No sibling
//!   fallback can make an unlisted-only Page eligible.
//! * **The landing must be exact.** An authored `base.landing` must name an existing exact qualified
//!   Page (and, when it names a locale, a declared representation of that same Page). Its availability
//!   intent is `eligible` only when that exact Page has a public representation, otherwise the honest
//!   `unavailable-no-public-representation` future not-found intent — never an invented public route
//!   and never a sibling substitute.
//! * **Missing translations resolve inside one Page only.** `strict` creates no fallback; `default`
//!   targets the enclosing Site's declared `defaultLocale` **of this same Page** only when that
//!   declaration is public, else a conditional future not-found intent (the Page stays valid); an
//!   explicit `locale` must name a declared, public representation of this same Page whose token
//!   satisfies the current R21 restricted locale profile.
//!
//! # Authority
//!
//! Every plan reports `publication_authority=false`, `route_authority=false`, `serves_http=false`,
//! `publishes_derived_output=false`, `release_authority=false` and `deployment_authority=false`.
//! There is no constructor that accepts a caller-supplied ledger, boolean, page set or replacement
//! graph, no port of any kind, and no mutable global, filesystem, network, process, environment or
//! clock dependency: the work is pure and bounded.

use std::fmt;

use solo_domain::{
    LocaleKeyV1, QualifiedOwnerV1, RepresentationVisibilityLedgerV1,
    RepresentationVisibilityOriginV1, RepresentationVisibilityV1,
};

use crate::manifest_visibility_bridge::ManifestVisibilitySnapshotV1;
use crate::publication_manifest::{
    DeclaredCaseMatching, DeclaredMissingTranslation, PageDeclarationV1,
};

/// The restricted locale profile mirrored from the accepted R21 bridge, as a stable token.
pub const MANIFEST_ROUTE_POLICY_LOCALE_PROFILE: &str = "canonical-shaped-base";

/// The closed future comparison-policy intent for one representation.
///
/// This is an *intent*, not a matcher: nothing is compiled, indexed or matched here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RouteComparisonIntentV1 {
    /// Public, `caseMatching` omitted or authored `insensitive`: a future insensitive claim.
    PublicInsensitive,
    /// Public, `caseMatching` authored `sensitive`: a future case-sensitive claim.
    PublicSensitive,
    /// Effectively unlisted: exact-case reachability only, never discoverable.
    ExactUnlisted,
}

impl RouteComparisonIntentV1 {
    /// The stable diagnostic token.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::PublicInsensitive => "public-insensitive",
            Self::PublicSensitive => "public-sensitive",
            Self::ExactUnlisted => "exact-unlisted",
        }
    }
}

/// One representation's pre-route policy record.
///
/// It carries typed facts only: the exact qualified owner, the ledger's effective visibility and its
/// provenance, the closed comparison intent, whether `caseMatching` was actually authored, the inert
/// alias count and the discovery/robot intent. It carries no path text, source reference, host,
/// canonical URL or authored free text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepresentationRoutePolicyV1 {
    owner: QualifiedOwnerV1,
    effective: RepresentationVisibilityV1,
    origin: RepresentationVisibilityOriginV1,
    comparison: RouteComparisonIntentV1,
    case_matching_authored: bool,
    alias_count: usize,
}

impl RepresentationRoutePolicyV1 {
    /// The exact qualified owner this policy governs.
    #[must_use]
    pub const fn owner(&self) -> &QualifiedOwnerV1 {
        &self.owner
    }

    /// The effective visibility, taken from the accepted R21 ledger.
    #[must_use]
    pub const fn effective(&self) -> RepresentationVisibilityV1 {
        self.effective
    }

    /// Where that effective visibility came from (`page-default` or `representation-override`).
    #[must_use]
    pub const fn origin(&self) -> RepresentationVisibilityOriginV1 {
        self.origin
    }

    /// The closed future comparison-policy intent.
    #[must_use]
    pub const fn comparison(&self) -> RouteComparisonIntentV1 {
        self.comparison
    }

    /// Whether `route.caseMatching` was actually authored. An unlisted record is always `false`.
    #[must_use]
    pub const fn case_matching_authored(&self) -> bool {
        self.case_matching_authored
    }

    /// How many aliases were declared. Inert metadata: no alias becomes a route.
    #[must_use]
    pub const fn alias_count(&self) -> usize {
        self.alias_count
    }

    /// Whether a future discovery surface may advertise this representation.
    #[must_use]
    pub const fn discoverable(&self) -> bool {
        matches!(self.effective, RepresentationVisibilityV1::Public)
    }

    /// The robot *intent* only. Never an HTTP header, route decision or access control.
    #[must_use]
    pub const fn robots_intent(&self) -> &'static str {
        if self.discoverable() {
            "index,follow"
        } else {
            "noindex,nofollow"
        }
    }
}

/// How a Page's authored `entryPaths` are treated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryPathsPolicyV1 {
    /// No `entryPaths` were authored.
    Absent,
    /// Authored, but only as **pending inert** claims; no locale is selected and no target exists.
    PendingInert {
        /// How many authored claims were retained inert.
        count: usize,
    },
}

impl EntryPathsPolicyV1 {
    /// The stable diagnostic token.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Absent => "absent",
            Self::PendingInert { .. } => "pending-inert",
        }
    }

    /// How many inert claims were retained.
    #[must_use]
    pub const fn count(self) -> usize {
        match self {
            Self::Absent => 0,
            Self::PendingInert { count } => count,
        }
    }
}

/// The resolved *policy* for one Page's authored `missingTranslation`.
///
/// No fallback is ever performed here: this records only which future behaviour the authored claim
/// would be allowed to have, inside this one Page.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MissingTranslationPolicyV1 {
    /// `strict`: no fallback; a missing translation is a not-found.
    Strict,
    /// `default`: this exact Page declares the enclosing Site's `defaultLocale` and it is `public`.
    DefaultPublicTarget {
        /// The exact same-Page locale that would serve the future fallback.
        locale: LocaleKeyV1,
    },
    /// `default`, but no public same-Page default representation exists now: a conditional future
    /// not-found intent. The Page itself stays valid.
    DefaultConditionalNotFound,
    /// Explicit `locale`: a declared, public representation of this same Page.
    ExplicitPublicTarget {
        /// The exact same-Page locale that would serve the future fallback.
        locale: LocaleKeyV1,
    },
}

impl MissingTranslationPolicyV1 {
    /// The stable diagnostic token.
    #[must_use]
    pub const fn token(&self) -> &'static str {
        match self {
            Self::Strict => "strict",
            Self::DefaultPublicTarget { .. } => "default-public-target",
            Self::DefaultConditionalNotFound => "default-conditional-not-found",
            Self::ExplicitPublicTarget { .. } => "explicit-public-target",
        }
    }

    /// The fallback locale this policy would use, when one exists exactly.
    #[must_use]
    pub const fn fallback_locale(&self) -> Option<&LocaleKeyV1> {
        match self {
            Self::DefaultPublicTarget { locale } | Self::ExplicitPublicTarget { locale } => {
                Some(locale)
            }
            Self::Strict | Self::DefaultConditionalNotFound => None,
        }
    }
}

/// One Page's pre-route policy record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageRoutePolicyV1 {
    owner: QualifiedOwnerV1,
    public_representation_count: usize,
    entry_paths: EntryPathsPolicyV1,
    missing_translation: MissingTranslationPolicyV1,
}

impl PageRoutePolicyV1 {
    /// The exact qualified Page owner.
    #[must_use]
    pub const fn owner(&self) -> &QualifiedOwnerV1 {
        &self.owner
    }

    /// How many of this Page's representations are effectively public.
    #[must_use]
    pub const fn public_representation_count(&self) -> usize {
        self.public_representation_count
    }

    /// How the authored `entryPaths` are treated.
    #[must_use]
    pub const fn entry_paths(&self) -> EntryPathsPolicyV1 {
        self.entry_paths
    }

    /// The resolved missing-translation policy.
    #[must_use]
    pub const fn missing_translation(&self) -> &MissingTranslationPolicyV1 {
        &self.missing_translation
    }
}

/// How an authored `base.landing` is treated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LandingAvailabilityV1 {
    /// No landing was authored: a future base entry is simply not found.
    Absent,
    /// The exact declared Page has at least one public representation: eligible for later
    /// consideration. Not a route, not a redirect and not a canonical address.
    EligibleExactPage,
    /// The exact declared Page exists but has no public representation: a future base-entry
    /// not-found intent. No sibling, locale or slug substitute is ever chosen.
    UnavailableNoPublicRepresentation,
}

impl LandingAvailabilityV1 {
    /// The stable diagnostic token.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Absent => "absent",
            Self::EligibleExactPage => "eligible-exact-page",
            Self::UnavailableNoPublicRepresentation => "unavailable-no-public-representation",
        }
    }
}

/// The plan's landing policy: what was declared, which exact Page it named and its availability.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LandingPolicyV1 {
    availability: LandingAvailabilityV1,
    page: Option<QualifiedOwnerV1>,
}

impl LandingPolicyV1 {
    /// The availability intent.
    #[must_use]
    pub const fn availability(&self) -> LandingAvailabilityV1 {
        self.availability
    }

    /// The exact qualified Page the landing named, when one was declared and resolved.
    #[must_use]
    pub const fn page(&self) -> Option<&QualifiedOwnerV1> {
        self.page.as_ref()
    }

    /// Whether a landing was authored at all.
    #[must_use]
    pub const fn declared(&self) -> bool {
        self.page.is_some()
    }
}

/// One closed, nonpublishing pre-route policy plan for an accepted manifest snapshot.
///
/// It exists only through [`build_manifest_route_visibility_policy_v1`], so it cannot be assembled
/// from caller-supplied booleans, a replacement ledger, a foreign graph or a hand-picked page set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestRouteVisibilityPolicyV1 {
    manifest_sha256: String,
    representations: Vec<RepresentationRoutePolicyV1>,
    pages: Vec<PageRoutePolicyV1>,
    landing: LandingPolicyV1,
}

impl ManifestRouteVisibilityPolicyV1 {
    /// The internal provenance pin of the supplied manifest bytes. Never an authority claim.
    #[must_use]
    pub fn manifest_sha256(&self) -> &str {
        &self.manifest_sha256
    }

    /// Every representation policy, in authored containment order.
    #[must_use]
    pub fn representations(&self) -> &[RepresentationRoutePolicyV1] {
        &self.representations
    }

    /// Every Page policy, in authored containment order.
    #[must_use]
    pub fn pages(&self) -> &[PageRoutePolicyV1] {
        &self.pages
    }

    /// The landing policy.
    #[must_use]
    pub const fn landing(&self) -> &LandingPolicyV1 {
        &self.landing
    }

    /// The restricted locale profile this preflight applied.
    #[must_use]
    pub const fn locale_profile(&self) -> &'static str {
        MANIFEST_ROUTE_POLICY_LOCALE_PROFILE
    }

    /// The policy for one exact representation owner, when it is governed.
    #[must_use]
    pub fn representation(&self, owner: &QualifiedOwnerV1) -> Option<&RepresentationRoutePolicyV1> {
        self.representations
            .iter()
            .find(|record| record.owner() == owner)
    }

    /// The policy for one exact Page owner, when it is governed.
    #[must_use]
    pub fn page(&self, owner: &QualifiedOwnerV1) -> Option<&PageRoutePolicyV1> {
        self.pages.iter().find(|record| record.owner() == owner)
    }

    /// This plan grants no publication authority. Always `false`.
    #[must_use]
    pub const fn grants_publication_authority(&self) -> bool {
        false
    }

    /// This plan grants no route authority. Always `false`.
    #[must_use]
    pub const fn grants_route_authority(&self) -> bool {
        false
    }

    /// This plan serves no HTTP response. Always `false`.
    #[must_use]
    pub const fn serves_http(&self) -> bool {
        false
    }

    /// This plan publishes no derived output. Always `false`.
    #[must_use]
    pub const fn publishes_derived_output(&self) -> bool {
        false
    }

    /// This plan authorizes no release. Always `false`.
    #[must_use]
    pub const fn grants_release_authority(&self) -> bool {
        false
    }

    /// This plan authorizes no deployment. Always `false`.
    #[must_use]
    pub const fn grants_deployment_authority(&self) -> bool {
        false
    }
}

/// Why a pre-route policy preflight refused the snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManifestRoutePolicyRefusalKindV1 {
    /// An effectively unlisted representation authored `route.caseMatching`.
    UnlistedCaseMatching,
    /// An effectively unlisted representation authored one or more `route.aliases`.
    UnlistedAliases,
    /// A Page with no public representation authored `entryPaths`.
    UnlistedOnlyEntryPaths,
    /// An authored landing does not name an existing exact qualified Page (or representation).
    LandingUnknownTarget,
    /// An explicit `missingTranslation` locale is not declared on this exact Page.
    MissingTranslationTargetUnknown,
    /// An explicit `missingTranslation` locale is declared here but is not public.
    MissingTranslationTargetUnlisted,
    /// An explicit `missingTranslation` locale is outside the restricted locale profile.
    MissingTranslationTargetNotCanonical,
    /// The parsed tree and the accepted ledger disagree on membership or cardinality.
    MembershipInconsistent,
}

impl ManifestRoutePolicyRefusalKindV1 {
    /// The stable, safe diagnostic token.
    #[must_use]
    pub const fn detail(self) -> &'static str {
        match self {
            Self::UnlistedCaseMatching => "unlisted-case-matching",
            Self::UnlistedAliases => "unlisted-aliases",
            Self::UnlistedOnlyEntryPaths => "unlisted-only-entry-paths",
            Self::LandingUnknownTarget => "landing-unknown-target",
            Self::MissingTranslationTargetUnknown => "missing-translation-target-unknown",
            Self::MissingTranslationTargetUnlisted => "missing-translation-target-unlisted",
            Self::MissingTranslationTargetNotCanonical => {
                "missing-translation-target-not-canonical"
            }
            Self::MembershipInconsistent => "membership-inconsistent",
        }
    }
}

/// One refused pre-route policy preflight, attributed to a safe structural location only.
///
/// The location is a fixed dotted field path with indices, such as
/// `entity.facets[0].sites[1].pages[0].representations[2].route.caseMatching`. It never contains an
/// authored value, a route string, a source reference, a property value or a secret.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestRoutePolicyRefusalV1 {
    kind: ManifestRoutePolicyRefusalKindV1,
    location: String,
}

impl ManifestRoutePolicyRefusalV1 {
    /// One refusal of a closed kind at a safe structural location.
    #[must_use]
    fn at(kind: ManifestRoutePolicyRefusalKindV1, location: &str) -> Self {
        Self {
            kind,
            location: location.to_owned(),
        }
    }

    /// The closed cause.
    #[must_use]
    pub const fn kind(&self) -> ManifestRoutePolicyRefusalKindV1 {
        self.kind
    }

    /// The safe structural location this refusal concerns.
    #[must_use]
    pub fn location(&self) -> &str {
        &self.location
    }

    /// The stable diagnostic token.
    #[must_use]
    pub const fn detail(&self) -> &'static str {
        self.kind.detail()
    }

    /// The stable refusal code: this preflight has exactly one refusal family.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        "PREFLIGHT_MANIFEST_ROUTE_POLICY_INVALID"
    }
}

impl fmt::Display for ManifestRoutePolicyRefusalV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} at {}", self.detail(), self.location)
    }
}

/// Derives the closed, nonpublishing pre-route policy plan for one accepted R21 snapshot.
///
/// The snapshot is the **only** input: there is no parameter for a ledger, a boolean, a page set or a
/// graph, and no port of any kind. Traversal follows the parsed Entity's authored containment order,
/// and every representation's effective visibility is looked up in the accepted R21 ledger by fully
/// qualified owner — never recomputed from the authored strings. Every Page and representation must
/// have exactly one policy record; an impossible mismatch refuses the whole plan, so no partial
/// record set can escape.
///
/// # Errors
///
/// Returns one [`ManifestRoutePolicyRefusalV1`] for an unlisted `caseMatching`, an unlisted alias, a
/// fully unlisted Page's `entryPaths`, an inexact landing target, a missing/unlisted/non-canonical
/// explicit missing-translation target, or a membership/cardinality inconsistency.
pub fn build_manifest_route_visibility_policy_v1(
    snapshot: &ManifestVisibilitySnapshotV1,
) -> Result<ManifestRouteVisibilityPolicyV1, ManifestRoutePolicyRefusalV1> {
    use ManifestRoutePolicyRefusalKindV1 as Kind;

    let ledger = snapshot.ledger();
    let entity = snapshot.parsed().entity();
    let entity_id = entity.id().clone();

    let mut representations: Vec<RepresentationRoutePolicyV1> = Vec::new();
    let mut pages: Vec<PageRoutePolicyV1> = Vec::new();
    let mut public_by_page: Vec<(QualifiedOwnerV1, usize)> = Vec::new();

    for (facet_index, facet) in entity.facets().iter().enumerate() {
        let facet_id = facet.id().clone();
        for (site_index, site) in facet.sites().iter().enumerate() {
            let site_id = site.id().clone();
            for (page_index, page) in site.pages().iter().enumerate() {
                let page_location =
                    format!("entity.facets[{facet_index}].sites[{site_index}].pages[{page_index}]");
                let page_owner = QualifiedOwnerV1::page(
                    entity_id.clone(),
                    facet_id.clone(),
                    site_id.clone(),
                    page.id().clone(),
                );
                let mut public_count = 0_usize;
                for (representation_index, representation) in
                    page.representations().iter().enumerate()
                {
                    let representation_location =
                        format!("{page_location}.representations[{representation_index}]");
                    let locale = LocaleKeyV1::from_already_canonical(representation.locale())
                        .map_err(|_| {
                            ManifestRoutePolicyRefusalV1::at(
                                Kind::MembershipInconsistent,
                                &format!("{representation_location}.locale"),
                            )
                        })?;
                    let owner = QualifiedOwnerV1::representation(
                        entity_id.clone(),
                        facet_id.clone(),
                        site_id.clone(),
                        page.id().clone(),
                        locale,
                    );
                    // The effective state comes from the accepted ledger, never from the strings.
                    let entry = ledger.for_owner(&owner).ok_or_else(|| {
                        ManifestRoutePolicyRefusalV1::at(
                            Kind::MembershipInconsistent,
                            &representation_location,
                        )
                    })?;
                    let effective = entry.effective();
                    let route = representation.route();
                    let authored_case = route.case_matching();
                    let alias_count = route.aliases().len();

                    let comparison = match effective {
                        RepresentationVisibilityV1::Unlisted => {
                            if authored_case.is_some() {
                                return Err(ManifestRoutePolicyRefusalV1::at(
                                    Kind::UnlistedCaseMatching,
                                    &format!("{representation_location}.route.caseMatching"),
                                ));
                            }
                            if alias_count != 0 {
                                return Err(ManifestRoutePolicyRefusalV1::at(
                                    Kind::UnlistedAliases,
                                    &format!("{representation_location}.route.aliases"),
                                ));
                            }
                            RouteComparisonIntentV1::ExactUnlisted
                        }
                        RepresentationVisibilityV1::Public => {
                            public_count += 1;
                            match authored_case {
                                Some(DeclaredCaseMatching::Sensitive) => {
                                    RouteComparisonIntentV1::PublicSensitive
                                }
                                Some(DeclaredCaseMatching::Insensitive) | None => {
                                    RouteComparisonIntentV1::PublicInsensitive
                                }
                            }
                        }
                    };

                    representations.push(RepresentationRoutePolicyV1 {
                        owner,
                        effective,
                        origin: entry.origin(),
                        comparison,
                        case_matching_authored: authored_case.is_some(),
                        alias_count,
                    });
                }
                // PAGE_TAIL
                let entry_paths = if page.entry_paths().is_empty() {
                    EntryPathsPolicyV1::Absent
                } else {
                    if public_count == 0 {
                        return Err(ManifestRoutePolicyRefusalV1::at(
                            Kind::UnlistedOnlyEntryPaths,
                            &format!("{page_location}.entryPaths"),
                        ));
                    }
                    EntryPathsPolicyV1::PendingInert {
                        count: page.entry_paths().len(),
                    }
                };

                let missing_translation = resolve_missing_translation(
                    snapshot,
                    page,
                    site.default_locale(),
                    &page_owner,
                    &page_location,
                )?;

                pages.push(PageRoutePolicyV1 {
                    owner: page_owner.clone(),
                    public_representation_count: public_count,
                    entry_paths,
                    missing_translation,
                });
                public_by_page.push((page_owner, public_count));
            }
        }
    }
    // COVERAGE_AND_LANDING
    if representations.len() != ledger.len()
        || pages.len() != snapshot.page_count()
        || representations.len() != snapshot.representation_count()
    {
        return Err(ManifestRoutePolicyRefusalV1::at(
            Kind::MembershipInconsistent,
            "entity",
        ));
    }

    let landing = resolve_landing(snapshot, &public_by_page)?;

    Ok(ManifestRouteVisibilityPolicyV1 {
        manifest_sha256: snapshot.manifest_sha256(),
        representations,
        pages,
        landing,
    })
}

/// Resolves one Page's authored `missingTranslation` inside that Page only.
///
/// `strict` creates no fallback. `default` targets the enclosing Site's declared `defaultLocale`
/// only when **this** Page declares it and it is public; otherwise it records a conditional future
/// not-found intent and leaves the Page valid — a visitor may simply never request that locale.
/// An explicit `locale` must be canonical-shaped for the restricted profile and must name a
/// declared, public representation of this same Page. No other Page, sibling Site or locale is ever
/// consulted, and no URL, redirect, `Vary` or header is produced.
fn resolve_missing_translation(
    snapshot: &ManifestVisibilitySnapshotV1,
    page: &PageDeclarationV1,
    site_default_locale: &str,
    page_owner: &QualifiedOwnerV1,
    page_location: &str,
) -> Result<MissingTranslationPolicyV1, ManifestRoutePolicyRefusalV1> {
    use ManifestRoutePolicyRefusalKindV1 as Kind;

    let ledger = snapshot.ledger();
    let fallback_location = format!("{page_location}.missingTranslation.fallbackLocale");

    match page.missing_translation() {
        DeclaredMissingTranslation::Strict => Ok(MissingTranslationPolicyV1::Strict),
        DeclaredMissingTranslation::Default => {
            match effective_of_locale(ledger, page, site_default_locale, page_owner) {
                Some(RepresentationVisibilityV1::Public) => {
                    let locale =
                        LocaleKeyV1::from_already_canonical(site_default_locale).map_err(|_| {
                            ManifestRoutePolicyRefusalV1::at(
                                Kind::MembershipInconsistent,
                                &fallback_location,
                            )
                        })?;
                    Ok(MissingTranslationPolicyV1::DefaultPublicTarget { locale })
                }
                Some(RepresentationVisibilityV1::Unlisted) | None => {
                    Ok(MissingTranslationPolicyV1::DefaultConditionalNotFound)
                }
            }
        }
        DeclaredMissingTranslation::Locale(token) => {
            if !is_canonical_shaped_base_profile(token) {
                return Err(ManifestRoutePolicyRefusalV1::at(
                    Kind::MissingTranslationTargetNotCanonical,
                    &fallback_location,
                ));
            }
            match effective_of_locale(ledger, page, token, page_owner) {
                Some(RepresentationVisibilityV1::Public) => {
                    let locale = LocaleKeyV1::from_already_canonical(token).map_err(|_| {
                        ManifestRoutePolicyRefusalV1::at(
                            Kind::MissingTranslationTargetNotCanonical,
                            &fallback_location,
                        )
                    })?;
                    Ok(MissingTranslationPolicyV1::ExplicitPublicTarget { locale })
                }
                Some(RepresentationVisibilityV1::Unlisted) => {
                    Err(ManifestRoutePolicyRefusalV1::at(
                        Kind::MissingTranslationTargetUnlisted,
                        &fallback_location,
                    ))
                }
                None => Err(ManifestRoutePolicyRefusalV1::at(
                    Kind::MissingTranslationTargetUnknown,
                    &fallback_location,
                )),
            }
        }
    }
}

/// The effective visibility of one exact locale **of this Page**, taken from the accepted ledger.
///
/// Returns `None` when this Page declares no such representation. A locale declared only on another
/// Page — including a same-named sibling — is never consulted.
fn effective_of_locale(
    ledger: &RepresentationVisibilityLedgerV1,
    page: &PageDeclarationV1,
    locale: &str,
    page_owner: &QualifiedOwnerV1,
) -> Option<RepresentationVisibilityV1> {
    let declared = page
        .representations()
        .iter()
        .find(|representation| representation.locale() == locale)?;
    let owner = QualifiedOwnerV1::representation(
        page_owner.entity_id().clone(),
        page_owner.facet_id()?.clone(),
        page_owner.site_id()?.clone(),
        page_owner.page_id()?.clone(),
        LocaleKeyV1::from_already_canonical(declared.locale()).ok()?,
    );
    ledger.for_owner(&owner).map(|entry| entry.effective())
}

/// Resolves the authored `base.landing` into an availability **intent** only.
///
/// The landing must name an exact qualified Page that exists in this snapshot's parsed tree; a
/// declared locale must additionally name a declared representation of that same Page. Nothing is
/// inferred from a slug, an ID coincidence, a locale, a Site default or a title, and no base URL,
/// canonical address or redirect target is produced. A declared but entirely unlisted Page yields
/// the honest future base-entry not-found intent, never a sibling substitute.
fn resolve_landing(
    snapshot: &ManifestVisibilitySnapshotV1,
    public_by_page: &[(QualifiedOwnerV1, usize)],
) -> Result<LandingPolicyV1, ManifestRoutePolicyRefusalV1> {
    use ManifestRoutePolicyRefusalKindV1 as Kind;

    let Some(target) = snapshot.parsed().base().landing() else {
        return Ok(LandingPolicyV1 {
            availability: LandingAvailabilityV1::Absent,
            page: None,
        });
    };

    let entity = snapshot.parsed().entity();
    let facet = entity
        .facets()
        .iter()
        .find(|candidate| candidate.id().as_str() == target.facet());
    let site = facet.and_then(|facet| {
        facet
            .sites()
            .iter()
            .find(|candidate| candidate.id().as_str() == target.site())
    });
    let page = site.and_then(|site| {
        site.pages()
            .iter()
            .find(|candidate| candidate.id().as_str() == target.page())
    });
    let (Some(facet), Some(site), Some(page)) = (facet, site, page) else {
        return Err(ManifestRoutePolicyRefusalV1::at(
            Kind::LandingUnknownTarget,
            "base.landing",
        ));
    };
    if target.entity() != entity.id().as_str() {
        return Err(ManifestRoutePolicyRefusalV1::at(
            Kind::LandingUnknownTarget,
            "base.landing.entity",
        ));
    }
    let locale_unbound = target.locale().is_some_and(|locale| {
        !page
            .representations()
            .iter()
            .any(|representation| representation.locale() == locale)
    });
    if locale_unbound {
        return Err(ManifestRoutePolicyRefusalV1::at(
            Kind::LandingUnknownTarget,
            "base.landing.locale",
        ));
    }

    let owner = QualifiedOwnerV1::page(
        entity.id().clone(),
        facet.id().clone(),
        site.id().clone(),
        page.id().clone(),
    );
    let Some((_, public_count)) = public_by_page
        .iter()
        .find(|(candidate, _)| *candidate == owner)
    else {
        return Err(ManifestRoutePolicyRefusalV1::at(
            Kind::MembershipInconsistent,
            "base.landing",
        ));
    };
    let availability = if *public_count == 0 {
        LandingAvailabilityV1::UnavailableNoPublicRepresentation
    } else {
        LandingAvailabilityV1::EligibleExactPage
    };
    Ok(LandingPolicyV1 {
        availability,
        page: Some(owner),
    })
}

/// Mirrors the accepted R21 restricted locale profile exactly: lowercase two/three-letter language,
/// optional Title-cased four-letter script and a mandatory uppercase two-letter or three-digit
/// region, with no further subtag. Deliberately narrower than valid BCP47; nothing is case-folded,
/// repaired or inferred.
fn is_canonical_shaped_base_profile(candidate: &str) -> bool {
    let mut subtags = candidate.split('-');
    let Some(language) = subtags.next() else {
        return false;
    };
    if !(2..=3).contains(&language.len()) || !language.bytes().all(|byte| byte.is_ascii_lowercase())
    {
        return false;
    }
    let Some(second) = subtags.next() else {
        return false;
    };
    let region_subtag = if is_title_cased_script(second) {
        let Some(region_subtag) = subtags.next() else {
            return false;
        };
        region_subtag
    } else {
        second
    };
    if subtags.next().is_some() {
        return false;
    }
    is_region_subtag(region_subtag)
}

/// A four-letter script with an uppercase first letter and lowercase remainder.
fn is_title_cased_script(subtag: &str) -> bool {
    let bytes = subtag.as_bytes();
    bytes.len() == 4
        && bytes[0].is_ascii_uppercase()
        && bytes[1..].iter().all(u8::is_ascii_lowercase)
}

/// An uppercase two-letter region or a three-digit numeric region.
fn is_region_subtag(subtag: &str) -> bool {
    let bytes = subtag.as_bytes();
    (bytes.len() == 2 && bytes.iter().all(u8::is_ascii_uppercase))
        || (bytes.len() == 3 && bytes.iter().all(u8::is_ascii_digit))
}
