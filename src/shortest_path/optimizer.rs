use std::ops::Div;

use crate::{
    category::{Category, Key, Object},
    collections::Replace,
    morphism::MorphismMeta,
};

use super::path::WellFormedPath;

pub trait Optimizer<M, Size, Cost, const NON_NEGATIVE: bool = false>
where
    M: MorphismMeta,
    Size: Clone,
{
    type Error<Id: Key, Obj>;

    /// Returns the cheapest path from source to target
    #[allow(clippy::type_complexity)]
    fn shortest_path<Id, Obj>(
        category: &Category<Id, M, Obj>,
        source: Id,
        target: Id,
        input_size: Size,
    ) -> Result<Option<WellFormedPath<Id, M, Obj, Size, Cost>>, Self::Error<Id, Obj>>
    where
        Id: Key,
        Obj: Object<Id>;

    /// Returns the cheapest path from each source to each target
    #[allow(clippy::type_complexity)]
    fn shortest_paths<Id, Obj>(
        category: &Category<Id, M, Obj>,
        sources: Vec<(Id, Size)>,
        targets: Vec<Id>,
    ) -> Result<Vec<WellFormedPath<Id, M, Obj, Size, Cost>>, Self::Error<Id, Obj>>
    where
        Id: Key,
        Obj: Object<Id>,
    {
        let mut results = vec![];
        for (source, input) in sources {
            for target in targets.clone() {
                results.push(Self::shortest_path(
                    category,
                    source.clone(),
                    target,
                    input.clone(),
                ));
            }
        }
        results
            .into_iter()
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
    /// The calculated Score replaces the Cost in the path. Likewise you can
    /// provide a score calculator that returns whatever you want:
    /// - the original Cost value.
    /// - a transformation of the original Cost value with the same type.
    /// - a value with any type, as long as it implements the required trait
    ///   bounds for a cost.
    fn ranked_paths<Id, Obj, Score, PathRet, Calculator>(
        category: &Category<Id, M, Obj>,
        sources: Vec<(Id, Size)>,
        targets: Vec<Id>,
        calculate_score: Calculator,
    ) -> Result<Vec<PathRet::With<Score>>, Self::Error<Id, Obj>>
    where
        Id: Key,
        Obj: Object<Id>,
        Score: Ord + Clone,
        PathRet: From<WellFormedPath<Id, M, Obj, Size, Cost>> + Replace<Cost>,
        Calculator: Fn(&PathRet) -> Score,
    {
        let mut paths = Self::shortest_paths(category, sources, targets)?
            .into_iter()
            .map(PathRet::from)
            .map(|path| {
                let score = calculate_score(&path);
                path.replace(score).0
            })
            .collect::<Vec<PathRet::With<Score>>>();

        paths.sort_by_key(|p| p.read().clone());

        Ok(paths)
    }
}

/// Common score calculations for MorphismOptimizer::ranked_paths
pub mod score {
    use crate::vertex::Vertex;

    use super::*;

    /// This just passes along the original cost as the score
    pub fn cost<Id, M, Obj, Size, Cost>(path: &WellFormedPath<Id, M, Obj, Size, Cost>) -> Cost
    where
        Id: Key,
        Obj: Object<Id>,
        M: MorphismMeta,
        Size: Clone,
        Cost: Clone,
    {
        path.0.cost.clone()
    }

    /// This calculates the ratio of cost divided by input size. It works if
    /// Size, Cost, and Score are all the same numeric type. More generally,
    /// this works if Cost implements Div<Size, Output = Score>
    pub fn cost_per_input<Id, M, Obj, Size, Cost, Score>(
        path: &WellFormedPath<Id, M, Obj, Size, Cost>,
    ) -> Score
    where
        Id: Key,
        Obj: Object<Id>,
        M: MorphismMeta,
        Cost: Div<Size, Output = Score> + Clone,
        Size: Clone,
    {
        let Vertex::Morphism { input, .. } = &path.0.vertices.get(1).unwrap() else { unreachable!() };
        path.0.cost.clone() / input.clone()
    }
}
