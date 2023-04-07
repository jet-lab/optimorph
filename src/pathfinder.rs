use std::hash::Hash;

use crate::category::Key;
use crate::cost::ApplyMorphism;
use crate::morphism::MorphismMeta;
use crate::object::HasId;
use crate::vertex::Vertex;
use pathfinding::num_traits::Zero;
use pathfinding::prelude::dijkstra;

use crate::category::Category;

pub fn optimize_single_path_with_dijkstra<
    Id: Key,
    Object: HasId<Id>,
    M: MorphismMeta + ApplyMorphism<Size, Cost>,
    Size: PathfindingSize,
    Cost: PathfindingCost,
>(
    category: Category<Id, M, Object>,
    source: Id,
    target: Id,
    input_size: Size,
) -> Option<(Vec<Vertex<Id, M, Size>>, Cost)> {
    let start_vertex = Vertex::Object {
        id: source,
        size: input_size,
    };
    dijkstra(
        &start_vertex,
        move |n| n.successors(&category),
        move |n| n.is_object_with_id(&target),
    )
}

pub trait PathfindingCost: Zero + Eq + Hash + Clone + Ord + Copy {}
impl<T: Zero + Eq + Hash + Clone + Ord + Copy> PathfindingCost for T {}

pub trait PathfindingSize: Clone + Eq + Hash {}
impl<T: Clone + Eq + Hash> PathfindingSize for T {}

fn thing<T: Limiter<Selection = NonNegative>>(x: T) {}

struct LimitMe;

impl Limiter for LimitMe {
    type Selection = NonNegative;
}

trait Limiter {
    type Selection;
}

// trait SelectMe{}
struct NonNegative;
struct NonComposable;

/*
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
