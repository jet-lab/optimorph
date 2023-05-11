pub mod category;
pub mod collections;
pub mod impls;
pub mod morphism;
pub mod shortest_path;
pub mod vertex;

#[cfg(test)]
mod test;

use std::{
    convert::Infallible,
    fmt::{Debug, Display},
};

pub trait InfallibleResultExt<T> {
    fn safe_unwrap(self) -> T;
}

impl<T> InfallibleResultExt<T> for Result<T, Infallible> {
    fn safe_unwrap(self) -> T {
        self.unwrap()
    }
}

pub struct DebugWithDisplay<T>(pub T);
impl<T: Display> Debug for DebugWithDisplay<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}
