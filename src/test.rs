#![cfg(test)]

use std::{fmt::Debug, rc::Rc};

use crate::category::HasId;
use crate::impls::{DeductiveLinearCost, Float};
use crate::morphism::{ApplyMorphism, Morphism, MorphismOutput};
use crate::shortest_path::pet::shortest_single_path_with_bellman_ford;
use crate::{category::Category, shortest_path::pathfinder::shortest_single_path_with_dijkstra};

type Instruction = Morphism<PositionId, InstructionMeta>;

#[derive(Clone)]
struct InstructionMeta {
    name: String,
    logic: Rc<dyn ApplyMorphism>,
}

impl Debug for InstructionMeta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InstructionMeta")
            .field("name", &self.name)
            .finish()
    }
}

impl PartialEq for InstructionMeta {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for InstructionMeta {}
impl std::hash::Hash for InstructionMeta {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
impl ApplyMorphism for InstructionMeta {
    fn apply(&self, input: Float) -> MorphismOutput {
        self.logic.apply(input)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct PositionId {
    position_token_mint: u64,
    variant: u8,
}
impl PositionId {
    pub fn new(position_token_mint: u64) -> Self {
        PositionId {
            position_token_mint,
            variant: 0,
        }
    }
}

impl HasId<PositionId> for PositionId {
    fn id(&self) -> PositionId {
        *self
    }
}

// enum PositionVariant {
//     Only,
//     FixedTermClaim(FixedTermClaim),
// }

// #[derive(Clone, Debug, PartialEq, Eq, Hash)]
// enum FixedTermClaim {
//     BorrowOrder,
//     Loan,
//     PastDueLoan,
// }

// pub struct LiquidationCost {
//     change_in_effective_collateral: Float,
//     change_in_required_collateral: Float,
//     lost_equity: Float,
// }

fn get_positions() -> Category<PositionId, InstructionMeta> {
    let repay_loan = Instruction::new(
        PositionId::new(1),
        PositionId::new(0),
        InstructionMeta {
            name: "repay".to_owned(),
            logic: Rc::new(DeductiveLinearCost {
                rate: 1.into(),
                constant: 1.into(),
            }),
        },
    );
    let swap1to2 = Instruction::new(
        PositionId::new(1),
        PositionId::new(2),
        InstructionMeta {
            name: "repay".to_owned(),
            logic: Rc::new(DeductiveLinearCost {
                rate: 10.into(),
                constant: 10.into(),
            }),
        },
    );
    let swap2to1 = Instruction::new(
        PositionId::new(2),
        PositionId::new(1),
        InstructionMeta {
            name: "repay".to_owned(),
            logic: Rc::new(DeductiveLinearCost {
                rate: 10.into(),
                constant: 10.into(),
            }),
        },
    );

    vec![repay_loan, swap1to2, swap2to1].into()
}

#[test]
fn dijkstra_pathfinding() {
    let x = shortest_single_path_with_dijkstra(
        get_positions(),
        PositionId::new(2),
        PositionId::new(0),
        100.into(),
    );

    println!("{x:#?}");
}

#[test]
fn bellman_ford_petgraph() {
    let path = shortest_single_path_with_bellman_ford(
        get_positions(),
        PositionId::new(2),
        PositionId::new(0),
        100.into(),
    );
    println!("{path:#?}");
}
