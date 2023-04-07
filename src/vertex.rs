use std::hash::Hash;

use pathfinding::num_traits::Zero;

use crate::{
    category::HasId,
    category::{Category, Key},
    morphism::ApplyMorphism,
    morphism::{Morphism, MorphismMeta},
};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Vertex<Id, M, Size>
where
    Id: Key,
    M: MorphismMeta,
    Size: Clone,
{
    Object { id: Id, size: Size },
    Morphism { inner: Morphism<Id, M>, input: Size },
}

impl<Id, M, Size> Default for Vertex<Id, M, Size>
where
    Id: Key,
    M: MorphismMeta,
    Size: Clone,
{
    fn default() -> Self {
        panic!("do not use this. it makes no sense. this is only implemented to satisfy annoying trait bounds that are not actually used");
    }
}

impl<Id, M, Size> Clone for Vertex<Id, M, Size>
where
    Id: Key,
    M: MorphismMeta,
    Size: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Self::Object { id, size } => Self::Object {
                id: id.clone(),
                size: size.clone(),
            },
            Self::Morphism { inner, input } => Self::Morphism {
                inner: inner.clone(),
                input: input.clone(),
            },
        }
    }
}

impl<Id, M, Size> Vertex<Id, M, Size>
where
    Id: Key,
    M: MorphismMeta,
    Size: Clone,
{
    pub fn successors<Object: HasId<Id>, Cost: Zero>(
        &self,
        category: &Category<Id, M, Object>,
    ) -> Vec<(Vertex<Id, M, Size>, Cost)>
    where
        M: ApplyMorphism<Size, Cost>,
    {
        match self {
            Vertex::Object { id, size } => category
                .get_outbound(id)
                .expect("The object id was not found in the category") //todo
                .iter()
                .map(|m| {
                    (
                        Vertex::Morphism {
                            inner: m.clone(),
                            input: size.clone(),
                        },
                        Cost::zero(),
                    )
                })
                .collect(),
            Vertex::Morphism {
                inner,
                input: input_size,
            } => inner.successors(category, input_size.clone()),
        }
    }

    pub fn is_object_with_id(&self, id: &Id) -> bool {
        match self {
            Vertex::Object { id: inner, .. } => &inner.clone() == id,
            Vertex::Morphism { .. } => false,
        }
    }
}
