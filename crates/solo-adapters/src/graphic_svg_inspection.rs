//! Strict, contained, static **true-vector** SVG sanitizer (`SOLO-WO-116 R01`).
//!
//! This module is the one safe-Rust SVG path. It does four separate things, and none of them
//! substitutes for another:
//!
//! 1. `parse_v1` reads the supplied bytes with the pinned `quick-xml 0.42.0` reader in strict mode
//!    and builds a **closed** element tree. Only elements and attributes named by the approved
//!    allowlist survive; every other tag, attribute, declaration, comment, CDATA section,
//!    processing instruction, entity reference or doctype refuses the **whole** document.
//! 2. Every retained attribute is re-parsed with the pinned `svgtypes 0.16.1` grammar and
//!    re-emitted from the parsed value, so output can never echo unchecked author text.
//! 3. `<text>` is converted into an original, fully code-defined 5x7 geometric glyph outline: no
//!    font file, no system font, no font request and no host-dependent `<text>` survives.
//! 4. `serialize_v1` writes one canonical document with LF endings, a fixed attribute order and a
//!    controlled decimal representation, so the same input produces byte-identical output on every
//!    host.
//!
//! Deliberate non-goals: nothing here rasterises, fetches, resolves an external resource, evaluates
//! CSS, inherits an ambient page colour, silently drops an unsupported node or claims publication
//! authority. `currentColor` resolves to the fixed self-contained value [`SVG_CURRENT_COLOR_V1`]
//! rather than to an unknown ambient colour.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};
use svgtypes::{
    Color, Length, LengthUnit, NumberListParser, PathParser, PathSegment, TransformListParser,
    TransformListToken, ViewBox,
};

use solo_application::{MediaWorkerDigestV1, sha256_media_worker_bytes_v1};

/// The canonical MIME of a confirmed SVG vector document.
pub const SVG_MIME_V1: &str = solo_application::IMAGE_MIME_SVG_V3;

/// Maximum accepted original SVG source, matching the approved 20 MiB media source ceiling.
pub const MAX_SVG_SOURCE_BYTES_V1: u64 = solo_application::MAX_SVG_SOURCE_BYTES_V3;

/// Maximum accepted canonical SVG output.
pub const MAX_SVG_CANONICAL_BYTES_V1: usize = solo_application::MAX_SVG_CANONICAL_BYTES_V3;

/// Maximum accepted number of XML elements in one document.
pub const MAX_SVG_ELEMENTS_V1: usize = solo_application::MAX_SVG_ELEMENTS_V3;

/// Maximum accepted element nesting depth.
pub const MAX_SVG_NESTING_V1: usize = solo_application::MAX_SVG_NESTING_V3;

/// Maximum accepted attributes on one element.
pub const MAX_SVG_ATTRS_V1: usize = solo_application::MAX_SVG_ATTRS_PER_ELEMENT_V3;

/// Maximum accepted `id`/`url(#id)`/`href="#id"` reference count.
pub const MAX_SVG_REFERENCES_V1: usize = solo_application::MAX_SVG_REFERENCES_V3;

/// Maximum accepted aggregate path-command count.
pub const MAX_SVG_PATH_COMMANDS_V1: usize = solo_application::MAX_SVG_PATH_COMMANDS_V3;

/// Maximum accepted number of `<filter>` elements.
pub const MAX_SVG_FILTER_ELEMENTS_V1: usize = solo_application::MAX_SVG_FILTER_ELEMENTS_V3;

/// Maximum accepted primitive operations inside one `<filter>`.
pub const MAX_SVG_FILTER_OPERATIONS_V1: usize = solo_application::MAX_SVG_FILTER_OPERATIONS_V3;

/// The fixed, self-contained colour `currentColor` resolves to.
pub const SVG_CURRENT_COLOR_V1: &str = "#000000";

/// The one namespace URI the root element may declare.
pub const SVG_NAMESPACE_V1: &str = "http://www.w3.org/2000/svg";

/// The largest accepted magnitude of any authored number, in user units.
const MAX_SVG_NUMBER_V1: f64 = 1.0e9;

/// The largest accepted `font-size` from which a glyph cell grid is derived.
const MAX_SVG_FONT_SIZE_V1: u32 = 1024;

/// The fixed column count of the approved 5x7 glyph repertoire.
const GLYPH_COLS: u32 = 5;

/// The fixed row count of the approved 5x7 glyph repertoire.
const GLYPH_ROWS: u32 = 7;

/// Every way this sanitizer can refuse. Every variant refuses the whole document.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SvgRefusalV1 {
    /// The source was empty.
    EmptySource,
    /// The source exceeded [`MAX_SVG_SOURCE_BYTES_V1`].
    OversizeSource,
    /// The source was not valid UTF-8.
    NotUtf8,
    /// The bytes were not one well-formed XML document.
    MalformedXml,
    /// A doctype, entity declaration, comment, CDATA section, declaration or PI was present.
    UnsupportedConstruct,
    /// An element outside the closed allowlist was present.
    UnsupportedElement,
    /// An attribute outside the closed allowlist was present.
    UnsupportedAttribute,
    /// One element carried more than [`MAX_SVG_ATTRS_V1`] attributes.
    AttributeCountOutOfRange,
    /// The document carried more than [`MAX_SVG_ELEMENTS_V1`] elements.
    ElementBudgetExceeded,
    /// Nesting exceeded [`MAX_SVG_NESTING_V1`].
    NestingTooDeep,
    /// An attribute value was not in its approved grammar or range.
    InvalidAttributeValue,
    /// The same `id` was authored twice.
    DuplicateId,
    /// A `url(#id)` or `href="#id"` target did not exist.
    ReferenceUnresolved,
    /// A gradient `href` chain formed a cycle.
    ReferenceCycle,
    /// More than [`MAX_SVG_REFERENCES_V1`] references were authored.
    ReferenceBudgetExceeded,
    /// More than [`MAX_SVG_PATH_COMMANDS_V1`] aggregate path commands were authored.
    PathBudgetExceeded,
    /// A filter budget was exceeded.
    FilterBudgetExceeded,
    /// The intrinsic geometry was absent, zero, non-finite or above the accepted ceiling.
    DimensionsOutOfRange,
    /// The pixel ceiling was exceeded.
    PixelCountOutOfRange,
    /// The canonical document exceeded [`MAX_SVG_CANONICAL_BYTES_V1`].
    CanonicalOutputTooLarge,
    /// A `<text>` character was outside the fixed glyph repertoire.
    TextCharacterUnsupported,
    /// The document had no root `<svg>` element.
    MissingRoot,
}

impl SvgRefusalV1 {
    /// The stable machine-readable token of this refusal.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::EmptySource => "svg-empty-source",
            Self::OversizeSource => "svg-oversize-source",
            Self::NotUtf8 => "svg-not-utf8",
            Self::MalformedXml => "svg-malformed-xml",
            Self::UnsupportedConstruct => "svg-unsupported-construct",
            Self::UnsupportedElement => "svg-unsupported-element",
            Self::UnsupportedAttribute => "svg-unsupported-attribute",
            Self::AttributeCountOutOfRange => "svg-attribute-count-out-of-range",
            Self::ElementBudgetExceeded => "svg-element-budget-exceeded",
            Self::NestingTooDeep => "svg-nesting-too-deep",
            Self::InvalidAttributeValue => "svg-invalid-attribute-value",
            Self::DuplicateId => "svg-duplicate-id",
            Self::ReferenceUnresolved => "svg-reference-unresolved",
            Self::ReferenceCycle => "svg-reference-cycle",
            Self::ReferenceBudgetExceeded => "svg-reference-budget-exceeded",
            Self::PathBudgetExceeded => "svg-path-budget-exceeded",
            Self::FilterBudgetExceeded => "svg-filter-budget-exceeded",
            Self::DimensionsOutOfRange => "svg-dimensions-out-of-range",
            Self::PixelCountOutOfRange => "svg-pixel-count-out-of-range",
            Self::CanonicalOutputTooLarge => "svg-canonical-output-too-large",
            Self::TextCharacterUnsupported => "svg-text-character-unsupported",
            Self::MissingRoot => "svg-missing-root",
        }
    }
}

/// The independently observable facts of one accepted SVG document.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SvgVectorFactsV1 {
    width: u32,
    height: u32,
    elements: usize,
    depth: usize,
    references: usize,
    path_commands: usize,
    filters: usize,
}

impl SvgVectorFactsV1 {
    /// The intrinsic `width` in user units, as an integer pixel count.
    #[must_use]
    pub const fn width(self) -> u32 {
        self.width
    }

    /// The intrinsic `height` in user units, as an integer pixel count.
    #[must_use]
    pub const fn height(self) -> u32 {
        self.height
    }

    /// The `width x height` product.
    #[must_use]
    pub const fn pixels(self) -> u64 {
        self.width as u64 * self.height as u64
    }

    /// The number of retained XML elements.
    #[must_use]
    pub const fn elements(self) -> usize {
        self.elements
    }

    /// The maximum observed element nesting depth.
    #[must_use]
    pub const fn depth(self) -> usize {
        self.depth
    }

    /// The number of validated internal references.
    #[must_use]
    pub const fn references(self) -> usize {
        self.references
    }

