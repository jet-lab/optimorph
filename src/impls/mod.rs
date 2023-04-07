//! Provides some basic concrete types that implement the abstract traits in
//! this crate, so clients can easily get started with basic usages.

mod morphism;
mod float;

pub use morphism::*;
pub use float::*;

use crate::category::{Key, HasId};

/// This can be used directly as an object in a category.
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct SimpleObject<K: Key=String>(K);

impl<K: Key> HasId<SimpleObject<K>> for SimpleObject<K> {
    fn id(&self) -> SimpleObject<K> {
        self.clone()
    }
}
