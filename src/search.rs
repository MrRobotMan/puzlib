use std::{
    cmp::Eq,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    hash::Hash,
};

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

pub fn a_star<
    N: Hash + Ord + PartialOrd + Clone + std::fmt::Debug,
    G: Weighted<Node = N>,
    F: Fn(&N) -> usize,
>(
    start: &N,
    graph: &G,
    heuristic: F,
) -> Option<(HashMap<N, usize>, Vec<N>)> {
    let mut heap: BinaryHeap<MinHeapState<N>> = BinaryHeap::new();
    let mut path: HashMap<N, N> = HashMap::new();
    let mut dist: HashMap<N, usize> = HashMap::new();
    let mut index: HashSet<N> = HashSet::new();

    heap.push(MinHeapState {
        node: start.clone(),
        cost: heuristic(start),
    });
    dist.insert(start.clone(), 0);
    index.insert(start.clone());

    while let Some(MinHeapState { node, .. }) = heap.pop() {
        // Reached our goal.
        if graph.is_done(&node) {
            return Some((dist, get_path(path, node, start)));
        }

        for next_move in graph.moves(&node) {
            // Build the queue as we go instead of putting all nodes in at the start.
            if index.insert(next_move.clone()) {
                dist.insert(next_move.clone(), usize::MAX);
            }
            let tentative_cost = dist[&node] + graph.weight(&node, &next_move);
            if tentative_cost < dist[&next_move] {
                let next_cost = tentative_cost + heuristic(&next_move);
                heap.push(MinHeapState {
                    node: next_move.clone(),
                    cost: next_cost,
                });
                dist.entry(next_move.clone())
                    .and_modify(|v| *v = tentative_cost);
                let cur = path.entry(next_move.clone()).or_insert(node.clone());
                *cur = node.clone();
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
    use crate::{Dir, Vec2D};

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

    #[test]
    fn test_a_star() {
        let expected = 28;
        let chamber: Chamber = r#"#######
#6769##
S50505E
#97434#
#######"#
            .lines()
            .collect::<Vec<_>>()
            .into();
        let res = a_star(&chamber.start[0], &chamber, |n| chamber.heuristic(n)).unwrap();
        println!("{:?}", res.1);
        let actual = res.0[&chamber.end];
        assert_eq!(expected, actual);
    }

    #[derive(Debug, Default)]
    struct Chamber {
        chamber: HashMap<Vec2D<i64>, i64>,
        start: Vec<Vec2D<i64>>,
        end: Vec2D<i64>,
        size: (usize, usize),
    }
    impl Chamber {
        fn heuristic(&self, n: &Vec2D<i64>) -> usize {
            self.end.manhattan(*n) as usize
        }
    }

    impl Graph for Chamber {
        type Node = Vec2D<i64>;

        fn height(&self) -> usize {
            self.size.0
        }

        fn width(&self) -> usize {
            self.size.1
        }

        fn moves(&self, node: &Self::Node) -> Vec<Self::Node> {
            Dir::<i64>::cardinals(node)
                .iter()
                .filter_map(|n| {
                    if let Some(a) = n
                        && self.chamber.contains_key(a)
                    {
                        Some(*a)
                    } else {
                        None
                    }
                })
                .collect()
        }

        fn is_done(&self, node: &Self::Node) -> bool {
            node == &self.end
        }
    }
    impl Weighted for Chamber {
        fn weight(&self, cur: &Self::Node, next: &Self::Node) -> usize {
            let a = self.chamber[cur];
            let b = self.chamber[next];
            let cost = (a - b).abs().min(10 - (b - a).abs()) + 1;
            cost as usize
        }
    }

    impl<S: AsRef<str>> From<Vec<S>> for Chamber {
        fn from(value: Vec<S>) -> Self {
            let mut chamber = Chamber {
                size: (value.len(), value[0].as_ref().len()),
                ..Default::default()
            };
            for (row, line) in value.into_iter().enumerate() {
                for (col, ch) in line.as_ref().chars().enumerate() {
                    let row = row as i64;
                    let col = col as i64;
                    match ch {
                        'E' => {
                            chamber.end = Vec2D(row, col);
                            chamber.chamber.insert(Vec2D(row, col), 0);
                        }
                        'S' => {
                            chamber.start.push(Vec2D(row, col));
                            chamber.chamber.insert(Vec2D(row, col), 0);
                        }
                        x if x.is_ascii_digit() => {
                            chamber
                                .chamber
                                .insert(Vec2D(row, col), (x as u8 - b'0') as i64);
                        }
                        _ => (),
                    }
                }
            }
            chamber
        }
    }
}