    /// The aggregate number of validated path commands.
    #[must_use]
    pub const fn path_commands(self) -> usize {
        self.path_commands
    }

    /// The number of `<filter>` elements.
    #[must_use]
    pub const fn filters(self) -> usize {
        self.filters
    }
}

/// One accepted, canonicalised SVG document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SvgSanitizedV1 {
    canonical: Vec<u8>,
    facts: SvgVectorFactsV1,
}

impl SvgSanitizedV1 {
    /// The canonical SVG text bytes. Always LF-ended, never a bitmap and never a raster.
    #[must_use]
    pub fn canonical(&self) -> &[u8] {
        &self.canonical
    }

    /// The canonical byte length.
    #[must_use]
    pub fn canonical_len(&self) -> u64 {
        self.canonical.len() as u64
    }

    /// The parent-recomputable SHA-256 of exactly the canonical bytes.
    #[must_use]
    pub fn canonical_digest(&self) -> MediaWorkerDigestV1 {
        sha256_media_worker_bytes_v1(&self.canonical)
    }

    /// The independently observable document facts.
    #[must_use]
    pub const fn facts(&self) -> SvgVectorFactsV1 {
        self.facts
    }
}

/// One retained element of the closed document tree.
///
/// `name` is the canonical allowlist spelling, never the authored bytes, so serialization can
/// never echo an unvetted tag name.
#[derive(Debug, Clone)]
struct NodeV1 {
    name: &'static str,
    attrs: BTreeMap<String, String>,
    text: String,
    children: Vec<NodeV1>,
}

/// The closed element allowlist. Matching is exact and case-sensitive, as XML requires.
const ALLOWED_ELEMENTS_V1: [&str; 26] = [
    "svg",
    "g",
    "defs",
    "title",
    "desc",
    "text",
    "path",
    "rect",
    "circle",
    "ellipse",
    "line",
    "polyline",
    "polygon",
    "linearGradient",
    "radialGradient",
    "stop",
    "clipPath",
    "mask",
    "filter",
    "feGaussianBlur",
    "feOffset",
    "feFlood",
    "feBlend",
    "feComposite",
    "feMerge",
    "feMergeNode",
];

/// Resolves one authored element name to its canonical allowlist spelling.
fn allowed_element_v1(authored: &str) -> Option<&'static str> {
    ALLOWED_ELEMENTS_V1
        .into_iter()
        .find(|name| *name == authored)
}

/// Formats one accepted number canonically.
///
/// The representation is fixed: exact integers print without a fraction, everything else prints at
/// six decimal places with trailing zeros trimmed, and negative zero always becomes `0`. Rust's
/// fixed-precision float formatting rounds identically on every supported host, and no value here
/// is derived from host state, locale or iteration order.
fn fmt_number_v1(value: f64) -> String {
    let rounded = if value == 0.0 { 0.0 } else { value };
    let mut text = format!("{rounded:.6}");
    if text.contains('.') {
        while text.ends_with('0') {
            text.pop();
        }
        if text.ends_with('.') {
            text.pop();
        }
    }
    if text == "-0" || text.is_empty() {
        text = "0".to_string();
    }
    text
}

/// Checks one authored number found inside a list or structure grammar.
fn bounded_v1(value: f64) -> Result<f64, SvgRefusalV1> {
    if !value.is_finite() || value.abs() > MAX_SVG_NUMBER_V1 {
        return Err(SvgRefusalV1::InvalidAttributeValue);
    }
    Ok(value)
}

/// Parses one authored number, requiring a finite magnitude inside the accepted bound.
fn number_v1(text: &str) -> Result<f64, SvgRefusalV1> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Err(SvgRefusalV1::InvalidAttributeValue);
    }
    let value: f64 = trimmed
        .parse()
        .map_err(|_| SvgRefusalV1::InvalidAttributeValue)?;
    bounded_v1(value)
}

/// Parses one authored length, accepting only unitless and `px` values.
fn length_v1(text: &str) -> Result<String, SvgRefusalV1> {
    let length: Length = text
        .trim()
        .parse()
        .map_err(|_| SvgRefusalV1::InvalidAttributeValue)?;
    let number = bounded_v1(length.number)?;
    match length.unit {
        LengthUnit::None => Ok(fmt_number_v1(number)),
        LengthUnit::Px => Ok(format!("{}px", fmt_number_v1(number))),
        _ => Err(SvgRefusalV1::InvalidAttributeValue),
    }
}

/// Parses one authored colour into its canonical `#rrggbb` or `#rrggbbaa` spelling.
fn color_v1(text: &str) -> Result<String, SvgRefusalV1> {
    let trimmed = text.trim();
    if trimmed == "currentColor" {
        return Ok(SVG_CURRENT_COLOR_V1.to_string());
    }
    let color: Color = trimmed
        .parse()
        .map_err(|_| SvgRefusalV1::InvalidAttributeValue)?;
    if color.alpha == 255 {
        Ok(format!(
            "#{:02x}{:02x}{:02x}",
            color.red, color.green, color.blue
        ))
    } else {
        Ok(format!(
            "#{:02x}{:02x}{:02x}{:02x}",
            color.red, color.green, color.blue, color.alpha
        ))
    }
}

/// Parses one authored opacity into its canonical bounded spelling.
fn opacity_v1(text: &str) -> Result<String, SvgRefusalV1> {
    let value = number_v1(text)?;
    if !(0.0..=1.0).contains(&value) {
        return Err(SvgRefusalV1::InvalidAttributeValue);
    }
    Ok(fmt_number_v1(value))
}

/// Parses one authored identifier.
fn ident_v1(text: &str) -> Result<String, SvgRefusalV1> {
    let trimmed = text.trim();
    let bytes = trimmed.as_bytes();
    if bytes.is_empty() || bytes.len() > 64 {
        return Err(SvgRefusalV1::InvalidAttributeValue);
    }
    if !bytes[0].is_ascii_alphabetic() {
        return Err(SvgRefusalV1::InvalidAttributeValue);
    }
    if !bytes
        .iter()
        .all(|byte| byte.is_ascii_alphanumeric() || *byte == b'_' || *byte == b'-')
    {
        return Err(SvgRefusalV1::InvalidAttributeValue);
    }
    Ok(trimmed.to_string())
}

/// Parses one authored inert `class` token list.
///
/// Class values are never evaluated as CSS: they are carried as tightly validated, non-executable
/// tokens only, and a token that is not a plain identifier refuses the whole document.
fn class_v1(text: &str) -> Result<String, SvgRefusalV1> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Err(SvgRefusalV1::InvalidAttributeValue);
    }
    let tokens: Vec<&str> = trimmed.split_ascii_whitespace().collect();
    if tokens.is_empty() || tokens.len() > 8 {
        return Err(SvgRefusalV1::InvalidAttributeValue);
    }
    let mut canonical = String::new();
    for (index, token) in tokens.iter().enumerate() {
        let checked = ident_v1(token)?;
        if index > 0 {
            canonical.push(' ');
        }
        canonical.push_str(&checked);
    }
    Ok(canonical)
}

/// Parses one authored internal fragment reference of the form `#id`.
fn fragment_v1(text: &str) -> Result<String, SvgRefusalV1> {
    let trimmed = text.trim();
    let ident = trimmed
        .strip_prefix('#')
        .ok_or(SvgRefusalV1::InvalidAttributeValue)?;
    let checked = ident_v1(ident)?;
    Ok(format!("#{checked}"))
}

/// Splits one authored `url(#id)` reference into its identifier.
fn func_iri_v1(text: &str) -> Result<String, SvgRefusalV1> {
    let trimmed = text.trim();
    let inner = trimmed
        .strip_prefix("url(")
        .and_then(|rest| rest.strip_suffix(')'))
        .ok_or(SvgRefusalV1::InvalidAttributeValue)?;
    let inner = inner.trim().trim_matches(|c| c == '"' || c == '\'');
    let ident = inner
        .strip_prefix('#')
        .ok_or(SvgRefusalV1::InvalidAttributeValue)?;
    ident_v1(ident)
}

/// Parses one authored value from a closed token set.
fn enum_v1(text: &str, allowed: &[&str]) -> Result<String, SvgRefusalV1> {
    let trimmed = text.trim();
    if allowed.contains(&trimmed) {
        Ok(trimmed.to_string())
    } else {
        Err(SvgRefusalV1::InvalidAttributeValue)
    }
}

/// Parses one authored paint: `none`, `currentColor`, a colour or an internal gradient reference.
fn paint_v1(text: &str) -> Result<String, SvgRefusalV1> {
    let trimmed = text.trim();
    if trimmed == "none" {
        return Ok("none".to_string());
    }
    if trimmed.starts_with("url(") {
        let ident = func_iri_v1(trimmed)?;
        return Ok(format!("url(#{ident})"));
    }
    color_v1(trimmed)
}

/// Parses one authored `none`-or-`url(#id)` functional reference.
fn optional_reference_v1(text: &str) -> Result<String, SvgRefusalV1> {
    let trimmed = text.trim();
    if trimmed == "none" {
        return Ok("none".to_string());
    }
    let ident = func_iri_v1(trimmed)?;
    Ok(format!("url(#{ident})"))
}

