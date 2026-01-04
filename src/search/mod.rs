use std::{cmp::Eq, collections::HashMap, hash::Hash};

mod median;
pub use median::*;

mod basic;
pub use basic::*;

mod dijkstra;
pub use dijkstra::dijkstra;

mod a_star;
pub use a_star::a_star;

pub trait Graph {
    type Node: Eq + Hash + Clone;
    fn height(&self) -> usize;
    fn width(&self) -> usize;
    fn moves(&self, node: &Self::Node) -> Vec<Self::Node>;
    fn is_done(&self, node: &Self::Node) -> bool;
}

pub trait Weighted: Graph {
    fn weight(&self, cur: &Self::Node, next: &Self::Node) -> usize;
}

pub fn get_path<S: PartialEq + Eq + Hash + Clone>(
    moves: HashMap<S, S>,
    end: S,
    start: &S,
) -> Vec<S> {
    let mut found = Vec::new();
    found.push(end);
    while let Some(node) = moves.get(found.last().unwrap()) {
        found.push(node.clone());
        if node == start {
            break;
        }
    }
    found.reverse();
    found
}
