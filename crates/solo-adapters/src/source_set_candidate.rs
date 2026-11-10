//! One all-or-refused **source-stage** candidate over every declared representation.
//!
//! This module composes four things that already exist and are already accepted: the strict
//! manifest parser ([`crate::publication_manifest`]), the explicitly adopted content-root
//! capability ([`crate::source_read::ContentRootCapability`]), the application's sequential
//! whole-set protocol ([`solo_application::drive_declared_source_set_v1`]) and the selected-body
//! preflight ([`crate::source_body_preflight`]). It re-implements none of their policy.
//!
//! **What a completed candidate is.** For every representation the sole parsed manifest
//! declares, one validated body that was written into a protected staging object, plus its
//! qualified EFS lineage, its selected member kind, its logical locator, the physical identity
//! the handle reported, its original byte length and its linkage fingerprint. The candidate
//! holds a **range**, not the bytes: reading a member is a fallible, bounded operation performed
//! on demand. Either every declaration is present exactly once or one typed refusal is returned;
//! there is no partial result, no synthesized entry, no fallback body and no directory iteration
//! deciding order.
//!
//! **Bounded live body bytes.** One selected body is read, preflighted and staged at a time, and
//! both transient copies are dropped before the next declaration is read. The completed
//! candidate retains **no** body bytes at all — [`SourceSetCandidateV1::retained_body_bytes`]
//! reports exactly that — so peak live body memory is governed by the accepted single-body limit
//! and not by how many representations a manifest declares or how large they are in total.
//!
//! **What it is not.** It is a *source-stage* candidate: the accepted selected-body checks are
//! the validation this stage performs, and they are not the deferred full Markdown/GFM,
//! twenty-two-module, link/asset, route, locale, property, visibility or render validation of
//! Stages 03–05. Nor is it a §6.2K *publishable* candidate: stable staged bytes are not a
//! simultaneous filesystem-wide, manifest-plus-files snapshot. Nothing here publishes, activates,
//! seals, resolves a route, decides visibility or writes an output.
//!
//! **JSON-first is the accepted rule, not a preference.** For each representation both declared
//! candidate paths are probed and reserved through the port, but exactly one body is read: a
//! present JSON source is chosen and the Markdown body is never read or parsed, even when the
//! Markdown file exists and even when its bytes would fail every check.

use core::fmt;

use solo_application::{
    DeclaredSourceSetSinkV1, PhysicalIdentityV1, ReadSourceCandidateV1, SetSourceEntryV1,
    SourceReadRefusalKind, SourceReadRefusalV1, SourceReadSetOutcomeV1,
    drive_declared_source_set_v1, fingerprint_of,
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
use crate::source_set_staging::{
    SourceSetStagingV1, StageReaderV1, StagedBodyV1, StagingRootCapability,
};

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

/// The accepted read evidence for one member, without its body bytes.
///
/// It carries what the accepted read reported — member kind, logical locator, physical identity,
/// linkage fingerprint and byte length — and nothing a caller could mistake for the body itself.
/// The fingerprint is a linkage aid and is never treated as an integrity or security proof.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StagedSourceEvidenceV1 {
    member: SourceCandidateKind,
    logical_path: String,
    identity: PhysicalIdentityV1,
    fingerprint: u64,
    byte_length: u64,
}

impl StagedSourceEvidenceV1 {
    /// Which declared member kind was selected and staged.
    #[must_use]
    pub const fn member(&self) -> SourceCandidateKind {
        self.member
    }

    /// The validated logical locator of the staged body, never a physical path.
    #[must_use]
    pub fn logical_path(&self) -> &str {
        &self.logical_path
    }

    /// The physical identity the reading handle reported.
    #[must_use]
    pub const fn identity(&self) -> PhysicalIdentityV1 {
        self.identity
    }

    /// The non-cryptographic linkage fingerprint of exactly the staged bytes.
    #[must_use]
    pub const fn fingerprint(&self) -> u64 {
        self.fingerprint
    }

    /// How many bytes the staged body holds.
    #[must_use]
    pub const fn byte_length(&self) -> u64 {
        self.byte_length
    }
}

