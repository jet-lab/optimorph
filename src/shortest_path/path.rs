use std::{fmt::Display, ops::Deref};

use pathfinding::num_traits::Zero;
use petgraph::algo::FloatMeasure;
use thiserror::Error;

use crate::{
    category::HasId,
    collections::{Replace, SomeVec},
    impls::Float,
    morphism::{ApplyMorphism, Morphism, MorphismOutput},
    vertex::Vertex,
};

////////////////////////////////////////
// Path types
//

/// The basic type produced by the path optimization algorithms.
#[derive(Clone, Debug)]
pub struct Path<Id, M, Obj = Id, Size = Float, Cost = Float> {
    pub vertices: SomeVec<Vertex<Id, M, Obj, Size>>,
    pub cost: Cost,
}

/// Privately wraps SimplePath as a promise that the contained data is
/// structured in a way to ensure it can be converted without issue into
/// ClosedPath or CompositeMorphism, even though the Path type does not
/// enforce that structure.
///
/// The path is guaranteed to be in object/morphism alternating form,
/// starting and ending with objects. It has at least three vertices in the
/// form O-M-O, but may have more, such as O-M-O-M-O-M-O-M-O...
///
/// It is critical that the inner field remains private to the shortest_path
/// module. Any mutation or instantiation of this type must be tightly
/// restricted to ensure its integrity.
#[derive(Clone, Debug)]
pub struct WellFormedPath<Id, M, Obj = Id, Size = Float, Cost = Float>(
    pub(super) Path<Id, M, Obj, Size, Cost>,
);

/// Alternate representation of a WellFormedPath with more structure:
/// Organized as a list of each morphism from the path combined with its two
/// adjacent objects.
#[derive(Clone, Debug)]
pub struct AppliedCompositeMorphism<Id, M, Obj = Id, Size = Float, Cost = Float> {
    pub morphisms: SomeVec<AppliedMorphism<Id, M, Obj, Size>>,
    pub cost: Cost,
}

/// A heavyweight version of Morphism that includes the full input and output
/// objects plus their sizes since this is applied in a path.
#[derive(Clone, Debug)]
pub struct AppliedMorphism<Id, M, Obj = Id, Size = Float> {
    pub morphism: Morphism<Id, M>,
    pub source: (Obj, Size),
    pub target: (Obj, Size),
}

impl<Id, M, Obj, Size> Display for AppliedMorphism<Id, M, Obj, Size>
where
    Id: Display,
    Obj: HasId<Id>,
    M: Display,
    Size: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let alt = f.alternate();
        let nl = if alt { "\n" } else { " " };
        let indent = if alt { "  " } else { "" };
        Display::fmt(&self.morphism.metadata, f)?;
        f.write_str(if alt { ":\n" } else { "{ " })?;
        // source
        f.write_str(indent)?;
        f.write_str(if alt { "┌──" } else { "" })?;
        Display::fmt(&self.source.1, f)?;
        f.write_str(" of ")?;
        Display::fmt(&self.source.0.id(), f)?;
        f.write_str(nl)?;
        // target
        f.write_str(indent)?;
        f.write_str(if alt { "└─▶" } else { "─▶ " })?;
        Display::fmt(&self.target.1, f)?;
        f.write_str(" of ")?;
        Display::fmt(&self.target.0.id(), f)?;
        //
        f.write_str(if alt { "" } else { " }" })
    }
}

impl<Id, M, Obj, Size, Cost> AppliedCompositeMorphism<Id, M, Obj, Size, Cost>
where
    Size: Clone,
{
    pub fn reapply<const NON_NEGATIVE: bool>(self, new_input: Size) -> Self
    where
        M: ApplyMorphism<Size, Cost, NON_NEGATIVE>,
    {
        let mut applied_morphisms = vec![];
        let (first, rest) = self.morphisms.destruct();
        let mut output = first.morphism.metadata.apply(new_input.clone());
        applied_morphisms.push(AppliedMorphism {
            morphism: first.morphism,
            source: (first.source.0, new_input),
            target: (first.target.0, output.size.clone()),
        });
        for item in rest {
            let input = output.size;
            output = item.morphism.metadata.apply(input.clone());
            applied_morphisms.push(AppliedMorphism {
                morphism: item.morphism,
                source: (item.source.0, input),
                target: (item.target.0, output.size.clone()),
            });
        }

        Self {
            morphisms: applied_morphisms.try_into().expect("first() guarantees >1"),
            cost: output.cost,
        }
    }
}