/// Re-serialises one authored path from its parsed segments and charges the command budget.
fn path_data_v1(text: &str, aggregate: &mut usize) -> Result<String, SvgRefusalV1> {
    let mut out = String::new();
    let mut count = 0usize;
    for segment in PathParser::from(text.trim()) {
        let segment = segment.map_err(|_| SvgRefusalV1::InvalidAttributeValue)?;
        count += 1;
        if count > MAX_SVG_PATH_COMMANDS_V1 {
            return Err(SvgRefusalV1::PathBudgetExceeded);
        }
        out.push(segment.command() as char);
        match segment {
            PathSegment::MoveTo { x, y, .. } | PathSegment::LineTo { x, y, .. } => {
                let _ = write!(
                    out,
                    " {} {}",
                    fmt_number_v1(bounded_v1(x)?),
                    fmt_number_v1(bounded_v1(y)?)
                );
            }
            PathSegment::HorizontalLineTo { x, .. } => {
                let _ = write!(out, " {}", fmt_number_v1(bounded_v1(x)?));
            }
            PathSegment::VerticalLineTo { y, .. } => {
                let _ = write!(out, " {}", fmt_number_v1(bounded_v1(y)?));
            }
            PathSegment::CurveTo {
                x1,
                y1,
                x2,
                y2,
                x,
                y,
                ..
            } => {
                let _ = write!(
                    out,
                    " {} {} {} {} {} {}",
                    fmt_number_v1(bounded_v1(x1)?),
                    fmt_number_v1(bounded_v1(y1)?),
                    fmt_number_v1(bounded_v1(x2)?),
                    fmt_number_v1(bounded_v1(y2)?),
                    fmt_number_v1(bounded_v1(x)?),
                    fmt_number_v1(bounded_v1(y)?)
                );
            }
            PathSegment::SmoothCurveTo { x2, y2, x, y, .. } => {
                let _ = write!(
                    out,
                    " {} {} {} {}",
                    fmt_number_v1(bounded_v1(x2)?),
                    fmt_number_v1(bounded_v1(y2)?),
                    fmt_number_v1(bounded_v1(x)?),
                    fmt_number_v1(bounded_v1(y)?)
                );
            }
            PathSegment::Quadratic { x1, y1, x, y, .. } => {
                let _ = write!(
                    out,
                    " {} {} {} {}",
                    fmt_number_v1(bounded_v1(x1)?),
                    fmt_number_v1(bounded_v1(y1)?),
                    fmt_number_v1(bounded_v1(x)?),
                    fmt_number_v1(bounded_v1(y)?)
                );
            }
            PathSegment::SmoothQuadratic { x, y, .. } => {
                let _ = write!(
                    out,
                    " {} {}",
                    fmt_number_v1(bounded_v1(x)?),
                    fmt_number_v1(bounded_v1(y)?)
                );
            }
            PathSegment::EllipticalArc {
                rx,
                ry,
                x_axis_rotation,
                large_arc,
                sweep,
                x,
                y,
                ..
            } => {
                let _ = write!(
                    out,
                    " {} {} {} {} {} {} {}",
                    fmt_number_v1(bounded_v1(rx.abs())?),
                    fmt_number_v1(bounded_v1(ry.abs())?),
                    fmt_number_v1(bounded_v1(x_axis_rotation)?),
                    u8::from(large_arc),
                    u8::from(sweep),
                    fmt_number_v1(bounded_v1(x)?),
                    fmt_number_v1(bounded_v1(y)?)
                );
            }
            PathSegment::ClosePath { .. } => {}
        }
        out.push(' ');
    }
    if count == 0 {
        return Err(SvgRefusalV1::InvalidAttributeValue);
    }
    *aggregate += count;
    if *aggregate > MAX_SVG_PATH_COMMANDS_V1 {
        return Err(SvgRefusalV1::PathBudgetExceeded);
    }
    out.pop();
    Ok(out)
}

/// Re-serialises one authored point list.
fn points_v1(text: &str) -> Result<String, SvgRefusalV1> {
    let mut values: Vec<f64> = Vec::new();
    for value in NumberListParser::from(text.trim()) {
        let value = value.map_err(|_| SvgRefusalV1::InvalidAttributeValue)?;
        values.push(bounded_v1(value)?);
        if values.len() > 2 * MAX_SVG_PATH_COMMANDS_V1 {
            return Err(SvgRefusalV1::PathBudgetExceeded);
        }
    }
    if values.len() < 4 || !values.len().is_multiple_of(2) {
        return Err(SvgRefusalV1::InvalidAttributeValue);
    }
    let mut out = String::new();
    for pair in values.chunks(2) {
        if !out.is_empty() {
            out.push(' ');
        }
        let _ = write!(out, "{},{}", fmt_number_v1(pair[0]), fmt_number_v1(pair[1]));
    }
    Ok(out)
}

/// Re-serialises one authored transform list.
fn transform_v1(text: &str) -> Result<String, SvgRefusalV1> {
    let mut out = String::new();
    let mut count = 0usize;
    for token in TransformListParser::from(text.trim()) {
        let token = token.map_err(|_| SvgRefusalV1::InvalidAttributeValue)?;
        count += 1;
        if count > 64 {
            return Err(SvgRefusalV1::InvalidAttributeValue);
        }
        if !out.is_empty() {
            out.push(' ');
        }
        match token {
            TransformListToken::Matrix { a, b, c, d, e, f } => {
                let _ = write!(
                    out,
                    "matrix({} {} {} {} {} {})",
                    fmt_number_v1(bounded_v1(a)?),
                    fmt_number_v1(bounded_v1(b)?),
                    fmt_number_v1(bounded_v1(c)?),
                    fmt_number_v1(bounded_v1(d)?),
                    fmt_number_v1(bounded_v1(e)?),
                    fmt_number_v1(bounded_v1(f)?)
                );
            }
            TransformListToken::Translate { tx, ty } => {
                let _ = write!(
                    out,
                    "translate({} {})",
                    fmt_number_v1(bounded_v1(tx)?),
                    fmt_number_v1(bounded_v1(ty)?)
                );
            }
            TransformListToken::Scale { sx, sy } => {
                let _ = write!(
                    out,
                    "scale({} {})",
                    fmt_number_v1(bounded_v1(sx)?),
                    fmt_number_v1(bounded_v1(sy)?)
                );
            }
            TransformListToken::Rotate { angle } => {
                let _ = write!(out, "rotate({})", fmt_number_v1(bounded_v1(angle)?));
            }
            TransformListToken::SkewX { angle } => {
                let _ = write!(out, "skewX({})", fmt_number_v1(bounded_v1(angle)?));
            }
            TransformListToken::SkewY { angle } => {
                let _ = write!(out, "skewY({})", fmt_number_v1(bounded_v1(angle)?));
            }
        }
    }
    if count == 0 {
        return Err(SvgRefusalV1::InvalidAttributeValue);
    }
    Ok(out)
}

/// Re-serialises one authored `viewBox`.
fn view_box_v1(text: &str) -> Result<String, SvgRefusalV1> {
    let view: ViewBox = text
        .trim()
        .parse()
        .map_err(|_| SvgRefusalV1::InvalidAttributeValue)?;
    let (x, y, w, h) = (
        bounded_v1(view.x)?,
        bounded_v1(view.y)?,
        bounded_v1(view.w)?,
        bounded_v1(view.h)?,
    );
    if w <= 0.0 || h <= 0.0 {
        return Err(SvgRefusalV1::InvalidAttributeValue);
    }
    Ok(format!(
        "{} {} {} {}",
        fmt_number_v1(x),
        fmt_number_v1(y),
        fmt_number_v1(w),
        fmt_number_v1(h)
    ))
}

/// Parses one authored gradient stop offset into a bounded canonical fraction.
fn offset_v1(text: &str) -> Result<String, SvgRefusalV1> {
    let trimmed = text.trim();
    let value = match trimmed.strip_suffix('%') {
        Some(number) => number_v1(number)? / 100.0,
        None => number_v1(trimmed)?,
    };
    if !(0.0..=1.0).contains(&value) {
        return Err(SvgRefusalV1::InvalidAttributeValue);
    }
    Ok(fmt_number_v1(value))
}

/// Parses one authored `stdDeviation` list of one or two bounded non-negative numbers.
fn std_deviation_v1(text: &str) -> Result<String, SvgRefusalV1> {
    let mut values: Vec<f64> = Vec::new();
    for value in NumberListParser::from(text.trim()) {
        let value = value.map_err(|_| SvgRefusalV1::InvalidAttributeValue)?;
        if !(0.0..=64.0).contains(&value) {
            return Err(SvgRefusalV1::InvalidAttributeValue);
        }
        values.push(value);
    }
    match values.len() {
        1 => Ok(fmt_number_v1(values[0])),
        2 => Ok(format!(
            "{} {}",
            fmt_number_v1(values[0]),
            fmt_number_v1(values[1])
        )),
        _ => Err(SvgRefusalV1::InvalidAttributeValue),
    }
}

