//! Strict worker-envelope protocol parsing (`R03.C3`, outer adapter half).
//!
//! This module owns exactly one job: turn **already-supplied** worker metadata bytes into the
//! inward-owned [`MediaWorkerEnvelopeV1`] claim, or refuse them. It performs no I/O, starts no
//! child, reads no file, holds no handle and makes no media claim.
//!
//! **Why the parser is written by hand.** `solo-adapters` intentionally depends on `serde` without
//! its `derive` feature — the accepted manifest parser in this same crate hand-implements
//! `Deserialize` with explicit visitors, and enabling `derive` would add a dependency edge and
//! change the locked graph, which this work order forbids. The hand-written visitors below
//! therefore reproduce, and in several places strengthen, every property the contract requires: a
//! single complete UTF-8 JSON object, a size ceiling, closed keys at every nesting level, duplicate
//! rejection at every nesting level, no floats, no negatives, no overflow, no aliases, no comments,
//! no trailing document, no free-form `serde_json::Value` shortcut and no silent normalisation.
//!
//! **What a success proves.** Only that the bytes were a structurally valid version 1 envelope
//! claim. Whether the claim is *true* is decided entirely by the parent application validator
//! against the source and plane bytes it owns; this parser never asserts a successful decode,
//! isolation, licensing or publication.

use std::fmt;

use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
use serde_json::error::Category;

use solo_application::{
    MEDIA_WORKER_MAX_ENVELOPE_BYTES_V1, MEDIA_WORKER_PROTOCOL_VERSION_V1, MediaWorkerColorModelV1,
    MediaWorkerDigestV1, MediaWorkerEnvelopeV1, MediaWorkerFormatV1, MediaWorkerMimeV1,
    MediaWorkerOrientationV1, MediaWorkerPlaneClaimV1, MediaWorkerStatusV1,
};

/// The stable message prefix this parser uses for a duplicate key at any nesting level.
const DUPLICATE_FIELD_MESSAGE: &str = "duplicate field";

/// The stable message prefix this parser uses for an unknown key at any nesting level.
const UNKNOWN_FIELD_MESSAGE: &str = "unknown field";

/// The stable message prefix this parser uses for a missing key.
const MISSING_FIELD_MESSAGE: &str = "missing field";

/// The closed root key set, in the contract's own order.
const ROOT_FIELDS: [&str; 16] = [
    "version",
    "status",
    "format",
    "mime",
    "source_length",
    "source_sha256",
    "width",
    "height",
    "bit_depth",
    "color_model",
    "has_alpha",
    "is_grid",
    "is_sequence",
    "orientation",
    "planes",
    "output_sha256",
];

/// The closed per-plane key set.
const PLANE_FIELDS: [&str; 5] = ["ordinal", "stride", "rows", "length", "sha256"];

/// The largest accepted plane list, matching the largest colour model's plane count.
const MAX_PLANES: usize = 4;

/// Every way the protocol parser can refuse, with one stable token each.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaWorkerProtocolRefusalKindV1 {
    /// The supplied bytes were empty.
    Empty,
    /// The supplied bytes exceeded [`MEDIA_WORKER_MAX_ENVELOPE_BYTES_V1`], whitespace included.
    Oversize,
    /// The bytes were not one complete well-formed JSON object.
    Syntax,
    /// The same key appeared twice inside one object.
    DuplicateField,
    /// An object carried a key outside the closed schema.
    UnknownField,
    /// A required key was absent.
    MissingField,
    /// A value had the wrong type, token, spelling, range or plane count.
    FieldValue,
    /// A second JSON document followed the first.
    TrailingData,
}

