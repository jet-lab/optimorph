#![cfg(test)]

use std::rc::Rc;

use cost::{ApplyMorphism, DeductiveLinearCost, Size};
use morphism::Morphism;
use object::Object;
use pathfinding::prelude::dijkstra;
use vertex::Vertex;

use crate::category::Category;

mod category;
mod cost;
mod morphism;
mod object;
mod pet;
mod vertex;

type Position = Object<PositionId>;
type Instruction = Morphism<PositionId, InstructionMeta>;

#[derive(Clone, Debug)]
struct InstructionMeta {
    name: String,
    logic: Rc<dyn ApplyMorphism<Size>>,
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
impl ApplyMorphism<Size> for InstructionMeta {
    fn apply(&self, input: Size) -> cost::MorphismOutput<Size> {
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
    let loan = Position::new_terminal(PositionId::new(0), 1);
    let deposit = Position::new(PositionId::new(1), 1);
    let other_deposit = Position::new(PositionId::new(2), 1);
    let repay_loan = Instruction {
        source: PositionId::new(1),
        target: PositionId::new(0),
        metadata: InstructionMeta {
            name: "repay".to_owned(),
            logic: Rc::new(DeductiveLinearCost { m: 1, b: 1 }),
        },
        input_size: 0,
    };
    let swap1to2 = Instruction {
        source: PositionId::new(1),
        target: PositionId::new(2),
        metadata: InstructionMeta {
            name: "repay".to_owned(),
            logic: Rc::new(DeductiveLinearCost { m: 10, b: 10 }),
        },
        input_size: 0,
    };
    let swap2to1 = Instruction {
        source: PositionId::new(2),
        target: PositionId::new(1),
        metadata: InstructionMeta {
            name: "repay".to_owned(),
            logic: Rc::new(DeductiveLinearCost { m: 10, b: 10 }),
        },
        input_size: 0,
    };

    let category = Category::of(
        vec![loan, deposit, other_deposit],
        vec![repay_loan, swap1to2, swap2to1],
    )
    .unwrap();

    category
}

#[test]
fn asdaf2() {
    let category = get_positions();
    let other_deposit = Vertex::Object(category.get_object(&PositionId::new(2)).unwrap().clone());
    let x = dijkstra(
        &other_deposit,
        move |n| n.successors(&category),
        move |n| n.is_object_with_id(&PositionId::new(0)),
    );

    println!("{x:#?}");
}

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
