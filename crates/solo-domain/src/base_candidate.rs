//! Deliberately narrow base-candidate validation (`BaseCandidateV1`).
//!
//! This is an **engineering preflight subset** of the separately authored public
//! base URL contract, not the future full §6.2J profile. It accepts a small,
//! unambiguous ASCII HTTP(S) shape so that `R01.A` can prove real typed
//! validation end to end. Addresses outside the subset — IP literals, IDNA or
//! otherwise internationalized hosts, and any form needing the full profile — are
//! refused as **unsupported by this preflight**, which is not a finding that the
//! accepted Charter prohibits them.
//!
//! The value is never an Entity identity source, never inferred from a request,
//! and never forces HTTPS authorship: `http://` is accepted on its own terms.

use core::fmt;

/// Maximum accepted host length, in ASCII octets.
pub const MAX_HOST_OCTETS: usize = 253;
/// Maximum accepted length of one DNS label, in ASCII octets.
pub const MAX_LABEL_OCTETS: usize = 63;
/// Maximum accepted number of prefix segments after the origin.
pub const MAX_PREFIX_SEGMENTS: usize = 5;

/// Broad refusal class of a candidate base.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BaseCandidateErrorKind {
    /// The authored value is malformed or ambiguous.
    Invalid,
    /// The authored value is a legitimate shape that this preflight does not
    /// yet implement.
    Unsupported,
}

/// Precise refusal reason.
///
/// Each reason is a fixed ASCII token, so no authored input can reach a
/// diagnostic through it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BaseCandidateDetail {
    /// Nothing was authored.
    Empty,
    /// A control character or whitespace appeared.
    ControlOrWhitespace,
    /// The scheme was absent or was not HTTP(S).
    Scheme,
    /// The scheme was spelled in a non-lowercase form.
    SchemeCase,
    /// User-info credentials were present.
    Credentials,
    /// A query string was present.
    Query,
    /// A fragment was present.
    Fragment,
    /// A backslash appeared, which would be an ambiguous separator.
    Backslash,
    /// The host was absent.
    HostMissing,
    /// The host exceeded [`MAX_HOST_OCTETS`].
    HostTooLong,
    /// A DNS label was empty.
    LabelEmpty,
    /// A DNS label exceeded [`MAX_LABEL_OCTETS`].
    LabelTooLong,
    /// A DNS label began or ended with a hyphen.
    LabelEdge,
    /// A DNS label contained a character outside `[a-z0-9-]`.
    LabelCharacter,
    /// A port separator carried no digits.
    PortMissing,
    /// The port was not purely numeric.
    PortNotNumeric,
    /// Padding zeros made the port ambiguous.
    PortLeadingZero,
    /// The port was zero or above 65535.
    PortOutOfRange,
    /// Two separators produced an empty segment.
    SegmentEmpty,
    /// More than [`MAX_PREFIX_SEGMENTS`] segments were authored.
    SegmentTooMany,
    /// A segment contained a character outside `[a-z][a-z0-9-]*`.
    SegmentCharacter,
    /// A dot segment would traverse the prefix.
    SegmentTraversal,
    /// Percent-encoded input was authored.
    PercentEncoded,
    /// The host contained non-ASCII text.
    UnicodeHost,
    /// A DNS label claimed an `xn--` internationalized form.
    IdnaLabel,
    /// The host was an IP literal.
    IpLiteral,
}

impl BaseCandidateDetail {
    /// The refusal class this reason belongs to.
    #[must_use]
    pub const fn kind(self) -> BaseCandidateErrorKind {
        match self {
            Self::UnicodeHost | Self::IdnaLabel | Self::IpLiteral => {
                BaseCandidateErrorKind::Unsupported
            }
            Self::Empty
            | Self::ControlOrWhitespace
            | Self::Scheme
            | Self::SchemeCase
            | Self::Credentials
            | Self::Query
            | Self::Fragment
            | Self::Backslash
            | Self::HostMissing
            | Self::HostTooLong
            | Self::LabelEmpty
            | Self::LabelTooLong
            | Self::LabelEdge
            | Self::LabelCharacter
            | Self::PortMissing
            | Self::PortNotNumeric
            | Self::PortLeadingZero
            | Self::PortOutOfRange
            | Self::SegmentEmpty
            | Self::SegmentTooMany
            | Self::SegmentCharacter
            | Self::SegmentTraversal
            | Self::PercentEncoded => BaseCandidateErrorKind::Invalid,
        }
    }

