//! Validated logical `sourceCandidates` locators.
//!
//! This module implements the **pure lexical** half of Architecture §6.2C.1–§6.2C.2
//! and the `sourceCandidates` declaration shape of §6.2B. It decides only whether an
//! authored declaration string is an acceptable logical candidate path, and whether
//! two declarations claim the same one. It opens no file, tests no existence, resolves
//! no symlink, selects no body and never chooses between JSON and Markdown.
//!
//! **What is explicitly not proven here.** Logical path equality is *not* filesystem
//! alias, hardlink, junction or case-equivalence identity (§6.2C.3–§6.2C.4): those
//! require the future no-follow snapshot adapter and are represented by
//! [`FILESYSTEM_IDENTITY_PROVEN`], which is `false`. A path that passes this module is
//! a syntactically portable *claim*, not a confined, existing or owned source.

use core::fmt;

/// Maximum accepted length of one locator, in ASCII bytes (§6.2C.1).
pub const MAX_LOCATOR_BYTES: usize = 240;

/// Maximum accepted number of slash-separated segments in one locator (§6.2C.1).
pub const MAX_LOCATOR_SEGMENTS: usize = 8;

/// Maximum accepted length of one segment, in ASCII bytes (§6.2C.1).
pub const MAX_SEGMENT_BYTES: usize = 80;

/// The one logical namespace Page candidate bodies may live under (§6.2A.4).
pub const SOURCE_NAMESPACE: &str = "pages";

/// Windows reserved basenames/stems, refused even when an extension follows
/// (§6.2C.2). Stored without extension.
pub const RESERVED_WINDOWS_STEMS: [&str; 22] = [
    "con", "prn", "aux", "nul", "com1", "com2", "com3", "com4", "com5", "com6", "com7", "com8",
    "com9", "lpt1", "lpt2", "lpt3", "lpt4", "lpt5", "lpt6", "lpt7", "lpt8", "lpt9",
];

/// Whether this slice proves filesystem identity for a candidate claim.
///
/// It never does: alias, hardlink and case-equivalence identity belong to the future
/// confined snapshot adapter. A consumer that needs that guarantee must not read this
/// module's equality as if it provided it.
pub const FILESYSTEM_IDENTITY_PROVEN: bool = false;

/// The two named `sourceCandidates` members (§6.2B), each with its mandatory exact
/// lowercase extension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SourceCandidateKind {
    /// The `json` member, whose locator must end in `.json`.
    Json,
    /// The `markdown` member, whose locator must end in `.md`.
    Markdown,
}

impl SourceCandidateKind {
    /// The declaration member name as authored.
    #[must_use]
    pub const fn member(self) -> &'static str {
        match self {
            Self::Json => "json",
            Self::Markdown => "markdown",
        }
    }

    /// The mandatory dot-prefixed lowercase extension.
    #[must_use]
    pub const fn extension(self) -> &'static str {
        match self {
            Self::Json => ".json",
            Self::Markdown => ".md",
        }
    }

    /// The extension of the other kind, used only to name a precise refusal.
    const fn other_extension(self) -> &'static str {
        match self {
            Self::Json => ".md",
            Self::Markdown => ".json",
        }
    }
}

impl fmt::Display for SourceCandidateKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.member())
    }
}

