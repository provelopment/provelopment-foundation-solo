//! `SOLO-WO-118 R12` — the strict, bounded `1.0.0` Platform asset catalogue parser.
//!
//! It parses bytes the caller already owns into the closed typed contract of
//! [`solo_application::PlatformAssetCatalogueV1`]. It opens no file, resolves no directory, reads no
//! Platform source and grants no authority: a successful parse proves only that one document is
//! syntactically and structurally admissible **for review**.
//!
//! **Duplicate keys are rejected at every object depth.** `serde`'s `derive` feature is not part of
//! the locked feature set, so every nested shape is a hand-written `Deserialize` visitor with an
//! explicit duplicate mask and a closed key set — the same pattern the accepted worker-protocol
//! parser already uses. A `serde_json::Value` round trip, which silently overwrites duplicates, is
//! never used as a proof.

use std::fmt;

use serde::de::{Deserialize, Deserializer, Error as DeError, MapAccess, Visitor};
use serde_json::error::Category;
use solo_application::{
    MAX_PLATFORM_CATALOGUE_ENTRIES_V1, PlatformAssetCatalogueEntryV1, PlatformAssetCatalogueV1,
    PlatformAssetDeclarationV1, PlatformAssetLicenceV1, PlatformAssetProvenanceV1,
    PlatformCatalogueRefusalV1,
};

/// The maximum accepted catalogue document size in bytes.
pub const MAX_PLATFORM_CATALOGUE_BYTES_V1: usize = 2 * 1024 * 1024;

/// The stable message prefix this parser uses for a repeated key at any nesting level.
const DUPLICATE_FIELD_MESSAGE: &str = "duplicate field";

/// The stable message prefix this parser uses for an unknown key at any nesting level.
const UNKNOWN_FIELD_MESSAGE: &str = "unknown field";

/// The stable message prefix this parser uses for a missing key.
const MISSING_FIELD_MESSAGE: &str = "missing field";

/// The closed root key set, in the provisional `1.0.0` order.
const ROOT_FIELDS: [&str; 2] = ["platformAssetLibraryVersion", "assets"];

/// The closed asset key set.
const ASSET_FIELDS: [&str; 8] = [
    "id",
    "sha256",
    "mediaClass",
    "mime",
    "width",
    "height",
    "licence",
    "provenance",
];

/// The closed licence key set.
const LICENCE_FIELDS: [&str; 3] = ["expression", "rightsHolder", "evidence"];

/// The closed provenance key set.
const PROVENANCE_FIELDS: [&str; 2] = ["source", "evidence"];

/// One recognized asset key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AssetField {
    Id,
    Sha256,
    MediaClass,
    Mime,
    Width,
    Height,
    Licence,
    Provenance,
}

impl AssetField {
    const fn bit(self) -> u32 {
        1 << (self as u32)
    }
}

impl<'de> Deserialize<'de> for AssetField {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_identifier(AssetFieldVisitor)
    }
}

struct AssetFieldVisitor;

impl Visitor<'_> for AssetFieldVisitor {
    type Value = AssetField;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a closed catalogue asset key")
    }

    fn visit_str<E: DeError>(self, value: &str) -> Result<Self::Value, E> {
        match value {
            "id" => Ok(AssetField::Id),
            "sha256" => Ok(AssetField::Sha256),
            "mediaClass" => Ok(AssetField::MediaClass),
            "mime" => Ok(AssetField::Mime),
            "width" => Ok(AssetField::Width),
            "height" => Ok(AssetField::Height),
            "licence" => Ok(AssetField::Licence),
            "provenance" => Ok(AssetField::Provenance),
            other => Err(E::custom(format!(
                "{UNKNOWN_FIELD_MESSAGE} `{other}` in a catalogue asset"
            ))),
        }
    }
}

/// One recognized licence key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LicenceField {
    Expression,
    RightsHolder,
    Evidence,
}

impl LicenceField {
    const fn bit(self) -> u32 {
        1 << (self as u32)
    }
}

