//! `SOLO-WO-118 R21` — the inert bridge from the accepted strict publication manifest to the
//! accepted typed EFS ownership graph and R20 representation-visibility ledger.
//!
//! This module adds **no parser**. It calls the accepted
//! [`crate::publication_manifest::parse_manifest_bytes`] exactly once on the supplied bytes and
//! converts that parser's *complete* parsed containment tree into the separately accepted R02
//! [`solo_domain::EfsOwnershipGraph`], then into the R20
//! [`solo_domain::RepresentationVisibilityLedgerV1`]. Nothing is re-parsed, re-derived from JSON,
//! defaulted or repaired.
//!
//! # What a snapshot is, and is not
//!
//! A [`ManifestVisibilitySnapshotV1`] is a **nonpublishing** value. It owns the original parsed
//! candidate, one graph built from that candidate's complete tree, one complete visibility ledger
//! built from the same Pages, and the checked SHA-256 of the supplied bytes as a provenance pin.
//! Every authority claim on it is a constant `false`: it grants no publication, route, HTTP,
//! derived-output, release or deployment authority, and it carries no source bytes, no resolved
//! body, no URL and no activated snapshot.
//!
//! Being *syntactically accepted by the parser* is not semantic validation. The parsed `base`,
//! route claims, aliases, missing-translation rules, `sourceCandidates`, inert `properties` and
//! labels keep exactly their existing syntactic checks and remain expressly **pending** semantic
//! conformance. This bridge neither re-validates nor discards them, and never claims they are
//! production-ready.
//!
//! # Deliberately restricted locale profile
//!
//! [`solo_domain::LocaleKeyV1::from_already_canonical`] validates membership *shape* only; it is
//! not a BCP47 validator. This bridge therefore admits only an **unambiguous canonical-shaped base
//! profile** — lowercase two/three-letter language, optional Title-cased four-letter script, and a
//! mandatory uppercase two-letter or three-digit region, for example `en-001`, `de-DE`, `id-ID`,
//! `zh-Hans-CN`. Mixed case, bare languages, extensions, variants and ambiguous spellings are
//! refused as *locale-not-yet-validated*, with no case-folding and no region inference. This is
//! intentionally **narrower** than valid BCP47: full ICU4X/Unicode-18 canonicalisation, locale
//! aliases and complete equivalence remain separate `R04.B`/`E05` work, and a currently
//! unsupported-but-valid variant is a safe refusal, not a claim that the standard forbids it.

use std::fmt;

use solo_application::{MediaWorkerDigestV1, sha256_media_worker_bytes_v1};
use solo_domain::{
    EfsOwnershipDraft, EfsOwnershipGraph, FacetDraft, GraphicCascadeInputV1, LocaleKeyV1,
    PageDraft, PageVisibilityDeclarationV1, QualifiedOwnerV1, RepresentationVisibilityLedgerV1,
    RepresentationVisibilityV1, SiteDraft, build_representation_visibility_ledger_v1,
};

use crate::publication_manifest::{
    DeclaredVisibility, ManifestError, ManifestErrorKind, ParsedManifestCandidateV1,
    parse_manifest_bytes,
};

/// The restricted locale profile this bridge admits, as a stable diagnostic token.
pub const MANIFEST_VISIBILITY_LOCALE_PROFILE: &str = "canonical-shaped-base";

/// This bridge grants no publication authority. Always `false`.
pub const MANIFEST_VISIBILITY_GRANTS_PUBLICATION_AUTHORITY: bool = false;

/// Why a manifest-to-visibility conversion was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManifestVisibilityBridgeRefusalKindV1 {
    /// The accepted parser refused the bytes; its exact cause is preserved.
    Manifest(ManifestErrorKind),
    /// A declared locale (or the Site `defaultLocale`) is outside the restricted profile.
    LocaleProfileRefused,
    /// The accepted EFS draft refused the parsed tree.
    GraphRefused,
    /// The accepted R20 ledger refused the built graph and its declarations.
    LedgerRefused,
    /// The parsed tree and the built graph disagree on cardinality or coverage.
    MembershipInconsistent,
    /// The candidate's validated EFS membership is not this manifest's membership.
    ScopeMismatch,
}

