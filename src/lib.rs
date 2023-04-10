pub mod category;
pub mod impls;
pub mod morphism;
pub mod shortest_path;
pub mod vertex;

#[cfg(test)]
mod test;

use std::convert::Infallible;

pub trait InfallibleResultExt<T> {
    fn safe_unwrap(self) -> T;
}

impl<T> InfallibleResultExt<T> for Result<T, Infallible> {
    fn safe_unwrap(self) -> T {
        self.unwrap()
    }
}
