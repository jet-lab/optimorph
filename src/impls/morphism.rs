use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use crate::morphism::{ApplyMorphism, MorphismMeta, MorphismOutput};

use super::Float;

/// This can be used as the metadata field in Morphism.
#[derive(Debug)]
pub struct SimpleMorphism<Meta = String, Logic = ConstantCost>
where
    Meta: MorphismMeta,
{
    /// This should be sufficiently unique to distinguish the morphism from
    /// other morphisms that have the same source and target.
    pub meta: Meta,
    pub logic: Logic,
}

impl<Meta: MorphismMeta, Logic> PartialEq for SimpleMorphism<Meta, Logic> {
    fn eq(&self, other: &Self) -> bool {
        self.meta == other.meta
    }
}
impl<Meta: MorphismMeta, Logic> Eq for SimpleMorphism<Meta, Logic> {}
impl<Meta: MorphismMeta, Logic> Hash for SimpleMorphism<Meta, Logic> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.meta.hash(state);
    }
}

impl<Meta: MorphismMeta> SimpleMorphism<Meta> {
    pub fn new(meta: Meta) -> Self {
        Self {
            meta,
            logic: ConstantCost,
        }
    }
}

impl<Size, Cost, Meta, Logic, const NON_NEGATIVE: bool> ApplyMorphism<Size, Cost, NON_NEGATIVE>
    for SimpleMorphism<Meta, Logic>
where
    Meta: MorphismMeta,
    Logic: ApplyMorphism<Size, Cost, NON_NEGATIVE>,
{
    fn apply(&self, input: Size) -> MorphismOutput<Size, Cost> {
        self.logic.apply(input)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DeductiveLinearCost {
    pub rate: Float,
    pub constant: Float,
}

impl ApplyMorphism<Float, Float, true> for DeductiveLinearCost {
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
#[derive(Clone, Debug)]
pub struct ConstantCost;

impl ApplyMorphism<(), Float, true> for ConstantCost {
    fn apply(&self, _input: ()) -> MorphismOutput<(), Float> {
        MorphismOutput {
            size: (),
            cost: 1.0.into(),
        }
    }
}
