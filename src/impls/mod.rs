//! Provides some basic concrete types that implement the abstract traits in
//! this crate, so clients can easily get started with basic usages.

mod float;
mod morphism;

pub use float::*;
pub use morphism::*;

use crate::category::{HasId, Key};

/// This can be used directly as an object in a category.
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct SimpleObject<K: Key = String>(pub K);

impl<K: Key> HasId<SimpleObject<K>> for SimpleObject<K> {
    fn id(&self) -> SimpleObject<K> {
        self.clone()
    }
}

impl<K: Key> SimpleObject<K> {
    pub fn new(inner: K) -> SimpleObject<K> {
        Self(inner)
    }
}
