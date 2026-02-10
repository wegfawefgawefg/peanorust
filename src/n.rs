use std::cmp::Ordering;
use std::fmt;

// Natural numbers (N) in Peano form: 0 | S(n).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Nat {
    Zero,
    Succ(Box<Nat>),
}

impl Nat {
    pub fn zero() -> Nat {
        Nat::Zero
    }

    pub fn one() -> Nat {
        Nat::Succ(Box::new(Nat::Zero))
    }

    pub fn is_zero(&self) -> bool {
        matches!(self, Nat::Zero)
    }

    pub fn succ(self) -> Nat {
        Nat::Succ(Box::new(self))
    }

    pub fn pred(&self) -> Option<&Nat> {
        match self {
            Nat::Zero => None,
            Nat::Succ(n) => Some(n),
        }
    }

    pub fn from_usize(mut n: usize) -> Nat {
        let mut out = Nat::Zero;
        while n > 0 {
            out = out.succ();
            n -= 1;
        }
        out
    }

    pub fn to_usize(&self) -> usize {
        match self {
            Nat::Zero => 0,
            Nat::Succ(n) => 1 + n.to_usize(),
        }
    }

    pub fn add(&self, other: &Nat) -> Nat {
        match self {
            Nat::Zero => other.clone(),
            Nat::Succ(n) => Nat::Succ(Box::new(n.add(other))),
        }
    }

    // Partial subtraction on naturals: returns None if other > self.
    pub fn sub(&self, other: &Nat) -> Option<Nat> {
        match (self, other) {
            (Nat::Zero, Nat::Zero) => Some(Nat::Zero),
            (Nat::Zero, Nat::Succ(_)) => None,
            (Nat::Succ(n), Nat::Zero) => Some(Nat::Succ(Box::new((**n).clone()))),
            (Nat::Succ(n), Nat::Succ(m)) => n.sub(m),
        }
    }

    pub fn sub_unchecked_gte(&self, other: &Nat) -> Nat {
        // Only call when self >= other.
        match (self, other) {
            (Nat::Zero, Nat::Zero) => Nat::Zero,
            (Nat::Succ(n), Nat::Zero) => Nat::Succ(Box::new((**n).clone())),
            (Nat::Succ(n), Nat::Succ(m)) => n.sub_unchecked_gte(m),
            _ => panic!("sub_unchecked_gte called with self < other"),
        }
    }

    pub fn mul(&self, other: &Nat) -> Nat {
        match self {
            Nat::Zero => Nat::Zero,
            Nat::Succ(n) => n.mul(other).add(other),
        }
    }

    pub fn cmp_nat(&self, other: &Nat) -> Ordering {
        match (self, other) {
            (Nat::Zero, Nat::Zero) => Ordering::Equal,
            (Nat::Zero, Nat::Succ(_)) => Ordering::Less,
            (Nat::Succ(_), Nat::Zero) => Ordering::Greater,
            (Nat::Succ(a), Nat::Succ(b)) => a.cmp_nat(b),
        }
    }

    // div_mod by repeated subtraction. Very slow; that's the point here.
    pub fn div_mod(&self, d: &Nat) -> Option<(Nat, Nat)> {
        if d.is_zero() {
            return None;
        }
        let mut q = Nat::Zero;
        let mut r = self.clone();
        while r.cmp_nat(d) != Ordering::Less {
            r = r.sub_unchecked_gte(d);
            q = q.succ();
        }
        Some((q, r))
    }

    pub fn gcd(&self, other: &Nat) -> Nat {
        // Euclid: gcd(a, 0) = a, gcd(a, b) = gcd(b, a mod b)
        let mut a = self.clone();
        let mut b = other.clone();
        while !b.is_zero() {
            let (_, r) = a.div_mod(&b).expect("b != 0");
            a = b;
            b = r;
        }
        a
    }

    pub fn div_exact(&self, d: &Nat) -> Option<Nat> {
        let (q, r) = self.div_mod(d)?;
        if r.is_zero() { Some(q) } else { None }
    }
}

impl PartialOrd for Nat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp_nat(other))
    }
}

impl Ord for Nat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmp_nat(other)
    }
}

impl fmt::Display for Nat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_usize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nat_add_mul_sub_div_mod() {
        let a = Nat::from_usize(7);
        let b = Nat::from_usize(3);
        assert_eq!(a.add(&b).to_usize(), 10);
        assert_eq!(b.mul(&Nat::from_usize(4)).to_usize(), 12);
        assert_eq!(a.sub(&b).unwrap().to_usize(), 4);
        assert!(b.sub(&a).is_none());
        let (q, r) = a.div_mod(&b).unwrap();
        assert_eq!(q.to_usize(), 2);
        assert_eq!(r.to_usize(), 1);
    }
}
