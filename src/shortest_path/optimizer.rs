use std::ops::Div;

use crate::{
    category::{Category, HasId, Key},
    morphism::MorphismMeta,
};

use super::path::Path;

pub trait Optimizer<M, Size, Cost, const NON_NEGATIVE: bool = false>
where
    M: MorphismMeta,
{
    type Error<Id: Key, O>;

    /// Returns the cheapest path from source to target
    fn shortest_path<Id, O>(
        category: &Category<Id, M, O>,
        source: Id,
        target: Id,
        input_size: Size,
    ) -> Result<Option<Path<Id, M, O, Size, Cost>>, Self::Error<Id, O>>
    where
        Id: Key,
        O: HasId<Id>;

    /// Returns the cheapest path from each source to each target
    fn shortest_paths<Id, O>(
        category: &Category<Id, M, O>,
        sources: Vec<(Id, Size)>,
        target: Vec<Id>,
    ) -> Result<Vec<Path<Id, M, O, Size, Cost>>, Self::Error<Id, O>>
    where
        Id: Key,
        O: HasId<Id>,
    {
        sources
            .into_iter()
            .zip(target)
            .map(|((source, input), target)| Self::shortest_path(category, source, target, input))
            .filter_map(|x| match x {
                Ok(Some(ok)) => Some(Ok(ok)),
                Ok(None) => None,
                Err(e) => Some(Err(e)),
            })
            .collect::<Result<Vec<_>, _>>()
    }

    /// Returns the cheapest path from each source to each target, sorted by
    /// Score.
    ///
    /// Score is a Cost. This allows you to use the original cost, or to
    /// transform the cost into a different type for comparison between paths.
    fn ranked_paths<Id, O, Score>(
        category: &Category<Id, M, O>,
        sources: Vec<(Id, Size)>,
        target: Vec<Id>,
        calculate_score: fn(&Path<Id, M, O, Size, Cost>) -> Score,
    ) -> Result<Vec<Path<Id, M, O, Size, Score>>, Self::Error<Id, O>>
    where
        Id: Key,
        O: HasId<Id>,
        Score: Ord + Clone,
    {
        let mut paths = Self::shortest_paths(category, sources, target)?
            .into_iter()
            .map(|path| Path {
                cost: calculate_score(&path),
                vertices: path.vertices,
                source: path.source,
                target: path.target,
            })
            .collect::<Vec<_>>();

        paths.sort_by_key(|p| p.cost.clone());

        Ok(paths)
    }
}

/// Common score calculations for MorphismOptimizer::ranked_paths
pub mod score {
    use super::*;

    /// This just passes along the original cost as the score
    pub fn cost<Id, M, O, Size, Cost>(path: Path<Id, M, O, Size, Cost>) -> Cost
    where
        Id: Key,
        O: HasId<Id>,
        M: MorphismMeta,
    {
        path.cost
    }

    /// This calculates the ratio of cost divided by input size. It works if
    /// Size, Cost, and Score are all the same numeric type. More generally,
    /// this works if Cost implements Div<Size, Output = Score>
    pub fn cost_per_input<Id, M, O, Size, Cost, Score>(path: Path<Id, M, O, Size, Cost>) -> Score
    where
        Id: Key,
        O: HasId<Id>,
        M: MorphismMeta,
        Cost: Div<Size, Output = Score>,
    {
        path.cost / path.source.1
    }
}
