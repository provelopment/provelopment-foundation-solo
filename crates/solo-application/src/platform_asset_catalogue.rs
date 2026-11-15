//! `SOLO-WO-118 R12` — the closed Platform asset catalogue contract and its inert V4 fact binding.
//!
//! This module is a **pure typed invariant gate**. It opens no file, starts no worker, reads no
//! Platform directory and authorizes nothing: it validates the shape of a proposed `1.0.0` catalogue
//! profile and can compare one typed entry with one **already checked** V4 inspection record.
//!
//! **A catalogue is an assertion made by its author.** A parsed, well-formed entry proves only that
//! the declaration is internally consistent and bounded. It is not legal clearance, not proof of
//! rights, not media correctness, not a publication decision and not a Platform source acquisition.
//! Every authority flag here is a constant `false`, and no API in this module can override them.
//!
//! R12 deliberately contains **no** Platform reader and **no** product catalogue: the accepted
//! Entity source reader must keep refusing `AssetNamespace::Platform` as `ASSET_PLATFORM_UNVERIFIED`
//! until a later Owner-issued rights and artwork decision.

use std::fmt;

use solo_domain::{AssetNamespace, AssetReferenceV1, MAX_LOCATOR_SEGMENTS, RESERVED_WINDOWS_STEMS};

use crate::graphic_image_inspection::{GraphicImageFormatV3, GraphicImageInspectionV4};
use crate::media_worker::MediaWorkerDigestV1;

/// The one accepted Platform asset library version of this profile.
///
/// The validator receives the selected version as an **independent trusted argument**; it is never
/// discovered, guessed, defaulted or upgraded from the document it is checking.
pub const PLATFORM_ASSET_LIBRARY_VERSION_V1: &str = "1.0.0";

/// Maximum number of catalogue entries one `1.0.0` profile may declare.
pub const MAX_PLATFORM_CATALOGUE_ENTRIES_V1: usize = 1024;

/// Maximum accepted byte length of one catalogue asset identifier.
pub const MAX_PLATFORM_ASSET_ID_BYTES_V1: usize = 240;

/// Maximum accepted number of slash-separated identifier segments.
pub const MAX_PLATFORM_ASSET_ID_SEGMENTS_V1: usize = MAX_LOCATOR_SEGMENTS;

/// Maximum accepted number of Unicode scalar values in one rights or provenance text field.
pub const MAX_PLATFORM_RIGHTS_SCALARS_V1: usize = 512;

/// The accepted maximum side of one catalogued image.
pub const MAX_PLATFORM_ASSET_SIDE_V1: u32 = 8192;

/// The accepted maximum `width × height` of one catalogued image.
pub const MAX_PLATFORM_ASSET_PIXELS_V1: u64 = 32_000_000;

/// Whether a validated catalogue authorizes publication. Always `false`.
pub const PLATFORM_ASSET_CATALOGUE_GRANTS_PUBLICATION_AUTHORITY_V1: bool = false;

/// Whether a validated catalogue proves the media itself. Always `false`.
pub const PLATFORM_ASSET_CATALOGUE_PROVES_MEDIA_CORRECTNESS_V1: bool = false;

/// Whether a validated catalogue proves rights clearance. Always `false`.
pub const PLATFORM_ASSET_CATALOGUE_RIGHTS_REVIEWED_V1: bool = false;

/// Whether a validated catalogue makes a Platform source readable. Always `false`.
pub const PLATFORM_ASSET_CATALOGUE_PLATFORM_SOURCE_READABLE_V1: bool = false;

/// The closed media classes this image-only profile accepts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformAssetMediaClassV1 {
    /// The one accepted class of this slice: a still or animated image inspected by V4.
    Image,
}

impl PlatformAssetMediaClassV1 {
    /// The exact authored spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Image => "image",
        }
    }

    /// Parses the exact authored spelling; case folding and aliases are refused.
    #[must_use]
    pub fn parse(text: &str) -> Option<Self> {
        if text == "image" {
            Some(Self::Image)
        } else {
            None
        }
    }
}