/// Why a declared locator or candidate set was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceLocatorErrorKind {
    /// The declaration was the empty string.
    Empty,
    /// The declaration exceeded [`MAX_LOCATOR_BYTES`].
    TooLong,
    /// The declaration exceeded [`MAX_LOCATOR_SEGMENTS`].
    TooManySegments,
    /// A non-ASCII byte appeared.
    NonAscii,
    /// A C0/C1 control character or NUL appeared.
    ControlCharacter,
    /// A backslash appeared; separators are always `/`.
    Backslash,
    /// The path started with `/`, making it absolute.
    LeadingSlash,
    /// The path ended with `/`.
    TrailingSlash,
    /// Two separators appeared together, or a segment was empty.
    EmptySegment,
    /// A `.` segment appeared.
    DotSegment,
    /// A `..` segment appeared.
    DotDotSegment,
    /// A colon appeared, which also covers drive letters and `scheme:` forms.
    Colon,
    /// A `~` appeared, which is shell expansion rather than a literal name.
    Tilde,
    /// A `?` or `#` appeared, which makes the value a URL reference.
    QueryOrFragment,
    /// A `%` appeared, which would be an encoded path component.
    PercentEscape,
    /// The first segment was not the logical [`SOURCE_NAMESPACE`].
    NotUnderSourceNamespace,
    /// One segment exceeded [`MAX_SEGMENT_BYTES`].
    SegmentTooLong,
    /// A segment did not start with a lowercase ASCII letter.
    SegmentInvalidStart,
    /// A segment contained a byte outside `[a-z0-9._-]`.
    SegmentInvalidCharacter,
    /// A segment ended with a dot.
    SegmentTrailingDot,
    /// A segment contained two consecutive dots.
    SegmentConsecutiveDots,
    /// A segment's stem was a reserved Windows basename.
    ReservedWindowsStem,
    /// The locator did not end with the extension required by its named kind.
    WrongExtension,
    /// The locator carried the *other* kind's extension, an extension mismatch.
    ExtensionMismatch,
    /// A `sourceCandidates` object declared no member at all.
    EmptyCandidateSet,
    /// The same member name was declared twice.
    DuplicateMember,
    /// A member was declared as `null` rather than omitted.
    NullMember,
    /// A member key was not `json` or `markdown`.
    UnknownMember,
}

impl SourceLocatorErrorKind {
    /// Stable, safe diagnostic token for this refusal reason.
    ///
    /// The tokens are fixed ASCII words, so no authored value can leak into a
    /// diagnostic through them.
    #[must_use]
    pub const fn detail(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::TooLong => "too-long",
            Self::TooManySegments => "too-many-segments",
            Self::NonAscii => "non-ascii",
            Self::ControlCharacter => "control-character",
            Self::Backslash => "backslash",
            Self::LeadingSlash => "leading-slash",
            Self::TrailingSlash => "trailing-slash",
            Self::EmptySegment => "empty-segment",
            Self::DotSegment => "dot-segment",
            Self::DotDotSegment => "dot-dot-segment",
            Self::Colon => "colon",
            Self::Tilde => "tilde",
            Self::QueryOrFragment => "query-or-fragment",
            Self::PercentEscape => "percent-escape",
            Self::NotUnderSourceNamespace => "not-under-source-namespace",
            Self::SegmentTooLong => "segment-too-long",
            Self::SegmentInvalidStart => "segment-invalid-start",
            Self::SegmentInvalidCharacter => "segment-invalid-character",
            Self::SegmentTrailingDot => "segment-trailing-dot",
            Self::SegmentConsecutiveDots => "segment-consecutive-dots",
            Self::ReservedWindowsStem => "reserved-windows-stem",
            Self::WrongExtension => "wrong-extension",
            Self::ExtensionMismatch => "extension-mismatch",
            Self::EmptyCandidateSet => "empty-candidate-set",
            Self::DuplicateMember => "duplicate-member",
            Self::NullMember => "null-member",
            Self::UnknownMember => "unknown-member",
        }
    }
}

/// Refusal of a declared locator or candidate set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceLocatorError {
    kind: SourceLocatorErrorKind,
    segment: Option<usize>,
}

impl SourceLocatorError {
    const fn at(kind: SourceLocatorErrorKind, segment: Option<usize>) -> Self {
        Self { kind, segment }
    }

    /// The refusal reason.
    #[must_use]
    pub const fn kind(&self) -> SourceLocatorErrorKind {
        self.kind
    }

    /// The 1-based segment index the refusal applies to, when it applies to one.
    ///
    /// Only the position is reported, never the refused text.
    #[must_use]
    pub const fn segment(&self) -> Option<usize> {
        self.segment
    }

    /// Stable engineering code for this refusal.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        "PREFLIGHT_SOURCE_LOCATOR_INVALID"
    }

    /// Stable, safe diagnostic token for the refusal reason.
    #[must_use]
    pub const fn detail(&self) -> &'static str {
        self.kind.detail()
    }
}

impl fmt::Display for SourceLocatorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code(), self.detail())?;
        if let Some(segment) = self.segment {
            write!(formatter, " (segment {segment})")?;
        }
        Ok(())
    }
}

impl std::error::Error for SourceLocatorError {}

/// One validated logical `sourceCandidates` path.
///
/// Construction is fallible and goes through [`SourceCandidatePathV1::parse`]; there is
/// no public constructor, builder or `From<&str>` that could bypass the rule. A value
/// is neither normalized into validity nor repaired: it is either already portable or
/// refused.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceCandidatePathV1 {
    path: String,
    kind: SourceCandidateKind,
}

