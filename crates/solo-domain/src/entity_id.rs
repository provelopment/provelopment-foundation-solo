//! Validated, language-neutral Entity identity (`EntityIdV1`).
//!
//! The accepted model has exactly one Entity whose identifier is a stable,
//! language-neutral fact. This type never lowercases, rewrites, truncates,
//! pads, generates or infers an identifier: a value that does not already match
//! the accepted pattern is refused, and the caller must author a correct one.
//! Changing the public base address never changes this value.

use core::fmt;

/// Maximum accepted length of an Entity ID, in ASCII bytes.
pub const ENTITY_ID_MAX_BYTES: usize = 63;

/// Why a candidate Entity ID was refused.
///
/// Every variant maps to the single engineering preflight code
/// `PREFLIGHT_ENTITY_ID_INVALID`; the variant exists so tests and diagnostics can
/// stay precise without exposing the refused input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityIdErrorKind {
    /// The value was empty.
    Empty,
    /// The value exceeded [`ENTITY_ID_MAX_BYTES`] bytes.
    TooLong,
    /// The value contained a non-ASCII byte.
    NonAscii,
    /// The first character was not an ASCII lowercase letter.
    InvalidStart,
    /// A character outside `[a-z0-9-]` appeared.
    InvalidCharacter,
    /// The value ended with a hyphen.
    TrailingHyphen,
    /// Two consecutive hyphens appeared.
    ConsecutiveHyphens,
}

impl EntityIdErrorKind {
    /// Stable, safe diagnostic token for this refusal reason.
    ///
    /// Tokens are fixed ASCII words, so no authored input can leak into a
    /// diagnostic through them.
    #[must_use]
    pub const fn detail(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::TooLong => "too-long",
            Self::NonAscii => "non-ascii",
            Self::InvalidStart => "invalid-start",
            Self::InvalidCharacter => "invalid-character",
            Self::TrailingHyphen => "trailing-hyphen",
            Self::ConsecutiveHyphens => "consecutive-hyphens",
        }
    }
}

/// Refusal of a candidate Entity ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntityIdError {
    kind: EntityIdErrorKind,
}

impl EntityIdError {
    const fn new(kind: EntityIdErrorKind) -> Self {
        Self { kind }
    }

    /// The refusal reason.
    #[must_use]
    pub const fn kind(&self) -> EntityIdErrorKind {
        self.kind
    }

    /// Stable engineering preflight code for this refusal.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        "PREFLIGHT_ENTITY_ID_INVALID"
    }

    /// Stable, safe diagnostic token for the refusal reason.
    #[must_use]
    pub const fn detail(&self) -> &'static str {
        self.kind.detail()
    }
}

impl fmt::Display for EntityIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code(), self.detail())
    }
}

impl std::error::Error for EntityIdError {}

/// A validated Entity identifier.
///
/// Construction is fallible and goes through [`EntityIdV1::parse`]; there is no
/// public constructor, builder or `From<&str>` conversion that could bypass the
/// domain rule.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EntityIdV1(String);

impl EntityIdV1 {
    /// Validates an authored identifier against `^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$`
    /// with a length of 1 to [`ENTITY_ID_MAX_BYTES`] ASCII bytes.
    ///
    /// The value is neither repaired nor inferred: any deviation is refused.
    pub fn parse(candidate: &str) -> Result<Self, EntityIdError> {
        if candidate.is_empty() {
            return Err(EntityIdError::new(EntityIdErrorKind::Empty));
        }
        if !candidate.is_ascii() {
            return Err(EntityIdError::new(EntityIdErrorKind::NonAscii));
        }
        let bytes = candidate.as_bytes();
        if bytes.len() > ENTITY_ID_MAX_BYTES {
            return Err(EntityIdError::new(EntityIdErrorKind::TooLong));
        }

        let mut seen_hyphen = false;
        for (index, byte) in bytes.iter().copied().enumerate() {
            if index == 0 {
                // Only a lowercase ASCII letter may open the identifier, so a leading
                // digit, separator or uppercase letter is refused rather than repaired.
                if !byte.is_ascii_lowercase() {
                    return Err(EntityIdError::new(EntityIdErrorKind::InvalidStart));
                }
                continue;
            }
            match byte {
                b'a'..=b'z' | b'0'..=b'9' => seen_hyphen = false,
                b'-' => {
                    if seen_hyphen {
                        return Err(EntityIdError::new(EntityIdErrorKind::ConsecutiveHyphens));
                    }
                    seen_hyphen = true;
                }
                _ => return Err(EntityIdError::new(EntityIdErrorKind::InvalidCharacter)),
            }
        }
        if seen_hyphen {
            return Err(EntityIdError::new(EntityIdErrorKind::TrailingHyphen));
        }

        Ok(Self(candidate.to_owned()))
    }

    /// The validated identifier as authored.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for EntityIdV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}
