#![allow(clippy::suspicious_arithmetic_impl)]

use std::ops;

use primal::is_prime;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Gaussian(pub i64, pub i64);
impl Gaussian {
    pub const ZERO: Gaussian = Gaussian(0, 0);
    pub const ONE: Gaussian = Gaussian(1, 0);
    pub const I: Gaussian = Gaussian(0, 1);

    pub fn conj(&self) -> Gaussian {
        Gaussian(self.0, -self.1)
    }
    pub fn norm(&self) -> i64 {
        (self * self.conj()).0
    }
    pub fn re(&self) -> i64 {
        self.0
    }
    pub fn im(&self) -> i64 {
        self.1
    }
    pub fn is_zero(&self) -> bool {
        *self == Self::ZERO
    }
    pub fn is_unit(&self) -> bool {
        self.norm() == 1
    }
    /// Returns the factorization of a Gaussian integer
    /// into a unit and its associate in the first quadrant.
    pub fn factor_unit(&self) -> (Gaussian, Gaussian) {
        let unit = {
            if self.re() > 0 && self.im() >= 0 {
                Gaussian::ONE
            } else if self.im() > 0 && self.re() <= 0 {
                Gaussian::I
            } else if self.re() < 0 && self.im() <= 0 {
                -Gaussian::ONE
            } else if self.im() < 0 && self.re() >= 0 {
                -Gaussian::I
            } else {
                // Must be zero.
                Gaussian::ONE
            }
        };
        (unit, unit.conj() * self)
    }
    pub fn is_prime(&self) -> bool {
        let (_, a) = self.factor_unit();
        if a.im() == 0 {
            is_prime(a.re() as u64) && a.re() % 4 == 3
        } else {
            is_prime(a.norm() as u64)
        }
    }
}

impl From<i64> for Gaussian {
    fn from(value: i64) -> Gaussian {
        Gaussian(value, 0)
    }
}

impl ops::Neg for Gaussian {
    type Output = Gaussian;

    fn neg(self) -> Gaussian {
        Gaussian(-self.0, -self.1)
    }
}
impl ops::Neg for &Gaussian {
    type Output = Gaussian;

    fn neg(self) -> Gaussian {
        Gaussian(-self.0, -self.1)
    }
}

impl ops::Add<Gaussian> for Gaussian {
    type Output = Gaussian;

