use std::{fmt::Debug, hash::Hash, rc::Rc};

use crate::{
    category::{Category, Key},
    cost::{ApplyMorphism, Size},
    morphism::{Morphism, MorphismMeta},
    vertex::Vertex,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Object<Id>
where
    Id: Key,
{
    pub id: Id,
    pub size: Size,
}

impl<Id> Object<Id>
where
    Id: Key,
{
    pub fn new(id: Id, size: Size) -> Self {
        Self { id, size }
    }

    pub fn new_terminal(id: Id, size: Size) -> Self {
        Self { id, size }
    }

    pub fn with_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
}

impl<Id> Object<Id>
where
    Id: Key,
{
    pub fn successors<M: MorphismMeta>(
        &self,
        category: &Category<Id, M>,
    ) -> Vec<(Vertex<Id, M>, Size)> {
        category
            .get_outbound(&self.id)
            .unwrap() //todo
            .iter()
            .map(|m| {
                (
                    Vertex::Morphism {
                        inner: m.clone(),
                        input_size: self.size,
                    },
                    0,
                )
            })
            .collect()
    }
}
