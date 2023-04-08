use std::collections::HashMap;

use petgraph::{
    algo::{bellman_ford, FloatMeasure},
    stable_graph::NodeIndex,
    Graph,
};
use thiserror::Error;

use crate::{
    category::HasId,
    category::{Category, Key},
    morphism::ApplyMorphism,
    morphism::MorphismMeta,
    vertex::{LeanVertex, Vertex},
};

pub fn shortest_single_path_with_bellman_ford<
    const NON_NEGATIVE: bool,
    Id: Key,
    Object: HasId<Id>,
    M: MorphismMeta + ApplyMorphism<Size, Cost, NON_NEGATIVE>,
    Size: Clone,
    Cost: FloatMeasure,
>(
    category: &Category<Id, M, Object>,
    source: Id,
    target: Id,
    input_size: Size, // used for all morphisms - accumulation is not supported
) -> Result<Option<(Vec<Vertex<Id, M, Object, Size>>, Cost)>, PathFindingError<Id>> {
    if category.get_object(&source).is_none() {
        return Ok(None)
    }
    let cg = CategoryGraph::new(category, input_size);
    let source_index = *cg
        .object_id_to_index
        .get(&source)
        .ok_or(MissingObject(source.clone()))?;
    let target_index = *cg
        .object_id_to_index
        .get(&target)
        .ok_or(MissingObject(target.clone()))?;
    let paths = bellman_ford(&cg.graph, source_index).map_err(|_| NegativeCycle)?;
    let mut path = vec![];
    let mut last = target_index;
    while last != source_index {
        path.push(last);
        if let None = paths.predecessors[last.index()] {
            return Ok(None)
        }
        last = paths.predecessors[last.index()].unwrap();
    }
    path.push(last);
    path.reverse();

    Ok(Some((
        path.into_iter()
            .map(|idx| cg.index_to_vertex.get(&idx).cloned())
            .collect::<Option<Vec<_>>>()
            .unwrap() // todo
            .into_iter()
            .map(|v| Vertex::from(v, category))
            .collect(),
        paths.distances[last.index()], // todo i think this is wrong
    )))
}

#[derive(Error, Debug)]
pub enum PathFindingError<Id: std::fmt::Debug> {
    #[error("The object could not be identified as a vertex in the underlying graph")]
    MissingObject(Id),
    #[error("There is a cycle of negative costs that prevent shortest path optimization")]
    NegativeCycle,
}
use PathFindingError::*;

struct CategoryGraph<Id, M, Size, Cost, const NON_NEGATIVE: bool>
where
    Id: Key,
    M: MorphismMeta + ApplyMorphism<Size, Cost, NON_NEGATIVE>,
    Size: Clone,
    Cost: FloatMeasure,
{
    graph: Graph<LeanVertex<Id, M, Size>, Cost>,
    object_id_to_index: HashMap<Id, NodeIndex>,
    index_to_vertex: HashMap<NodeIndex, LeanVertex<Id, M, Size>>,
}

impl<Id, M, Size, Cost, const NON_NEGATIVE: bool> CategoryGraph<Id, M, Size, Cost, NON_NEGATIVE>
where
    Id: Key,
    M: MorphismMeta + ApplyMorphism<Size, Cost, NON_NEGATIVE>,
    Size: Clone,
    Cost: FloatMeasure,
{
    fn new<Object: HasId<Id>>(
        category: &Category<Id, M, Object>,
        input_size: Size,
    ) -> CategoryGraph<Id, M, Size, Cost, NON_NEGATIVE> {
        let mut graph = Graph::new();
        let (objects, morphisms, _) = category.clone().destruct();
        let mut object_id_to_index = HashMap::new();
        let mut morphism_to_index = HashMap::new();
        let mut index_to_vertex = HashMap::new();
        for object in objects.into_values() {
            let index = graph.add_node(LeanVertex::Object {
                id: object.id(),
                size: input_size.clone(),
            });
            object_id_to_index.insert(object.id(), index);
            index_to_vertex.insert(
                index,
                LeanVertex::Object {
                    id: object.id(),
                    size: input_size.clone(),
                },
            );
        }
        for morphism in &morphisms {
            let index = graph.add_node(LeanVertex::Morphism {
                inner: morphism.clone(),
                input: input_size.clone(),
            });
            morphism_to_index.insert(morphism.clone(), index);
            index_to_vertex.insert(
                index,
                LeanVertex::Morphism {
                    inner: morphism.clone(),
                    input: input_size.clone(),
                },
            );
        }
        for morphism in morphisms {
            let index = *morphism_to_index.get(&morphism).unwrap();
            graph.extend_with_edges(&[
                (
                    *object_id_to_index.get(&morphism.source).unwrap(),
                    index,
                    Cost::zero(),
                ),
                (
                    index,
                    *object_id_to_index.get(&morphism.target).unwrap(),
                    morphism.metadata.apply(input_size.clone()).cost,
                ),
            ]);
        }

        CategoryGraph {
            graph,
            object_id_to_index,
            index_to_vertex,
        }
    }
}

