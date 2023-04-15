use cosmwasm_std::{Addr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// The response of both NftINfo and PrivateMetadata queries are Metadata
//

/// token metadata
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Eq, Debug, Default)]
pub struct Metadata {
    /// optional uri for off-chain metadata.  This should be prefixed with `http://`, `https://`, `ipfs://`, or
    /// `ar://`.  Only use this if you are not using `extension`
    pub token_uri: Option<String>,
    /// optional on-chain metadata.  Only use this if you are not using `token_uri`
    pub extension: Option<Extension>,
}

/// metadata extension
/// You can add any metadata fields you need here.  These fields are based on
/// <https://docs.opensea.io/docs/metadata-standards> and are the metadata fields that
/// Stashh uses for robust NFT display.  Urls should be prefixed with `http://`, `https://`, `ipfs://`, or
/// `ar://`
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Eq, Debug, Default)]
pub struct Extension {
    /// url to the image
    pub image: Option<String>,
    /// raw SVG image data (not recommended). Only use this if you're not including the image parameter
    pub image_data: Option<String>,
    /// url to allow users to view the item on your site
    pub external_url: Option<String>,
    /// item description
    pub description: Option<String>,
    /// name of the item
    pub name: Option<String>,
    /// item attributes
    pub attributes: Option<Vec<Trait>>,
    /// background color represented as a six-character hexadecimal without a pre-pended #
    pub background_color: Option<String>,
    /// url to a multimedia attachment
    pub animation_url: Option<String>,
    /// url to a YouTube video
    pub youtube_url: Option<String>,
    /// media files as specified on Stashh that allows for basic authenticatiion and decryption keys.
    /// Most of the above is used for bridging public eth NFT metadata easily, whereas `media` will be used
    /// when minting NFTs on Stashh
    pub media: Option<Vec<MediaFile>>,
    /// a select list of trait_types that are in the private metadata.  This will only ever be used
    /// in public metadata
    pub protected_attributes: Option<Vec<String>>,
    /// NFP data.
    pub nfp: Option<Nfp>,
    /// raw data stored on chain.
    pub raw_data: Option<Vec<RawData>>,
    /// genome
    pub genome: Option<Uint128>,
}

/// attribute trait
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Eq, Debug, Default)]
pub struct Trait {
    /// indicates how a trait should be displayed
    pub display_type: Option<String>,
    /// name of the trait
    pub trait_type: Option<String>,
    /// trait value
    pub value: String,
    /// optional max value for numerical traits
    pub max_value: Option<String>,
}

/// media file
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Eq, Debug, Default)]
pub struct MediaFile {
    /// file type
    /// Stashh currently uses: "image", "video", "audio", "text", "font", "application"
    pub file_type: Option<String>,
    /// file extension
    pub extension: Option<String>,
    /// authentication information
    pub authentication: Option<Authentication>,
    /// url to the file.  Urls should be prefixed with `http://`, `https://`, `ipfs://`, or `ar://`
    pub url: String,
}

/// media file authentication
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Eq, Debug, Default)]
pub struct Authentication {
    /// either a decryption key for encrypted files or a password for basic authentication
    pub key: Option<String>,
    /// username used in basic authentication
    pub user: Option<String>,
}

/// raw data on chain
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug, Default, Eq)]
pub struct RawData {
    /// data bytes in base64
    pub data: String,
    /// encoding of the media data (eg. svg, svgz, png, webp)
    pub encoding: Option<String>,
    /// optional filename
    pub name: Option<String>,
}

/// NFP data
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug, Default, Eq)]
pub struct Nfp {
    /// NFP data bytes and encoding
    pub data: Option<Vec<RawData>>,
    /// program reference in NFP package manager
    pub code: Option<PackageReference>,
    /// off-chain media files for NFP
    pub media: Option<Vec<MediaFile>>,
}

/// Stored NFP package reference
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug, Eq)]
pub struct PackageReference {
    /// package manager contract
    pub package_manager: Addr,
    /// program id in package manager
    pub id: String,
    /// version in package manager
    pub version: Option<String>,
}