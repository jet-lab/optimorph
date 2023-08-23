use std::{
    collections::{hash_map::Entry, hash_set, HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};
use thiserror::Error;

use crate::{
    impls::SimpleMorphism,
    morphism::{Morphism, MorphismMeta},
};

pub trait Object<Id: Key>: HasId<Id> + Clone {}
impl<Id: Key, T> Object<Id> for T where T: HasId<Id> + Clone {}

/// todo should Id be an associated type?
pub trait HasId<Id> {
    fn id(&self) -> Id;
}

impl<Id: Clone> HasId<Id> for Id {
    fn id(&self) -> Id {
        self.clone()
    }
}

pub trait Key: Eq + Hash + Debug + Clone {}
impl<K: Eq + Hash + Debug + Clone> Key for K {}

#[derive(Clone, Debug)]
pub struct Category<Id = String, M = SimpleMorphism, Obj = Id> {
    objects: HashMap<Id, Obj>,
    morphisms: HashSet<Morphism<Id, M>>,
    outbound: HashMap<Id, Vec<Morphism<Id, M>>>,
}

impl<Id, M> From<Vec<Morphism<Id, M>>> for Category<Id, M, Id>
where
    Id: Key + HasId<Id>,
    M: MorphismMeta,
{
    fn from(morphisms: Vec<Morphism<Id, M>>) -> Self {
        Self::from_morphisms(morphisms)
    }
}

impl<Id, M> Category<Id, M, Id>
where
    Id: Key + HasId<Id>,
    M: MorphismMeta,
{
    // If the object and id are the same type, all you need are morphisms to create the category
    pub fn from_morphisms(morphisms: impl IntoIterator<Item = Morphism<Id, M>>) -> Self {
        let mut new = Self::new();
        for morphism in morphisms {
            new.objects
                .insert(morphism.source.clone(), morphism.source.clone());
            new.objects
                .insert(morphism.target.clone(), morphism.target.clone());
            new.add_morphism_unchecked(morphism);
        }
        new
    }
}

impl<Id, M, Obj> Default for Category<Id, M, Obj>
where
    Id: Key,
    Obj: Object<Id>,
    M: MorphismMeta,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Id, M, Obj> Category<Id, M, Obj> {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            morphisms: HashSet::new(),
            outbound: HashMap::new(),
        }
    }

    pub fn morphisms(&self) -> hash_set::Iter<Morphism<Id, M>> {
        self.morphisms.iter()
    }

    #[allow(clippy::type_complexity)]
    pub fn destruct(
        self,
    ) -> (
        HashMap<Id, Obj>,
        HashSet<Morphism<Id, M>>,
        HashMap<Id, Vec<Morphism<Id, M>>>,
    ) {
        (self.objects, self.morphisms, self.outbound)
    }
}

impl<Id, M, Obj> Category<Id, M, Obj>
where
    Id: Key,
{
    pub fn of(
        objects: impl IntoIterator<Item = Obj>,
        morphisms: impl IntoIterator<Item = Morphism<Id, M>>,
    ) -> Result<Self, CategoryError>
    where
        Obj: HasId<Id>,
        M: MorphismMeta,
    {
        let mut new = Self::new();
        new.add_objects(objects)?;
        new.add_morphisms(morphisms)?;

        Ok(new)
    }

    pub fn add_objects(
        &mut self,
        objects: impl IntoIterator<Item = Obj>,
    ) -> Result<(), CategoryError>
    where
        Obj: HasId<Id>,
    {
        for object in objects {
            self.add_object(object)?;
        }
        Ok(())
    }

    pub fn add_morphisms(
        &mut self,
        morphisms: impl IntoIterator<Item = Morphism<Id, M>>,
    ) -> Result<(), CategoryError>
    where
        M: MorphismMeta,
    {
        for morphism in morphisms {
            self.add_morphism(morphism)?;
        }
        Ok(())
    }

    pub fn add_object(&mut self, object: Obj) -> Result<(), CategoryError>
    where
        Obj: HasId<Id>,
    {
        let id = object.id();
        match self.objects.entry(id.clone()) {
            Entry::Occupied(_) => return Err(ObjectAlreadyInserted(format!("{:?}", object.id()))),
            Entry::Vacant(x) => {
                x.insert(object);
            }
        };
        if self.outbound.insert(id, Vec::new()).is_some() {
            unreachable!("Category has a bug. This entry should have been empty.")
        }

        Ok(())
    }

    pub fn verify_morphism(&self, morphism: &Morphism<Id, M>) -> Result<(), CategoryError>
    where
        M: MorphismMeta,
    {
        if self.morphisms.contains(morphism) {
            return Err(MorphismAlreadyInserted(
                format!("{:?}", morphism.source),
                format!("{:?}", morphism.source),
            ));
        }
        let mut missing = vec![];
        if self.objects.get(&morphism.source).is_none() {
            missing.push(format!("source: {:?}", morphism.source));
        }
        if self.objects.get(&morphism.target).is_none() {
            missing.push(format!("target: {:?}", morphism.target));
        }
        if !missing.is_empty() {
            return Err(MissingObjects(missing));
        }

        Ok(())
    }

    pub fn add_morphism(&mut self, morphism: Morphism<Id, M>) -> Result<(), CategoryError>
    where
        M: MorphismMeta,
    {
        self.verify_morphism(&morphism)?;
        self.add_morphism_unchecked(morphism);
        Ok(())
    }

    fn add_morphism_unchecked(&mut self, morphism: Morphism<Id, M>)
    where
        M: MorphismMeta,
    {
        self.morphisms.insert(morphism.clone());
        match self.outbound.entry(morphism.source.clone()) {
            Entry::Occupied(mut x) => x.get_mut().push(morphism),
            Entry::Vacant(x) => x.insert(vec![]).push(morphism),
        }
    }

    pub fn get_outbound(&self, id: &Id) -> Option<&Vec<Morphism<Id, M>>> {
        self.outbound.get(id)
    }

    pub fn get_object(&self, id: &Id) -> Option<&Obj> {
        self.objects.get(id)
    }

    // pub fn objects(&self) -> hash_map::Iter<Id, Rc<Object>> {
    //     self.objects.iter().map(Clone::clone)
    // }
}

/// todo smarter about debug and string and types etc
#[derive(Error, Debug)]
pub enum CategoryError {
    #[error("There is already an object with this key: {0:?}")]
    ObjectAlreadyInserted(String),
    #[error("This morphism was already inserted. start: {0:?}, end: {1:?}, metadata: cannot be displayed")]
    MorphismAlreadyInserted(String, String),
    #[error("The objects were expected but not found: {0:?}")]
    MissingObjects(Vec<String>),
}
use CategoryError::*;