/// Validates the shared paint/geometry presentation attribute set.
///
/// Returns `None` when the name is not a presentation attribute at all, so the caller can report an
/// element-specific refusal instead of a generic one.
fn presentation_v1(name: &str, raw: &str) -> Option<Result<String, SvgRefusalV1>> {
    let result = match name {
        "fill" | "stroke" => paint_v1(raw),
        "stroke-width" => number_v1(raw).and_then(|value| {
            if value < 0.0 {
                Err(SvgRefusalV1::InvalidAttributeValue)
            } else {
                Ok(fmt_number_v1(value))
            }
        }),
        "stroke-linecap" => enum_v1(raw, &["butt", "round", "square"]),
        "stroke-linejoin" => enum_v1(raw, &["miter", "round", "bevel"]),
        "fill-opacity" | "stroke-opacity" | "opacity" => opacity_v1(raw),
        "fill-rule" => enum_v1(raw, &["nonzero", "evenodd"]),
        "transform" => transform_v1(raw),
        "clip-path" | "mask" | "filter" => optional_reference_v1(raw),
        "class" => class_v1(raw),
        "id" => ident_v1(raw),
        _ => return None,
    };
    Some(result)
}

/// Validates one filter-primitive input selector.
fn filter_input_v1(raw: &str) -> Result<String, SvgRefusalV1> {
    let trimmed = raw.trim();
    if trimmed == "SourceGraphic" || trimmed == "SourceAlpha" {
        return Ok(trimmed.to_string());
    }
    ident_v1(trimmed)
}

/// Validates one authored attribute against the closed element/attribute table.
fn canonical_attribute_v1(
    element: &'static str,
    name: &str,
    raw: &str,
    path_commands: &mut usize,
) -> Result<(String, String), SvgRefusalV1> {
    let value = match (element, name) {
        ("svg", "xmlns") => enum_v1(raw, &[SVG_NAMESPACE_V1])?,
        ("svg", "width") | ("svg", "height") => length_v1(raw)?,
        ("svg", "viewBox") => view_box_v1(raw)?,
        ("text", "x") | ("text", "y") => length_v1(raw)?,
        ("text", "font-size") => {
            let value = number_v1(raw)?;
            if value < 7.0 || value > MAX_SVG_FONT_SIZE_V1 as f64 || value.fract() != 0.0 {
                return Err(SvgRefusalV1::InvalidAttributeValue);
            }
            fmt_number_v1(value)
        }
        ("path", "d") => path_data_v1(raw, path_commands)?,
        ("rect", "x")
        | ("rect", "y")
        | ("rect", "width")
        | ("rect", "height")
        | ("rect", "rx")
        | ("rect", "ry") => length_v1(raw)?,
        ("circle", "cx") | ("circle", "cy") | ("circle", "r") => length_v1(raw)?,
        ("ellipse", "cx") | ("ellipse", "cy") | ("ellipse", "rx") | ("ellipse", "ry") => {
            length_v1(raw)?
        }
        ("line", "x1") | ("line", "y1") | ("line", "x2") | ("line", "y2") => length_v1(raw)?,
        ("polyline", "points") | ("polygon", "points") => points_v1(raw)?,
        ("linearGradient", "x1")
        | ("linearGradient", "y1")
        | ("linearGradient", "x2")
        | ("linearGradient", "y2") => length_v1(raw)?,
        ("linearGradient", "gradientUnits") | ("radialGradient", "gradientUnits") => {
            enum_v1(raw, &["objectBoundingBox", "userSpaceOnUse"])?
        }
        ("linearGradient", "spreadMethod") | ("radialGradient", "spreadMethod") => {
            enum_v1(raw, &["pad", "reflect", "repeat"])?
        }
        ("linearGradient", "gradientTransform") | ("radialGradient", "gradientTransform") => {
            transform_v1(raw)?
        }
        ("linearGradient", "href") | ("radialGradient", "href") => fragment_v1(raw)?,
        ("radialGradient", "cx")
        | ("radialGradient", "cy")
        | ("radialGradient", "r")
        | ("radialGradient", "fx")
        | ("radialGradient", "fy") => length_v1(raw)?,
        ("stop", "offset") => offset_v1(raw)?,
        ("stop", "stop-color") | ("feFlood", "flood-color") => color_v1(raw)?,
        ("stop", "stop-opacity") | ("feFlood", "flood-opacity") => opacity_v1(raw)?,
        ("clipPath", "clipPathUnits") => enum_v1(raw, &["userSpaceOnUse", "objectBoundingBox"])?,
        ("mask", "maskUnits") => enum_v1(raw, &["userSpaceOnUse", "objectBoundingBox"])?,
        ("mask", "x") | ("mask", "y") | ("mask", "width") | ("mask", "height") => length_v1(raw)?,
        ("filter", "filterUnits") | ("filter", "primitiveUnits") => {
            enum_v1(raw, &["userSpaceOnUse", "objectBoundingBox"])?
        }
        ("filter", "x") | ("filter", "y") | ("filter", "width") | ("filter", "height") => {
            length_v1(raw)?
        }
        ("feGaussianBlur", "in")
        | ("feOffset", "in")
        | ("feBlend", "in")
        | ("feBlend", "in2")
        | ("feComposite", "in")
        | ("feComposite", "in2")
        | ("feMergeNode", "in") => filter_input_v1(raw)?,
        ("feGaussianBlur", "stdDeviation") => std_deviation_v1(raw)?,
        ("feOffset", "dx") | ("feOffset", "dy") => fmt_number_v1(number_v1(raw)?),
        ("feBlend", "mode") => {
            enum_v1(raw, &["normal", "multiply", "screen", "darken", "lighten"])?
        }
        ("feComposite", "operator") => {
            enum_v1(raw, &["over", "in", "out", "atop", "xor", "arithmetic"])?
        }
        ("feComposite", "k1")
        | ("feComposite", "k2")
        | ("feComposite", "k3")
        | ("feComposite", "k4") => fmt_number_v1(number_v1(raw)?),
        (
            "feGaussianBlur" | "feOffset" | "feFlood" | "feBlend" | "feComposite" | "feMerge",
            "result",
        ) => ident_v1(raw)?,
        (_, "id") => ident_v1(raw)?,
        _ => {
            if !matches!(
                element,
                "svg"
                    | "g"
                    | "text"
                    | "path"
                    | "rect"
                    | "circle"
                    | "ellipse"
                    | "line"
                    | "polyline"
                    | "polygon"
            ) {
                return Err(SvgRefusalV1::UnsupportedAttribute);
            }
            match presentation_v1(name, raw) {
                Some(checked) => checked?,
                None => return Err(SvgRefusalV1::UnsupportedAttribute),
            }
        }
    };
    Ok((name.to_string(), value))
}

/// Builds one retained node from a parsed start tag, applying the closed attribute table.
fn node_from_start_v1(
    start: &BytesStart<'_>,
    path_commands: &mut usize,
) -> Result<NodeV1, SvgRefusalV1> {
    let qualified = start.name();
    let authored: &str = qualified.as_ref();
    let canonical = allowed_element_v1(authored).ok_or(SvgRefusalV1::UnsupportedElement)?;
    let mut attrs: BTreeMap<String, String> = BTreeMap::new();
    let mut count = 0usize;
    for attribute in start.attributes() {
        let attribute = attribute.map_err(|_| SvgRefusalV1::MalformedXml)?;
        let key = attribute.key.as_ref();
        let value = attribute
            .normalized_value(quick_xml::XmlVersion::Implicit1_0)
            .map_err(|_| SvgRefusalV1::MalformedXml)?;
        count += 1;
        if count > MAX_SVG_ATTRS_V1 {
            return Err(SvgRefusalV1::AttributeCountOutOfRange);
        }
        let (name, checked) = canonical_attribute_v1(canonical, key, &value, path_commands)?;
        if attrs.insert(name, checked).is_some() {
            return Err(SvgRefusalV1::MalformedXml);
        }
    }
    Ok(NodeV1 {
        name: canonical,
        attrs,
        text: String::new(),
        children: Vec::new(),
    })
}

/// Attaches one finished node to its parent, or records it as the single root.
fn attach_v1(stack: &mut [NodeV1], root: &mut Option<NodeV1>, node: NodeV1) {
    match stack.len() {
        0 => {
            if root.is_none() {
                *root = Some(node);
            }
        }
        _ => {
            if let Some(parent) = stack.last_mut() {
                parent.children.push(node);
            }
        }
    }
}

