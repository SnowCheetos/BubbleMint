use crate::edges::*;
use crate::nodes::statics::StaticNode;

pub trait StaticEdge: Clone + Send + Sync {
    type Source: StaticNode;
    type Target: StaticNode;

    fn name(&self) -> &'static str;
}