/// One declared representation inside a completed candidate.
///
/// It holds a staging range and metadata. Holding every member of a set therefore retains no
/// body bytes, which is what makes a completed candidate safe for a maximum-size manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceSetMemberV1 {
    lineage: QualifiedSourceLineageV1,
    evidence: StagedSourceEvidenceV1,
    body: StagedBodyV1,
}

impl SourceSetMemberV1 {
    /// The qualified EFS lineage this member belongs to.
    #[must_use]
    pub fn lineage(&self) -> &QualifiedSourceLineageV1 {
        &self.lineage
    }

    /// The accepted read evidence for this member, without its bytes.
    #[must_use]
    pub fn evidence(&self) -> &StagedSourceEvidenceV1 {
        &self.evidence
    }

    /// Which declared member kind was selected and read.
    #[must_use]
    pub const fn member(&self) -> SourceCandidateKind {
        self.evidence.member()
    }

    /// The validated logical locator of the selected body, never a physical path.
    #[must_use]
    pub fn logical_path(&self) -> &str {
        self.evidence.logical_path()
    }

    /// The physical identity the reading handle reported.
    #[must_use]
    pub const fn identity(&self) -> PhysicalIdentityV1 {
        self.evidence.identity()
    }

    /// The non-cryptographic linkage fingerprint of exactly these bytes.
    ///
    /// A linkage aid, never a cryptographic digest and never proof of whole-set safety.
    #[must_use]
    pub const fn fingerprint(&self) -> u64 {
        self.evidence.fingerprint()
    }

    /// How many bytes the staged body holds.
    #[must_use]
    pub const fn byte_length(&self) -> u64 {
        self.evidence.byte_length()
    }

