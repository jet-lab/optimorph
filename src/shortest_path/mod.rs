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
/// Cost is allowed to be negative. An error will be returned if a negative
/// cycle exists, because that means there is no optimal path.
///
/// Every morphism uses the user-provided "input_size" as its input. There is no
/// accumulation of sizes through a path. No object size or morphism input can
/// be based on the shape of the graph that came before it.
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

/// Use this if you prefer "good enough" results instead of errors.
///
/// If there are paths from source to target, but none are optimal (due to a
/// negative cycle), the Negatable optimizer will return an error.
/// NegatableInfallible instead tries to return a "good enough" path when there
/// is no optimal path, instead of returning an error.
///
/// In general, this uses the same logic as Negatable, except that it recovers
/// from errors by using the same dijkstra implementation that is used by the
/// Accumulating optimizer when a negative cycle is found.
///
/// This means that the returned result may not always be optimal because
/// dijkstra does not provide accurate results with negative costs. But this
/// would only occur here if there is actually no optimal solution.
///
/// You should carefully assess the return value from this optimizer to decide
/// if it is good enough.
pub struct NegatableInfallible;

impl<M, Size, Cost, const NON_NEGATIVE: bool> Optimizer<M, Size, Cost, NON_NEGATIVE>
    for NegatableInfallible
where
    M: MorphismMeta + ApplyMorphism<Size, Cost, NON_NEGATIVE>,
    Size: PathfindingSize + Clone,
    Cost: PathfindingCost + FloatMeasure,
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
        Ok(my_petgraph::shortest_single_path_with_bellman_ford(
            category,
            source.clone(),
            target.clone(),
            input_size.clone(),
        )
        .unwrap_or_else(|_| {
            my_pathfinding::inaccurate_shortest_single_path_with_dijkstra(
                category, source, target, input_size,
            )
        }))
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
            match my_petgraph::shortest_multi_path_with_bellman_ford(
                category,
                source.clone(),
                &targets,
                input.clone(),
            ) {
                Ok(paths) => results.extend(paths),
                Err(_) => {
                    for target in targets.clone() {
                        if let Some(path) =
                            my_pathfinding::inaccurate_shortest_single_path_with_dijkstra(
                                category,
                                source.clone(),
                                target,
                                input.clone(),
                            )
                        {
                            results.push(path);
                        }
                    }
                }
            }
        }
        Ok(results)
    }
}
