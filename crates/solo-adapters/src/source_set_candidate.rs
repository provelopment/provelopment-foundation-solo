//! One all-or-refused **source-stage** candidate over every declared representation.
//!
//! This module composes four things that already exist and are already accepted: the strict
//! manifest parser ([`crate::publication_manifest`]), the explicitly adopted content-root
//! capability ([`crate::source_read::ContentRootCapability`]), the application's whole-set read
//! orchestration ([`solo_application::read_declared_source_set_v1`]) and the selected-body
//! preflight ([`crate::source_body_preflight`]). It re-implements none of their policy.
//!
//! **What a completed candidate is.** For every representation the sole parsed manifest
//! declares, an owned body with its qualified EFS lineage, its selected member kind, its logical
//! locator, the physical identity the handle reported and a linkage fingerprint. Either every
//! declaration is present exactly once or one typed refusal is returned; there is no partial
//! result, no synthesized entry, no fallback body and no directory iteration deciding order.
//!
//! **What it is not.** It is a *source-stage* candidate: the accepted selected-body checks are
//! the validation this stage performs, and they are not the deferred full Markdown/GFM,
//! twenty-two-module, link/asset, route, locale, property, visibility or render validation of
//! Stages 03–05. Nor is it a §6.2K *publishable* candidate: it is immutable and owned in memory,
//! which is not a simultaneous filesystem-wide, manifest-plus-files snapshot. Nothing here
//! publishes, activates, seals, resolves a route, decides visibility or writes an output.
//!
//! **JSON-first is the accepted rule, not a preference.** For each representation both declared
//! candidate paths are probed and reserved through the port, but exactly one body is read: a
//! present JSON source is chosen and the Markdown body is never read or parsed, even when the
//! Markdown file exists and even when its bytes would fail every check.

use core::fmt;

use solo_application::{
    PhysicalIdentityV1, ReadSourceCandidateV1, SourceReadRefusalKind, SourceReadRefusalV1,
    SourceReadSetOutcomeV1, fingerprint_of, read_declared_source_set_v1,
};
use solo_domain::{
    SourceCandidateKind, SourceCandidateSetV1, SourceObservationV1, SourceSelectionRefusal,
    SuppliedObservationsV1, select_provisional_source,
};

use crate::publication_manifest::{ManifestError, ParsedManifestCandidateV1, parse_manifest_bytes};
use crate::source_body_preflight::{
    BodyPreflightRefusal, InertBodyCandidateV1, preflight_selected_body,
};
use crate::source_read::ContentRootCapability;

/// The qualified EFS lineage of one declared representation.
///
/// These are authored identifiers and a declared full locale — never a public route, a canonical
/// spelling, a route segment or a physical path. Identity here comes from the declaration, so a
/// label, route, base-URL or locale change cannot re-identify it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualifiedSourceLineageV1 {
    entity: String,
    facet: String,
    site: String,
    page: String,
    locale: String,
}

impl QualifiedSourceLineageV1 {
    /// The declared Entity identifier.
    #[must_use]
    pub fn entity(&self) -> &str {
        &self.entity
    }

    /// The declared Facet identifier.
    #[must_use]
    pub fn facet(&self) -> &str {
        &self.facet
    }

    /// The declared Site identifier.
    #[must_use]
    pub fn site(&self) -> &str {
        &self.site
    }

    /// The declared Page identifier.
    #[must_use]
    pub fn page(&self) -> &str {
        &self.page
    }

    /// The declared full locale of this representation.
    #[must_use]
    pub fn locale(&self) -> &str {
        &self.locale
    }
}

/// One declared representation's validated source inside an assembled set.
///
/// It carries the accepted read evidence — member kind, logical locator, physical identity,
/// linkage fingerprint — and the preflight's owned, immutable copy of exactly those bytes. The
/// public surface hands out no mutable access, so a caller cannot alter a member after assembly.
#[derive(Debug, PartialEq, Eq)]
pub struct SourceSetMemberV1 {
    lineage: QualifiedSourceLineageV1,
    source: ReadSourceCandidateV1,
    body: InertBodyCandidateV1,
}

impl SourceSetMemberV1 {
    /// The qualified EFS lineage this member belongs to.
    #[must_use]
    pub fn lineage(&self) -> &QualifiedSourceLineageV1 {
        &self.lineage
    }

    /// The accepted read evidence for this member.
    #[must_use]
    pub fn source(&self) -> &ReadSourceCandidateV1 {
        &self.source
    }

    /// The preflighted, owned body.
    #[must_use]
    pub fn body(&self) -> &InertBodyCandidateV1 {
        &self.body
    }

