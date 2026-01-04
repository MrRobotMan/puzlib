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

#[cfg(test)]
mod tests {
    use crate::{Dir, Graph, Vec2D};

    use super::*;

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
