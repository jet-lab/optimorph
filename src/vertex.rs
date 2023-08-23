//! A "Vertex" is a vertex in the graph that is used to represent the category,
//! where all objects and morphisms are vertices in the graph, and the implicit
//! edges in the graph represent the fact that objects and morphisms are
//! connected.

use std::{collections::HashSet, hash::Hash};

use pathfinding::num_traits::Zero;

use crate::{
    category::{Category, HasId, Key, Object},
    impls::Float,
    morphism::ApplyMorphism,
    morphism::Morphism,
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
    {
        match lean {
            LeanVertex::Object { inner: id, size } => Self::Object {
                inner: category.get_object(&id).unwrap().clone(), //todo unwrap
                size,
            },
            LeanVertex::Morphism { inner, input } => Self::Morphism { inner, input },
        }
    }

    pub fn is_object_with_id(&self, id: &Id) -> bool
    where
        Id: Eq,
        Obj: HasId<Id>,
    {
        match self {
            Vertex::Object { inner, .. } => &inner.id() == id,
            Vertex::Morphism { .. } => false,
        }
    }
}

/// Used as a vertex in the underlying graph optimization algorithms. Only
/// refers to an object by its id, to keep things simple and lightweight.
pub(crate) type LeanVertex<Id, M, Size> = Vertex<Id, M, Id, Size>;

impl<Id, M, Size> Default for LeanVertex<Id, M, Size> {
    fn default() -> Self {
        unimplemented!("do not use this. it makes no sense. this is only implemented to satisfy annoying trait bounds that are not actually used");
    }
}

// TODO: consolidate
impl<Id, M, Size> LeanVertex<Id, M, Size> {
    pub fn successors<const NON_NEGATIVE: bool, Obj, Cost: Zero>(
        &self,
        category: &Category<Id, M, Obj>,
    ) -> Vec<(LeanVertex<Id, M, Size>, Cost)>
    where
        Id: Key,
        Obj: HasId<Id>,
        M: ApplyMorphism<Size, Cost, NON_NEGATIVE>,
        M: Clone,
        Size: Clone,
    {
        match self {
            LeanVertex::Object { inner: id, size } => category
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
            } => {
                let (id, size, cost) = inner.successor(category, input_size.clone());
                vec![(LeanVertex::Object { inner: id, size }, cost)]
            }
        }
    }

    /// Avoids cycles by preventing re-visit of any objects.
    pub fn blacklisted_successors<const NON_NEGATIVE: bool, Obj, Cost: Zero>(
        &self,
        category: &Category<Id, M, Obj>,
        blacklist: &mut HashSet<Id>,
    ) -> Vec<(LeanVertex<Id, M, Size>, Cost)>
    where
        Id: Key,
        Obj: HasId<Id>,
        M: ApplyMorphism<Size, Cost, NON_NEGATIVE>,
        M: Clone,
        Size: Clone,
    {
        match self {
            LeanVertex::Object { inner: id, size } => {
                blacklist.insert(id.clone());
                category
                    .get_outbound(id)
                    .expect("The object id was not found in the category") //todo
                    .iter()
                    .filter_map(|m| {
                        if blacklist.contains(&m.target) {
                            None
                        } else {
                            Some((
                                LeanVertex::Morphism {
                                    inner: m.clone(),
                                    input: size.clone(),
                                },
                                Cost::zero(),
                            ))
                        }
                    })
                    .collect()
            }
            LeanVertex::Morphism {
                inner,
                input: input_size,
            } => {
                let (id, size, cost) = inner.successor(category, input_size.clone());
                if blacklist.contains(&id) {
                    vec![]
                } else {
                    vec![(LeanVertex::Object { inner: id, size }, cost)]
                }
            }
        }
    }
}
