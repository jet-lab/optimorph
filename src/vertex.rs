use std::{fmt::Debug, hash::Hash, rc::Rc};

use crate::{
    category::{Category, Key},
    cost::{ApplyMorphism, Size},
    morphism::{Morphism, MorphismMeta},
    object::Object,
};

/// There are two layers of graphs.
///
/// 1. The directed graph composed of vertices and edges. This is the layer of
///    abstraction that the shortest-path optimizer works with. Edges point to
///    successor vertices. Edges are anonymous and not represented in the type
///    system. Edges are implied by the ability to get the successors for a
///    particular vertex. An edge can be thought of as an ordered pair of
///    vertices. There can only be two edges between any two vertices - one for
///    each direction.
///
/// 2. The directed graph layered on top of graph 1 is a "category". A category
///    is made of "objects", which are similar to vertices, and "morphisms",
///    which are similar to edges. Objects and morphisms are each represented in
///    layer 1 as vertices. Two objects may be connected by any number of unique
///    morphisms.
///
/// Every object->object relationship in graph #2 is represented by three
/// vertices in graph #1: one for each object and one for the arrow/morphism.
/// This allows us to treat morphisms as first class citizens with uniquely
/// identifying metadata. Two morphisms with the same start and end objects can
/// be distinguished, which is not true of the edges in graph 1.
///
/// Let's say we use a function from the pathfinding crate to calculate the
/// optimal path from object A to object D as (A)-f->(B)-g->(C)-h->(D). The
/// function returns a list of vertices: [A,f,B,g,C,h,D]. We can filter out all
/// the objects from this list, leaving a list of morphisms: [f,g,h]. This can
/// be thought of as the single composite morphism h∘g∘f in (A)--(h∘g∘f)-->(D).
///
/// For cost-based path optimization, we can assign relevant information to each
/// vertex. Objects have sizes. Morphisms have cost functions. A cost function
/// accepts the size of the input object as a parameter. The cost function tells
/// us the cost of using that morphism to transform a particular size of its
/// input object. This information is used by the pathfinding algorithm by
/// assigning the morphism's cost as the weight of the edge from the input
/// object to the morphism. Edges from morphisms to objects have 0 weight.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Vertex<Id, M>
where
    Id: Key,
    M: MorphismMeta,
{
    Object(Object<Id>),
    Morphism {
        inner: Morphism<Id, M>,
        input_size: Size,
    },
}

impl<Id, M> Default for Vertex<Id, M>
where
    Id: Key,
    M: MorphismMeta,
{
    fn default() -> Self {
        panic!("do not use this. it makes no sense. this is only provided to satisfy annoying trait bounds that are not actually used");
    }
}

impl<Id, M> Vertex<Id, M>
where
    Id: Key,
    M: MorphismMeta,
{
    pub fn successors(&self, category: &Category<Id, M>) -> Vec<(Vertex<Id, M>, Size)> {
        match self {
            // Vertex::Null => vec![],
            Vertex::Object(o) => o.successors(category),
            Vertex::Morphism { inner, input_size } => inner.successors(category, *input_size),
        }
    }

    pub fn is_object_with_id(&self, id: &Id) -> bool {
        match self {
            // Vertex::Null => false,
            Vertex::Object(o) => &o.id == id,
            Vertex::Morphism { .. } => false,
        }
    }
}

pub enum KeyOrValue<K, V> {
    Key(K),
    Value(V),
}
