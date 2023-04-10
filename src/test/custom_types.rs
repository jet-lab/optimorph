use std::fmt::Debug;

use crate::category::{Category, HasId};
use crate::impls::Float;
use crate::morphism::{ApplyMorphism, Morphism, MorphismOutput};
use crate::shortest_path::*;

type MyMorph = Morphism<MyObjId, MyMorphMeta>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum MyMorphMeta {
    Static(Float),
    Dynamic(Float, Float),
}

impl ApplyMorphism<Float, Float, true> for MyMorphMeta {
    fn apply(&self, input: Float) -> MorphismOutput {
        match self.clone() {
            MyMorphMeta::Static(cost) => MorphismOutput { size: input, cost },
            MyMorphMeta::Dynamic(a, b) => MorphismOutput {
                cost: input * a,
                size: input * b,
            },
        }
    }
}

#[derive(Debug)]
pub struct MyObject {
    id: MyObjId,
    _data: &'static str,
}
impl HasId<MyObjId> for MyObject {
    fn id(&self) -> MyObjId {
        self.id
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct MyObjId([u8; 2]);

fn get_category() -> Category<MyObjId, MyMorphMeta, MyObject> {
    let mut category = Category::new();

    category
        .add_objects(vec![
            MyObject {
                id: MyObjId([0, 0]),
                _data: "whatever",
            },
            MyObject {
                id: MyObjId([0, 1]),
                _data: "whatever",
            },
            MyObject {
                id: MyObjId([1, 0]),
                _data: "whatever",
            },
            MyObject {
                id: MyObjId([1, 1]),
                _data: "whatever",
            },
        ])
        .unwrap();

    let step0 = MyMorph::new(
        MyObjId([0, 0]),
        MyObjId([0, 1]),
        MyMorphMeta::Static(100.into()),
    );
    let step0_reversed = MyMorph::new(
        MyObjId([0, 1]),
        MyObjId([0, 0]),
        MyMorphMeta::Static(13.into()),
    );
    let step1_static = MyMorph::new(
        MyObjId([0, 1]),
        MyObjId([1, 0]),
        MyMorphMeta::Static(100.into()),
    );
    let step1_dynamic = MyMorph::new(
        MyObjId([0, 1]),
        MyObjId([1, 0]),
        MyMorphMeta::Dynamic(5.into(), 11.into()),
    );
    let step2_static = MyMorph::new(
        MyObjId([1, 0]),
        MyObjId([1, 1]),
        MyMorphMeta::Static(100.into()),
    );
    let step2_dynamic = MyMorph::new(
        MyObjId([1, 0]),
        MyObjId([1, 1]),
        MyMorphMeta::Dynamic(7.into(), 11.into()),
    );

    category
        .add_morphisms(vec![
            step0,
            step0_reversed,
            step1_static,
            step1_dynamic,
            step2_static,
            step2_dynamic,
        ])
        .unwrap();

    category
}

/// prefers the step2_static because step1_dynamic increased the size which
/// increases the cost of step2_static
#[test]
fn dijkstra_pathfinding() {
    let (_path, cost) = shortest_single_path_with_accumulating_sizes(
        &get_category(),
        MyObjId([0, 0]),
        MyObjId([1, 1]),
        10.into(),
    )
    .unwrap();

    assert_eq!(cost, 250.into());
}

/// prefers the step2_dynamic because sizes are not accumulated and 70 < 100.
#[test]
fn bellman_ford_petgraph() {
    let (_path, cost) = shortest_single_path_allowing_negative_cost(
        &get_category(),
        MyObjId([0, 0]),
        MyObjId([1, 1]),
        10.into(),
    )
    .unwrap()
    .unwrap();

    assert_eq!(cost, 220.into());
}
