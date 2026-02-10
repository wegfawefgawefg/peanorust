use std::cmp::Ordering;
use std::fmt;

use crate::n::Nat;
use crate::z::Int;

// Rationals (Q) built on top of Z and N (positive denominator).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rat {
    pub num: Int,
    pub den: Nat, // invariant: den != 0
}

impl Rat {
    pub fn new(num: Int, den: Nat) -> Option<Rat> {
        if den.is_zero() {
            return None;
        }
        Some(Rat { num, den }.normalized())
    }

    pub fn zero() -> Rat {
        Rat {
            num: Int::Zero,
            den: Nat::one(),
        }
    }

    pub fn one() -> Rat {
        Rat {
            num: Int::from_nat(Nat::one()),
            den: Nat::one(),
        }
    }

    pub fn is_zero(&self) -> bool {
        self.num.is_zero()
    }

    pub fn is_negative(&self) -> bool {
        self.num.is_negative()
    }

    pub fn from_int(i: Int) -> Rat {
        Rat::new(i, Nat::one()).expect("den != 0")
    }

    pub fn abs(&self) -> Rat {
        if self.is_negative() {
            self.neg()
        } else {
            self.clone()
        }
    }

    fn normalized(mut self) -> Rat {
        if self.num.is_zero() {
            self.den = Nat::one();
            return self;
        }

        let g = self.num.abs_nat().gcd(&self.den);
        if g.is_zero() || g == Nat::one() {
            return self;
        }

        let den2 = self.den.div_exact(&g).expect("g divides den");
        let abs2 = self.num.abs_nat().div_exact(&g).expect("g divides |num|");

        let num2 = if self.num.is_negative() {
            Int::Neg(abs2)
        } else {
            Int::from_nat(abs2)
        };

        Rat {
            num: num2,
            den: den2,
        }
    }

    pub fn neg(&self) -> Rat {
        Rat {
            num: self.num.neg(),
            den: self.den.clone(),
        }
    }

    pub fn add(&self, other: &Rat) -> Rat {
        // a/b + c/d = (ad + cb) / (bd)
        let a = &self.num;
        let b = &self.den;
        let c = &other.num;
        let d = &other.den;

        let ad = a.mul(&Int::from_nat(d.clone()));
        let cb = c.mul(&Int::from_nat(b.clone()));
        let num = ad.add(&cb);
        let den = b.mul(d);

        Rat::new(num, den).expect("den != 0")
    }

    pub fn sub(&self, other: &Rat) -> Rat {
        self.add(&other.neg())
    }

    pub fn mul(&self, other: &Rat) -> Rat {
        // (a/b)(c/d) = (ac)/(bd)
        let num = self.num.mul(&other.num);
        let den = self.den.mul(&other.den);
        Rat::new(num, den).expect("den != 0")
    }

    pub fn div(&self, other: &Rat) -> Option<Rat> {
        // (a/b)/(c/d) = (a*d)/(b*c)
        if other.num.is_zero() {
            return None;
        }
        let abs_c = other.num.abs_nat();
        if abs_c.is_zero() {
            return None;
        }

        let mut num = self.num.mul(&Int::from_nat(other.den.clone()));
        if other.num.is_negative() {
            num = num.neg();
        }
        let den = self.den.mul(&abs_c);
        Rat::new(num, den)
    }

    pub fn cmp_rat(&self, other: &Rat) -> Ordering {
        // Compare a/b and c/d by comparing ad and cb (denominators are > 0).
        let a = &self.num;
        let b = &self.den;
        let c = &other.num;
        let d = &other.den;

        let ad = a.mul(&Int::from_nat(d.clone()));
        let cb = c.mul(&Int::from_nat(b.clone()));
        ad.cmp(&cb)
    }
}

impl PartialOrd for Rat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp_rat(other))
    }
}

impl Ord for Rat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmp_rat(other)
    }
}

impl fmt::Display for Rat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.den == Nat::one() {
            write!(f, "{}", self.num)
        } else {
            write!(f, "{}/{}", self.num, self.den)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rat_normalizes_and_adds() {
        let two = Int::from_nat(Nat::from_usize(2));
        let four = Nat::from_usize(4);
        let half = Rat::new(two, four).unwrap();
        assert_eq!(format!("{}", half), "1/2");

        let one = Rat::new(Int::from_nat(Nat::one()), Nat::from_usize(2)).unwrap(); // 1/2
        let three = Rat::new(Int::from_nat(Nat::one()), Nat::from_usize(3)).unwrap(); // 1/3
        let s = one.add(&three);
        assert_eq!(format!("{}", s), "5/6");
    }
}
