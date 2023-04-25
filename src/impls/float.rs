use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign},
};

use ordered_float::OrderedFloat;
use pathfinding::num_traits::Zero;
use petgraph::algo::FloatMeasure;

pub const ZERO: Float = Float(OrderedFloat(0.0));
pub const ONE: Float = Float(OrderedFloat(1.0));
pub const INFINITY: Float = Float(OrderedFloat(f64::INFINITY));

/// Basic floating point number that implements all the traits necessary to be
/// used as a Size or a Cost
#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Float(OrderedFloat<f64>);

impl Float {
    pub fn to_f64(self) -> f64 {
        self.0 .0
    }

    pub fn abs(self) -> Float {
        if self < ZERO {
            -self
        } else {
            self
        }
    }
}

pub const fn float(x: f64) -> Float {
    Float(OrderedFloat(x))
}

impl Debug for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Float({:?})", self.0)
    }
}

impl Add for Float {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.add(rhs.0))
    }
}
impl Add for &Float {
    type Output = Float;

    fn add(self, rhs: Self) -> Self::Output {
        Float(self.0.add(rhs.0))
    }
}

impl AddAssign for Float {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Sub for Float {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.sub(rhs.0))
    }
}
impl Sub for &Float {
    type Output = Float;

    fn sub(self, rhs: Self) -> Self::Output {
        Float(self.0.sub(rhs.0))
    }
}

impl SubAssign for Float {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl Neg for Float {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}
impl Neg for &Float {
    type Output = Float;

    fn neg(self) -> Self::Output {
        Float(-self.0)
    }
}

impl Mul for Float {
    type Output = Float;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0.mul(rhs.0))
    }
}
impl Mul for &Float {
    type Output = Float;

    fn mul(self, rhs: Self) -> Self::Output {
        Float(self.0.mul(rhs.0))
    }
}

impl Div for Float {
    type Output = Float;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0.div(rhs.0))
    }
}
impl Div for &Float {
    type Output = Float;

    fn div(self, rhs: Self) -> Self::Output {
        Float(self.0.div(rhs.0))
    }
}

impl<T: Into<f64>> From<T> for Float {
    fn from(value: T) -> Self {
        Self(OrderedFloat(value.into()))
    }
}

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
