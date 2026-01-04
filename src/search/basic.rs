use std::collections::{HashMap, VecDeque};

use super::{Graph, get_path};

pub fn dfs<G: Graph>(start: &G::Node, graph: &G) -> Option<Vec<G::Node>> {
    let mut path = HashMap::new();
    let mut to_visit = vec![start.clone()];
    while let Some(node) = to_visit.pop() {
        if path.contains_key(&node) {
            continue;
        }
        if graph.is_done(&node) {
            return Some(get_path(path, node, start));
        }
        for next_move in graph.moves(&node) {
            to_visit.push(next_move.clone());
            path.insert(next_move.clone(), node.clone());
        }
    }
    None
}

pub fn bfs<G: Graph>(start: &G::Node, graph: &G) -> Option<Vec<G::Node>> {
    let mut path = HashMap::new();
    let mut to_visit = VecDeque::new();
    to_visit.push_front(start.clone());
    while let Some(node) = to_visit.pop_front() {
        if graph.is_done(&node) {
            return Some(get_path(path, node, start));
        }
        for next_move in graph.moves(&node) {
            if path.contains_key(&next_move) {
                continue;
            }
            to_visit.push_back(next_move.clone());
            path.insert(next_move.clone(), node.clone());
        }
    }
    None
}
