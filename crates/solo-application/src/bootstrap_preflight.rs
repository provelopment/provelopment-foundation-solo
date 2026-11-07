//! The `BootstrapPreflightV1` inbound use case.
//!
//! The use case answers one engineering question only: given explicitly authored
//! values, is this Entity identifier valid, and is this base address acceptable to
//! the narrow preflight subset? It creates no Installation, publishes nothing,
//! renders nothing, serves nothing and reads no manifest. It has no outbound port
//! because it has no external effect at all.

use solo_domain::{BaseCandidateV1, EntityIdV1};

/// Stable engine-level refusal code for a refused preflight.
///
/// These are **engineering preflight codes**, not entries in the future §6.2K
/// production diagnostic registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BootstrapRefusalCode {
    /// The authored Entity identifier was refused.
    EntityIdInvalid,
    /// The authored base address was malformed or ambiguous.
    BaseInvalid,
    /// The authored base address is a legitimate shape this preflight does not
    /// implement yet.
    BaseUnsupported,
}

impl BootstrapRefusalCode {
    /// The stable code spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EntityIdInvalid => "PREFLIGHT_ENTITY_ID_INVALID",
            Self::BaseInvalid => "PREFLIGHT_BASE_INVALID",
            Self::BaseUnsupported => "PREFLIGHT_BASE_UNSUPPORTED",
        }
    }
}

/// Raw, explicitly authored bootstrap input, as supplied across the boundary.
///
/// Values are unvalidated strings on purpose: the adapter translates authored text
/// into this shape, and the use case — not the adapter — decides validity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapInputV1 {
    entity_id: String,
    base_url: String,
}

impl BootstrapInputV1 {
    /// Creates the raw input record. No validation happens here by design.
    #[must_use]
    pub fn new(entity_id: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self {
            entity_id: entity_id.into(),
            base_url: base_url.into(),
        }
    }

    /// The authored Entity identifier text.
    #[must_use]
    pub fn entity_id(&self) -> &str {
        &self.entity_id
    }

    /// The authored base address text.
    #[must_use]
    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}

/// A successful preflight result.
///
/// It carries only the two validated value objects; it is not an Installation, not
/// a publication and not a persisted record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapAcceptedV1 {
    entity_id: EntityIdV1,
    base: BaseCandidateV1,
}

impl BootstrapAcceptedV1 {
    /// The validated Entity identifier.
    #[must_use]
    pub fn entity_id(&self) -> &EntityIdV1 {
        &self.entity_id
    }

    /// The validated base candidate.
    #[must_use]
    pub fn base(&self) -> &BaseCandidateV1 {
        &self.base
    }
}

/// A refusal result with a stable code and a bounded, safe detail token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapRefusedV1 {
    code: BootstrapRefusalCode,
    detail: &'static str,
}

impl BootstrapRefusedV1 {
    const fn new(code: BootstrapRefusalCode, detail: &'static str) -> Self {
        Self { code, detail }
    }

    /// The stable refusal code.
    #[must_use]
    pub const fn code(&self) -> BootstrapRefusalCode {
        self.code
    }

    /// The stable, safe detail token.
    ///
    /// Detail tokens are fixed ASCII words drawn from the domain refusals, so no
    /// authored input can appear in a diagnostic.
    #[must_use]
    pub const fn detail(&self) -> &'static str {
        self.detail
    }
}

/// The typed result of the inbound use case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BootstrapOutcomeV1 {
    /// Both authored values were accepted.
    Accepted(BootstrapAcceptedV1),
    /// At least one authored value was refused.
    Refused(BootstrapRefusedV1),
}

/// The `BootstrapPreflightV1` inbound contract.
///
/// A driving adapter depends on this interface, never on the concrete service.
pub trait BootstrapPreflightV1 {
    /// Validates the authored input and reports a typed outcome.
    fn execute(&self, input: BootstrapInputV1) -> BootstrapOutcomeV1;
}

/// The concrete application service implementing [`BootstrapPreflightV1`].
///
/// The value is stateless: identical input always yields an identical outcome and
/// no call leaves any trace behind.
#[derive(Debug, Clone, Copy, Default)]
pub struct BootstrapPreflightService;

impl BootstrapPreflightV1 for BootstrapPreflightService {
    fn execute(&self, input: BootstrapInputV1) -> BootstrapOutcomeV1 {
        // Identity is validated first so that the reported refusal is
        // deterministic when both authored values are wrong.
        let entity_id = match EntityIdV1::parse(input.entity_id()) {
            Ok(value) => value,
            Err(error) => {
                return BootstrapOutcomeV1::Refused(BootstrapRefusedV1::new(
                    BootstrapRefusalCode::EntityIdInvalid,
                    error.detail(),
                ));
            }
        };

        let base = match BaseCandidateV1::parse(input.base_url()) {
            Ok(value) => value,
            Err(error) => {
                let code = match error.kind() {
                    solo_domain::BaseCandidateErrorKind::Invalid => {
                        BootstrapRefusalCode::BaseInvalid
                    }
                    solo_domain::BaseCandidateErrorKind::Unsupported => {
                        BootstrapRefusalCode::BaseUnsupported
                    }
                };
                return BootstrapOutcomeV1::Refused(BootstrapRefusedV1::new(code, error.detail()));
            }
        };

        BootstrapOutcomeV1::Accepted(BootstrapAcceptedV1 { entity_id, base })
    }
}
