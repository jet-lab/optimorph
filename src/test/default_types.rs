use crate::impls::{DeductiveLinearCost, Float, SimpleMorphism};
use crate::morphism::Morphism;
use crate::shortest_path::*;
use crate::vertex::Vertex;

type Transition = Morphism<u8, TransitionMeta>;
type TransitionMeta = SimpleMorphism<String, DeductiveLinearCost>;

fn transitions() -> Vec<Transition> {
    let from1to0_cheap = Transition::new(
        1,
        0,
        TransitionMeta {
            meta: "1to0_cheap".to_owned(),
            logic: DeductiveLinearCost {
                rate: 1.into(),
                constant: 1.into(),
            },
        },
    );
    let from1to0 = Transition::new(
        1,
        0,
        TransitionMeta {
            meta: "1to0".to_owned(),
            logic: DeductiveLinearCost {
                rate: 10.into(),
                constant: 10.into(),
            },
        },
    );
    let from1to2 = Transition::new(
        1,
        2,
        TransitionMeta {
            meta: "1to2".to_owned(),
            logic: DeductiveLinearCost {
                rate: 10.into(),
                constant: 10.into(),
            },
        },
    );
    let from2to1 = Transition::new(
        2,
        1,
        TransitionMeta {
            meta: "2to1".to_owned(),
            logic: DeductiveLinearCost {
                rate: 10.into(),
                constant: 10.into(),
            },
        },
    );

    vec![from1to0_cheap, from1to0, from1to2, from2to1]
}

#[test]
fn dijkstra_pathfinding() {
    let (path, cost) =
        shortest_single_path_with_accumulating_sizes(&transitions().into(), 2, 0, 100.into())
            .unwrap();
    let expected = expected([100, 0, 0]);
    assert_eq!(expected.len(), path.len());
    for (expected_vertex, actual_vertex) in expected.into_iter().zip(path) {
        assert_eq!(expected_vertex, actual_vertex);
    }
    assert_eq!(Float::from(1011), cost);
}

#[test]
fn bellman_ford_petgraph() {
    let (path, cost) =
        shortest_single_path_allowing_negative_cost(&transitions().into(), 2, 0, 100.into())
            .unwrap()
            .unwrap();
    let expected = expected([100, 100, 100]);
    assert_eq!(expected.len(), path.len());
    for (expected_vertex, actual_vertex) in expected.into_iter().zip(path) {
        assert_eq!(expected_vertex, actual_vertex);
    }
    assert_eq!(Float::from(1111), cost);
}

fn expected(sizes: [i32; 3]) -> Vec<Vertex<u8, TransitionMeta>> {
    let transitions = transitions();
    vec![
        Vertex::Object {
            inner: 2.into(),
            size: sizes[0].into(),
        },
        Vertex::Morphism {
            inner: transitions[3].clone(),
            input: sizes[0].into(),
        },
        Vertex::Object {
            inner: 1.into(),
            size: sizes[1].into(),
        },
        Vertex::Morphism {
            inner: transitions[0].clone(),
            input: sizes[1].into(),
        },
        Vertex::Object {
            inner: 0.into(),
            size: sizes[2].into(),
        },
    ]
}
