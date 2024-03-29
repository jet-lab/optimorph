use std::collections::HashMap;

use petgraph::{
    algo::{bellman_ford, FloatMeasure},
    stable_graph::NodeIndex,
    Graph,
};
use thiserror::Error;

use super::path::{reapply, sum_petgraph, Path, WellFormedPath};
use crate::{
    category::{Category, Key, Object},
    morphism::ApplyMorphism,
    morphism::MorphismMeta,
    vertex::{LeanVertex, Vertex},
};

/// Finds the most cost-efficient path from source to target using the bellman
/// ford algorithm.
///
/// During shortest path optimization, the same `input_size` is used for all
/// morphisms. Accumulation (applying the output of one morphism as the input of
/// the next) is not supported by bellman-ford. However, once the shortest path
/// is found, it will be re-applied with accumulation. This means that there may
/// be some error in terms of which path has been selected, but there will be no
/// error in the values contained within the returned path.
#[allow(clippy::type_complexity)]
pub fn shortest_single_path_with_bellman_ford<
    const NON_NEGATIVE: bool,
    Id: Key,
    Obj: Object<Id>,
    M: MorphismMeta + ApplyMorphism<Size, Cost, NON_NEGATIVE>,
    Size: Clone,
    Cost: FloatMeasure,
>(
    category: &Category<Id, M, Obj>,
    source: Id,
    target: Id,
    input_size: Size,
) -> Result<Option<WellFormedPath<Id, M, Obj, Size, Cost>>, PathFindingError<Id>> {
    let mut general =
        shortest_multi_path_with_bellman_ford(category, source, &[target], input_size)?;
    if general.is_empty() {
        Ok(None)
    } else {
        // function below should guarantee return of at most a single item when
        // `targets` has a length of 1
        Ok(Some(general.swap_remove(0)))
    }
}

/// Finds the most cost-efficient paths from the source to each target using the
/// bellman ford algorithm. At most one path is returned for each target
/// provided, and it is possible that there is no path to the target.
///
/// During shortest path optimization, the same `input_size` is used for all
/// morphisms. Accumulation (applying the output of one morphism as the input of
/// the next) is not supported by bellman-ford. However, once the shortest paths
/// are found, they will be re-applied with accumulation. This means that there
/// may be some error in terms of which paths have been selected, but there will
/// be no error in the values contained within the returned paths.
#[allow(clippy::type_complexity)]
pub fn shortest_multi_path_with_bellman_ford<
    const NON_NEGATIVE: bool,
    Id: Key,
    Obj: Object<Id>,
    M: MorphismMeta + ApplyMorphism<Size, Cost, NON_NEGATIVE>,
    Size: Clone,
    Cost: FloatMeasure,
>(
    category: &Category<Id, M, Obj>,
    source: Id,
    targets: &[Id],
    input_size: Size,
) -> Result<Vec<WellFormedPath<Id, M, Obj, Size, Cost>>, PathFindingError<Id>> {
    if targets.is_empty() || category.get_object(&source).is_none() {
        return Ok(vec![]);
    }
    let cg = CategoryGraph::new(category, input_size.clone());
    let source_index = *cg
        .object_id_to_index
        .get(&source)
        .ok_or(MissingObject(source.clone()))?;
    let paths = bellman_ford(&cg.graph, source_index).map_err(|_| NegativeCycle)?;

    let mut resolved_paths = vec![];
    'outer: for target in targets {
        let target_index = *cg
            .object_id_to_index
            .get(target)
            .ok_or(MissingObject(target.clone()))?;
        let mut work_back = target_index;
        let mut path = vec![];
        while work_back != source_index {
            path.push(work_back);
            if paths.predecessors[work_back.index()].is_none() {
                continue 'outer;
            }
            work_back = paths.predecessors[work_back.index()].unwrap();
        }
        path.push(work_back);
        path.reverse();
        let unaccumulated_vertices = path
            .into_iter()
            .map(|idx| cg.index_to_vertex.get(&idx).cloned())
            .collect::<Option<Vec<_>>>()
            .unwrap() // todo
            .into_iter()
            .map(|v| Vertex::from(v, category))
            .collect::<Vec<_>>();
        // let cost = paths.distances[target_index.index()]; // incorrect: based on unaccumulated morphism outputs
        let (vertices, costs) = reapply(unaccumulated_vertices, input_size.clone());
        resolved_paths.push(WellFormedPath(Path {
            vertices: vertices.try_into().expect("`continue 'outer` avoids this"),
            cost: sum_petgraph(costs),
        }));
    }

    Ok(resolved_paths)
}

#[derive(Error, Debug)]
pub enum PathFindingError<Id: std::fmt::Debug> {
    #[error("The object could not be identified as a vertex in the underlying graph")]
    MissingObject(Id),
    #[error("There is a cycle of negative costs that prevent shortest path optimization")]
    NegativeCycle,
}
use PathFindingError::*;

struct CategoryGraph<Id, M, Size, Cost, const NON_NEGATIVE: bool> {
    graph: Graph<LeanVertex<Id, M, Size>, Cost>,
    object_id_to_index: HashMap<Id, NodeIndex>,
    index_to_vertex: HashMap<NodeIndex, LeanVertex<Id, M, Size>>,
}

impl<Id, M, Size, Cost, const NON_NEGATIVE: bool> CategoryGraph<Id, M, Size, Cost, NON_NEGATIVE>
where
    Id: Key,
    M: MorphismMeta + ApplyMorphism<Size, Cost, NON_NEGATIVE>,
    Size: Clone,
    Cost: FloatMeasure,
{
    fn new<Obj: Object<Id>>(
        category: &Category<Id, M, Obj>,
        input_size: Size,
    ) -> CategoryGraph<Id, M, Size, Cost, NON_NEGATIVE> {
        let mut graph = Graph::new();
        let (objects, morphisms, _) = category.clone().destruct();
        let mut object_id_to_index = HashMap::new();
        let mut morphism_to_index = HashMap::new();
        let mut index_to_vertex = HashMap::new();
        for object in objects.into_values() {
            let index = graph.add_node(LeanVertex::Object {
                inner: object.id(),
                size: input_size.clone(),
            });
            object_id_to_index.insert(object.id(), index);
            index_to_vertex.insert(
                index,
                LeanVertex::Object {
                    inner: object.id(),
                    size: input_size.clone(),
                },
            );
        }
        for morphism in &morphisms {
            let index = graph.add_node(LeanVertex::Morphism {
                inner: morphism.clone(),
                input: input_size.clone(),
            });
            morphism_to_index.insert(morphism.clone(), index);
            index_to_vertex.insert(
                index,
                LeanVertex::Morphism {
                    inner: morphism.clone(),
                    input: input_size.clone(),
                },
            );
        }
        for morphism in morphisms {
            let index = *morphism_to_index.get(&morphism).unwrap();
            graph.extend_with_edges(&[
                (
                    *object_id_to_index.get(&morphism.source).unwrap(),
                    index,
                    Cost::zero(),
                ),
                (
                    index,
                    *object_id_to_index.get(&morphism.target).unwrap(),
                    morphism.metadata.apply(input_size.clone()).cost,
                ),
            ]);
        }

        CategoryGraph {
            graph,
            object_id_to_index,
            index_to_vertex,
        }
    }
}
