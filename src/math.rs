use std::{
    mem::swap,
    ops::{Div, Mul, Rem},
};

pub fn gcd<T>(mut a: T, mut b: T) -> T
where
    T: Default + Copy + Div<Output = T> + Rem<Output = T> + PartialOrd,
{
    if b > a {
        swap(&mut a, &mut b);
    }
    if b == T::default() {
        return a;
    }
    a = a % b;
    gcd(b, a % b)
}

pub fn lcm<T>(a: T, b: T) -> T
where
    T: Default + Copy + Mul<Output = T> + Div<Output = T> + Rem<Output = T> + PartialOrd,
{
    a * b / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        let expected = 6;
        let actual = gcd(18, 48);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_gcd_same() {
        let expected = 6;
        let actual = gcd(6, 6);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_gcd_zero() {
        let expected = 6;
        let actual = gcd(0, 6);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_lcm() {
        let expected = 144;
        let actual = lcm(18, 48);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_lcm_primes() {
        let expected = 15;
        let actual = lcm(3, 5);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_lcm_zero() {
        let expected = 0;
        let actual = lcm(0, 6);
        assert_eq!(expected, actual);
    }
}
