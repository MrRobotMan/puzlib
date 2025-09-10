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
            return if self.items.is_empty() {
                None
            } else {
                Some(self.items.clone())
            };
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

pub trait Combinations<T> {
    fn combinations(&self) -> CombinationsIterator<T>;
    fn choose(&self, choose: usize) -> ChooseIterator<T>;
}

impl<T: Clone + Hash + Eq> Combinations<T> for Vec<T> {
    fn combinations(&self) -> CombinationsIterator<T> {
        CombinationsIterator::new(self)
    }
    fn choose(&self, choose: usize) -> ChooseIterator<T> {
        ChooseIterator::new(self, choose)
    }
}

impl<T: Clone + Hash + Eq> Combinations<T> for &[T] {
    fn combinations(&self) -> CombinationsIterator<T> {
        CombinationsIterator::new(self)
    }
    fn choose(&self, choose: usize) -> ChooseIterator<T> {
        ChooseIterator::new(self, choose)
    }
}

pub struct CombinationsIterator<T> {
    items: Vec<T>,
    count: usize,
}

impl<T: Clone> CombinationsIterator<T> {
    fn new(items: &[T]) -> Self {
        Self {
            items: items.into(),
            count: (1 << items.len()) - 1,
        }
    }
}

impl<T: Clone> Iterator for CombinationsIterator<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            return None;
        }
        let mut res = vec![];
        let mut idx = 0;
        let mut mask = self.count;
        self.count -= 1;
        while mask != 0 {
            if (mask & 1) == 1 {
                res.push(self.items[idx].clone());
            }
            idx += 1;
            mask >>= 1;
        }
        Some(res)
    }
}

pub struct ChooseIterator<T> {
    items: Vec<T>,
    idx: Vec<usize>,
    choose: usize,
    len: usize,
    first: bool,
}

impl<T: Clone> ChooseIterator<T> {
    fn new(items: &[T], choose: usize) -> Self {
        Self {
            items: items.into(),
            idx: (0..choose).collect(),
            choose,
            len: items.len(),
            first: true,
        }
    }

    fn combo(&self) -> Vec<T> {
        self.idx
            .iter()
            .map(|idx| self.items[*idx].clone())
            .collect()
    }
}

impl<T: Clone> Iterator for ChooseIterator<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.items.is_empty() || self.items.len() < self.choose {
            return None;
        }
        if self.first {
            self.first = false;
            return Some(self.combo());
        }
        let mut complete = false;
        while !complete {
            complete = true;
            'outer: for idx in (0..self.choose).rev() {
                self.idx[idx] += 1;
                if self.idx[idx] == self.len {
                    continue;
                }

                for next in idx + 1..self.choose {
                    self.idx[next] = self.idx[next - 1] + 1;
                    if self.idx[next] == self.len {
                        continue 'outer;
                    }
                }

                complete = false;
                break;
            }
            if !complete {
                return Some(self.combo());
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutations() {
        let mut expected = HashSet::from([
            vec![1, 2, 1, 2, 1],
            vec![2, 1, 1, 2, 1],
            vec![1, 1, 2, 2, 1],
            vec![2, 2, 1, 1, 1],
            vec![1, 2, 2, 1, 1],
            vec![2, 1, 2, 1, 1],
            vec![1, 1, 2, 1, 2],
            vec![2, 1, 1, 1, 2],
            vec![1, 2, 1, 1, 2],
            vec![1, 1, 1, 2, 2],
        ]);
        let actual = vec![1, 2, 1, 2, 1].permutations().collect::<Vec<_>>();
        for item in actual {
            assert!(expected.contains(&item));
            expected.remove(&item);
        }
        assert!(expected.is_empty());
    }

    #[test]
    fn test_permutations_without_duplicates() {
        assert_eq!(120, [1, 2, 3, 4, 5].as_slice().permutations().count());
    }

    #[test]
    fn test_permutations_repeating() {
        assert_eq!(1, vec![1, 1, 1, 1].permutations().count());
    }

    #[test]
    fn test_permutations_one_item() {
        assert_eq!(1, vec![1].permutations().count());
    }
    #[test]
    fn test_permutations_empty() {
        let emp = Vec::<i32>::new();
        assert_eq!(0, emp.permutations().count());
    }

    #[test]
    fn test_combinations() {
        let mut expected = HashSet::from([
            vec![1],
            vec![2],
            vec![3],
            vec![1, 2],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ]);
        let actual = vec![1, 2, 3].combinations();
        for item in actual {
            assert!(expected.contains(&item));
            expected.remove(&item);
        }
        assert!(expected.is_empty());
    }

    #[test]
    fn test_combinations_one_item() {
        assert_eq!(1, vec![1].combinations().count());
    }

    #[test]
    fn test_combinations_empty() {
        let emp = Vec::<i32>::new();
        assert_eq!(0, emp.combinations().count());
    }

    #[test]
    fn test_choose() {
        let mut expected = HashSet::from([
            vec![1, 2, 3],
            vec![1, 2, 4],
            vec![1, 2, 5],
            vec![1, 3, 4],
            vec![1, 3, 5],
            vec![1, 4, 5],
            vec![2, 3, 4],
            vec![2, 3, 5],
            vec![2, 4, 5],
            vec![3, 4, 5],
        ]);
        let actual = vec![1, 2, 3, 4, 5].choose(3);
        for item in actual {
            assert!(expected.contains(&item));
            expected.remove(&item);
        }
        assert!(expected.is_empty());
    }

    #[test]
    fn test_choose_too_many() {
        assert_eq!(0, [1, 2, 3, 4].as_slice().choose(6).count())
    }

    #[test]
    fn test_choose_all() {
        assert_eq!(1, [1, 2, 3, 4].as_slice().choose(4).count())
    }

    #[test]
    fn test_choose_one_item() {
        assert_eq!(1, vec![1].choose(1).count());
    }

    #[test]
    fn test_choose_empty() {
        let emp = Vec::<i32>::new();
        assert_eq!(0, emp.choose(1).count());
    }
}
