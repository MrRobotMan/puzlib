use std::ops::Deref;

/// Get the midpoint(s) of an array. If the array is sorted this will get the median value(s);
/// For odd length arrays the slice will contain 1 element, for even length 2.
pub trait Median<T: Ord> {
    fn mid(&self) -> &[T]
    where
        Self: Deref<Target = [T]>,
    {
        let midpoint = self.len() / 2;
        let len = self.len();
        if len <= 2 {
            return self.deref();
        }
        if len % 2 == 1 {
            &self[midpoint..midpoint + 1]
        } else {
            &self[midpoint - 1..midpoint + 1]
        }
    }
}

impl<T: Ord> Median<T> for Vec<T> {}
impl<T: Ord> Median<T> for &[T] {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_odd_array_mid() {
        let mut odd = vec![1, 4, 3, 5, 6];
        assert_eq!(odd.mid(), [3]);
        odd.sort();
        assert_eq!(odd.mid(), [4]);
    }

    #[test]
    fn test_even_array_mid() {
        let even = [1, 2, 3, 4];
        assert_eq!(even.as_slice().mid(), [2, 3]);
    }
}
