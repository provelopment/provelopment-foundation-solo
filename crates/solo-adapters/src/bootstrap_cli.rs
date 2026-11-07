//! CLI translation and rendering for `bootstrap-preflight`.
//!
//! Every option must be authored explicitly, exactly once, with one separately
//! supplied nonempty UTF-8 value. There is no interactive prompt, no environment
//! or working-directory fallback, and no URL or identifier inference. Diagnostics
//! are built only from fixed ASCII tokens, so no authored input — and therefore no
//! control sequence, secret, path or unfiltered text — can reach them.

use std::ffi::OsString;

use solo_application::{BootstrapInputV1, BootstrapOutcomeV1, BootstrapPreflightV1};

/// Exit status for an accepted preflight.
pub const EXIT_OK: u8 = 0;
/// Exit status for an internal error.
pub const EXIT_INTERNAL: u8 = 1;
/// Exit status for a refused preflight.
pub const EXIT_REFUSED: u8 = 2;

/// The single supported subcommand.
pub const SUBCOMMAND: &str = "bootstrap-preflight";
/// The Entity identifier option.
pub const FLAG_ENTITY_ID: &str = "--entity-id";
/// The base address option.
pub const FLAG_BASE_URL: &str = "--base-url";

/// Argument-level refusal codes.
///
/// These are engineering preflight codes; they are not entries in the future
/// §6.2K production diagnostic registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CliRefusalCode {
    /// A required argument was absent.
    Missing,
    /// An option was supplied more than once.
    Duplicate,
    /// An unrecognized flag, subcommand or positional token was supplied.
    Unknown,
    /// An argument was not valid UTF-8.
    Encoding,
}

impl CliRefusalCode {
    /// The stable code spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Missing => "PREFLIGHT_ARGS_MISSING",
            Self::Duplicate => "PREFLIGHT_ARGS_DUPLICATE",
            Self::Unknown => "PREFLIGHT_ARGS_UNKNOWN",
            Self::Encoding => "PREFLIGHT_ARGS_ENCODING",
        }
    }
}

/// An argument-level refusal with a stable code and safe detail token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CliRefusal {
    code: CliRefusalCode,
    detail: &'static str,
}

impl CliRefusal {
    const fn new(code: CliRefusalCode, detail: &'static str) -> Self {
        Self { code, detail }
    }

    /// The stable refusal code.
    #[must_use]
    pub const fn code(self) -> CliRefusalCode {
        self.code
    }

    /// The stable, safe detail token.
    #[must_use]
    pub const fn detail(self) -> &'static str {
        self.detail
    }
}

/// A rendered CLI outcome, ready for the composition root to emit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CliOutcome {
    /// The preflight accepted both authored values.
    Accepted {
        /// The single deterministic success line, without its terminator.
        line: String,
    },
    /// The preflight refused an authored value, or the arguments were unusable.
    Refused {
        /// The stable refusal code.
        code: &'static str,
        /// The stable detail token.
        detail: &'static str,
    },
    /// The process could not complete for an internal reason.
    Internal {
        /// The stable detail token.
        detail: &'static str,
    },
}

impl CliOutcome {
    /// The process exit status for this outcome.
    #[must_use]
    pub const fn exit_code(&self) -> u8 {
        match self {
            Self::Accepted { .. } => EXIT_OK,
            Self::Refused { .. } => EXIT_REFUSED,
            Self::Internal { .. } => EXIT_INTERNAL,
        }
    }

    /// The stdout payload, or `None` when nothing may be written to stdout.
    #[must_use]
    pub fn stdout(&self) -> Option<String> {
        match self {
            Self::Accepted { line } => Some(format!("{line}\n")),
            Self::Refused { .. } | Self::Internal { .. } => None,
        }
    }

    /// The stderr payload, or `None` when nothing may be written to stderr.
    #[must_use]
    pub fn stderr(&self) -> Option<String> {
        match self {
            Self::Accepted { .. } => None,
            Self::Refused { code, detail } => Some(format!("error: {code}: {detail}\n")),
            Self::Internal { detail } => Some(format!("error: PREFLIGHT_INTERNAL: {detail}\n")),
        }
    }
}

/// Translates raw OS arguments into typed application input.
///
/// The first item is the program name and is ignored. Every remaining token must be
/// valid UTF-8.
pub fn parse_request<I>(args: I) -> Result<BootstrapInputV1, CliRefusal>
where
    I: IntoIterator<Item = OsString>,
{
    let mut tokens = Vec::new();
    for argument in args {
        match argument.into_string() {
            Ok(text) => tokens.push(text),
            Err(_) => return Err(CliRefusal::new(CliRefusalCode::Encoding, "non-utf8")),
        }
    }

    let mut remaining = tokens.into_iter();
    // The operating system supplies the program name first; it is never interpreted.
    if remaining.next().is_none() {
        return Err(CliRefusal::new(CliRefusalCode::Missing, "subcommand"));
    }

    match remaining.next() {
        None => return Err(CliRefusal::new(CliRefusalCode::Missing, "subcommand")),
        Some(command) if command == SUBCOMMAND => {}
        Some(_) => return Err(CliRefusal::new(CliRefusalCode::Unknown, "subcommand")),
    }

    let mut entity_id: Option<String> = None;
    let mut base_url: Option<String> = None;

    while let Some(token) = remaining.next() {
        let slot = match token.as_str() {
            FLAG_ENTITY_ID => &mut entity_id,
            FLAG_BASE_URL => &mut base_url,
            flag if flag.starts_with("--") => {
                return Err(CliRefusal::new(CliRefusalCode::Unknown, "flag"));
            }
            _ => return Err(CliRefusal::new(CliRefusalCode::Unknown, "positional")),
        };
        if slot.is_some() {
            return Err(CliRefusal::new(CliRefusalCode::Duplicate, "option"));
        }
        let value = match remaining.next() {
            None => return Err(CliRefusal::new(CliRefusalCode::Missing, "value")),
            Some(value) if value.is_empty() || value.starts_with("--") => {
                return Err(CliRefusal::new(CliRefusalCode::Missing, "value"));
            }
            Some(value) => value,
        };
        *slot = Some(value);
    }

    let entity_id =
        entity_id.ok_or_else(|| CliRefusal::new(CliRefusalCode::Missing, FLAG_ENTITY_ID))?;
    let base_url =
        base_url.ok_or_else(|| CliRefusal::new(CliRefusalCode::Missing, FLAG_BASE_URL))?;

    Ok(BootstrapInputV1::new(entity_id, base_url))
}

/// Runs the adapter against the given inbound use case.
///
/// This is the only place that maps an application outcome onto CLI rendering; it
/// performs no I/O itself, so the composition root stays a thin shell.
#[must_use]
pub fn run<I>(args: I, preflight: &dyn BootstrapPreflightV1) -> CliOutcome
where
    I: IntoIterator<Item = OsString>,
{
    let input = match parse_request(args) {
        Ok(input) => input,
        Err(refusal) => {
            return CliOutcome::Refused {
                code: refusal.code().as_str(),
                detail: refusal.detail(),
            };
        }
    };

    match preflight.execute(input) {
        BootstrapOutcomeV1::Accepted(accepted) => CliOutcome::Accepted {
            line: format!(
                "preflight=ok entity_id={} base={}",
                accepted.entity_id().as_str(),
                accepted.base().as_str()
            ),
        },
        BootstrapOutcomeV1::Refused(refused) => CliOutcome::Refused {
            code: refused.code().as_str(),
            detail: refused.detail(),
        },
    }
}
