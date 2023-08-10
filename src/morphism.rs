use std::{fmt::Display, hash::Hash};

use pathfinding::num_traits::Zero;

use crate::{
    category::{Category, HasId, Key},
    collections::SomeVec,
    impls::Float,
    vertex::LeanVertex,
};

pub trait MorphismMeta: Hash + Eq + Clone {}
impl<M> MorphismMeta for M where M: Hash + Eq + Clone {}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Morphism<Id, M> {
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

impl<Id: Display, M: Display> Display for Morphism<Id, M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let alt = f.alternate();
        let nl = if alt { "\n" } else { " " };
        let indent = if alt { "  " } else { "" };
        Display::fmt(&self.metadata, f)?;
        f.write_str(if alt { ":\n" } else { "{ " })?;
        // source
        f.write_str(indent)?;
        f.write_str(if alt { "┌──" } else { "" })?;
        Display::fmt(&self.source, f)?;
        f.write_str(nl)?;
        // target
        f.write_str(indent)?;
        f.write_str(if alt { "└─▶" } else { "─▶ " })?;
        Display::fmt(&self.target, f)?;
        //
        f.write_str(if alt { "" } else { " }" })
    }
}

impl<Id, M, IntoId1, IntoId2, IntoM> From<(IntoId1, IntoId2, IntoM)> for Morphism<Id, M>
where
    IntoId1: Into<Id>,
    IntoId2: Into<Id>,
    IntoM: Into<M>,
{
    fn from((source, target, metadata): (IntoId1, IntoId2, IntoM)) -> Self {
        Self {
            source: source.into(),
            target: target.into(),
            metadata: metadata.into(),
        }
    }
}

impl<Id, M> Morphism<Id, M> {
    pub fn new(source: Id, target: Id, metadata: impl Into<M>) -> Self {
        Self {
            source,
            target,
            metadata: metadata.into(),
        }
    }

    /// Needed for `pathfinding`
    pub(crate) fn successors<const NON_NEGATIVE: bool, Obj, Size: Clone, Cost>(
        &self,
        category: &Category<Id, M, Obj>,
        input_size: Size,
    ) -> Vec<(LeanVertex<Id, M, Size>, Cost)>
    where
        M: ApplyMorphism<Size, Cost, NON_NEGATIVE>,
        Id: Key,
        Obj: HasId<Id>,
    {
        // todo find a way to get a compile-time guarantee that unwrap cannot fail
        // todo should apply have access to these states?
        let _input_object = category.get_object(&self.source).unwrap();
        let _output_object = category.get_object(&self.target).unwrap();
        let output = self.metadata.apply(input_size);
        //todo configurable: replace by output, do not touch, set to constant
        // next_object.size = output.size;
        vec![(
            LeanVertex::Object {
                inner: self.target.clone(),
                size: output.size,
            },
            output.cost,
        )]
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

impl<Size, Cost: Zero> MorphismOutput<Size, Cost> {
    pub fn free(size: Size) -> Self {
        Self {
            size,
            cost: Cost::zero(),
        }
    }
}

pub struct CompositeMorphism<Id, M>(pub SomeVec<Morphism<Id, M>>);

impl<Id, M, Size, Cost, const NON_NEGATIVE: bool> ApplyMorphism<Size, Cost, NON_NEGATIVE>
    for CompositeMorphism<Id, M>
where
    M: ApplyMorphism<Size, Cost, NON_NEGATIVE>,
{
    fn apply(&self, input: Size) -> MorphismOutput<Size, Cost> {
        let mut output = self.0.first().metadata.apply(input);
        for item in self.0.iter_rest() {
            output = item.metadata.apply(output.size);
        }
        output
    }
}

/// A "free" morphism has zero cost and passes through the input untouched.
pub trait FreeMorphism: Sized + Clone {
    fn free<Cost: Zero>(&self) -> MorphismOutput<Self, Cost> {
        MorphismOutput::free(self.clone())
    }
}
impl<T: Sized + Clone> FreeMorphism for T {}
