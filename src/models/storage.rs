
use std::{fmt::Debug, future::Future};
use crate::models::hash::Hash;
use crate::models::revision::Revision;

use crate::models::branch::Branch;

pub trait Storage: Sized {
    type Error: std::error::Error + Debug;
    type Context;

    fn get_context(
        &self,
        hash: Hash,
    ) -> impl Future<Output = Result<Self::Context, Self::Error>> + Send;
    fn store(
        &self,
        // hash: Hash,
        rev: Revision,
        context: Self::Context,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
    fn read(&self, hash: Hash)
        -> impl Future<Output = Result<Revision, Self::Error>> + Send + Sync;
    fn get_branch(
        &self,
        hash: Hash,
    ) -> impl Future<Output = Result<Branch<Self::Context>, Self::Error>> + Send;
    fn list(&self) -> impl Future<Output = Result<Vec<Hash>, Self::Error>> + Send;
    fn update_handler<F: Fn(Hash, String) + Send + Sync>(
        &self,
        f: F,
    ) -> impl Future<Output = Result<std::convert::Infallible, Self::Error>> + Send;
}