/// Parses the supplied UTF-8 XML text into the closed tree.
fn parse_v1(text: &str, path_commands: &mut usize) -> Result<NodeV1, SvgRefusalV1> {
    let mut reader = Reader::from_str(text);
    reader.config_mut().allow_dangling_amp = false;

    let mut stack: Vec<NodeV1> = Vec::new();
    let mut root: Option<NodeV1> = None;
    let mut elements = 0usize;
    loop {
        let event = reader
            .read_event()
            .map_err(|_| SvgRefusalV1::MalformedXml)?;
        match event {
            Event::Start(start) => {
                elements += 1;
                if elements > MAX_SVG_ELEMENTS_V1 {
                    return Err(SvgRefusalV1::ElementBudgetExceeded);
                }
                if stack.len() + 1 > MAX_SVG_NESTING_V1 {
                    return Err(SvgRefusalV1::NestingTooDeep);
                }
                let node = node_from_start_v1(&start, path_commands)?;
                stack.push(node);
            }
            Event::Empty(start) => {
                elements += 1;
                if elements > MAX_SVG_ELEMENTS_V1 {
                    return Err(SvgRefusalV1::ElementBudgetExceeded);
                }
                if stack.len() + 1 > MAX_SVG_NESTING_V1 {
                    return Err(SvgRefusalV1::NestingTooDeep);
                }
                let node = node_from_start_v1(&start, path_commands)?;
                attach_v1(&mut stack, &mut root, node);
            }
            Event::End(_) => {
                let Some(node) = stack.pop() else {
                    return Err(SvgRefusalV1::MalformedXml);
                };
                attach_v1(&mut stack, &mut root, node);
            }
            Event::Text(content) => {
                let decoded = content.xml10_content();
                let text = decoded.as_ref();
                match stack.last_mut() {
                    Some(parent)
                        if parent.name == "title"
                            || parent.name == "desc"
                            || parent.name == "text" =>
                    {
                        parent.text.push_str(text);
                    }
                    _ => {
                        if !text.trim().is_empty() {
                            return Err(SvgRefusalV1::UnsupportedConstruct);
                        }
                    }
                }
            }
            Event::Eof => break,
            Event::Decl(_)
            | Event::PI(_)
            | Event::DocType(_)
            | Event::Comment(_)
            | Event::CData(_)
            | Event::GeneralRef(_) => return Err(SvgRefusalV1::UnsupportedConstruct),
        }
    }

    if !stack.is_empty() {
        return Err(SvgRefusalV1::MalformedXml);
    }
    let root = root.ok_or(SvgRefusalV1::MissingRoot)?;
    if root.name != "svg" {
        return Err(SvgRefusalV1::MissingRoot);
    }
    Ok(root)
}

/// One internal reference edge, used for both resolution and cycle detection.
struct ReferenceGraphV1 {
    ids: BTreeSet<String>,
    href_edges: Vec<(String, String)>,
    references: usize,
}

impl ReferenceGraphV1 {
    fn new() -> Self {
        Self {
            ids: BTreeSet::new(),
            href_edges: Vec::new(),
            references: 0,
        }
    }
}

/// Collects every retained identifier and reference, charging the reference budget.
fn collect_references_v1(node: &NodeV1, graph: &mut ReferenceGraphV1) -> Result<(), SvgRefusalV1> {
    if let Some(id) = node.attrs.get("id")
        && !graph.ids.insert(id.clone())
    {
        return Err(SvgRefusalV1::DuplicateId);
    }
    for name in ["fill", "stroke", "clip-path", "mask", "filter", "href"] {
        if let Some(value) = node.attrs.get(name)
            && let Some(ident) = (if name == "href" {
                value.strip_prefix('#')
            } else {
                value
                    .strip_prefix("url(#")
                    .and_then(|rest| rest.strip_suffix(')'))
            })
        {
            graph.references += 1;
            if graph.references > MAX_SVG_REFERENCES_V1 {
                return Err(SvgRefusalV1::ReferenceBudgetExceeded);
            }
            if name == "href"
                && let Some(owner) = node.attrs.get("id")
            {
                graph.href_edges.push((owner.clone(), ident.to_string()));
            }
            if !graph.ids.contains(ident) {
                // The target may be authored later in document order, so resolution is deferred.
                graph.href_edges.push((String::new(), ident.to_string()));
            }
        }
    }
    for child in &node.children {
        collect_references_v1(child, graph)?;
    }
    Ok(())
}

/// Proves every reference resolves and no `href` chain cycles.
fn check_references_v1(graph: &ReferenceGraphV1) -> Result<(), SvgRefusalV1> {
    for (owner, target) in &graph.href_edges {
        if !graph.ids.contains(target) {
            return Err(SvgRefusalV1::ReferenceUnresolved);
        }
        if !owner.is_empty() {
            let mut seen: BTreeSet<&str> = BTreeSet::new();
            let mut current = target.as_str();
            while let Some(next) = graph
                .href_edges
                .iter()
                .find(|(from, _)| from == current)
                .map(|(_, to)| to.as_str())
            {
                if !seen.insert(current) {
                    return Err(SvgRefusalV1::ReferenceCycle);
                }
                if next == owner {
                    return Err(SvgRefusalV1::ReferenceCycle);
                }
                current = next;
                if seen.len() > MAX_SVG_REFERENCES_V1 {
                    return Err(SvgRefusalV1::ReferenceCycle);
                }
            }
        }
    }
    Ok(())
}

/// The structural statistics of one retained tree.
#[derive(Default)]
struct TreeStatsV1 {
    elements: usize,
    depth: usize,
    filters: usize,
}

/// Walks the retained tree once, counting elements, depth and filters.
fn measure_v1(node: &NodeV1, depth: usize, stats: &mut TreeStatsV1) -> Result<(), SvgRefusalV1> {
    stats.elements += 1;
    stats.depth = stats.depth.max(depth);
    if node.name == "filter" {
        stats.filters += 1;
        if stats.filters > MAX_SVG_FILTER_ELEMENTS_V1 {
            return Err(SvgRefusalV1::FilterBudgetExceeded);
        }
        let operations = node
            .children
            .iter()
            .filter(|child| child.name.starts_with("fe"))
            .count();
        if operations > MAX_SVG_FILTER_OPERATIONS_V1 {
            return Err(SvgRefusalV1::FilterBudgetExceeded);
        }
    }
    for child in &node.children {
        measure_v1(child, depth + 1, stats)?;
    }
    Ok(())
}

/// Parses one canonical length attribute into an intrinsic side.
fn side_v1(value: &str) -> Option<u32> {
    let number = value.strip_suffix("px").unwrap_or(value);
    let parsed: f64 = number.parse().ok()?;
    if !parsed.is_finite() || parsed < 1.0 || parsed > 8192.0 || parsed.fract() != 0.0 {
        return None;
    }
    Some(parsed as u32)
}

/// Resolves the intrinsic geometry from the retained root element.
fn dimensions_v1(root: &NodeV1) -> Result<(u32, u32), SvgRefusalV1> {
    let width = root
        .attrs
        .get("width")
        .and_then(|value| side_v1(value))
        .ok_or(SvgRefusalV1::DimensionsOutOfRange)?;
    let height = root
        .attrs
        .get("height")
        .and_then(|value| side_v1(value))
        .ok_or(SvgRefusalV1::DimensionsOutOfRange)?;
    let pixels = u64::from(width) * u64::from(height);
    if pixels == 0 || pixels > 32_000_000 {
        return Err(SvgRefusalV1::PixelCountOutOfRange);
    }
    Ok((width, height))
}

/// Escapes one retained attribute value for the canonical writer.
fn escape_attribute_v1(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(character),
        }
    }
    out
}

/// Escapes one retained text node for the canonical writer.
fn escape_text_v1(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            _ => out.push(character),
        }
    }
    out
}

/// Writes one indentation level.
fn indent_v1(out: &mut String, depth: usize) {
    for _ in 0..depth {
        out.push_str("  ");
    }
}

/// Serialises one retained node, converting `<text>` into vector outlines.
fn serialize_node_v1(
    node: &NodeV1,
    depth: usize,
    out: &mut String,
    glyph_commands: &mut usize,
) -> Result<(), SvgRefusalV1> {
    if node.name == "text" {
        return serialize_text_v1(node, depth, out, glyph_commands);
    }
    indent_v1(out, depth);
    out.push('<');
    out.push_str(node.name);
    for (name, value) in &node.attrs {
        let _ = write!(out, " {}=\"{}\"", name, escape_attribute_v1(value));
    }
    let inline_text = matches!(node.name, "title" | "desc") && !node.text.is_empty();
    if node.children.is_empty() && !inline_text {
        out.push_str(" />\n");
        return Ok(());
    }
    out.push('>');
    if inline_text {
        out.push_str(&escape_text_v1(&node.text));
        let _ = writeln!(out, "</{}>", node.name);
        return Ok(());
    }
    out.push('\n');
    for child in &node.children {
        serialize_node_v1(child, depth + 1, out, glyph_commands)?;
    }
    indent_v1(out, depth);
    let _ = writeln!(out, "</{}>", node.name);
    Ok(())
}

/// Serialises one `<text>` element as a translated group of geometric glyph outlines.
fn serialize_text_v1(
    node: &NodeV1,
    depth: usize,
    out: &mut String,
    glyph_commands: &mut usize,
) -> Result<(), SvgRefusalV1> {
    let x = node
        .attrs
        .get("x")
        .map_or_else(|| "0".to_string(), Clone::clone);
    let y = node
        .attrs
        .get("y")
        .map_or_else(|| "0".to_string(), Clone::clone);
    let font_size = node
        .attrs
        .get("font-size")
        .and_then(|value| value.parse::<u32>().ok())
        .ok_or(SvgRefusalV1::InvalidAttributeValue)?;
    let fill = node
        .attrs
        .get("fill")
        .map_or_else(|| SVG_CURRENT_COLOR_V1.to_string(), Clone::clone);
    let cell = font_size.div_ceil(GLYPH_ROWS);
    let path = glyph_path_v1(&node.text, cell, glyph_commands)?;

    indent_v1(out, depth);
    let _ = writeln!(out, "<g transform=\"translate({x} {y})\">");
    if !node.text.is_empty() {
        indent_v1(out, depth + 1);
        let _ = writeln!(out, "<title>{}</title>", escape_text_v1(&node.text));
    }
    if !path.is_empty() {
        indent_v1(out, depth + 1);
        let _ = writeln!(out, "<path d=\"{}\" fill=\"{}\" />", path, fill);
    }
    indent_v1(out, depth);
    out.push_str("</g>\n");
    Ok(())
}

