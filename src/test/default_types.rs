use crate::category::Category;
use crate::impls::{DeductiveLinearCost, SimpleMorphism};
use crate::morphism::Morphism;
use crate::shortest_path::*;

type Instruction = Morphism<u8, InstructionMeta>;
type InstructionMeta = SimpleMorphism<String, DeductiveLinearCost>;

fn get_positions() -> Category<u8, InstructionMeta> {
    let repay_loan = Instruction::new(
        1,
        0,
        InstructionMeta {
            meta: "repay".to_owned(),
            logic: DeductiveLinearCost {
                rate: 1.into(),
                constant: 1.into(),
            },
        },
    );
    let swap1to2 = Instruction::new(
        1,
        2,
        InstructionMeta {
            meta: "repay".to_owned(),
            logic: DeductiveLinearCost {
                rate: 10.into(),
                constant: 10.into(),
            },
        },
    );
    let swap2to1 = Instruction::new(
        2,
        1,
        InstructionMeta {
            meta: "repay".to_owned(),
            logic: DeductiveLinearCost {
                rate: 10.into(),
                constant: 10.into(),
            },
        },
    );

    vec![repay_loan, swap1to2, swap2to1].into()
}

#[test]
fn dijkstra_pathfinding() {
    let x = shortest_single_path_with_accumulating_sizes(&get_positions(), 2, 0, 100.into());

    println!("{x:#?}");
}

#[test]
fn bellman_ford_petgraph() {
    let path =
        shortest_single_path_allowing_negative_cost(&get_positions(), 2, 0, 100.into()).unwrap();
    println!("{path:#?}");
}