impl ManifestVisibilityBridgeRefusalKindV1 {
    /// The stable, safe diagnostic token.
    #[must_use]
    pub const fn detail(self) -> &'static str {
        match self {
            Self::Manifest(kind) => kind.detail(),
            Self::LocaleProfileRefused => "locale-not-yet-validated",
            Self::GraphRefused => "graph-refused",
            Self::LedgerRefused => "ledger-refused",
            Self::MembershipInconsistent => "membership-inconsistent",
            Self::ScopeMismatch => "scope-mismatch",
        }
    }
}

/// One refused conversion, attributed to a safe structural location only.
///
/// The location is a fixed dotted field path such as
/// `entity.facets[0].sites[1].pages[0].representations[2].locale`. It never contains an authored
/// value, a source byte, a private property value or a secret, and it is never presented as a
/// public response.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestVisibilityBridgeRefusalV1 {
    kind: ManifestVisibilityBridgeRefusalKindV1,
    location: String,
}

impl ManifestVisibilityBridgeRefusalV1 {
    /// One refusal of a closed kind at a safe structural location.
    #[must_use]
    pub fn at(kind: ManifestVisibilityBridgeRefusalKindV1, location: &str) -> Self {
        Self {
            kind,
            location: location.to_owned(),
        }
    }

    /// The closed cause.
    #[must_use]
    pub const fn kind(&self) -> ManifestVisibilityBridgeRefusalKindV1 {
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

    /// The stable refusal code: this operation has exactly one refusal family.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        "PREFLIGHT_MANIFEST_VISIBILITY_INVALID"
    }
}

impl fmt::Display for ManifestVisibilityBridgeRefusalV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} at {}", self.detail(), self.location)
    }
}

/// Wraps one accepted-parser refusal without erasing its exact kind or safe location.
fn from_manifest_error(error: &ManifestError) -> ManifestVisibilityBridgeRefusalV1 {
    ManifestVisibilityBridgeRefusalV1::at(
        ManifestVisibilityBridgeRefusalKindV1::Manifest(error.kind()),
        error.location(),
    )
}

/// One nonpublishing manifest visibility snapshot.
///
/// The value exists only through [`parse_manifest_visibility_snapshot_v1`], so a foreign graph or
/// ledger can never be presented as manifest-derived. It exposes read-only typed facts only: there
/// is no public constructor, no setter, no interior mutability and no authority flag that can be
/// raised.
#[derive(Debug, Clone, PartialEq)]
pub struct ManifestVisibilitySnapshotV1 {
    parsed: ParsedManifestCandidateV1,
    graph: EfsOwnershipGraph,
    ledger: RepresentationVisibilityLedgerV1,
    manifest_sha256: MediaWorkerDigestV1,
    manifest_bytes: usize,
}

impl ManifestVisibilitySnapshotV1 {
    /// The original parsed candidate the accepted parser produced — never a rebuilt look-alike.
    #[must_use]
    pub const fn parsed(&self) -> &ParsedManifestCandidateV1 {
        &self.parsed
    }

    /// The ownership graph built from that candidate's complete containment tree.
    #[must_use]
    pub const fn graph(&self) -> &EfsOwnershipGraph {
        &self.graph
    }

    /// The complete R20 visibility ledger built from those same Pages.
    #[must_use]
    pub const fn ledger(&self) -> &RepresentationVisibilityLedgerV1 {
        &self.ledger
    }

    /// The checked SHA-256 of the **supplied** manifest bytes, as a provenance identity.
    ///
    /// It is an internal pin, never a signed author grant and never source or publication approval.
    #[must_use]
    pub fn manifest_sha256(&self) -> String {
        self.manifest_sha256.to_hex()
    }

    /// The exact byte length of the supplied manifest.
    #[must_use]
    pub const fn manifest_bytes(&self) -> usize {
        self.manifest_bytes
    }

