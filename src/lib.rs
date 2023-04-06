#![cfg(test)]

use std::{marker::PhantomData, rc::Rc};

use cost::{ApplyMorphism, Float, DeductiveLinearCost, MorphismOutput};
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
    logic: Rc<dyn ApplyMorphism>,
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
    let loan = Position::new(PositionId::new(0), 1.into());
    let deposit = Position::new(PositionId::new(1), 1.into());
    let other_deposit = Position::new(PositionId::new(2), 1.into());
    let repay_loan = Instruction {
        source: PositionId::new(1),
        target: PositionId::new(0),
        metadata: InstructionMeta {
            name: "repay".to_owned(),
            logic: Rc::new(DeductiveLinearCost {
                m: 1.into(),
                b: 1.into(),
            }),
        },
        input_size: 0.into(),
        _phantom: PhantomData,
    };
    let swap1to2 = Instruction {
        source: PositionId::new(1),
        target: PositionId::new(2),
        metadata: InstructionMeta {
            name: "repay".to_owned(),
            logic: Rc::new(DeductiveLinearCost {
                m: 10.into(),
                b: 10.into(),
            }),
        },
        input_size: 0.into(),
        _phantom: PhantomData,
    };
    let swap2to1 = Instruction {
        source: PositionId::new(2),
        target: PositionId::new(1),
        metadata: InstructionMeta {
            name: "repay".to_owned(),
            logic: Rc::new(DeductiveLinearCost {
                m: 10.into(),
                b: 10.into(),
            }),
        },
        input_size: 0.into(),
        _phantom: PhantomData,
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



