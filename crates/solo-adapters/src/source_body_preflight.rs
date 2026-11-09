//! Inert byte preflight of one provisionally selected body.
//!
//! This module accepts **only** the bytes already supplied for the candidate the pure
//! selection policy chose. It performs no I/O: it opens no file, stats no path, resolves
//! no symlink and confirms no absence. Its result owns an immutable copy so a later
//! caller cannot change it, which is an **in-memory ownership** guarantee and explicitly
//! not a filesystem snapshot guarantee.
//!
//! **What is proven.** For a JSON body: bounded size, strict UTF-8 without a byte-order
//! mark, strict RFC 8259 JSON with no trailing input, no comments and no duplicate member
//! at any depth, and the closed accepted body root (§6.2E.1) — mandatory
//! `contentVersion: "1.0.0"`, mandatory non-empty `title` of at most 140 Unicode scalar
//! values, optional `summary` of at most 500, and a `blocks` array of 1–250 elements. For
//! a Markdown body: bounded size, strict UTF-8 without a byte-order mark, not empty, and
//! no rewriting.
//!
//! **What is explicitly not proven.** Node, `props`, `presentation` and module-registry
//! validity; CommonMark 0.31.2 or GFM 0.29-gfm parsing; raw-HTML refusal; heading and
//! anchor semantics; link and URL safety; BCP 47 or Unicode NFC conformance; and any
//! publication authority. A preflight success is never a publishable body, and
//! [`CONTENT_VALIDITY_PROVEN`] is `false` to keep that visible to callers.

use core::fmt;

use solo_domain::{
    ProvisionalSourceChoiceV1, SourceCandidateKind, SourceSelectionRefusal, SuppliedObservationsV1,
    select_provisional_source,
};

use crate::publication_manifest::{
    DeclaredNode, ManifestError, ParsedManifestCandidateV1, parse_manifest_bytes,
};

/// Maximum accepted JSON body size in bytes, per selected source (§6.2E.1).
pub const MAX_JSON_BODY_BYTES: usize = 2 * 1024 * 1024;

/// Maximum accepted Markdown body size in bytes, per selected source (§6.2D).
pub const MAX_MARKDOWN_BODY_BYTES: usize = 1024 * 1024;

/// The only accepted JSON body `contentVersion` in this slice (§6.2E.1).
pub const BODY_CONTENT_VERSION: &str = "1.0.0";

/// Maximum accepted `title` length in Unicode scalar values (§6.2E.1).
pub const MAX_TITLE_SCALARS: usize = 140;

/// Maximum accepted `summary` length in Unicode scalar values (§6.2E.1).
pub const MAX_SUMMARY_SCALARS: usize = 500;

/// Maximum accepted `blocks` elements (§6.2E.1).
pub const MAX_BLOCKS: usize = 250;

/// The four — and only four — accepted JSON body root keys (§6.2E.1).
pub const BODY_ROOT_KEYS: [&str; 4] = ["contentVersion", "title", "summary", "blocks"];

/// Whether a successful preflight proves the body is valid or publishable.
///
/// It never does. Full module, `props`, `presentation`, Markdown grammar and semantic
/// validation remain future gates, so a consumer must not read this module's success as
/// content validity or as publication authority.
pub const CONTENT_VALIDITY_PROVEN: bool = false;

/// The stable token the accepted [`DeclaredNode`] visitor uses to carry a duplicate-member
/// refusal out of map access.
///
/// It is mirrored here only to classify an already-refused parse; the accepted visitor in
/// `publication_manifest` remains the component that performs the refusal.
const DUPLICATE_MEMBER_TOKEN: &str = "solo-manifest-duplicate-key";

/// Why supplied body bytes were refused by preflight.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BodyPreflightRefusalKind {
    /// The supplied body was empty.
    Empty,
    /// The supplied body exceeded its accepted maximum size.
    TooLarge,
    /// The supplied body began with a UTF-8 byte-order mark.
    ByteOrderMark,
    /// The supplied body was not valid UTF-8.
    InvalidUtf8,
    /// The supplied body was not strict RFC 8259 JSON, or had trailing content.
    MalformedJson,
    /// A JSON object member was repeated, at the root or at any nesting depth.
    DuplicateMember,
    /// The JSON body root was not an object.
    NonObjectRoot,
    /// A mandatory JSON body root member was absent.
    MissingMember,
    /// A JSON body root member was not part of the closed accepted shape.
    UnknownMember,
    /// `contentVersion` was absent or not exactly the accepted value.
    UnsupportedContentVersion,
    /// `title` was not a plain string, or was empty, or exceeded its scalar bound.
    InvalidTitle,
    /// `summary` was not a plain string, or exceeded its scalar bound.
    InvalidSummary,
    /// `blocks` was not an array, or held fewer than 1 or more than 250 elements.
    InvalidBlocks,
}