    /// How many EFS nodes the parsed candidate declared.
    #[must_use]
    pub const fn node_count(&self) -> usize {
        self.parsed.node_count()
    }

    /// How many Pages the parsed candidate declared.
    #[must_use]
    pub const fn page_count(&self) -> usize {
        self.parsed.page_count()
    }

    /// How many representations the parsed candidate declared.
    #[must_use]
    pub const fn representation_count(&self) -> usize {
        self.parsed.representation_count()
    }

    /// The restricted locale profile this conversion applied.
    #[must_use]
    pub const fn locale_profile(&self) -> &'static str {
        MANIFEST_VISIBILITY_LOCALE_PROFILE
    }

    /// This snapshot grants no publication authority. Always `false`.
    #[must_use]
    pub const fn grants_publication_authority(&self) -> bool {
        false
    }

    /// This snapshot routes no URL. Always `false`.
    #[must_use]
    pub const fn routes_any_url(&self) -> bool {
        false
    }

    /// This snapshot serves no HTTP response. Always `false`.
    #[must_use]
    pub const fn serves_http(&self) -> bool {
        false
    }

    /// This snapshot publishes no derived output. Always `false`.
    #[must_use]
    pub const fn publishes_derived_output(&self) -> bool {
        false
    }

    /// This snapshot authorizes no release. Always `false`.
    #[must_use]
    pub const fn grants_release_authority(&self) -> bool {
        false
    }

    /// This snapshot authorizes no deployment. Always `false`.
    #[must_use]
    pub const fn grants_deployment_authority(&self) -> bool {
        false
    }
}

/// The single accepted result of a successful EFS scope comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManifestEfsScopeV1 {
    /// The candidate's validated EFS membership is exactly this manifest's membership.
    ExactEfsMembership,
}

impl ManifestEfsScopeV1 {
    /// The stable diagnostic token.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::ExactEfsMembership => "exact-EFS-membership",
        }
    }
}