// #[test]
// fn pet01() {
//     let cg = category_graph(&get_positions(), 100.into());
//     let start_deposit = *cg.object_id_to_index.get(&PositionId::new(2)).unwrap();
//     let loan = *cg.object_id_to_index.get(&PositionId::new(0)).unwrap();
//     let x = bellman_ford(&cg.graph, start_deposit).unwrap();
//     println!("{x:?}");
//     println!("{:?}", x.distances[start_deposit.index()]);
//     let mut path = vec![];
//     let mut last = loan;
//     while last != start_deposit {
//         path.push(last);
//         last = x.predecessors[last.index()].unwrap();
//     }
//     path.push(last);
//     path.reverse();
//     for item in path {
//         println!("{:#?}", cg.index_to_vertex.get(&item).unwrap());
//     }
//     // println!("{path:#?}");
// }
// #[test]
// fn pet() {
//     let mut g = Graph::new();
//     let other_deposit = g.add_node("other_deposit");
//     let deposit = g.add_node("deposit");
//     let loan = g.add_node("loan");
//     g.extend_with_edges(&[(other_deposit, deposit, 1.0), (deposit, loan, 0.0)]);
//     let x = bellman_ford(&g, other_deposit).unwrap();
//     println!("{x:?}");
//     println!("{:?}", x.distances[other_deposit.index()]);
//     let mut path = vec![];
//     let mut last = loan;
//     while last != other_deposit {
//         path.push(last);
//         last = x.predecessors[last.index()].unwrap();
//     }
//     println!("{path:#?}");
// }

// #[test]
// fn pet2() {
//     let mut g = Graph::<&str, f64>::new();
//     let mut deposits = vec![];
//     for _ in 0..100 {
//         let deposit = g.add_node("some deposit");
//         let loan = g.add_node("some loan");
//         g.extend_with_edges(&[(deposit, loan, -1.0)]);
//         deposits.push(deposit);
//     }
//     for pair in deposits.windows(2) {
//         let &[d1, d2] = pair else { panic!() };
//         if d1 != d2 {
//             g.extend_with_edges(&[(d1, d2, 1.0)]);
//         }
//     }

//     let x = bellman_ford(&g, deposits[0]).unwrap();
//     println!("{x:?}");
//     // println!("{:?}", x.distances[other_deposit.index()]);
//     // let mut path = vec![];
//     // let mut last = loan;
//     // while last != other_deposit {
//     //     path.push(last);
//     //     last = x.predecessors[last.index()].unwrap();
//     // }
//     // println!("{path:#?}");
// }

// const FREE: LiquidationCost = LiquidationCost {
//     change_in_effective_collateral: OrderedFloat(0.0),
//     change_in_required_collateral: OrderedFloat(0.0),
//     lost_equity: OrderedFloat(0.0),
// };

// const SWAP: LiquidationCost = LiquidationCost {
//     change_in_effective_collateral: OrderedFloat(0.0),
//     change_in_required_collateral: OrderedFloat(0.0),
//     lost_equity: OrderedFloat(-1.0),
// };

// const REPAY: LiquidationCost = LiquidationCost {
//     change_in_effective_collateral: OrderedFloat(0.0),
//     change_in_required_collateral: OrderedFloat(0.0),
//     lost_equity: OrderedFloat(0.0),
// };

// #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
// pub struct LiquidationCost {
//     collateral_impact: OrderedFloat<f64>,
//     equity_impact: OrderedFloat<f64>,
// }

// impl LiquidationCost {
//     fn efficiency(&self) -> OrderedFloat<f64> {
//         self.collateral_impact / self.equity_impact
//     }
// }

// impl PartialOrd for LiquidationCost {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         tada!()
//     }
// }

// impl Add for LiquidationCost {
//     type Output = LiquidationCost;

//     fn add(self, rhs: Self) -> Self::Output {
//         LiquidationCost {
//             change_in_effective_collateral: self.change_in_effective_collateral
//                 * rhs.change_in_effective_collateral,
//             change_in_required_collateral: self.change_in_required_collateral
//                 * rhs.change_in_required_collateral,
//             lost_equity: self.lost_equity + rhs.lost_equity,
//         }
//     }
// }

// impl FloatMeasure for LiquidationCost {
//     fn zero() -> Self {
//         LiquidationCost::default()
//     }

//     fn infinite() -> Self {
//         LiquidationCost {
//             change_in_effective_collateral: OrderedFloat(f64::INFINITY),
//             change_in_required_collateral: OrderedFloat(f64::INFINITY),
//             lost_equity: OrderedFloat(f64::INFINITY),
//         }
//     }
// }
