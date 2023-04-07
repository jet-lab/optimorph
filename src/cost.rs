use std::ops::{Add, Mul, Sub};

use ordered_float::OrderedFloat;
use pathfinding::num_traits::Zero;
use petgraph::algo::FloatMeasure;

/// Basic floating point number that implements all the traits necessary to be
/// used as a Size or a Cost
#[derive(Clone, Copy, Default, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Float(OrderedFloat<f64>);
impl Add for Float {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.add(rhs.0))
    }
}
impl Sub for Float {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.sub(rhs.0))
    }
}
impl Mul for Float {
    type Output = Float;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0.mul(rhs.0))
    }
}
impl<T: Into<f64>> From<T> for Float {
    fn from(value: T) -> Self {
        Self(OrderedFloat(value.into()))
    }
}
const ZERO: Float = Float(OrderedFloat(0.0));
impl Zero for Float {
    fn zero() -> Self {
        ZERO
    }
    fn is_zero(&self) -> bool {
        self == &ZERO
    }
}
impl FloatMeasure for Float {
    fn zero() -> Self {
        ZERO
    }

    fn infinite() -> Self {
        Float(OrderedFloat(f64::INFINITY))
    }
}

/// Determines the outcome of applying a morphism to its input object of the
/// provided Size. Outputs the Cost of this  and the Size of the target object
/// after application.
/// 
/// # NON_NEGATIVE
/// When true, the implementor promises that the Cost output will never be
/// negative. This guarantee is necessary for most shortest-path optimizations
/// algorithms to work properly, such as dijkstra.
/// 
/// The precise requirement is the following: For any two Sizes s1 and s2, the
/// following must be true:
/// * apply_non_negative(s1) + apply_non_negative(s2) >= s1
/// * apply_non_negative(s1) + apply_non_negative(s2) >= s2
/// 
/// This guarantee cannot be provided by the compiler. Implement this trait at
/// your own risk.
pub trait ApplyMorphism<Size = Float, Cost = Float, const NON_NEGATIVE: bool = false> {
    fn apply(&self, input: Size) -> MorphismOutput<Size, Cost>;
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DeductiveLinearCost {
    pub m: Float,
    pub b: Float,
}

impl ApplyMorphism<Float, Float> for DeductiveLinearCost {
    fn apply(&self, input: Float) -> MorphismOutput<Float, Float> {
        let cost = self.m * input + self.b;
        MorphismOutput {
            // todo should this be the default way to accumulate sizes?
            size: if cost > input { 0.into() } else { input - cost },
            cost,
        }
    }
}

/// Every morphism is always a cost of 1, for a basic unweighted graph.
pub struct ConstantCost;

impl ApplyMorphism<(), Float> for ConstantCost {
    fn apply(&self, input: ()) -> MorphismOutput<(), Float> {
        MorphismOutput {
            size: (),
            cost: 1.0.into(),
        }
    }
}

// impl<Size> ApplyMorphism<Size> for Size {
//     fn apply(&self, _: Size) -> Size {
//         *self
//     }
// }

// pub struct CostAccumulator {
//     /// The accumulated costs for a sequence of events
//     pub cost: Size,
//     /// The volume that has reached this point
//     pub size: Size,
//     //todo something to account for effective/required collateral or overall health
// }

/*
position with 90% cw -> position with 100% cw
cost of 1%
cv goes from 90 to 99


 */

pub struct MorphismOutput<Size = Float, Cost = Float> {
    pub size: Size,
    pub cost: Cost,
}

pub struct LiquidationCost {
    change_in_effective_collateral: Float,
    change_in_required_collateral: Float,
    lost_equity: Float,
}

unsafe trait NonNegative {} // this one is solid
unsafe trait NonDependent {} // this probably doesn't even make sense?
