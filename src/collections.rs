//! Generic container types that have nothing to do with graphs or categories.

use std::{
    array,
    fmt::{Debug, Display},
    iter::Chain,
    slice::Iter,
    vec,
};

use crate::DebugWithDisplay;

pub trait Replace<T> {
    type With<U>: Replace<U>;
    fn read(&self) -> &T;
    /// Self contains some value of type T. This method replaces that item of
    /// type T with a new value of a *different* type, R. The returned type
    /// is a variant of Self containing the new type.
    fn replace<R>(self, item: R) -> (Self::With<R>, T);
}

/// A Vec that is guaranteed not to be empty
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SomeVec<T> {
    start: [T; 1],
    rest: Vec<T>,
}

impl<T> SomeVec<T> {
    pub fn to_vec(mut self) -> Vec<T> {
        let [fst] = self.start;
        self.rest.insert(0, fst);
        self.rest
    }

    pub fn first(&self) -> &T {
        &self.start[0]
    }

    pub fn destruct(self) -> (T, Vec<T>) {
        let [first] = self.start;
        (first, self.rest)
    }

    pub fn last(&self) -> &T {
        if self.rest.is_empty() {
            &self.start[0]
        } else {
            &self.rest[self.rest.len() - 1]
        }
    }

    pub fn len(&self) -> usize {
        1 + self.rest.len()
    }

    pub const fn is_empty(&self) -> bool {
        false
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index == 0 {
            Some(&self.start[0])
        } else {
            self.rest.get(index - 1)
        }
    }

    pub fn iter(&self) -> Chain<Iter<T>, Iter<T>> {
        self.start.iter().chain(self.rest.iter())
    }

    pub fn iter_rest(&self) -> Iter<T> {
        self.rest.iter()
    }
}

impl<T> IntoIterator for SomeVec<T> {
    type Item = T;

    type IntoIter = Chain<array::IntoIter<T, 1>, vec::IntoIter<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.start.into_iter().chain(self.rest.into_iter())
    }
}

impl<T> TryFrom<Vec<T>> for SomeVec<T> {
    type Error = ();

    fn try_from(mut value: Vec<T>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(());
        }
        let first = value.remove(0);
        Ok(Self {
            start: [first],
            rest: value,
        })
    }
}

impl<T: Debug> Debug for SomeVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(&self.start)
            .entries(&self.rest)
            .finish()
    }
}

impl<T: Display> Display for SomeVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.start.iter().map(DebugWithDisplay))
            .entries(self.rest.iter().map(DebugWithDisplay))
            .finish()
    }
}