/// Parses the supplied manifest bytes **once** and converts the complete parsed tree.
///
/// The accepted strict, duplicate-aware parser is the only parser involved. Every declared Facet,
/// Site, Page and representation of the parsed Entity is visited in authored containment order, the
/// accepted [`EfsOwnershipDraft`] builds the graph (so duplicate, cardinality and identity refusals
/// stay the accepted ones), and exactly one Page declaration per declared Page feeds the accepted
/// R20 ledger builder together with that exact graph. There is no defaulted `main`, no invented
/// Page, no inferred locale and no partial graph/ledger pair.
///
/// # Errors
///
/// Returns [`ManifestVisibilityBridgeRefusalV1`] preserving the parser's exact
/// [`ManifestErrorKind`], or one closed bridge cause: a locale outside the restricted profile, a
/// refused graph, a refused ledger, or a cardinality/membership inconsistency.
pub fn parse_manifest_visibility_snapshot_v1(
    supplied_manifest_bytes: &[u8],
) -> Result<ManifestVisibilitySnapshotV1, ManifestVisibilityBridgeRefusalV1> {
    use ManifestVisibilityBridgeRefusalKindV1 as Kind;

    let parsed = parse_manifest_bytes(supplied_manifest_bytes)
        .map_err(|error| from_manifest_error(&error))?;

    let entity = parsed.entity();
    let mut draft = EfsOwnershipDraft::new(entity.id().clone());
    let mut declarations: Vec<PageVisibilityDeclarationV1> = Vec::new();

    for (facet_index, facet) in entity.facets().iter().enumerate() {
        let mut facet_draft = FacetDraft::new(facet.id().clone());
        for (site_index, site) in facet.sites().iter().enumerate() {
            // The Site `defaultLocale` is profile-checked, but it never invents a representation and
            // never adds a cross-Page membership policy of its own.
            restricted_locale(site.default_locale()).map_err(|()| {
                ManifestVisibilityBridgeRefusalV1::at(
                    Kind::LocaleProfileRefused,
                    &format!("entity.facets[{facet_index}].sites[{site_index}].defaultLocale"),
                )
            })?;
            let mut site_draft = SiteDraft::new(site.id().clone());
            for (page_index, page) in site.pages().iter().enumerate() {
                let page_location =
                    format!("entity.facets[{facet_index}].sites[{site_index}].pages[{page_index}]");
                let page_owner = QualifiedOwnerV1::page(
                    entity.id().clone(),
                    facet.id().clone(),
                    site.id().clone(),
                    page.id().clone(),
                );
                // `visibility_default()` already resolves an omitted Page default to `public`, so
                // this is the effective default; whether it was authored stays unknowable here.
                let mut declaration = PageVisibilityDeclarationV1::new(
                    page_owner,
                    Some(visibility_of(page.visibility_default())),
                )
                .map_err(|_| {
                    ManifestVisibilityBridgeRefusalV1::at(Kind::LedgerRefused, &page_location)
                })?;
                let mut page_draft = PageDraft::new(page.id().clone());
                for (representation_index, representation) in
                    page.representations().iter().enumerate()
                {
                    let representation_location =
                        format!("{page_location}.representations[{representation_index}]");
                    let locale = restricted_locale(representation.locale()).map_err(|()| {
                        ManifestVisibilityBridgeRefusalV1::at(
                            Kind::LocaleProfileRefused,
                            &format!("{representation_location}.locale"),
                        )
                    })?;
                    page_draft.add_representation(locale.clone());
                    // An absent override stays absent; only a real declaration becomes one.
                    if let Some(declared) = representation.visibility() {
                        declaration = declaration
                            .with_override(locale, visibility_of(declared))
                            .map_err(|_| {
                                ManifestVisibilityBridgeRefusalV1::at(
                                    Kind::LedgerRefused,
                                    &format!("{representation_location}.visibility"),
                                )
                            })?;
                    }
                }
                site_draft.add_page(page_draft);
                declarations.push(declaration);
            }
            facet_draft.add_site(site_draft);
        }
        draft.add_facet(facet_draft);
    }

    let graph = draft
        .build()
        .map_err(|_| ManifestVisibilityBridgeRefusalV1::at(Kind::GraphRefused, "entity"))?;
    let ledger =
        build_representation_visibility_ledger_v1(&graph, &declarations).map_err(|refusal| {
            let location = refusal.owner().map_or_else(
                || "entity".to_owned(),
                |owner| owner_location(&graph, owner),
            );
            ManifestVisibilityBridgeRefusalV1::at(Kind::LedgerRefused, &location)
        })?;

    // The conversion must cover the parsed tree exactly: no silent partial pair, no dropped Page.
    if ledger.len() != parsed.representation_count()
        || graph_page_count(&graph) != parsed.page_count()
    {
        return Err(ManifestVisibilityBridgeRefusalV1::at(
            Kind::MembershipInconsistent,
            "entity",
        ));
    }

    Ok(ManifestVisibilitySnapshotV1 {
        parsed,
        graph,
        ledger,
        manifest_sha256: sha256_media_worker_bytes_v1(supplied_manifest_bytes),
        manifest_bytes: supplied_manifest_bytes.len(),
    })
}

/// Checks that one already validated candidate's EFS membership **is** this manifest's membership.
///
/// The comparison is the complete, fully qualified, ordered Entity → Facet → Site → Page →
/// representation membership — never only the Entity identifier, a target owner, a count, a shared
/// Page identifier or a prefix. A sibling Site's `home` and an identically named Page in another
/// Facet are different owners and never pass as the same one. This is deliberately narrow: it
/// binds **EFS ownership scope only**. It does not claim that the candidate's inert properties,
/// source bytes, routes or any other manifest field were derived from these bytes, and it grants no
/// authority beyond the existing `public`/`unlisted` typed facts.
///
/// # Errors
///
/// Returns [`ManifestVisibilityBridgeRefusalKindV1::ScopeMismatch`] for a foreign Entity, a
/// renamed or missing Facet/Site/Page, a changed representation locale, a missing sibling or any
/// extra owner. The check is pure, so it refuses before any reader or worker can be reached.
pub fn check_manifest_efs_scope_v1(
    snapshot: &ManifestVisibilitySnapshotV1,
    candidate: &GraphicCascadeInputV1,
) -> Result<ManifestEfsScopeV1, ManifestVisibilityBridgeRefusalV1> {
    let expected = graph_owners_in_order(snapshot.graph());
    if expected.as_slice() != candidate.validated().owners() {
        return Err(ManifestVisibilityBridgeRefusalV1::at(
            ManifestVisibilityBridgeRefusalKindV1::ScopeMismatch,
            "entity",
        ));
    }
    Ok(ManifestEfsScopeV1::ExactEfsMembership)
}

