//! This module defines structures related to file content, revisions, signatures, and witness inputs.

// use std::collections::BTreeMap;
use crate::models::base64::Base64;
use crate::models::hash::Hash;

/// Input data for a revision during the witness operation.
/// This includes information about the file, transaction, and wallet involved.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct RevisionWitnessInput{
  /// Name of the file involved in the revision
  pub  filename: String,
  /// Hash of the transaction associated with the revision
  pub  tx_hash: String,
  /// Address of the wallet used in the revision
  pub  wallet_address: String,
  /// Network where the transaction occurs(e.g Ethereum,)
  pub network: String,
}


/// New content with revised signature.
/// This structure holds information about the file and its updated signature data.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct RevisionContentSignature {
  /// Name of the file with revised content
  pub  filename: String,
  /// Public Key used for signing the revised content
  pub  publickey: String,
  /// Signature of the revised content
  pub  signature: String,
  /// Address of the wallet associated with the signature
  pub  wallet_address: String,
}

/// The user visible content
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
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


/// A structured representation of revision content data.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct RevisionContentContent {
    /// Hash of the file associated with the revision.
    pub file_hash: Hash,
}


/// The content of the file.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FileContent {
    /// The content of the file in Base64 encoding.
    pub data: Base64,
    /// Name of the file.
    pub filename: String,
    /// Size of the file in bytes.
    pub size: u32,
    /// Optional comment associated with the file content.
    pub comment: String,
}
