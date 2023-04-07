#![cfg(test)]

use std::{fmt::Debug, rc::Rc};

use cost::{ApplyMorphism, DeductiveLinearCost, Float, MorphismOutput};
use morphism::Morphism;
use object::HasId;

use crate::{category::Category, pathfinder::optimize_single_path_with_dijkstra};

mod category;
mod cost;
mod morphism;
mod object;
mod pathfinder;
mod pet;
mod vertex;

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

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
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

self_identify!(PositionId);

enum PositionVariant {
    Only,
    FixedTermClaim(FixedTermClaim),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum FixedTermClaim {
    BorrowOrder,
    Loan,
    PastDueLoan,
}

fn get_positions() -> Category<PositionId, InstructionMeta> {
    let repay_loan = Instruction::new(
        PositionId::new(1),
        PositionId::new(0),
        InstructionMeta {
            name: "repay".to_owned(),
            logic: Rc::new(DeductiveLinearCost {
                m: 1.into(),
                b: 1.into(),
            }),
        },
    );
    let swap1to2 = Instruction::new(
        PositionId::new(1),
        PositionId::new(2),
        InstructionMeta {
            name: "repay".to_owned(),
            logic: Rc::new(DeductiveLinearCost {
                m: 10.into(),
                b: 10.into(),
            }),
        },
    );
    let swap2to1 = Instruction::new(
        PositionId::new(2),
        PositionId::new(1),
        InstructionMeta {
            name: "repay".to_owned(),
            logic: Rc::new(DeductiveLinearCost {
                m: 10.into(),
                b: 10.into(),
            }),
        },
    );

    vec![repay_loan, swap1to2, swap2to1].into()
}

#[test]
fn asdaf2() {
    let x = optimize_single_path_with_dijkstra(
        get_positions(),
        PositionId::new(2),
        PositionId::new(0),
        100.into(),
    );

    println!("{x:#?}");
}
