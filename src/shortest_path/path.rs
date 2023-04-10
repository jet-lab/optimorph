use thiserror::Error;

use crate::{
    category::{HasId, Key},
    impls::Float,
    morphism::MorphismMeta,
    vertex::Vertex,
};

impl<Id, M, O, Size, Cost> TryFrom<(Vec<Vertex<Id, M, O, Size>>, Cost)>
    for Path<Id, M, O, Size, Cost>
where
    Id: Key,
    O: HasId<Id>,
    M: MorphismMeta,
    Size: Clone,
{
    type Error = InvalidPath;

    fn try_from(value: (Vec<Vertex<Id, M, O, Size>>, Cost)) -> Result<Self, Self::Error> {
        if value.0.len() == 0 {
            return Err(InvalidPath::EmptyPath);
        }
        let source = match &value.0[0] {
            Vertex::Object { inner, size } => (inner.id(), size.clone()),
            _ => return Err(InvalidPath::SourceIsNotObject),
        };
        let target = match &value.0[value.0.len() - 1] {
            Vertex::Object { inner, size } => (inner.id(), size.clone()),
            _ => return Err(InvalidPath::TargetIsNotObject),
        };
        Ok(Path {
            vertices: value.0,
            cost: value.1,
            source,
            target,
        })
    }
}

pub struct Path<Id, M, O = Id, Size = Float, Cost = Float>
where
    Id: Key,
    O: HasId<Id>,
    M: MorphismMeta,
{
    pub vertices: Vec<Vertex<Id, M, O, Size>>,
    pub cost: Cost,
    pub source: (Id, Size),
    pub target: (Id, Size),
}

#[derive(Error, Debug)]
pub enum InvalidPath {
    #[error("The source vertex of this path is not an object")]
    SourceIsNotObject,
    #[error("The target vertex of this path is not an object")]
    TargetIsNotObject,
    #[error("The lack of a path should be represented with None")]
    EmptyPath,
}