/// Every way one proposed catalogue, entry or binding can be refused.
///
/// Each is a closed, path-free category: the token is stable, carries no asset path, licence text,
/// holder, evidence reference or untrusted JSON fragment, and refuses the **whole** candidate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformCatalogueRefusalV1 {
    /// The document was not one bounded, well-formed JSON object of the expected shape.
    InvalidSyntax,
    /// A JSON object repeated a key at some depth.
    DuplicateField,
    /// A JSON object carried a key this closed profile does not define.
    UnknownField,
    /// The declared or selected library version was not exactly the accepted profile.
    VersionMismatch,
    /// The catalogue declared no entry at all.
    EmptyCatalogue,
    /// The catalogue declared more entries than this profile admits.
    TooManyEntries,
    /// An identifier was not one accepted closed portable relative image path.
    InvalidId,
    /// Two entries declared the same exact identifier.
    DuplicateId,
    /// Two identifiers collided case-insensitively.
    CaseCollidingIdentifier,
    /// A digest was not exactly 64 lowercase hexadecimal digits, or was all-zero.
    InvalidDigest,
    /// Two entries declared the same digest alias.
    DuplicateDigest,
    /// The media class was not exactly `image`.
    InvalidMediaClass,
    /// The declared MIME was not one of the six accepted V4 image types.
    UnsupportedMediaType,
    /// The declared MIME and the identifier extension disagreed.
    MimeExtensionMismatch,
    /// A dimension was zero, out of range, or the checked pixel count exceeded the ceiling.
    InvalidDimensions,
    /// A licence or provenance text field was empty or out of bounds.
    MissingRightsEvidence,
    /// A licence or provenance text field carried control or NUL characters.
    InvalidRightsText,
}

impl PlatformCatalogueRefusalV1 {
    /// The stable machine-readable token; the only public diagnostic text.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::InvalidSyntax => "PLATFORM_CATALOGUE_INVALID_SYNTAX",
            Self::DuplicateField => "PLATFORM_CATALOGUE_DUPLICATE_FIELD",
            Self::UnknownField => "PLATFORM_CATALOGUE_UNKNOWN_FIELD",
            Self::VersionMismatch => "PLATFORM_CATALOGUE_VERSION_MISMATCH",
            Self::EmptyCatalogue => "PLATFORM_CATALOGUE_EMPTY",
            Self::TooManyEntries => "PLATFORM_CATALOGUE_TOO_MANY_ENTRIES",
            Self::InvalidId => "PLATFORM_CATALOGUE_INVALID_ID",
            Self::DuplicateId => "PLATFORM_CATALOGUE_DUPLICATE_ID",
            Self::CaseCollidingIdentifier => "PLATFORM_CATALOGUE_CASE_COLLIDING_ID",
            Self::InvalidDigest => "PLATFORM_CATALOGUE_INVALID_DIGEST",
            Self::DuplicateDigest => "PLATFORM_CATALOGUE_DUPLICATE_DIGEST",
            Self::InvalidMediaClass => "PLATFORM_CATALOGUE_INVALID_MEDIA_CLASS",
            Self::UnsupportedMediaType => "PLATFORM_CATALOGUE_UNSUPPORTED_MEDIA_TYPE",
            Self::MimeExtensionMismatch => "PLATFORM_CATALOGUE_MIME_EXTENSION_MISMATCH",
            Self::InvalidDimensions => "PLATFORM_CATALOGUE_INVALID_DIMENSIONS",
            Self::MissingRightsEvidence => "PLATFORM_CATALOGUE_MISSING_RIGHTS_EVIDENCE",
            Self::InvalidRightsText => "PLATFORM_CATALOGUE_INVALID_RIGHTS_TEXT",
        }
    }
}

impl fmt::Display for PlatformCatalogueRefusalV1 {
    /// Renders only the closed token.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.token())
    }
}

/// The reviewable licence assertion one catalogue entry carries.
///
/// These strings are the author's assertion. They are **not** legal approval, and this type cannot
/// represent "approved": no constructor accepts an approval flag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformAssetLicenceV1 {
    expression: String,
    rights_holder: String,
    evidence: String,
}