    /// Stable, safe diagnostic token for this reason.
    #[must_use]
    pub const fn detail(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::ControlOrWhitespace => "control-or-whitespace",
            Self::Scheme => "scheme",
            Self::SchemeCase => "scheme-case",
            Self::Credentials => "credentials",
            Self::Query => "query",
            Self::Fragment => "fragment",
            Self::Backslash => "backslash",
            Self::HostMissing => "host-missing",
            Self::HostTooLong => "host-too-long",
            Self::LabelEmpty => "label-empty",
            Self::LabelTooLong => "label-too-long",
            Self::LabelEdge => "label-edge",
            Self::LabelCharacter => "label-character",
            Self::PortMissing => "port-missing",
            Self::PortNotNumeric => "port-not-numeric",
            Self::PortLeadingZero => "port-leading-zero",
            Self::PortOutOfRange => "port-out-of-range",
            Self::SegmentEmpty => "segment-empty",
            Self::SegmentTooMany => "segment-too-many",
            Self::SegmentCharacter => "segment-character",
            Self::SegmentTraversal => "segment-traversal",
            Self::PercentEncoded => "percent-encoded",
            Self::UnicodeHost => "unicode-host",
            Self::IdnaLabel => "idna-label",
            Self::IpLiteral => "ip-literal",
        }
    }
}

/// Refusal of a candidate base.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaseCandidateError {
    detail: BaseCandidateDetail,
}

impl BaseCandidateError {
    const fn new(detail: BaseCandidateDetail) -> Self {
        Self { detail }
    }

    /// The refusal class.
    #[must_use]
    pub const fn kind(&self) -> BaseCandidateErrorKind {
        self.detail.kind()
    }

    /// Stable engineering preflight code for this refusal.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self.detail.kind() {
            BaseCandidateErrorKind::Invalid => "PREFLIGHT_BASE_INVALID",
            BaseCandidateErrorKind::Unsupported => "PREFLIGHT_BASE_UNSUPPORTED",
        }
    }

    /// Stable, safe diagnostic token for the refusal reason.
    #[must_use]
    pub const fn detail(&self) -> &'static str {
        self.detail.detail()
    }
}

impl fmt::Display for BaseCandidateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code(), self.detail())
    }
}

impl std::error::Error for BaseCandidateError {}

const fn refuse(detail: BaseCandidateDetail) -> BaseCandidateError {
    BaseCandidateError::new(detail)
}

/// The two accepted schemes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BaseScheme {
    /// Cleartext HTTP.
    Http,
    /// TLS HTTP.
    Https,
}

impl BaseScheme {
    /// The canonical lowercase scheme spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Http => "http",
            Self::Https => "https",
        }
    }

    /// Whether `port` is this scheme's default and therefore omitted.
    #[must_use]
    pub const fn is_default_port(self, port: u16) -> bool {
        matches!((self, port), (Self::Http, 80) | (Self::Https, 443))
    }
}

/// A validated base candidate: canonical origin plus zero to five safe segments.
///
/// Construction is fallible and goes through [`BaseCandidateV1::parse`]; no public
/// constructor bypasses validation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BaseCandidateV1 {
    canonical: String,
    scheme: BaseScheme,
    host: String,
    port: Option<u16>,
    segments: Vec<String>,
}

