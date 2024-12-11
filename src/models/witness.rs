//! This module defines the `MerkleNode` struct, representing a single node in the Merkle tree, 
//! and the `RevisionWitness` struct, which contains the information stored on the blockchain.


use crate::models::hash::Hash;
use crate::models::tx_hash::TxHash;

/// Contains the information stored on the blockchain
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RevisionWitness {
     /// Hash representing the genesis state of the domain snapshot.
     pub domain_snapshot_genesis_hash: Hash,

     /// The root of the Merkle tree for the revision.
     pub merkle_root: Hash,
 
     /// Identifier for the network that recorded this witness.
     pub witness_network: String,
 
     /// Transaction hash where the witness event is recorded.
     pub witness_event_transaction_hash: TxHash,
 
     /// Verification hash for the witness event.
     pub witness_event_verification_hash: Hash,
 
     /// Hash representing the overall witness state.
     pub witness_hash: Hash,
 
     /// A structured proof for verifying Merkle tree membership.
     pub structured_merkle_proof: Vec<MerkleNode>,
}

/// Represents a single node in the Merkle tree.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MerkleNode {
    /// The hash of the left child (leaf or node).
    pub left_leaf: Hash,

    /// The hash of the right child (leaf or node).
    pub right_leaf: Hash,

    /// The resulting hash after combining `left_leaf` and `right_leaf`.
    pub successor: Hash,
}
