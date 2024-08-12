use crate::edges::*;

pub trait BackwardDynEdge<S, T, Ix, X>: StaticEdge
where
    S: DynamicNode<Ix, X>,
    T: StaticNode,
    Ix: Clone + Hash + Eq + PartialOrd,
    X: Clone,
{
    fn backward_corr(&self, src: &S, tgt: &T);
    fn update(&mut self, src: &S, tgt: &T);
}

impl BackwardDynEdge<Publisher, Article, Instant, f64> for Published {
    fn backward_corr(&self, src: &Publisher, tgt: &Article) {
        todo!()
    }

    fn update(&mut self, src: &Publisher, tgt: &Article) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