    /// Which declared member kind was selected and read.
    #[must_use]
    pub fn member(&self) -> SourceCandidateKind {
        self.source.member()
    }

    /// The validated logical locator of the selected body, never a physical path.
    #[must_use]
    pub fn logical_path(&self) -> &str {
        self.source.logical_path()
    }

    /// The physical identity the reading handle reported.
    #[must_use]
    pub fn identity(&self) -> PhysicalIdentityV1 {
        self.source.identity()
    }

    /// The non-cryptographic linkage fingerprint of exactly these bytes.
    ///
    /// It is a linkage aid, never a cryptographic digest and never proof of whole-set safety.
    #[must_use]
    pub fn fingerprint(&self) -> u64 {
        self.source.fingerprint()
    }

    /// The preflighted body bytes.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        self.body.bytes()
    }

    /// Whether this member may be published. It never may.
    #[must_use]
    pub const fn is_publication_authorized(&self) -> bool {
        false
    }

    /// Whether this member is a verified filesystem snapshot. It never is.
    #[must_use]
    pub const fn is_verified_filesystem_snapshot(&self) -> bool {
        false
    }
}

/// The complete, owned, inert source-stage candidate for every declared representation.
///
/// The value only exists when **every** declaration was probed, reserved, read and preflighted
/// successfully. Its members are in manifest declaration order, so the same authored manifest
/// always yields the same order, and the same qualified lineage appears exactly once.
#[derive(Debug, PartialEq, Eq)]
pub struct SourceSetCandidateV1 {
    entity: String,
    members: Vec<SourceSetMemberV1>,
}

impl SourceSetCandidateV1 {
    /// The declared Entity identifier the whole set belongs to.
    #[must_use]
    pub fn entity(&self) -> &str {
        &self.entity
    }

    /// The members, in declaration order.
    #[must_use]
    pub fn members(&self) -> &[SourceSetMemberV1] {
        &self.members
    }

    /// How many declared representations the candidate holds.
    #[must_use]
    pub fn len(&self) -> usize {
        self.members.len()
    }

    /// Whether the candidate holds no representation at all.
    ///
    /// A completed candidate never is empty: an empty declaration is refused rather than
    /// returned as a successful empty candidate.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }

    /// The member for one declaration index, when the candidate holds it.
    #[must_use]
    pub fn get(&self, declaration: usize) -> Option<&SourceSetMemberV1> {
        self.members.get(declaration)
    }

    /// The members, in declaration order.
    pub fn iter(&self) -> impl Iterator<Item = &SourceSetMemberV1> {
        self.members.iter()
    }

    /// Whether this candidate may be published. It never may.
    #[must_use]
    pub const fn is_publication_authorized(&self) -> bool {
        false
    }

    /// Whether this candidate is a verified filesystem-wide snapshot. It never is.
    #[must_use]
    pub const fn is_verified_filesystem_snapshot(&self) -> bool {
        false
    }
}

/// Why an assembled source-stage candidate was refused.
///
/// Every variant delegates its stable code to a token that already exists in the accepted
/// contract, so this order introduces no new diagnostic vocabulary. A declaration index is
/// carried wherever the failure belongs to one representation; a set-level failure carries none.
#[derive(Debug)]
pub enum SourceSetRefusalV1 {
    /// The supplied manifest bytes were refused by the accepted strict parser.
    Manifest(ManifestError),
    /// The parsed manifest declared no representation at all.
    NoDeclaredRepresentation,
    /// Two declarations produced the same qualified EFS lineage.
    DuplicateLineage {
        /// The declaration index whose lineage was already declared.
        declaration: usize,
    },
    /// A physical reservation or a whole-set read refused before or during a read.
    Read {
        /// The declaration the refusal concerns, where one did.
        declaration: Option<usize>,
        /// The accepted refusal, unchanged.
        refusal: SourceReadRefusalV1,
    },
    /// The accepted selection policy refused the observations rebuilt for one chosen body.
    Selection {
        /// The declaration being preflighted.
        declaration: usize,
        /// The accepted policy refusal, unchanged.
        refusal: SourceSelectionRefusal,
    },
    /// The selected body failed the accepted inert body preflight.
    Body {
        /// The declaration being preflighted.
        declaration: usize,
        /// The accepted preflight refusal, unchanged.
        refusal: BodyPreflightRefusal,
    },
    /// The rebuilt selection or the preflighted bytes disagreed with the accepted read.
    ///
    /// This cannot happen while the accepted policy, port and preflight behave as they are
    /// tested to behave. It exists so a disagreement is refused loudly instead of being handed
    /// on as a body that nothing validated.
    SelectionDisagreement {
        /// The declaration where the disagreement was observed.
        declaration: usize,
    },
}