impl BodyPreflightRefusalKind {
    /// Stable, safe diagnostic token for this refusal reason.
    #[must_use]
    pub const fn detail(self) -> &'static str {
        match self {
            Self::Empty => "the supplied body was empty",
            Self::TooLarge => "the supplied body exceeded its accepted maximum size",
            Self::ByteOrderMark => "the supplied body began with a byte-order mark",
            Self::InvalidUtf8 => "the supplied body was not valid UTF-8",
            Self::MalformedJson => "the supplied body was not strict JSON",
            Self::DuplicateMember => "the supplied JSON repeated a member name",
            Self::NonObjectRoot => "the supplied JSON body root was not an object",
            Self::MissingMember => "the supplied JSON body root omitted a mandatory member",
            Self::UnknownMember => "the supplied JSON body root carried an unknown member",
            Self::UnsupportedContentVersion => "the supplied JSON body contentVersion was refused",
            Self::InvalidTitle => "the supplied JSON body title was refused",
            Self::InvalidSummary => "the supplied JSON body summary was refused",
            Self::InvalidBlocks => "the supplied JSON body blocks were refused",
        }
    }
}

/// A refused body preflight.
///
/// The diagnostic names the refusal reason and a fixed location token, and it never
/// echoes authored body text, an absolute path or a secret.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BodyPreflightRefusal {
    kind: BodyPreflightRefusalKind,
    location: &'static str,
}

impl BodyPreflightRefusal {
    const fn new(kind: BodyPreflightRefusalKind, location: &'static str) -> Self {
        Self { kind, location }
    }

    /// The refusal reason.
    #[must_use]
    pub const fn kind(&self) -> BodyPreflightRefusalKind {
        self.kind
    }

    /// The fixed, safe location token the refusal concerns.
    #[must_use]
    pub const fn location(&self) -> &'static str {
        self.location
    }

    /// Stable engineering code for this refusal.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        "PREFLIGHT_SOURCE_BODY_REFUSED"
    }

    /// Stable, safe diagnostic token for the refusal reason.
    #[must_use]
    pub const fn detail(&self) -> &'static str {
        self.kind.detail()
    }
}

impl fmt::Display for BodyPreflightRefusal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}: {} (at `{}`)",
            self.code(),
            self.detail(),
            self.location
        )
    }
}

impl std::error::Error for BodyPreflightRefusal {}

/// An owned, immutable, **inert** preflight result for one selected body.
///
/// The bytes are a private owned copy: mutating or reusing the caller's buffer after the
/// call cannot change this value, and no public API hands out mutable access. That is an
/// in-memory ownership property only. It is not a filesystem snapshot, not content
/// validity and not publication authority.
#[derive(Debug, PartialEq, Eq)]
pub struct InertBodyCandidateV1 {
    kind: SourceCandidateKind,
    locator: String,
    bytes: Box<[u8]>,
}

impl InertBodyCandidateV1 {
    /// Which declared member this body was preflighted as.
    #[must_use]
    pub const fn kind(&self) -> SourceCandidateKind {
        self.kind
    }

    /// The chosen member's logical path, never a physical location.
    #[must_use]
    pub fn logical_path(&self) -> &str {
        &self.locator
    }

    /// The owned body bytes, readable but never mutable.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// How many bytes the owned copy holds.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Whether the owned copy is empty. Preflight refuses empty input, so this is `false`.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Whether this body may be published. It never may.
    #[must_use]
    pub const fn is_publication_authorized(&self) -> bool {
        false
    }

    /// Whether this body is a verified immutable filesystem snapshot. It never is.
    #[must_use]
    pub const fn is_verified_filesystem_snapshot(&self) -> bool {
        false
    }
}

fn refusal(kind: BodyPreflightRefusalKind, location: &'static str) -> BodyPreflightRefusal {
    BodyPreflightRefusal::new(kind, location)
}

/// Checks the size, emptiness and byte-order-mark envelope shared by both body kinds.
fn check_envelope(bytes: &[u8], maximum: usize) -> Result<(), BodyPreflightRefusal> {
    if bytes.is_empty() {
        return Err(refusal(BodyPreflightRefusalKind::Empty, "body"));
    }
    if bytes.len() > maximum {
        return Err(refusal(BodyPreflightRefusalKind::TooLarge, "body"));
    }
    if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        return Err(refusal(BodyPreflightRefusalKind::ByteOrderMark, "body"));
    }
    Ok(())
}

fn member<'a>(members: &'a [(String, DeclaredNode)], key: &str) -> Option<&'a DeclaredNode> {
    members
        .iter()
        .find(|(name, _)| name == key)
        .map(|(_, value)| value)
}