impl PlatformAssetLicenceV1 {
    /// Validates one licence assertion.
    ///
    /// # Errors
    ///
    /// Returns [`PlatformCatalogueRefusalV1::MissingRightsEvidence`] for empty or over-long text and
    /// [`PlatformCatalogueRefusalV1::InvalidRightsText`] for control characters.
    pub fn new(
        expression: &str,
        rights_holder: &str,
        evidence: &str,
    ) -> Result<Self, PlatformCatalogueRefusalV1> {
        Ok(Self {
            expression: rights_text(expression)?,
            rights_holder: rights_text(rights_holder)?,
            evidence: rights_text(evidence)?,
        })
    }

    /// The declared licence identifier or expression.
    #[must_use]
    pub fn expression(&self) -> &str {
        &self.expression
    }

    /// The declared rights holder.
    #[must_use]
    pub fn rights_holder(&self) -> &str {
        &self.rights_holder
    }

    /// The declared reviewable rights evidence reference.
    #[must_use]
    pub fn evidence(&self) -> &str {
        &self.evidence
    }

    /// Whether this assertion proves rights clearance. Always `false`.
    #[must_use]
    pub const fn rights_reviewed(&self) -> bool {
        PLATFORM_ASSET_CATALOGUE_RIGHTS_REVIEWED_V1
    }
}

/// The reviewable provenance assertion one catalogue entry carries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformAssetProvenanceV1 {
    source: String,
    evidence: String,
}

impl PlatformAssetProvenanceV1 {
    /// Validates one provenance assertion.
    ///
    /// # Errors
    ///
    /// Returns [`PlatformCatalogueRefusalV1::MissingRightsEvidence`] for empty or over-long text and
    /// [`PlatformCatalogueRefusalV1::InvalidRightsText`] for control characters.
    pub fn new(source: &str, evidence: &str) -> Result<Self, PlatformCatalogueRefusalV1> {
        Ok(Self {
            source: rights_text(source)?,
            evidence: rights_text(evidence)?,
        })
    }

    /// The declared source or creation description.
    #[must_use]
    pub fn source(&self) -> &str {
        &self.source
    }

    /// The declared reviewable origin record reference.
    #[must_use]
    pub fn evidence(&self) -> &str {
        &self.evidence
    }
}

/// Bounded, control-free rights or provenance text.
fn rights_text(text: &str) -> Result<String, PlatformCatalogueRefusalV1> {
    let scalars = text.chars().count();
    if scalars == 0
        || scalars > MAX_PLATFORM_RIGHTS_SCALARS_V1
        || text.trim().is_empty()
        || text.trim() != text
    {
        return Err(PlatformCatalogueRefusalV1::MissingRightsEvidence);
    }
    if text.chars().any(|character| character.is_control()) {
        return Err(PlatformCatalogueRefusalV1::InvalidRightsText);
    }
    Ok(text.to_owned())
}

/// One declared asset identity, exactly as a catalogue document states it.
///
/// The fields are unvalidated here; [`PlatformAssetCatalogueEntryV1::new`] validates every one of
/// them and returns the closed refusal. Keeping the declaration separate keeps the validated entry
/// constructor small and keeps raw text out of the validated type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformAssetDeclarationV1 {
    id: String,
    sha256: String,
    media_class: String,
    mime: String,
    width: u32,
    height: u32,
}

impl PlatformAssetDeclarationV1 {
    /// Records one declared identity exactly as authored.
    #[must_use]
    pub fn of(
        id: &str,
        sha256: &str,
        media_class: &str,
        mime: &str,
        width: u32,
        height: u32,
    ) -> Self {
        Self {
            id: id.to_owned(),
            sha256: sha256.to_owned(),
            media_class: media_class.to_owned(),
            mime: mime.to_owned(),
            width,
            height,
        }
    }

    /// The declared identifier.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// The declared source digest text.
    #[must_use]
    pub fn sha256(&self) -> &str {
        &self.sha256
    }

    /// The declared media class text.
    #[must_use]
    pub fn media_class(&self) -> &str {
        &self.media_class
    }

    /// The declared MIME text.
    #[must_use]
    pub fn mime(&self) -> &str {
        &self.mime
    }

    /// The declared width.
    #[must_use]
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// The declared height.
    #[must_use]
    pub const fn height(&self) -> u32 {
        self.height
    }
}