impl SourceCandidatePathV1 {
    /// Validates one declared locator against [`SourceCandidateKind`].
    ///
    /// The path must be a relative, slash-separated, portable **lowercase ASCII**
    /// locator under the logical `pages/` namespace and must end with exactly the
    /// extension of its named kind. Nothing is inferred: a missing, doubled or
    /// mismatched extension is refused rather than corrected.
    pub fn parse(candidate: &str, kind: SourceCandidateKind) -> Result<Self, SourceLocatorError> {
        if candidate.is_empty() {
            return Err(SourceLocatorError::at(SourceLocatorErrorKind::Empty, None));
        }
        // Only ASCII is portable here, so the byte ceiling is also a character ceiling
        // for any value that could ever be accepted.
        if candidate.len() > MAX_LOCATOR_BYTES {
            return Err(SourceLocatorError::at(
                SourceLocatorErrorKind::TooLong,
                None,
            ));
        }
        for byte in candidate.bytes() {
            if byte == 0x00 || byte < 0x20 || byte == 0x7F {
                return Err(SourceLocatorError::at(
                    SourceLocatorErrorKind::ControlCharacter,
                    None,
                ));
            }
        }
        if !candidate.is_ascii() {
            return Err(SourceLocatorError::at(
                SourceLocatorErrorKind::NonAscii,
                None,
            ));
        }
        if candidate.contains('\\') {
            return Err(SourceLocatorError::at(
                SourceLocatorErrorKind::Backslash,
                None,
            ));
        }
        if candidate.starts_with('/') {
            return Err(SourceLocatorError::at(
                SourceLocatorErrorKind::LeadingSlash,
                None,
            ));
        }
        if candidate.ends_with('/') {
            return Err(SourceLocatorError::at(
                SourceLocatorErrorKind::TrailingSlash,
                None,
            ));
        }
        if candidate.contains(':') {
            return Err(SourceLocatorError::at(SourceLocatorErrorKind::Colon, None));
        }
        if candidate.contains('~') {
            return Err(SourceLocatorError::at(SourceLocatorErrorKind::Tilde, None));
        }
        if candidate.contains('?') || candidate.contains('#') {
            return Err(SourceLocatorError::at(
                SourceLocatorErrorKind::QueryOrFragment,
                None,
            ));
        }
        if candidate.contains('%') {
            return Err(SourceLocatorError::at(
                SourceLocatorErrorKind::PercentEscape,
                None,
            ));
        }

        let segments: Vec<&str> = candidate.split('/').collect();
        if segments.len() > MAX_LOCATOR_SEGMENTS {
            return Err(SourceLocatorError::at(
                SourceLocatorErrorKind::TooManySegments,
                None,
            ));
        }
        if segments.iter().any(|segment| segment.is_empty()) {
            return Err(SourceLocatorError::at(
                SourceLocatorErrorKind::EmptySegment,
                None,
            ));
        }

        for (index, segment) in segments.iter().enumerate() {
            let position = Some(index + 1);
            if *segment == "." {
                return Err(SourceLocatorError::at(
                    SourceLocatorErrorKind::DotSegment,
                    position,
                ));
            }
            if *segment == ".." {
                return Err(SourceLocatorError::at(
                    SourceLocatorErrorKind::DotDotSegment,
                    position,
                ));
            }
            if index == 0 {
                if *segment != SOURCE_NAMESPACE {
                    return Err(SourceLocatorError::at(
                        SourceLocatorErrorKind::NotUnderSourceNamespace,
                        position,
                    ));
                }
                continue;
            }
            validate_segment(segment, position)?;
        }

        // The value is non-empty and holds no empty segment, so the last segment exists.
        // Because the first segment must be the namespace, a body locator always has at
        // least two segments and the last one carries the extension.
        let last = segments[segments.len() - 1];
        if last.ends_with(kind.other_extension()) {
            return Err(SourceLocatorError::at(
                SourceLocatorErrorKind::ExtensionMismatch,
                None,
            ));
        }
        if !last.ends_with(kind.extension()) {
            return Err(SourceLocatorError::at(
                SourceLocatorErrorKind::WrongExtension,
                None,
            ));
        }
        // A name must remain after the extension, so `pages/.json` is refused rather
        // than accepted as an unnamed body.
        if last.len() == kind.extension().len() {
            return Err(SourceLocatorError::at(
                SourceLocatorErrorKind::SegmentInvalidStart,
                None,
            ));
        }

        Ok(Self {
            path: candidate.to_owned(),
            kind,
        })
    }