/// Serialises the retained tree into canonical bytes.
fn serialize_v1(root: &NodeV1, glyph_commands: &mut usize) -> Result<String, SvgRefusalV1> {
    let mut out = String::new();
    serialize_node_v1(root, 0, &mut out, glyph_commands)?;
    if out.len() > MAX_SVG_CANONICAL_BYTES_V1 {
        return Err(SvgRefusalV1::CanonicalOutputTooLarge);
    }
    Ok(out)
}

/// The approved original 5x7 geometric glyph repertoire.
///
/// Each glyph is seven rows of five cells; bit `0b10000` is the leftmost column. The table is
/// literal data of this module: it is not a font, not a downloaded asset and not platform-library
/// artwork, and no glyph is loaded at run time. A character outside the table refuses the whole
/// document rather than silently disappearing.
const fn glyph_rows_v1(character: char) -> Option<[u8; 7]> {
    let rows = match character {
        ' ' => [
            0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000,
        ],
        'A' => [
            0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],
        'B' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110,
        ],
        'C' => [
            0b01111, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b01111,
        ],
        'D' => [
            0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110,
        ],
        'E' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111,
        ],
        'F' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000,
        ],
        'G' => [
            0b01111, 0b10000, 0b10000, 0b10111, 0b10001, 0b10001, 0b01110,
        ],
        'H' => [
            0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],
        'I' => [
            0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b11111,
        ],
        'J' => [
            0b00111, 0b00010, 0b00010, 0b00010, 0b00010, 0b10010, 0b01100,
        ],
        'K' => [
            0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001,
        ],
        'L' => [
            0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111,
        ],
        'M' => [
            0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001,
        ],
        'N' => [
            0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b10001,
        ],
        'O' => [
            0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        'P' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000,
        ],
        'Q' => [
            0b01110, 0b10001, 0b10001, 0b10001, 0b10101, 0b10010, 0b01101,
        ],
        'R' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001,
        ],
        'S' => [
            0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110,
        ],
        'T' => [
            0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100,
        ],
        'U' => [
            0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        'V' => [
            0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100,
        ],
        'W' => [
            0b10001, 0b10001, 0b10001, 0b10101, 0b10101, 0b11011, 0b10001,
        ],
        'X' => [
            0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b10001,
        ],
        'Y' => [
            0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00100,
        ],
        'Z' => [
            0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b11111,
        ],
        'a' => [
            0b00000, 0b00000, 0b01110, 0b00001, 0b01111, 0b10001, 0b01111,
        ],
        'b' => [
            0b10000, 0b10000, 0b11110, 0b10001, 0b10001, 0b10001, 0b11110,
        ],
        'c' => [
            0b00000, 0b00000, 0b01111, 0b10000, 0b10000, 0b10000, 0b01111,
        ],
        'd' => [
            0b00001, 0b00001, 0b01111, 0b10001, 0b10001, 0b10001, 0b01111,
        ],
        'e' => [
            0b00000, 0b00000, 0b01110, 0b10001, 0b11111, 0b10000, 0b01110,
        ],
        'f' => [
            0b00110, 0b01001, 0b01000, 0b11100, 0b01000, 0b01000, 0b01000,
        ],
        'g' => [
            0b00000, 0b01111, 0b10001, 0b10001, 0b01111, 0b00001, 0b01110,
        ],
        'h' => [
            0b10000, 0b10000, 0b11110, 0b10001, 0b10001, 0b10001, 0b10001,
        ],
        'i' => [
            0b00100, 0b00000, 0b01100, 0b00100, 0b00100, 0b00100, 0b01110,
        ],
        'j' => [
            0b00010, 0b00000, 0b00110, 0b00010, 0b00010, 0b10010, 0b01100,
        ],
        'k' => [
            0b10000, 0b10000, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010,
        ],
        'l' => [
            0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110,
        ],
        'm' => [
            0b00000, 0b00000, 0b11010, 0b10101, 0b10101, 0b10101, 0b10101,
        ],
        'n' => [
            0b00000, 0b00000, 0b11110, 0b10001, 0b10001, 0b10001, 0b10001,
        ],
        'o' => [
            0b00000, 0b00000, 0b01110, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        'p' => [
            0b00000, 0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000,
        ],
        'q' => [
            0b00000, 0b01111, 0b10001, 0b10001, 0b01111, 0b00001, 0b00001,
        ],
        'r' => [
            0b00000, 0b00000, 0b10110, 0b11001, 0b10000, 0b10000, 0b10000,
        ],
        's' => [
            0b00000, 0b00000, 0b01111, 0b10000, 0b01110, 0b00001, 0b11110,
        ],
        't' => [
            0b01000, 0b01000, 0b11100, 0b01000, 0b01000, 0b01001, 0b00110,
        ],
        'u' => [
            0b00000, 0b00000, 0b10001, 0b10001, 0b10001, 0b10011, 0b01101,
        ],
        'v' => [
            0b00000, 0b00000, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100,
        ],
        'w' => [
            0b00000, 0b00000, 0b10001, 0b10101, 0b10101, 0b10101, 0b01010,
        ],
        'x' => [
            0b00000, 0b00000, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001,
        ],
        'y' => [
            0b00000, 0b10001, 0b10001, 0b10001, 0b01111, 0b00001, 0b01110,
        ],
        'z' => [
            0b00000, 0b00000, 0b11111, 0b00010, 0b00100, 0b01000, 0b11111,
        ],

        '0' => [
            0b01110, 0b10011, 0b10101, 0b10101, 0b10101, 0b11001, 0b01110,
        ],
        '1' => [
            0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110,
        ],
        '2' => [
            0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b01000, 0b11111,
        ],
        '3' => [
            0b11110, 0b00001, 0b00001, 0b01110, 0b00001, 0b00001, 0b11110,
        ],
        '4' => [
            0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010,
        ],
        '5' => [
            0b11111, 0b10000, 0b11110, 0b00001, 0b00001, 0b10001, 0b01110,
        ],
        '6' => [
            0b00110, 0b01000, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110,
        ],
        '7' => [
            0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000,
        ],
        '8' => [
            0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110,
        ],
        '9' => [
            0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00010, 0b01100,
        ],
        '.' => [
            0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00110, 0b00110,
        ],
        ',' => [
            0b00000, 0b00000, 0b00000, 0b00000, 0b00110, 0b00110, 0b01100,
        ],
        ':' => [
            0b00000, 0b00110, 0b00110, 0b00000, 0b00110, 0b00110, 0b00000,
        ],
        ';' => [
            0b00000, 0b00110, 0b00110, 0b00000, 0b00110, 0b00110, 0b01100,
        ],
        '!' => [
            0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00000, 0b00100,
        ],
        '?' => [
            0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b00000, 0b00100,
        ],
        '\'' => [
            0b00100, 0b00100, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000,
        ],
        '"' => [
            0b01010, 0b01010, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000,
        ],
        '(' => [
            0b00010, 0b00100, 0b01000, 0b01000, 0b01000, 0b00100, 0b00010,
        ],
        ')' => [
            0b01000, 0b00100, 0b00010, 0b00010, 0b00010, 0b00100, 0b01000,
        ],
        '-' => [
            0b00000, 0b00000, 0b00000, 0b11111, 0b00000, 0b00000, 0b00000,
        ],
        '+' => [
            0b00000, 0b00100, 0b00100, 0b11111, 0b00100, 0b00100, 0b00000,
        ],
        '/' => [
            0b00001, 0b00010, 0b00010, 0b00100, 0b01000, 0b01000, 0b10000,
        ],
        '#' => [
            0b01010, 0b01010, 0b11111, 0b01010, 0b11111, 0b01010, 0b01010,
        ],
        '&' => [
            0b01100, 0b10010, 0b10100, 0b01000, 0b10101, 0b10010, 0b01101,
        ],
        '*' => [
            0b00000, 0b10101, 0b01110, 0b11111, 0b01110, 0b10101, 0b00000,
        ],
        '=' => [
            0b00000, 0b00000, 0b11111, 0b00000, 0b11111, 0b00000, 0b00000,
        ],
        '_' => [
            0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b11111,
        ],
        '@' => [
            0b01110, 0b10001, 0b10111, 0b10101, 0b10111, 0b10000, 0b01111,
        ],
        '%' => [
            0b11001, 0b11010, 0b00010, 0b00100, 0b01000, 0b01011, 0b10011,
        ],
        _ => return None,
    };
    Some(rows)
}

