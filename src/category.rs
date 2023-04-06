use std::{
    collections::{
        hash_map::{self, Entry},
        hash_set, HashMap, HashSet,
    },
    fmt::{Debug, Display},
    hash::Hash, marker::PhantomData,
};
use thiserror::Error;

use crate::{
    morphism::{Morphism, MorphismMeta},
    object::Object, cost::Float,
};

pub trait Key: Eq + Hash + Debug + Clone {}
impl<K: Eq + Hash + Debug + Clone> Key for K {}

#[derive(Clone, Debug)]
pub struct Category<Id, M, Size=Float, Cost=Float>
where
    Id: Key,
    M: MorphismMeta<Size, Cost>,
    Size: Clone, //todo size where?
{
    objects: HashMap<Id, Object<Id, Size>>,
    morphisms: HashSet<Morphism<Id, M, Size, Cost>>,
    outbound: HashMap<Id, Vec<Morphism<Id, M, Size, Cost>>>,
    _phantom: PhantomData<Cost>,
}


impl<Id, M, Size, Cost> Category<Id, M, Size, Cost>
where
    Id: Key,
    M: MorphismMeta<Size, Cost>,
    Size: Clone, //todo size where?
{
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            morphisms: HashSet::new(),
            outbound: HashMap::new(),
            _phantom: PhantomData,
        }
    }

    pub fn of(
        objects: impl IntoIterator<Item = Object<Id, Size>>,
        morphisms: impl IntoIterator<Item = Morphism<Id, M, Size, Cost>>,
    ) -> Result<Self, CategoryError> {
        let mut new = Self::new();
        new.add_objects(objects)?;
        new.add_morphisms(morphisms)?;

        Ok(new)
    }

    pub fn add_objects(
        &mut self,
        objects: impl IntoIterator<Item = Object<Id, Size>>,
    ) -> Result<(), CategoryError> {
        for object in objects {
            self.add_object(object)?;
        }
        Ok(())
    }

    pub fn add_morphisms(
        &mut self,
        morphisms: impl IntoIterator<Item = Morphism<Id, M, Size, Cost>>,
    ) -> Result<(), CategoryError> {
        for morphism in morphisms {
            self.add_morphism(morphism)?;
        }
        Ok(())
    }

    pub fn add_object(&mut self, object: Object<Id, Size>) -> Result<(), CategoryError> {
        let id = object.id.clone();
        match self.objects.entry(id.clone()) {
            Entry::Occupied(_) => return Err(AlreadyInserted),
            Entry::Vacant(x) => {
                x.insert(object);
            }
        };
        if self.outbound.insert(id, Vec::new()).is_some() {
            unreachable!("Category has a bug. This entry should have been empty.")
        }

        Ok(())
    }

    pub fn verify_morphism(&self, morphism: &Morphism<Id, M, Size, Cost>) -> Result<(), CategoryError> {
        if self.morphisms.contains(morphism) {
            return Err(AlreadyInserted);
        }
        let mut missing = vec![];
        if self.objects.get(&morphism.source).is_none() {
            missing.push(format!("source: {:?}", morphism.source));
        }
        if self.objects.get(&morphism.target).is_none() {
            missing.push(format!("target: {:?}", morphism.target));
        }
        if missing.len() > 0 {
            return Err(MissingNodes(missing));
        }

        Ok(())
    }

    pub fn add_morphism(&mut self, morphism: Morphism<Id, M, Size, Cost>) -> Result<(), CategoryError> {
        self.verify_morphism(&morphism)?;
        self.morphisms.insert(morphism.clone());
        match self.outbound.entry(morphism.source.clone()) {
            Entry::Occupied(mut x) => x.get_mut().push(morphism),
            Entry::Vacant(x) => x.insert(vec![]).push(morphism),
        }

        Ok(())
    }

    pub fn get_outbound(&self, id: &Id) -> Option<&Vec<Morphism<Id, M, Size, Cost>>> {
        self.outbound.get(id)
    }

    pub fn get_object(&self, id: &Id) -> Option<&Object<Id, Size>> {
        self.objects.get(id)
    }

    pub fn objects(&self) -> hash_map::Iter<Id, Object<Id, Size>> {
        self.objects.iter()
    }

    pub fn morphisms(&self) -> hash_set::Iter<Morphism<Id, M, Size, Cost>> {
        self.morphisms.iter()
    }

    pub fn destruct(
        self,
    ) -> (
        HashMap<Id, Object<Id, Size>>,
        HashSet<Morphism<Id, M, Size, Cost>>,
        HashMap<Id, Vec<Morphism<Id, M, Size, Cost>>>,
    ) {
        (self.objects, self.morphisms, self.outbound)
    }
}


#[derive(Error, Debug)]
pub enum CategoryError {
    #[error("There is already a record with this item's key")]
    AlreadyInserted,
    #[error("The nodes were expected but not found: {0:?}")]
    MissingNodes(Vec<String>),
}
use CategoryError::*;
