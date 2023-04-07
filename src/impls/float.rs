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