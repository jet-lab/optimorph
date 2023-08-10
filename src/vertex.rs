//! A "Vertex" is a vertex in the graph that is used to represent the category,
//! where all objects and morphisms are vertices in the graph, and the implicit
//! edges in the graph represent the fact that objects and morphisms are
//! connected.

use std::hash::Hash;

use pathfinding::num_traits::Zero;

use crate::{
    category::{Category, Key, Object},
    impls::Float,
    morphism::ApplyMorphism,
    morphism::{Morphism, MorphismMeta},
};

/// Comprehensive return type that includes the full object
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Vertex<Id, M, Obj = Id, Size = Float> {
    Object { inner: Obj, size: Size },
    Morphism { inner: Morphism<Id, M>, input: Size },
}

impl<Obj, Id, M, Size> Vertex<Id, M, Obj, Size> {
    pub(crate) fn from(lean: LeanVertex<Id, M, Size>, category: &Category<Id, M, Obj>) -> Self
    where
        Obj: Object<Id>,
        Id: Key,
        M: MorphismMeta,
        Size: Clone,
    {
        match lean {
            LeanVertex::Object { id, size } => Self::Object {
                inner: category.get_object(&id).unwrap().clone(), //todo unwrap
                size,
            },
            LeanVertex::Morphism { inner, input } => Self::Morphism { inner, input },
        }
    }
}

/// Used as a vertex in the underlying graph optimization algorithms. Only
/// refers to an object by its id, to keep things simple and lightweight.
#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) enum LeanVertex<Id, M, Size> {
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
        unimplemented!("do not use this. it makes no sense. this is only implemented to satisfy annoying trait bounds that are not actually used");
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
    pub fn successors<const NON_NEGATIVE: bool, Obj: Object<Id>, Cost: Zero>(
        &self,
        category: &Category<Id, M, Obj>,
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