/// One validated catalogue entry: a declared asset identity plus its rights assertions.
///
/// The identifier is validated with the shared closed portable-relative-path grammar, but with the
/// **Platform** namespace: the plain grammar is shared, the namespace authority is not. No Entity
/// namespace authorizes Platform I/O anywhere in this module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformAssetCatalogueEntryV1 {
    reference: AssetReferenceV1,
    sha256: MediaWorkerDigestV1,
    format: GraphicImageFormatV3,
    media_class: PlatformAssetMediaClassV1,
    width: u32,
    height: u32,
    licence: PlatformAssetLicenceV1,
    provenance: PlatformAssetProvenanceV1,
}

impl PlatformAssetCatalogueEntryV1 {
    /// Validates one declared entry.
    ///
    /// # Errors
    ///
    /// Returns a closed [`PlatformCatalogueRefusalV1`] for an unacceptable identifier, digest,
    /// media class, MIME/extension pair, geometry, licence or provenance assertion.
    pub fn new(
        declaration: &PlatformAssetDeclarationV1,
        licence: PlatformAssetLicenceV1,
        provenance: PlatformAssetProvenanceV1,
    ) -> Result<Self, PlatformCatalogueRefusalV1> {
        let reference = platform_reference(declaration.id())?;
        let sha256 = parse_digest(declaration.sha256())?;
        let class = PlatformAssetMediaClassV1::parse(declaration.media_class())
            .ok_or(PlatformCatalogueRefusalV1::InvalidMediaClass)?;
        let format = platform_format(&reference).ok_or(PlatformCatalogueRefusalV1::InvalidId)?;
        let expected_mime = match format {
            GraphicImageFormatV3::Png => "image/png",
            GraphicImageFormatV3::Jpeg => "image/jpeg",
            GraphicImageFormatV3::WebP => "image/webp",
            GraphicImageFormatV3::Gif => "image/gif",
            GraphicImageFormatV3::Svg => "image/svg+xml",
            GraphicImageFormatV3::Avif => "image/avif",
        };
        if !is_accepted_image_mime(declaration.mime()) {
            return Err(PlatformCatalogueRefusalV1::UnsupportedMediaType);
        }
        if declaration.mime() != expected_mime {
            return Err(PlatformCatalogueRefusalV1::MimeExtensionMismatch);
        }
        let (width, height) = (declaration.width(), declaration.height());
        if width == 0
            || height == 0
            || width > MAX_PLATFORM_ASSET_SIDE_V1
            || height > MAX_PLATFORM_ASSET_SIDE_V1
        {
            return Err(PlatformCatalogueRefusalV1::InvalidDimensions);
        }
        let pixels = u64::from(width) * u64::from(height);
        if pixels == 0 || pixels > MAX_PLATFORM_ASSET_PIXELS_V1 {
            return Err(PlatformCatalogueRefusalV1::InvalidDimensions);
        }
        Ok(Self {
            reference,
            sha256,
            format,
            media_class: class,
            width,
            height,
            licence,
            provenance,
        })
    }

    /// The exact declared identifier.
    #[must_use]
    pub fn id(&self) -> &str {
        self.reference.reference()
    }

    /// The Platform namespace this identifier was validated under.
    #[must_use]
    pub const fn namespace(&self) -> AssetNamespace {
        self.reference.namespace()
    }

    /// The declared source digest.
    #[must_use]
    pub const fn sha256(&self) -> MediaWorkerDigestV1 {
        self.sha256
    }

    /// The image format the identifier extension declares.
    #[must_use]
    pub const fn format(&self) -> GraphicImageFormatV3 {
        self.format
    }

    /// The canonical MIME of that format.
    #[must_use]
    pub const fn mime(&self) -> &'static str {
        self.format.canonical_mime()
    }

    /// The declared media class.
    #[must_use]
    pub const fn media_class(&self) -> PlatformAssetMediaClassV1 {
        self.media_class
    }

    /// The declared intrinsic or displayed width.
    #[must_use]
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// The declared intrinsic or displayed height.
    #[must_use]
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// The declared licence assertion.
    #[must_use]
    pub const fn licence(&self) -> &PlatformAssetLicenceV1 {
        &self.licence
    }

    /// The declared provenance assertion.
    #[must_use]
    pub const fn provenance(&self) -> &PlatformAssetProvenanceV1 {
        &self.provenance
    }
}

