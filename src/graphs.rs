#[derive(Debug)]
pub struct DisjointSet {
    nodes: Vec<Node>,
    version: DisjointType,
}

impl DisjointSet {
    /// Initialize the disjoint set based on checking tree size.
    pub fn init_size(node_count: usize) -> Self {
        Self {
            nodes: (0..node_count)
                .map(|idx| Node {
                    parent: idx,
                    size: 1,
                    rank: 0,
                })
                .collect(),
            version: DisjointType::Size,
        }
    }

    /// Initialize the disjoint set based on checking the rank of the tree root.
    pub fn init_rank(node_count: usize) -> Self {
        Self {
            nodes: (0..node_count)
                .map(|idx| Node {
                    parent: idx,
                    size: 1,
                    rank: 0,
                })
                .collect(),
            version: DisjointType::Rank,
        }
    }

    /// Get the root index
    pub fn find_root(&mut self, idx: usize) -> usize {
        if self.nodes[idx].parent != idx {
            self.nodes[idx].parent = self.find_root(self.nodes[idx].parent);
            self.nodes[idx].parent
        } else {
            idx
        }
    }

    /// Combine trees together. Returns true if the trees were previously disconnected.
    pub fn union(&mut self, left: usize, right: usize) -> bool {
        let mut left_root = self.find_root(left);
        let mut right_root = self.find_root(right);

        if left_root == right_root {
            return false;
        }

        (left_root, right_root) = self.order(left_root, right_root);

        self.nodes[right_root].parent = left_root;
        self.nodes[left_root].size += self.nodes[right_root].size;
        if self.nodes[left_root].rank == self.nodes[right_root].rank {
            self.nodes[left_root].size += 1;
        }
        true
    }

    fn order(&self, left: usize, right: usize) -> (usize, usize) {
        match self.version {
            DisjointType::Size if self.nodes[left].size < self.nodes[right].size => (right, left),
            DisjointType::Rank if self.nodes[left].rank < self.nodes[right].rank => (right, left),
            _ => (left, right),
        }
    }
}

#[derive(Debug)]
enum DisjointType {
    Size,
    Rank,
}

#[derive(Debug)]
pub struct Node {
    parent: usize,
    size: usize,
    rank: usize,
}
