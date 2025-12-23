use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
};

use super::*;

/// 2D Vector
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Vec2D<T>(pub T, pub T);

impl<T> Vec2D<T>
where
    T: Copy + Clone + Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Ord + Default,
{
    /// Scale by a factor
    pub fn scale(&mut self, factor: T) -> Self {
        Self(self.0 * factor, self.1 * factor)
    }

    /// Get the Manhattan / taxi cab distance
    pub fn manhattan(&self, other: Self) -> T {
        let x = self.0.max(other.0) - self.0.min(other.0);
        let y = self.1.max(other.1) - self.1.min(other.1);
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

    /// Map the vector to another vector with a different interior type based on a closure
    pub fn map<U>(&self, mut f: impl FnMut(T) -> U) -> Vec2D<U> {
        let Self(x, y) = *self;
        Vec2D(f(x), f(y))
    }

    /// Map the vector to another vector with a different interior type based on a closure
    pub fn map_into<U: From<T>>(&self) -> Vec2D<U> {
        let Self(x, y) = *self;
        Vec2D(x.into(), y.into())
    }
}

impl<T> Vec2D<T>
where
    T: Ord + Into<f64> + Sub<Output = T> + Copy,
{
    /// Get the distance as the arrow flies to another Vec2D
    pub fn distance_to(&self, other: Self) -> f64 {
        let a: f64 = (self.0.max(other.0) - self.0.min(other.0)).into();
        let b: f64 = (self.1.max(other.1) - self.1.min(other.1)).into();
        (a.powi(2) + b.powi(2)).sqrt()
    }
}

impl<T: From<U>, U> From<(U, U)> for Vec2D<T> {
    /// Convert from a tuple to Vec2D while changing numeric types
    /// ```
    /// let conv: puzlib::Vec2D<i64> = (10_u8, 2).into();
    /// assert_eq!(conv, puzlib::Vec2D(10_i64, 2));
    /// ```
    fn from(value: (U, U)) -> Self {
        Self(value.0.into(), value.1.into())
    }
}

impl<T: Copy + Clone> From<&(T, T)> for Vec2D<T> {
    /// Convert from reference tuple to Vec2D.
    fn from(value: &(T, T)) -> Self {
        Self(value.0, value.1)
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
    fn test_distance_2d() {
        let expected = 5.0;
        let actual = Vec2D(2, 9).distance_to(Vec2D(3, 5));
        assert!(actual - expected < 1e-6)
    }
}