/// The six accepted image MIME values of the V4 slice.
const ACCEPTED_IMAGE_MIMES_V1: [&str; 6] = [
    "image/png",
    "image/jpeg",
    "image/webp",
    "image/gif",
    "image/svg+xml",
    "image/avif",
];

/// Segments that would make an identifier name a reserved container or namespace rather than an
/// asset: the Platform, Entity and route-owning prefixes plus the catalogue's own filename.
const RESERVED_CATALOGUE_SEGMENTS_V1: [&str; 5] = [
    "platform",
    "entity",
    "content",
    "pages",
    "platform-asset-catalogue.json",
];

/// Whether this MIME is one of the six accepted image types.
fn is_accepted_image_mime(mime: &str) -> bool {
    ACCEPTED_IMAGE_MIMES_V1.contains(&mime)
}

/// The closed extension grammar shared with the V4 inspector, applied to a **Platform** identifier.
///
/// The V4 `declared_by` gate is the Entity reader's own authority and deliberately refuses a
/// non-Entity namespace, so it is left untouched and is not repurposed here. Only the plain textual
/// extension mapping is reused; the namespace authority is not.
fn platform_format(reference: &AssetReferenceV1) -> Option<GraphicImageFormatV3> {
    let text = reference.reference();
    let segment = text.rsplit('/').next().unwrap_or(text);
    let (_, extension) = segment.rsplit_once('.')?;
    GraphicImageFormatV3::ALL
        .into_iter()
        .find(|format| format.accepted_extensions().contains(&extension))
}

/// Validates one identifier as a closed portable relative image path under **Platform**.
fn platform_reference(id: &str) -> Result<AssetReferenceV1, PlatformCatalogueRefusalV1> {
    if id.len() > MAX_PLATFORM_ASSET_ID_BYTES_V1 {
        return Err(PlatformCatalogueRefusalV1::InvalidId);
    }
    let reference = AssetReferenceV1::parse(AssetNamespace::Platform, id)
        .map_err(|_| PlatformCatalogueRefusalV1::InvalidId)?;
    let mut segments = id.split('/');
    let mut count = 0usize;
    for segment in &mut segments {
        count += 1;
        if count > MAX_PLATFORM_ASSET_ID_SEGMENTS_V1 {
            return Err(PlatformCatalogueRefusalV1::InvalidId);
        }
        let lowered = segment.to_ascii_lowercase();
        if RESERVED_CATALOGUE_SEGMENTS_V1.contains(&lowered.as_str()) {
            return Err(PlatformCatalogueRefusalV1::InvalidId);
        }
        let stem = lowered
            .split_once('.')
            .map_or(lowered.as_str(), |(stem, _)| stem);
        if RESERVED_WINDOWS_STEMS.contains(&stem) {
            return Err(PlatformCatalogueRefusalV1::InvalidId);
        }
        if segment.ends_with('.') || segment.ends_with(' ') || segment.starts_with(' ') {
            return Err(PlatformCatalogueRefusalV1::InvalidId);
        }
    }
    Ok(reference)
}

/// Parses one exact 64-lowercase-hexadecimal, non-zero source digest.
fn parse_digest(text: &str) -> Result<MediaWorkerDigestV1, PlatformCatalogueRefusalV1> {
    if text.len() != 64
        || !text
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(PlatformCatalogueRefusalV1::InvalidDigest);
    }
    let digest = MediaWorkerDigestV1::parse_lower_hex(text)
        .ok_or(PlatformCatalogueRefusalV1::InvalidDigest)?;
    if digest.as_bytes() == &[0u8; 32] {
        return Err(PlatformCatalogueRefusalV1::InvalidDigest);
    }
    Ok(digest)
}

/// One validated `1.0.0` catalogue: an ordered, duplicate-free set of declared entries.
///
/// The catalogue is an assertion, never an authority. It holds no reader, no directory handle and no
/// publication flag; every authority accessor is a constant `false`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformAssetCatalogueV1 {
    version: &'static str,
    entries: Vec<PlatformAssetCatalogueEntryV1>,
}