impl MediaWorkerProtocolRefusalKindV1 {
    /// The stable diagnostic token. It carries no worker-supplied text.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::Empty => "MEDIA_WORKER_ENVELOPE_EMPTY",
            Self::Oversize => "MEDIA_WORKER_ENVELOPE_OVERSIZE",
            Self::Syntax => "MEDIA_WORKER_ENVELOPE_SYNTAX",
            Self::DuplicateField => "MEDIA_WORKER_ENVELOPE_DUPLICATE_FIELD",
            Self::UnknownField => "MEDIA_WORKER_ENVELOPE_UNKNOWN_FIELD",
            Self::MissingField => "MEDIA_WORKER_ENVELOPE_MISSING_FIELD",
            Self::FieldValue => "MEDIA_WORKER_ENVELOPE_FIELD_VALUE",
            Self::TrailingData => "MEDIA_WORKER_ENVELOPE_TRAILING_DATA",
        }
    }

    /// The fixed, bounded explanation. It never repeats worker-supplied text.
    #[must_use]
    pub const fn detail(self) -> &'static str {
        match self {
            Self::Empty => "no envelope bytes were supplied",
            Self::Oversize => "the envelope exceeds the documented size bound",
            Self::Syntax => "the envelope is not one complete well-formed JSON object",
            Self::DuplicateField => "the envelope repeats a key inside one object",
            Self::UnknownField => "the envelope carries a key outside the closed schema",
            Self::MissingField => "the envelope omits a required key",
            Self::FieldValue => "an envelope value is not acceptable in the closed schema",
            Self::TrailingData => "the envelope is followed by further input",
        }
    }
}

impl fmt::Display for MediaWorkerProtocolRefusalKindV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code(), self.detail())
    }
}

/// One typed protocol refusal.
///
/// It carries only the stable category: never a worker-supplied token, digest, path or diagnostic
/// string, because a worker's own text is not evidence and must not reach a caller's log.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MediaWorkerProtocolRefusalV1 {
    kind: MediaWorkerProtocolRefusalKindV1,
}

impl MediaWorkerProtocolRefusalV1 {
    /// A refusal of exactly one category.
    #[must_use]
    pub const fn of(kind: MediaWorkerProtocolRefusalKindV1) -> Self {
        Self { kind }
    }

    /// The stable category.
    #[must_use]
    pub const fn kind(&self) -> MediaWorkerProtocolRefusalKindV1 {
        self.kind
    }

    /// The stable diagnostic token.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        self.kind.code()
    }

    /// The fixed, bounded explanation.
    #[must_use]
    pub const fn detail(&self) -> &'static str {
        self.kind.detail()
    }
}

impl fmt::Display for MediaWorkerProtocolRefusalV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code(), self.detail())
    }
}

impl std::error::Error for MediaWorkerProtocolRefusalV1 {}

/// One recognized root key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EnvelopeField {
    Version,
    Status,
    Format,
    Mime,
    SourceLength,
    SourceSha256,
    Width,
    Height,
    BitDepth,
    ColorModel,
    HasAlpha,
    IsGrid,
    IsSequence,
    Orientation,
    Planes,
    OutputSha256,
}

impl EnvelopeField {
    /// The bit this key owns in the duplicate-detection mask.
    const fn bit(self) -> u32 {
        1 << (self as u32)
    }
}

impl<'de> Deserialize<'de> for EnvelopeField {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_identifier(EnvelopeFieldVisitor)
    }
}

struct EnvelopeFieldVisitor;

impl<'de> Visitor<'de> for EnvelopeFieldVisitor {
    type Value = EnvelopeField;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a key of the closed version 1 media worker envelope")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        let field = match value {
            "version" => EnvelopeField::Version,
            "status" => EnvelopeField::Status,
            "format" => EnvelopeField::Format,
            "mime" => EnvelopeField::Mime,
            "source_length" => EnvelopeField::SourceLength,
            "source_sha256" => EnvelopeField::SourceSha256,
            "width" => EnvelopeField::Width,
            "height" => EnvelopeField::Height,
            "bit_depth" => EnvelopeField::BitDepth,
            "color_model" => EnvelopeField::ColorModel,
            "has_alpha" => EnvelopeField::HasAlpha,
            "is_grid" => EnvelopeField::IsGrid,
            "is_sequence" => EnvelopeField::IsSequence,
            "orientation" => EnvelopeField::Orientation,
            "planes" => EnvelopeField::Planes,
            "output_sha256" => EnvelopeField::OutputSha256,
            other => {
                return Err(E::unknown_field(other, &ROOT_FIELDS));
            }
        };
        Ok(field)
    }
}

/// One recognized per-plane key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PlaneField {
    Ordinal,
    Stride,
    Rows,
    Length,
    Sha256,
}

impl PlaneField {
    /// The bit this key owns in the duplicate-detection mask.
    const fn bit(self) -> u32 {
        1 << (self as u32)
    }
}

