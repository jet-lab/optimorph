//! A "Vertex" is a vertex in the graph that is used to represent the category,
//! where all objects and morphisms are vertices in the graph, and the implicit
//! edges in the graph represent the fact that objects and morphisms are
//! connected.

use std::{hash::Hash, rc::Rc};

use pathfinding::num_traits::Zero;

use crate::{
    category::HasId,
    category::{Category, Key},
    impls::Float,
    morphism::ApplyMorphism,
    morphism::{Morphism, MorphismMeta},
};

/// Comprehensive return type that includes the full object
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Vertex<Id, M, Object = Id, Size = Float>
where
    Object: HasId<Id>,
    Id: Key,
    M: MorphismMeta,
{
    Object { inner: Rc<Object>, size: Size },
    Morphism { inner: Morphism<Id, M>, input: Size },
}

impl<Object, Id, M, Size> Vertex<Id, M, Object, Size>
where
    Object: HasId<Id>,
    Id: Key,
    M: MorphismMeta,
    Size: Clone,
{
    pub(crate) fn from(lean: LeanVertex<Id, M, Size>, category: &Category<Id, M, Object>) -> Self {
        match lean {
            LeanVertex::Object { id, size } => Self::Object {
                inner: category.get_object(&id).unwrap(), //todo
                size,
            },
            LeanVertex::Morphism { inner, input } => Self::Morphism { inner, input },
        }
    }
}

/// Used as a vertex in the underlying graph optimization algorithms. Only
/// refers to an object by its id, to keep things simple and lightweight.
#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) enum LeanVertex<Id, M, Size>
where
    Id: Key,
    M: MorphismMeta,
    Size: Clone,
{
    Object { id: Id, size: Size },
    Morphism { inner: Morphism<Id, M>, input: Size },
}

impl<Id, M, Size> Default for LeanVertex<Id, M, Size>
where
    Id: Key,
    M: MorphismMeta,
    Size: Clone,
{
    fn default() -> Self {
        panic!("do not use this. it makes no sense. this is only implemented to satisfy annoying trait bounds that are not actually used");
    }
}

impl<Id, M, Size> Clone for LeanVertex<Id, M, Size>
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

impl<Id, M, Size> LeanVertex<Id, M, Size>
where
    Id: Key,
    M: MorphismMeta,
    Size: Clone,
{
    pub fn successors<const NON_NEGATIVE: bool, Object: HasId<Id>, Cost: Zero>(
        &self,
        category: &Category<Id, M, Object>,
    ) -> Vec<(LeanVertex<Id, M, Size>, Cost)>
    where
        M: ApplyMorphism<Size, Cost, NON_NEGATIVE>,
    {
        match self {
            LeanVertex::Object { id, size } => category
                .get_outbound(id)
                .expect("The object id was not found in the category") //todo
                .iter()
                .map(|m| {
                    (
                        LeanVertex::Morphism {
                            inner: m.clone(),
                            input: size.clone(),
                        },
                        Cost::zero(),
                    )
                })
                .collect(),
            LeanVertex::Morphism {
                inner,
                input: input_size,
            } => inner.successors(category, input_size.clone()),
        }
    }

    pub fn is_object_with_id(&self, id: &Id) -> bool {
        match self {
            LeanVertex::Object { id: inner, .. } => &inner.clone() == id,
            LeanVertex::Morphism { .. } => false,
        }
    }
}