impl PlatformAssetCatalogueV1 {
    /// Validates one selected profile against an **independent** trusted version argument.
    ///
    /// # Errors
    ///
    /// Refuses a version other than [`PLATFORM_ASSET_LIBRARY_VERSION_V1`], an empty entry list, an
    /// over-long entry list, a duplicate or case-colliding identifier and a duplicate digest alias.
    pub fn new(
        selected_version: &str,
        entries: Vec<PlatformAssetCatalogueEntryV1>,
    ) -> Result<Self, PlatformCatalogueRefusalV1> {
        if selected_version != PLATFORM_ASSET_LIBRARY_VERSION_V1 {
            return Err(PlatformCatalogueRefusalV1::VersionMismatch);
        }
        if entries.is_empty() {
            return Err(PlatformCatalogueRefusalV1::EmptyCatalogue);
        }
        if entries.len() > MAX_PLATFORM_CATALOGUE_ENTRIES_V1 {
            return Err(PlatformCatalogueRefusalV1::TooManyEntries);
        }
        for (index, entry) in entries.iter().enumerate() {
            for earlier in &entries[..index] {
                if earlier.id() == entry.id() {
                    return Err(PlatformCatalogueRefusalV1::DuplicateId);
                }
                if earlier.id().eq_ignore_ascii_case(entry.id()) {
                    return Err(PlatformCatalogueRefusalV1::CaseCollidingIdentifier);
                }
                if earlier.sha256() == entry.sha256() {
                    return Err(PlatformCatalogueRefusalV1::DuplicateDigest);
                }
            }
        }
        Ok(Self {
            version: PLATFORM_ASSET_LIBRARY_VERSION_V1,
            entries,
        })
    }

    /// The accepted profile version.
    #[must_use]
    pub const fn version(&self) -> &'static str {
        self.version
    }

    /// Every entry, exactly once, in declared order.
    #[must_use]
    pub fn entries(&self) -> &[PlatformAssetCatalogueEntryV1] {
        &self.entries
    }

    /// How many entries were declared.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the catalogue declared no entry. Never true for a validated catalogue.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Looks up one exact identifier; never case-folded, aliased or approximated.
    #[must_use]
    pub fn exact_entry(&self, id: &str) -> Option<&PlatformAssetCatalogueEntryV1> {
        self.entries.iter().find(|entry| entry.id() == id)
    }

    /// Whether this catalogue authorizes publication. Always `false`.
    #[must_use]
    pub const fn grants_publication_authority(&self) -> bool {
        PLATFORM_ASSET_CATALOGUE_GRANTS_PUBLICATION_AUTHORITY_V1
    }

    /// Whether this catalogue proves the media itself. Always `false`.
    #[must_use]
    pub const fn proves_media_correctness(&self) -> bool {
        PLATFORM_ASSET_CATALOGUE_PROVES_MEDIA_CORRECTNESS_V1
    }

    /// Whether this catalogue proves rights clearance. Always `false`.
    #[must_use]
    pub const fn rights_reviewed(&self) -> bool {
        PLATFORM_ASSET_CATALOGUE_RIGHTS_REVIEWED_V1
    }

    /// Whether this catalogue makes a Platform source readable. Always `false`.
    #[must_use]
    pub const fn platform_source_readable(&self) -> bool {
        PLATFORM_ASSET_CATALOGUE_PLATFORM_SOURCE_READABLE_V1
    }

    /// Whether this catalogue proves a derived output. Always `false`.
    #[must_use]
    pub const fn derived_output_verified(&self) -> bool {
        false
    }

    /// Whether this catalogue is release-ready. Always `false`.
    #[must_use]
    pub const fn release_ready(&self) -> bool {
        false
    }
}

/// The scope one inert evidence binding is allowed to claim.
///
/// R12 has **no** Platform reader, so a binding can only ever compare content facts against a record
/// produced from a different namespace during review. The real later call site is represented
/// explicitly so it can be proved refused rather than silently assumed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformCatalogueBindingScopeV1 {
    /// Review-only content-fact comparison against an already checked record. Never a Platform read.
    ContentFactComparisonOnly,
    /// The intended future call site, which requires a real Platform-sourced record. Refused in R12.
    PlatformSource,
}

