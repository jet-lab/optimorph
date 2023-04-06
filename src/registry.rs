use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    fmt::{Debug, Display},
    hash::Hash,
};
use thiserror::Error;

use crate::{
    errify,
    morphism::{Morphism, MorphismMeta},
    object::Object,
};

pub struct Pointer(u64);

/// SafeRegistry is only safe if this is implemented exhaustively.
pub trait RefersToOthers<K> {
    fn refs_to_others(&self) -> Vec<K>;
}

pub trait Referable<K> {
    fn refer(&self) -> K;
}

pub trait Key: Eq + Hash + Debug + Clone {}
pub trait Value<K: Key>: RefersToOthers<K> + Referable<K> + Debug {}

impl<K: Eq + Hash + Debug + Clone> Key for K {}
impl<K: Key, V: RefersToOthers<K> + Referable<K> + Debug> Value<K> for V {}

/// This may contain references to data that are not contained within. Call
/// `verify` to check integrity and return a SafeHeap upon success.
#[derive(Debug)]
pub struct RegistryBuilder<K: Key, V: Value<K>>(HashMap<K, V>);
impl<K: Key, V: Value<K>> RegistryBuilder<K, V> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// todo require new
    pub fn add(&mut self, item: V) {
        self.0.insert(item.refer(), item);
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.0.get_mut(key)
    }

    pub fn extend(&mut self, items: Vec<V>) {
        for item in items {
            self.add(item);
        }
    }

    pub fn verify(self) -> Result<SafeRegistry<K, V>, FailedVerification<K, V>> {
        let mut missing_referents = vec![];
        for referer in self.0.values() {
            for referent in referer.refs_to_others() {
                if self.0.get(&referent).is_none() {
                    missing_referents.push(referent);
                }
            }
        }
        if missing_referents.len() == 0 {
            Ok(SafeRegistry(self.0))
        } else {
            Err(FailedVerification {
                builder: self,
                missing_referents,
            })
        }
    }
}

/// This is always well formed. None of its internal data contains a reference
/// to data that is not also contained within the registry.
pub struct SafeRegistry<K: Key, V: Value<K>>(HashMap<K, V>);
impl<K: Key, V: Value<K>> SafeRegistry<K, V> {
    pub fn get(&self, id: &K) -> Option<&V> {
        self.0.get(id)
    }

    pub fn to_builder(self) -> RegistryBuilder<K, V> {
        RegistryBuilder(self.0)
    }
}

#[derive(Debug)]
pub struct FailedVerification<K: Key, V: Value<K>> {
    pub builder: RegistryBuilder<K, V>,
    pub missing_referents: Vec<K>,
}

impl<K: Key, V: Value<K>> Display for FailedVerification<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Failed to verify registry due to missing referents: {:?}",
            self.missing_referents
        )
    }
}

impl<K: Key, V: Value<K>> std::error::Error for FailedVerification<K, V> {}
