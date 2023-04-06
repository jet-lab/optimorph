use std::{fmt::Debug, hash::Hash, rc::Rc};

use pathfinding::num_traits::Zero;

use crate::{
    category::{Category, Key},
    cost::{ApplyMorphism, Float},
    morphism::{Morphism, MorphismMeta},
    vertex::Vertex,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Object<Id, Size=Float>
where
    Id: Key,
{
    pub id: Id,
    pub size: Size,
}

impl<Id, Size> Object<Id, Size>
where
    Id: Key,
{
    pub fn new(id: Id, size: Size) -> Self {
        Self { id, size }
    }

    pub fn with_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
}

impl<Id, Size> Object<Id, Size>
where
    Id: Key,
    Size: Clone, //todo size where?
{
    pub fn successors<M: MorphismMeta<Size, Cost>, Cost: Zero>(
        &self,
        category: &Category<Id, M, Size, Cost>,
    ) -> Vec<(Vertex<Id, M, Size, Cost>, Cost)> {
        category
            .get_outbound(&self.id)
            .unwrap() //todo
            .iter()
            .map(|m| {
                (
                    Vertex::Morphism {
                        inner: m.clone(),
                        input_size: self.size.clone(),
                    },
                    Cost::zero(),
                )
            })
            .collect()
    }
}
