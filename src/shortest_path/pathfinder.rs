use std::hash::Hash;

use crate::category::HasId;
use crate::category::Key;
use crate::morphism::ApplyMorphism;
use crate::morphism::MorphismMeta;
use crate::vertex::LeanVertex;
use crate::vertex::Vertex;
use pathfinding::num_traits::Zero;
use pathfinding::prelude::dijkstra;

use crate::category::Category;

pub fn shortest_single_path_with_dijkstra<
    Id: Key,
    Object: HasId<Id>,
    M: MorphismMeta + ApplyMorphism<Size, Cost, true>,
    Size: PathfindingSize,
    Cost: PathfindingCost,
>(
    category: &Category<Id, M, Object>,
    source: Id,
    target: Id,
    input_size: Size,
) -> Option<(Vec<Vertex<Id, M, Object, Size>>, Cost)> {
    let start_vertex = LeanVertex::Object {
        id: source,
        size: input_size,
    };
    dijkstra(
        &start_vertex,
        move |n| n.successors(category),
        move |n| n.is_object_with_id(&target),
    )
    .map(|(items, cost)| {
        (
            items
                .into_iter()
                .map(|v| Vertex::from(v, category))
                .collect(),
            cost,
        )
    })
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
