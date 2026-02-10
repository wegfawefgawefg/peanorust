use std::cmp::Ordering;
use std::fmt;

use crate::n::Nat;

// Integers (Z) built on top of N. We normalize to a canonical form:
// - Zero
// - Pos(n) where n > 0
// - Neg(n) where n > 0
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Int {
    Neg(Nat),
    Zero,
    Pos(Nat),
}

impl Int {
    pub fn zero() -> Int {
        Int::Zero
    }

    pub fn from_nat(n: Nat) -> Int {
        if n.is_zero() { Int::Zero } else { Int::Pos(n) }
    }

    pub fn neg(&self) -> Int {
        match self {
            Int::Zero => Int::Zero,
            Int::Pos(n) => Int::Neg(n.clone()),
            Int::Neg(n) => Int::Pos(n.clone()),
        }
    }

    pub fn abs_nat(&self) -> Nat {
        match self {
            Int::Zero => Nat::Zero,
            Int::Pos(n) | Int::Neg(n) => n.clone(),
        }
    }

    pub fn is_zero(&self) -> bool {
        matches!(self, Int::Zero)
    }

    pub fn is_negative(&self) -> bool {
        matches!(self, Int::Neg(_))
    }

    pub fn from_diff(pos: Nat, neg: Nat) -> Int {
        // Represents pos - neg, normalize to canonical enum.
        match pos.cmp(&neg) {
            std::cmp::Ordering::Equal => Int::Zero,
            std::cmp::Ordering::Greater => {
                let d = pos.sub_unchecked_gte(&neg);
                if d.is_zero() { Int::Zero } else { Int::Pos(d) }
            }
            std::cmp::Ordering::Less => {
                let d = neg.sub_unchecked_gte(&pos);
                if d.is_zero() { Int::Zero } else { Int::Neg(d) }
            }
        }
    }

    fn as_diff(&self) -> (Nat, Nat) {
        // Return (pos, neg) meaning pos - neg.
        match self {
            Int::Zero => (Nat::Zero, Nat::Zero),
            Int::Pos(n) => (n.clone(), Nat::Zero),
            Int::Neg(n) => (Nat::Zero, n.clone()),
        }
    }

    pub fn add(&self, other: &Int) -> Int {
        let (a_pos, a_neg) = self.as_diff();
        let (b_pos, b_neg) = other.as_diff();
        Int::from_diff(a_pos.add(&b_pos), a_neg.add(&b_neg))
    }

    pub fn sub(&self, other: &Int) -> Int {
        self.add(&other.neg())
    }

    pub fn mul(&self, other: &Int) -> Int {
        // (a-b)(c-d) = (ac + bd) - (ad + bc)
        let (a, b) = self.as_diff();
        let (c, d) = other.as_diff();
        let ac = a.mul(&c);
        let bd = b.mul(&d);
        let ad = a.mul(&d);
        let bc = b.mul(&c);
        Int::from_diff(ac.add(&bd), ad.add(&bc))
    }

    pub fn cmp_int(&self, other: &Int) -> Ordering {
        match (self, other) {
            (Int::Neg(a), Int::Neg(b)) => b.cmp(a), // more negative = smaller
            (Int::Neg(_), Int::Zero) => Ordering::Less,
            (Int::Neg(_), Int::Pos(_)) => Ordering::Less,
            (Int::Zero, Int::Neg(_)) => Ordering::Greater,
            (Int::Zero, Int::Zero) => Ordering::Equal,
            (Int::Zero, Int::Pos(_)) => Ordering::Less,
            (Int::Pos(_), Int::Neg(_)) => Ordering::Greater,
            (Int::Pos(_), Int::Zero) => Ordering::Greater,
            (Int::Pos(a), Int::Pos(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for Int {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp_int(other))
    }
}

impl Ord for Int {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmp_int(other)
    }
}

impl fmt::Display for Int {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Int::Zero => write!(f, "0"),
            Int::Pos(n) => write!(f, "{}", n),
            Int::Neg(n) => write!(f, "-{}", n),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int_add_sub_mul() {
        let three = Int::from_nat(Nat::from_usize(3));
        let five = Int::from_nat(Nat::from_usize(5));
        let neg_two = three.sub(&five);
        assert_eq!(format!("{}", neg_two), "-2");

        let seven = Int::from_nat(Nat::from_usize(7));
        assert_eq!(format!("{}", neg_two.add(&seven)), "5");

        let neg_three = Int::from_diff(Nat::Zero, Nat::from_usize(3));
        let four = Int::from_nat(Nat::from_usize(4));
        assert_eq!(format!("{}", neg_three.mul(&four)), "-12");
    }
}
