use crate::impls::{DeductiveLinearCost, Float, SimpleMorphism};
use crate::morphism::Morphism;
use crate::shortest_path::optimizer::Optimizer;
use crate::vertex::Vertex;
use crate::{shortest_path::*, InfallibleResultExt};

type MyMorph = Morphism<u8, MyMorphMeta>;
type MyMorphMeta = SimpleMorphism<String, DeductiveLinearCost>;

fn transitions() -> Vec<MyMorph> {
    let from1to0_cheap = MyMorph::new(
        1,
        0,
        MyMorphMeta {
            meta: "1to0_cheap".to_owned(),
            logic: DeductiveLinearCost {
                rate: 1.into(),
                constant: 1.into(),
            },
        },
    );
    let from1to0 = MyMorph::new(
        1,
        0,
        MyMorphMeta {
            meta: "1to0".to_owned(),
            logic: DeductiveLinearCost {
                rate: 10.into(),
                constant: 10.into(),
            },
        },
    );
    let from1to2 = MyMorph::new(
        1,
        2,
        MyMorphMeta {
            meta: "1to2".to_owned(),
            logic: DeductiveLinearCost {
                rate: 10.into(),
                constant: 10.into(),
            },
        },
    );
    let from2to1 = MyMorph::new(
        2,
        1,
        MyMorphMeta {
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
    let path = Accumulating
        .shortest_path(&transitions().into(), 2, 0, 100.into())
        .safe_unwrap()
        .unwrap();
    let expected = expected([100, 0, 0]);
    assert_eq!(expected.len(), path.vertices.len());
    for (expected_vertex, actual_vertex) in expected.into_iter().zip(path.vertices.iter()) {
        assert_eq!(&expected_vertex, actual_vertex);
    }
    assert_eq!(Float::from(1011), path.cost);
}

#[test]
fn bellman_ford_petgraph() {
    let path = Negatable
        .shortest_path(&transitions().into(), 2, 0, 100.into())
        .unwrap()
        .unwrap();
    let expected = expected([100, 0, 0]);
    assert_eq!(expected.len(), path.vertices.len());
    for (expected_vertex, actual_vertex) in expected.into_iter().zip(path.vertices.iter()) {
        assert_eq!(&expected_vertex, actual_vertex);
    }
    assert_eq!(Float::from(1011), path.cost);
}

fn expected(sizes: [i32; 3]) -> Vec<Vertex<u8, MyMorphMeta>> {
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