/// Emits the geometric outline of one text run as SVG path commands.
///
/// Every filled cell becomes an axis-aligned sub-path, and horizontally adjacent filled cells are
/// merged into one run so the command count stays small. All arithmetic is integer cell arithmetic:
/// no coordinate is derived from host state, and no glyph depends on a font.
fn glyph_path_v1(text: &str, cell: u32, commands: &mut usize) -> Result<String, SvgRefusalV1> {
    let mut out = String::new();
    let mut cursor: u32 = 0;
    for character in text.chars() {
        let rows = glyph_rows_v1(character).ok_or(SvgRefusalV1::TextCharacterUnsupported)?;
        for (row, bits) in rows.iter().enumerate() {
            let mut column = 0u32;
            while column < GLYPH_COLS {
                let filled = bits & (0b10000 >> column) != 0;
                if !filled {
                    column += 1;
                    continue;
                }
                let start = column;
                while column < GLYPH_COLS && bits & (0b10000 >> column) != 0 {
                    column += 1;
                }
                let run = column - start;
                let x = cursor + start * cell;
                let y = row as u32 * cell;
                let _ = write!(
                    out,
                    "M {x} {y} h {} v {} h -{} z",
                    run * cell,
                    cell,
                    run * cell
                );
                // The canonical writer separates sub-paths with one space after `z`, so the
                // emitted attribute must already carry that exact separator.
                out.push(' ');
                *commands += 5;
                if *commands > MAX_SVG_PATH_COMMANDS_V1 {
                    return Err(SvgRefusalV1::PathBudgetExceeded);
                }
            }
        }
        cursor += (GLYPH_COLS + 1) * cell;
    }
    Ok(out.trim_end().to_string())
}