    fn add(self, rhs: Gaussian) -> Gaussian {
        Gaussian(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl ops::Add<Gaussian> for &Gaussian {
    type Output = Gaussian;

    fn add(self, rhs: Gaussian) -> Gaussian {
        Gaussian(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl ops::Add<&Gaussian> for Gaussian {
    type Output = Gaussian;

    fn add(self, rhs: &Gaussian) -> Gaussian {
        Gaussian(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl ops::Add<&Gaussian> for &Gaussian {
    type Output = Gaussian;

    fn add(self, rhs: &Gaussian) -> Gaussian {
        Gaussian(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl ops::Sub<Gaussian> for Gaussian {
    type Output = Gaussian;

    fn sub(self, rhs: Gaussian) -> Gaussian {
        Gaussian(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl ops::Sub<Gaussian> for &Gaussian {
    type Output = Gaussian;

    fn sub(self, rhs: Gaussian) -> Gaussian {
        Gaussian(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl ops::Sub<&Gaussian> for Gaussian {
    type Output = Gaussian;

    fn sub(self, rhs: &Gaussian) -> Gaussian {
        Gaussian(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl ops::Sub<&Gaussian> for &Gaussian {
    type Output = Gaussian;

    fn sub(self, rhs: &Gaussian) -> Gaussian {
        Gaussian(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl ops::Mul<Gaussian> for Gaussian {
    type Output = Gaussian;

    fn mul(self, rhs: Gaussian) -> Gaussian {
        Gaussian(
            self.0 * rhs.0 - self.1 * rhs.1,
            self.0 * rhs.1 + self.1 * rhs.0,
        )
    }
}
impl ops::Mul<Gaussian> for &Gaussian {
    type Output = Gaussian;

    fn mul(self, rhs: Gaussian) -> Gaussian {
        Gaussian(
            self.0 * rhs.0 - self.1 * rhs.1,
            self.0 * rhs.1 + self.1 * rhs.0,
        )
    }
}
impl ops::Mul<&Gaussian> for Gaussian {
    type Output = Gaussian;

    fn mul(self, rhs: &Gaussian) -> Gaussian {
        Gaussian(
            self.0 * rhs.0 - self.1 * rhs.1,
            self.0 * rhs.1 + self.1 * rhs.0,
        )
    }
}
impl ops::Mul<&Gaussian> for &Gaussian {
    type Output = Gaussian;

    fn mul(self, rhs: &Gaussian) -> Gaussian {
        Gaussian(
            self.0 * rhs.0 - self.1 * rhs.1,
            self.0 * rhs.1 + self.1 * rhs.0,
        )
    }
}

fn div_round(a: i64, b: i64) -> i64 {
    (a + b / 2).div_euclid(b)
}

impl ops::Div<Gaussian> for Gaussian {
    type Output = Gaussian;

    fn div(self, rhs: Gaussian) -> Gaussian {
        let a = self * rhs.conj();
        let n = rhs.norm();
        Gaussian(div_round(a.re(), n), div_round(a.im(), n))
    }
}
impl ops::Div<Gaussian> for &Gaussian {
    type Output = Gaussian;

    fn div(self, rhs: Gaussian) -> Gaussian {
        let a = self * rhs.conj();
        let n = rhs.norm();
        Gaussian(div_round(a.re(), n), div_round(a.im(), n))
    }
}
impl ops::Div<&Gaussian> for Gaussian {
    type Output = Gaussian;

    fn div(self, rhs: &Gaussian) -> Gaussian {
        let a = self * rhs.conj();
        let n = rhs.norm();
        Gaussian(div_round(a.re(), n), div_round(a.im(), n))
    }
}
impl ops::Div<&Gaussian> for &Gaussian {
    type Output = Gaussian;

    fn div(self, rhs: &Gaussian) -> Gaussian {
        let a = self * rhs.conj();
        let n = rhs.norm();
        Gaussian(div_round(a.re(), n), div_round(a.im(), n))
    }
}

impl ops::Rem<Gaussian> for Gaussian {
    type Output = Gaussian;

    fn rem(self, rhs: Gaussian) -> Gaussian {
        let q = self / rhs;
        self - q * rhs
    }
}
impl ops::Rem<Gaussian> for &Gaussian {
    type Output = Gaussian;

    fn rem(self, rhs: Gaussian) -> Gaussian {
        let q = self / rhs;
        self - q * rhs
    }
}
impl ops::Rem<&Gaussian> for Gaussian {
    type Output = Gaussian;

    fn rem(self, rhs: &Gaussian) -> Gaussian {
        let q = self / rhs;
        self - q * rhs
    }
}
impl ops::Rem<&Gaussian> for &Gaussian {
    type Output = Gaussian;

    fn rem(self, rhs: &Gaussian) -> Gaussian {
        let q = self / rhs;
        self - q * rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conj() {
        let a = Gaussian(1, 2);
        assert_eq!(Gaussian(1, -2), a.conj());
    }

    #[test]
    fn test_norm() {
        let a = Gaussian(1, 2);
        assert_eq!(5, a.norm());
    }

    #[test]
    fn test_re() {
        let a = Gaussian(1, 2);
        assert_eq!(1, a.re());
    }

    #[test]
    fn test_im() {
        let a = Gaussian(1, 2);
        assert_eq!(2, a.im());
    }

    #[test]
    fn test_is_zero() {
        assert!(Gaussian(0, 0).is_zero());
        assert!(!Gaussian(1, 0).is_zero());
        assert!(!Gaussian(0, 1).is_zero());
        assert!(!Gaussian(1, 1).is_zero());
        assert!(!Gaussian(1, -1).is_zero());
        assert!(!Gaussian(-1, 1).is_zero());
        assert!(!Gaussian(-1, -1).is_zero());
    }

    #[test]
    fn test_is_unit() {
        assert!(Gaussian(1, 0).is_unit());
        assert!(Gaussian(-1, 0).is_unit());
        assert!(Gaussian(0, 1).is_unit());
        assert!(Gaussian(0, -1).is_unit());
        assert!(!Gaussian(0, 0).is_unit());
        assert!(!Gaussian(1, 1).is_unit());
    }

    #[test]
    fn test_factor_unit() {
        assert_eq!(
            (Gaussian::ONE, Gaussian(3, 0)),
            Gaussian(3, 0).factor_unit()
        );
        assert_eq!((Gaussian::I, Gaussian(3, 0)), Gaussian(0, 3).factor_unit());
        assert_eq!(
            (-Gaussian::ONE, Gaussian(3, 0)),
            Gaussian(-3, 0).factor_unit()
        );
        assert_eq!(
            (-Gaussian::I, Gaussian(3, 0)),
            Gaussian(0, -3).factor_unit()
        );
    }

    #[test]
    fn test_is_prime() {
        assert!(!Gaussian(0, 0).is_prime());

        assert!(!Gaussian(1, 0).is_prime());
        assert!(!Gaussian(0, 1).is_prime());
        assert!(!Gaussian(-1, 0).is_prime());
        assert!(!Gaussian(0, -1).is_prime());

        assert!(Gaussian(1, 1).is_prime());
        assert!(Gaussian(1, -1).is_prime());
        assert!(Gaussian(-1, 1).is_prime());
        assert!(Gaussian(-1, -1).is_prime());

        assert!(!Gaussian(2, 0).is_prime());
        assert!(!Gaussian(0, 2).is_prime());
        assert!(!Gaussian(-2, 0).is_prime());
        assert!(!Gaussian(0, -2).is_prime());

        assert!(Gaussian(3, 0).is_prime());
        assert!(Gaussian(0, 3).is_prime());
        assert!(Gaussian(-3, 0).is_prime());
        assert!(Gaussian(0, -3).is_prime());

        assert!(!Gaussian(5, 0).is_prime());
        assert!(Gaussian(7, 0).is_prime());
        assert!(Gaussian(11, 0).is_prime());
        assert!(!Gaussian(13, 0).is_prime());
        assert!(!Gaussian(17, 0).is_prime());
        assert!(Gaussian(19, 0).is_prime());
        assert!(Gaussian(23, 0).is_prime());

        assert!(Gaussian(2, 1).is_prime());
        assert!(Gaussian(2, -1).is_prime());
        assert!(Gaussian(-2, 1).is_prime());
        assert!(Gaussian(-2, -1).is_prime());
        assert!(Gaussian(1, 2).is_prime());
        assert!(Gaussian(1, -2).is_prime());
        assert!(Gaussian(-1, 2).is_prime());
        assert!(Gaussian(-1, -2).is_prime());
    }

    #[test]
    fn test_add() {
        let a = Gaussian(1, 2);
        let b = Gaussian(3, 5);
        assert_eq!(Gaussian(4, 7), a + b);
        assert_eq!(Gaussian(4, 7), a + &b);
        assert_eq!(Gaussian(4, 7), &a + b);
        assert_eq!(Gaussian(4, 7), &a + &b);
    }

    #[test]
    fn test_sub() {
        let a = Gaussian(1, 2);
        let b = Gaussian(3, 5);
        assert_eq!(Gaussian(-2, -3), a - b);
        assert_eq!(Gaussian(-2, -3), a - &b);
        assert_eq!(Gaussian(-2, -3), &a - b);
        assert_eq!(Gaussian(-2, -3), &a - &b);
    }

    #[test]
    fn test_mul() {
        let a = Gaussian(1, 2);
        let b = Gaussian(3, 5);
        assert_eq!(Gaussian(-7, 11), a * b);
        assert_eq!(Gaussian(-7, 11), a * &b);
        assert_eq!(Gaussian(-7, 11), &a * b);
        assert_eq!(Gaussian(-7, 11), &a * &b);
    }

    #[test]
    fn test_div_rem() {
        for a_re in -20..=20 {
            for a_im in -20..=20 {
                let a = Gaussian(a_re, a_im);
                for b_re in -20..=20 {
                    for b_im in -20..=20 {
                        let b = Gaussian(b_re, b_im);
                        if !b.is_zero() {
                            let q = a / b;
                            let r = a % b;
                            assert_eq!(a, q * b + r);
                            assert!(r.norm() <= b.norm() / 2);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_div() {
        let a = Gaussian(1, 2);
        let b = Gaussian(3, 5);
        assert_eq!(Gaussian(0, 0), a / b);
        assert_eq!(Gaussian(0, 0), a / &b);
        assert_eq!(Gaussian(0, 0), &a / b);
        assert_eq!(Gaussian(0, 0), &a / &b);
    }

    #[test]
    fn test_rem() {
        let a = Gaussian(1, 2);
        let b = Gaussian(3, 5);
        assert_eq!(Gaussian(1, 2), a % b);
        assert_eq!(Gaussian(1, 2), a % &b);
        assert_eq!(Gaussian(1, 2), &a % b);
        assert_eq!(Gaussian(1, 2), &a % &b);
    }

    #[test]
    fn test_div_round() {
        assert_eq!(0, div_round(0, 3));
        assert_eq!(0, div_round(1, 3));
        assert_eq!(1, div_round(2, 3));
        assert_eq!(1, div_round(3, 3));
    }
}
