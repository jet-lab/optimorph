//! Generic container types that have nothing to do with graphs or categories.

use std::{array, iter::Chain, slice::Iter, vec};

pub trait Replace<T> {
    type With<U>: Replace<U>;
    fn read(&self) -> &T;
    /// Self contains some value of type T. This method replaces that item of
    /// type T with a new value of a *different* type, R. The returned type
    /// is a variant of Self containing the new type.
    fn replace<R>(self, item: R) -> (Self::With<R>, T);
}

/// A Vec that is guaranteed not to be empty
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
        if self.rest.len() == 0 {
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

    pub fn into_iter(self) -> Chain<array::IntoIter<T, 1>, vec::IntoIter<T>> {
        self.start.into_iter().chain(self.rest.into_iter())
    }
}

impl<T> TryFrom<Vec<T>> for SomeVec<T> {
    type Error = ();

    fn try_from(mut value: Vec<T>) -> Result<Self, Self::Error> {
        if value.len() == 0 {
            return Err(());
        }
        let first = value.remove(0);
        Ok(Self {
            start: [first],
            rest: value,
        })
    }
}
