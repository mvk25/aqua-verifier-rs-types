//! # Branch - revisions with the same `genesis_hash`

use crate::models::hash::Hash;

/// Represents a branch - revisions with the same `genesis_hash`
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Branch<T> {
    pub metadata: T,
    pub hashes: Vec<Hash>,
}
