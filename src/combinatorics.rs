use std::{collections::HashSet, hash::Hash};

/// Generate all unique permutations of array of items where items can repeat in the array.
pub fn permutations_with_duplicates<N: Eq + Hash + Clone>(items: &[N]) -> HashSet<Vec<N>> {
    let mut items = items.to_vec();
    let mut perms = HashSet::new();
    perms.insert(items.clone());
    let mut state = vec![0; items.len()];
    let mut idx = 1;
    while idx < items.len() {
        if state[idx] < idx {
            if idx % 2 == 0 && items[0] != items[idx] {
                items.swap(0, idx);
            } else if idx % 2 == 1 && items[state[idx]] != items[idx] {
                items.swap(state[idx], idx);
            }
            perms.insert(items.clone());
            state[idx] += 1;
            idx = 1;
        } else {
            state[idx] = 0;
            idx += 1;
        }
    }
    perms
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutations_with_duplicates() {
        assert_eq!(10, permutations_with_duplicates(&[1, 1, 2, 2, 2]).len());
    }

    #[test]
    fn test_permutations_with_duplicates_long() {
        assert_eq!(
            9240,
            permutations_with_duplicates(&[1, 1, 1, 1, 1, 2, 2, 2, 3, 3, 3]).len()
        );
    }
}
