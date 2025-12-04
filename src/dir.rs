use std::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Add, Sub},
};

mod vec2d;
use vec2d::Vec2D;

#[derive(Debug)]
pub struct Dir<T> {
    _phantom: PhantomData<T>,
}

impl<T> Dir<T>
where
    T: Copy + Debug + From<u8> + CheckedAdd + CheckedSub + Sized,
{
    pub fn cardinals(from: Vec2D<T>) -> [Option<Vec2D<T>>; 4] {
        let one: T = 1_u8.into();
        let up = from.0.checked_sub(&one).map(|n| Vec2D(n, from.1));
        let right = from.0.checked_add(&one).map(|n| Vec2D(from.0, n));
        let down = from.1.checked_add(&one).map(|n| Vec2D(n, from.1));
        let left = from.1.checked_sub(&one).map(|n| Vec2D(from.0, n));
        [up, right, down, left]
    }
}

macro_rules! checked_impl {
    ($trait_name:ident, $method:ident, $t:ty) => {
        impl $trait_name for $t {
            #[inline]
            fn $method(&self, v: &$t) -> Option<$t> {
                <$t>::$method(*self, *v)
            }
        }
    };
}

pub trait CheckedAdd: Sized + Add<Self, Output = Self> {
    /// Adds two numbers, checking for overflow. If overflow happens, `None` is
    /// returned.
    fn checked_add(&self, v: &Self) -> Option<Self>;
}

checked_impl!(CheckedAdd, checked_add, u8);
checked_impl!(CheckedAdd, checked_add, u16);
checked_impl!(CheckedAdd, checked_add, u32);
checked_impl!(CheckedAdd, checked_add, u64);
checked_impl!(CheckedAdd, checked_add, usize);
checked_impl!(CheckedAdd, checked_add, u128);

checked_impl!(CheckedAdd, checked_add, i8);
checked_impl!(CheckedAdd, checked_add, i16);
checked_impl!(CheckedAdd, checked_add, i32);
checked_impl!(CheckedAdd, checked_add, i64);
checked_impl!(CheckedAdd, checked_add, isize);
checked_impl!(CheckedAdd, checked_add, i128);

/// Performs subtraction that returns `None` instead of wrapping around on underflow.
pub trait CheckedSub: Sized + Sub<Self, Output = Self> {
    /// Subtracts two numbers, checking for underflow. If underflow happens,
    /// `None` is returned.
    fn checked_sub(&self, v: &Self) -> Option<Self>;
}

checked_impl!(CheckedSub, checked_sub, u8);
checked_impl!(CheckedSub, checked_sub, u16);
checked_impl!(CheckedSub, checked_sub, u32);
checked_impl!(CheckedSub, checked_sub, u64);
checked_impl!(CheckedSub, checked_sub, usize);
checked_impl!(CheckedSub, checked_sub, u128);

checked_impl!(CheckedSub, checked_sub, i8);
checked_impl!(CheckedSub, checked_sub, i16);
checked_impl!(CheckedSub, checked_sub, i32);
checked_impl!(CheckedSub, checked_sub, i64);
checked_impl!(CheckedSub, checked_sub, isize);
checked_impl!(CheckedSub, checked_sub, i128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directions_signed() {
        let expected = [
            Some(Vec2D(-1, 0)),
            Some(Vec2D(0, 1)),
            Some(Vec2D(1, 0)),
            Some(Vec2D(0, -1)),
        ];
        let actual = Dir::<i64>::cardinals(Vec2D(0, 0));
        assert_eq!(expected, actual);
    }

    #[test]
    fn directions_unsigned_in_bounds() {
        let expected = [
            Some(Vec2D(9, 10)),
            Some(Vec2D(10, 11)),
            Some(Vec2D(11, 10)),
            Some(Vec2D(10, 9)),
        ];
        let actual = Dir::<i64>::cardinals(Vec2D(10, 10));
        assert_eq!(expected, actual);
    }

    #[test]
    fn directions_unsigned_out_of_bounds() {
        let expected = [None, Some(Vec2D(0, 1)), Some(Vec2D(1, 0)), None];
        let actual = Dir::<u8>::cardinals(Vec2D(0, 0));
        assert_eq!(expected, actual);
    }
}
