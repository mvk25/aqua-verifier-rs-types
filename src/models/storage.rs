//! Defines the `Storage` trait, which specifies an interface for a storage system.


use std::{fmt::Debug, future::Future};
use crate::models::hash::Hash;
use crate::models::revision::Revision;

use crate::models::branch::Branch;

/// Trait defining an interface for a storage system.
/// 
/// This trait provides methods for managing and retrieving contexts, revisions, branches, 
/// and other metadata in an asynchronous manner.
pub trait Storage: Sized {
    /// Type of error stored in all storage operations.
    type Error: std::error::Error + Debug;

    /// Type of context associated with the storage.
    type Context;

    /// Retrieves the context associated with the given `Hash`.
    /// 
    /// # Parameters
    /// - `hash`: The hash identifying the context.
    /// 
    /// # Returns
    /// An asynchronous result containing the context or an error.
    fn get_context(
        &self,
        hash: Hash,
    ) -> impl Future<Output = Result<Self::Context, Self::Error>> + Send;


    /// Stores a revision and its associated context in the storage.
    /// 
    /// # Parameters
    /// - `rev`: The revision to store.
    /// - `context`: The associated context to store.
    /// 
    /// # Returns
    /// An asynchronous result indicating success or failure.
    fn store(
        &self,
        // hash: Hash,
        rev: Revision,
        context: Self::Context,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;


    /// Reads a revision identified by the given `Hash`.
    /// 
    /// # Parameters
    /// - `hash`: The hash identifying the revision.
    /// 
    /// # Returns
    /// An asynchronous result containing the revision or an error.
    fn read(&self, hash: Hash)
        -> impl Future<Output = Result<Revision, Self::Error>> + Send + Sync;

    /// Retrieves a branch associated with the given `Hash`.
    /// 
    /// # Parameters
    /// - `hash`: The hash identifying the branch.
    /// 
    /// # Returns
    /// An asynchronous result containing the branch or an error.
    fn get_branch(
        &self,
        hash: Hash,
    ) -> impl Future<Output = Result<Branch<Self::Context>, Self::Error>> + Send;

    /// Lists all hashes currently stored in the system.
    /// 
    /// # Returns
    /// An asynchronous result containing a vector of hashes or an error.
    fn list(&self) -> impl Future<Output = Result<Vec<Hash>, Self::Error>> + Send;

    /// Registers an update handler to be invoked on storage updates.
    /// 
    /// # Parameters
    /// - `f`: A callback function that takes a `Hash` and a `String` description of the update.
    /// 
    /// # Returns
    /// An asynchronous result containing an `Infallible` or an error.
    fn update_handler<F: Fn(Hash, String) + Send + Sync>(
        &self,
        f: F,
    ) -> impl Future<Output = Result<std::convert::Infallible, Self::Error>> + Send;
}