/// Every way one catalogue entry or binding can be refused against checked V4 evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformCatalogueMatchRefusalV1 {
    /// A Platform-sourced record is required and no Platform reader exists in this slice.
    PlatformSourceUnavailable,
    /// The catalogue entry and the requested position do not correspond one-to-one.
    CatalogueCountMismatch,
    /// The requested exact identifier is absent from the catalogue.
    AssetIdAbsent,
    /// The declared source digest is not the parent-computed V4 source digest.
    SourceDigestMismatch,
    /// The declared format is not the confirmed V4 format.
    FormatMismatch,
    /// The declared MIME is not the confirmed V4 canonical MIME.
    MimeMismatch,
    /// The declared media class does not describe the observed V4 payload.
    MediaClassMismatch,
    /// The declared geometry is not the confirmed V4 geometry.
    DimensionsMismatch,
    /// The record claims an authority the catalogue cannot confer.
    RecordAuthorityClaimed,
}

impl PlatformCatalogueMatchRefusalV1 {
    /// The stable machine-readable token; the only public diagnostic text.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::PlatformSourceUnavailable => "PLATFORM_MATCH_PLATFORM_SOURCE_UNAVAILABLE",
            Self::CatalogueCountMismatch => "PLATFORM_MATCH_COUNT_MISMATCH",
            Self::AssetIdAbsent => "PLATFORM_MATCH_ASSET_ID_ABSENT",
            Self::SourceDigestMismatch => "PLATFORM_MATCH_SOURCE_DIGEST_MISMATCH",
            Self::FormatMismatch => "PLATFORM_MATCH_FORMAT_MISMATCH",
            Self::MimeMismatch => "PLATFORM_MATCH_MIME_MISMATCH",
            Self::MediaClassMismatch => "PLATFORM_MATCH_MEDIA_CLASS_MISMATCH",
            Self::DimensionsMismatch => "PLATFORM_MATCH_DIMENSIONS_MISMATCH",
            Self::RecordAuthorityClaimed => "PLATFORM_MATCH_RECORD_AUTHORITY_CLAIMED",
        }
    }
}

impl fmt::Display for PlatformCatalogueMatchRefusalV1 {
    /// Renders only the closed token.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.token())
    }
}

/// The closed result of one inert catalogue-to-evidence comparison.
///
/// `MatchedForReview` is deliberately not named `Approved`, `Licensed`, `Publishable` or
/// `PlatformRead`: the catalogue cannot confer any of those.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformCatalogueMatchV1 {
    /// The declared facts agree with the already checked record. Review input only.
    MatchedForReview,
    /// One closed refusal; nothing was retained.
    Refused(PlatformCatalogueMatchRefusalV1),
}

/// The closed result of one whole-catalogue binding over an ordered record set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformCatalogueBindingV1 {
    /// Every declared entry agreed with its corresponding record. Review input only.
    CompleteForReview {
        /// How many entries were compared, one to one.
        matched: usize,
    },
    /// One refused position; **no** partial match survives.
    Refused {
        /// The zero-based position that refused.
        position: usize,
        /// The closed refusal category.
        kind: PlatformCatalogueMatchRefusalV1,
    },
}

impl PlatformCatalogueBindingV1 {
    /// Whether this binding authorizes publication. Always `false`.
    #[must_use]
    pub const fn grants_publication_authority(&self) -> bool {
        false
    }

    /// Whether this binding proves rights clearance. Always `false`.
    #[must_use]
    pub const fn rights_reviewed(&self) -> bool {
        false
    }

    /// Whether this binding read a Platform source. Always `false`.
    #[must_use]
    pub const fn platform_source_readable(&self) -> bool {
        false
    }
}