    /// The validated locator exactly as authored.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.path
    }

    /// The named kind whose extension this locator must carry.
    #[must_use]
    pub const fn kind(&self) -> SourceCandidateKind {
        self.kind
    }
}

impl fmt::Display for SourceCandidatePathV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.path)
    }
}

fn validate_segment(segment: &str, position: Option<usize>) -> Result<(), SourceLocatorError> {
    if segment.len() > MAX_SEGMENT_BYTES {
        return Err(SourceLocatorError::at(
            SourceLocatorErrorKind::SegmentTooLong,
            position,
        ));
    }
    let Some(start) = segment.bytes().next() else {
        return Err(SourceLocatorError::at(
            SourceLocatorErrorKind::EmptySegment,
            position,
        ));
    };
    if !start.is_ascii_lowercase() {
        return Err(SourceLocatorError::at(
            SourceLocatorErrorKind::SegmentInvalidStart,
            position,
        ));
    }
    if segment.ends_with('.') {
        return Err(SourceLocatorError::at(
            SourceLocatorErrorKind::SegmentTrailingDot,
            position,
        ));
    }
    if segment.contains("..") {
        return Err(SourceLocatorError::at(
            SourceLocatorErrorKind::SegmentConsecutiveDots,
            position,
        ));
    }
    for byte in segment.bytes() {
        if !matches!(byte, b'a'..=b'z' | b'0'..=b'9' | b'.' | b'_' | b'-') {
            return Err(SourceLocatorError::at(
                SourceLocatorErrorKind::SegmentInvalidCharacter,
                position,
            ));
        }
    }
    // Windows treats a reserved stem as reserved regardless of what follows it, so the
    // stem is everything before the first dot.
    let stem = segment.split('.').next().unwrap_or(segment);
    if RESERVED_WINDOWS_STEMS.contains(&stem) {
        return Err(SourceLocatorError::at(
            SourceLocatorErrorKind::ReservedWindowsStem,
            position,
        ));
    }
    Ok(())
}

/// The builder for one `sourceCandidates` object.
///
/// A member is declared by *setting* it, so omission is the only way to express
/// absence. Declaring a member twice is refused rather than overwritten, and an empty
/// object is refused outright — matching §6.2B, where one or both members are required
/// and `null` is never a valid member value.
#[derive(Debug, Clone, Default)]
pub struct SourceCandidateDraftV1 {
    json: Option<SourceCandidatePathV1>,
    markdown: Option<SourceCandidatePathV1>,
}

impl SourceCandidateDraftV1 {
    /// A draft declaring nothing yet.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Declares the `json` member.
    pub fn json(&mut self, path: SourceCandidatePathV1) -> Result<&mut Self, SourceLocatorError> {
        if self.json.is_some() {
            return Err(SourceLocatorError::at(
                SourceLocatorErrorKind::DuplicateMember,
                None,
            ));
        }
        self.json = Some(path);
        Ok(self)
    }

    /// Declares the `markdown` member.
    pub fn markdown(
        &mut self,
        path: SourceCandidatePathV1,
    ) -> Result<&mut Self, SourceLocatorError> {
        if self.markdown.is_some() {
            return Err(SourceLocatorError::at(
                SourceLocatorErrorKind::DuplicateMember,
                None,
            ));
        }
        self.markdown = Some(path);
        Ok(self)
    }

    /// Validates the declared set.
    pub fn build(self) -> Result<SourceCandidateSetV1, SourceLocatorError> {
        if self.json.is_none() && self.markdown.is_none() {
            return Err(SourceLocatorError::at(
                SourceLocatorErrorKind::EmptyCandidateSet,
                None,
            ));
        }
        Ok(SourceCandidateSetV1 {
            json: self.json,
            markdown: self.markdown,
        })
    }
}

/// A validated, non-empty `sourceCandidates` declaration.
///
/// Holding one proves only that the *declaration* is well formed. It says nothing about
/// whether either file exists, is confined, is owned by this representation, or holds
/// valid content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceCandidateSetV1 {
    json: Option<SourceCandidatePathV1>,
    markdown: Option<SourceCandidatePathV1>,
}