impl<Id, M, Obj, Size, Cost, const NON_NEGATIVE: bool> ApplyMorphism<Size, Cost, NON_NEGATIVE>
    for AppliedCompositeMorphism<Id, M, Obj, Size, Cost>
where
    M: ApplyMorphism<Size, Cost, NON_NEGATIVE>,
{
    fn apply(&self, input: Size) -> MorphismOutput<Size, Cost> {
        let mut output = self.morphisms.first().morphism.metadata.apply(input);
        for item in self.morphisms.iter_rest() {
            output = item.morphism.metadata.apply(output.size);
        }
        output
    }
}

/// Reapplies the sequence of morphisms based on a new input size. This function
/// accumulates, which means it uses the output size of each morphism as the
/// input size of the next.
///
/// Use one of the sum functions to aggregate the cost into a single number.
///
/// The costs cannot be accumulated here because there are different traits that
/// describe how to get a zero value depending on the optimization library. a
/// single unified trait cannot be defined to cover both with auto-impls because
/// there would be conflicting implementations. Specialization could solve this
/// elegantly, but it's not stable. Alternatively you could use multiple wrapper
/// or marker structs plus traits with associated types, but this is overly
/// complicated. Simply summing outside the function is clearer in most cases.
#[allow(clippy::type_complexity)]
pub(crate) fn reapply<Id, M, Obj, Size, Cost, const NON_NEGATIVE: bool>(
    vertices: impl IntoIterator<Item = Vertex<Id, M, Obj, Size>>,
    mut input: Size,
) -> (Vec<Vertex<Id, M, Obj, Size>>, Vec<Cost>)
where
    Size: Clone,
    M: ApplyMorphism<Size, Cost, NON_NEGATIVE>,
{
    let mut new_path = vec![];
    let mut cost_agg = vec![];
    for vertex in vertices {
        match vertex {
            Vertex::Object { inner, .. } => new_path.push(Vertex::Object {
                inner,
                size: input.clone(),
            }),
            Vertex::Morphism { inner, .. } => {
                let MorphismOutput { size, cost } = inner.metadata.apply(input.clone());
                new_path.push(Vertex::Morphism {
                    inner,
                    input: input.clone(),
                });
                input = size;
                cost_agg.push(cost);
            }
        }
    }
    (new_path, cost_agg)
}

#[allow(unused)]
pub(crate) fn sum_zero<T: Zero>(ns: impl IntoIterator<Item = T>) -> T {
    let mut agg = T::zero();
    for n in ns {
        agg = n + agg;
    }
    agg
}

pub(crate) fn sum_petgraph<T: FloatMeasure>(ns: impl IntoIterator<Item = T>) -> T {
    let mut agg = T::zero();
    for n in ns {
        agg = n + agg;
    }
    agg
}

////////////////////////////////////////
// Conversions
//

// Do not implement DerefMut. That would defeat the entire purpose of
// WellFormedPath. Use Into<Path> to make changes.
impl<Id, M, Obj, Size, Cost> Deref for WellFormedPath<Id, M, Obj, Size, Cost> {
    type Target = Path<Id, M, Obj, Size, Cost>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Id, M, Obj, Size, Cost> WellFormedPath<Id, M, Obj, Size, Cost> {
    pub fn into_inner(self) -> Path<Id, M, Obj, Size, Cost> {
        self.0
    }
}

impl<Id, M, Obj, Size, Cost> From<WellFormedPath<Id, M, Obj, Size, Cost>>
    for Path<Id, M, Obj, Size, Cost>
{
    fn from(value: WellFormedPath<Id, M, Obj, Size, Cost>) -> Self {
        value.0
    }
}

impl<Id, M, Obj, Size, Cost> From<WellFormedPath<Id, M, Obj, Size, Cost>>
    for AppliedCompositeMorphism<Id, M, Obj, Size, Cost>
where
    Id: Clone,
    Obj: Clone,
    M: Clone,
    Size: Clone,
{
    fn from(value: WellFormedPath<Id, M, Obj, Size, Cost>) -> Self {
        value
            .0
            .try_into()
            .expect("path was assumed to be well formed due to its wrapper type")
    }
}

impl<Id, M, Obj, Size, Cost> Display for AppliedCompositeMorphism<Id, M, Obj, Size, Cost>
where
    Id: Display,
    Obj: HasId<Id>,
    M: Display,
    Size: Display,
    Cost: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.cost, f)?;
        f.write_str(" => ")?;
        Display::fmt(&self.morphisms, f)
    }
}