impl SourceSetRefusalV1 {
    /// The declaration this refusal concerns, where one does.
    #[must_use]
    pub const fn declaration(&self) -> Option<usize> {
        match self {
            Self::Manifest(_) | Self::NoDeclaredRepresentation => None,
            Self::DuplicateLineage { declaration }
            | Self::Selection { declaration, .. }
            | Self::Body { declaration, .. }
            | Self::SelectionDisagreement { declaration } => Some(*declaration),
            Self::Read { declaration, .. } => *declaration,
        }
    }

    /// Stable engineering code for this refusal, taken from the accepted contract.
    ///
    /// The empty-declaration and duplicate-lineage cases reuse the already registered tokens
    /// rather than introducing new ones.
    #[must_use]
    pub fn code(&self) -> &'static str {
        match self {
            Self::Manifest(error) => error.code(),
            Self::NoDeclaredRepresentation => SourceReadRefusalKind::NoneEffective.code(),
            Self::DuplicateLineage { .. } => "PREFLIGHT_SOURCE_CLAIM_DUPLICATE",
            Self::Read { refusal, .. } => refusal.code(),
            Self::Selection { refusal, .. } => refusal.code(),
            Self::Body { refusal, .. } => refusal.code(),
            Self::SelectionDisagreement { .. } => SourceReadRefusalKind::Unsupported.code(),
        }
    }
}

impl fmt::Display for SourceSetRefusalV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())?;
        match self {
            Self::Manifest(error) => write!(formatter, ": {}", error.detail()),
            Self::NoDeclaredRepresentation => {
                formatter.write_str(": the manifest declares no representation")
            }
            Self::DuplicateLineage { declaration } => write!(
                formatter,
                ": duplicate qualified lineage (declaration {declaration})"
            ),
            Self::Read { refusal, .. } => write!(formatter, ": {}", refusal.detail()),
            Self::Selection { refusal, .. } => write!(formatter, ": {}", refusal.detail()),
            Self::Body { refusal, .. } => write!(formatter, ": {}", refusal.detail()),
            Self::SelectionDisagreement { declaration } => write!(
                formatter,
                ": the rebuilt selection or preflight disagreed with the accepted read \
                 (declaration {declaration})"
            ),
        }
    }
}

impl std::error::Error for SourceSetRefusalV1 {}

/// One parsed declaration: its qualified lineage and its own validated candidate set.
struct Declaration<'m> {
    lineage: QualifiedSourceLineageV1,
    candidates: &'m SourceCandidateSetV1,
}

/// Walks **every** declared representation, in manifest order, and no other.
///
/// The walk is the declaration graph only: no directory is listed, no filename is guessed and
/// no authored manifest is synthesized. Two declarations that produce the same qualified
/// lineage cannot both name one representation, so that is refused rather than deduplicated.
fn declare_every_representation(
    parsed: &ParsedManifestCandidateV1,
) -> Result<Vec<Declaration<'_>>, SourceSetRefusalV1> {
    let entity = parsed.entity().id().as_str();
    let mut declarations: Vec<Declaration<'_>> = Vec::new();

    for facet in parsed.entity().facets() {
        for site in facet.sites() {
            for page in site.pages() {
                for representation in page.representations() {
                    let lineage = QualifiedSourceLineageV1 {
                        entity: entity.to_owned(),
                        facet: facet.id().as_str().to_owned(),
                        site: site.id().as_str().to_owned(),
                        page: page.id().as_str().to_owned(),
                        locale: representation.locale().to_owned(),
                    };
                    if declarations
                        .iter()
                        .any(|declared| declared.lineage == lineage)
                    {
                        return Err(SourceSetRefusalV1::DuplicateLineage {
                            declaration: declarations.len(),
                        });
                    }
                    declarations.push(Declaration {
                        lineage,
                        candidates: representation.source_candidates(),
                    });
                }
            }
        }
    }

    if declarations.is_empty() {
        return Err(SourceSetRefusalV1::NoDeclaredRepresentation);
    }
    Ok(declarations)
}

