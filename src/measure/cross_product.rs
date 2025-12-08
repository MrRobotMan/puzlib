use std::fmt::Debug;

use crate::{Vec2D, Vec3D};

/// Normal axes for Vec3D
#[derive(Debug, Default)]
pub enum Axis {
    X,
    Y,
    #[default]
    Z,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross_2d() {
        let expected = Vec3D(0, 0, 2);
        let actual = Vec2D(2, 2).cross(Vec2D(5, 6));
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_cross_3d() {
        let expected = Vec3D(-3.0, 6.0, -3.0);
        let actual = Vec3D(2.0, 3.0, 4.0).cross(Vec3D(5.0, 6.0, 7.0));
        assert_eq!(expected, actual);
    }
}
