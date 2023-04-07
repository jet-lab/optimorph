use std::hash::Hash;

use crate::{
    category::{Category, HasId, Key},
    impls::Float,
    vertex::Vertex,
};

pub trait MorphismMeta: Hash + Clone + Eq {}
impl<M> MorphismMeta for M where M: Hash + Clone + Eq {}

#[derive(Debug, Clone)]
pub struct Morphism<Id, M>
where
    Id: Key,
    M: MorphismMeta,
{
    pub source: Id,
    pub target: Id,
    /// This should contain:
    /// - Uniquely identifying information that distinguishes this morphism from
    ///   other morphisms with the same source and target. Eq and Hash values
    ///   must be unique.
    /// - Logic to determine cost and output size from applying the morphism. It
    ///   should implement some variant of ApplyMorphism in order to be useful.
    pub metadata: M,
}

impl<Id, M> Morphism<Id, M>
where
    Id: Key,
    M: MorphismMeta,
{
    pub fn new(source: Id, target: Id, metadata: M) -> Self {
        Self {
            source,
            target,
            metadata,
        }
    }

    /// Needed for `pathfinding`
    pub fn successors<Object: HasId<Id>, Size: Clone, Cost>(
        &self,
        category: &Category<Id, M, Object>,
        input_size: Size,
    ) -> Vec<(Vertex<Id, M, Size>, Cost)>
    where
        M: ApplyMorphism<Size, Cost>,
    {
        // todo find a way to get a compile-time guarantee that unwrap cannot fail
        // todo should apply have access to these states?
        let _input_object = category.get_object(&self.source).unwrap();
        let _output_object = category.get_object(&self.target).unwrap();
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

/// Determines the outcome of applying a morphism to its input object of the
/// provided Size. Outputs the Cost of this  and the Size of the target object
/// after application.
///
/// # NON_NEGATIVE
/// When true, the implementor promises that the Cost output will never be
/// negative. This guarantee is necessary for most shortest-path optimizations
/// algorithms to work properly, such as dijkstra.
///
/// The precise requirement is the following: For any two Sizes s1 and s2, the
/// following must be true:
/// * apply_non_negative(s1) + apply_non_negative(s2) >= s1
/// * apply_non_negative(s1) + apply_non_negative(s2) >= s2
///
/// This guarantee cannot be provided by the compiler. Implement this trait at
/// your own risk.
pub trait ApplyMorphism<Size = Float, Cost = Float, const NON_NEGATIVE: bool = false> {
    fn apply(&self, input: Size) -> MorphismOutput<Size, Cost>;
}

pub struct MorphismOutput<Size = Float, Cost = Float> {
    pub size: Size,
    pub cost: Cost,
}