impl<'de> Deserialize<'de> for PlaneField {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_identifier(PlaneFieldVisitor)
    }
}

struct PlaneFieldVisitor;

impl<'de> Visitor<'de> for PlaneFieldVisitor {
    type Value = PlaneField;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a key of the closed version 1 plane claim")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        let field = match value {
            "ordinal" => PlaneField::Ordinal,
            "stride" => PlaneField::Stride,
            "rows" => PlaneField::Rows,
            "length" => PlaneField::Length,
            "sha256" => PlaneField::Sha256,
            other => {
                return Err(E::unknown_field(other, &PLANE_FIELDS));
            }
        };
        Ok(field)
    }
}

/// One parsed plane claim, still holding its digest as unvalidated text.
#[derive(Debug)]
struct WirePlane {
    ordinal: u32,
    stride: u64,
    rows: u64,
    length: u64,
    sha256: String,
}

impl<'de> Deserialize<'de> for WirePlane {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(WirePlaneVisitor)
    }
}

struct WirePlaneVisitor;

impl<'de> Visitor<'de> for WirePlaneVisitor {
    type Value = WirePlane;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a closed version 1 plane claim object")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut seen: u32 = 0;
        let mut ordinal: Option<u32> = None;
        let mut stride: Option<u64> = None;
        let mut rows: Option<u64> = None;
        let mut length: Option<u64> = None;
        let mut sha256: Option<String> = None;
        while let Some(field) = map.next_key::<PlaneField>()? {
            if seen & field.bit() != 0 {
                return Err(de::Error::custom(DUPLICATE_FIELD_MESSAGE));
            }
            seen |= field.bit();
            match field {
                PlaneField::Ordinal => ordinal = Some(map.next_value()?),
                PlaneField::Stride => stride = Some(map.next_value()?),
                PlaneField::Rows => rows = Some(map.next_value()?),
                PlaneField::Length => length = Some(map.next_value()?),
                PlaneField::Sha256 => sha256 = Some(map.next_value()?),
            }
        }
        Ok(WirePlane {
            ordinal: ordinal
                .ok_or_else(|| de::Error::custom(format!("{MISSING_FIELD_MESSAGE} `ordinal`")))?,
            stride: stride
                .ok_or_else(|| de::Error::custom(format!("{MISSING_FIELD_MESSAGE} `stride`")))?,
            rows: rows
                .ok_or_else(|| de::Error::custom(format!("{MISSING_FIELD_MESSAGE} `rows`")))?,
            length: length
                .ok_or_else(|| de::Error::custom(format!("{MISSING_FIELD_MESSAGE} `length`")))?,
            sha256: sha256
                .ok_or_else(|| de::Error::custom(format!("{MISSING_FIELD_MESSAGE} `sha256`")))?,
        })
    }
}

/// One parsed envelope, still holding its tokens and digests as unvalidated text.
#[derive(Debug)]
struct WireEnvelope {
    version: u32,
    status: String,
    format: String,
    mime: String,
    source_length: u64,
    source_sha256: String,
    width: u32,
    height: u32,
    bit_depth: u8,
    color_model: String,
    has_alpha: bool,
    is_grid: bool,
    is_sequence: bool,
    orientation: String,
    planes: Vec<WirePlane>,
    output_sha256: String,
}

/// The in-progress field set, so a missing key is distinguishable from a bad value.
#[derive(Default)]
struct PartialEnvelope {
    version: Option<u32>,
    status: Option<String>,
    format: Option<String>,
    mime: Option<String>,
    source_length: Option<u64>,
    source_sha256: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
    bit_depth: Option<u8>,
    color_model: Option<String>,
    has_alpha: Option<bool>,
    is_grid: Option<bool>,
    is_sequence: Option<bool>,
    orientation: Option<String>,
    planes: Option<Vec<WirePlane>>,
    output_sha256: Option<String>,
}

