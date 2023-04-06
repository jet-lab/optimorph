/// todo generify
pub type Size = i64;

pub trait ApplyMorphism<Cost>: std::fmt::Debug {
    fn apply(&self, input: Size) -> MorphismOutput<Cost>;
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DeductiveLinearCost {
    pub m: Size,
    pub b: Size,
}

impl ApplyMorphism<Size> for DeductiveLinearCost {
    fn apply(&self, input: Size) -> MorphismOutput<Size> {
        let cost = self.m * input + self.b;
        MorphismOutput {
            size: if cost > input { 0 } else { input - cost },
            cost,
        }
    }
}

// impl<Size> ApplyMorphism<Size> for Size {
//     fn apply(&self, _: Size) -> Size {
//         *self
//     }
// }

pub struct CostAccumulator {
    /// The accumulated costs for a sequence of events
    pub cost: Size,
    /// The volume that has reached this point
    pub size: Size,
    //todo something to account for effective/required collateral or overall health
}

/*
position with 90% cw -> position with 100% cw
cost of 1%
cv goes from 90 to 99


 */

pub struct MorphismOutput<Cost> {
    pub size: Size,
    pub cost: Cost,
}

pub struct LiquidationCost {
    change_in_effective_collateral: Size,
    change_in_required_collateral: Size,
    lost_equity: Size,
}

unsafe trait NonNegative {} // this one is solid
unsafe trait NonDependent {} // this probably doesn't even make sense?
