use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use ordered_float::OrderedFloat;
use pathfinding::num_traits::Zero;
use petgraph::algo::FloatMeasure;

pub const ZERO: Float = float(0.0);
pub const ONE: Float = float(1.0);
pub const INFINITY: Float = float(f64::INFINITY);

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

impl_math!(Add::add, Sub::sub, Mul::mul, Div::div);
impl_math! {
    AddAssign::add_assign => add,
    SubAssign::sub_assign => sub,
    MulAssign::mul_assign => mul,
    DivAssign::div_assign => div
}

macro_rules! impl_math {
    ($($Trait:ident::$method:ident),*$(,)*) => {
        $(
            impl_math!($Trait::$method for Float:inner | Float:inner);
            impl_math!($Trait::$method for Float:inner | &Float:inner);
            impl_math!($Trait::$method for Float:inner | f64:to_ordered);
            impl_math!($Trait::$method for Float:inner | &f64:to_ordered);
            impl_math!($Trait::$method for f64:to_ordered | Float:inner);
            impl_math!($Trait::$method for f64:to_ordered | &Float:inner);
            impl_math!($Trait::$method for &Float:inner | Float:inner);
            impl_math!($Trait::$method for &Float:inner | &Float:inner);
            impl_math!($Trait::$method for &Float:inner | f64:to_ordered);
            impl_math!($Trait::$method for &Float:inner | &f64:to_ordered);
            impl_math!($Trait::$method for &f64:to_ordered | Float:inner);
            impl_math!($Trait::$method for &f64:to_ordered | &Float:inner);
        )*
    };
    ($($Trait:ident::$method:ident => $sub_method:ident),*$(,)*) => {
        $(
            impl_math!($Trait::$method for Float:inner | Float:inner => $sub_method);
            impl_math!($Trait::$method for Float:inner | &Float:inner => $sub_method);
            impl_math!($Trait::$method for Float:inner | f64:to_ordered => $sub_method);
            impl_math!($Trait::$method for Float:inner | &f64:to_ordered => $sub_method);
        )*
    };
    ($Trait:ident::$method:ident for $For:ty$(:$l_accessor:ident)? | $Rhs:ty$(:$r_accessor:ident)?) => {
        impl $Trait<$Rhs> for $For {
            type Output = Float;

            fn $method(self, rhs: $Rhs) -> Self::Output {
                Float(self$(.$l_accessor())?.$method(rhs$(.$r_accessor())?))
            }
        }
    };
    ($Trait:ident::$method:ident for $For:ty$(:$l_accessor:ident)? | $Rhs:ty$(:$r_accessor:ident)? => $sub_method:ident) => {
        impl $Trait<$Rhs> for $For {
            fn $method(&mut self, rhs: $Rhs) {
                *self = self.$sub_method(rhs).clone()
            }
        }
    };
}
use impl_math;

/// keep this private, only for macro
trait ToOrdered {
    fn to_ordered(self) -> OrderedFloat<f64>;
}
impl ToOrdered for f64 {
    fn to_ordered(self) -> OrderedFloat<f64> {
        self.into()
    }
}
impl Float {
    /// keep this private, only for macro
    fn inner(&self) -> OrderedFloat<f64> {
        self.0
    }
}