impl<'de> Deserialize<'de> for LicenceField {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_identifier(LicenceFieldVisitor)
    }
}

struct LicenceFieldVisitor;

impl Visitor<'_> for LicenceFieldVisitor {
    type Value = LicenceField;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a closed licence key")
    }

    fn visit_str<E: DeError>(self, value: &str) -> Result<Self::Value, E> {
        match value {
            "expression" => Ok(LicenceField::Expression),
            "rightsHolder" => Ok(LicenceField::RightsHolder),
            "evidence" => Ok(LicenceField::Evidence),
            other => Err(E::custom(format!(
                "{UNKNOWN_FIELD_MESSAGE} `{other}` in licence"
            ))),
        }
    }
}

/// One recognized provenance key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProvenanceField {
    Source,
    Evidence,
}

impl ProvenanceField {
    const fn bit(self) -> u32 {
        1 << (self as u32)
    }
}

impl<'de> Deserialize<'de> for ProvenanceField {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_identifier(ProvenanceFieldVisitor)
    }
}

struct ProvenanceFieldVisitor;

impl Visitor<'_> for ProvenanceFieldVisitor {
    type Value = ProvenanceField;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a closed provenance key")
    }

    fn visit_str<E: DeError>(self, value: &str) -> Result<Self::Value, E> {
        match value {
            "source" => Ok(ProvenanceField::Source),
            "evidence" => Ok(ProvenanceField::Evidence),
            other => Err(E::custom(format!(
                "{UNKNOWN_FIELD_MESSAGE} `{other}` in provenance"
            ))),
        }
    }
}

/// One recognized root key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RootField {
    Version,
    Assets,
}

impl<'de> Deserialize<'de> for RootField {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_identifier(RootFieldVisitor)
    }
}

impl RootField {
    const fn bit(self) -> u32 {
        1 << (self as u32)
    }
}

struct RootFieldVisitor;

impl Visitor<'_> for RootFieldVisitor {
    type Value = RootField;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a closed catalogue key")
    }

    fn visit_str<E: DeError>(self, value: &str) -> Result<Self::Value, E> {
        match value {
            "platformAssetLibraryVersion" => Ok(RootField::Version),
            "assets" => Ok(RootField::Assets),
            other => Err(E::custom(format!(
                "{UNKNOWN_FIELD_MESSAGE} `{other}` at catalogue root"
            ))),
        }
    }
}

/// The parsed wire shape of one catalogue document.
#[derive(Debug)]
struct WireCatalogue {
    platform_asset_library_version: String,
    assets: Vec<WireAsset>,
}

impl<'de> Deserialize<'de> for WireCatalogue {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(WireCatalogueVisitor)
    }
}

struct WireCatalogueVisitor;

impl<'de> Visitor<'de> for WireCatalogueVisitor {
    type Value = WireCatalogue;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a closed platform asset catalogue object")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut seen: u32 = 0;
        let mut version: Option<String> = None;
        let mut assets: Option<Vec<WireAsset>> = None;
        while let Some(field) = map.next_key::<RootField>()? {
            if seen & field.bit() != 0 {
                return Err(DeError::custom(DUPLICATE_FIELD_MESSAGE));
            }
            seen |= field.bit();
            match field {
                RootField::Version => version = Some(map.next_value()?),
                RootField::Assets => assets = Some(map.next_value()?),
            }
        }
        Ok(WireCatalogue {
            platform_asset_library_version: version.ok_or_else(|| {
                DeError::custom(format!(
                    "{MISSING_FIELD_MESSAGE} `platformAssetLibraryVersion` in {ROOT_FIELDS:?}"
                ))
            })?,
            assets: assets.ok_or_else(|| {
                DeError::custom(format!(
                    "{MISSING_FIELD_MESSAGE} `assets` in {ROOT_FIELDS:?}"
                ))
            })?,
        })
    }
}

/// The parsed wire shape of one catalogue asset.
#[derive(Debug)]
struct WireAsset {
    id: String,
    sha256: String,
    media_class: String,
    mime: String,
    width: u32,
    height: u32,
    licence: WireLicence,
    provenance: WireProvenance,
}