impl SourceCandidateSetV1 {
    /// The declared `json` locator, if any.
    #[must_use]
    pub fn json(&self) -> Option<&SourceCandidatePathV1> {
        self.json.as_ref()
    }

    /// The declared `markdown` locator, if any.
    #[must_use]
    pub fn markdown(&self) -> Option<&SourceCandidatePathV1> {
        self.markdown.as_ref()
    }

    /// Both declarations in authored-member order (`json`, then `markdown`).
    pub fn iter(&self) -> impl Iterator<Item = &SourceCandidatePathV1> {
        self.json.iter().chain(self.markdown.iter())
    }

    /// How many members were declared; always one or two.
    #[must_use]
    pub fn len(&self) -> usize {
        let mut declared = 0;
        if self.json.is_some() {
            declared += 1;
        }
        if self.markdown.is_some() {
            declared += 1;
        }
        declared
    }

    /// Whether no member was declared. Always `false`: the set is non-empty by
    /// construction, so this exists only to keep the Rust API idiomatic.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Why a declaration claim was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceClaimErrorKind {
    /// Two distinct representations reserved the same logical candidate path.
    DuplicateClaim,
}

impl SourceClaimErrorKind {
    /// Stable, safe diagnostic token for this refusal reason.
    #[must_use]
    pub const fn detail(self) -> &'static str {
        match self {
            Self::DuplicateClaim => "duplicate-claim",
        }
    }
}

/// Refusal of a duplicate logical candidate claim.
///
/// Owners are reported by the opaque integer key the caller supplied, so no authored
/// text can reach a diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceClaimError {
    kind: SourceClaimErrorKind,
    first: u64,
    second: u64,
}

impl SourceClaimError {
    /// The refusal reason.
    #[must_use]
    pub const fn kind(&self) -> SourceClaimErrorKind {
        self.kind
    }

    /// The owner key that already held the claim.
    #[must_use]
    pub const fn first(&self) -> u64 {
        self.first
    }

    /// The owner key whose claim was refused.
    #[must_use]
    pub const fn second(&self) -> u64 {
        self.second
    }

    /// Stable engineering code for this refusal.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        "PREFLIGHT_SOURCE_CLAIM_DUPLICATE"
    }

    /// Stable, safe diagnostic token for the refusal reason.
    #[must_use]
    pub const fn detail(&self) -> &'static str {
        self.kind.detail()
    }
}

impl fmt::Display for SourceClaimError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}: {} (owners {} and {})",
            self.code(),
            self.detail(),
            self.first,
            self.second
        )
    }
}

impl std::error::Error for SourceClaimError {}

/// Reservations of logical candidate paths across representations.
///
/// A claim is registered per **declared** path, so an absent file still reserves its
/// path (§6.2C.4). The decision never depends on declaration order: whatever the order,
/// the second distinct owner of one logical path is refused. Comparison is **logical
/// only** — see [`FILESYSTEM_IDENTITY_PROVEN`].
#[derive(Debug, Clone, Default)]
pub struct SourceCandidateReservationsV1 {
    claimed: std::collections::BTreeMap<String, u64>,
}

impl SourceCandidateReservationsV1 {
    /// An empty reservation set.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// How many logical paths are reserved.
    #[must_use]
    pub fn len(&self) -> usize {
        self.claimed.len()
    }

    /// Whether nothing is reserved yet.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.claimed.is_empty()
    }

    /// Reserves every declared candidate of `owner`.
    ///
    /// Either all claims are registered or none is: on a duplicate the reservation set is
    /// left exactly as it was before the call, so a refused owner never half-claims.
    pub fn reserve(
        &mut self,
        owner: u64,
        candidates: &SourceCandidateSetV1,
    ) -> Result<(), SourceClaimError> {
        for candidate in candidates.iter() {
            if let Some(existing) = self.claimed.get(candidate.as_str())
                && *existing != owner
            {
                return Err(SourceClaimError {
                    kind: SourceClaimErrorKind::DuplicateClaim,
                    first: *existing,
                    second: owner,
                });
            }
        }
        for candidate in candidates.iter() {
            self.claimed
                .entry(candidate.as_str().to_owned())
                .or_insert(owner);
        }
        Ok(())
    }

    /// The owner holding a logical path, if it is reserved.
    #[must_use]
    pub fn owner_of(&self, path: &str) -> Option<u64> {
        self.claimed.get(path).copied()
    }
}