impl<Id, M, Obj, Size, Cost> TryFrom<Path<Id, M, Obj, Size, Cost>>
    for AppliedCompositeMorphism<Id, M, Obj, Size, Cost>
where
    Id: Clone,
    Obj: Clone,
    M: Clone,
    Size: Clone,
{
    type Error = InvalidPath;

    fn try_from(value: Path<Id, M, Obj, Size, Cost>) -> Result<Self, Self::Error> {
        let len = value.vertices.len();
        if len == 0 {
            return Err(InvalidPath::EmptyPath);
        }
        if len < 3 || len % 2 != 1 {
            return Err(InvalidPath::MalformedStructure);
        }
        let components = value
            .vertices
            .to_vec()
            .chunks(2)
            .collect::<Vec<_>>()
            .windows(2)
            .map(|window| {
                let [this, that] = window else { unreachable!() };
                let [source, morphism] = this else { unreachable!() };
                let target = &that[0];
                let source = match source {
                    Vertex::Object { inner, size } => (inner.clone(), size.clone()),
                    _ => return Err(InvalidPath::SourceIsNotObject),
                };
                let target = match target {
                    Vertex::Object { inner, size } => (inner.clone(), size.clone()),
                    _ => return Err(InvalidPath::TargetIsNotObject),
                };
                let Vertex::Morphism { inner, .. } = morphism.clone()
                    else { return Err(InvalidPath::InnerIsNotMorphism)};
                Ok(AppliedMorphism {
                    morphism: inner,
                    source,
                    target,
                })
            })
            .collect::<Result<Vec<_>, InvalidPath>>()?;

        Ok(Self {
            morphisms: components.try_into().expect("already checked length"),
            cost: value.cost,
        })
    }
}

impl<Id, M, Obj, Size, Cost> Replace<Cost> for AppliedCompositeMorphism<Id, M, Obj, Size, Cost> {
    type With<U> = AppliedCompositeMorphism<Id, M, Obj, Size, U>;

    fn read(&self) -> &Cost {
        &self.cost
    }

    fn replace<R>(self, item: R) -> (Self::With<R>, Cost) {
        (
            AppliedCompositeMorphism {
                morphisms: self.morphisms,
                cost: item,
            },
            self.cost,
        )
    }
}

impl<Id, M, Obj, Size, Cost> Replace<Cost> for Path<Id, M, Obj, Size, Cost> {
    type With<U> = Path<Id, M, Obj, Size, U>;

    fn read(&self) -> &Cost {
        &self.cost
    }

    fn replace<R>(self, item: R) -> (Self::With<R>, Cost) {
        (
            Path {
                vertices: self.vertices,
                cost: item,
            },
            self.cost,
        )
    }
}

impl<Id, M, Obj, Size, Cost> Replace<Cost> for WellFormedPath<Id, M, Obj, Size, Cost> {
    type With<U> = WellFormedPath<Id, M, Obj, Size, U>;

    fn read(&self) -> &Cost {
        &self.0.cost
    }

    fn replace<R>(self, item: R) -> (Self::With<R>, Cost) {
        let (one, two) = self.0.replace(item);
        (WellFormedPath(one), two)
    }
}

//////////////////////////////////////
// Errors
//

#[derive(Error, Debug)]
pub enum InvalidPath {
    #[error("The source vertex of this path is not an object")]
    SourceIsNotObject,
    #[error("")]
    InnerIsNotMorphism,
    #[error("The target vertex of this path is not an object")]
    TargetIsNotObject,
    #[error("The lack of a path should be represented with None")]
    EmptyPath,
    #[error("")]
    MalformedStructure,
}