impl BaseCandidateV1 {
    /// Validates and canonicalizes an authored base address.
    pub fn parse(candidate: &str) -> Result<Self, BaseCandidateError> {
        if candidate.is_empty() {
            return Err(refuse(BaseCandidateDetail::Empty));
        }
        for character in candidate.chars() {
            if character.is_control() || character.is_whitespace() {
                return Err(refuse(BaseCandidateDetail::ControlOrWhitespace));
            }
        }
        if candidate.contains('\\') {
            return Err(refuse(BaseCandidateDetail::Backslash));
        }
        if candidate.contains('?') {
            return Err(refuse(BaseCandidateDetail::Query));
        }
        if candidate.contains('#') {
            return Err(refuse(BaseCandidateDetail::Fragment));
        }
        if candidate.contains('@') {
            return Err(refuse(BaseCandidateDetail::Credentials));
        }

        let (scheme, remainder) = if let Some(rest) = candidate.strip_prefix("https://") {
            (BaseScheme::Https, rest)
        } else if let Some(rest) = candidate.strip_prefix("http://") {
            (BaseScheme::Http, rest)
        } else {
            let lowered = candidate.to_ascii_lowercase();
            if lowered.starts_with("http://") || lowered.starts_with("https://") {
                return Err(refuse(BaseCandidateDetail::SchemeCase));
            }
            return Err(refuse(BaseCandidateDetail::Scheme));
        };

        let (authority, path) = match remainder.find('/') {
            Some(index) => (&remainder[..index], &remainder[index..]),
            None => (remainder, ""),
        };
        if authority.is_empty() {
            return Err(refuse(BaseCandidateDetail::HostMissing));
        }

        let (host_raw, port_raw) = match authority.rfind(':') {
            Some(index) => {
                if authority[..index].contains(':') {
                    return Err(refuse(BaseCandidateDetail::IpLiteral));
                }
                (&authority[..index], Some(&authority[index + 1..]))
            }
            None => (authority, None),
        };
        if host_raw.is_empty() {
            return Err(refuse(BaseCandidateDetail::HostMissing));
        }
        if host_raw.starts_with('[') || host_raw.ends_with(']') {
            return Err(refuse(BaseCandidateDetail::IpLiteral));
        }
        if !host_raw.is_ascii() {
            return Err(refuse(BaseCandidateDetail::UnicodeHost));
        }

        let host = host_raw.to_ascii_lowercase();
        if host
            .bytes()
            .all(|byte| byte.is_ascii_digit() || byte == b'.')
        {
            return Err(refuse(BaseCandidateDetail::IpLiteral));
        }
        if host.len() > MAX_HOST_OCTETS {
            return Err(refuse(BaseCandidateDetail::HostTooLong));
        }
        for label in host.split('.') {
            if label.is_empty() {
                return Err(refuse(BaseCandidateDetail::LabelEmpty));
            }
            if label.len() > MAX_LABEL_OCTETS {
                return Err(refuse(BaseCandidateDetail::LabelTooLong));
            }
            if label.starts_with("xn--") {
                return Err(refuse(BaseCandidateDetail::IdnaLabel));
            }
            let bytes = label.as_bytes();
            let (first, last) = (bytes[0], bytes[bytes.len() - 1]);
            if !first.is_ascii_alphanumeric() || !last.is_ascii_alphanumeric() {
                return Err(refuse(BaseCandidateDetail::LabelEdge));
            }
            if !bytes
                .iter()
                .all(|byte| byte.is_ascii_alphanumeric() || *byte == b'-')
            {
                return Err(refuse(BaseCandidateDetail::LabelCharacter));
            }
        }

        let port = match port_raw {
            None => None,
            Some(raw) => {
                if raw.is_empty() {
                    return Err(refuse(BaseCandidateDetail::PortMissing));
                }
                if !raw.bytes().all(|byte| byte.is_ascii_digit()) {
                    return Err(refuse(BaseCandidateDetail::PortNotNumeric));
                }
                if raw.len() > 1 && raw.starts_with('0') {
                    return Err(refuse(BaseCandidateDetail::PortLeadingZero));
                }
                let value: u32 = raw
                    .parse()
                    .map_err(|_| refuse(BaseCandidateDetail::PortOutOfRange))?;
                if value == 0 || value > 65_535 {
                    return Err(refuse(BaseCandidateDetail::PortOutOfRange));
                }
                let value = u16::try_from(value)
                    .map_err(|_| refuse(BaseCandidateDetail::PortOutOfRange))?;
                Some(value)
            }
        }
        .filter(|port| !scheme.is_default_port(*port));

        let mut segments: Vec<String> = Vec::new();
        if !path.is_empty() && path != "/" {
            // A lone slash is the authored origin-only form and canonicalizes to
            // the bare origin; one terminal slash is removed from a longer prefix.
            let mut body = &path[1..];
            if let Some(stripped) = body.strip_suffix('/') {
                body = stripped;
            }
            if body.is_empty() {
                return Err(refuse(BaseCandidateDetail::SegmentEmpty));
            }
            for segment in body.split('/') {
                if segment.is_empty() {
                    return Err(refuse(BaseCandidateDetail::SegmentEmpty));
                }
                if segment == "." || segment == ".." {
                    return Err(refuse(BaseCandidateDetail::SegmentTraversal));
                }
                if segment.contains('%') {
                    return Err(refuse(BaseCandidateDetail::PercentEncoded));
                }
                if segments.len() == MAX_PREFIX_SEGMENTS {
                    return Err(refuse(BaseCandidateDetail::SegmentTooMany));
                }
                let bytes = segment.as_bytes();
                if !bytes[0].is_ascii_lowercase()
                    || !bytes.iter().all(|byte| {
                        byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'-'
                    })
                {
                    return Err(refuse(BaseCandidateDetail::SegmentCharacter));
                }
                segments.push(segment.to_owned());
            }
        }

        let mut canonical = String::new();
        canonical.push_str(scheme.as_str());
        canonical.push_str("://");
        canonical.push_str(&host);
        if let Some(port) = port {
            canonical.push(':');
            canonical.push_str(&port.to_string());
        }
        for segment in &segments {
            canonical.push('/');
            canonical.push_str(segment);
        }

        Ok(Self {
            canonical,
            scheme,
            host,
            port,
            segments,
        })
    }

    /// The canonical spelling of the validated base.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.canonical
    }

    /// The validated scheme.
    #[must_use]
    pub const fn scheme(&self) -> BaseScheme {
        self.scheme
    }

    /// The normalized lowercase host.
    #[must_use]
    pub fn host(&self) -> &str {
        &self.host
    }

    /// The effective nondefault port, when one was authored.
    #[must_use]
    pub const fn port(&self) -> Option<u16> {
        self.port
    }

    /// The validated prefix segments.
    #[must_use]
    pub fn segments(&self) -> &[String] {
        &self.segments
    }
}

impl fmt::Display for BaseCandidateV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.canonical)
    }
}