impl PartialEnvelope {
    /// Requires every key to have been present exactly once, then materializes the wire value.
    fn finish<E: de::Error>(self) -> Result<WireEnvelope, E> {
        let missing = |name: &str| E::custom(format!("{MISSING_FIELD_MESSAGE} `{name}`"));
        Ok(WireEnvelope {
            version: self.version.ok_or_else(|| missing("version"))?,
            status: self.status.ok_or_else(|| missing("status"))?,
            format: self.format.ok_or_else(|| missing("format"))?,
            mime: self.mime.ok_or_else(|| missing("mime"))?,
            source_length: self.source_length.ok_or_else(|| missing("source_length"))?,
            source_sha256: self.source_sha256.ok_or_else(|| missing("source_sha256"))?,
            width: self.width.ok_or_else(|| missing("width"))?,
            height: self.height.ok_or_else(|| missing("height"))?,
            bit_depth: self.bit_depth.ok_or_else(|| missing("bit_depth"))?,
            color_model: self.color_model.ok_or_else(|| missing("color_model"))?,
            has_alpha: self.has_alpha.ok_or_else(|| missing("has_alpha"))?,
            is_grid: self.is_grid.ok_or_else(|| missing("is_grid"))?,
            is_sequence: self.is_sequence.ok_or_else(|| missing("is_sequence"))?,
            orientation: self.orientation.ok_or_else(|| missing("orientation"))?,
            planes: self.planes.ok_or_else(|| missing("planes"))?,
            output_sha256: self.output_sha256.ok_or_else(|| missing("output_sha256"))?,
        })
    }
}

impl<'de> Deserialize<'de> for WireEnvelope {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(WireEnvelopeVisitor)
    }
}

struct WireEnvelopeVisitor;

impl<'de> Visitor<'de> for WireEnvelopeVisitor {
    type Value = WireEnvelope;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a closed version 1 media worker envelope object")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut seen: u32 = 0;
        let mut partial = PartialEnvelope::default();
        while let Some(field) = map.next_key::<EnvelopeField>()? {
            if seen & field.bit() != 0 {
                return Err(de::Error::custom(DUPLICATE_FIELD_MESSAGE));
            }
            seen |= field.bit();
            match field {
                EnvelopeField::Version => partial.version = Some(map.next_value()?),
                EnvelopeField::Status => partial.status = Some(map.next_value()?),
                EnvelopeField::Format => partial.format = Some(map.next_value()?),
                EnvelopeField::Mime => partial.mime = Some(map.next_value()?),
                EnvelopeField::SourceLength => partial.source_length = Some(map.next_value()?),
                EnvelopeField::SourceSha256 => partial.source_sha256 = Some(map.next_value()?),
                EnvelopeField::Width => partial.width = Some(map.next_value()?),
                EnvelopeField::Height => partial.height = Some(map.next_value()?),
                EnvelopeField::BitDepth => partial.bit_depth = Some(map.next_value()?),
                EnvelopeField::ColorModel => partial.color_model = Some(map.next_value()?),
                EnvelopeField::HasAlpha => partial.has_alpha = Some(map.next_value()?),
                EnvelopeField::IsGrid => partial.is_grid = Some(map.next_value()?),
                EnvelopeField::IsSequence => partial.is_sequence = Some(map.next_value()?),
                EnvelopeField::Orientation => partial.orientation = Some(map.next_value()?),
                EnvelopeField::Planes => partial.planes = Some(map.next_value()?),
                EnvelopeField::OutputSha256 => partial.output_sha256 = Some(map.next_value()?),
            }
        }
        partial.finish()
    }
}

/// Parses exactly one complete worker metadata envelope.
///
/// The input must be one UTF-8 JSON object of at most [`MEDIA_WORKER_MAX_ENVELOPE_BYTES_V1`] bytes
/// including whitespace, with the closed key set at both nesting levels, no duplicate key
/// anywhere, no float, negative, overflowing or unrepresentable number, no alias, no comment and
/// no trailing document. On success the value is an **untrusted claim**: it becomes evidence only
/// after the parent application validator re-checks it against the bytes it owns.
///
/// # Errors
///
/// Returns [`MediaWorkerProtocolRefusalKindV1::Empty`] or
/// [`MediaWorkerProtocolRefusalKindV1::Oversize`] before any parsing, then a syntax, duplicate,
/// unknown, missing, value or trailing-data refusal. The refusal never repeats worker-supplied
/// text.
pub fn parse_media_worker_envelope_v1(
    bytes: &[u8],
) -> Result<MediaWorkerEnvelopeV1, MediaWorkerProtocolRefusalV1> {
    if bytes.is_empty() {
        return Err(MediaWorkerProtocolRefusalV1::of(
            MediaWorkerProtocolRefusalKindV1::Empty,
        ));
    }
    if bytes.len() > MEDIA_WORKER_MAX_ENVELOPE_BYTES_V1 {
        return Err(MediaWorkerProtocolRefusalV1::of(
            MediaWorkerProtocolRefusalKindV1::Oversize,
        ));
    }

    let mut deserializer = serde_json::Deserializer::from_slice(bytes);
    let wire = WireEnvelope::deserialize(&mut deserializer)
        .map_err(|error| MediaWorkerProtocolRefusalV1::of(classify(&error)))?;
    // A successfully parsed object may still be followed by more input; the contract accepts one
    // document and nothing else, and `end()` is the deterministic check for that.
    deserializer.end().map_err(|_| {
        MediaWorkerProtocolRefusalV1::of(MediaWorkerProtocolRefusalKindV1::TrailingData)
    })?;

    map_envelope_v1(wire)
}

