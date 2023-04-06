use std::{fmt::Debug, hash::Hash, marker::PhantomData, rc::Rc};

use petgraph::IntoWeightedEdge;

use crate::{
    category::{Category, Key},
    cost::{ApplyMorphism, Float, MorphismOutput},
    object::Object,
    vertex::Vertex,
};

//todo generic over Cost (not Size)
pub trait MorphismMeta<Size, Cost>: Hash + Debug + Clone + Eq + ApplyMorphism<Size, Cost> {}
impl<Size, Cost, M> MorphismMeta<Size, Cost> for M where
    M: Hash + Debug + Clone + Eq + ApplyMorphism<Size, Cost>
{
}

#[derive(Debug)]
pub struct Morphism<Id, M, Size = Float, Cost = Float>
where
    Id: Key,
    M: MorphismMeta<Size, Cost>,
    Size: Clone, //todo size where?
{
    pub source: Id,
    pub target: Id,
    /// This should contain:
    /// - Uniquely identifying information, to distinguish this morphism from
    ///   other morphisms with the same source and target. Eq and Hash values
    ///   must be unique.
    /// - Logic to determine cost and output size from applying the morphism.
    pub metadata: M,
    /// - for static used with one start position, set to start position size
    /// - for static used with any start position, this doesn't work super
    ///   great. set to 1 or the average of all position sizes or whatever i
    ///   dunno
    /// - for dynamic it doesn't matter, you can set to zero
    pub input_size: Size,  //todo size where?
    pub _phantom: PhantomData<Cost>,
}

impl<Id, M, Size, Cost> Clone for Morphism<Id, M, Size, Cost>
where
    Id: Key,
    M: MorphismMeta<Size, Cost>,
    Size: Clone, //todo size where?
{
    fn clone(&self) -> Self {
        Self {
            source: self.source.clone(),
            target: self.target.clone(),
            metadata: self.metadata.clone(),
            input_size: self.input_size.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<Id, M, Size, Cost> Morphism<Id, M, Size, Cost>
where
    Id: Key,
    M: MorphismMeta<Size, Cost>,
    Size: Clone, //todo size where?
{
    /// Needed for `pathfinding`
    pub fn successors(
        &self,
        category: &Category<Id, M, Size, Cost>,
        input_size: Size,
    ) -> Vec<(Vertex<Id, M, Size, Cost>, Cost)> {
        // todo find a way to get a compile-time guarantee that unwrap cannot fail
        let mut next_object = category.get_object(&self.target).unwrap().clone();
        let output = self.metadata.apply(input_size);
        //todo configurable: replace by output, do not touch, set to constant
        next_object.size = output.size;
        vec![(Vertex::Object(next_object), output.cost)]
    }
}

/// Needed for `petgraph`
impl<Id, M, Size, Cost> IntoWeightedEdge<Cost> for Morphism<Id, M, Size, Cost>
where
    Id: Key,
    M: MorphismMeta<Size, Cost>,
    Size: Clone, //todo size where?
{
    type NodeId = Id;

    fn into_weighted_edge(self) -> (Self::NodeId, Self::NodeId, Cost) {
        (
            self.source,
            self.target,
            self.metadata.apply(self.input_size).cost,
        )
    }
}

/// Needed for `pathfinding`
impl<Id, M, Size, Cost> PartialEq for Morphism<Id, M, Size, Cost>
where
    Id: Key,
    M: MorphismMeta<Size, Cost>,
    Size: Clone, //todo size where?
{
    fn eq(&self, other: &Self) -> bool {
        self.metadata == other.metadata
            && self.target == other.target
            && self.source == other.source
    }
}

/// Needed for `pathfinding`
impl<Id, M, Size, Cost> Eq for Morphism<Id, M, Size, Cost>
where
    Id: Key,
    M: MorphismMeta<Size, Cost>,
    Size: Clone, //todo size where?
{
}

/// Needed for `pathfinding`
impl<Id, M, Size, Cost> std::hash::Hash for Morphism<Id, M, Size, Cost>
where
    Id: Key,
    M: MorphismMeta<Size, Cost>,
    Size: Clone, //todo size where?
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.source.hash(state);
        self.target.hash(state);
        self.metadata.hash(state);
    }
}
