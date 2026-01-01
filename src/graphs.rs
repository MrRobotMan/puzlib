#[derive(Debug)]
pub struct DisjointSet<'a, T> {
    nodes: Vec<Node<'a, T>>,
}

impl<'a, T> DisjointSet<'a, T> {
    pub fn init(data: &'a [T]) -> Self {
        Self {
            nodes: data
                .iter()
                .enumerate()
                .map(|(idx, data)| Node {
                    parent: idx,
                    size: 1,
                    rank: 0,
                    data,
                })
                .collect(),
        }
    }

    pub fn find_root(&mut self, idx: usize) -> usize {
        if self.nodes[idx].parent != idx {
            self.nodes[idx].parent = self.find_root(self.nodes[idx].parent);
            self.nodes[idx].parent
        } else {
            idx
        }
    }

    pub fn union(&mut self, left: usize, right: usize, union_type: UnionType) {
        let mut left_root = self.find_root(left);
        let mut right_root = self.find_root(right);

        if left_root == right_root {
            return;
        }
        match union_type {
            UnionType::Size => {
                if self.nodes[left_root].size < self.nodes[right_root].size {
                    (left_root, right_root) = (right_root, left_root)
                }
            }
            UnionType::Rank => {
                if self.nodes[left_root].rank < self.nodes[right_root].rank {
                    (left_root, right_root) = (right_root, left_root)
                }
            }
        }
        self.nodes[right_root].parent = left_root;
        match union_type {
            UnionType::Size => self.nodes[left_root].size += self.nodes[right_root].size,
            UnionType::Rank => self.nodes[left_root].size += 1,
        }
    }
}

#[derive(Debug)]
pub enum UnionType {
    Size,
    Rank,
}

#[derive(Debug)]
pub struct Node<'a, T> {
    parent: usize,
    size: usize,
    rank: usize,
    data: &'a T,
}

// ToDo Minimum spanning tree