/// Preflights the one body that was actually chosen for one declaration.
///
/// The observations are rebuilt for the body in hand — present bytes for its own member, plus a
/// reported JSON absence only when the declaration does declare JSON and Markdown was chosen —
/// and handed to the accepted policy again. Reusing the policy is deliberate: this module must
/// not grow a second opinion about which member wins. The chosen member, its logical locator and
/// the resulting bytes must all agree with the accepted read, or the whole set is refused.
fn preflight_chosen_body(
    candidates: &SourceCandidateSetV1,
    source: &ReadSourceCandidateV1,
    declaration: usize,
) -> Result<InertBodyCandidateV1, SourceSetRefusalV1> {
    let mut observations = SuppliedObservationsV1::new();
    match source.member() {
        SourceCandidateKind::Json => {
            observations = observations.with_json(SourceObservationV1::Present(source.bytes()));
        }
        SourceCandidateKind::Markdown => {
            if candidates.json().is_some() {
                observations = observations.with_json(SourceObservationV1::Absent);
            }
            observations = observations.with_markdown(SourceObservationV1::Present(source.bytes()));
        }
    }

    let choice = select_provisional_source(candidates, observations).map_err(|refusal| {
        SourceSetRefusalV1::Selection {
            declaration,
            refusal,
        }
    })?;
    if choice.kind() != source.member() || choice.logical_path() != source.logical_path() {
        return Err(SourceSetRefusalV1::SelectionDisagreement { declaration });
    }

    let body = preflight_selected_body(&choice).map_err(|refusal| SourceSetRefusalV1::Body {
        declaration,
        refusal,
    })?;

    // The preflight must have validated exactly the bytes the read accepted. Anything else would
    // be handing on a body that nothing checked.
    if body.bytes() != source.bytes() || fingerprint_of(body.bytes()) != source.fingerprint() {
        return Err(SourceSetRefusalV1::SelectionDisagreement { declaration });
    }
    Ok(body)
}

/// Assembles one complete, owned, inert **source-stage** candidate.
///
/// The steps run in the only order this stage permits: strict parse of the already-supplied
/// manifest bytes; enumeration of every declared representation; reservation of every declared
/// candidate's physical identity **before** any body read; exactly one body per declaration under
/// the accepted JSON-first rule; inert preflight of exactly that body; assembly in declaration
/// order. The first failure anywhere returns one typed refusal and no candidate.
///
/// # Errors
///
/// Returns [`SourceSetRefusalV1`] for a refused manifest, a manifest declaring no representation,
/// a duplicate qualified lineage, a reservation or read refusal, a selection refusal, or a
/// selected-body preflight refusal. A present-but-invalid selected body is that body's refusal,
/// never a reason to try another declared member.
pub fn assemble_declared_source_set_v1(
    manifest_bytes: &[u8],
    root: &ContentRootCapability,
) -> Result<SourceSetCandidateV1, SourceSetRefusalV1> {
    let parsed = parse_manifest_bytes(manifest_bytes).map_err(SourceSetRefusalV1::Manifest)?;
    let declarations = declare_every_representation(&parsed)?;

    // The accepted whole-set read takes a slice of declarations, so the validated sets are cloned
    // once here. Cloning two already-validated locators adds no policy and no body.
    let sets: Vec<SourceCandidateSetV1> = declarations
        .iter()
        .map(|declaration| declaration.candidates.clone())
        .collect();

    let complete = match read_declared_source_set_v1(root, &sets) {
        SourceReadSetOutcomeV1::Complete(complete) => complete,
        SourceReadSetOutcomeV1::Refused(refusal) => {
            return Err(SourceSetRefusalV1::Read {
                declaration: refusal.declaration(),
                refusal: refusal.refusal(),
            });
        }
    };

    let entries = complete.into_entries();
    // A completed set must cover every declaration, exactly once, in declaration order.
    if entries.len() != declarations.len() {
        return Err(SourceSetRefusalV1::SelectionDisagreement {
            declaration: entries.len(),
        });
    }

    let mut members = Vec::with_capacity(declarations.len());
    for (index, entry) in entries.into_iter().enumerate() {
        let declaration = entry.declaration();
        if declaration != index {
            return Err(SourceSetRefusalV1::SelectionDisagreement { declaration });
        }
        let (Some(candidates), Some(declared)) =
            (sets.get(declaration), declarations.get(declaration))
        else {
            return Err(SourceSetRefusalV1::SelectionDisagreement { declaration });
        };
        let source = entry.into_source();
        let body = preflight_chosen_body(candidates, &source, declaration)?;
        members.push(SourceSetMemberV1 {
            lineage: declared.lineage.clone(),
            source,
            body,
        });
    }

    Ok(SourceSetCandidateV1 {
        entity: parsed.entity().id().as_str().to_owned(),
        members,
    })
}
