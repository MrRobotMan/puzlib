use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
};

use super::*;

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

impl<T: Display> Display for Vec3D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

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
}