fn scalar_count(value: &str) -> usize {
    value.chars().count()
}

/// Validates the closed accepted JSON body root (§6.2E.1).
fn validate_json_root(node: &DeclaredNode) -> Result<(), BodyPreflightRefusal> {
    let DeclaredNode::Object(members) = node else {
        return Err(refusal(BodyPreflightRefusalKind::NonObjectRoot, "body"));
    };

    for (key, _) in members {
        if !BODY_ROOT_KEYS.contains(&key.as_str()) {
            return Err(refusal(BodyPreflightRefusalKind::UnknownMember, "body"));
        }
    }

    let version = member(members, "contentVersion")
        .ok_or(refusal(BodyPreflightRefusalKind::MissingMember, "contentVersion"))?;
    match version {
        DeclaredNode::String(value) if value.as_str() == BODY_CONTENT_VERSION => {}
        DeclaredNode::String(_) => {
            return Err(refusal(
                BodyPreflightRefusalKind::UnsupportedContentVersion,
                "contentVersion",
            ));
        }
        _ => {
            return Err(refusal(
                BodyPreflightRefusalKind::UnsupportedContentVersion,
                "contentVersion",
            ));
        }
    }

    let title = member(members, "title")
        .ok_or(refusal(BodyPreflightRefusalKind::MissingMember, "title"))?;
    let DeclaredNode::String(title) = title else {
        return Err(refusal(BodyPreflightRefusalKind::InvalidTitle, "title"));
    };
    let title_scalars = scalar_count(title);
    if title_scalars == 0 || title_scalars > MAX_TITLE_SCALARS {
        return Err(refusal(BodyPreflightRefusalKind::InvalidTitle, "title"));
    }

    if let Some(summary) = member(members, "summary") {
        let DeclaredNode::String(summary) = summary else {
            return Err(refusal(BodyPreflightRefusalKind::InvalidSummary, "summary"));
        };
        if scalar_count(summary) > MAX_SUMMARY_SCALARS {
            return Err(refusal(BodyPreflightRefusalKind::InvalidSummary, "summary"));
        }
    }

    let blocks = member(members, "blocks")
        .ok_or(refusal(BodyPreflightRefusalKind::MissingMember, "blocks"))?;
    let DeclaredNode::Array(items) = blocks else {
        return Err(refusal(BodyPreflightRefusalKind::InvalidBlocks, "blocks"));
    };
    if items.is_empty() || items.len() > MAX_BLOCKS {
        return Err(refusal(BodyPreflightRefusalKind::InvalidBlocks, "blocks"));
    }

    Ok(())
}

fn parse_json_document(bytes: &[u8]) -> Result<DeclaredNode, BodyPreflightRefusal> {
    let text = core::str::from_utf8(bytes)
        .map_err(|_| refusal(BodyPreflightRefusalKind::InvalidUtf8, "body"))?;
    // The accepted `DeclaredNode` visitor refuses a duplicate member during map access, so
    // no collapsing map is ever built; here the refusal is only classified.
    serde_json::from_str::<DeclaredNode>(text).map_err(|error| {
        if error.to_string().contains(DUPLICATE_MEMBER_TOKEN) {
            refusal(BodyPreflightRefusalKind::DuplicateMember, "body")
        } else {
            refusal(BodyPreflightRefusalKind::MalformedJson, "body")
        }
    })
}

/// Preflights the bytes of the **provisionally selected** candidate only.
///
/// The envelope is bounded before the bytes are copied, so an oversized body is refused
/// without allocating an owned copy equal to it. JSON is validated against the closed
/// accepted body root; Markdown receives the byte-level checks this slice authorizes.
///
/// # Errors
///
/// Returns a [`BodyPreflightRefusal`] for empty, oversized, byte-order-marked or invalid
/// UTF-8 input, for any JSON syntax or duplicate-member fault, and for any body-root shape
/// violation. A JSON refusal is final: there is no Markdown fallback and no second attempt.
pub fn preflight_selected_body(
    choice: &ProvisionalSourceChoiceV1<'_>,
) -> Result<InertBodyCandidateV1, BodyPreflightRefusal> {
    let bytes = choice.supplied_bytes();

    match choice.kind() {
        SourceCandidateKind::Json => {
            check_envelope(bytes, MAX_JSON_BODY_BYTES)?;
            let node = parse_json_document(bytes)?;
            validate_json_root(&node)?;
        }
        SourceCandidateKind::Markdown => {
            check_envelope(bytes, MAX_MARKDOWN_BODY_BYTES)?;
            core::str::from_utf8(bytes)
                .map_err(|_| refusal(BodyPreflightRefusalKind::InvalidUtf8, "body"))?;
        }
    }

    Ok(InertBodyCandidateV1 {
        kind: choice.kind(),
        locator: choice.logical_path().to_owned(),
        bytes: Box::from(bytes),
    })
}

