use std::{
    cmp::Eq,
    collections::{BinaryHeap, HashMap, HashSet},
    hash::Hash,
};

use super::{Weighted, get_path};

#[derive(Debug, Clone, PartialEq, Eq)]
struct MinHeapState<S: Hash + Ord + PartialOrd + Eq + PartialEq> {
    node: S,
    cost: usize,
}

impl<S: Hash + Ord> Ord for MinHeapState<S> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl<S: Hash + Ord> PartialOrd for MinHeapState<S> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn dijkstra<N: Hash + Ord + PartialOrd + Clone, G: Weighted<Node = N>>(
    start: &N,
    graph: &G,
) -> Option<(HashMap<N, usize>, Vec<N>)> {
    let mut heap: BinaryHeap<MinHeapState<N>> = BinaryHeap::new();
    let mut dist: HashMap<N, usize> = HashMap::new();
    let mut index: HashSet<N> = HashSet::new();
    let mut path: HashMap<N, N> = HashMap::new();

    heap.push(MinHeapState {
        node: start.clone(),
        cost: 0,
    });
    dist.insert(start.clone(), 0);
    index.insert(start.clone());

    while let Some(MinHeapState { node, cost }) = heap.pop() {
        // Reached our goal.
        if graph.is_done(&node) {
            return Some((dist, get_path(path, node, start)));
        }

        // Already have a better path to node.
        if cost > dist[&node] {
            continue;
        }
        for next_move in graph.moves(&node) {
            let next_cost = cost + graph.weight(&node, &next_move);
            // Build the queue as we go instead of putting all nodes in at the start.
            if index.insert(next_move.clone()) {
                dist.insert(next_move.clone(), usize::MAX);
            }
            if next_cost < dist[&next_move] {
                heap.push(MinHeapState {
                    node: next_move.clone(),
                    cost: next_cost,
                });
                dist.entry(next_move.clone()).and_modify(|v| *v = next_cost);
                let cur = path.entry(next_move.clone()).or_insert(node.clone());
                *cur = node.clone();
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::Graph;

    use super::*;

    #[test]
    fn test_dijkstra() {
        // Testing the example from https://doc.rust-lang.org/stable/std/collections/binary_heap/index.html
        let start = 3;
        let mut graph = Layout {
            nodes: vec![
                // Node 0
                vec![(2, 10), (1, 1)],
                // Node 1
                vec![(3, 2)],
                // Node 2
                vec![(1, 1), (3, 3), (4, 1)],
                // Node 3
                vec![(0, 7), (4, 2)],
                // Node 4
                vec![],
            ],
            target: 0,
        };
        assert_eq!(
            Some((7_usize, vec![3, 0])),
            dijkstra(&start, &graph).map(|g| (g.0[&graph.target], g.1))
        );
        graph.target = 4;
        let start = 0;
        assert_eq!(
            Some((5_usize, vec![0, 1, 3, 4])),
            dijkstra(&start, &graph).map(|g| (g.0[&graph.target], g.1))
        );
    }
    struct Layout {
        nodes: Vec<Vec<(usize, usize)>>,
        target: usize,
    }

    impl Graph for Layout {
        type Node = usize;

        fn moves(&self, cur: &usize) -> Vec<usize> {
            self.nodes[*cur].iter().map(|v| v.0).collect()
        }

        fn is_done(&self, node: &usize) -> bool {
            *node == self.target
        }

        fn height(&self) -> usize {
            3
        }

        fn width(&self) -> usize {
            4
        }
    }

    impl Weighted for Layout {
        fn weight(&self, cur: &usize, next: &usize) -> usize {
            self.nodes[*cur]
                .iter()
                .filter_map(|v| if v.0 == *next { Some(v.1) } else { None })
                .next()
                .unwrap()
        }
    }
}
