#![allow(unused)]

use crate::gaussian_integers::Gaussian;

pub fn mod_from(m: Gaussian, a: Gaussian) -> Gaussian {
    a % m
}

pub fn mod_add(m: Gaussian, a: Gaussian, b: Gaussian) -> Gaussian {
    mod_from(m, a + b)
}

pub fn mod_sub(m: Gaussian, a: Gaussian, b: Gaussian) -> Gaussian {
    mod_from(m, a - b)
}

pub fn mod_mul(m: Gaussian, a: Gaussian, b: Gaussian) -> Gaussian {
    mod_from(m, a * b)
}

pub fn mod_square(m: Gaussian, a: Gaussian) -> Gaussian {
    mod_from(m, a * a)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::gaussian_integers::Gaussian;

    use super::*;

    #[test]
    fn test_mod_from() {
        for m in [
            Gaussian(1, 1),
            Gaussian(3, 2),
            Gaussian(3, -2),
            Gaussian(-3, 2),
            Gaussian(-3, -2),
        ] {
            let mut representatives = HashSet::new();
            for re in -20..=20 {
                for im in -20..=20 {
                    representatives.insert(mod_from(m, Gaussian(re, im)));
                }
            }
            assert_eq!(m.norm() as usize, representatives.len());
        }
    }

    #[test]
    fn test_mod_add() {
        let m = Gaussian(3, 2);
        assert_eq!(
            mod_from(m, Gaussian(4, 7)),
            mod_add(m, Gaussian(1, 2), Gaussian(3, 5))
        )
    }

    #[test]
    fn test_mod_sub() {
        let m = Gaussian(3, 2);
        assert_eq!(
            mod_from(m, Gaussian(-2, -3)),
            mod_sub(m, Gaussian(1, 2), Gaussian(3, 5))
        )
    }

    #[test]
    fn test_mod_mul() {
        let m = Gaussian(3, 2);
        assert_eq!(
            mod_from(m, Gaussian(-7, 11)),
            mod_mul(m, Gaussian(1, 2), Gaussian(3, 5))
        )
    }

    #[test]
    fn test_mod_square() {
        let m = Gaussian(3, 2);
        assert_eq!(mod_from(m, Gaussian(-3, 4)), mod_square(m, Gaussian(1, 2)))
    }
}
