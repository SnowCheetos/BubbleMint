pub mod data;
pub mod edges;
pub mod graph;
pub mod nodes;
pub mod utils;

use std::{
    collections::HashSet,
    hash::Hash,
    time::{Duration, Instant},
};
#[macro_use]
extern crate lazy_static;
#[macro_use(defer)]
extern crate scopeguard;
extern crate nalgebra as na;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
