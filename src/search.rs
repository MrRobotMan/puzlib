use std::{
    cmp::Eq,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    hash::Hash,
};

pub trait Graph {
    fn height(&self) -> usize;
    fn width(&self) -> usize;
}

pub trait Searcher<G: Graph>: Eq + Hash + Clone {
    fn moves(&self, graph: &G) -> Vec<Self>
    where
        Self: Sized;
    fn is_done(&self, graph: &G) -> bool;
}

pub fn dfs<S: Searcher<G>, G: Graph>(start: &S, graph: &G) -> Option<Vec<S>> {
    let mut path = HashMap::new();
    let mut to_visit = vec![start.clone()];
    while let Some(node) = to_visit.pop() {
        if path.contains_key(&node) {
            continue;
        }
        if node.is_done(graph) {
            return Some(get_path(path, node, start));
        }
        for next_move in node.moves(graph) {
            to_visit.push(next_move.clone());
            path.insert(next_move.clone(), node.clone());
        }
    }
    None
}

pub fn bfs<S: Searcher<G>, G: Graph>(start: &S, graph: &G) -> Option<Vec<S>> {
    let mut path = HashMap::new();
    let mut to_visit = VecDeque::new();
    to_visit.push_front(start.clone());
    while let Some(node) = to_visit.pop_front() {
        if node.is_done(graph) {
            return Some(get_path(path, node, start));
        }
        for next_move in node.moves(graph) {
            if path.contains_key(&next_move) {
                continue;
            }
            to_visit.push_back(next_move.clone());
            path.insert(next_move.clone(), node.clone());
        }
    }
    None
}

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

pub trait Weighted {
    type Node: Eq + Hash + Clone;
    fn weight(&self, cur: &Self::Node, next: &Self::Node) -> usize;
    fn moves(&self, cur: &Self::Node) -> Vec<Self::Node>;
    fn is_done(&self, node: &Self::Node) -> bool;
}

pub fn dijkstra<N: Hash + Ord + PartialOrd + Clone, G: Weighted<Node = N>>(
    start: &N,
    graph: &G,
) -> Option<HashMap<N, usize>> {
    let mut heap: BinaryHeap<MinHeapState<N>> = BinaryHeap::new();
    let mut dist: HashMap<N, usize> = HashMap::new();
    let mut index: HashSet<N> = HashSet::new();

    heap.push(MinHeapState {
        node: start.clone(),
        cost: 0,
    });
    dist.insert(start.clone(), 0);
    index.insert(start.clone());

    while let Some(MinHeapState { node, cost }) = heap.pop() {
        // Reached our goal.
        if graph.is_done(&node) {
            return Some(dist);
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
                dist.entry(next_move).and_modify(|v| *v = next_cost);
            }
        }
    }

    None
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

#[cfg(test)]
mod tests {
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
            Some(7_usize),
            dijkstra(&start, &graph).map(|g| g[&graph.target])
        );
        graph.target = 4;
        let start = 0;
        assert_eq!(
            Some(5_usize),
            dijkstra(&start, &graph).map(|g| g[&graph.target])
        );
    }

    struct Layout {
        nodes: Vec<Vec<(usize, usize)>>,
        target: usize,
    }

    impl Weighted for Layout {
        type Node = usize;
        fn weight(&self, cur: &usize, next: &usize) -> usize {
            self.nodes[*cur]
                .iter()
                .filter_map(|v| if v.0 == *next { Some(v.1) } else { None })
                .next()
                .unwrap()
        }

        fn moves(&self, cur: &usize) -> Vec<usize> {
            self.nodes[*cur].iter().map(|v| v.0).collect()
        }

        fn is_done(&self, node: &usize) -> bool {
            *node == self.target
        }
    }
}
