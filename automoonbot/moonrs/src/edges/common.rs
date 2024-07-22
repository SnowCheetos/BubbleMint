use crate::edges::*;

#[derive(Debug)]
pub struct TestEdge {
    pub(super) src_index: NodeIndex,
    pub(super) tgt_index: NodeIndex,
    pub(super) value: f64,
    pub(super) covariance: Option<f64>,
    pub(super) correlation: Option<f64>,
}

impl TestEdge {
    pub fn new(
        src_index: NodeIndex,
        tgt_index: NodeIndex,
        src_node: &dyn StaticNode,
        tgt_node: &dyn StaticNode,
    ) -> Self {
        let value = Self::difference(src_node, tgt_node);
        TestEdge {
            src_index,
            tgt_index,
            value,
            covariance: None,
            correlation: None,
        }
    }

    pub fn try_new(
        src_index: NodeIndex,
        tgt_index: NodeIndex,
        src_node: &dyn StaticNode,
        tgt_node: &dyn StaticNode,
    ) -> Option<Self> {
        if src_index == tgt_index {
            return None;
        }
        let value = Self::difference(src_node, tgt_node);
        if value > 0.0 {
            Some(TestEdge {
                src_index,
                tgt_index,
                value,
                covariance: None,
                correlation: None,
            })
        } else {
            None
        }
    }

    pub fn difference(src_node: &dyn StaticNode, tgt_node: &dyn StaticNode) -> f64 {
        (src_node.value() - tgt_node.value()).abs()
    }
}
