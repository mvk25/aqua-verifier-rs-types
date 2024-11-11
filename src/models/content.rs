// use std::collections::BTreeMap;
use crate::models::base64::Base64;
use crate::models::hash::Hash;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct RevisionWitnessInput{
  pub  filename: String,
  pub  tx_hash: String,
  pub  wallet_address: String,
  pub network: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct RevisionContentSignature {
  pub  filename: String,
  pub  publickey: String,
  pub  signature: String,
  pub  wallet_address: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
/// The user visible content
pub struct RevisionContent {
    /// File in the revision. See: [`FileContent`]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<FileContent>,
    /// (key, value) map for the content `revision` -> `content`->`content` in JSON file.\
    /// Keys (i.e. `main`, `transclusion_hashes`) need to be sorted, thus using a [`BTreeMap`]
    pub content: RevisionContentContent, // BTreeMap<String, String>,
    /// Value of `content_hash` key of a revision in JSON file
    pub content_hash: Hash,
}
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
/// The user visible content
pub struct RevisionContentContent {
    pub file_hash: Hash,
}

/// The content of the file
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FileContent {
    /// The content of the file in Base64 encoding
    pub data: Base64,
    pub filename: String,
    pub size: u32,
    pub comment: String,
}
