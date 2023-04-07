use std::{fmt::Debug, hash::Hash, marker::PhantomData};

use crate::{
    category::{Category, Key},
    cost::{ApplyMorphism, Float},
    object::HasId,
    vertex::Vertex,
};

//todo generic over Cost (not Size)
pub trait MorphismMeta<Size, Cost>: Hash + Clone + Eq + ApplyMorphism<Size, Cost> {}
impl<Size, Cost, M> MorphismMeta<Size, Cost> for M where
    M: Hash + Clone + Eq + ApplyMorphism<Size, Cost>
{
}

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
    pub _phantom_cost: PhantomData<Cost>,
    pub _phantom_size: PhantomData<Size>,
}

impl<Id, M, Size, Cost> Debug for Morphism<Id, M, Size, Cost>
where
    Id: Key,
    M: MorphismMeta<Size, Cost> + Debug,
    Size: Clone, //todo size where?
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Morphism")
            .field("source", &self.source)
            .field("target", &self.target)
            .field("metadata", &self.metadata)
            .finish()
    }
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
            _phantom_cost: PhantomData,
            _phantom_size: PhantomData,
        }
    }
}

impl<Id, M, Size, Cost> Morphism<Id, M, Size, Cost>
where
    Id: Key,
    M: MorphismMeta<Size, Cost>,
    Size: Clone, //todo size where?
{
    pub fn new(source: Id, target: Id, metadata: M) -> Self {
        Self {
            source,
            target,
            metadata,
            _phantom_cost: PhantomData,
            _phantom_size: PhantomData,
        }
    }

    /// Needed for `pathfinding`
    pub fn successors<Object: HasId<Id>>(
        &self,
        category: &Category<Id, M, Object, Size, Cost>,
        input_size: Size,
    ) -> Vec<(Vertex<Id, M, Size, Cost>, Cost)> {
        // todo find a way to get a compile-time guarantee that unwrap cannot fail
        // todo should apply have access to these states?
        let _input_object = category.get_object(&self.source).unwrap().clone();
        let _output_object = category.get_object(&self.target).unwrap().clone();
        let output = self.metadata.apply(input_size);
        //todo configurable: replace by output, do not touch, set to constant
        // next_object.size = output.size;
        vec![(
            Vertex::Object {
                id: self.target.clone(),
                size: output.size,
            },
            output.cost,
        )]
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
