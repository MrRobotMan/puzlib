use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
};

#[derive(Debug)]
pub struct Dir<T> {
    _phantom: PhantomData<T>,
}

impl<T> Dir<T>
where
    T: Copy + From<i8> + Debug,
{
    /// Cardinal directions (N, E, S, W)
    /// ```
    /// use puzlib::{Vec2D, Dir};
    /// let cardinals = Dir::<i64>::cardinals();
    /// assert_eq!([Vec2D(-1, 0), Vec2D(0, 1), Vec2D(1, 0), Vec2D(0, -1)], cardinals);
    /// ```
    pub fn cardinals() -> [Vec2D<T>; 4] {
        let one = 1_i8.into();
        let zero = 0.into();
        let minus_one = (-1).into();
        [
            Vec2D(minus_one, zero),
            Vec2D(zero, one),
            Vec2D(one, zero),
            Vec2D(zero, minus_one),
        ]
    }

    /// Ordinal directions (NE, SE, SW, NW)
    /// ```
    /// use puzlib::{Vec2D, Dir};
    /// let ordinals = Dir::<i64>::ordinals();
    /// assert_eq!([Vec2D(-1, 1), Vec2D(1, 1), Vec2D(1, -1), Vec2D(-1, -1)], ordinals);
    /// ```
    pub fn ordinals() -> [Vec2D<T>; 4] {
        let one = 1_i8.into();
        let minus_one = (-1).into();
        [
            Vec2D(minus_one, one),
            Vec2D(one, one),
            Vec2D(one, minus_one),
            Vec2D(minus_one, minus_one),
        ]
    }

    /// Compass directions (N, NE, E, SE, S, SW, W, NW)
    /// ```
    /// use puzlib::{Vec2D, Dir};
    /// let compass = Dir::<i64>::compass();
    /// assert_eq!([Vec2D(-1, 0),Vec2D(-1, 1),Vec2D(0, 1),Vec2D(1, 1),Vec2D(1, 0),Vec2D(1, -1),Vec2D(0, -1),Vec2D(-1, -1)], compass);
    /// ```
    pub fn compass() -> [Vec2D<T>; 8] {
        let one = 1.into();
        let zero = 0.into();
        let minus_one = (-1).into();
        [
            Vec2D(minus_one, zero),
            Vec2D(minus_one, one),
            Vec2D(zero, one),
            Vec2D(one, one),
            Vec2D(one, zero),
            Vec2D(one, minus_one),
            Vec2D(zero, minus_one),
            Vec2D(minus_one, minus_one),
        ]
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Vec2D<T>(pub T, pub T);

impl<T> Vec2D<T>
where
    T: Copy + Clone + Mul<Output = T> + Sub<Output = T> + Add<Output = T> + PartialOrd,
{
    /// Scale by a factor
    pub fn scale(&mut self, factor: T) -> Self {
        Self(self.0 * factor, self.1 * factor)
    }

    /// Get the Manhatten / taxi cab distance
    pub fn manhatten(&self, other: &Self) -> T {
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
        let mut v = vec![];
        for i in iter {
            v.push(i)
        }
        Self(v[0], v[1])
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

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Vec3D<T>(pub T, pub T, pub T);

impl<T> Vec3D<T>
where
    T: Copy + Clone + Mul<Output = T> + Sub<Output = T> + Add<Output = T> + PartialOrd,
{
    /// Return the point in the plane normal to the provided axis.
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

    /// Scales a point by some value
    pub fn scale(&self, scale: T) -> Self {
        Self(self.0 * scale, self.1 * scale, self.2 * scale)
    }

    /// Get the Manhatten / taxi cab distance
    pub fn manhatten(&self, other: &Self) -> T {
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
}

impl<T: From<u8>> From<Vec2D<T>> for Vec3D<T> {
    fn from(value: Vec2D<T>) -> Self {
        Self(value.0, value.1, 0.into())
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
        let v = iter.into_iter().collect::<Vec<_>>();
        if v.len() != 3 {
            panic!("Can only collect length 3 iterators into points.");
        }
        Self(v[0], v[1], v[2])
    }
}

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