/// Maps one parsed wire value into the inward-owned claim, or refuses an unacceptable value.
fn map_envelope_v1(
    wire: WireEnvelope,
) -> Result<MediaWorkerEnvelopeV1, MediaWorkerProtocolRefusalV1> {
    let invalid = || MediaWorkerProtocolRefusalV1::of(MediaWorkerProtocolRefusalKindV1::FieldValue);

    if wire.version != MEDIA_WORKER_PROTOCOL_VERSION_V1
        || wire.status != MediaWorkerStatusV1::Decoded.as_str()
        || wire.format != MediaWorkerFormatV1::Avif.as_str()
        || wire.mime != MediaWorkerMimeV1::ImageAvif.as_str()
    {
        return Err(invalid());
    }
    let color_model = MediaWorkerColorModelV1::from_token(&wire.color_model).ok_or_else(invalid)?;
    let orientation =
        MediaWorkerOrientationV1::from_token(&wire.orientation).ok_or_else(invalid)?;
    let source_sha256 =
        MediaWorkerDigestV1::parse_lower_hex(&wire.source_sha256).ok_or_else(invalid)?;
    let output_sha256 =
        MediaWorkerDigestV1::parse_lower_hex(&wire.output_sha256).ok_or_else(invalid)?;

    if wire.planes.is_empty() || wire.planes.len() > MAX_PLANES {
        return Err(invalid());
    }
    let mut planes = Vec::with_capacity(wire.planes.len());
    for plane in &wire.planes {
        let sha256 = MediaWorkerDigestV1::parse_lower_hex(&plane.sha256).ok_or_else(invalid)?;
        planes.push(MediaWorkerPlaneClaimV1::new(
            plane.ordinal,
            plane.stride,
            plane.rows,
            plane.length,
            sha256,
        ));
    }

    Ok(MediaWorkerEnvelopeV1::new(
        wire.version,
        MediaWorkerStatusV1::Decoded,
        MediaWorkerFormatV1::Avif,
        MediaWorkerMimeV1::ImageAvif,
        wire.source_length,
        source_sha256,
        wire.width,
        wire.height,
        wire.bit_depth,
        color_model,
        wire.has_alpha,
        wire.is_grid,
        wire.is_sequence,
        orientation,
        planes,
        output_sha256,
    ))
}

/// Maps one `serde_json` failure onto the stable refusal categories.
///
/// Duplicate, unknown and missing keys are recognized by this parser's own fixed message prefixes,
/// so the classification never depends on the wording of an arbitrary third-party diagnostic.
fn classify(error: &serde_json::Error) -> MediaWorkerProtocolRefusalKindV1 {
    match error.classify() {
        Category::Data => {
            let message = error.message();
            if message.starts_with(DUPLICATE_FIELD_MESSAGE) {
                MediaWorkerProtocolRefusalKindV1::DuplicateField
            } else if message.starts_with(UNKNOWN_FIELD_MESSAGE) {
                MediaWorkerProtocolRefusalKindV1::UnknownField
            } else if message.starts_with(MISSING_FIELD_MESSAGE) {
                MediaWorkerProtocolRefusalKindV1::MissingField
            } else {
                MediaWorkerProtocolRefusalKindV1::FieldValue
            }
        }
        _ => MediaWorkerProtocolRefusalKindV1::Syntax,
    }
}