/// Sanitises one supplied SVG document into canonical vector bytes, or refuses it whole.
///
/// The order is deliberate: byte ceilings first, then strict parsing, then intrinsic geometry, then
/// reference resolution, then structural measurement, then canonical serialization. No stage
/// returns a partial document, and no unsupported construct is ever silently dropped.
///
/// # Errors
///
/// A closed [`SvgRefusalV1`] naming the single reason the whole document was refused.
pub fn sanitize_svg_v1(source: &[u8]) -> Result<SvgSanitizedV1, SvgRefusalV1> {
    if source.is_empty() {
        return Err(SvgRefusalV1::EmptySource);
    }
    if source.len() as u64 > MAX_SVG_SOURCE_BYTES_V1 {
        return Err(SvgRefusalV1::OversizeSource);
    }
    let text = std::str::from_utf8(source).map_err(|_| SvgRefusalV1::NotUtf8)?;

    let mut path_commands = 0usize;
    let root = parse_v1(text, &mut path_commands)?;
    let (width, height) = dimensions_v1(&root)?;

    let mut graph = ReferenceGraphV1::new();
    collect_references_v1(&root, &mut graph)?;
    check_references_v1(&graph)?;

    let mut stats = TreeStatsV1::default();
    measure_v1(&root, 1, &mut stats)?;

    let mut glyph_commands = 0usize;
    let canonical = serialize_v1(&root, &mut glyph_commands)?;

    let total_path_commands = path_commands.saturating_add(glyph_commands);
    if total_path_commands > MAX_SVG_PATH_COMMANDS_V1 {
        return Err(SvgRefusalV1::PathBudgetExceeded);
    }

    // The measured facts must describe exactly the bytes this function returns. `<text>` expands
    // into a `<g>`, a `<title>` and a `<path>` during serialisation, so the source-tree measurement
    // is not the canonical tree's measurement. Proved by the `n116_` host cases, which observed a
    // `FrameMalformed` refusal for a document whose only element was `<text>`. Re-measuring the
    // bounded, already-canonical output makes the claim and the bytes agree by construction.
    let canonical_bytes = canonical.into_bytes();
    let canonical_text =
        std::str::from_utf8(&canonical_bytes).map_err(|_| SvgRefusalV1::NotUtf8)?;
    let mut reread_commands = 0usize;
    let reread = parse_v1(canonical_text, &mut reread_commands)?;
    let mut stats = TreeStatsV1::default();
    measure_v1(&reread, 1, &mut stats)?;
    if reread_commands > MAX_SVG_PATH_COMMANDS_V1 {
        return Err(SvgRefusalV1::PathBudgetExceeded);
    }
    let facts = SvgVectorFactsV1 {
        width,
        height,
        elements: stats.elements,
        depth: stats.depth,
        references: graph.references,
        path_commands: total_path_commands,
        filters: stats.filters,
    };
    Ok(SvgSanitizedV1 {
        canonical: canonical_bytes,
        facts,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn document(body: &str) -> String {
        format!(
            "<svg xmlns=\"{SVG_NAMESPACE_V1}\" width=\"24\" height=\"24\" viewBox=\"0 0 24 24\">{body}</svg>"
        )
    }

    fn accepted(body: &str) -> SvgSanitizedV1 {
        sanitize_svg_v1(document(body).as_bytes())
            .unwrap_or_else(|refusal| panic!("refused: {}", refusal.token()))
    }

    fn canonical(body: &str) -> String {
        String::from_utf8(accepted(body).canonical().to_vec()).expect("canonical output is UTF-8")
    }

    #[test]
    fn svg_01_a_minimal_document_is_accepted_and_canonical() {
        let sanitized = accepted("<path d=\"M0 0h24v24H0z\" fill=\"none\" />");
        let text = String::from_utf8(sanitized.canonical().to_vec()).expect("utf8");
        assert!(text.starts_with("<svg "), "canonical root: {text}");
        assert!(text.ends_with('\n'), "canonical output is LF terminated");
        assert!(!text.contains('\r'), "canonical output never carries CR");
        assert!(text.contains("viewBox=\"0 0 24 24\""));
        assert_eq!(sanitized.facts().width(), 24);
        assert_eq!(sanitized.facts().height(), 24);
        assert_eq!(sanitized.facts().depth(), 2);
    }

    #[test]
    fn svg_02_every_mandated_path_command_and_shape_survives() {
        let body = concat!(
            "<path d=\"M1 1L2 2H3V4C5 5 6 6 7 7S8 8 9 9Q10 10 11 11T12 12A3 3 0 0 1 13 13Z\" />",
            "<rect x=\"1\" y=\"1\" width=\"2\" height=\"3\" rx=\"1\" />",
            "<circle cx=\"4\" cy=\"4\" r=\"2\" />",
            "<ellipse cx=\"4\" cy=\"4\" rx=\"2\" ry=\"3\" />",
            "<line x1=\"0\" y1=\"0\" x2=\"4\" y2=\"4\" />",
            "<polyline points=\"0,0 1,1 2,0\" />",
            "<polygon points=\"0,0 2,0 1,2\" />"
        );
        let text = canonical(body);
        for tag in [
            "<path",
            "<rect",
            "<circle",
            "<ellipse",
            "<line",
            "<polyline",
            "<polygon",
        ] {
            assert!(text.contains(tag), "missing {tag} in {text}");
        }
        assert!(accepted(body).facts().path_commands() >= 10);
    }

    #[test]
    fn svg_03_groups_transforms_gradients_clip_mask_and_filters_survive() {
        let body = concat!(
            "<defs><linearGradient id=\"lg\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"1\">",
            "<stop offset=\"0\" stop-color=\"#ff0000\" /><stop offset=\"100%\" stop-color=\"#0000ff\" />",
            "</linearGradient><radialGradient id=\"rg\" cx=\"0.5\" cy=\"0.5\" r=\"0.5\">",
            "<stop offset=\"0.5\" stop-color=\"#00ff00\" /></radialGradient>",
            "<clipPath id=\"cp\"><rect x=\"0\" y=\"0\" width=\"8\" height=\"8\" /></clipPath>",
            "<mask id=\"mk\"><circle cx=\"4\" cy=\"4\" r=\"3\" /></mask>",
            "<filter id=\"fl\"><feGaussianBlur in=\"SourceGraphic\" stdDeviation=\"1.5\" result=\"b\" />",
            "<feOffset in=\"b\" dx=\"1\" dy=\"1\" result=\"o\" />",
            "<feComposite in=\"o\" in2=\"SourceAlpha\" operator=\"over\" result=\"c\" />",
            "<feMerge><feMergeNode in=\"c\" /></feMerge></filter></defs>",
            "<g transform=\"translate(1 2) rotate(30)\" clip-path=\"url(#cp)\" mask=\"url(#mk)\" filter=\"url(#fl)\">",
            "<rect x=\"0\" y=\"0\" width=\"8\" height=\"8\" fill=\"url(#lg)\" stroke=\"url(#rg)\" />",
            "</g>"
        );
        let text = canonical(body);
        for tag in [
            "linearGradient",
            "radialGradient",
            "clipPath",
            "mask",
            "feGaussianBlur",
            "feOffset",
            "feComposite",
            "feMerge",
            "feMergeNode",
            "transform=",
        ] {
            assert!(text.contains(tag), "missing {tag}");
        }
        let facts = accepted(body).facts();
        assert_eq!(facts.filters(), 1);
        assert!(facts.references() >= 5);
    }

    #[test]
    fn svg_04_text_becomes_geometric_outlines_with_a_title() {
        let text =
            canonical("<text x=\"2\" y=\"3\" font-size=\"14\" fill=\"#123456\">Hi 42!</text>");
        assert!(text.contains("<g transform=\"translate(2 3)\">"), "{text}");
        assert!(text.contains("<title>Hi 42!</title>"), "semantics retained");
        assert!(text.contains("fill=\"#123456\""));
        assert!(text.contains("<path d=\"M"), "text becomes a path");
        assert!(!text.contains("<text"), "no host-dependent text survives");
        assert!(!text.contains("font"), "no font request survives");
    }

    #[test]
    fn svg_05_current_color_and_class_tokens_are_resolved_and_inert() {
        let text = canonical(
            "<g class=\"icon  icon-tabler\" stroke=\"currentColor\"><path d=\"M0 0h4v4H0z\" /></g>",
        );
        assert!(text.contains("stroke=\"#000000\""), "{text}");
        assert!(
            text.contains("class=\"icon icon-tabler\""),
            "tokens re-emitted single spaced"
        );
    }

    #[test]
    fn svg_06_canonical_output_is_idempotent() {
        let first = accepted("<g><path d=\"M0 0 h1 v1 h-1 z\" fill=\"#ff0000\" /></g>");
        let second = sanitize_svg_v1(first.canonical()).expect("canonical output re-parses");
        assert_eq!(first.canonical(), second.canonical());
        assert_eq!(first.canonical_digest(), second.canonical_digest());
    }

    #[test]
    fn svg_07_the_pinned_foundation_icon_is_accepted() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../solo-cli/tests/fixtures/svg/icon-home.svg"
        );
        let raw = std::fs::read(path).expect("fixture present");
        let normalized: Vec<u8> = {
            let mut out = Vec::with_capacity(raw.len());
            let mut index = 0;
            while index < raw.len() {
                if raw[index] == b'\r' && raw.get(index + 1) == Some(&b'\n') {
                    index += 1;
                    continue;
                }
                out.push(raw[index]);
                index += 1;
            }
            out
        };
        let sanitized = sanitize_svg_v1(&normalized).expect("foundation icon accepted");
        let text = String::from_utf8(sanitized.canonical().to_vec()).expect("utf8");
        assert!(text.contains("class=\"icon icon-tabler icons-tabler-outline icon-tabler-home\""));
        assert!(
            text.contains("stroke=\"#000000\""),
            "currentColor resolved: {text}"
        );
        assert_eq!(sanitized.facts().width(), 24);
        assert_eq!(sanitized.facts().height(), 24);
        assert!(!text.contains("currentColor"));
    }

    #[test]
    fn svg_08_active_and_external_constructs_refuse_whole() {
        let cases: [(&str, SvgRefusalV1); 12] = [
            (
                "<script>alert(1)</script>",
                SvgRefusalV1::UnsupportedElement,
            ),
            (
                "<foreignObject><div>x</div></foreignObject>",
                SvgRefusalV1::UnsupportedElement,
            ),
            (
                "<image href=\"https://example.com/a.png\" />",
                SvgRefusalV1::UnsupportedElement,
            ),
            ("<a href=\"#x\">y</a>", SvgRefusalV1::UnsupportedElement),
            (
                "<animate attributeName=\"x\" />",
                SvgRefusalV1::UnsupportedElement,
            ),
            (
                "<path d=\"M0 0\" onclick=\"x()\" />",
                SvgRefusalV1::UnsupportedAttribute,
            ),
            (
                "<path d=\"M0 0\" style=\"fill:red\" />",
                SvgRefusalV1::UnsupportedAttribute,
            ),
            ("<use href=\"#x\" />", SvgRefusalV1::UnsupportedElement),
            (
                "<style>.a{fill:red}</style>",
                SvgRefusalV1::UnsupportedElement,
            ),
            ("<g><span>x</span></g>", SvgRefusalV1::UnsupportedElement),
            (
                "<path d=\"M0 0\" fill=\"url(https://example.com/x.svg#a)\" />",
                SvgRefusalV1::InvalidAttributeValue,
            ),
            (
                "<path d=\"M0 0\" transform=\"matrix(NaN 0 0 1 0 0)\" />",
                SvgRefusalV1::InvalidAttributeValue,
            ),
        ];
        for (body, expected) in cases {
            let refused = sanitize_svg_v1(document(body).as_bytes());
            assert_eq!(refused.err(), Some(expected), "body: {body}");
        }
    }

    #[test]
    fn svg_09_dtd_comments_pi_and_cdata_refuse() {
        let cases = [
            "<!DOCTYPE svg [<!ENTITY x \"y\">]><svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1\" height=\"1\" />",
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1\" height=\"1\"><!-- c --></svg>",
            "<?xml version=\"1.0\"?><svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1\" height=\"1\" />",
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1\" height=\"1\"><desc><![CDATA[x]]></desc></svg>",
        ];
        for case in cases {
            assert_eq!(
                sanitize_svg_v1(case.as_bytes()).err(),
                Some(SvgRefusalV1::UnsupportedConstruct),
                "case: {case}"
            );
        }
    }

    #[test]
    fn svg_10_budgets_and_geometry_refuse() {
        assert_eq!(sanitize_svg_v1(b"").err(), Some(SvgRefusalV1::EmptySource));
        assert_eq!(
            sanitize_svg_v1(
                b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"0\" height=\"24\" />"
            )
            .err(),
            Some(SvgRefusalV1::DimensionsOutOfRange)
        );
        assert_eq!(
            sanitize_svg_v1(
                b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"8192\" height=\"8192\" />"
            )
            .err(),
            Some(SvgRefusalV1::PixelCountOutOfRange)
        );
        let deep = format!(
            "<svg xmlns=\"{SVG_NAMESPACE_V1}\" width=\"8\" height=\"8\">{}{}</svg>",
            "<g>".repeat(40),
            "</g>".repeat(40)
        );
        assert_eq!(
            sanitize_svg_v1(deep.as_bytes()).err(),
            Some(SvgRefusalV1::NestingTooDeep)
        );
        let many_filters = format!(
            "<svg xmlns=\"{SVG_NAMESPACE_V1}\" width=\"8\" height=\"8\"><defs>{}</defs></svg>",
            "<filter id=\"f\"><feGaussianBlur stdDeviation=\"1\" /></filter>".repeat(65)
        );
        assert!(matches!(
            sanitize_svg_v1(many_filters.as_bytes()).err(),
            Some(SvgRefusalV1::FilterBudgetExceeded) | Some(SvgRefusalV1::DuplicateId)
        ));
    }

    #[test]
    fn svg_11_references_resolve_and_cycles_refuse() {
        assert_eq!(
            sanitize_svg_v1(
                document("<rect width=\"1\" height=\"1\" fill=\"url(#missing)\" />").as_bytes()
            )
            .err(),
            Some(SvgRefusalV1::ReferenceUnresolved)
        );
        let cycle = concat!(
            "<defs><linearGradient id=\"a\" href=\"#b\"><stop offset=\"0\" stop-color=\"#fff\" /></linearGradient>",
            "<linearGradient id=\"b\" href=\"#a\"><stop offset=\"1\" stop-color=\"#000\" /></linearGradient></defs>",
            "<rect width=\"1\" height=\"1\" fill=\"url(#a)\" />"
        );
        assert_eq!(
            sanitize_svg_v1(document(cycle).as_bytes()).err(),
            Some(SvgRefusalV1::ReferenceCycle)
        );
    }

    #[test]
    fn svg_12_unsupported_text_characters_refuse() {
        assert_eq!(
            sanitize_svg_v1(document("<text font-size=\"14\">\u{00e9}</text>").as_bytes()).err(),
            Some(SvgRefusalV1::TextCharacterUnsupported)
        );
        assert_eq!(
            sanitize_svg_v1(document("<text font-size=\"14\">\u{4e16}</text>").as_bytes()).err(),
            Some(SvgRefusalV1::TextCharacterUnsupported)
        );
    }

    #[test]
    fn svg_13_duplicate_ids_and_namespace_prefixes_refuse() {
        assert_eq!(
            sanitize_svg_v1(document("<g id=\"a\" /><g id=\"a\" />").as_bytes()).err(),
            Some(SvgRefusalV1::DuplicateId)
        );
        assert_eq!(
            sanitize_svg_v1(
                b"<svg:svg xmlns:svg=\"http://www.w3.org/2000/svg\" width=\"1\" height=\"1\" />"
            )
            .err(),
            Some(SvgRefusalV1::UnsupportedElement)
        );
        assert_eq!(
            sanitize_svg_v1(b"<svg width=\"8\" height=\"8\" xmlns=\"urn:not-svg\" />").err(),
            Some(SvgRefusalV1::InvalidAttributeValue)
        );
    }

    #[test]
    fn svg_14_every_glyph_in_the_repertoire_renders() {
        // `&` and `<` are deliberately absent: an XML entity reference is refused outright, so
        // those characters are not representable in retained text and must refuse rather than
        // silently vanish.
        let text =
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789 .,:;!?'\"()-+/#*=@%";
        let sanitized = sanitize_svg_v1(
            document(&format!(
                "<text x=\"0\" y=\"0\" font-size=\"14\">{text}</text>"
            ))
            .as_bytes(),
        )
        .expect("full repertoire accepted");
        assert!(sanitized.facts().path_commands() > 1000);
        assert!(sanitized.facts().path_commands() <= MAX_SVG_PATH_COMMANDS_V1);
    }
}
