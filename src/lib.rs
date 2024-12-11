//! # Aqua verifier types
//!
//! This crate provides modular components for working with hashes, metadata, signatures, and more.
//!
//! ## Models
//!
//! The primary modules are located under the `models` namespace and include:
//! - `content`
//! - `hash`
//! - `metadata`
//! - `signature`
//! - `witness`
//! - `base64`
//! - `stack_str`
//! - `timestamp`
//! - `public_key`
//! - `tx_hash`
//! - `page_data`
//! - `revision`
//! - `storage`
//! - `branch`

/// Models for working with various data types and functionalities.
pub mod models {
    pub mod content;
    pub mod hash;
    pub mod metadata;
    pub mod signature;
    pub mod witness;
    pub mod base64;
    pub mod stack_str;
    pub mod timestamp;
    pub mod public_key;
    pub mod tx_hash;
    pub mod page_data;
    pub mod revision;
    pub mod storage;
    pub mod branch;

    /// Internal tests for the `models` module.
    #[doc(hidden)]
    pub mod tests;
}

/// Cryptography utilities for hashing and digesting data.
///
/// **Note**: This module is hidden from the documentation as it is not part of the primary API.
#[doc(hidden)]
pub mod crypt {
    pub type Hasher = sha3::Sha3_512;
    pub type Hash = sha3::digest::Output<Hasher>;
    pub use sha3::*;
}

// Other hidden utilities or functions.
// #[doc(hidden)]
// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