impl<'de> Deserialize<'de> for WireAsset {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(WireAssetVisitor)
    }
}

struct WireAssetVisitor;

impl<'de> Visitor<'de> for WireAssetVisitor {
    type Value = WireAsset;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a closed platform catalogue asset object")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut seen: u32 = 0;
        let mut id: Option<String> = None;
        let mut sha256: Option<String> = None;
        let mut media_class: Option<String> = None;
        let mut mime: Option<String> = None;
        let mut width: Option<u32> = None;
        let mut height: Option<u32> = None;
        let mut licence: Option<WireLicence> = None;
        let mut provenance: Option<WireProvenance> = None;
        while let Some(field) = map.next_key::<AssetField>()? {
            if seen & field.bit() != 0 {
                return Err(DeError::custom(DUPLICATE_FIELD_MESSAGE));
            }
            seen |= field.bit();
            match field {
                AssetField::Id => id = Some(map.next_value()?),
                AssetField::Sha256 => sha256 = Some(map.next_value()?),
                AssetField::MediaClass => media_class = Some(map.next_value()?),
                AssetField::Mime => mime = Some(map.next_value()?),
                AssetField::Width => width = Some(map.next_value()?),
                AssetField::Height => height = Some(map.next_value()?),
                AssetField::Licence => licence = Some(map.next_value()?),
                AssetField::Provenance => provenance = Some(map.next_value()?),
            }
        }
        let required = || format!("{MISSING_FIELD_MESSAGE} a key of {ASSET_FIELDS:?}");
        Ok(WireAsset {
            id: id.ok_or_else(|| DeError::custom(required()))?,
            sha256: sha256.ok_or_else(|| DeError::custom(required()))?,
            media_class: media_class.ok_or_else(|| DeError::custom(required()))?,
            mime: mime.ok_or_else(|| DeError::custom(required()))?,
            width: width.ok_or_else(|| DeError::custom(required()))?,
            height: height.ok_or_else(|| DeError::custom(required()))?,
            licence: licence.ok_or_else(|| DeError::custom(required()))?,
            provenance: provenance.ok_or_else(|| DeError::custom(required()))?,
        })
    }
}

/// The parsed wire shape of one licence assertion.
#[derive(Debug)]
struct WireLicence {
    expression: String,
    rights_holder: String,
    evidence: String,
}

impl<'de> Deserialize<'de> for WireLicence {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(WireLicenceVisitor)
    }
}

struct WireLicenceVisitor;

impl<'de> Visitor<'de> for WireLicenceVisitor {
    type Value = WireLicence;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a closed licence object")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut seen: u32 = 0;
        let mut expression: Option<String> = None;
        let mut rights_holder: Option<String> = None;
        let mut evidence: Option<String> = None;
        while let Some(field) = map.next_key::<LicenceField>()? {
            if seen & field.bit() != 0 {
                return Err(DeError::custom(DUPLICATE_FIELD_MESSAGE));
            }
            seen |= field.bit();
            match field {
                LicenceField::Expression => expression = Some(map.next_value()?),
                LicenceField::RightsHolder => rights_holder = Some(map.next_value()?),
                LicenceField::Evidence => evidence = Some(map.next_value()?),
            }
        }
        let required = || format!("{MISSING_FIELD_MESSAGE} a key of {LICENCE_FIELDS:?}");
        Ok(WireLicence {
            expression: expression.ok_or_else(|| DeError::custom(required()))?,
            rights_holder: rights_holder.ok_or_else(|| DeError::custom(required()))?,
            evidence: evidence.ok_or_else(|| DeError::custom(required()))?,
        })
    }
}

/// The parsed wire shape of one provenance assertion.
#[derive(Debug)]
struct WireProvenance {
    source: String,
    evidence: String,
}

impl<'de> Deserialize<'de> for WireProvenance {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(WireProvenanceVisitor)
    }
}

struct WireProvenanceVisitor;

