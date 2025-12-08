use std::{fmt::Debug, marker::PhantomData};

use crate::{CheckedAdd, CheckedSub, Vec2D};

/// Directions for stepping
#[derive(Debug)]
pub struct Dir<T> {
    _phantom: PhantomData<T>,
}

impl<T> Dir<T>
where
    T: Debug + Copy + From<u8> + Sized + CheckedAdd + CheckedSub,
{
    /// Cardinal directions (N, E, S, W)
    /// ```
    /// use puzlib::{Vec2D, Dir};
    /// let cardinals = Dir::<i64>::cardinals(&Vec2D(0,0));
    /// assert_eq!([Some(Vec2D(-1, 0)), Some(Vec2D(0, 1)), Some(Vec2D(1, 0)), Some(Vec2D(0, -1))], cardinals);
    /// ```
    pub fn cardinals(from: &Vec2D<T>) -> [Option<Vec2D<T>>; 4] {
        let one = 1_u8.into();
        let up = from.0.checked_sub(&one).map(|n| Vec2D(n, from.1));
        let right = from.1.checked_add(&one).map(|n| Vec2D(from.0, n));
        let down = from.0.checked_add(&one).map(|n| Vec2D(n, from.1));
        let left = from.1.checked_sub(&one).map(|n| Vec2D(from.0, n));
        [up, right, down, left]
    }

    /// Ordinal directions (NE, SE, SW, NW)
    /// ```
    /// use puzlib::{Vec2D, Dir};
    /// let ordinals = Dir::<i64>::ordinals(&Vec2D(0,0));
    /// assert_eq!([Some(Vec2D(-1, 1)), Some(Vec2D(1, 1)), Some(Vec2D(1, -1)), Some(Vec2D(-1, -1))], ordinals);
    /// ```
    pub fn ordinals(from: &Vec2D<T>) -> [Option<Vec2D<T>>; 4] {
        let one = 1_u8.into();
        let up = from.0.checked_sub(&one);
        let down = from.0.checked_add(&one);
        let right = from.1.checked_add(&one);
        let left = from.1.checked_sub(&one);
        let up_right = if let Some(n1) = up
            && let Some(n2) = right
        {
            Some(Vec2D(n1, n2))
        } else {
            None
        };
        let down_right = if let Some(n1) = down
            && let Some(n2) = right
        {
            Some(Vec2D(n1, n2))
        } else {
            None
        };
        let down_left = if let Some(n1) = down
            && let Some(n2) = left
        {
            Some(Vec2D(n1, n2))
        } else {
            None
        };
        let up_left = if let Some(n1) = up
            && let Some(n2) = left
        {
            Some(Vec2D(n1, n2))
        } else {
            None
        };
        [up_right, down_right, down_left, up_left]
    }

    /// Compass directions (N, NE, E, SE, S, SW, W, NW)
    /// ```
    /// use puzlib::{Vec2D, Dir};
    /// let compass = Dir::<i64>::compass(&Vec2D(0,0));
    /// assert_eq!([Some(Vec2D(-1, 0)),Some(Vec2D(-1, 1)),Some(Vec2D(0, 1)),Some(Vec2D(1, 1)),Some(Vec2D(1, 0)),Some(Vec2D(1, -1)),Some(Vec2D(0, -1)),Some(Vec2D(-1, -1))], compass);
    /// ```
    pub fn compass(from: &Vec2D<T>) -> [Option<Vec2D<T>>; 8] {
        let c = Self::cardinals(from);
        let o = Self::ordinals(from);
        [c[0], o[0], c[1], o[1], c[2], o[2], c[3], o[3]]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cardinals() {
        let expected = [
            Some(Vec2D(1, 0)),
            Some(Vec2D(2, 1)),
            Some(Vec2D(3, 0)),
            None,
        ];
        let actual = Dir::<usize>::cardinals(&Vec2D(2, 0));
        assert_eq!(expected, actual);
    }
}