/// Compares one typed catalogue entry with one **already checked** V4 record.
///
/// The comparison is inert: it opens no byte, starts no worker and takes nothing from the catalogue
/// that could override an observation. The catalogue must agree with the parent-computed **source**
/// digest — never an AVIF plane/output digest, GIF timeline digest, canonical SVG output digest or a
/// decoded plane digest — and with the confirmed format, canonical MIME, media class and geometry.
///
/// # Errors
///
/// Returns [`PlatformCatalogueMatchRefusalV1::PlatformSourceUnavailable`] for the real Platform call
/// site, which this slice cannot serve, and one closed fact-mismatch category otherwise. It never
/// returns a partial or approved state.
pub fn match_catalogue_entry_to_v4_evidence_v1(
    entry: &PlatformAssetCatalogueEntryV1,
    record: &GraphicImageInspectionV4,
    scope: PlatformCatalogueBindingScopeV1,
) -> PlatformCatalogueMatchV1 {
    use PlatformCatalogueMatchRefusalV1 as Refusal;
    use PlatformCatalogueMatchV1 as Match;

    if scope == PlatformCatalogueBindingScopeV1::PlatformSource {
        // No Platform reader exists in R12: a real Platform acquisition can never be claimed here.
        return Match::Refused(Refusal::PlatformSourceUnavailable);
    }
    if entry.sha256() != *record.source_digest() {
        return Match::Refused(Refusal::SourceDigestMismatch);
    }
    if entry.format() != record.format() {
        return Match::Refused(Refusal::FormatMismatch);
    }
    if entry.mime() != record.mime() {
        return Match::Refused(Refusal::MimeMismatch);
    }
    if !media_class_matches_record(entry.media_class(), record) {
        return Match::Refused(Refusal::MediaClassMismatch);
    }
    if entry.width() != record.width() || entry.height() != record.height() {
        return Match::Refused(Refusal::DimensionsMismatch);
    }
    if record.grants_publication_authority() {
        return Match::Refused(Refusal::RecordAuthorityClaimed);
    }
    Match::MatchedForReview
}

/// Whether the declared media class describes the payload the record actually carries.
fn media_class_matches_record(
    class: PlatformAssetMediaClassV1,
    record: &GraphicImageInspectionV4,
) -> bool {
    match class {
        // Every accepted class in this slice is an image, and every V4 record carries typed image
        // evidence: a still plane, a GIF timeline, canonical vector text or a checked AVIF envelope.
        PlatformAssetMediaClassV1::Image => {
            record.format() == GraphicImageFormatV3::Avif
                || record.format() == GraphicImageFormatV3::Gif
                || record.format() == GraphicImageFormatV3::Svg
                || record.format() == GraphicImageFormatV3::Png
                || record.format() == GraphicImageFormatV3::Jpeg
                || record.format() == GraphicImageFormatV3::WebP
        }
    }
}

/// Binds a whole catalogue to an ordered record set, one entry to one record.
///
/// Both sides must correspond one-to-one. The **first** disagreement refuses the whole binding and
/// retains **no** earlier match; the refusal carries only a position and a closed category, never an
/// asset identifier, path, holder or evidence text.
#[must_use]
pub fn bind_catalogue_to_v4_evidence_v1(
    catalogue: &PlatformAssetCatalogueV1,
    records: &[GraphicImageInspectionV4],
    scope: PlatformCatalogueBindingScopeV1,
) -> PlatformCatalogueBindingV1 {
    if scope == PlatformCatalogueBindingScopeV1::PlatformSource {
        return PlatformCatalogueBindingV1::Refused {
            position: 0,
            kind: PlatformCatalogueMatchRefusalV1::PlatformSourceUnavailable,
        };
    }
    if catalogue.len() != records.len() {
        return PlatformCatalogueBindingV1::Refused {
            position: 0,
            kind: PlatformCatalogueMatchRefusalV1::CatalogueCountMismatch,
        };
    }
    for (position, (entry, record)) in catalogue.entries().iter().zip(records).enumerate() {
        match match_catalogue_entry_to_v4_evidence_v1(entry, record, scope) {
            PlatformCatalogueMatchV1::MatchedForReview => {}
            PlatformCatalogueMatchV1::Refused(kind) => {
                return PlatformCatalogueBindingV1::Refused { position, kind };
            }
        }
    }
    PlatformCatalogueBindingV1::CompleteForReview {
        matched: records.len(),
    }
}