impl<'de> Visitor<'de> for WireProvenanceVisitor {
    type Value = WireProvenance;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a closed provenance object")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut seen: u32 = 0;
        let mut source: Option<String> = None;
        let mut evidence: Option<String> = None;
        while let Some(field) = map.next_key::<ProvenanceField>()? {
            if seen & field.bit() != 0 {
                return Err(DeError::custom(DUPLICATE_FIELD_MESSAGE));
            }
            seen |= field.bit();
            match field {
                ProvenanceField::Source => source = Some(map.next_value()?),
                ProvenanceField::Evidence => evidence = Some(map.next_value()?),
            }
        }
        let required = || format!("{MISSING_FIELD_MESSAGE} a key of {PROVENANCE_FIELDS:?}");
        Ok(WireProvenance {
            source: source.ok_or_else(|| DeError::custom(required()))?,
            evidence: evidence.ok_or_else(|| DeError::custom(required()))?,
        })
    }
}

/// Classifies one deserialisation failure into the closed refusal taxonomy.
fn classify(error: &serde_json::Error) -> PlatformCatalogueRefusalV1 {
    match error.classify() {
        Category::Data => {
            let message = error.to_string();
            if message.starts_with(DUPLICATE_FIELD_MESSAGE) {
                PlatformCatalogueRefusalV1::DuplicateField
            } else if message.starts_with(UNKNOWN_FIELD_MESSAGE) {
                PlatformCatalogueRefusalV1::UnknownField
            } else {
                PlatformCatalogueRefusalV1::InvalidSyntax
            }
        }
        _ => PlatformCatalogueRefusalV1::InvalidSyntax,
    }
}

/// Parses one bounded catalogue document against an **independent** trusted selected version.
///
/// # Errors
///
/// Returns a closed [`PlatformCatalogueRefusalV1`]. Nothing partial is ever returned: one bad entry
/// refuses the whole document, and the refusal carries no asset path, licence text or holder.
pub fn parse_platform_asset_catalogue_v1(
    bytes: &[u8],
    selected_version: &str,
) -> Result<PlatformAssetCatalogueV1, PlatformCatalogueRefusalV1> {
    use PlatformCatalogueRefusalV1 as Refusal;

    if bytes.is_empty() || bytes.len() > MAX_PLATFORM_CATALOGUE_BYTES_V1 {
        return Err(Refusal::InvalidSyntax);
    }
    // A UTF-8 byte-order mark is refused explicitly rather than handed to the JSON parser.
    if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        return Err(Refusal::InvalidSyntax);
    }
    if std::str::from_utf8(bytes).is_err() {
        return Err(Refusal::InvalidSyntax);
    }
    let mut deserializer = serde_json::Deserializer::from_slice(bytes);
    let wire = WireCatalogue::deserialize(&mut deserializer).map_err(|error| classify(&error))?;
    // One document and nothing else: `end()` is the deterministic trailing-data check.
    deserializer.end().map_err(|_| Refusal::InvalidSyntax)?;
    if wire.platform_asset_library_version != selected_version {
        return Err(Refusal::VersionMismatch);
    }
    if wire.assets.len() > MAX_PLATFORM_CATALOGUE_ENTRIES_V1 {
        return Err(Refusal::TooManyEntries);
    }
    let mut entries = Vec::with_capacity(wire.assets.len());
    for asset in wire.assets {
        let licence = PlatformAssetLicenceV1::new(
            &asset.licence.expression,
            &asset.licence.rights_holder,
            &asset.licence.evidence,
        )?;
        let provenance =
            PlatformAssetProvenanceV1::new(&asset.provenance.source, &asset.provenance.evidence)?;
        entries.push(PlatformAssetCatalogueEntryV1::new(
            &PlatformAssetDeclarationV1::of(
                &asset.id,
                &asset.sha256,
                &asset.media_class,
                &asset.mime,
                asset.width,
                asset.height,
            ),
            licence,
            provenance,
        )?);
    }
    PlatformAssetCatalogueV1::new(selected_version, entries)
}
