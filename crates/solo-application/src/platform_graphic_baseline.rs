//! `SOLO-WO-118 R17` — the inward verified-approved-catalogue binding for the graphic baseline.
//!
//! The domain's root-layer graphic registration may not trust a caller's word that an asset is
//! approved. This module is the one place that converts *transferred bytes plus accepted confined
//! read evidence* into the domain's [`ApprovedPlatformGraphicBindingV1`]:
//!
//! * the selected library version must be the accepted one;
//! * the catalogue digest is recomputed here from the bytes the reader transferred, never taken
//!   from the model or from the port;
//! * the catalogue must enumerate the accepted asset count;
//! * an asset enters the binding only when one accepted confined read produced its bytes **and**
//!   that evidence's own digest equals the catalogue's declared digest for that exact identifier;
//! * a read set that carries an identifier the catalogue does not declare is refused.
//!
//! Holding a binding proves identity and membership only. It grants no publication authority, no
//! media validity, no route and no release.

use solo_domain::{
    ApprovedPlatformAssetV1, ApprovedPlatformGraphicBindingV1, PlatformAssetMediaClassV1,
};

use crate::media_worker::sha256_media_worker_bytes_v1;
use crate::platform_asset_catalogue::PlatformAssetCatalogueV1;
use crate::platform_asset_read::{
    APPROVED_PLATFORM_ASSET_COUNT_V1, APPROVED_PLATFORM_CATALOGUE_SHA256_V1,
    APPROVED_PLATFORM_LIBRARY_VERSION_V1, PlatformAssetSourceSetV1,
};

/// A successful binding grants **no** publication authority.
pub const PLATFORM_GRAPHIC_BASELINE_GRANTS_PUBLICATION_AUTHORITY_V1: bool = false;

/// Why a graphic-baseline binding was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlatformGraphicBaselineRefusalKindV1 {
    /// The selected library version is not the accepted one.
    UnselectedVersion,
    /// The transferred catalogue bytes are not the accepted document.
    CatalogueUnapproved,
    /// The catalogue does not enumerate the accepted asset count.
    CatalogueCountMismatch,
    /// No accepted confined read contributed any approved source.
    NoAcceptedSource,
    /// A read evidence record disagrees with the catalogue for its identifier.
    SourceEvidenceMismatch,
    /// The read set carries an identifier the catalogue never declared.
    UndeclaredSource,
    /// A catalogue identifier is not convertible to a registered media class.
    UnsupportedMediaClass,
}

impl PlatformGraphicBaselineRefusalKindV1 {
    /// The stable, safe detail token.
    #[must_use]
    pub const fn detail(self) -> &'static str {
        match self {
            Self::UnselectedVersion => "unselected-version",
            Self::CatalogueUnapproved => "catalogue-unapproved",
            Self::CatalogueCountMismatch => "catalogue-count-mismatch",
            Self::NoAcceptedSource => "no-accepted-source",
            Self::SourceEvidenceMismatch => "source-evidence-mismatch",
            Self::UndeclaredSource => "undeclared-source",
            Self::UnsupportedMediaClass => "unsupported-media-class",
        }
    }
}

/// One refused graphic-baseline binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformGraphicBaselineRefusalV1 {
    kind: PlatformGraphicBaselineRefusalKindV1,
}

impl PlatformGraphicBaselineRefusalV1 {
    /// Builds one refusal.
    #[must_use]
    pub const fn of(kind: PlatformGraphicBaselineRefusalKindV1) -> Self {
        Self { kind }
    }

    /// The closed refusal reason.
    #[must_use]
    pub const fn kind(self) -> PlatformGraphicBaselineRefusalKindV1 {
        self.kind
    }

    /// The stable, safe detail token.
    #[must_use]
    pub const fn detail(self) -> &'static str {
        self.kind.detail()
    }
}

impl core::fmt::Display for PlatformGraphicBaselineRefusalV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            formatter,
            "platform-graphic-baseline-refused: {}",
            self.detail()
        )
    }
}

impl std::error::Error for PlatformGraphicBaselineRefusalV1 {}

/// Builds the inward approved binding from verified bytes and accepted read evidence.
///
/// # Errors
///
/// Returns a [`PlatformGraphicBaselineRefusalV1`] when the version is not the accepted one, the
/// transferred catalogue bytes do not match the accepted pin, the catalogue count is wrong, no
/// accepted source was read, a read disagrees with the catalogue, the read set carries an
/// undeclared identifier, or a declared identifier has no registered media class.
pub fn approved_platform_graphic_binding_v1(
    selected_version: &str,
    catalogue_bytes: &[u8],
    catalogue: &PlatformAssetCatalogueV1,
    read: &PlatformAssetSourceSetV1,
) -> Result<ApprovedPlatformGraphicBindingV1, PlatformGraphicBaselineRefusalV1> {
    use PlatformGraphicBaselineRefusalKindV1 as Kind;

    if selected_version != APPROVED_PLATFORM_LIBRARY_VERSION_V1 {
        return Err(PlatformGraphicBaselineRefusalV1::of(
            Kind::UnselectedVersion,
        ));
    }
    // The pin is recomputed from the transferred bytes here, never trusted from the caller.
    if sha256_media_worker_bytes_v1(catalogue_bytes).to_hex()
        != APPROVED_PLATFORM_CATALOGUE_SHA256_V1
    {
        return Err(PlatformGraphicBaselineRefusalV1::of(
            Kind::CatalogueUnapproved,
        ));
    }
    if catalogue.len() != APPROVED_PLATFORM_ASSET_COUNT_V1 {
        return Err(PlatformGraphicBaselineRefusalV1::of(
            Kind::CatalogueCountMismatch,
        ));
    }
    // A read set may never contribute an identifier the approved catalogue does not declare.
    for evidence in read.evidence() {
        let declared = catalogue
            .entries()
            .iter()
            .any(|entry| entry.id() == evidence.asset_id());
        if !declared {
            return Err(PlatformGraphicBaselineRefusalV1::of(Kind::UndeclaredSource));
        }
    }

    let mut assets = Vec::with_capacity(read.len());
    for entry in catalogue.entries() {
        let Some(evidence) = read.exact(entry.id()) else {
            continue;
        };
        if evidence.source_sha256() != entry.sha256().to_hex() {
            return Err(PlatformGraphicBaselineRefusalV1::of(
                Kind::SourceEvidenceMismatch,
            ));
        }
        let extension = entry
            .id()
            .rsplit_once('.')
            .map(|(_, tail)| tail)
            .ok_or_else(|| PlatformGraphicBaselineRefusalV1::of(Kind::UnsupportedMediaClass))?;
        let media_class = PlatformAssetMediaClassV1::from_extension(extension)
            .ok_or_else(|| PlatformGraphicBaselineRefusalV1::of(Kind::UnsupportedMediaClass))?;
        let digest = entry.sha256().to_hex();
        let asset = ApprovedPlatformAssetV1::new(entry.id(), &digest, media_class)
            .map_err(|_| PlatformGraphicBaselineRefusalV1::of(Kind::SourceEvidenceMismatch))?;
        assets.push(asset);
    }
    if assets.is_empty() {
        return Err(PlatformGraphicBaselineRefusalV1::of(Kind::NoAcceptedSource));
    }
    ApprovedPlatformGraphicBindingV1::new(
        APPROVED_PLATFORM_LIBRARY_VERSION_V1,
        APPROVED_PLATFORM_CATALOGUE_SHA256_V1,
        assets,
    )
    .map_err(|_| PlatformGraphicBaselineRefusalV1::of(Kind::SourceEvidenceMismatch))
}