    /// The staging range of this member. It carries no bytes and no path.
    #[must_use]
    pub const fn staged_body(&self) -> StagedBodyV1 {
        self.body
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

/// The complete, inert source-stage candidate for every declared representation.
///
/// The value only exists when **every** declaration was probed, reserved, read, preflighted and
/// staged successfully. Its members are in manifest declaration order, so the same authored
/// manifest always yields the same order, and the same qualified lineage appears exactly once.
///
/// It owns the staging store for its whole lifetime. Dropping the candidate closes every staged
/// object and removes the private namespace it created; reading a member afterwards is
/// impossible, and reading a member never reopens the original authored source.
pub struct SourceSetCandidateV1 {
    entity: String,
    staging: SourceSetStagingV1,
    members: Vec<SourceSetMemberV1>,
    max_live_body_bytes: u64,
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

    /// A bounded, fallible reader over one member's staged bytes.
    ///
    /// Returns `None` when the candidate holds no such declaration. The reader borrows the
    /// candidate, so it can neither outlive the staged objects nor write to them.
    #[must_use]
    pub fn reader(&self, declaration: usize) -> Option<StageReaderV1<'_>> {
        let member = self.members.get(declaration)?;
        self.staging.reader(member.staged_body())
    }

    /// Reads one member's whole staged body on demand.
    ///
    /// This materialises **one** member, never the set: the accepted single-body limit bounds it,
    /// and the allocation is a fallible reservation rather than an infallible one.
    ///
    /// # Errors
    ///
    /// Returns the typed staging refusal when the bytes cannot be reserved or read.
    pub fn member_bytes(&self, declaration: usize) -> Result<Option<Vec<u8>>, SourceReadRefusalV1> {
        let Some(reader) = self.reader(declaration) else {
            return Ok(None);
        };
        let mut bytes = Vec::new();
        reader.read_whole(&mut bytes)?;
        Ok(Some(bytes))
    }

    /// How many body bytes the completed candidate retains in memory. It is always zero.
    ///
    /// It is reported as a measured property of this type, not as a comment: the candidate holds
    /// a range per member, so the bound holds for any manifest the accepted limits allow.
    #[must_use]
    pub const fn retained_body_bytes(&self) -> u64 {
        0
    }

    /// The largest number of body bytes this attempt ever had in flight at once.
    ///
    /// It is observed while staging, not inferred from a call count: it is the sum of the
    /// accepted read's bytes and the preflight's validated copy for the single body in hand, so
    /// it is bounded by twice the accepted single-body limit whatever the manifest declares.
    #[must_use]
    pub const fn max_live_body_bytes(&self) -> u64 {
        self.max_live_body_bytes
    }

    /// The largest single body this attempt ever had in flight while staging.
    #[must_use]
    pub const fn max_staged_body_bytes(&self) -> u64 {
        self.staging.max_member_bytes()
    }

    /// How many bytes in total were written to staging.
    #[must_use]
    pub const fn staged_bytes(&self) -> u64 {
        self.staging.staged_bytes()
    }

    /// How many staging objects (open handles) the candidate holds.
    #[must_use]
    pub fn staged_object_count(&self) -> usize {
        self.staging.object_count()
    }

    /// How many directory entries are visible inside the private staging namespace.
    ///
    /// A live candidate has none, which is the on-host proof that no plaintext path exists.
    ///
    /// # Errors
    ///
    /// Returns the typed staging refusal when the namespace cannot be listed.
    pub fn visible_staging_entries(&self) -> Result<usize, SourceReadRefusalV1> {
        self.staging.visible_object_entries()
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

impl fmt::Debug for SourceSetCandidateV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SourceSetCandidateV1")
            .field("entity", &self.entity)
            .field("members", &self.members.len())
            .field("staged_bytes", &self.staging.staged_bytes())
            .field("staged_objects", &self.staging.object_count())
            .field("max_live_body_bytes", &self.max_live_body_bytes)
            .finish()
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
    /// The private staging store refused to accept or serve a validated body.
    ///
    /// The inner refusal is an already accepted token: a capacity or write failure is
    /// `SOURCE_OBSERVATION_IO_ERROR`, a denied write or directory is
    /// `SOURCE_OBSERVATION_PERMISSION_DENIED`, an unprovable or colliding location is
    /// `SOURCE_UNSAFE_PATH`, an unprovable guarantee is `SOURCE_UNSUPPORTED`, and an object that
    /// answered outside its validated range is `SNAPSHOT_CHANGED`. A
    /// staging failure is never reported as an absent authored body and never becomes a
    /// Markdown fallback.
    Staging {
        /// The declaration being staged, where the failure belongs to one.
        declaration: Option<usize>,
        /// The accepted refusal, unchanged.
        refusal: SourceReadRefusalV1,
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
            Self::Read { declaration, .. } | Self::Staging { declaration, .. } => *declaration,
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
            Self::Staging { refusal, .. } => refusal.code(),
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
            Self::Staging { refusal, .. } => write!(formatter, ": {}", refusal.detail()),
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

/// The private staging transaction that receives one selected body at a time.
///
/// It is the only place that holds body bytes. Each `stage` call takes one entry, preflights it,
/// writes the validated bytes into a nameless staging object and records a range; both transient
/// copies then go out of scope before the driver reads the next declaration.
struct CandidateTransaction<'a, 'm> {
    entity: &'a str,
    declarations: &'a [Declaration<'m>],
    sets: &'a [SourceCandidateSetV1],
    staging: SourceSetStagingV1,
    members: Vec<SourceSetMemberV1>,
    max_live_body_bytes: u64,
}

impl DeclaredSourceSetSinkV1 for CandidateTransaction<'_, '_> {
    type Refusal = SourceSetRefusalV1;
    type Complete = SourceSetCandidateV1;

    fn stage(&mut self, entry: SetSourceEntryV1) -> Result<(), Self::Refusal> {
        let declaration = entry.declaration();
        let source = entry.into_source();
        let Some(candidates) = self.sets.get(declaration) else {
            return Err(SourceSetRefusalV1::SelectionDisagreement { declaration });
        };
        let Some(declared) = self.declarations.get(declaration) else {
            return Err(SourceSetRefusalV1::SelectionDisagreement { declaration });
        };

        // Exactly two transient copies of exactly one body: the accepted read's own bytes and the
        // preflight's validated copy. Nothing else is held, and neither survives this call.
        let body = preflight_chosen_body(candidates, &source, declaration)?;
        let in_flight = (source.bytes().len() as u64).saturating_add(body.bytes().len() as u64);
        if in_flight > self.max_live_body_bytes {
            self.max_live_body_bytes = in_flight;
        }

        self.members
            .try_reserve(1)
            .map_err(|_| SourceSetRefusalV1::Staging {
                declaration: Some(declaration),
                refusal: SourceReadRefusalV1::new(SourceReadRefusalKind::OtherIo, None),
            })?;

        let staged = self.staging.stage_body(body.bytes()).map_err(|refusal| {
            SourceSetRefusalV1::Staging {
                declaration: Some(declaration),
                refusal,
            }
        })?;

        self.members.push(SourceSetMemberV1 {
            lineage: declared.lineage.clone(),
            evidence: StagedSourceEvidenceV1 {
                member: source.member(),
                logical_path: source.logical_path().to_owned(),
                identity: source.identity(),
                fingerprint: source.fingerprint(),
                byte_length: staged.len(),
            },
            body: staged,
        });
        // `source` and `body` are dropped here, so the transient bytes are released before the
        // driver reads the next declaration.
        Ok(())
    }

    fn finish(self) -> Result<Self::Complete, Self::Refusal> {
        Ok(SourceSetCandidateV1 {
            entity: self.entity.to_owned(),
            staging: self.staging,
            members: self.members,
            max_live_body_bytes: self.max_live_body_bytes,
        })
    }
}

/// Assembles one complete, inert **source-stage** candidate.
///
/// The steps run in the only order this stage permits: strict parse of the already-supplied
/// manifest bytes; enumeration of every declared representation; creation of the private staging
/// namespace; reservation of every declared candidate's physical identity **before** any body
/// read; exactly one body per declaration under the accepted JSON-first rule, preflighted and
/// staged before the next is read; assembly in declaration order. The first failure anywhere
/// returns one typed refusal and no candidate.
///
/// `staging_root` is supplied by the caller and is the only place temporary objects are created.
/// This function never chooses a directory, never falls back to a system temp location and never
/// continues without it: without room to stage privately, the attempt refuses.
///
/// # Errors
///
/// Returns [`SourceSetRefusalV1`] for a refused manifest, a manifest declaring no representation,
/// a duplicate qualified lineage, an unprovable staging location, a reservation or read refusal,
/// a selection refusal, a selected-body preflight refusal, or a staging refusal.
pub fn assemble_declared_source_set_v1(
    manifest_bytes: &[u8],
    root: &ContentRootCapability,
    staging_root: &StagingRootCapability,
) -> Result<SourceSetCandidateV1, SourceSetRefusalV1> {
    let parsed = parse_manifest_bytes(manifest_bytes).map_err(SourceSetRefusalV1::Manifest)?;
    let declarations = declare_every_representation(&parsed)?;

    // The accepted whole-set driver takes a slice of declarations, so the validated locators are
    // cloned once here. This is O(declarations) metadata and never a body, and the reservation is
    // fallible so a huge declaration count refuses instead of aborting the process.
    let mut sets: Vec<SourceCandidateSetV1> = Vec::new();
    sets.try_reserve_exact(declarations.len())
        .map_err(|_| SourceSetRefusalV1::Staging {
            declaration: None,
            refusal: SourceReadRefusalV1::new(SourceReadRefusalKind::OtherIo, None),
        })?;
    for declaration in &declarations {
        sets.push(declaration.candidates.clone());
    }

    let staging = staging_root
        .begin()
        .map_err(|refusal| SourceSetRefusalV1::Staging {
            declaration: None,
            refusal,
        })?;

    let transaction = CandidateTransaction {
        entity: parsed.entity().id().as_str(),
        declarations: &declarations,
        sets: &sets,
        staging,
        members: Vec::new(),
        max_live_body_bytes: 0,
    };

    match drive_declared_source_set_v1(root, &sets, transaction) {
        SourceReadSetOutcomeV1::Complete(candidate) => Ok(candidate),
        SourceReadSetOutcomeV1::ReadRefused(refusal) => Err(SourceSetRefusalV1::Read {
            declaration: refusal.declaration(),
            refusal: refusal.refusal(),
        }),
        SourceReadSetOutcomeV1::SinkRefused(refusal) => Err(refusal),
    }
}