/// Everything one in-memory source question needs, with explicit authored identifiers.
///
/// The identifiers are the **already parsed** EFS keys used only to locate a declaration.
/// They are never turned into a public route, a canonical locale key or a physical path.
#[derive(Debug, Clone, Copy)]
pub struct InMemorySourceRequestV1<'a> {
    manifest_bytes: &'a [u8],
    facet: &'a str,
    site: &'a str,
    page: &'a str,
    locale: &'a str,
    observations: SuppliedObservationsV1<'a>,
}

impl<'a> InMemorySourceRequestV1<'a> {
    /// Builds a request over already-supplied manifest bytes and typed observations.
    #[must_use]
    pub const fn new(
        manifest_bytes: &'a [u8],
        facet: &'a str,
        site: &'a str,
        page: &'a str,
        locale: &'a str,
        observations: SuppliedObservationsV1<'a>,
    ) -> Self {
        Self {
            manifest_bytes,
            facet,
            site,
            page,
            locale,
            observations,
        }
    }
}

/// Why the in-memory pipeline refused.
#[derive(Debug)]
pub enum InMemorySourceRefusalV1 {
    /// The supplied manifest bytes were refused by the accepted strict parser.
    Manifest(ManifestError),
    /// No declaration matched the supplied Facet/Site/Page/locale identifiers.
    UnknownRepresentation,
    /// The pure selection policy refused the supplied observations.
    Selection(SourceSelectionRefusal),
    /// The selected body failed inert byte preflight.
    Body(BodyPreflightRefusal),
}

impl InMemorySourceRefusalV1 {
    /// Stable engineering code for this refusal, delegated where one already exists.
    #[must_use]
    pub fn code(&self) -> &'static str {
        match self {
            Self::Manifest(error) => error.code(),
            Self::UnknownRepresentation => "PREFLIGHT_SOURCE_PIPELINE_UNKNOWN_REPRESENTATION",
            Self::Selection(error) => error.code(),
            Self::Body(error) => error.code(),
        }
    }
}

impl fmt::Display for InMemorySourceRefusalV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Manifest(error) => formatter.write_str(error.code()),
            Self::UnknownRepresentation => formatter.write_str(self.code()),
            Self::Selection(error) => write!(formatter, "{error}"),
            Self::Body(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for InMemorySourceRefusalV1 {}

fn find_representation<'m>(
    parsed: &'m ParsedManifestCandidateV1,
    request: &InMemorySourceRequestV1<'_>,
) -> Option<&'m solo_domain::SourceCandidateSetV1> {
    for facet_node in parsed.entity().facets() {
        if facet_node.id().as_str() != request.facet {
            continue;
        }
        for site_node in facet_node.sites() {
            if site_node.id().as_str() != request.site {
                continue;
            }
            for page_node in site_node.pages() {
                if page_node.id().as_str() != request.page {
                    continue;
                }
                for representation in page_node.representations() {
                    if representation.locale() == request.locale {
                        return Some(representation.source_candidates());
                    }
                }
            }
        }
    }
    None
}

/// The one documented in-memory source pipeline.
///
/// It parses the sole manifest's already-supplied bytes with the accepted strict parser,
/// walks the one Entity/Facet/Site/Page lineage to the named representation, takes that
/// representation's already-parsed candidate set, selects provisionally over the supplied
/// observations, and preflights **only** the selected bytes. Nothing is read from disk, no
/// unselected body is requested or parsed, and the result stays inert.
///
/// # Errors
///
/// Returns [`InMemorySourceRefusalV1`] for manifest refusal, an unmatched representation,
/// a selection refusal, or a body preflight refusal. A present-but-invalid JSON body is a
/// body refusal, never a reason to try a declared Markdown body.
pub fn run_in_memory_source_pipeline(
    request: &InMemorySourceRequestV1<'_>,
) -> Result<InertBodyCandidateV1, InMemorySourceRefusalV1> {
    let supplied = parse_manifest_bytes(request.manifest_bytes);
    let parsed = supplied.map_err(InMemorySourceRefusalV1::Manifest)?;

    let unknown = InMemorySourceRefusalV1::UnknownRepresentation;
    let found = find_representation(&parsed, request);
    let candidates = found.ok_or(unknown)?;

    let selected = select_provisional_source(candidates, request.observations);
    let choice = selected.map_err(InMemorySourceRefusalV1::Selection)?;

    let preflighted = preflight_selected_body(&choice);
    preflighted.map_err(InMemorySourceRefusalV1::Body)
}
