use std::collections::HashSet;
use std::hash::Hash;

use crate::category::Key;
use crate::category::Object;
use crate::morphism::ApplyMorphism;
use crate::morphism::MorphismMeta;
use crate::vertex::LeanVertex;
use crate::vertex::Vertex;
use pathfinding::num_traits::Zero;
use pathfinding::prelude::yen;

use crate::category::Category;

use super::path::{Path, WellFormedPath};

// TODO: clarify the names of these functions, and probably the optimizer methods too

pub fn shortest_single_path_with_dijkstra<
    Id: Key,
    Obj: Object<Id>,
    M: MorphismMeta + ApplyMorphism<Size, Cost, true>,
    Size: PathfindingSize,
    Cost: PathfindingCost,
>(
    category: &Category<Id, M, Obj>,
    source: Id,
    target: Id,
    input_size: Size,
) -> Option<WellFormedPath<Id, M, Obj, Size, Cost>> {
    inaccurate_shortest_single_path_with_dijkstra_yen(category, source, target, input_size, 1).pop()
}

/// This is considered "inaccurate" because it does not express the requirement
/// through the type system that costs must be non-negative, which is a
/// requirement for dijkstra to provide accurate results. This function may
/// return a sub-optimal path if you cannot guarantee the cost to be
/// non-negative.
pub(crate) fn inaccurate_shortest_single_path_with_dijkstra<
    const NON_NEGATIVE: bool,
    Id: Key,
    Obj: Object<Id>,
    M: MorphismMeta + ApplyMorphism<Size, Cost, NON_NEGATIVE>,
    Size: PathfindingSize,
    Cost: PathfindingCost,
>(
    category: &Category<Id, M, Obj>,
    source: Id,
    target: Id,
    input_size: Size,
) -> Option<WellFormedPath<Id, M, Obj, Size, Cost>> {
    inaccurate_shortest_single_path_with_dijkstra_yen(category, source, target, input_size, 1).pop()
}

/// This is considered "inaccurate" because it does not express the requirement
/// through the type system that costs must be non-negative, which is a
/// requirement for dijkstra to provide accurate results. This function may
/// return a sub-optimal path if you cannot guarantee the cost to be
/// non-negative.
pub(crate) fn inaccurate_shortest_single_path_with_dijkstra_yen<
    const NON_NEGATIVE: bool,
    Id: Key,
    Obj: Object<Id>,
    M: MorphismMeta + ApplyMorphism<Size, Cost, NON_NEGATIVE>,
    Size: PathfindingSize,
    Cost: PathfindingCost,
>(
    category: &Category<Id, M, Obj>,
    source: Id,
    target: Id,
    input_size: Size,
    n_paths: usize,
) -> Vec<WellFormedPath<Id, M, Obj, Size, Cost>> {
    if source == target
        || category.get_object(&source).is_none()
        || category.get_object(&target).is_none()
    {
        return vec![];
    }
    let start_vertex = LeanVertex::Object {
        inner: source,
        size: input_size,
    };
    let mut already_seen = HashSet::new();
    let mut ret = yen(
        &start_vertex,
        move |n| n.blacklisted_successors(category, &mut already_seen),
        move |n| n.is_object_with_id(&target),
        n_paths,
    )
    .into_iter()
    .map(|(items, cost)| {
        WellFormedPath(Path {
            vertices: items
                .into_iter()
                .map(|v| Vertex::from(v, category))
                .collect::<Vec<_>>()
                .try_into()
                .expect("would be none, not empty"),
            cost,
        })
    })
    .collect::<Vec<WellFormedPath<Id, M, Obj, Size, Cost>>>();

    // while the returned value is expected to be sorted, dijkstra being
    // well-behaved is based on the assumption that costs are positive. this
    // sort is an insurance mechanism to ensure that the result is sorted
    // properly even if the assumptions are not satsified.
    ret.sort_by_key(|wfp| wfp.cost);

    ret
}

pub trait PathfindingCost: Zero + Eq + Hash + Clone + Ord + Copy {}
impl<T: Zero + Eq + Hash + Clone + Ord + Copy> PathfindingCost for T {}

pub trait PathfindingSize: Clone + Eq + Hash {}
impl<T: Clone + Eq + Hash> PathfindingSize for T {}

// fn thing<T: Limiter<Selection = NonNegative>>(x: T) {}

// struct LimitMe;

// impl Limiter for LimitMe {
//     type Selection = NonNegative;
// }

// trait Limiter {
//     type Selection;
// }

// // trait SelectMe{}
// struct NonNegative;
// struct NonComposable;

/*
brainstorming

dijkstra:
- requires non-negative cost
- allows size accumulation

bellman_ford:
- allows negative cost
- ignores size accumulation

generic optimization method:
- if cost can be negative, use bellman_ford
- if cost cannot be negative, use dijkstra

specific optimization methods:
- with_size_accumulation - dijkstra requires non-negative
- allowing_negative_cost - bellman_ford works with anything but comes with a caveat in the name or docs

trait CostMeta {
    const IS_NEGATABLE: bool;
}

unsafe trait NeverNegative
- type constraint on dijkstra
- impl Negatable with false

 */
