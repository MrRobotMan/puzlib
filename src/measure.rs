use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
};

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

/// 2D Vector
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Vec2D<T>(pub T, pub T);

impl<T> Vec2D<T>
where
    T: Copy + Clone + Mul<Output = T> + Sub<Output = T> + Add<Output = T> + PartialOrd + Default,
{
    /// Scale by a factor
    pub fn scale(&mut self, factor: T) -> Self {
        Self(self.0 * factor, self.1 * factor)
    }

    /// Get the Manhattan / taxi cab distance
    pub fn manhattan(&self, other: Self) -> T {
        let x = if self.0 > other.0 {
            self.0 - other.0
        } else {
            other.0 - self.0
        };
        let y = if self.1 > other.1 {
            self.1 - other.1
        } else {
            other.1 - self.1
        };
        x + y
    }

    /// Dot product (magnitude) of two vectors
    pub fn dot(&self, other: Self) -> T {
        self.0 * other.0 + self.1 * other.1
    }

    /// Convert to a 3D vector with normal axis set to 0.
    /// 2D points will retain relative position to each other.
    pub fn as_vec3d(&self, normal: Axis) -> Vec3D<T> {
        match normal {
            Axis::X => Vec3D(T::default(), self.0, self.1),
            Axis::Y => Vec3D(self.0, T::default(), self.1),
            Axis::Z => Vec3D(self.0, self.1, T::default()),
        }
    }
}

impl<T, U> From<(U, U)> for Vec2D<T>
where
    T: From<U>,
{
    /// Convert from a tuple to Vec2D
    /// ```
    /// let conv: puzlib::Vec2D<i64> = (10_u8, 2).into();
    /// assert_eq!(conv, puzlib::Vec2D(10_i64, 2));
    /// ```
    fn from(value: (U, U)) -> Self {
        Self(value.0.into(), value.1.into())
    }
}

impl<T> TryFrom<char> for Vec2D<T>
where
    T: From<i8>,
{
    type Error = String;

    /// Convert common character to Vec2D.
    fn try_from(value: char) -> Result<Self, Self::Error> {
        let one = 1_i8.into();
        let minus_one = (-1).into();
        let zero = 0.into();
        match value {
            'N' | 'U' | '^' => Ok(Self(minus_one, zero)),
            'S' | 'D' | 'v' => Ok(Self(one, zero)),
            'E' | 'R' | '<' => Ok(Self(zero, minus_one)),
            'W' | 'L' | '>' => Ok(Self(zero, one)),
            d => Err(format!("Unknown direction {d}")),
        }
    }
}

impl<T: Copy> FromIterator<T> for Vec2D<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        let s = Self(iter.next().unwrap(), iter.next().unwrap());
        if iter.next().is_some() {
            panic!("Can only collect length 2 iterators into points.");
        }
        s
    }
}

