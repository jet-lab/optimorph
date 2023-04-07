use crate::morphism::{ApplyMorphism, MorphismOutput};

use super::Float;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DeductiveLinearCost {
    pub rate: Float,
    pub constant: Float,
}

impl ApplyMorphism<Float, Float> for DeductiveLinearCost {
    fn apply(&self, input: Float) -> MorphismOutput<Float, Float> {
        let cost = self.rate * input + self.constant;
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
    fn apply(&self, _input: ()) -> MorphismOutput<(), Float> {
        MorphismOutput {
            size: (),
            cost: 1.0.into(),
        }
    }
}