/// Maps one declared visibility onto the closed R20 two-state type.
///
/// `internal`/`private` never appears: the accepted parser admits only these two literals for a
/// Page `visibilityDefault` or a representation `visibility`.
const fn visibility_of(declared: DeclaredVisibility) -> RepresentationVisibilityV1 {
    match declared {
        DeclaredVisibility::Public => RepresentationVisibilityV1::Public,
        DeclaredVisibility::Unlisted => RepresentationVisibilityV1::Unlisted,
    }
}

/// Admits only the unambiguous canonical-shaped base profile, then the accepted membership key.
///
/// The profile is lowercase two/three-letter language, optional Title-cased four-letter script and
/// a mandatory uppercase two-letter or three-digit region. Nothing is case-folded, inferred or
/// repaired: a mixed-case, bare-language, extended or variant tag is refused as
/// `locale-not-yet-validated`.
fn restricted_locale(candidate: &str) -> Result<LocaleKeyV1, ()> {
    if !is_canonical_shaped_base_profile(candidate) {
        return Err(());
    }
    LocaleKeyV1::from_already_canonical(candidate).map_err(|_| ())
}

/// The restricted profile predicate. Deliberately narrower than valid BCP47.
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

/// Every owner of one graph, parent first, in the accepted deterministic graph order.
///
/// This mirrors the accepted domain walk exactly (Entity, then per Facet its Facet and Sites, then
/// per Site its Pages, then per Page its representations), which is the order
/// `ValidatedPropertyInputV1::owners()` produces. It is used only to compare membership, never to
/// create a competing identity model.
fn graph_owners_in_order(graph: &EfsOwnershipGraph) -> Vec<QualifiedOwnerV1> {
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

/// How many Pages one graph declares.
fn graph_page_count(graph: &EfsOwnershipGraph) -> usize {
    graph
        .facets()
        .iter()
        .flat_map(|facet| facet.sites())
        .map(|site| site.pages().len())
        .sum()
}

/// The safe structural path of one owner inside the graph, indices only.
///
/// It never contains an authored identifier, so a refusal can be attributed in a log without
/// leaking any authored value.
fn owner_location(graph: &EfsOwnershipGraph, owner: &QualifiedOwnerV1) -> String {
    for (facet_index, facet) in graph.facets().iter().enumerate() {
        if owner.facet_id() != Some(facet.id()) {
            continue;
        }
        for (site_index, site) in facet.sites().iter().enumerate() {
            if owner.site_id() != Some(site.id()) {
                continue;
            }
            for (page_index, page) in site.pages().iter().enumerate() {
                if owner.page_id() != Some(page.id()) {
                    continue;
                }
                for (representation_index, locale) in page.representations().iter().enumerate() {
                    if owner.locale() == Some(locale) {
                        return format!(
                            "entity.facets[{facet_index}].sites[{site_index}].pages[{page_index}].representations[{representation_index}]"
                        );
                    }
                }
                return format!(
                    "entity.facets[{facet_index}].sites[{site_index}].pages[{page_index}]"
                );
            }
            return format!("entity.facets[{facet_index}].sites[{site_index}]");
        }
        return format!("entity.facets[{facet_index}]");
    }
    "entity".to_owned()
}
