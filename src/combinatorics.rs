use std::{collections::HashSet, hash::Hash};

pub trait Permutations<T> {
    fn permutations(&self) -> PermutationsIterator<T>;
}

impl<T: Clone + Hash + Eq> Permutations<T> for Vec<T> {
    fn permutations(&self) -> PermutationsIterator<T> {
        PermutationsIterator::new(self)
    }
}

impl<T: Clone + Hash + Eq> Permutations<T> for &[T] {
    fn permutations(&self) -> PermutationsIterator<T> {
        PermutationsIterator::new(self)
    }
}

pub struct PermutationsIterator<T> {
    items: Vec<T>,
    perms: HashSet<Vec<T>>,
    state: Vec<usize>,
    first: bool,
}

impl<T: Clone + Hash + Eq> PermutationsIterator<T> {
    fn new(items: &[T]) -> Self {
        let mut perms = HashSet::new();
        perms.insert(items.into());
        let state = vec![0; items.len()];
        Self {
            items: items.into(),
            first: true,
            perms,
            state,
        }
    }
}

impl<T: Clone + Hash + Eq> Iterator for PermutationsIterator<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            return Some(self.items.clone());
        }

        let mut idx = 1;

        while idx < self.items.len() {
            if self.state[idx] < idx {
                if idx % 2 == 0 && self.items[0] != self.items[idx] {
                    self.items.swap(0, idx);
                } else if idx % 2 == 1 && self.items[self.state[idx]] != self.items[idx] {
                    self.items.swap(self.state[idx], idx);
                }
                self.state[idx] += 1;
                if self.perms.insert(self.items.clone()) {
                    return Some(self.items.clone());
                };
                idx = 1;
            } else {
                self.state[idx] = 0;
                idx += 1;
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutations_trait() {
        assert_eq!(10, vec![1, 2, 1, 2, 1].permutations().count());
    }

    #[test]
    fn test_permutations_with_duplicates() {
        assert_eq!(120, [1, 2, 3, 4, 5].as_slice().permutations().count());
    }
}
