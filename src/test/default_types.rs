use crate::category::Category;
use crate::impls::{DeductiveLinearCost, SimpleMorphism, SimpleObject};
use crate::morphism::Morphism;
use crate::shortest_path::*;

type Instruction = Morphism<PositionId, InstructionMeta>;
type InstructionMeta = SimpleMorphism<String, DeductiveLinearCost>;
type PositionId = SimpleObject<u8>;

fn get_positions() -> Category<PositionId, InstructionMeta> {
    let repay_loan = Instruction::new(
        PositionId::new(1),
        PositionId::new(0),
        InstructionMeta {
            meta: "repay".to_owned(),
            logic: DeductiveLinearCost {
                rate: 1.into(),
                constant: 1.into(),
            },
        },
    );
    let swap1to2 = Instruction::new(
        PositionId::new(1),
        PositionId::new(2),
        InstructionMeta {
            meta: "repay".to_owned(),
            logic: DeductiveLinearCost {
                rate: 10.into(),
                constant: 10.into(),
            },
        },
    );
    let swap2to1 = Instruction::new(
        PositionId::new(2),
        PositionId::new(1),
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
    let x = shortest_single_path_with_accumulating_sizes(
        get_positions(),
        PositionId::new(2),
        PositionId::new(0),
        100.into(),
    );

    println!("{x:#?}");
}

#[test]
fn bellman_ford_petgraph() {
    let path = shortest_single_path_allowing_negative_cost(
        get_positions(),
        PositionId::new(2),
        PositionId::new(0),
        100.into(),
    );
    println!("{path:#?}");
}