impl<T: Copy + Add<Output = T>> Add for Vec2D<T> {
    type Output = Vec2D<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T: Copy + Add<Output = T>> Add for &Vec2D<T> {
    type Output = Vec2D<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2D(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T: Copy + AddAssign> AddAssign for Vec2D<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl<T: Copy + Sub<Output = T>> Sub for Vec2D<T> {
    type Output = Vec2D<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T: Copy + Sub<Output = T>> Sub for &Vec2D<T> {
    type Output = Vec2D<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2D(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T: Copy + SubAssign> SubAssign for Vec2D<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl<T: Display> Display for Vec2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

/// 3D Vector
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Vec3D<T>(pub T, pub T, pub T);

impl<T> Vec3D<T>
where
    T: Copy + Clone + Mul<Output = T> + Sub<Output = T> + Add<Output = T> + PartialOrd,
{
    /// Return the vector projected into the plane normal to the provided axis.
    /// Normal X => (Y, Z),
    /// Normal Y => (X, Z),
    /// Normal Z => (X, Y)
    pub fn planer(&self, normal: Axis) -> Vec2D<T> {
        match normal {
            Axis::X => Vec2D::<T>(self.1, self.2),
            Axis::Y => Vec2D::<T>(self.0, self.2),
            Axis::Z => Vec2D::<T>(self.0, self.1),
        }
    }

    /// Scale a point by some value
    pub fn scale(&self, scale: T) -> Self {
        Self(self.0 * scale, self.1 * scale, self.2 * scale)
    }

    /// Get the Manhattan / taxi cab distance
    pub fn manhattan(&self, other: Self) -> T {
        let x = if self.0 > other.0 {
            self.0 - other.0
        } else {
            other.0 - self.0
        };
        let y = if self.1 > other.1 {
            self.1 - other.1
        } else {
            other.1 - self.1
        };
        let z = if self.2 > other.2 {
            self.2 - other.2
        } else {
            other.2 - self.2
        };
        x + y + z
    }

    /// Dot product (magnitude) of two vectors
    pub fn dot(&self, other: Self) -> T {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }
}

impl<T: Default> From<Vec2D<T>> for Vec3D<T> {
    fn from(value: Vec2D<T>) -> Self {
        Self(value.0, value.1, T::default())
    }
}

impl<T, U> From<(U, U, U)> for Vec3D<T>
where
    T: From<U>,
{
    /// Convert from a tuple to Vec2D
    /// ```
    /// let conv: puzlib::Vec3D<i64> = (10_u8, 2, 12).into();
    /// assert_eq!(conv, puzlib::Vec3D(10_i64, 2, 12));
    /// ```
    fn from(value: (U, U, U)) -> Self {
        Self(value.0.into(), value.1.into(), value.2.into())
    }
}

impl<T: Copy> FromIterator<T> for Vec3D<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        let s = Self(
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        );
        if iter.next().is_some() {
            panic!("Can only collect length 3 iterators into points.");
        }
        s
    }
}

/// Normal axes for Vec3D
#[derive(Debug, Default)]
pub enum Axis {
    X,
    Y,
    #[default]
    Z,
}

impl<T: Copy + Add<Output = T>> Add for Vec3D<T> {
    type Output = Vec3D<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl<T: Copy + Add<Output = T>> Add for &Vec3D<T> {
    type Output = Vec3D<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3D(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl<T: Copy + AddAssign> AddAssign for Vec3D<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl<T: Copy + Sub<Output = T>> Sub for Vec3D<T> {
    type Output = Vec3D<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl<T: Copy + Sub<Output = T>> Sub for &Vec3D<T> {
    type Output = Vec3D<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3D(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl<T: Copy + SubAssign> SubAssign for Vec3D<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

pub trait Cross {
    type Output;
    /// Cross product of two vectors
    fn cross(&self, other: Self) -> Self::Output;
}

macro_rules! cross_impl{
    ($($t:ty) *) => ($(
            impl Cross for Vec3D<$t> {
                type Output = Self;
                 fn cross(&self, other: Self) -> Self::Output {
                    Self(
                        self.1 * other.2 - self.2 * other.1,
                        self.2 * other.0 - self.0 * other.2,
                        self.0 * other.1 - self.1 * other.0,
                    )
                }
            }
            impl Cross for Vec2D<$t> {
                type Output = Vec3D<$t>;
                 fn cross(&self, other: Self) -> Self::Output {
                        let v1: Vec3D<$t> = (*self).into();
                        let v2 = other.into();
                        v1.cross(v2)
                }
            }
        )*)
}

cross_impl!(i8 i16 i32 i64 i128 isize f32 f64);

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
    fn test_manhattan_2d() {
        let expected = 10;
        let actual = Vec2D(-1, 6).manhattan(Vec2D(7, 8));
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_dot_2d() {
        let expected = 66;
        let actual = Vec2D(-6, 8).dot(Vec2D(5, 12));
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_cross_2d() {
        let expected = Vec3D(0, 0, 2);
        let actual = Vec2D(2, 2).cross(Vec2D(5, 6));
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_manhattan_3d() {
        let expected = 10;
        let actual = Vec3D(-1, 6, 5).manhattan(Vec3D(5, 8, 3));
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_dot_3d() {
        let expected = 602;
        let actual = Vec3D(-6, 8, 12).dot(Vec3D(5, 13, 44));
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_cross_3d() {
        let expected = Vec3D(-3.0, 6.0, -3.0);
        let actual = Vec3D(2.0, 3.0, 4.0).cross(Vec3D(5.0, 6.0, 7.0));
        assert_eq!(expected, actual);
    }

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
