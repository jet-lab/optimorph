use std::{fmt::Debug, hash::Hash, rc::Rc};

use petgraph::IntoWeightedEdge;

use crate::{
    cost::{ApplyMorphism, MorphismOutput, Size},
    object::Object,
    vertex::Vertex, category::{Key, Category},
};

//todo generic over Cost (not Size)
pub trait MorphismMeta: Hash + Debug + Clone + Eq + ApplyMorphism<Size> {}
impl<M: Hash + Debug + Clone + Eq + ApplyMorphism<Size>> MorphismMeta for M {}

#[derive(Clone, Debug)]
pub struct Morphism<Id, M>
where
    Id: Key,
    M: MorphismMeta,
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
    pub input_size: Size,
}

impl<Id, M> Morphism<Id, M>
where
    Id: Key,
    M: MorphismMeta,
{
    /// Needed for `pathfinding`
    pub fn successors(
        &self,
        category: &Category<Id, M>,
        input_size: Size,
    ) -> Vec<(Vertex<Id, M>, Size)> {
        // todo find a way to get a compile-time guarantee that unwrap cannot fail
        let mut next_object = category.get_object(&self.target).unwrap().clone();
        let output = self.metadata.apply(input_size);
        //todo configurable: replace by output, do not touch, set to constant
        next_object.size = output.size;
        vec![(Vertex::Object(next_object), output.cost)]
    }
}

/// Needed for `petgraph`
impl<Id, M> IntoWeightedEdge<Size> for Morphism<Id, M>
where
    Id: Key,
    M: MorphismMeta,
{
    type NodeId = Id;

    fn into_weighted_edge(self) -> (Self::NodeId, Self::NodeId, Size) {
        (
            self.source,
            self.target,
            self.metadata.apply(self.input_size).cost,
        )
    }
}

/// Needed for `pathfinding`
impl<Id, M> PartialEq for Morphism<Id, M>
where
    Id: Key,
    M: MorphismMeta,
{
    fn eq(&self, other: &Self) -> bool {
        self.metadata == other.metadata
            && self.target == other.target
            && self.source == other.source
    }
}

/// Needed for `pathfinding`
impl<Id, M> Eq for Morphism<Id, M>
where
    Id: Key,
    M: MorphismMeta,
{
}

/// Needed for `pathfinding`
impl<Id, M> std::hash::Hash for Morphism<Id, M>
where
    Id: Key,
    M: MorphismMeta,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.source.hash(state);
        self.target.hash(state);
        self.metadata.hash(state);
    }
}
