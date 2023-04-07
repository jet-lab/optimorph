use std::{
    collections::{hash_map::Entry, hash_set, HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
    marker::PhantomData,
    rc::Rc,
};
use thiserror::Error;

use crate::{
    cost::Float,
    morphism::{Morphism, MorphismMeta},
    object::HasId,
};

pub trait Key: Eq + Hash + Debug + Clone {}
impl<K: Eq + Hash + Debug + Clone> Key for K {}

#[derive(Debug)]
pub struct Category<Id, M, Object = Id, Size = Float, Cost = Float>
where
    Id: Key,
    Object: HasId<Id>,
    M: MorphismMeta<Size, Cost>,
    Size: Clone, //todo size where?
{
    objects: HashMap<Id, Rc<Object>>,
    morphisms: HashSet<Morphism<Id, M, Size, Cost>>,
    outbound: HashMap<Id, Vec<Morphism<Id, M, Size, Cost>>>,
    _phantom: PhantomData<Cost>,
}

impl<Id, M, Object, Size, Cost> Clone for Category<Id, M, Object, Size, Cost>
where
    Id: Key,
    Object: HasId<Id>,
    M: MorphismMeta<Size, Cost>,
    Size: Clone, //todo size where?
{
    fn clone(&self) -> Self {
        Self {
            objects: self.objects.clone(),
            morphisms: self.morphisms.clone(),
            outbound: self.outbound.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<Id, M, Size, Cost> From<Vec<Morphism<Id, M, Size, Cost>>> for Category<Id, M, Id, Size, Cost>
where
    Id: Key + HasId<Id>,
    M: MorphismMeta<Size, Cost>,
    Size: Clone, //todo size where?
{
    fn from(morphisms: Vec<Morphism<Id, M, Size, Cost>>) -> Self {
        let mut new = Self::new();
        for morphism in morphisms {
            new.objects
                .insert(morphism.source.clone(), Rc::new(morphism.source.clone()));
            new.objects
                .insert(morphism.target.clone(), Rc::new(morphism.target.clone()));
            new.add_morphism_unchecked(morphism);
        }
        new
    }
}

impl<Id, M, Object, Size, Cost> Category<Id, M, Object, Size, Cost>
where
    Id: Key,
    Object: HasId<Id>,
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
        objects: impl IntoIterator<Item = Object>,
        morphisms: impl IntoIterator<Item = Morphism<Id, M, Size, Cost>>,
    ) -> Result<Self, CategoryError> {
        let mut new = Self::new();
        new.add_objects(objects)?;
        new.add_morphisms(morphisms)?;

        Ok(new)
    }

    pub fn add_objects(
        &mut self,
        objects: impl IntoIterator<Item = Object>,
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

    pub fn add_object(&mut self, object: Object) -> Result<(), CategoryError> {
        let id = object.id();
        match self.objects.entry(id.clone()) {
            Entry::Occupied(_) => return Err(AlreadyInserted),
            Entry::Vacant(x) => {
                x.insert(Rc::new(object));
            }
        };
        if self.outbound.insert(id, Vec::new()).is_some() {
            unreachable!("Category has a bug. This entry should have been empty.")
        }

        Ok(())
    }

    pub fn verify_morphism(
        &self,
        morphism: &Morphism<Id, M, Size, Cost>,
    ) -> Result<(), CategoryError> {
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

    pub fn add_morphism(
        &mut self,
        morphism: Morphism<Id, M, Size, Cost>,
    ) -> Result<(), CategoryError> {
        self.verify_morphism(&morphism)?;
        self.add_morphism_unchecked(morphism);
        Ok(())
    }

    fn add_morphism_unchecked(&mut self, morphism: Morphism<Id, M, Size, Cost>) {
        self.morphisms.insert(morphism.clone());
        match self.outbound.entry(morphism.source.clone()) {
            Entry::Occupied(mut x) => x.get_mut().push(morphism),
            Entry::Vacant(x) => x.insert(vec![]).push(morphism),
        }
    }

    pub fn get_outbound(&self, id: &Id) -> Option<&Vec<Morphism<Id, M, Size, Cost>>> {
        self.outbound.get(id)
    }

    pub fn get_object(&self, id: &Id) -> Option<Rc<Object>> {
        self.objects.get(id).map(Clone::clone)
    }

    // pub fn objects(&self) -> hash_map::Iter<Id, Rc<Object>> {
    //     self.objects.iter().map(Clone::clone)
    // }

    pub fn morphisms(&self) -> hash_set::Iter<Morphism<Id, M, Size, Cost>> {
        self.morphisms.iter()
    }

    pub fn destruct(
        self,
    ) -> (
        HashMap<Id, Rc<Object>>,
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
