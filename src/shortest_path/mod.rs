mod my_pathfinding;
mod my_petgraph;
pub mod optimizer;
pub mod path;

use std::convert::Infallible;

use petgraph::algo::FloatMeasure;

use crate::{
    category::{Category, Key, Object},
    morphism::{ApplyMorphism, MorphismMeta},
};

use self::{
    my_pathfinding::{PathfindingCost, PathfindingSize},
    my_petgraph::PathFindingError,
    optimizer::Optimizer,
    path::WellFormedPath,
};

/// Shortest path optimizer that uses pathfinding::dijkstra.
///
/// Cost is not allowed to be negative.
///
/// Accumulates Size information. This uses a morphism's output as the size of
/// the next object, which is the input for all of the next object's outbound
/// morphisms.
pub struct Accumulating;

impl<M, Size, Cost> Optimizer<M, Size, Cost, true> for Accumulating
where
    M: MorphismMeta + ApplyMorphism<Size, Cost, true>,
    Size: PathfindingSize,
    Cost: PathfindingCost,
{
    type Error<Id: Key, O> = Infallible;

    fn shortest_path<Id, Obj>(
        category: &Category<Id, M, Obj>,
        source: Id,
        target: Id,
        input_size: Size,
    ) -> Result<Option<WellFormedPath<Id, M, Obj, Size, Cost>>, Infallible>
    where
        Id: Key,
        Obj: Object<Id>,
    {
        Ok(my_pathfinding::shortest_single_path_with_dijkstra(
            category, source, target, input_size,
        ))
    }
}

/// Shortest path optimizer that uses petgraph::bellman_ford.
///
/// Cost is allowed to be negative.
///
/// Every morphism uses the user-provided "input_size" as its input. There is no
/// accumulation of sizes through a path. No object size or morphism input can
/// be based on the shape of the graph that came before it.
///
/// TODO: There is a much faster approach to shortest_paths that uses the fact
/// that the petgraph output for bellmon_ford already calculates the distance
/// from the input source to every possible target.
pub struct Negatable;

impl<M, Size, Cost, const NON_NEGATIVE: bool> Optimizer<M, Size, Cost, NON_NEGATIVE> for Negatable
where
    M: MorphismMeta + ApplyMorphism<Size, Cost, NON_NEGATIVE>,
    Size: Clone,
    Cost: FloatMeasure,
{
    type Error<Id: Key, O> = PathFindingError<Id>;

    fn shortest_path<Id, Obj>(
        category: &Category<Id, M, Obj>,
        source: Id,
        target: Id,
        input_size: Size,
    ) -> Result<Option<WellFormedPath<Id, M, Obj, Size, Cost>>, PathFindingError<Id>>
    where
        Id: Key,
        Obj: Object<Id>,
    {
        my_petgraph::shortest_single_path_with_bellman_ford(category, source, target, input_size)
    }

    fn shortest_paths<Id, Obj>(
        category: &Category<Id, M, Obj>,
        sources: Vec<(Id, Size)>,
        targets: Vec<Id>,
    ) -> Result<Vec<WellFormedPath<Id, M, Obj, Size, Cost>>, Self::Error<Id, Obj>>
    where
        Id: Key,
        Obj: Object<Id>,
    {
        let mut results = vec![];
        for (source, input) in sources {
            results.extend(my_petgraph::shortest_multi_path_with_bellman_ford(
                category,
                source.clone(),
                &targets,
                input.clone(),
            )?);
        }
        Ok(results)
    }
}
